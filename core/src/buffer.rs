//! Composition buffer management

/// The active composition buffer
///
/// Holds the current state of text being composed.
/// Language-agnostic: stores what user typed, interpretation is up to language pack.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Buffer {
    /// Raw committed text (already accepted by user)
    committed: String,
    /// Text currently being composed
    composing: String,
    /// Cursor position within composing text
    cursor: usize,
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            committed: String::new(),
            composing: String::new(),
            cursor: 0,
        }
    }

    /// Get committed text
    pub fn committed(&self) -> &str {
        &self.committed
    }

    /// Get composing text
    pub fn composing(&self) -> &str {
        &self.composing
    }

    /// Get cursor position in composing text
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Append to composing text
    pub fn append(&mut self, ch: char) {
        self.composing.insert(self.cursor, ch);
        self.cursor = self.composing.len();
    }

    /// Delete character before cursor (backspace)
    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.composing.remove(self.cursor);
        }
    }

    /// Delete character at cursor (delete key)
    pub fn delete(&mut self) {
        if self.cursor < self.composing.len() {
            self.composing.remove(self.cursor);
        }
    }

    /// Move cursor
    pub fn move_cursor(&mut self, pos: usize) {
        self.cursor = pos.min(self.composing.len());
    }

    /// Commit composing text
    pub fn commit(&mut self) {
        if !self.composing.is_empty() {
            self.committed.push_str(&self.composing);
            self.composing.clear();
            self.cursor = 0;
        }
    }

    /// Commit with specific text
    pub fn commit_with(&mut self, text: &str) {
        self.committed.push_str(text);
        self.composing.clear();
        self.cursor = 0;
    }

    /// Clear composing text without committing
    pub fn clear(&mut self) {
        self.composing.clear();
        self.cursor = 0;
    }

    /// Get full display text
    pub fn display(&self) -> String {
        format!("{}[{}]", self.committed, self.composing)
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.committed.is_empty() && self.composing.is_empty()
    }

    /// Total length
    pub fn len(&self) -> usize {
        self.committed.len() + self.composing.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_append() {
        let mut buf = Buffer::new();
        buf.append('a');
        buf.append('b');
        assert_eq!(buf.composing(), "ab");
        assert_eq!(buf.cursor(), 2);
    }

    #[test]
    fn test_buffer_backspace() {
        let mut buf = Buffer::new();
        buf.append('a');
        buf.append('b');
        buf.backspace();
        assert_eq!(buf.composing(), "a");
    }

    #[test]
    fn test_buffer_commit() {
        let mut buf = Buffer::new();
        buf.append('x');
        buf.commit();
        assert_eq!(buf.committed(), "x");
        assert!(buf.composing().is_empty());
    }

    #[test]
    fn test_buffer_commit_with() {
        let mut buf = Buffer::new();
        buf.append('x');
        buf.commit_with("y");
        assert_eq!(buf.committed(), "y");
        assert!(buf.composing().is_empty());
    }
}
