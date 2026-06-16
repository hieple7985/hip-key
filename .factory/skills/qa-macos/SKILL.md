---
name: qa-macos
description: >
  QA tests for the hip-key macOS IME adapter. Tests the Swift
  IMKInputController integration, FFI bridging, and .app bundle via manual
  steps (agent-browser for System Preferences, screenshots for IME behavior).
---

# qa-macos — hip-key macOS IME Testing

Tests the macOS input method adapter at `platform/macos/`. The adapter is a
Swift app (`HipKeyInputMethod`) using `InputMethodKit` (IMK) that links
`libhip_key_ffi.a` statically and bridges via `HipKeyBridge.h`.

## Testing Target

hip-key macOS IME is a local `.app` bundle. There is NO remote deployment.

**CRITICAL: macOS IME testing is manual-heavy.** It requires:
- macOS host (cannot run on Linux CI).
- Xcode + XcodeGen installed.
- System Preferences access to enable the input source.
- A real text editor (TextEdit, Notes) to receive IME output.

The GitHub Actions workflow runs on `ubuntu-latest` for CLI/FFI tests.
**macOS IME tests run ONLY locally** (the workflow marks them BLOCKED in CI
with the note "macOS IME requires a macOS host").

### Build

```bash
# Pre-flight: build the FFI static library and stage it
bash platform/macos/build-ffi.sh
```

This produces `platform/macos/HipKeyInputMethod/Frameworks/libhip_key_ffi.a`
and the bridging header. Verify:

```bash
ls -la platform/macos/HipKeyInputMethod/Frameworks/libhip_key_ffi.a
ls -la platform/macos/HipKeyInputMethod/Frameworks/hip-key.h
ls -la platform/macos/HipKeyInputMethod/Frameworks/HipKeyBridge.h
```

If any artifact is missing, report BLOCKED for qa-macos.

### Generate Xcode project

```bash
# Requires XcodeGen (brew install xcodegen)
cd platform/macos
xcodegen generate
# Produces HipKeyInputMethod.xcodeproj
```

If `xcodegen` is not installed, report BLOCKED with the install command.

### Build the .app bundle

Open `HipKeyInputMethod.xcodeproj` in Xcode, select the
`HipKeyInputMethod` scheme, and Build (Cmd+B). The output is
`HipKeyInputMethod.app`.

In CI (non-macOS), this step is BLOCKED. Mark as:
"BLOCKED: macOS IME build requires a macOS host with Xcode."

## Available Test Flows

Manual flows. Use agent-browser for System Preferences navigation, and
screenshots for IME output verification. Capture screenshots to
`./qa-results/$RUN_ID/` and reference filenames in the report.

### Flow 1: FFI Static Library Integrity

Verifies `build-ffi.sh` produces a valid static library with the expected
symbols.

Steps:
1. Run `bash platform/macos/build-ffi.sh`.
2. Verify `libhip_key_ffi.a` exists and is non-empty.
3. Inspect symbols: `nm platform/macos/HipKeyInputMethod/Frameworks/libhip_key_ffi.a | grep hipkey_`.
4. Expect to see: `hipkey_engine_create`, `hipkey_engine_destroy`,
   `hipkey_process_keystroke`, `hipkey_get_composing_text`, `hipkey_commit`,
   `hipkey_clear`, `hipkey_get_candidates`,
   `hipkey_candidate_list_free`, `hipkey_string_free`.
5. Capture the `nm` output as evidence.

Relevant when: `ffi/src/lib.rs`, `ffi/hip-key.h`, or
`platform/macos/build-ffi.sh` change.

### Flow 2: Bridging Header Sync

Verifies `HipKeyBridge.h` and `hip-key.h` are in sync and the Swift
project can find them.

Steps:
1. Check `HipKeyBridge.h` includes `hip-key.h`.
2. Verify `project.yml` references the bridging header:
   `SWIFT_OBJC_BRIDGING_HEADER`.
3. Verify `LIBRARY_SEARCH_PATHS` includes `$(PROJECT_DIR)/Frameworks`.
4. Capture the relevant `project.yml` lines as evidence.

Relevant when: `platform/macos/HipKeyInputMethod/project.yml` or bridging
headers change.

### Flow 3: IMKInputController Wiring (static analysis)

Verifies `HipKeyInputController.swift` correctly maps NSEvent key codes to
FFI key codes and handles all EngineEvent cases.

Steps (static, can run on any OS):
1. Read `platform/macos/HipKeyInputMethod/HipKeyInputController.swift`.
2. Verify `mapKeyCode` handles: Backspace (51→0x08), Delete (117→0x7F),
   Enter (36→0x0D), Escape (53→0x1B), Tab (48→0x09), Space (49→0x20),
   Arrows (123-126→0x11-0x14).
3. Verify `handle(_:client:)` switches on all `HipKeyEngineEvent` cases:
   `BUFFER_CHANGED`, `COMMIT`, `PASS_THROUGH`, `CANDIDATES_UPDATED`.
4. Verify `updateComposing`, `commitAndClear`, `showCandidates` helper
   methods call the correct FFI functions and free strings.
5. Capture the switch-statement and mapKeyCode code as evidence.

Relevant when: `platform/macos/.../HipKeyInputController.swift` changes, or
FFI event enum changes.

### Flow 4: .app Bundle Install (macOS-only)

Verifies the built `.app` can be installed and enabled as an input source.

Steps (requires macOS):
1. Copy `HipKeyInputMethod.app` to `~/Library/Input Methods/`.
2. Log out and log back in (or run `killall HipKeyInputMethod`).
3. Open System Preferences > Keyboard > Input Sources.
4. Click `+`, find "Vietnamese" → "HipKey".
5. Add it.
6. Capture a screenshot of the Input Sources list showing HipKey.

In CI: BLOCKED with "requires macOS host."

Relevant when: `platform/macos/` Info.plist, bundle ID, or input source
registration changes.

### Flow 5: End-to-End Typing (macOS-only)

Verifies typing in a real text editor produces correct Vietnamese output.

Steps (requires macOS with IME installed):
1. Switch to HipKey input source (Ctrl+Space or menu bar).
2. Open TextEdit.
3. Type `aws` → expect `ắ` to appear.
4. Type `dd` → expect `đ`.
5. Type `chaof` → expect `chào`.
6. Type Backspace mid-composition → expect buffer to shorten.
7. Capture screenshots of TextEdit showing each result.

In CI: BLOCKED with "requires macOS host."

Relevant when: any change to FFI, engine, or
`HipKeyInputController.swift`.

## Negative Tests

Include at least one when macOS is affected:

1. **Missing FFI staticlib.** Delete (temporarily) the staged
   `libhip_key_ffi.a` and run `build-ffi.sh` → expect it to rebuild and
   restore the file. If it does not, the script is broken.
2. **Stale bridging header.** Add a new FFI function to `ffi/src/lib.rs`
   but do NOT update `hip-key.h` → the Swift build should fail (or warn).
   This verifies the header and source stay in sync.

## Known Failure Modes

1. **XcodeGen not installed.** `brew install xcodegen` then
   `xcodegen generate` in `platform/macos/`.
2. **Stale FFI library.** After changing FFI source, always re-run
   `build-ffi.sh` before opening Xcode. The Swift project links the staged
   `.a`, not the cargo build output directly.
3. **Input source not registered.** After installing the `.app`, you must
   log out/in or `killall` the IME process for macOS to detect it.
4. **macOS sandbox.** The IME runs in a restricted sandbox. File writes and
   network calls from the IME process may be blocked. This is expected;
   hip-key is local-first by design.
5. **CI cannot test macOS.** The GitHub Actions workflow runs on Linux. Any
   macOS-only flow must report BLOCKED in CI, not FAIL. This is a known
   environment limitation, not a code defect.
