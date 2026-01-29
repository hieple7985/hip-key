//! Core input method engine

use crate::buffer::Buffer;
use crate::keystroke::Keystroke;
use crate::langpack::{LanguagePack, ProcessResult};
use crate::candidate::CandidateList;

/// Main input method engine
///
/// Responsibilities:
/// - Manage buffer state
/// - Route keystrokes to language pack
/// - Handle commit/undo
/// - NOT: interpret keystrokes (language pack's job)
pub struct Engine {
    buffer: Buffer,
    lang_pack: Option<Box<dyn LanguagePack>>,
    candidates: CandidateList,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            lang_pack: None,
            candidates: Vec::new(),
        }
    }

    /// Load a language pack
    pub fn set_language_pack(&mut self, pack: Box<dyn LanguagePack>) {
        self.lang_pack = Some(pack);
    }

    /// Get current language pack info
    pub fn language_pack_id(&self) -> Option<&str> {
        self.lang_pack.as_ref().map(|p| p.id())
    }

    /// Process a keystroke through the engine
    pub fn process(&mut self, keystroke: &Keystroke) -> EngineEvent {
        // Handle terminators first
        if keystroke.is_terminator() {
            return EngineEvent::Commit(self.buffer.composing().to_string());
        }

        // Handle deletions directly
        if keystroke.is_deletion() {
            match keystroke.key {
                crate::keystroke::Key::Backspace => self.buffer.backspace(),
                crate::keystroke::Key::Delete => self.buffer.delete(),
                _ => {}
            }
            return EngineEvent::BufferChanged;
        }

        // Route to language pack if available
        if let Some(pack) = &self.lang_pack {
            let result = pack.process(keystroke, self.buffer.composing());

            match result {
                ProcessResult::BufferUpdated(new_buffer) => {
                    // Language pack provided new buffer content
                    self.buffer.set_composing(&new_buffer);
                    EngineEvent::BufferChanged
                }
                ProcessResult::Consumed => {
                    // Language pack handled it, append the keystroke
                    if let crate::keystroke::Key::Char(c) = keystroke.key {
                        self.buffer.append(c);
                    }
                    EngineEvent::BufferChanged
                }
                ProcessResult::PassThrough => {
                    // Let the keystroke through as-is
                    EngineEvent::PassThrough
                }
                ProcessResult::Candidates(candidates) => {
                    self.candidates = candidates;
                    EngineEvent::CandidatesUpdated
                }
                ProcessResult::ReadyToCommit(text) => {
                    self.buffer.commit_with(&text);
                    EngineEvent::Commit(text)
                }
            }
        } else {
            // No language pack: simple passthrough
            EngineEvent::PassThrough
        }
    }

    /// Commit current composition
    pub fn commit(&mut self) -> String {
        let text = self.buffer.composing().to_string();
        self.buffer.commit();
        text
    }

    /// Get current buffer state
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Get current candidates
    pub fn candidates(&self) -> &[crate::candidate::Candidate] {
        &self.candidates
    }

    /// Clear composition state
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.candidates.clear();
    }

    /// Check if engine is idle (no active composition)
    pub fn is_idle(&self) -> bool {
        self.buffer.composing().is_empty()
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

/// Events emitted by the engine
///
/// These are consumed by the platform adapter or UI layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineEvent {
    /// Buffer composition changed
    BufferChanged,
    /// Candidates list updated
    CandidatesUpdated,
    /// Text ready to commit
    Commit(String),
    /// Keystroke should pass through unchanged
    PassThrough,
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestLanguagePack;

    impl LanguagePack for TestLanguagePack {
        fn process(&self, keystroke: &Keystroke, buffer: &str) -> ProcessResult {
            if let crate::keystroke::Key::Char(c) = keystroke.key {
                if buffer == "a" && c == 'w' {
                    return ProcessResult::ReadyToCommit(String::from("ă"));
                }
            }
            ProcessResult::Consumed
        }

        fn generate_candidates(&self, _buffer: &str) -> CandidateList {
            vec![]
        }

        fn is_valid_composition(&self, _buffer: &str) -> bool {
            true
        }

        fn id(&self) -> &str {
            "test"
        }

        fn name(&self) -> &str {
            "Test"
        }
    }

    #[test]
    fn test_engine_new() {
        let engine = Engine::new();
        assert!(engine.is_idle());
        assert!(engine.language_pack_id().is_none());
    }

    #[test]
    fn test_engine_set_language_pack() {
        let mut engine = Engine::new();
        engine.set_language_pack(Box::new(TestLanguagePack));
        assert_eq!(engine.language_pack_id(), Some("test"));
    }

    #[test]
    fn test_engine_backspace() {
        let mut engine = Engine::new();
        engine.set_language_pack(Box::new(TestLanguagePack));

        // Type "ab"
        let _ = engine.process(&Keystroke::char('a'));
        let _ = engine.process(&Keystroke::char('b'));
        assert_eq!(engine.buffer().composing(), "ab");

        // Backspace
        let _ = engine.process(&Keystroke::backspace());
        assert_eq!(engine.buffer().composing(), "a");
    }

    #[test]
    fn test_engine_commit_ready() {
        let mut engine = Engine::new();
        engine.set_language_pack(Box::new(TestLanguagePack));

        // Type 'a'
        let _ = engine.process(&Keystroke::char('a'));
        assert_eq!(engine.buffer().composing(), "a");

        // Type 'w' -> should trigger "ă" commit
        let event = engine.process(&Keystroke::char('w'));
        assert_eq!(event, EngineEvent::Commit(String::from("ă")));
        assert_eq!(engine.buffer().committed(), "ă");
    }

    #[test]
    fn test_engine_clear() {
        let mut engine = Engine::new();
        engine.set_language_pack(Box::new(TestLanguagePack));

        let _ = engine.process(&Keystroke::char('a'));
        assert!(!engine.is_idle());

        engine.clear();
        assert!(engine.is_idle());
    }

    #[test]
    fn test_engine_keystroke_by_keystroke() {
        // Test that engine processes keystrokes correctly
        let mut engine = Engine::new();

        // No language pack - pass through
        let event = engine.process(&Keystroke::char('a'));
        assert_eq!(event, EngineEvent::PassThrough);
        assert_eq!(engine.buffer().composing(), "");

        // With language pack - characters append
        engine.set_language_pack(Box::new(TestLanguagePack));
        let _ = engine.process(&Keystroke::char('x'));
        assert_eq!(engine.buffer().composing(), "x");
        let _ = engine.process(&Keystroke::char('i'));
        assert_eq!(engine.buffer().composing(), "xi");
        let _ = engine.process(&Keystroke::char('n'));
        assert_eq!(engine.buffer().composing(), "xin");
    }

    #[test]
    fn test_engine_backspace_with_buffer() {
        let mut engine = Engine::new();
        engine.set_language_pack(Box::new(TestLanguagePack));

        // Type "abc"
        let _ = engine.process(&Keystroke::char('a'));
        let _ = engine.process(&Keystroke::char('b'));
        let _ = engine.process(&Keystroke::char('c'));
        assert_eq!(engine.buffer().composing(), "abc");

        // Backspace twice
        let _ = engine.process(&Keystroke::backspace());
        assert_eq!(engine.buffer().composing(), "ab");
        let _ = engine.process(&Keystroke::backspace());
        assert_eq!(engine.buffer().composing(), "a");

        // Type again
        let _ = engine.process(&Keystroke::char('b'));
        assert_eq!(engine.buffer().composing(), "ab");
    }
}
