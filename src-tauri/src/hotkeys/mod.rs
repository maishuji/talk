mod config;
mod ending;
mod paste_line;
mod phase;
mod press_line;

pub use config::{
    config, find, init, suspends_sync, tell_owed, update_config,
    HotkeyConfig,
};
pub use phase::Phase;

use crate::audio::Capture;
use crate::{audio, audio_encoder, database, overlay_feedback, server_transcription, AppState, RecordingMode};
use crate::settings::TranscriptionMode;
use crate::dictation_queue::{CancelScope, PasteTarget, Release, Transcript};
use std::sync::atomic::{AtomicBool, Ordering};
use ending::Ending;
use paste_line::{PasteLine, Turn};
use phase::Opened;
use press_line::{Edge, PressLine, Step};
use std::sync::{Arc, OnceLock};
use tauri::{AppHandle, Emitter, EventTarget, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

/// Where the volume was before the machine was turned down, and which step has the
/// last word. One lock, taken together with the fade ticket, for each transition.
static DUCK: parking_lot::Mutex<crate::ducking::DuckState> =
    parking_lot::Mutex::new(crate::ducking::DuckState::new());

/// Held while what is persisted is made to say what memory says, so that two such
/// steps on two threads cannot land in the opposite order.
static PERSIST_IO: parking_lot::Mutex<()> = parking_lot::Mutex::new(());

/// Make the marker file say what memory says now: the level while the volume is
/// down, nothing once it is back. It runs off the paths that wait for the volume,
/// never goes through the settings store or the sync, and a failure leaves memory
/// as it is.
fn sync_persisted_level() {
    let _io = PERSIST_IO.lock();
    let level = DUCK.lock().level();
    let marker = crate::ducking::DuckMarker::in_config_dir();
    match level {
        Some(level) => {
            if let Err(e) = marker.write(level) {
                eprintln!("Failed to write the volume marker: {}", e);
            }
        }
        None => marker.clear(),
    }
}

/// Take the machine down while the microphone is open, and remember where it was.
///
/// This replaces sending MediaPlayPause at whatever window happened to be in
/// front, which hit the wrong application as often as the right one and had no
/// way of knowing whether it had paused or resumed. Lowering the render
/// endpoint touches everything at once and is exactly reversible.
fn duck_audio() {
    let (ducking, percent) = crate::settings::read(|s| (s.duck_audio_on_record, s.duck_volume_percent));
    if !ducking {
        return;
    }

    let Some(before) = crate::ducking::current_volume() else {
        return;
    };

    let target = crate::ducking::duck_level(before, percent);
    // Nothing to do if it is already at or below where we would put it. Storing
    // the level anyway would restore somebody's volume upwards on stop.
    if before <= target {
        return;
    }

    // The ticket is taken with the transition: a restore that comes after takes a
    // later one and stops this slide, and one that came before is stopped by it.
    let (ducked, ticket) = {
        let mut state = DUCK.lock();
        (state.duck(before), crate::ducking::take_fade_ticket())
    };

    // The slide goes on its own thread, since this path still has an overlay to show
    // and an event to emit.
    std::thread::spawn(move || {
        crate::ducking::fade_volume_with(ticket, target, crate::ducking::FADE_DOWN_MS);
    });

    // The persisted level is for a process that dies while the volume is down: the
    // next launch is the only thing left that can put it back. It is written off
    // this path and off the slide.
    if ducked.first {
        std::thread::spawn(sync_persisted_level);
    }
}

/// Put the volume back where it was, if this recording is what moved it.
fn restore_audio() {
    let (restoring, ticket) = {
        let mut state = DUCK.lock();
        let Some(restoring) = state.restore(None) else {
            return;
        };
        (restoring, crate::ducking::take_fade_ticket())
    };

    // The level is forgotten once the volume is actually back, and not before: a
    // process that dies halfway up would otherwise leave the machine quiet with
    // nothing left saying where it came from. A recording started during the slide
    // moves the state on, and this slide then leaves the level for the newer one.
    std::thread::spawn(move || {
        crate::ducking::fade_volume_with(ticket, restoring.level, crate::ducking::FADE_UP_MS);
        if DUCK.lock().finish_restore(restoring.generation) {
            sync_persisted_level();
        }
    });
}

/// Put the volume back now and wait for it, for a moment when nothing runs
/// afterwards: the launch that finds a level left by a crash, and quitting.
/// This is the one place that restores from what was persisted.
///
/// Its ticket stops a slide still running, and its step moves the state on, so a
/// restore thread leaves the level alone. The level is forgotten only once the
/// volume is back.
pub fn restore_audio_now() {
    let known = DUCK.lock().level().is_some();
    let left_by_a_crash = if known { None } else { crate::ducking::DuckMarker::in_config_dir().read() };
    let (restoring, ticket) = {
        let mut state = DUCK.lock();
        let Some(restoring) = state.restore(left_by_a_crash) else {
            return;
        };
        (restoring, crate::ducking::take_fade_ticket())
    };

    // The fade is what stops a slide still running, and the set lands the
    // exact level whatever step it stopped on.
    if crate::ducking::fade_volume_with(ticket, restoring.level, 0)
        && crate::ducking::set_volume(restoring.level)
        && DUCK.lock().finish_restore(restoring.generation)
    {
        sync_persisted_level();
    }
}

pub fn setup_shortcuts(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let config = config();
    let state = app.state::<AppState>();

    // Store the parsed shortcuts in AppState: the single handler in
    // Builder::with_handler dispatches on these values, so nothing here needs
    // an on_shortcut closure of its own.
    if let Ok(main) = parse_shortcut(&config.shortcut) {
        *state.main_shortcut.lock() = Some(main);
    }
    if let Ok(cancel) = parse_shortcut(&config.cancel_shortcut) {
        *state.cancel_shortcut.lock() = Some(cancel);
    }
    if let Ok(paste) = parse_shortcut(&config.paste_shortcut) {
        *state.paste_shortcut.lock() = Some(paste);
    }

    register_stored(app.handle());

    Ok(())
}

/// Put every stored shortcut back up, dropping whatever was registered.
///
/// The plugin unregisters by naming a combination, and each caller here has
/// just replaced one of the three, so the lot goes down and the lot comes back
/// rather than each path remembering to re-register the two it did not touch.
/// A combination another application already holds is reported and skipped,
/// leaving the other two working.
fn register_stored(app: &AppHandle) {
    let global_shortcut = app.global_shortcut();
    let state = app.state::<AppState>();

    if let Err(e) = global_shortcut.unregister_all() {
        eprintln!("Warning: failed to unregister shortcuts: {}", e);
    }

    let stored = [
        ("Main", *state.main_shortcut.lock()),
        ("Cancel", *state.cancel_shortcut.lock()),
        ("Paste", *state.paste_shortcut.lock()),
    ];

    for (name, shortcut) in stored {
        let Some(shortcut) = shortcut else { continue };
        if let Err(e) = global_shortcut.register(shortcut) {
            eprintln!("{} shortcut register error: {} - try a different one", name, e);
        }
    }
}

pub fn disable_shortcuts(app: &AppHandle) {
    let global_shortcut = app.global_shortcut();
    if let Err(e) = global_shortcut.unregister_all() {
        eprintln!("Warning: failed to unregister shortcuts: {}", e);
    }
}

pub fn enable_shortcuts(app: &AppHandle) {
    register_stored(app);
}

/// The three below write the file first, and put the combination in memory and
/// register it with the system only once it is written. One that cannot be parsed
/// or cannot be written is an error and changes nothing.
pub fn update_shortcut(app: &AppHandle, new_shortcut: &str) -> Result<(), Box<dyn std::error::Error>> {
    let parsed = parse_shortcut(new_shortcut)?;
    update_config(|config| config.shortcut = new_shortcut.to_string())?;
    *app.state::<AppState>().main_shortcut.lock() = Some(parsed);
    register_stored(app);
    Ok(())
}

pub fn update_cancel_shortcut(app: &AppHandle, new_shortcut: &str) -> Result<(), Box<dyn std::error::Error>> {
    let parsed = parse_shortcut(new_shortcut)?;
    update_config(|config| config.cancel_shortcut = new_shortcut.to_string())?;
    *app.state::<AppState>().cancel_shortcut.lock() = Some(parsed);
    register_stored(app);
    Ok(())
}

pub fn update_paste_shortcut(app: &AppHandle, new_shortcut: &str) -> Result<(), Box<dyn std::error::Error>> {
    let parsed = parse_shortcut(new_shortcut)?;
    update_config(|config| config.paste_shortcut = new_shortcut.to_string())?;
    *app.state::<AppState>().paste_shortcut.lock() = Some(parsed);
    register_stored(app);
    Ok(())
}

fn parse_shortcut(shortcut_str: &str) -> Result<Shortcut, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = shortcut_str.split('+').collect();

    let mut modifiers = Modifiers::empty();
    let mut key_code = None;

    for part in parts {
        let part = part.trim();
        match part.to_lowercase().as_str() {
            "ctrl" | "control" => modifiers |= Modifiers::CONTROL,
            "shift" => modifiers |= Modifiers::SHIFT,
            "alt" => modifiers |= Modifiers::ALT,
            "super" | "win" | "meta" => modifiers |= Modifiers::SUPER,
            "space" => key_code = Some(Code::Space),
            "enter" | "return" => key_code = Some(Code::Enter),
            "tab" => key_code = Some(Code::Tab),
            "escape" | "esc" => key_code = Some(Code::Escape),
            "backspace" => key_code = Some(Code::Backspace),
            "delete" => key_code = Some(Code::Delete),
            "insert" => key_code = Some(Code::Insert),
            "home" => key_code = Some(Code::Home),
            "end" => key_code = Some(Code::End),
            "pageup" => key_code = Some(Code::PageUp),
            "pagedown" => key_code = Some(Code::PageDown),
            "up" | "arrowup" => key_code = Some(Code::ArrowUp),
            "down" | "arrowdown" => key_code = Some(Code::ArrowDown),
            "left" | "arrowleft" => key_code = Some(Code::ArrowLeft),
            "right" | "arrowright" => key_code = Some(Code::ArrowRight),
            // F-keys
            "f1" => key_code = Some(Code::F1),
            "f2" => key_code = Some(Code::F2),
            "f3" => key_code = Some(Code::F3),
            "f4" => key_code = Some(Code::F4),
            "f5" => key_code = Some(Code::F5),
            "f6" => key_code = Some(Code::F6),
            "f7" => key_code = Some(Code::F7),
            "f8" => key_code = Some(Code::F8),
            "f9" => key_code = Some(Code::F9),
            "f10" => key_code = Some(Code::F10),
            "f11" => key_code = Some(Code::F11),
            "f12" => key_code = Some(Code::F12),
            // Numbers
            "0" | "digit0" => key_code = Some(Code::Digit0),
            "1" | "digit1" => key_code = Some(Code::Digit1),
            "2" | "digit2" => key_code = Some(Code::Digit2),
            "3" | "digit3" => key_code = Some(Code::Digit3),
            "4" | "digit4" => key_code = Some(Code::Digit4),
            "5" | "digit5" => key_code = Some(Code::Digit5),
            "6" | "digit6" => key_code = Some(Code::Digit6),
            "7" | "digit7" => key_code = Some(Code::Digit7),
            "8" | "digit8" => key_code = Some(Code::Digit8),
            "9" | "digit9" => key_code = Some(Code::Digit9),
            // Letters
            "a" => key_code = Some(Code::KeyA),
            "b" => key_code = Some(Code::KeyB),
            "c" => key_code = Some(Code::KeyC),
            "d" => key_code = Some(Code::KeyD),
            "e" => key_code = Some(Code::KeyE),
            "f" => key_code = Some(Code::KeyF),
            "g" => key_code = Some(Code::KeyG),
            "h" => key_code = Some(Code::KeyH),
            "i" => key_code = Some(Code::KeyI),
            "j" => key_code = Some(Code::KeyJ),
            "k" => key_code = Some(Code::KeyK),
            "l" => key_code = Some(Code::KeyL),
            "m" => key_code = Some(Code::KeyM),
            "n" => key_code = Some(Code::KeyN),
            "o" => key_code = Some(Code::KeyO),
            "p" => key_code = Some(Code::KeyP),
            "q" => key_code = Some(Code::KeyQ),
            "r" => key_code = Some(Code::KeyR),
            "s" => key_code = Some(Code::KeyS),
            "t" => key_code = Some(Code::KeyT),
            "u" => key_code = Some(Code::KeyU),
            "v" => key_code = Some(Code::KeyV),
            "w" => key_code = Some(Code::KeyW),
            "x" => key_code = Some(Code::KeyX),
            "y" => key_code = Some(Code::KeyY),
            "z" => key_code = Some(Code::KeyZ),
            _ => {}
        }
    }

    let code = key_code.ok_or("No valid key found in shortcut")?;

    Ok(Shortcut::new(Some(modifiers), code))
}

/// Keeps the overlay alive as long as any dictation still needs it.
///
/// Recording and transcribing overlap: `is_recording` is cleared as soon as the
/// audio is taken, so pressing the shortcut again starts a new capture while the
/// previous transcription is still running. The overlay is one shared window,
/// and before this the first transcription to finish hid it, pulling it out from
/// under whatever had started since.
///
/// Releasing on drop rather than at the end of the happy path matters: the
/// transcription has half a dozen early returns, and every one of them used to
/// be a way to leave the overlay up or the count wrong.
struct OverlayLease {
    app: AppHandle,
}

impl OverlayLease {
    fn take(app: &AppHandle) -> Self {
        let state = app.state::<AppState>();
        // A transcription takes the overlay: whatever timer was running for an older state is void.
        state.overlay_gen.begin();
        *state.overlay_hold_until.lock() = None;
        let count = state.jobs_in_flight.fetch_add(1, Ordering::SeqCst) + 1;
        announce_jobs(app, count);
        Self { app: app.clone() }
    }
}

/// Take the overlay down after a while, if nothing else has put it to use in
/// the meantime: a recording or a transcription started since keeps it or gets it back,
/// and a state that took the overlay since is not ours to hide.
fn release_overlay_after(app: &AppHandle, hold_ms: u64) {
    let state = app.state::<AppState>();
    let generation = state.overlay_gen.begin();
    // A job that lets go of the overlay while this is up leaves it to the hold's own end.
    *state.overlay_hold_until.lock() = Some(overlay_feedback::hold_end(std::time::Instant::now(), hold_ms));
    let app = app.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(hold_ms));
        let state = app.state::<AppState>();
        // The decision and what it does happen under the phase lock, so a recording cannot
        // start in between and have its overlay hidden under it.
        let phase = state.phase.lock();
        let jobs = state.jobs_in_flight.load(Ordering::SeqCst);
        let overlay = EventTarget::webview_window("overlay");
        match overlay_feedback::after_hold(state.overlay_gen.is_current(generation), phase.active(), jobs) {
            overlay_feedback::Hold::Leave => {}
            overlay_feedback::Hold::ResumeRecording => {
                *state.overlay_hold_until.lock() = None;
                let _ = app.emit_to(overlay, "processing-state", "recording");
            }
            overlay_feedback::Hold::Resume => {
                *state.overlay_hold_until.lock() = None;
                // The job's own state, so a server's job keeps its icon.
                let job = *state.job_state.lock();
                let _ = app.emit_to(overlay, "processing-state", job);
            }
            overlay_feedback::Hold::Hide => hide_overlay(&app),
        }
    });
}

/// Tell the overlay how many dictations are still being transcribed, so a
/// recording started behind them can show that they are on their way.
fn announce_jobs(app: &AppHandle, count: usize) {
    let _ = app.emit_to(EventTarget::webview_window("overlay"), "jobs-in-flight", count);
}

impl Drop for OverlayLease {
    fn drop(&mut self) {
        let state = self.app.state::<AppState>();
        // fetch_sub returns the value before the subtraction, so 1 means this
        // was the last one.
        let before = state.jobs_in_flight.fetch_sub(1, Ordering::SeqCst);
        announce_jobs(&self.app, before - 1);
        let was_last = before == 1;
        // Before the recording lock below: the sync reads it too.
        if was_last {
            crate::sync::dictation_ended(&self.app);
        }
        // Under the phase lock from the check to the hide, so a recording cannot start in
        // between and have its overlay hidden under it.
        let phase = state.phase.lock();
        if !was_last || phase.active() {
            return;
        }

        // Text just reached the focused window, or a dictation was turned away: the overlay is
        // held for it, and the hold has its own end.
        if overlay_feedback::holding(*state.overlay_hold_until.lock(), std::time::Instant::now()) {
            return;
        }

        let _ = self.app.emit_to(
            EventTarget::webview_window("overlay"),
            "processing-state",
            "idle",
        );
        crate::overlay::hide(&self.app);
    }
}

/// Run the local engine without parking a runtime worker.
///
/// `transcribe_with_options` is synchronous and holds the engine mutex for its
/// whole run, which is seconds on a long dictation. Awaiting that on a tokio
/// worker blocks the worker, so a second dictation stopping in the meantime has
/// nowhere to run. The blocking pool is where work like this belongs.
///
/// `Ok(None)` means no model is loaded, which each caller words differently.
async fn transcribe_locally(
    app: &AppHandle,
    audio: Vec<f32>,
    vocabulary: Option<String>,
    cancel: Arc<AtomicBool>,
) -> Result<Option<String>, String> {
    let app_for_job = app.clone();
    let app_for_progress = app.clone();
    let cancel_for_progress = cancel.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let state = app_for_job.state::<AppState>();
        let engine_lock = state.whisper_engine.lock();
        let Some(engine) = engine_lock.as_ref() else {
            return Ok(None);
        };

        // Cancelled while it waited for the engine: nothing to start.
        if cancel.load(Ordering::SeqCst) {
            return Err("Cancelled".to_string());
        }

        engine
            .transcribe_with_options(
                &audio,
                vocabulary.as_deref(),
                move |progress| {
                    // A cancelled run keeps going until whisper next looks,
                    // and its progress would be drawn over the next one's.
                    if cancel_for_progress.load(Ordering::SeqCst) {
                        return;
                    }
                    let _ = app_for_progress.emit_to(
                        EventTarget::webview_window("overlay"),
                        "transcription-progress",
                        progress,
                    );
                },
                move || cancel.load(Ordering::SeqCst),
            )
            .map(Some)
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| format!("Local transcription did not run: {}", e))?
}

/// Where every text on its way to the focused window waits its turn.
static PASTES: PasteLine = PasteLine::new();

/// Take a place in the paste line for what the queue just let go. Called while the
/// queue's lock is still held, which is what makes the line's order the queue's order.
fn take_turn(release: &Release) -> Option<Turn> {
    (!release.is_empty()).then(|| PASTES.take(release.parts.clone()))
}

/// Act on a release once its turn has come, with no lock of the queue held.
fn hand_out_in_turn(app: &AppHandle, release: Release, turn: Option<Turn>) {
    if let Some(turn) = turn {
        turn.run(|cancelled| hand_out(app, release, cancelled));
        // The sync waits for a paste too, and its round may be owed.
        crate::sync::dictation_ended(app);
    }
}

/// Whether a text is being pasted or waiting for its turn to be.
pub fn pasting() -> bool {
    PASTES.pending()
}

/// Where every press and release of the main shortcut goes through.
static PRESSES: OnceLock<PressLine> = OnceLock::new();

pub fn handle_shortcut_event(app: &AppHandle, state: ShortcutState) {
    let mode = *app.state::<AppState>().recording_mode.lock();
    let edge = match state {
        ShortcutState::Pressed => Edge::Pressed,
        ShortcutState::Released => Edge::Released,
    };
    PRESSES
        .get_or_init(|| {
            let app = app.clone();
            let for_recovery = app.clone();
            PressLine::spawn(move |(mode, edge)| act_on(&app, mode, edge), move || reset_after_panic(&for_recovery))
        })
        .push((mode, edge));
}

/// A handler that panicked may have left a capture open, the machine turned down and the
/// overlay on "recording": it ends like any other dictation, so that the next press starts clean.
fn reset_after_panic(app: &AppHandle) {
    ending::end_dictation(app, Ending::Panicked);
}

/// Do what one event of the line amounts to. The line hands them over one at a
/// time, so a release always finds the press before it already done.
fn act_on(app: &AppHandle, mode: RecordingMode, edge: Edge) {
    let recording = app.state::<AppState>().phase.lock().active();
    match press_line::step(mode, edge, recording) {
        Step::Start => {
            let _ = start_recording_internal(app);
        }
        Step::Stop => {
            if let Ok(dictation) = stop_recording(app) {
                // The transcription runs on its own, so the next press is not held up behind it.
                tauri::async_runtime::spawn(finish_dictation(app.clone(), dictation));
            }
        }
        Step::Ignore => {}
    }
}

/// Drop the recording if one is running, otherwise what is being transcribed.
///
/// The recording goes first because it is what the user is in the middle of,
/// and a second press then reaches the transcriptions behind it.
///
/// Either can paste, when a held paragraph completes or the next dictation in
/// line is let through, so like the paste shortcut it stays off the thread the
/// shortcut handler runs on.
pub fn cancel(app: &AppHandle) {
    let app = app.clone();
    std::thread::spawn(move || {
        // A press under way, opening or recording, is what goes first.
        if !ending::end_dictation(&app, Ending::Cancelled) {
            cancel_transcriptions(&app);
        }
    });
}

fn cancel_transcriptions(app: &AppHandle) {
    let state = app.state::<AppState>();
    let settings = crate::settings::read(|s| s.queue);

    let mut queue = state.dictation_queue.lock();
    // What the queue already let go and is waiting for its paste is cancellable too, under the
    // same lock so that nothing slips out in between. The oldest waiting text is older than
    // anything still transcribing, so for the "current" scope it is the one.
    let pasting = PASTES.cancel(settings.cancel_scope);
    // A cancelled text is forgotten everywhere, the paste-last batch included.
    queue.forget(&pasting.parts);
    let (any, release) = if pasting.any() && settings.cancel_scope == CancelScope::Current {
        (false, Release::default())
    } else {
        queue.cancel(settings.cancel_scope, settings.delivery, false)
    };
    if !any && !pasting.any() {
        return;
    }
    let turn = take_turn(&release);
    let idle = queue.is_idle();
    drop(queue);
    hand_out_in_turn(app, release, turn);

    // The transcriptions themselves end when whisper or the server next
    // notices, which can be a second away. The user asked for it gone now.
    if idle {
        hide_overlay(app);
    }

    play_sound_feedback(app, "stop");
}

fn hide_overlay(app: &AppHandle) {
    *app.state::<AppState>().overlay_hold_until.lock() = None;
    let _ = app.emit_to(
        EventTarget::webview_window("overlay"),
        "processing-state",
        "idle",
    );
    crate::overlay::hide(app);
}

/// Paste what was dictated last, wherever the caret happens to be.
///
/// Without it the text is reachable only by coming back to the window and
/// copying the card, which is a trip through two applications for something
/// that was on screen a second ago.
///
/// The row is read from the database rather than from memory, so the shortcut
/// still answers after a restart, and it answers with what the history shows
/// rather than with a copy that outlived a clear.
pub fn paste_last_transcription(app: &AppHandle) {
    let app = app.clone();

    // The clipboard path waits for the shortcut's own modifiers to come back
    // up and sleeps either side of the paste, so it does not belong on the
    // thread the shortcut handler runs on.
    std::thread::spawn(move || {
        let state = app.state::<AppState>();

        // The batch lives in memory only, so after a restart this falls back
        // to the last row, which is all a single dictation ever was anyway.
        let batch = match crate::settings::read(|s| s.queue.paste_target) {
            PasteTarget::Batch => state.dictation_queue.lock().latest_batch(),
            PasteTarget::Last => None,
        };

        let text = match batch {
            Some(text) => text,
            None => match app.state::<database::Database>().get_transcriptions(1, 0) {
                // Nothing has been dictated yet, or the history was cleared.
                // Pasting an empty string would wipe a selection for nothing.
                Ok(rows) => match rows.into_iter().next() {
                    Some(last) => last.text,
                    None => return,
                },
                Err(e) => {
                    eprintln!("Failed to read the last transcription: {}", e);
                    return;
                }
            },
        };

        let preserve = crate::settings::read(|s| s.preserve_clipboard);
        PASTES.take(Vec::new()).run(|cancelled| {
            if cancelled.load(Ordering::SeqCst) {
                return;
            }
            if let Err(e) = crate::clipboard::type_text(&text, preserve) {
                eprintln!("Failed to paste the last transcription: {}", e);
            }
        });
        // The sync waits for a paste too, and its round may be owed.
        crate::sync::dictation_ended(&app);
    });
}

/// Play sound feedback if enabled in settings. Non-blocking.
fn play_sound_feedback(app: &AppHandle, sound_type: &str) {
    let (enabled, preset) = crate::settings::read(|s| {
        let preset = match sound_type {
            "start" => s.start_sound.clone(),
            "refused" => String::new(),
            _ => s.stop_sound.clone(),
        };
        (s.sound_feedback, preset)
    });
    if !enabled || preset == "none" {
        return;
    }
    let state = app.state::<AppState>();
    let engine_lock = state.sound_engine.lock();
    if let Some(ref engine) = *engine_lock {
        engine.play(sound_type, &preset);
    }
}

/// Turn a dictation away before it starts when nothing could transcribe it.
///
/// In local mode without a model the recording used to run as usual and the
/// text simply never came, which looks exactly like a microphone that heard
/// nothing. Server mode is left alone: the server does the work, and with the
/// fallback on, a missing model only matters once the server has failed.
fn refuse_without_model(app: &AppHandle) -> bool {
    let state = app.state::<AppState>();
    if crate::settings::read(|s| s.transcription_mode) != TranscriptionMode::Local {
        return false;
    }
    // Never wait on the engine here. A transcription holds that lock for its
    // whole run, so waiting would hold a new recording back until the previous
    // dictation is done. A lock held means a model is loaded and working.
    match state.whisper_engine.try_lock() {
        None => return false,
        Some(engine) if engine.is_some() => return false,
        Some(_) => {}
    }

    let reason = if state.model_loading.load(Ordering::SeqCst) {
        "model_loading"
    } else {
        "no_model"
    };

    ending::end_dictation(app, Ending::Refused(reason));
    true
}

/// Turn a dictation away: the refusal sound, and the overlay in the danger colour saying why.
///
/// The reasons the overlay words itself: `no_model`, `model_loading`, `capture_failed`,
/// `capture_lost`, `paste_failed`.
fn refuse(app: &AppHandle, reason: &str) {
    play_sound_feedback(app, "refused");
    crate::overlay::show(app);
    let _ = app.emit_to(EventTarget::webview_window("overlay"), "processing-state", reason);
    release_overlay_after(app, overlay_feedback::REFUSED_HOLD_MS);
}

fn start_recording_internal(app: &AppHandle) -> Result<(), String> {
    let state = app.state::<AppState>();

    if state.phase.lock().active() {
        return Ok(());
    }

    if refuse_without_model(app) {
        return Ok(());
    }

    // 1. A dictation is under way from the press, not from the moment the microphone is open:
    // the phase is what an earlier dictation about to hide the overlay, the cancel shortcut and
    // the sync all read. Then show the overlay first (pre-created at startup, just show it,
    // never recreate): opening the microphone takes as long as the driver takes, and the user
    // sees the press answered.
    let Some(generation) = state.phase.lock().begin_open() else {
        return Ok(());
    };
    state.overlay_gen.begin();
    *state.overlay_hold_until.lock() = None;
    crate::overlay::show(app);
    let _ = app.emit("recording-started", ());

    // 2. Mute virtual mic (instant, it just flips an AtomicBool)
    {
        let vm = state.virtual_mic.lock();
        if vm.is_active() {
            vm.mute();
        }
    }

    // 3. Start audio capture (use selected device or system default)
    let device_name = crate::settings::read(|s| s.input_device_name.clone());
    let (buffer, handle) = match audio::start_capture_device(device_name.as_deref()) {
        Ok(started) => started,
        Err(e) => {
            // A microphone that will not open is a dictation turned away like any other, unless
            // a cancel got there first and has already ended it.
            ending::end_dictation(app, Ending::OpenFailed(generation));
            return Err(e.to_string());
        }
    };
    let buffer_for_spectrum = buffer.clone();

    // Decided and stored in one step with the phase: a cancel after this clears what is stored,
    // and one before it ended the press this capture was opened for.
    let discarded = {
        let mut phase = state.phase.lock();
        match phase.opened(generation) {
            Opened::Keep => {
                *state.audio_buffer.lock() = Some(buffer);
                *state.audio_capture_handle.lock() = Some(handle);
                None
            }
            Opened::Discard => Some(handle),
        }
    };
    if let Some(handle) = discarded {
        // Dropping the handle ends the stream. The cancel did the rest, and the overlay it hid
        // may have been drawn again by the press that was still placing it.
        drop(handle);
        ending::settle_overlay(app);
        return Ok(());
    }

    // 4. Sound feedback (instant, from a pre-computed PCM buffer)
    play_sound_feedback(app, "start");

    // 5. Take the machine down so it does not talk over the speaker
    duck_audio();
    if !state.phase.lock().is_current(generation) {
        // Ended in between: its restore ran before the volume was taken down.
        restore_audio();
        return Ok(());
    }

    // 6. Start spectrum emission thread, which ends with the recording it was started for
    let app_for_spectrum = app.clone();
    std::thread::spawn(move || {
        let num_bars = 8;
        loop {
            let state = app_for_spectrum.state::<AppState>();
            if !state.phase.lock().is_current(generation) {
                break;
            }

            let levels = buffer_for_spectrum.get_spectrum(num_bars);
            let _ = app_for_spectrum.emit_to(
                EventTarget::webview_window("overlay"),
                "audio-spectrum",
                levels,
            );

            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    });

    Ok(())
}

/// A recording taken in, waiting to be transcribed.
struct Dictation {
    seq: u64,
    cancel: Arc<AtomicBool>,
    audio: Vec<f32>,
    /// Whether the microphone kept answering all along.
    capture: Capture,
    /// Keeps the overlay up for as long as this transcription runs, whichever way it ends.
    lease: OverlayLease,
}

/// End the recording and take its audio, with a place in line for the text.
fn stop_recording(app: &AppHandle) -> Result<Dictation, String> {
    let state = app.state::<AppState>();

    if !state.phase.lock().recording() {
        return Err("Not recording".to_string());
    }

    let audio_data = {
        let buffer_lock = state.audio_buffer.lock();
        if let Some(ref buffer) = *buffer_lock {
            buffer.take()
        } else {
            restore_audio();
            return Err("No audio buffer found".to_string());
        }
    };

    // Take a place in line before anything can finish, so the order the
    // dictations were spoken in is the order they come out in. And before the
    // recording flag drops: an earlier dictation finishing in between would
    // find nothing recording and nothing queued, and close the run without
    // this one. The overlay lease is taken for the same reason.
    let (seq, cancel) = state.dictation_queue.lock().enqueue();
    let lease = OverlayLease::take(app);

    // Stop audio capture - dropping the handle signals the stream thread to exit. A stream
    // that reported an error recorded silence from then on, and is read before it goes.
    let failed = state.audio_capture_handle.lock().take().is_some_and(|handle| handle.failed());
    let capture = Capture::of(failed, audio_data.len());

    // A capture that delivered nothing before it failed is turned away, with the refusal sound in
    // place of the stop one. One that delivered something is transcribed like any other, and the
    // loss is said after its paste.
    let ending = if capture == Capture::Empty { Ending::MicrophoneLost } else { Ending::Stopped };
    ending::end_dictation(app, ending);

    Ok(Dictation { seq, cancel, audio: audio_data, capture, lease })
}

/// Transcribe a recording and let its place in line go.
async fn finish_dictation(app: AppHandle, dictation: Dictation) {
    let state = app.state::<AppState>();
    let Dictation { seq, cancel, audio: audio_data, capture, lease: _lease } = dictation;

    // Both figures the history has always stored as null, because the frontend
    // was doing the saving and cannot know either of them. The capture is mono
    // at 16 kHz, which is what the encoder and whisper both assume.
    let audio_duration_ms = (audio_data.len() as f64 / 16_000.0 * 1000.0) as i64;
    let started = std::time::Instant::now();

    let outcome = if capture == Capture::Empty {
        // Said already, by the ending that turned it away.
        Err("The microphone stopped during the recording".to_string())
    } else {
        transcribe(&app, audio_data, cancel).await
    };

    // Nothing was said, or nothing came back. Whisper answers an empty string
    // for a recording with no speech in it, and a server can answer with
    // nothing at all while still answering. Going on would paste nothing, put a
    // blank card at the top of the history, and count a dictation that never
    // happened: the counters are permanent, so that last one is the one that
    // cannot be taken back from the interface.
    //
    // A failure still has to settle its place in line, or everything spoken
    // after it would wait for it forever.
    let transcript = match &outcome {
        Ok((text, source)) if !text.trim().is_empty() => Some(Transcript {
            text: text.clone(),
            source: *source,
            audio_duration_ms,
            processing_time_ms: started.elapsed().as_millis() as i64,
        }),
        Ok(_) => None,
        Err(e) => {
            eprintln!("Transcription failed: {}", e);
            None
        }
    };

    // What is still to be said about it once its text is out: the model that was not there when
    // the engine was asked for it, and the microphone that gave out part way.
    let refusal = match (&outcome, capture) {
        (Err(e), _) if e == NO_MODEL => Some("no_model"),
        (_, Capture::Cut) => Some("capture_lost"),
        _ => None,
    };

    let delivery = crate::settings::read(|s| s.queue.delivery);
    // The turn is taken under the lock, which keeps the order, and the paste waits for it
    // outside: two transcriptions finishing together do not paste over each other, and a
    // cancel is not held up behind a paste. Whether a recording is under way is read under the
    // same lock the ending of a press settles the queue under, so that a paragraph is never held
    // for a recording that has ended.
    let (release, turn) = {
        let mut queue = state.dictation_queue.lock();
        let recording = state.phase.lock().active();
        let release = queue.finish(seq, transcript, delivery, recording);
        let turn = take_turn(&release);
        (release, turn)
    };
    let app_for_paste = app.clone();
    let _ = tauri::async_runtime::spawn_blocking(move || {
        hand_out_in_turn(&app_for_paste, release, turn);
        // After the paste, once: the text is in the window, and the refusal sound and the
        // overlay's refusal state say why there is none, or that the microphone gave out.
        if let Some(reason) = refusal {
            ending::end_dictation(&app_for_paste, Ending::Refused(reason));
        }
    })
    .await;

    // The overlay goes down when the lease is dropped, and only if nothing else
    // still wants it.
}

/// What a transcription answers when the engine was asked for and none was there.
const NO_MODEL: &str = "No model loaded";

/// Run one dictation through whichever engine the mode says.
///
/// The source travels with the text: in server mode a fallback may have
/// quietly run this locally, and the history badge would otherwise lie about it.
async fn transcribe(
    app: &AppHandle,
    audio_data: Vec<f32>,
    cancel: Arc<AtomicBool>,
) -> Result<(String, &'static str), String> {
    let settings = crate::settings::get();
    let transcription_mode = settings.transcription_mode;
    let server_url = settings.server_url.clone();
    let server_fallback = settings.server_fallback;

    // Build vocabulary prompt from custom words only (comma-separated, no prefix)
    let vocabulary_prompt = if settings.vocabulary.is_empty() {
        None
    } else {
        Some(settings.vocabulary.join(", "))
    };

    match transcription_mode {
        TranscriptionMode::Server => {
            emit_to_overlay(app, "streaming");

            let wav_data = audio_encoder::encode_wav(&audio_data, 16000, 1)
                .map_err(|e| format!("Failed to encode WAV: {}", e))?;

            let app_for_stream = app.clone();
            let cancel_for_stream = cancel.clone();
            let on_segment = move |segment: server_transcription::TranscriptionSegment| {
                if cancel_for_stream.load(Ordering::SeqCst) {
                    return;
                }
                let _ = app_for_stream.emit_to(
                    EventTarget::webview_window("overlay"),
                    "transcription-segment",
                    &segment,
                );
            };

            // Note: detected_context.language is a programming language name (e.g. "rust",
            // "generic_dev"), NOT a Whisper language code. Pass None to let the server use
            // its configured DEFAULT_LANGUAGE.
            let request = server_transcription::transcribe(
                &server_url,
                &wav_data,
                Some(&settings.server_token),
                settings.server_model.as_deref(),
                None,
                vocabulary_prompt.as_deref(),
                on_segment,
                |_| {},
            );

            // Dropping the request closes the connection, which is the only
            // way to tell the server to stop.
            let result = tokio::select! {
                result = request => result,
                _ = cancelled(&cancel) => return Err("Cancelled".to_string()),
            };

            match result {
                Ok(text) => Ok((text, "server")),
                Err(e) => {
                    eprintln!("Server transcription failed: {}", e);

                    if !server_fallback {
                        return Err(format!("Server transcription failed: {}", e));
                    }

                    eprintln!("Falling back to local Whisper transcription");
                    emit_to_overlay(app, "transcribing");

                    match transcribe_locally(app, audio_data, vocabulary_prompt, cancel).await? {
                        Some(text) => Ok((text, "local")),
                        None => Err(format!(
                            "Server failed: {}. No local model loaded for fallback.",
                            e
                        )),
                    }
                }
            }
        }
        TranscriptionMode::Local => {
            match transcribe_locally(app, audio_data, vocabulary_prompt, cancel).await? {
                Some(text) => Ok((text, "local")),
                None => Err(NO_MODEL.to_string()),
            }
        }
    }
}

/// Resolve once the flag is raised.
async fn cancelled(flag: &AtomicBool) {
    while !flag.load(Ordering::SeqCst) {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}

/// Tell the overlay what this transcription is doing, unless a newer
/// recording has it. A transcription running behind a recording would
/// otherwise pull the overlay off the microphone while the user still talks.
fn emit_to_overlay(app: &AppHandle, processing_state: &'static str) {
    let state = app.state::<AppState>();
    // Remembered even while a recording has the overlay: it is what the transcription goes
    // back to showing, with the same icon, whenever the overlay is given back to it.
    *state.job_state.lock() = processing_state;
    if state.phase.lock().active() {
        return;
    }
    let _ = app.emit_to(EventTarget::webview_window("overlay"), "processing-state", processing_state);
}

/// Act on what the queue let go: paste what is due, and record and announce
/// every dictation whose turn has come.
fn hand_out(app: &AppHandle, release: Release, cancelled: &AtomicBool) {
    let state = app.state::<AppState>();

    #[cfg(any(windows, target_os = "linux"))]
    {
        let preserve = crate::settings::read(|s| s.preserve_clipboard);
        let (mut words, mut failures) = (0usize, 0usize);
        for text in &release.paste {
            // Read before each paste: a cancel since the turn was taken voids what has not
            // started, and the paste in the middle of its keystrokes finishes.
            if cancelled.load(Ordering::SeqCst) {
                break;
            }
            match crate::clipboard::type_text(text, preserve) {
                Ok(()) => words += text.split_whitespace().count(),
                Err(e) => {
                    eprintln!("Failed to paste the transcription: {}", e);
                    failures += 1;
                }
            }
        }

        // A dictation cancelled while it waited is not recorded either, like any other.
        if cancelled.load(Ordering::SeqCst) {
            return;
        }

        // Say what arrived, and what did not: a paste that failed never reads as one.
        let recording = state.phase.lock().active();
        let jobs = state.jobs_in_flight.load(Ordering::SeqCst);
        let said = overlay_feedback::confirmation(recording, jobs, words, failures);
        match &said {
            overlay_feedback::Confirmation::Pasted(count) => {
                let _ = app.emit_to(EventTarget::webview_window("overlay"), "dictation-pasted", count);
                release_overlay_after(app, u64::from(crate::settings::read(|s| s.overlay_look.pasted_hold_ms)));
            }
            // A refusal like any other: the state and the sound together, once. Over a recording
            // or a transcription it is brief, and the overlay then returns to what is in flight.
            overlay_feedback::Confirmation::Failed => refuse(app, "paste_failed"),
            overlay_feedback::Confirmation::Nothing => {}
        }
    }

    for transcript in release.delivered {
        // Save before announcing.
        //
        // The frontend used to do this, on the very event this line emits, so its
        // write raced the dashboard's refetch of the same event and the figures sat
        // one dictation behind. Saving here means the row is in by the time anyone
        // hears about it, and the id and the timings come from the side that knows
        // them.
        let entry = database::NewTranscription {
            id: uuid::Uuid::new_v4().to_string(),
            text: transcript.text,
            timestamp: chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            model: state.current_model.lock().clone(),
            source: transcript.source.to_string(),
            enhanced: false,
            audio_duration_ms: Some(transcript.audio_duration_ms),
            processing_time_ms: Some(transcript.processing_time_ms),
        };

        let db = app.state::<database::Database>();
        if let Err(e) = db.add_transcription(&entry) {
            eprintln!("Failed to save the transcription: {}", e);
        } else if let Err(e) = db.prune_transcriptions(crate::settings::read(|s| s.history_limit)) {
            eprintln!("Failed to prune the history: {}", e);
        }

        let _ = app.emit("transcription-complete", &entry);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tauri_plugin_global_shortcut::{Code, Modifiers};

    #[test]
    fn two_modifiers_and_a_key_parse_into_one_shortcut() {
        let parsed = parse_shortcut("Ctrl+Shift+Space").expect("should parse");

        assert_eq!(
            parsed,
            Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::Space)
        );
    }
}
