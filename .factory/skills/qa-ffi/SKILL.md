---
name: qa-ffi
description: >
  QA tests for the hip-key FFI layer (C-compatible API). Tests the public C
  surface in ffi/src/lib.rs against the hip-key.h contract: engine lifecycle,
  keystroke processing, candidate retrieval, commit, clear, and agent API.
---

# qa-ffi — hip-key FFI Testing

Tests the C-compatible API in `ffi/src/lib.rs` against the contract in
`ffi/hip-key.h`. The FFI layer is a `cdylib` + `staticlib` consumed by
platform adapters (currently macOS Swift).

## Testing Target

hip-key has NO remote FFI service. Always test locally:

1. Build the library: `cargo build -p hip-key-ffi`
2. Artifacts at `target/debug/libhip_key_ffi.{a,dylib}`
3. The FFI layer has its own integration tests in `ffi/src/lib.rs` under
   `#[cfg(test)]`. Functional QA here focuses on contract-level behavior
   beyond what unit tests cover.

In CI, the workflow runs on `ubuntu-latest` and builds the FFI crate.

## Build & Verify

```bash
# Pre-flight: must succeed
cargo build -p hip-key-ffi

# Verify artifacts exist
ls -la target/debug/libhip_key_ffi.a
ls -la target/debug/libhip_key_ffi.dylib   # .so on Linux
```

If the build fails, report BLOCKED for qa-ffi with the compile error.

## Available Test Flows

The orchestrator picks only flows relevant to the current diff. FFI tests are
"contract verification" flows: they exercise the public C surface and confirm
behavior matches `ffi/hip-key.h`.

### Flow 1: Engine Lifecycle

Verifies `hipkey_engine_create` returns non-null and
`hipkey_engine_destroy` accepts the pointer without error.

Steps:
1. Create engine: call `hipkey_engine_create()`.
2. Assert the returned pointer is non-null.
3. Destroy engine: call `hipkey_engine_destroy(ptr)`.
4. Assert no crash / no panic crossing FFI boundary.
5. Null safety: call `hipkey_engine_destroy(null)` → must be a safe no-op.

Relevant when: `ffi/src/lib.rs` engine lifecycle functions change.

### Flow 2: Language Pack Loading

Verifies `hipkey_engine_set_language_pack_vi` accepts method 0 (Telex) and
1 (VNI), and rejects invalid method codes.

Steps:
1. Create engine.
2. Set VI Telex (method=0) → expect `HIPKEY_SUCCESS` (0).
3. Set VI VNI (method=1) → expect `HIPKEY_SUCCESS`.
4. Set invalid method (e.g., 99) → expect `HIPKEY_INVALID_ARGUMENT` (-2).
5. Destroy engine.

Relevant when: `ffi/src/lib.rs` language pack functions change, or
`lang/vi/src/lib.rs` `InputMethod` enum changes.

### Flow 3: Keystroke Processing (Telex)

Verifies the full keystroke → buffer path through FFI.

Steps:
1. Create engine, set VI Telex.
2. Send `a` (key_code=0x61): expect `HIPKEY_EVENT_BUFFER_CHANGED`.
3. Send `w` (key_code=0x77): expect `HIPKEY_EVENT_BUFFER_CHANGED`.
4. Call `hipkey_get_composing_text` → expect string containing `ă`.
5. Call `hipkey_string_free` on the returned pointer.
6. Destroy engine.

Relevant when: `ffi/src/lib.rs` `hipkey_process_keystroke` or
`hipkey_get_composing_text` change, or `core/src/engine.rs` changes.

### Flow 4: Commit and Clear

Verifies commit moves composing to committed, and clear resets state.

Steps:
1. Create engine, set VI Telex.
2. Type `x`, `i`, `n` → buffer is `xin`.
3. Call `hipkey_commit` → expect `HIPKEY_SUCCESS`.
4. Call `hipkey_get_committed_text` → expect `xin`.
5. Call `hipkey_get_last_committed` → expect `xin` (must be freed with
   `hipkey_string_free`, NOT a foreign allocator).
6. Call `hipkey_clear` → expect `HIPKEY_SUCCESS`.
7. Call `hipkey_is_composing` → expect `false`.
8. Destroy engine.

Relevant when: `ffi/src/lib.rs` commit/clear/committed functions change.

### Flow 5: Null Safety Matrix

Verifies every FFI function handles null engine pointer gracefully.

Steps (for each function):
1. `hipkey_engine_set_language_pack_vi(null, 0)` → expect
   `HIPKEY_INVALID_ARGUMENT`.
2. `hipkey_process_keystroke(null, ...)` → expect
   `HIPKEY_EVENT_ERROR`.
3. `hipkey_get_composing_text(null)` → expect null pointer.
4. `hipkey_get_committed_text(null)` → expect null pointer.
5. `hipkey_get_last_committed(null)` → expect null pointer.
6. `hipkey_commit(null)` → expect `HIPKEY_INVALID_ARGUMENT`.
7. `hipkey_clear(null)` → expect `HIPKEY_INVALID_ARGUMENT`.
8. `hipkey_is_composing(null)` → expect `false`.
9. `hipkey_get_candidates(null)` → expect list with `len=0` and null
   `candidates` pointer.

Relevant when: ANY FFI function changes its null-handling.

### Flow 6: Candidate Retrieval

Verifies candidate list allocation, retrieval, and freeing.

Steps:
1. Create engine, set VI Telex.
2. Type a prefix that triggers candidates (e.g., `ch`).
3. Call `hipkey_get_candidates` → expect non-null `candidates` and `len > 0`.
4. For each candidate: read `text` (via `CStr::from_ptr`), read `confidence`.
5. Call `hipkey_candidate_list_free` → must free all candidate strings AND
   the array. Verify no double-free / leak.
6. Destroy engine.

Relevant when: `ffi/src/lib.rs` `hipkey_get_candidates` /
`hipkey_candidate_list_free` change, or `core/src/candidate.rs` changes.

### Flow 7: Agent API

Verifies agent enable/disable/process.

Steps:
1. Create engine.
2. `hipkey_agent_is_enabled` → expect `true` (enabled by default).
3. `hipkey_agent_disable` → expect `HIPKEY_SUCCESS`.
4. `hipkey_agent_is_enabled` → expect `false`.
5. `hipkey_agent_enable` → expect `HIPKEY_SUCCESS`.
6. `hipkey_agent_process(engine, "calc 10+5")` → expect result with
   `success=true`, `display_text` containing `15`.
7. `hipkey_agent_action_result_free(result)` → must free both
   `display_text` and `commit_text`.
8. Destroy engine.

Relevant when: `ffi/src/lib.rs` agent functions change, or
`core/src/agent.rs` changes.

## Negative Tests

Include at least one per run when FFI is affected:

1. **Foreign allocator free.** Pass a pointer NOT allocated by this crate to
   `hipkey_string_free` → this is UB by contract; document it but do NOT
   actually execute it (would crash). Instead verify the SAFETY comment
   documents the requirement.
2. **Massive key_code.** Send `key_code = u32::MAX` → expect
   `HIPKEY_EVENT_PASS_THROUGH` or graceful `Key::Unknown`, not a panic.
3. **Double destroy.** Destroy an engine twice → second call must be a safe
   no-op on null (the pointer is dangling; do NOT actually run this, just
   verify the contract says null is safe).

## Known Failure Modes

1. **Allocator mismatch.** If a test frees an FFI-returned string with the
   wrong allocator (e.g., `libc::free` on a Rust-allocated `CString`), the
   result is UB. Always use `hipkey_string_free` for strings returned by this
   crate.
2. **Layout::array panic.** If a caller passes a corrupted `len` to
   `hipkey_candidate_list_free`, `Layout::array(len).unwrap()` could panic.
   The current code guards with `if let Ok(layout)`. Verify this guard
   remains.
3. **CString::new with interior null.** If engine buffer ever contains a
   null byte, `CString::new` fails and returns null pointer. This is
   expected; tests should handle the null return.
4. **crate-type missing.** If `Cargo.toml` loses `crate-type = ["cdylib",
   "staticlib"]`, no `.a`/`.dylib` is produced. Verify the `[lib]` section.
