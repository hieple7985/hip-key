# AGENTS.md

Guidance for AI agents (and human contributors) working in the `hip-key` repository.

`hip-key` is a language-agnostic input method engine (IME) written in Rust, with the initial focus on Vietnamese (Telex/VNI). Local-first, low-latency, no mandatory cloud. Designed for longevity (10+ year horizon).

## Repo Layout

```
hip-key/
├── core/          # hip-key-core — language-agnostic engine (NO language rules here)
│   └── src/
│       ├── engine.rs           # Engine + EngineEvent (buffer state, keystroke routing)
│       ├── keystroke.rs        # Keystroke, Key, Modifiers
│       ├── buffer.rs           # Composition buffer (insert/backspace/delete)
│       ├── candidate.rs        # Candidate, CandidateList
│       ├── langpack.rs         # LanguagePack trait, ProcessResult, DynLanguagePack
│       ├── config.rs           # Config (toggles for spell-check, agent, macros…)
│       ├── macro_expander.rs   # Snippet/macro expansion (opt-in)
│       ├── learning.rs         # LearningStore (frequency, accept/reject feedback)
│       ├── agent.rs            # Intent detection + action automation (time/date/calc…)
│       └── spell.rs            # SpellCorrector + Correction/ErrorType
├── lang/vi/       # hip-key-lang-vi — Vietnamese pack (Telex, VNI, dictionary, trie)
├── ffi/           # hip-key-ffi — C-compatible API (cdylib + staticlib) + hip-key.h
├── cli/           # hip-key-cli — `hip-key` testing harness
├── platform/macos/  # Xcode project (InputMethodKit/Swift) + build-ffi.sh
└── docs/          # architecture.md, principles.md, language-pack-guide.md
```

Workspace is declared in the root `Cargo.toml` (members: `core`, `ffi`, `lang/vi`, `cli`). Edition 2021, stable toolchain, MIT OR Apache-2.0.

## Common Commands

```bash
cargo build                           # Build whole workspace
cargo test                            # All tests across workspace
cargo test -p hip-key-core            # Core crate only
cargo test -p hip-key-lang-vi         # Vietnamese pack only
cargo clippy --all-targets -- -D warnings   # Lint (treat warnings as errors)
cargo bench -p hip-key-lang-vi        # Benchmarks (criterion)
cargo run --bin hip-key               # CLI harness (Telex)
cargo run --bin hip-key -- vni        # CLI harness (VNI)

# macOS platform adapter (requires Xcode + XcodeGen)
bash platform/macos/build-ffi.sh      # Builds FFI, stages artifacts into Frameworks/
```

## Inviolable Principles (do NOT violate these)

These hold for the project's lifetime. See `docs/principles.md` for the full rationale.

1. **Core is language-agnostic.** Never put Vietnamese (or any language) rules, dictionaries, or assumptions inside `core/`. All language logic lives in `lang/<code>/` crates via the `LanguagePack` trait.
2. **Language packs are independent crates.** No cross-pollination; core must not depend on a specific language pack.
3. **No forced auto-correction.** Everything "smart" (spell-check, candidates, macros, agent actions) must be opt-in via `Config` and respect user intent. Never silently "fix" user input.
4. **Local-first, no mandatory cloud.** Engine must work fully offline. Any network feature must be explicitly opt-in.
5. **Latency > intelligence.** Keep the keystroke hot path allocation-free where possible. No blocking or network calls before the UI update. Profile, don't prematurely optimize.

When in doubt about a design choice, apply the decision framework in `docs/principles.md`.

## Code Conventions

- Rust edition 2021, stable toolchain.
- **No unnecessary comments** — code is self-documenting; add comments only for non-obvious design intent (existing files use `//!` module docs and a few inline `//` notes).
- **No external dependencies in `core/`** unless absolutely necessary; it currently has zero runtime deps.
- Every public API has tests. Use `#[cfg(test)]` modules for unit tests.
- Match the existing module style: small focused files, trait-based extension points (`LanguagePack`, `AgentAction`).
- Prefer `&str` / `Cow<str>` over owned `String` on hot paths.

## Commit Messages

Format: `type: description #issue` (Conventional Commits style).

```
feat: add new feature #123
fix: resolve bug in buffer #124
docs: update contributor guide #125
refactor: simplify trie lookup #126
test: add edge case tests for VNI #127
```

Scopes are allowed when helpful (e.g. `fix(macos): ...`, `feat(core): ...`).

## Workflow

1. Fork → branch off `main` as `feat/...`, `fix/...`, etc.
2. Make changes with tests.
3. `cargo test` + `cargo clippy --all-targets -- -D warnings` must pass.
4. Ensure benchmarks do not regress (`cargo bench`).
5. Open a Pull Request against `main`.

## Adding a Language Pack

See `docs/language-pack-guide.md`. Summary:

1. Create `lang/<code>/` with `Cargo.toml` depending on `hip-key-core`.
2. Implement the `LanguagePack` trait (`process`, `generate_candidates`, `is_valid_composition`, `id`, `name`).
3. Add input-method rules and dictionary data.
4. Add tests + a criterion bench if relevant.

## FFI / Platform Bridge Notes

- `ffi/` exposes a C API (`hip-key.h`) compiled as `cdylib` + `staticlib`. When changing the public C surface, update both `ffi/src/lib.rs` and `ffi/hip-key.h` together.
- The macOS adapter (`platform/macos/`) is an Xcode project generated from `project.yml` (XcodeGen). It links `libhip_key_ffi.a` statically and bridges via `HipKeyBridge.h`. Re-run `build-ffi.sh` after any FFI change before opening Xcode.

## Things to Avoid

- Do not add Vietnamese-specific constants/maps/dictionaries in `core/`.
- Do not introduce runtime dependencies into `core/` casually — discuss first.
- Do not add comments that restate what the code obviously does.
- Do not commit `target/`, `.claude/`, `.DS_Store`, or editor swap files (see `.gitignore`).
