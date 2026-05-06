# Language Pack Authoring Guide

This guide explains how to create a new language pack for hip-key.

## Overview

A language pack is a Rust crate that implements the `LanguagePack` trait from `hip-key-core`. It provides:
- Input method rules (keystroke → character mapping)
- Candidate suggestions (dictionary-based)
- Composition validation

## Step-by-Step

### 1. Create the Crate

```
lang/
└── <lang-code>/
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

`Cargo.toml`:
```toml
[package]
name = "hip-key-lang-<code>"
version.workspace = true
edition.workspace = true

[dependencies]
hip-key-core = { path = "../../core" }
```

### 2. Implement the Trait

```rust
use hip_key_core::{Keystroke, LanguagePack, ProcessResult, CandidateList};

pub struct MyLanguage {
    // your state
}

impl LanguagePack for MyLanguage {
    fn process(&self, keystroke: &Keystroke, buffer: &str) -> ProcessResult {
        // 1. Check if keystroke modifies the buffer
        // 2. Return appropriate ProcessResult
        todo!()
    }

    fn generate_candidates(&self, buffer: &str) -> CandidateList {
        // Return suggestions based on current buffer
        vec![]
    }

    fn is_valid_composition(&self, buffer: &str) -> bool {
        // Check if buffer contains valid characters for this language
        true
    }

    fn id(&self) -> &str { "code" }
    fn name(&self) -> &str { "Language Name" }
}
```

### 3. ProcessResult Variants

| Variant | Meaning | When to use |
|---------|---------|-------------|
| `Consumed` | Keystroke handled, append char to buffer | Default for regular characters |
| `PassThrough` | Not handled, let OS process | Non-language keystrokes |
| `BufferUpdated(String)` | Buffer content replaced | Vowel modifications, tone marks |
| `Candidates(CandidateList)` | New suggestions available | Dictionary matches |
| `ReadyToCommit(String)` | Composition complete | Whitespace, punctuation triggers |

### 4. Keystroke Handling Pattern

```rust
fn process(&self, keystroke: &Keystroke, buffer: &str) -> ProcessResult {
    if let Keystroke { key: Key::Char(c), .. } = keystroke {
        // Check for special input rules first
        if let Some(result) = self.try_modification(buffer, c) {
            return ProcessResult::BufferUpdated(result);
        }

        // Check for tone/diacritical marks
        if let Some(result) = self.try_tone(buffer, c) {
            return ProcessResult::BufferUpdated(result);
        }

        // Whitespace/punctuation = commit
        if c.is_ascii_whitespace() || c.is_ascii_punctuation() {
            return ProcessResult::ReadyToCommit(buffer.to_string());
        }

        // Default: append character
        ProcessResult::Consumed
    } else {
        // Non-character keystrokes
        ProcessResult::PassThrough
    }
}
```

### 5. Add Dictionary (Optional)

Use the Trie from the Vietnamese pack as reference:

```rust
// lang/<code>/src/trie.rs - copy and adapt
// lang/<code>/src/dictionary.rs - embed word list
```

### 6. Add Tests

Every language pack must include tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversion() {
        let lang = MyLanguage::new();
        assert_eq!(lang.convert("input"), "expected");
    }

    #[test]
    fn test_keystroke_processing() {
        let mut engine = hip_key_core::Engine::new();
        engine.set_language_pack(Box::new(MyLanguage::new()));

        let event = engine.process(&Keystroke::char('a'));
        assert_eq!(event, hip_key_core::EngineEvent::BufferChanged);
    }
}
```

### 7. Register in Workspace

Add to root `Cargo.toml`:
```toml
members = [
    "core",
    "ffi",
    "lang/vi",
    "lang/<code>",  # add this
]
```

## Best Practices

- **Zero external dependencies** if possible (match existing style)
- **Embed word data** at compile time for fast startup
- **Test edge cases**: empty buffer, repeated keystrokes, mixed input
- **Handle uppercase** if your language supports it
- **Respect user intent**: never auto-correct without explicit opt-in
- **Profile hot paths**: keystroke processing must be < 100μs

## Examples

- **Vietnamese** (`lang/vi/`): Full implementation with Telex, VNI, dictionary, Trie
- Use Vietnamese as a template for similar diacritical languages

## Need Help?

Open an issue at https://github.com/hieple7985/hip-key/issues
