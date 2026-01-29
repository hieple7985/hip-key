# hip-key

A language-agnostic input method engine (IME) designed for longevity and modularity.

## Overview

hip-key is an open-source, long-term IME project focusing on keyboard input and typing engines. Initial target is Vietnamese, with long-term support for other ambiguous/diacritic-based languages.

**Philosophy:** local-first, low-latency, respectful of user intent.

## Architecture

```
hip-key/
├── core/      # Language-agnostic engine (Rust)
├── ffi/       # C-compatible API for platform adapters
├── lang/
│   └── vi/    # Vietnamese language pack (Telex, VNI)
├── cli/       # Testing harness
└── docs/      # Architecture documentation
```

### Core Principles

1. **Language-agnostic core** — Input rules live in language packs
2. **No forced auto-correction** — Everything "smart" must be optional
3. **Local-first** — No mandatory cloud services
4. **Latency > intelligence** — Slow is failure

## Quick Start

### CLI Testing

```bash
# Build and run the CLI harness
cargo run --bin hip-key

# Or build release
cargo build --release --bin hip-key
./target/release/hip-key
```

### Library Usage

```rust
use hip_key_core::{Engine, Keystroke};
use hip_key_lang_vi::{Vietnamese, InputMethod};

let mut engine = Engine::new();
engine.set_language_pack(Box::new(Vietnamese::with_method(InputMethod::Telex)));

// Process keystrokes
let event = engine.process(&Keystroke::char('a'));
```

## Vietnamese Language Pack

### Telex Input

| Sequence | Result |
|----------|--------|
| aw       | ă      |
| aa       | â      |
| ow       | ơ      |
| oo       | ô      |
| uw       | ư      |
| dd       | đ      |
| ee       | ê      |

*(More tone mark rules coming soon)*

### VNI Input

*(Not yet implemented)*

## Status

- ✅ Core engine skeleton
- ✅ Vietnamese language pack (basic Telex)
- ✅ CLI testing harness
- ⏳ Full Telex rules (tone marks)
- ⏳ VNI input method
- ⏳ Dictionary-based candidates
- ⏳ Platform adapters (Windows/macOS/Linux)

## License

MIT OR Apache-2.0

## Repository

https://github.com/hieple7985/hip-key
