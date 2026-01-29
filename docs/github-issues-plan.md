# GitHub Issues Plan

## Phase 1: Core Telex Implementation (MVP)

### Issue #1 - Telex Tone Marks
**Title:** Implement Telex tone mark rules (s, f, j, r, x)
**Priority:** High
**Labels:** enhancement, vietnamese, telex
**Estimate:** Medium

**Description:**
Current Telex implementation only handles vowel modifications (ă, â, ê, ô, ơ, ư, đ). Need to add tone marks:
- s → sắc
- f -> hỏi
- j -> ngã
- r -> nặng
- x -> none (remove tone)

**Requirements:**
- Tone marks apply to the nearest vowel in the syllable
- Handle order-independent input (as, sa both work)
- Proper tone placement rules (e.g., tone goes on ă/â not a)

**Acceptance Criteria:**
- `as` → á, `sa` → á
- `aws` → ắ
- `dd` → đ (no tone)
- `ax` → a (remove tone)
- Test suite covering all combinations

---

### Issue #2 - VNI Input Method
**Title:** Implement VNI input method
**Priority:** High
**Labels:** enhancement, vietnamese, vni
**Estimate:** Medium

**Description:**
Add VNI input method alongside Telex.

**Requirements:**
- VNI vowel modifications: a8 → ă, a6 → â, o7 → ơ, o6 → ô, u7 → ư, d9 → đ
- VNI tone marks: 1 → sắc, 2 → huyền, 3 → hỏi, 4 → ngã, 5 → nặng

**Acceptance Criteria:**
- `a8` → ă
- `a81` → ắ
- VNI and Telex can coexist (selectable via config)

---

### Issue #3 - Proper Engine State Management
**Title:** Fix engine keystroke-by-keystroke processing
**Priority:** High
**Labels:** bug, core
**Estimate:** Medium

**Description:**
Current CLI uses a shortcut `convert_telex()` method. The real engine's `process()` method should work correctly keystroke-by-keystroke.

**Requirements:**
- Engine should accumulate state properly
- Language pack `process()` returns appropriate `ProcessResult`
- Buffer updates correctly after each keystroke

**Acceptance Criteria:**
- Can type character by character and see correct composition
- Backspace works correctly
- Engine integration tests pass

---

## Phase 2: Dictionary & Completion

### Issue #4 - Vietnamese Word Dictionary
**Title:** Add Vietnamese word dictionary for completion
**Priority:** Medium
**Labels:** enhancement, vietnamese, dictionary
**Estimate:** Large

**Description:**
Build and integrate a Vietnamese word dictionary for:
- Autocomplete suggestions
- Spell checking (optional)
- Frequency-based ranking

**Requirements:**
- Dictionary file format (JSON, CSV, or custom binary)
- Efficient lookup (trie or hash-based)
- ~10k-50k common words
- Word frequency data for ranking

**Acceptance Criteria:**
- Dictionary loads at startup
- `generate_candidates()` returns suggestions
- Performance: < 1ms per lookup

---

### Issue #5 - Candidate Selection UI
**Title:** Design candidate selection flow
**Priority:** Medium
**Labels:** design, ux
**Estimate:** Small

**Description:**
Define how users select from candidate suggestions.

**Options:**
1. Number key selection (1-9)
2. Arrow key navigation
3. Tab cycling

**Decision needed:** Which approach for CLI? Which for native UI?

---

## Phase 3: FFI & Platform Integration

### Issue #6 - Complete C FFI API
**Title:** Implement full C-compatible API
**Priority:** High
**Labels:** enhancement, ffi, c-api
**Estimate:** Large

**Description:**
Complete the FFI layer for platform adapters.

**Required Functions:**
```c
// Engine lifecycle
hipkey_engine_create()
hipkey_engine_destroy()
hipkey_engine_set_language_pack()

// Input processing
hipkey_process_keystroke()
hipkey_get_composing_text()
hipkey_commit()

// Candidates
hipkey_get_candidates()
hipkey_select_candidate()

// State
hipkey_clear()
hipkey_is_composing()
```

**Acceptance Criteria:**
- All functions implemented and tested
- C header file generated
- Memory safety verified (no leaks)

---

### Issue #7 - macOS Platform Adapter
**Title:** Build macOS input method adapter
**Priority:** Medium
**Labels:** platform, macos
**Estimate:** Large

**Requirements:**
- Uses FFI layer
- Integrates with macOS IME API
- System preference pane for language selection

---

### Issue #8 - Windows Platform Adapter
**Title:** Build Windows TSF adapter
**Priority:** Medium
**Labels:** platform, windows
**Estimate:** Large

---

### Issue #9 - Linux IBus/Fcitx Adapter
**Title:** Build Linux input method adapter
**Priority:** Low
**Labels:** platform, linux
**Estimate:** Large

---

## Phase 4: Polish & Features

### Issue #10 - Configuration System
**Title:** Design and implement config system
**Priority:** Medium
**Labels:** enhancement, config
**Estimate:** Medium

**Description:**
Allow users to configure:
- Default input method (Telex/VNI)
- Key bindings
- Auto-commit options
- Enable/disable smart features

**Config file locations:**
- macOS: `~/Library/Application Support/hip-key/config.toml`
- Linux: `~/.config/hip-key/config.toml`
- Windows: `%APPDATA%\hip-key\config.toml`

---

### Issue #11 - Macro/Abbreviation Expansion
**Title:** Add custom text expansion
**Priority:** Low
**Labels:** enhancement, feature
**Estimate:** Small

**Example:**
- `vk` → `việt Nam`
- `tg` → `tin greet` → `tin chào`
- User-defined in config

---

### Issue #12 - Learning/Ranking System
**Title:** Implement frequency-based candidate ranking
**Priority:** Low
**Labels:** enhancement, smart
**Estimate:** Large

**Description:**
- Track user word selections
- Adjust candidate ranking based on frequency
- Persist learning data locally

**Privacy:** All data local, no cloud sync (unless opt-in)

---

## Documentation

### Issue #13 - Contributor Guide
**Title:** Write contributing guidelines
**Priority:** Medium
**Labels:** documentation
**Estimate:** Small

**Include:**
- How to add a new language pack
- Code style guidelines
- PR process
- Test requirements

---

### Issue #14 - Language Pack Authoring Guide
**Title:** Document how to create language packs
**Priority:** Medium
**Labels:** documentation
**Estimate:** Medium

**Include:**
- `LanguagePack` trait documentation
- Example minimal language pack
- Best practices
- Testing guide

---

## Meta Issues

### Issue #15 - Performance Benchmarks
**Title:** Establish performance benchmarks
**Priority:** Medium
**Labels:** testing, performance
**Estimate:** Medium

**Metrics to track:**
- Keystroke processing latency (target: < 100μs)
- Dictionary lookup time (target: < 1ms)
- Memory usage

**Tools:** Criterion.rs for benchmarking

---

### Issue #16 - Security Audit
**Title:** Security review for FFI layer
**Priority:** High (before v1.0)
**Labels:** security
**Estimate:** Medium

**Review:**
- Buffer overflow risks
- Memory safety at FFI boundary
- Input validation

---

## Order of Implementation (Suggested)

1. **MVP Telex** (#1, #3) - Get basic typing working
2. **VNI** (#2) - Second input method
3. **FFI Complete** (#6) - Enable platform adapters
4. **macOS Adapter** (#7) - First real platform
5. **Dictionary** (#4) - Smart completion
6. **Config** (#10) - User customization
7. **Documentation** (#13, #14) - Enable contributors
8. **Windows/Linux** (#8, #9) - Cross-platform
9. **Advanced features** (#11, #12) - Nice-to-haves
10. **Benchmarks + Security** (#15, #16) - Pre-v1.0

---

## Labels to Create on GitHub

- `enhancement` - New features
- `bug` - Bugs to fix
- `vietnamese` - Vietnamese-specific work
- `telex` / `vni` - Input method specific
- `core` - Core engine work
- `ffi` - FFI layer
- `platform` - Platform adapters
- `documentation` - Docs
- `testing` - Tests
- `security` - Security issues
- `good first issue` - Beginner-friendly
- `help wanted` - Community contributions
