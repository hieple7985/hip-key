# Contributing to hip-key

Thank you for your interest in contributing! This guide covers everything you need to get started.

## Quick Start

```bash
git clone https://github.com/hieple7985/hip-key.git
cd hip-key
cargo build
cargo test
```

## Project Structure

```
hip-key/
├── core/          # Language-agnostic engine (no language-specific code here)
├── lang/vi/       # Vietnamese language pack (Telex, VNI, dictionary)
├── ffi/           # C-compatible FFI layer for platform adapters
├── cli/           # CLI testing harness
├── platform/      # Platform-specific adapters (macOS, Windows, Linux)
└── docs/          # Architecture and planning docs
```

## Core Principles

Before contributing, understand these principles (see `docs/principles.md`):

1. **Core engine is language-agnostic** — No Vietnamese rules in `core/`
2. **Language packs are independent** — Each language is a separate crate
3. **No forced auto-correction** — Everything smart must be optional
4. **Local-first** — No mandatory cloud/network
5. **Latency > intelligence** — Slow IME is broken IME

## Development Workflow

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/your-feature`
3. Make changes with tests
4. Run `cargo test` to verify
5. Run `cargo clippy` for lint
6. Submit a Pull Request

## Code Style

- Rust edition 2021, stable toolchain
- No unnecessary comments — code should be self-documenting
- No external dependencies in `core/` unless absolutely necessary
- Every public API must have tests
- Use `#[cfg(test)]` modules for unit tests

## Commit Messages

Format: `type: description #issue`

```
feat: add new feature #123
fix: resolve bug in buffer #124
docs: update contributor guide #125
refactor: simplify trie lookup #126
test: add edge case tests for VNI #127
```

## Testing

```bash
cargo test                    # All tests
cargo test -p hip-key-core    # Core only
cargo test -p hip-key-lang-vi # Vietnamese only
cargo bench                   # Performance benchmarks
```

Every PR must:
- Pass all existing tests
- Include tests for new functionality
- Not regress benchmark performance

## Adding a New Language Pack

See `docs/language-pack-guide.md` for a detailed guide.

Quick summary:
1. Create `lang/<code>/` with `Cargo.toml` depending on `hip-key-core`
2. Implement the `LanguagePack` trait
3. Add input method rules
4. Add dictionary data
5. Add tests

## Reporting Issues

- Use GitHub Issues
- Include: OS, Rust version, steps to reproduce
- For bugs: include expected vs actual behavior

## License

By contributing, you agree that your contributions will be licensed under MIT OR Apache-2.0.
