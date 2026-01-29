//! Keystroke representation and stream processing

use std::fmt;

/// Represents a single keystroke input
///
/// Language-agnostic: core doesn't interpret what keys mean.
/// That's the language pack's job.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Keystroke {
    /// The raw key code
    pub key: Key,
    /// Modifier state
    pub modifiers: Modifiers,
}

/// Physical or logical key representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    /// Printable character
    Char(char),
    /// Backspace
    Backspace,
    /// Delete
    Delete,
    /// Enter/Return
    Enter,
    /// Escape
    Escape,
    /// Tab
    Tab,
    /// Space
    Space,
    /// Arrow keys
    Arrow(ArrowDirection),
    /// Unknown key (with platform-specific code)
    Unknown(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArrowDirection {
    Up,
    Down,
    Left,
    Right,
}

/// Key modifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool, // Command/Windows key
}

impl Keystroke {
    /// Create a simple character keystroke
    pub fn char(c: char) -> Self {
        Self {
            key: Key::Char(c),
            modifiers: Modifiers::default(),
        }
    }

    /// Create a backspace keystroke
    pub fn backspace() -> Self {
        Self {
            key: Key::Backspace,
            modifiers: Modifiers::default(),
        }
    }

    /// Check if this keystroke should terminate composition
    pub fn is_terminator(&self) -> bool {
        matches!(
            self.key,
            Key::Enter | Key::Escape | Key::Arrow(_)
        )
    }

    /// Check if this is a deletion keystroke
    pub fn is_deletion(&self) -> bool {
        matches!(self.key, Key::Backspace | Key::Delete)
    }
}

impl fmt::Display for Keystroke {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.key {
            Key::Char(c) => write!(f, "{}", c),
            Key::Backspace => write!(f, "⌫"),
            Key::Delete => write!(f, "⌦"),
            Key::Enter => write!(f, "↵"),
            Key::Escape => write!(f, "⎋"),
            Key::Tab => write!(f, "⇥"),
            Key::Space => write!(f, " "),
            Key::Arrow(dir) => write!(f, "{:?}", dir),
            Key::Unknown(_) => write!(f, "?"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keystroke_char() {
        let k = Keystroke::char('a');
        assert_eq!(k.key, Key::Char('a'));
    }

    #[test]
    fn test_keystroke_backspace() {
        let k = Keystroke::backspace();
        assert_eq!(k.key, Key::Backspace);
        assert!(k.is_deletion());
    }

    #[test]
    fn test_is_terminator() {
        assert!(Keystroke { key: Key::Enter, modifiers: Modifiers::default() }.is_terminator());
        assert!(Keystroke { key: Key::Escape, modifiers: Modifiers::default() }.is_terminator());
        assert!(!Keystroke::char('a').is_terminator());
    }
}
