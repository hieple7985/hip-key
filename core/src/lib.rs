//! hip-key core engine
//!
//! Language-agnostic input method engine.

// Re-export public APIs
pub mod engine;
pub mod keystroke;
pub mod buffer;
pub mod candidate;
pub mod langpack;
pub mod config;
pub mod macro_expander;
pub mod learning;
pub mod agent;

// Core engine entry point
pub use engine::{Engine, EngineEvent};

// Common types for convenience
pub use keystroke::{Keystroke, Key, Modifiers};
pub use buffer::Buffer;
pub use candidate::{Candidate, CandidateList};
pub use langpack::{LanguagePack, ProcessResult, DynLanguagePack};
pub use config::Config;
pub use macro_expander::MacroExpander;
pub use learning::LearningStore;
pub use agent::{Agent, AgentAction, Intent, ActionResult};
