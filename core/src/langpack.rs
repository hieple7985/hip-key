//! Language pack interface
//!
//! Language packs implement this trait to provide:
//! - Input method rules (Telex, VNI, etc.)
//! - Candidate generation
//! - Optional context/ranking

use crate::keystroke::Keystroke;
use crate::candidate::CandidateList;

/// Result of processing a keystroke
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessResult {
    /// Keystroke consumed, composition modified
    Consumed,
    /// Keystroke not handled, should pass through
    PassThrough,
    /// Buffer content updated (language pack provides new buffer content)
    BufferUpdated(String),
    /// Candidate list updated
    Candidates(CandidateList),
    /// Composition ready to commit
    ReadyToCommit(String),
}

/// Language pack trait
///
/// All language-specific logic lives here.
/// Core engine is agnostic to how language packs work internally.
pub trait LanguagePack: Send + Sync {
    /// Process a keystroke and return result
    ///
    /// The buffer contains current composing text.
    /// Language pack decides:
    /// - Does this keystroke modify composition?
    /// - Should we generate candidates?
    /// - Should we commit?
    fn process(&self, keystroke: &Keystroke, buffer: &str) -> ProcessResult;

    /// Generate candidates for current buffer
    ///
    /// Called explicitly (e.g., user presses suggestion key).
    fn generate_candidates(&self, buffer: &str) -> CandidateList;

    /// Check if buffer contains valid composition
    fn is_valid_composition(&self, buffer: &str) -> bool;

    /// Get language pack identifier
    fn id(&self) -> &str;

    /// Get language pack display name
    fn name(&self) -> &str;

    /// Optional: Get version
    fn version(&self) -> &str {
        "0.1.0"
    }
}

/// Dynamic language pack for runtime loading
pub type DynLanguagePack = dyn LanguagePack;

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyLanguagePack;

    impl LanguagePack for DummyLanguagePack {
        fn process(&self, _keystroke: &Keystroke, _buffer: &str) -> ProcessResult {
            ProcessResult::PassThrough
        }

        fn generate_candidates(&self, _buffer: &str) -> CandidateList {
            vec![]
        }

        fn is_valid_composition(&self, _buffer: &str) -> bool {
            true
        }

        fn id(&self) -> &str {
            "dummy"
        }

        fn name(&self) -> &str {
            "Dummy"
        }
    }

    #[test]
    fn test_language_pack_trait() {
        let pack = DummyLanguagePack;
        assert_eq!(pack.id(), "dummy");
        assert_eq!(pack.name(), "Dummy");
    }
}
