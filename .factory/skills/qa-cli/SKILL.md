---
name: qa-cli
description: >
  QA tests for the hip-key CLI harness (Rust binary). Tests Telex/VNI input
  methods, tone marks, candidate suggestions, method switching, and agent
  actions via interactive TUI testing with tuistory.
---

# qa-cli — hip-key CLI Testing

Tests the `hip-key` binary (`cli/src/main.rs`) by launching it via tuistory
and sending real keystrokes. The CLI is an interactive REPL: it reads lines
from stdin and prints converted Vietnamese text.

## Testing Target

hip-key has NO remote deployments and NO preview URLs. Always test locally:

1. Build the binary: `cargo build --bin hip-key`
2. Binary location: `target/debug/hip-key`
3. Launch via tuistory (see droid-control skill)

In CI, prefix the launch with `env -u CI` to avoid any CI-detection quirks.

## Build & Launch

Use the `droid-control` skill for all tuistory interactions.

```bash
# Build first (pre-flight)
cargo build --bin hip-key

# Launch the CLI in a tuistory session
tuistory launch "target/debug/hip-key" -s qa-test --cols 110 --rows 36
# For VNI mode:
tuistory launch "target/debug/hip-key -- vni" -s qa-test-vni --cols 110 --rows 36
```

Wait for the prompt `[Telex] > ` or `[VNI] > ` to appear before sending input.

## Available Test Flows

The orchestrator picks only the flows relevant to the current diff. Each flow
below is a labeled menu item, not a checklist.

### Flow 1: Telex Vowel Modification

Verifies `aw→ă`, `aa→â`, `ow→ơ`, `oo→ô`, `uw→ư`, `dd→đ`, `ee→ê`.

Steps:
1. Launch CLI in Telex mode.
2. Type `aw` → expect output containing `ă`.
3. Type `aa` → expect output containing `â`.
4. Type `dd` → expect output containing `đ`.
5. Type `ee` → expect output containing `ê`.
6. Capture a snapshot after each conversion.

Relevant when: `core/src/engine.rs`, `core/src/buffer.rs`, `lang/vi/src/lib.rs`
change, or any Telex-related diff.

### Flow 2: Telex Tone Marks

Verifies `s→sắc`, `f→huyền`, `j→hỏi`, `r→ngã`, `x→nặng`.

Steps:
1. Launch CLI in Telex mode.
2. Type `as` → expect `á`.
3. Type `af` → expect `à`.
4. Type `aj` → expect `ả`.
5. Type `ar` → expect `ã`.
6. Type `ax` → expect `ạ`.
7. Test combined: type `aws` → expect `ắ`.
8. Capture snapshots.

Relevant when: `lang/vi/src/lib.rs` tone-mark logic changes.

### Flow 3: VNI Input

Verifies `a8→ă`, `a6→â`, `o7→ơ`, `o6→ô`, `u7→ư`, `d9→đ`, `e6→ê`, and tone
digits `1-5`.

Steps:
1. Launch CLI with `-- vni` argument (or type `m` to switch).
2. Type `a8` → expect `ă`.
3. Type `a6` → expect `â`.
4. Type `d9` → expect `đ`.
5. Type `a1` → expect `á`.
6. Type `a2` → expect `à`.
7. Capture snapshots.

Relevant when: `lang/vi/src/lib.rs` VNI logic changes.

### Flow 4: Backspace and Commit

Verifies buffer editing and commit behavior.

Steps:
1. Launch CLI in Telex mode.
2. Type `xin`, then Backspace → expect `xi`.
3. Type Enter → expect commit of `xi`.
4. Type `chaof` → expect `chào`.
5. Capture snapshots.

Relevant when: `core/src/buffer.rs`, `core/src/engine.rs` change.

### Flow 5: Candidate Suggestions

Verifies the `s` command shows ranked candidates with confidence bars.

Steps:
1. Launch CLI in Telex mode.
2. Type some text, then type `s` on a new line.
3. Expect numbered candidate list (1-9) with confidence percentages and bars.
4. Verify at least one candidate appears for common prefixes.
5. Capture snapshot of the candidate list.

Relevant when: `cli/src/main.rs`, `core/src/candidate.rs`,
`lang/vi/src/dictionary.rs` change.

### Flow 6: Method Switching

Verifies the `m` command toggles Telex/VNI.

Steps:
1. Launch CLI in Telex mode (prompt shows `[Telex]`).
2. Type `m` → expect prompt to change to `[VNI]`.
3. Type `m` again → expect prompt to change back to `[Telex]`.
4. Capture snapshots showing the prompt change.

Relevant when: `cli/src/main.rs` changes.

### Flow 7: Agent Actions (calc, time)

Verifies agent intent detection for `calc`, `giờ mấy`, etc.

Steps:
1. Launch CLI.
2. Type `calc 10+5` → expect output containing `15`.
3. Type `giờ mấy rồi` → expect a time response.
4. Capture snapshots.

Relevant when: `core/src/agent.rs` changes.

### Flow 8: Spell Correction

Verifies spell correction for common Vietnamese typos.

Steps:
1. Launch CLI.
2. Type a known misspelling (e.g., `chnagr` → should suggest `chắng`/`chẳng`).
3. Check candidate suggestions include the corrected form.
4. Capture snapshots.

Relevant when: `core/src/spell.rs` changes.

## Negative Tests

Include at least one of these per run when the CLI is affected:

1. **Empty input.** Press Enter on an empty line → expect no crash, prompt
   reappears.
2. **Quit command.** Type `q` → expect clean exit with `Bye!`.
3. **Unknown method arg.** Launch with `-- invalid` → expect graceful
   fallback to Telex (not a crash).

## Known Failure Modes

1. **CLI binary not built.** `cargo build --bin hip-key` must run first. If
   the binary is missing, report BLOCKED.
2. **Unicode in CI.** Some CI runners do not render Vietnamese diacritics
   correctly in the terminal snapshot. If the snapshot shows `?` or boxes,
   still verify the conversion logic by checking the engine output bytes,
   not the visual glyph.
3. **tuistory session stuck.** If a previous session `qa-test` did not exit
   cleanly, run `tuistory kill qa-test` before relaunching.
4. **Stdin buffering.** The CLI uses `io::stdin().read_line`. In tuistory,
   send input followed by Enter; do not rely on character-by-character
   events.
5. **Method switch edge case.** Typing `m` as the first character after
   launch switches to VNI before any conversion. This is expected behavior.
