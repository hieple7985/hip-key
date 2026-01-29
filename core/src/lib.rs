//! hip-key core engine
//!
//! Language-agnostic input method engine.

// Re-export public APIs
pub mod engine;
pub mod keystroke;
pub mod buffer;
pub mod candidate;
pub mod langpack;

// Core engine entry point
pub use engine::{Engine, EngineEvent};

// Common types for convenience
pub use keystroke::{Keystroke, Key, Modifiers};
pub use buffer::Buffer;
pub use candidate::{Candidate, CandidateList};
pub use langpack::{LanguagePack, ProcessResult, DynLanguagePack};
