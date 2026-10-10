#[cfg(windows)]
use clipboard_win::{formats, raw, set_clipboard};
#[cfg(any(windows, target_os = "linux"))]
use enigo::{Enigo, Keyboard, Settings};
#[cfg(windows)]
use enigo::Key;
use thiserror::Error;
#[cfg(windows)]
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, VIRTUAL_KEY, VK_CONTROL, VK_LWIN, VK_MENU, VK_RWIN, VK_SHIFT,
};

#[derive(Error, Debug)]
pub enum ClipboardError {
    #[cfg(windows)]
    #[error("Failed to set clipboard: {0}")]
    SetClipboard(String),
    #[error("Failed to simulate input: {0}")]
    SimulateInput(String),
}

#[cfg(windows)]
struct ClipboardSnapshot {
    formats: Vec<(u32, Vec<u8>)>,
}

/// Save all clipboard formats (except GDI handle-based ones) into a snapshot.
#[cfg(windows)]
fn save_clipboard() -> Option<ClipboardSnapshot> {
    // Formats based on GDI handles - cannot be read with get_vec
    const HANDLE_FORMATS: &[u32] = &[2, 3, 14]; // CF_BITMAP, CF_METAFILEPICT, CF_ENHMETAFILE

    if raw::open().is_err() {
        return None;
    }

    let mut formats = Vec::new();
    for format_id in raw::EnumFormats::new() {
        if HANDLE_FORMATS.contains(&format_id) {
            continue;
        }
        let mut data = Vec::new();
        if raw::get_vec(format_id, &mut data).is_ok() && !data.is_empty() {
            formats.push((format_id, data));
        }
    }

    let _ = raw::close();

    if formats.is_empty() {
        None
    } else {
        Some(ClipboardSnapshot { formats })
    }
}

/// Restore all clipboard formats from a snapshot, replacing current clipboard content.
#[cfg(windows)]
fn restore_clipboard(snapshot: ClipboardSnapshot) {
    if raw::open().is_err() {
        return;
    }

    let _ = raw::empty();

    for (format_id, data) in &snapshot.formats {
        let _ = raw::set_without_clear(*format_id, data);
    }

    let _ = raw::close();
}

#[cfg(windows)]
pub fn copy_and_paste(text: &str) -> Result<(), ClipboardError> {
    // Copy to clipboard
    set_clipboard(formats::Unicode, text)
        .map_err(|e| ClipboardError::SetClipboard(e.to_string()))?;

    // Small delay to ensure clipboard is updated
    std::thread::sleep(std::time::Duration::from_millis(50));

    simulate_paste()?;

    Ok(())
}

/// The modifiers that would turn the paste into something else if still held.
#[cfg(windows)]
const PASTE_MODIFIERS: [VIRTUAL_KEY; 5] = [VK_CONTROL, VK_SHIFT, VK_MENU, VK_LWIN, VK_RWIN];

/// Whether any of them is physically down right now.
#[cfg(windows)]
fn modifiers_held() -> bool {
    PASTE_MODIFIERS
        .iter()
        .any(|key| unsafe { GetAsyncKeyState(key.0 as i32) } as u16 & 0x8000 != 0)
}

/// Wait for the hand to leave the keyboard before the paste goes out.
///
/// A global shortcut fires on the key down, so a paste following it at once
/// arrives while its own modifiers are still held and the window in front reads
/// Ctrl+Shift+V, which is a different command in most editors and browsers.
/// Waiting costs nothing when nothing is held, which is the dictation case.
/// Past the deadline the modifiers are released synthetically: somebody leaning
/// on the key still has to get their text, and a real release afterwards is a
/// key-up for a key the window already believes is up.
#[cfg(windows)]
fn wait_for_modifiers(enigo: &mut Enigo) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(600);
    while modifiers_held() && std::time::Instant::now() < deadline {
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    if !modifiers_held() {
        return;
    }

    for key in [Key::Control, Key::Shift, Key::Alt, Key::Meta] {
        let _ = enigo.key(key, enigo::Direction::Release);
    }
}

#[cfg(windows)]
fn simulate_paste() -> Result<(), ClipboardError> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| ClipboardError::SimulateInput(e.to_string()))?;

    wait_for_modifiers(&mut enigo);

    enigo
        .key(Key::Control, enigo::Direction::Press)
        .map_err(|e| ClipboardError::SimulateInput(e.to_string()))?;
    enigo
        .key(Key::Unicode('v'), enigo::Direction::Click)
        .map_err(|e| ClipboardError::SimulateInput(e.to_string()))?;
    enigo
        .key(Key::Control, enigo::Direction::Release)
        .map_err(|e| ClipboardError::SimulateInput(e.to_string()))?;

    Ok(())
}

/// Copy text to clipboard and paste, optionally preserving the original clipboard content.
/// If `preserve_clipboard` is true, saves all clipboard formats, pastes, then restores them.
#[cfg(windows)]
pub fn type_text(text: &str, preserve_clipboard: bool) -> Result<(), ClipboardError> {
    if !preserve_clipboard {
        return copy_and_paste(text);
    }

    let saved = save_clipboard();

    // The user's clipboard comes back whether or not the paste went through: an error in it
    // would otherwise leave the dictation sitting in their clipboard.
    let pasted = copy_and_paste(text);

    if pasted.is_ok() {
        // Wait for paste to be processed by the target application
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    match saved {
        Some(snapshot) => restore_clipboard(snapshot),
        None => {
            // Clipboard was empty, clear it again
            if raw::open().is_ok() {
                let _ = raw::empty();
                let _ = raw::close();
            }
        }
    }

    pasted
}

#[cfg(target_os = "linux")]
pub fn copy_and_paste(text: &str) -> Result<(), ClipboardError> {
    // Enigo's X11 text entry writes to the focused window without replacing the
    // user's clipboard. A missing X11 session fails instead of silently losing text.
    let mut keyboard = Enigo::new(&Settings::default())
        .map_err(|e| ClipboardError::SimulateInput(e.to_string()))?;
    keyboard
        .text(text)
        .map_err(|e| ClipboardError::SimulateInput(e.to_string()))
}

#[cfg(target_os = "linux")]
pub fn type_text(text: &str, _preserve_clipboard: bool) -> Result<(), ClipboardError> {
    copy_and_paste(text)
}

#[cfg(not(any(windows, target_os = "linux")))]
pub fn copy_and_paste(_text: &str) -> Result<(), ClipboardError> {
    Err(ClipboardError::SimulateInput("Text entry is unavailable on this platform".into()))
}

#[cfg(not(any(windows, target_os = "linux")))]
pub fn type_text(_text: &str, _preserve_clipboard: bool) -> Result<(), ClipboardError> {
    Err(ClipboardError::SimulateInput("Text entry is unavailable on this platform".into()))
}

