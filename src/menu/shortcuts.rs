use egui::{Key, Modifiers};
use std::collections::HashMap;
use std::sync::Mutex;

// Global state to track shortcut states across frames
lazy_static::lazy_static! {
    static ref SHORTCUT_STATES: Mutex<HashMap<String, bool>> = Mutex::new(HashMap::new());
}

/// Keyboard shortcut for menu items.
///
/// Combines a primary `egui::Key` with optional modifier keys. Use
/// [`KeyboardShortcut::from_string`] to parse user-friendly strings like
/// "ctrl+shift+p" or create it programmatically via [`KeyboardShortcut::new`].
#[derive(Debug, Clone)]
pub struct KeyboardShortcut {
    /// Primary key that triggers the shortcut (e.g. `Key::S`).
    pub key: Key,
    /// Modifier state required for the shortcut (Ctrl/Cmd, Alt, Shift).
    pub modifiers: Modifiers,
}

/// Parse error for shortcut strings.
///
/// Returned by [`KeyboardShortcut::from_string`] when the provided string
/// contains an unknown key, unsupported modifier, or has the wrong format.
#[derive(Debug, Clone)]
pub enum ShortcutParseError {
    /// Unknown or unsupported key token, e.g. "foobar".
    InvalidKey(String),
    /// Unknown or unsupported modifier token, e.g. "hyper".
    InvalidModifier(String),
    /// General formatting issue (e.g. empty string).
    InvalidFormat(String),
}

impl KeyboardShortcut {
    /// Create a shortcut with a primary key and no modifiers.
    pub fn new(key: Key) -> Self {
        Self {
            key,
            modifiers: Modifiers::default(),
        }
    }

    /// Create a shortcut from a simple string like "t", "ctrl+t", "ctrl+shift+t", etc.
    ///
    /// # Examples
    /// ```
    /// KeyboardShortcut::from_string("t").unwrap()
    /// KeyboardShortcut::from_string("ctrl+t").unwrap()
    /// KeyboardShortcut::from_string("ctrl+shift+t").unwrap()
    /// KeyboardShortcut::from_string("alt+f4").unwrap()
    /// ```
    pub fn from_string(shortcut: &str) -> Result<Self, ShortcutParseError> {
        let parts: Vec<&str> = shortcut.split('+').collect();

        if parts.is_empty() {
            return Err(ShortcutParseError::InvalidFormat(shortcut.to_string()));
        }

        let mut modifiers = Modifiers::default();
        let key_str = parts.last().unwrap().to_lowercase();

        // Parse modifiers
        for part in &parts[..parts.len() - 1] {
            let modifier = part.to_lowercase();
            match modifier.as_str() {
                "ctrl" | "control" => modifiers.ctrl = true,
                "alt" => modifiers.alt = true,
                "shift" => modifiers.shift = true,
                "cmd" | "meta" | "super" => modifiers.command = true,
                _ => return Err(ShortcutParseError::InvalidModifier(part.to_string())),
            }
        }

        // Parse key
        let key = match key_str.as_str() {
            // Letters
            "a" => Key::A,
            "b" => Key::B,
            "c" => Key::C,
            "d" => Key::D,
            "e" => Key::E,
            "f" => Key::F,
            "g" => Key::G,
            "h" => Key::H,
            "i" => Key::I,
            "j" => Key::J,
            "k" => Key::K,
            "l" => Key::L,
            "m" => Key::M,
            "n" => Key::N,
            "o" => Key::O,
            "p" => Key::P,
            "q" => Key::Q,
            "r" => Key::R,
            "s" => Key::S,
            "t" => Key::T,
            "u" => Key::U,
            "v" => Key::V,
            "w" => Key::W,
            "x" => Key::X,
            "y" => Key::Y,
            "z" => Key::Z,

            // Numbers
            "0" => Key::Num0,
            "1" => Key::Num1,
            "2" => Key::Num2,
            "3" => Key::Num3,
            "4" => Key::Num4,
            "5" => Key::Num5,
            "6" => Key::Num6,
            "7" => Key::Num7,
            "8" => Key::Num8,
            "9" => Key::Num9,

            // Function keys
            "f1" => Key::F1,
            "f2" => Key::F2,
            "f3" => Key::F3,
            "f4" => Key::F4,
            "f5" => Key::F5,
            "f6" => Key::F6,
            "f7" => Key::F7,
            "f8" => Key::F8,
            "f9" => Key::F9,
            "f10" => Key::F10,
            "f11" => Key::F11,
            "f12" => Key::F12,

            // Special keys
            "enter" | "return" => Key::Enter,
            "space" => Key::Space,
            "tab" => Key::Tab,
            "escape" | "esc" => Key::Escape,
            "backspace" => Key::Backspace,
            "delete" | "del" => Key::Delete,
            "home" => Key::Home,
            "end" => Key::End,
            "pageup" | "pgup" => Key::PageUp,
            "pagedown" | "pgdown" => Key::PageDown,
            "up" => Key::ArrowUp,
            "down" => Key::ArrowDown,
            "left" => Key::ArrowLeft,
            "right" => Key::ArrowRight,

            // Punctuation
            "-" | "minus" => Key::Minus,
            "=" | "plus" => Key::Equals,
            "[" => Key::OpenBracket,
            "]" => Key::CloseBracket,
            ";" => Key::Semicolon,
            "'" => Key::Quote,
            "`" => Key::Backtick,
            "\\" => Key::Backslash,
            "," => Key::Comma,
            "." => Key::Period,
            "/" => Key::Slash,

            _ => return Err(ShortcutParseError::InvalidKey(key_str)),
        };

        Ok(Self { key, modifiers })
    }

    /// Create a shortcut from a string, panicking on invalid input.
    /// Use this when you know the shortcut string is valid.
    ///
    /// # Examples
    /// ```
    /// KeyboardShortcut::parse("ctrl+t")
    /// KeyboardShortcut::parse("alt+f4")
    /// ```
    pub fn parse(shortcut: &str) -> Self {
        Self::from_string(shortcut).expect(&format!("Invalid shortcut: {}", shortcut))
    }

    /// Check if this shortcut matches the current input
    pub fn matches(&self, key: Key, modifiers: Modifiers) -> bool {
        self.key == key && self.modifiers == modifiers
    }

    /// Check if this shortcut was just pressed
    pub fn just_pressed(&self, ctx: &egui::Context) -> bool {
        // Create a unique key for this shortcut
        let shortcut_key = format!(
            "{:?}_{}_{}_{}_{}",
            self.key,
            self.modifiers.ctrl,
            self.modifiers.alt,
            self.modifiers.shift,
            self.modifiers.command
        );

        // Check if this frame the key was pressed and modifiers match
        let current_frame_pressed = ctx.input(|i| {
            let key_pressed = i.key_pressed(self.key);

            // For Ctrl shortcuts, accept either ctrl OR cmd (Windows compatibility)
            let ctrl_held = i.modifiers.ctrl || i.modifiers.command;
            let ctrl_match = if self.modifiers.ctrl {
                ctrl_held // We want Ctrl, accept either ctrl or cmd
            } else {
                !ctrl_held // We don't want Ctrl, make sure neither is held
            };

            let alt_match = i.modifiers.alt == self.modifiers.alt;
            let shift_match = i.modifiers.shift == self.modifiers.shift;

            key_pressed && ctrl_match && alt_match && shift_match
        });

        // Get previous state
        let mut states = SHORTCUT_STATES.lock().unwrap();
        let was_pressed = states.get(&shortcut_key).copied().unwrap_or(false);

        // Update state
        states.insert(shortcut_key.clone(), current_frame_pressed);

        // Return true only on transition from not pressed to pressed (just pressed)
        current_frame_pressed && !was_pressed
    }

    /// Human-readable representation like "Ctrl+Shift+P".
    pub fn display_string(&self) -> String {
        let mut result = String::new();

        if self.modifiers.ctrl {
            result.push_str("Ctrl+");
        }
        if self.modifiers.alt {
            result.push_str("Alt+");
        }
        if self.modifiers.shift {
            result.push_str("Shift+");
        }
        if self.modifiers.command {
            result.push_str("Cmd+");
        }

        result.push_str(&self.key.name());
        result
    }
}
