# Architecture

## Design Principles

These principles must not be violated:

1. **Core engine is language-agnostic** — No hardcoded Vietnamese rules in core
2. **Language logic lives in plugins** — Each language is a separate crate
3. **No forced auto-correction** — Everything optional
4. **Everything local-first** — No mandatory cloud
5. **Latency > intelligence** — Slow = failure

## Component Structure

### Core Engine (`core/`)

**Responsibilities:**
- Keystroke stream processing
- Buffer & composition state management
- Candidate generation delegation (via language pack)
- Accept/reject/undo feedback loop

**Non-responsibilities:**
- No hardcoded language rules
- No UI
- No OS-specific APIs

### Language Packs (`lang/`)

Each language pack implements the `LanguagePack` trait:

```rust
pub trait LanguagePack: Send + Sync {
    fn process(&self, keystroke: &Keystroke, buffer: &str) -> ProcessResult;
    fn generate_candidates(&self, buffer: &str) -> CandidateList;
    fn is_valid_composition(&self, buffer: &str) -> bool;
    fn id(&self) -> &str;
    fn name(&self) -> &str;
}
```

**Vietnamese Pack (`lang/vi/`):**
- Telex input rules
- VNI input rules (future)
- Dictionary data (future)
- Frequency-based ranking (future)

### FFI Layer (`ffi/`)

Provides C-compatible API for platform adapters:
- `hipkey_engine_create()`
- `hipkey_engine_destroy()`
- `hipkey_process()`
- *(more to be implemented)*

### Platform Adapters (Future)

| Platform | Technology |
|----------|------------|
| Windows  | C++/Rust (Win32/TSF) |
| macOS    | Swift/Objective-C via FFI |
| Linux    | C/Rust (IBus/Fcitx) |

## Data Flow

```
User Input
    ↓
Platform Adapter
    ↓
FFI Layer (C API)
    ↓
Core Engine (Rust)
    ↓
Language Pack
    ↓
ProcessResult → Engine
    ↓
EngineEvent → UI
```

## Testing Strategy

- **Unit tests** for each module
- **Integration tests** for engine + language pack interaction
- **CLI harness** for manual testing
- **Property-based tests** (future) for edge cases
