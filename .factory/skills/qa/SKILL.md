---
name: qa
description: >
  Run QA tests for hip-key (Rust IME engine). Analyzes git diff to determine
  affected areas (cli / ffi / macos), runs the relevant sub-skill flows,
  and generates a diff-targeted report. Uses tuistory for CLI, cargo test
  for FFI, manual steps for macOS IME. Use when testing PRs, releases, or
  smoke testing local builds.
---

# QA Orchestrator

**SCOPE: This skill performs functional QA only. Interact with the CLI as a
real user would (tuistory), run FFI integration tests, and verify macOS IME
behavior. Do NOT run `cargo test`, `cargo clippy`, or unit tests as QA
rows. Those are CI concerns handled by other workflows.**

## Step 1: Load Configuration

Read `.factory/skills/qa/config.yaml` for app definitions, personas, and
build commands.

## Step 2: Determine Target Environment

hip-key is fully local. No remote environments. Always test against:
- `cli-local` for the CLI binary
- `ffi-local` for the FFI library
- `macos-app` for the macOS IME bundle

Respect `default_target` from config unless the user specifies otherwise.

## Step 3: Analyze Git Diff

Run `git diff` (against the PR base or `HEAD~1`) to determine what changed.
Map changed files to apps using `path_patterns` in config.yaml:

- `cli/**`, `core/src/engine.rs`, `core/src/buffer.rs`, `core/src/langpack.rs`,
  `core/src/keystroke.rs`, `core/src/candidate.rs` → **qa-cli**
- `ffi/**`, `core/src/engine.rs` → **qa-ffi**
- `platform/macos/**`, `ffi/**` → **qa-macos**

Files that do NOT match any app's path patterns (e.g. `docs/**`, `AGENTS.md`,
`README.md`, `Cargo.toml` root only) are not associated with any app. Do NOT
run app flows for them.

For each affected app, run ONLY that app's flows from its sub-skill. Generate
additional targeted tests based on the specific lines in the diff.

For apps NOT affected by the diff: do NOT load or run their module. Do NOT run
their pre-flight checks.

If NO app is affected (docs-only, README-only), report INCONCLUSIVE: "No app
code changed. QA not applicable for this diff."

## Step 4: Pre-flight Checks (app-specific only)

Run pre-flight checks ONLY for affected apps:

- **qa-cli**: `cargo build --bin hip-key` must succeed. If it fails, report
  BLOCKED for qa-cli with the compile error.
- **qa-ffi**: `cargo build -p hip-key-ffi` must succeed. If it fails, report
  BLOCKED for qa-ffi.
- **qa-macos**: `bash platform/macos/build-ffi.sh` must produce
  `platform/macos/HipKeyInputMethod/Frameworks/libhip_key_ffi.a`. Requires
  Xcode. If the FFI build fails or the staticlib is missing, report BLOCKED for
  qa-macos.

Do NOT run pre-flight checks for apps not affected by the diff. If a pre-flight
check fails for an affected app, report BLOCKED with the error and remediation,
then continue with other affected apps.

## Step 5: Execute Diff-Relevant Flows Only

For each affected app, read its sub-skill from
`.factory/skills/qa-<app-name>/SKILL.md`. The sub-skill contains a MENU of
flows. You must:

1. Read the diff and identify which flows are relevant to the change.
2. Run those flows PLUS adjacent flows that verify the change integrates
   correctly (e.g., if a new Telex rule is added to `core`, test it in both
   the CLI and confirm VNI paths are not broken).
3. Do NOT run completely unrelated flows (e.g., if the diff only touches FFI,
   do NOT test CLI method-switching).
4. If no existing flow covers the change, write a NEW ad-hoc test that
   directly verifies the changed behavior.
5. Do NOT run unit tests, lint, typecheck, or any automated suite. This is
   functional QA only.

## Step 6: Evidence Capture

After each significant test step, capture evidence. Use **text snapshots as
primary evidence**. They render inline in the PR comment with no hosting issues.

For CLI/TUI (tuistory):
- Use the `droid-control` skill for all tuistory interactions.
- Capture terminal state with `tuistory -s <session> snapshot --trim`.
- Embed the snapshot directly in the report as a fenced code block.
- Each snapshot MUST show something DIFFERENT. Wait for the UI to change
  before capturing again.

For FFI (cargo-test style integration):
- Capture the Rust test output verbatim (stdout/stderr).
- If ImageMagick is available (config.imagemagick: true), generate animated
  GIF diffs of TUI before/after snapshots for CLI tests.

For macOS IME (manual):
- Capture screenshots to `./qa-results/$RUN_ID/` and reference filenames.
  Do NOT embed `![image](url)` markdown; the workflow uploads screenshots as
  artifacts.

Evidence quality rules:
- Focus on the RELEVANT content. Trim snapshots to the meaningful part.
- Label each snapshot clearly: what it shows and why it matters.
- NEVER embed broken image links. Prefer text evidence.

## Step 7: Test Quality Gate

TEST QUALITY REQUIREMENTS:

1. CHANGE-SPECIFIC FIRST. At least half your tests should directly verify the
   behavioral change in the diff.
2. INTEGRATION TESTS ARE VALID. Tests that verify the change integrates with
   existing features are good (e.g., new Telex rule does not break VNI, new
   FFI function does not break existing engine flow).
3. NO UNRELATED FLOWS. Do NOT test features completely unrelated to the diff.
4. NO AUTOMATED TEST SUITES. Do NOT run `cargo test`, `cargo bench`, or
   `cargo clippy` as QA rows. This is functional QA only.
5. NEGATIVE TESTS. Include at least 1 test verifying error handling or
   boundary conditions related to the change.
6. INTERACTIVE TESTING. Test by actually interacting with the CLI as a real
   user would (tuistory keystrokes, not `droid exec`).
7. INCONCLUSIVE IF UNSURE. If you cannot articulate what the PR changes,
   mark INCONCLUSIVE rather than PASS.

## Step 8: Handle Failures

**Never silently skip a flow.** If a flow cannot complete, report it as
BLOCKED with what was tried and how the user can fix it. Then continue with
the next flow. Never abort the entire run for a single failure.

## Step 9: Generate Report

Generate the report at `./qa-results/report.md` using
`.factory/skills/qa/REPORT-TEMPLATE.md`.

Key rules:
- Start with `## QA Report` heading followed by the test results table.
- Result column uses emojis: PASS, FAIL, BLOCKED, FLAKY, INCONCLUSIVE.
- Keep it CONCISE. Table + short "Action Required" section (if any) +
  collapsed evidence = the entire report.
- Do NOT report setup/prerequisite steps (building, startup) as test rows.
- Put ALL evidence in a single collapsed `<details>` block.
- For TUI evidence: embed text snapshots as labeled fenced code blocks.

## Step 10: Suggest Skill Updates (Failure Learning)

After generating the report, check if any BLOCKED or FAIL results revealed a
**testing environment insight** that would help future QA runs succeed. This is
about learning how the testing environment works, NOT about fixing bad
selectors or skill typos.

Good suggestions (environment knowledge):
- "The CLI requires `cargo run --bin hip-key`, not just `hip-key` from PATH"
- "VNI mode needs the `vni` arg as the first positional, not a flag"
- "FFI tests panic on macOS without `DYLD_LIBRARY_PATH` set"

Bad suggestions (skill bugs, not environment insights):
- "Selector for the prompt changed" - that's a skill bug
- "Telex rule `aw` now produces `â` instead of `ă`" - that's a PR change

Format per `failure_learning: suggest_in_report` in config.yaml. Do NOT write
`skill-updates.json`. Only include the suggestion table in the report.

## Known Failure Modes

1. **CLI binary not built.** If `target/debug/hip-key` is missing, run
   `cargo build --bin hip-key` before launching tuistory.
2. **tuistory session name collision.** Always use `-s qa-test` and clean up
   with `tuistory kill qa-test` before a fresh launch.
3. **FFI staticlib stale.** After changing FFI source, re-run
   `bash platform/macos/build-ffi.sh` before testing macOS.
4. **Vietnamese input in CI.** Non-ASCII keystrokes may not register in some
   CI runners. Prefer Telex/VNI rule triggers (ASCII letters like `aw`, `dd`)
   over direct Unicode input.
