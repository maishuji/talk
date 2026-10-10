//! A file the application keeps for the user, found at launch in one of four
//! cases, decided once before anything reads it.
//!
//! - Absent: a first run.
//! - Healthy: it opens, parses, and every field was accepted.
//! - Cannot be opened: retried for a few seconds, all the files of the launch
//!   sharing one budget. If it still cannot, the application does not run on
//!   defaults against a file it could not read: its owner tells the user and exits.
//! - Not fully readable: it opens and does not parse, or parses and some field
//!   was refused. The original is copied aside, the user is owed a message, and
//!   its owner rewrites the file at once from what was loaded, so that the next
//!   launch finds a healthy one.
//!
//! Judging a file writes nothing: every file of the launch is judged before any
//! of them is acted on.

use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

/// How long the files that will not open are retried at launch, all of them
/// together, with a short backoff.
const OPEN_BUDGET: Duration = Duration::from_secs(5);

static OPEN_DEADLINE: OnceLock<Instant> = OnceLock::new();

/// What is left of the one budget the launch has for opening its files. The
/// first file to ask starts it, and the next ones share what remains.
pub fn launch_patience() -> Duration {
    OPEN_DEADLINE.get_or_init(|| Instant::now() + OPEN_BUDGET).saturating_duration_since(Instant::now())
}

/// What judging a file at launch found.
pub enum Judged<T> {
    Missing,
    Healthy(T),
    /// It opens and is not fully readable. The value is what could be read, none
    /// at all when the file does not parse. The bytes are the ones that were judged.
    NotFullyRead { value: Option<T>, bytes: Vec<u8> },
    /// It could not be opened for the whole time it was tried.
    CannotOpen(String),
}

/// Why the application cannot go on from a file: nothing was rewritten.
#[derive(Debug, Clone, PartialEq)]
pub enum Blocked {
    /// It could not be opened.
    CannotOpen(String),
    /// It is not fully readable and a copy of it could not be made.
    CannotKeepAside(String),
}

impl Blocked {
    pub fn reason(&self) -> &str {
        match self {
            Blocked::CannotOpen(reason) | Blocked::CannotKeepAside(reason) => reason,
        }
    }
}

/// Read the file once, retrying for `patience` a file that will not open, and
/// judge it. `parse` answers the value and whether every field of it was
/// accepted. Nothing is written.
pub fn judge_file_within<T>(
    path: &Path,
    patience: Duration,
    parse: impl FnOnce(&str) -> Result<(T, bool), String>,
) -> Judged<T> {
    let started = Instant::now();
    let mut pause = Duration::from_millis(10);
    let bytes = loop {
        match std::fs::read(path) {
            Ok(bytes) => break bytes,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Judged::Missing,
            Err(e) if started.elapsed() >= patience => return Judged::CannotOpen(e.to_string()),
            Err(_) => {
                std::thread::sleep(pause);
                pause = (pause * 2).min(Duration::from_millis(200));
            }
        }
    };
    // A file saved by PowerShell 5.1 or an old Notepad starts with a byte order mark
    // that JSON does not allow, and is as intact as any other.
    let parsed = std::str::from_utf8(&bytes)
        .map_err(|e| e.to_string())
        .and_then(|text| parse(text.strip_prefix('\u{feff}').unwrap_or(text)));
    match parsed {
        Ok((value, true)) => Judged::Healthy(value),
        Ok((value, false)) => Judged::NotFullyRead { value: Some(value), bytes },
        Err(_) => Judged::NotFullyRead { value: None, bytes },
    }
}

/// What became of a file that was not fully readable: where the original is.
#[derive(Debug, Clone, PartialEq)]
pub struct Damage {
    kept: PathBuf,
}

fn previous_of(aside: &Path) -> PathBuf {
    let mut name = aside.file_stem().unwrap_or_default().to_os_string();
    name.push(".previous.json");
    aside.with_file_name(name)
}

fn length(path: &Path) -> Option<u64> {
    std::fs::metadata(path)
        .ok()
        .filter(|meta| meta.is_file())
        .map(|meta| meta.len())
}

fn untold_of(aside_name: &str, dir: &Path) -> PathBuf {
    dir.join(format!("{}.untold", aside_name))
}

fn told_of(aside_name: &str, dir: &Path) -> PathBuf {
    dir.join(format!("{}.told", aside_name))
}

/// Write the bytes as a copy, flushed, and read its length back.
fn keep(copy: &Path, bytes: &[u8]) -> Result<(), String> {
    crate::atomic_file::write(copy, bytes).map_err(|e| e.to_string())?;
    match length(copy) {
        Some(found) if found == bytes.len() as u64 => Ok(()),
        found => Err(format!("the copy holds {:?} bytes where {} were written", found, bytes.len())),
    }
}

impl Damage {
    /// Copy the bytes that were judged beside the file under `aside_name`, keeping
    /// at most two generations and never losing the fullest copy to a smaller one:
    /// the file that damaged itself again, down to nothing, is the usual case. Then
    /// write the note that the user is owed a message. A copy that cannot be made
    /// is an error: the file is not to be rewritten without one.
    pub fn set_aside(path: &Path, bytes: &[u8], aside_name: &str, what: &str) -> Result<Self, Blocked> {
        let aside = path.with_file_name(aside_name);
        let existing = std::fs::read(&aside).ok();
        let kept = if existing.as_deref() == Some(bytes) || (bytes.is_empty() && existing.is_some()) {
            // The same damage found again, or nothing to keep: the copy there stands.
            aside
        } else {
            Self::rotate_then_copy(&aside, bytes, existing).map_err(Blocked::CannotKeepAside)?
        };
        let damage = Damage { kept };
        let fingerprint: String = Sha256::digest(bytes).iter().map(|b| format!("{:02x}", b)).collect();
        damage.owe_message(path.parent().unwrap_or(Path::new(".")), aside_name, what, &fingerprint);
        Ok(damage)
    }

    /// Nothing is deleted before the new copy is in place: the earlier copy is
    /// written as the second generation, then the new one replaces it as the first.
    fn rotate_then_copy(aside: &Path, bytes: &[u8], existing: Option<Vec<u8>>) -> Result<PathBuf, String> {
        if let Some(earlier) = existing {
            let previous = previous_of(aside);
            // The earlier copy becomes the second generation, unless the one it would
            // replace is the fuller of the two.
            if Some(earlier.len() as u64) >= length(&previous) {
                keep(&previous, &earlier).map_err(|e| format!("the earlier copy could not be kept: {}", e))?;
            }
        }
        keep(aside, bytes).map(|()| aside.to_path_buf())
    }

    /// Write the note that the message is owed, unless this very content was
    /// already told: the message is not said again at every launch.
    fn owe_message(&self, dir: &Path, aside_name: &str, what: &str, fingerprint: &str) {
        if std::fs::read_to_string(told_of(aside_name, dir)).is_ok_and(|told| told == fingerprint) {
            return;
        }
        let note = format!("{}\n{}\n{}", fingerprint, what, self.kept.display());
        if let Err(e) = std::fs::write(untold_of(aside_name, dir), note) {
            eprintln!("Could not note that the {} file's damage is still to be told: {}", what, e);
        }
    }

    pub fn kept_at(&self) -> &Path {
        &self.kept
    }
}

/// Say what is owed about a file that was not fully readable, at the first
/// launch that is not minimised, whatever the file's health by then, and not
/// again. Never called under a store's lock.
pub fn tell_owed(dir: &Path, aside_name: &str, minimised: bool) {
    let language = crate::startup_text::language();
    tell_owed_with(dir, aside_name, minimised, language, |message| {
        eprintln!("{}", message);
        crate::startup_notice::warn(message);
    });
}

fn tell_owed_with(
    dir: &Path,
    aside_name: &str,
    minimised: bool,
    language: crate::startup_text::Language,
    show: impl FnOnce(String),
) {
    let note = untold_of(aside_name, dir);
    let Ok(text) = std::fs::read_to_string(&note) else { return };
    if minimised {
        return;
    }
    let mut lines = text.splitn(3, '\n');
    let (fingerprint, what, kept) = (lines.next().unwrap_or(""), lines.next().unwrap_or(""), lines.next().unwrap_or(""));
    let _ = std::fs::write(told_of(aside_name, dir), fingerprint);
    let _ = std::fs::remove_file(&note);
    show(crate::startup_text::damage(language, what, kept));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::startup_text::Language;

    const ASIDE: &str = "w.unreadable.json";

    fn parse(text: &str) -> Result<(String, bool), String> {
        if !(text.starts_with('{') && text.ends_with('}')) {
            return Err("no".to_string());
        }
        Ok((text.to_string(), !text.contains("refused")))
    }

    fn judge(path: &Path) -> Judged<String> {
        judge_file_within(path, Duration::from_millis(30), parse)
    }

    /// Judge the file, which must not be fully readable, and keep it aside.
    fn try_damaged(path: &Path) -> Result<Damage, Blocked> {
        match judge(path) {
            Judged::NotFullyRead { bytes, .. } => Damage::set_aside(path, &bytes, ASIDE, "test"),
            _ => panic!("should not be fully read"),
        }
    }

    fn damaged(path: &Path) -> Damage {
        try_damaged(path).unwrap()
    }

    #[test]
    fn a_file_that_opens_and_does_not_parse_is_copied_aside_with_no_value() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("w.json");
        std::fs::write(&path, "{ half").unwrap();

        assert!(matches!(judge(&path), Judged::NotFullyRead { value: None, .. }));
        assert_eq!(std::fs::read_to_string(damaged(&path).kept_at()).unwrap(), "{ half");
    }

    #[test]
    fn a_file_that_parses_with_a_refused_field_is_copied_aside_with_what_was_read() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("w.json");
        std::fs::write(&path, "{ refused }").unwrap();

        match judge(&path) {
            Judged::NotFullyRead { value, .. } => assert_eq!(value.as_deref(), Some("{ refused }")),
            _ => panic!("should not be fully read"),
        }
        assert!(damaged(&path).kept_at().exists());
    }

    #[test]
    fn a_healthy_file_is_healthy_and_leaves_nothing_beside_it() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("w.json");
        std::fs::write(&path, "{ fine }").unwrap();

        assert!(matches!(judge(&path), Judged::Healthy(_)));
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
    }

    #[test]
    fn a_byte_order_mark_does_not_make_an_intact_file_unreadable() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("w.json");
        std::fs::write(&path, "\u{feff}{ fine }").unwrap();

        match judge(&path) {
            Judged::Healthy(text) => assert_eq!(text, "{ fine }"),
            _ => panic!("a file with a byte order mark is intact"),
        }
    }

    #[test]
    fn a_missing_file_is_missing() {
        let dir = tempfile::tempdir().unwrap();

        assert!(matches!(judge(&dir.path().join("w.json")), Judged::Missing));
    }

    #[test]
    fn a_file_that_cannot_be_opened_is_judged_so_and_nothing_is_written() {
        let dir = tempfile::tempdir().unwrap();
        // A directory where the file should be: opening it for reading fails.
        let path = dir.path().join("w.json");
        std::fs::create_dir(&path).unwrap();

        assert!(matches!(judge_file_within(&path, Duration::from_millis(60), parse), Judged::CannotOpen(_)));
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
    }

    #[test]
    fn a_copy_that_cannot_be_made_blocks_and_owes_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("w.json");
        std::fs::write(&path, "{ half").unwrap();
        // The place of the copy is taken by a directory.
        std::fs::create_dir(dir.path().join(ASIDE)).unwrap();

        assert!(matches!(try_damaged(&path), Err(Blocked::CannotKeepAside(_))));
        assert!(!untold_of(ASIDE, dir.path()).exists());
    }

    #[test]
    fn the_copy_is_made_from_the_bytes_that_were_judged() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("w.json");
        std::fs::write(&path, "{ judged").unwrap();
        let Judged::NotFullyRead { bytes, .. } = judge(&path) else { panic!("should not be fully read") };
        std::fs::write(&path, "{ changed since }").unwrap();

        let damage = Damage::set_aside(&path, &bytes, ASIDE, "test").unwrap();

        assert_eq!(std::fs::read(damage.kept_at()).unwrap(), b"{ judged");
        assert!(!dir.path().join("w.unreadable.json.tmp").exists());
    }

    #[test]
    fn a_second_generation_that_cannot_be_kept_blocks_and_the_earlier_copy_stays() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("w.json");
        std::fs::write(&path, "{ the first and fullest damaged content").unwrap();
        let aside = damaged(&path).kept_at().to_path_buf();
        std::fs::create_dir(previous_of(&aside)).unwrap();
        std::fs::write(&path, "{ second").unwrap();

        assert!(matches!(try_damaged(&path), Err(Blocked::CannotKeepAside(_))));
        assert_eq!(std::fs::read_to_string(&aside).unwrap(), "{ the first and fullest damaged content");
    }

    #[test]
    fn an_empty_file_does_not_replace_a_copy_that_holds_something() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("w.json");
        std::fs::write(&path, "{ the token and the address").unwrap();
        let aside = damaged(&path).kept_at().to_path_buf();

        std::fs::write(&path, "").unwrap();
        damaged(&path);

        assert_eq!(std::fs::read_to_string(&aside).unwrap(), "{ the token and the address");
    }

    #[test]
    fn two_generations_are_kept_and_the_fullest_is_never_lost() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("w.json");
        std::fs::write(&path, "{ the first and fullest damaged content").unwrap();
        let aside = damaged(&path).kept_at().to_path_buf();
        let previous = previous_of(&aside);

        std::fs::write(&path, "{ second").unwrap();
        damaged(&path);
        assert_eq!(std::fs::read_to_string(&aside).unwrap(), "{ second");
        assert_eq!(std::fs::read_to_string(&previous).unwrap(), "{ the first and fullest damaged content");

        std::fs::write(&path, "{ x").unwrap();
        damaged(&path);
        assert_eq!(std::fs::read_to_string(&aside).unwrap(), "{ x");
        assert_eq!(std::fs::read_to_string(&previous).unwrap(), "{ the first and fullest damaged content");
    }

    #[test]
    fn the_same_damage_found_again_changes_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("w.json");
        std::fs::write(&path, "{ same").unwrap();
        let aside = damaged(&path).kept_at().to_path_buf();

        damaged(&path);

        assert!(!previous_of(&aside).exists());
        assert_eq!(std::fs::read_to_string(&aside).unwrap(), "{ same");
    }

    #[test]
    fn the_message_is_owed_from_the_moment_of_the_damage_until_a_visible_launch_shows_it() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("w.json");
        std::fs::write(&path, "{ half").unwrap();
        damaged(&path);

        // A minimised start says nothing and keeps the debt, even once the file is healthy.
        std::fs::write(&path, "{}").unwrap();
        let mut shown = Vec::new();
        tell_owed_with(dir.path(), ASIDE, true, Language::English, |m| shown.push(m));
        assert!(shown.is_empty());

        tell_owed_with(dir.path(), ASIDE, false, Language::English, |m| shown.push(m));
        assert_eq!(shown.len(), 1);
        assert!(shown[0].contains(ASIDE));

        tell_owed_with(dir.path(), ASIDE, false, Language::English, |m| shown.push(m));
        assert_eq!(shown.len(), 1);
    }

    #[test]
    fn the_message_is_worded_when_it_is_told_in_the_language_of_that_launch() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("w.json");
        std::fs::write(&path, "{ half").unwrap();
        damaged(&path);
        let mut shown = Vec::new();

        tell_owed_with(dir.path(), ASIDE, false, Language::French, |m| shown.push(m));

        assert!(shown[0].contains("conservé sous"));
        assert!(shown[0].contains(ASIDE));
    }

    #[test]
    fn the_same_content_found_again_after_it_was_told_is_not_told_twice() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("w.json");
        std::fs::write(&path, "{ half").unwrap();
        damaged(&path);
        let mut shown = Vec::new();
        tell_owed_with(dir.path(), ASIDE, false, Language::English, |m| shown.push(m));

        damaged(&path);
        tell_owed_with(dir.path(), ASIDE, false, Language::English, |m| shown.push(m));

        assert_eq!(shown.len(), 1);
    }
}
