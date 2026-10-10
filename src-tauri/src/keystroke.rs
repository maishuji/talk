#[cfg(any(windows, target_os = "linux"))]
use enigo::{Direction, Enigo, Key, Keyboard, Settings};

/// Parse a shortcut string like "Ctrl+Shift+M" and simulate the keypress.
#[cfg(any(windows, target_os = "linux"))]
pub fn simulate_keystroke(keys: &str) -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("Failed to create enigo instance: {e}"))?;

    let parts: Vec<&str> = keys.split('+').map(|s| s.trim()).collect();
    let mut modifiers: Vec<Key> = Vec::new();
    let mut main_key: Option<Key> = None;

    for part in &parts {
        match part.to_lowercase().as_str() {
            "ctrl" | "control" => modifiers.push(Key::Control),
            "alt" => modifiers.push(Key::Alt),
            "shift" => modifiers.push(Key::Shift),
            "meta" | "win" | "super" => modifiers.push(Key::Meta),
            other => {
                main_key = Some(parse_key(other)?);
            }
        }
    }

    let key = main_key.ok_or_else(|| "No main key found in shortcut".to_string())?;

    // Press modifiers
    for m in &modifiers {
        enigo.key(*m, Direction::Press)
            .map_err(|e| format!("Failed to press modifier: {e}"))?;
    }

    // Press and release main key
    enigo.key(key, Direction::Click)
        .map_err(|e| format!("Failed to press key: {e}"))?;

    // Release modifiers in reverse
    for m in modifiers.iter().rev() {
        enigo.key(*m, Direction::Release)
            .map_err(|e| format!("Failed to release modifier: {e}"))?;
    }

    Ok(())
}

#[cfg(any(windows, target_os = "linux"))]
fn parse_key(s: &str) -> Result<Key, String> {
    match s.to_lowercase().as_str() {
        "space" => Ok(Key::Space),
        "enter" | "return" => Ok(Key::Return),
        "tab" => Ok(Key::Tab),
        "escape" | "esc" => Ok(Key::Escape),
        "backspace" => Ok(Key::Backspace),
        "delete" | "del" => Ok(Key::Delete),
        "up" => Ok(Key::UpArrow),
        "down" => Ok(Key::DownArrow),
        "left" => Ok(Key::LeftArrow),
        "right" => Ok(Key::RightArrow),
        "f1" => Ok(Key::F1),
        "f2" => Ok(Key::F2),
        "f3" => Ok(Key::F3),
        "f4" => Ok(Key::F4),
        "f5" => Ok(Key::F5),
        "f6" => Ok(Key::F6),
        "f7" => Ok(Key::F7),
        "f8" => Ok(Key::F8),
        "f9" => Ok(Key::F9),
        "f10" => Ok(Key::F10),
        "f11" => Ok(Key::F11),
        "f12" => Ok(Key::F12),
        c if c.len() == 1 => {
            let ch = c.chars().next().unwrap();
            Ok(Key::Unicode(ch))
        }
        other => Err(format!("Unknown key: {other}")),
    }
}

#[cfg(not(any(windows, target_os = "linux")))]
pub fn simulate_keystroke(_keys: &str) -> Result<(), String> {
    Err("Keystroke simulation is unavailable on this platform".to_string())
}
