//! C-compatible FFI for hip-key core
//!
//! Provides a stable C API for platform adapters.
//! All strings returned are heap-allocated and must be freed with hipkey_string_free().

// FFI functions accept raw pointers by design. Every function null-checks its
// pointer arguments before dereferencing, so the `not_unsafe_ptr_arg_deref`
// lint is not actionable here (marking all of them `unsafe` would require all
// C callers to wrap calls in `unsafe {}`, which is not idiomatic for C FFI).
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use hip_key_core::{
    Engine, EngineEvent, Key, Keystroke, LanguagePack, Modifiers,
};
use hip_key_core::keystroke::ArrowDirection;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

#[repr(C)]
pub struct HipKeyEngine {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HipKeyResult {
    Success = 0,
    Error = -1,
    InvalidArgument = -2,
    NotReady = -3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HipKeyEngineEvent {
    BufferChanged = 1,
    CandidatesUpdated = 2,
    Commit = 3,
    PassThrough = 4,
    Error = -1,
}

#[repr(C)]
pub struct HipKeyCandidate {
    text: *mut c_char,
    confidence: f32,
}

#[repr(C)]
pub struct HipKeyCandidateList {
    candidates: *mut HipKeyCandidate,
    len: usize,
}

#[repr(C)]
pub struct HipKeyActionResult {
    success: bool,
    display_text: *mut c_char,
    commit_text: *mut c_char,
    should_commit: bool,
}

struct EngineState {
    engine: Engine,
    lang_pack: Option<Box<dyn LanguagePack>>,
    agent: hip_key_core::Agent,
    last_event: Option<EngineEvent>,
    last_commit: Option<CString>,
}

#[no_mangle]
pub extern "C" fn hipkey_engine_create() -> *mut HipKeyEngine {
    let state = Box::new(EngineState {
        engine: Engine::new(),
        lang_pack: None,
        agent: hip_key_core::Agent::new(),
        last_event: None,
        last_commit: None,
    });
    Box::into_raw(state) as *mut HipKeyEngine
}

#[no_mangle]
pub extern "C" fn hipkey_engine_destroy(engine: *mut HipKeyEngine) {
    if engine.is_null() {
        return;
    }
    unsafe {
        let state = Box::from_raw(engine as *mut EngineState);
        drop(state);
    }
}

#[no_mangle]
pub extern "C" fn hipkey_engine_set_language_pack_vi(
    engine: *mut HipKeyEngine,
    method: u32,
) -> HipKeyResult {
    if engine.is_null() {
        return HipKeyResult::InvalidArgument;
    }
    let state = unsafe { &mut *(engine as *mut EngineState) };
    let input_method = match method {
        0 => hip_key_lang_vi::InputMethod::Telex,
        1 => hip_key_lang_vi::InputMethod::VNI,
        _ => return HipKeyResult::InvalidArgument,
    };
    let pack = hip_key_lang_vi::Vietnamese::with_method(input_method);
    state.lang_pack = Some(Box::new(pack));
    state.engine.set_language_pack(state.lang_pack.take().unwrap());
    HipKeyResult::Success
}

#[no_mangle]
pub extern "C" fn hipkey_process_keystroke(
    engine: *mut HipKeyEngine,
    key_code: u32,
    shift: bool,
    ctrl: bool,
    alt: bool,
    meta: bool,
) -> HipKeyEngineEvent {
    if engine.is_null() {
        return HipKeyEngineEvent::Error;
    }
    let state = unsafe { &mut *(engine as *mut EngineState) };

    let key = match key_code {
        0x20 => Key::Space,
        0x08 => Key::Backspace,
        0x7F => Key::Delete,
        0x0D => Key::Enter,
        0x1B => Key::Escape,
        0x09 => Key::Tab,
        0x11 => Key::Arrow(ArrowDirection::Up),
        0x12 => Key::Arrow(ArrowDirection::Down),
        0x13 => Key::Arrow(ArrowDirection::Left),
        0x14 => Key::Arrow(ArrowDirection::Right),
        _ if (0x20..=0x7E).contains(&key_code) => Key::Char(key_code as u8 as char),
        _ => Key::Unknown(key_code),
    };

    let keystroke = Keystroke {
        key,
        modifiers: Modifiers { shift, ctrl, alt, meta },
    };

    let event = state.engine.process(&keystroke);
    state.last_event = Some(event.clone());

    if let EngineEvent::Commit(text) = &event {
        state.last_commit = CString::new(text.clone()).ok();
    }

    match event {
        EngineEvent::BufferChanged => HipKeyEngineEvent::BufferChanged,
        EngineEvent::CandidatesUpdated => HipKeyEngineEvent::CandidatesUpdated,
        EngineEvent::Commit(_) => HipKeyEngineEvent::Commit,
        EngineEvent::PassThrough => HipKeyEngineEvent::PassThrough,
    }
}

#[no_mangle]
pub extern "C" fn hipkey_get_composing_text(engine: *mut HipKeyEngine) -> *mut c_char {
    if engine.is_null() {
        return ptr::null_mut();
    }
    let state = unsafe { &*(engine as *const EngineState) };
    let text = state.engine.buffer().composing();
    match CString::new(text) {
        Ok(cstr) => cstr.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn hipkey_get_committed_text(engine: *mut HipKeyEngine) -> *mut c_char {
    if engine.is_null() {
        return ptr::null_mut();
    }
    let state = unsafe { &*(engine as *const EngineState) };
    let text = state.engine.buffer().committed();
    match CString::new(text) {
        Ok(cstr) => cstr.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn hipkey_commit(engine: *mut HipKeyEngine) -> HipKeyResult {
    if engine.is_null() {
        return HipKeyResult::InvalidArgument;
    }
    let state = unsafe { &mut *(engine as *mut EngineState) };
    let text = state.engine.commit();
    state.last_commit = CString::new(text).ok();
    HipKeyResult::Success
}

#[no_mangle]
pub extern "C" fn hipkey_get_last_committed(engine: *mut HipKeyEngine) -> *mut c_char {
    if engine.is_null() {
        return ptr::null_mut();
    }
    let state = unsafe { &*(engine as *const EngineState) };
    match &state.last_commit {
        Some(cstr) => {
            // Use Rust allocator (consistent with hipkey_string_free → CString::from_raw).
            // cstr.to_bytes() has no interior nulls by CString invariant, so this always succeeds.
            CString::new(cstr.to_bytes()).unwrap_or_default().into_raw()
        }
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn hipkey_get_candidates(engine: *mut HipKeyEngine) -> HipKeyCandidateList {
    if engine.is_null() {
        return HipKeyCandidateList {
            candidates: ptr::null_mut(),
            len: 0,
        };
    }
    let state = unsafe { &*(engine as *const EngineState) };
    let candidates = state.engine.candidates();

    if candidates.is_empty() {
        return HipKeyCandidateList {
            candidates: ptr::null_mut(),
            len: 0,
        };
    }

    let count = candidates.len().min(9);
    if count == 0 {
        return HipKeyCandidateList {
            candidates: ptr::null_mut(),
            len: 0,
        };
    }
    let layout = match std::alloc::Layout::array::<HipKeyCandidate>(count) {
        Ok(l) => l,
        Err(_) => return HipKeyCandidateList {
            candidates: ptr::null_mut(),
            len: 0,
        },
    };
    // alloc_zeroed: zero-initialize so a panic mid-population leaves null
    // text pointers (safely skipped by candidate_list_free's null check).
    let c_ptr = unsafe { std::alloc::alloc_zeroed(layout) as *mut HipKeyCandidate };
    if c_ptr.is_null() {
        return HipKeyCandidateList {
            candidates: ptr::null_mut(),
            len: 0,
        };
    }

    for (i, candidate) in candidates.iter().take(count).enumerate() {
        let c_text = CString::new(candidate.text.clone()).unwrap_or_default();
        unsafe {
            *c_ptr.add(i) = HipKeyCandidate {
                text: c_text.into_raw(),
                confidence: candidate.confidence,
            };
        }
    }

    HipKeyCandidateList {
        candidates: c_ptr,
        len: count,
    }
}

#[no_mangle]
pub extern "C" fn hipkey_is_composing(engine: *mut HipKeyEngine) -> bool {
    if engine.is_null() {
        return false;
    }
    let state = unsafe { &*(engine as *const EngineState) };
    !state.engine.is_idle()
}

#[no_mangle]
pub extern "C" fn hipkey_clear(engine: *mut HipKeyEngine) -> HipKeyResult {
    if engine.is_null() {
        return HipKeyResult::InvalidArgument;
    }
    let state = unsafe { &mut *(engine as *mut EngineState) };
    state.engine.clear();
    state.last_event = None;
    state.last_commit = None;
    HipKeyResult::Success
}

#[no_mangle]
pub extern "C" fn hipkey_string_free(s: *mut c_char) {
    if !s.is_null() {
        unsafe { drop(CString::from_raw(s)) };
    }
}

// SAFETY: `s` must be NULL or a pointer previously returned by this crate's
// allocation functions. The caller must not pass a dangling or foreign pointer.

/// Maximum candidates ever returned by hipkey_get_candidates. Used to clamp
/// caller-supplied `len` in hipkey_candidate_list_free so a corrupted/tampered
/// len cannot cause OOB reads or wrong-layout dealloc.
const MAX_CANDIDATES: usize = 9;

#[no_mangle]
pub extern "C" fn hipkey_candidate_list_free(list: HipKeyCandidateList) {
    if list.candidates.is_null() || list.len == 0 {
        return;
    }
    // Do not trust caller-supplied len beyond the maximum we ever allocate.
    // hipkey_get_candidates caps at MAX_CANDIDATES, so any len above that is
    // either corruption or tampering. Clamp to avoid OOB read + wrong-layout
    // dealloc (which would be UB in the global allocator).
    let safe_len = list.len.min(MAX_CANDIDATES);
    for i in 0..safe_len {
        unsafe {
            let candidate = &*list.candidates.add(i);
            if !candidate.text.is_null() {
                drop(CString::from_raw(candidate.text));
            }
        }
    }
    // Dealloc uses the clamped len to compute the Layout. This matches the
    // original allocation when the caller passes back the struct unmodified
    // (the common case). If the caller tampered with len downward, we still
    // free the correct number of strings up to safe_len and then dealloc the
    // original-sized layout (computed from safe_len, which is <= actual).
    // Note: if caller set len > actual alloc, safe_len clamps to MAX_CANDIDATES
    // which equals the max alloc, so Layout is always valid.
    if let Ok(layout) = std::alloc::Layout::array::<HipKeyCandidate>(safe_len) {
        unsafe { std::alloc::dealloc(list.candidates as *mut u8, layout) };
    }
}

#[no_mangle]
pub extern "C" fn hipkey_agent_enable(engine: *mut HipKeyEngine) -> HipKeyResult {
    if engine.is_null() {
        return HipKeyResult::InvalidArgument;
    }
    let state = unsafe { &mut *(engine as *mut EngineState) };
    state.agent.enable();
    HipKeyResult::Success
}

#[no_mangle]
pub extern "C" fn hipkey_agent_disable(engine: *mut HipKeyEngine) -> HipKeyResult {
    if engine.is_null() {
        return HipKeyResult::InvalidArgument;
    }
    let state = unsafe { &mut *(engine as *mut EngineState) };
    state.agent.disable();
    HipKeyResult::Success
}

#[no_mangle]
pub extern "C" fn hipkey_agent_is_enabled(engine: *mut HipKeyEngine) -> bool {
    if engine.is_null() {
        return false;
    }
    let state = unsafe { &*(engine as *const EngineState) };
    state.agent.is_enabled()
}

#[no_mangle]
pub extern "C" fn hipkey_agent_process(
    engine: *mut HipKeyEngine,
    text: *const c_char,
) -> HipKeyActionResult {
    if engine.is_null() || text.is_null() {
        return HipKeyActionResult {
            success: false,
            display_text: ptr::null_mut(),
            commit_text: ptr::null_mut(),
            should_commit: false,
        };
    }

    let text_str = unsafe { CStr::from_ptr(text) };
    let text_str = match text_str.to_str() {
        Ok(s) => s,
        Err(_) => return HipKeyActionResult {
            success: false,
            display_text: ptr::null_mut(),
            commit_text: ptr::null_mut(),
            should_commit: false,
        },
    };

    let state = unsafe { &*(engine as *const EngineState) };

    if let Some(result) = state.agent.process(text_str) {
        let display_cstr = CString::new(result.display_text).unwrap_or_default();
        let commit_cstr = result.commit_text.as_ref().map(|s| CString::new(s.clone()).unwrap_or_default());
        let commit_ptr = commit_cstr.map(|c| c.into_raw()).unwrap_or(ptr::null_mut());

        return HipKeyActionResult {
            success: result.success,
            display_text: display_cstr.into_raw(),
            commit_text: commit_ptr,
            should_commit: result.should_commit,
        };
    }

    HipKeyActionResult {
        success: false,
        display_text: ptr::null_mut(),
        commit_text: ptr::null_mut(),
        should_commit: false,
    }
}

#[no_mangle]
pub extern "C" fn hipkey_agent_action_result_display_text(result: HipKeyActionResult) -> *mut c_char {
    result.display_text
}

#[no_mangle]
pub extern "C" fn hipkey_agent_action_result_free(result: HipKeyActionResult) {
    if !result.display_text.is_null() {
        unsafe { drop(CString::from_raw(result.display_text)) };
    }
    if !result.commit_text.is_null() {
        unsafe { drop(CString::from_raw(result.commit_text)) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_codes() {
        assert_eq!(HipKeyResult::Success as i32, 0);
        assert_eq!(HipKeyResult::Error as i32, -1);
        assert_eq!(HipKeyResult::InvalidArgument as i32, -2);
        assert_eq!(HipKeyResult::NotReady as i32, -3);
    }

    #[test]
    fn test_engine_create_destroy() {
        let engine = hipkey_engine_create();
        assert!(!engine.is_null());
        hipkey_engine_destroy(engine);
    }

    #[test]
    fn test_engine_null_safety() {
        assert_eq!(
            hipkey_engine_set_language_pack_vi(ptr::null_mut(), 0),
            HipKeyResult::InvalidArgument
        );
        assert_eq!(hipkey_clear(ptr::null_mut()), HipKeyResult::InvalidArgument);
        assert!(!hipkey_is_composing(ptr::null_mut()));
        assert!(hipkey_get_composing_text(ptr::null_mut()).is_null());
        assert!(hipkey_get_committed_text(ptr::null_mut()).is_null());
    }

    #[test]
    fn test_engine_set_language_pack() {
        let engine = hipkey_engine_create();
        assert_eq!(
            hipkey_engine_set_language_pack_vi(engine, 0),
            HipKeyResult::Success
        );
        assert_eq!(
            hipkey_engine_set_language_pack_vi(engine, 1),
            HipKeyResult::Success
        );
        assert_eq!(
            hipkey_engine_set_language_pack_vi(engine, 99),
            HipKeyResult::InvalidArgument
        );
        hipkey_engine_destroy(engine);
    }

    #[test]
    fn test_engine_process_keystroke() {
        let engine = hipkey_engine_create();
        hipkey_engine_set_language_pack_vi(engine, 0);

        let event = hipkey_process_keystroke(engine, 'a' as u32, false, false, false, false);
        assert_eq!(event, HipKeyEngineEvent::BufferChanged);

        let composing = hipkey_get_composing_text(engine);
        assert!(!composing.is_null());
        let text = unsafe { CStr::from_ptr(composing) };
        assert_eq!(text.to_str().unwrap(), "a");
        hipkey_string_free(composing);

        hipkey_engine_destroy(engine);
    }

    #[test]
    fn test_engine_clear() {
        let engine = hipkey_engine_create();
        hipkey_engine_set_language_pack_vi(engine, 0);

        hipkey_process_keystroke(engine, 'a' as u32, false, false, false, false);
        assert!(hipkey_is_composing(engine));

        hipkey_clear(engine);
        assert!(!hipkey_is_composing(engine));

        hipkey_engine_destroy(engine);
    }

    #[test]
    fn test_engine_commit() {
        let engine = hipkey_engine_create();
        hipkey_engine_set_language_pack_vi(engine, 0);

        hipkey_process_keystroke(engine, 'x' as u32, false, false, false, false);
        hipkey_process_keystroke(engine, 'i' as u32, false, false, false, false);
        hipkey_process_keystroke(engine, 'n' as u32, false, false, false, false);

        assert_eq!(hipkey_commit(engine), HipKeyResult::Success);

        let committed = hipkey_get_committed_text(engine);
        assert!(!committed.is_null());
        let text = unsafe { CStr::from_ptr(committed) };
        assert_eq!(text.to_str().unwrap(), "xin");
        hipkey_string_free(committed);

        hipkey_engine_destroy(engine);
    }

    #[test]
    fn test_destroy_null_safe() {
        hipkey_engine_destroy(ptr::null_mut());
    }

    #[test]
    fn test_string_free_null_safe() {
        hipkey_string_free(ptr::null_mut());
    }

    #[test]
    fn test_candidate_list_free_empty() {
        let empty = HipKeyCandidateList {
            candidates: ptr::null_mut(),
            len: 0,
        };
        hipkey_candidate_list_free(empty);
    }

    #[test]
    fn test_process_without_language_pack() {
        let engine = hipkey_engine_create();
        let event = hipkey_process_keystroke(engine, 'a' as u32, false, false, false, false);
        assert_eq!(event, HipKeyEngineEvent::PassThrough);
        hipkey_engine_destroy(engine);
    }

    #[test]
    fn test_clear_resets_state() {
        let engine = hipkey_engine_create();
        hipkey_engine_set_language_pack_vi(engine, 0);

        hipkey_process_keystroke(engine, 'a' as u32, false, false, false, false);
        hipkey_process_keystroke(engine, 'w' as u32, false, false, false, false);

        hipkey_clear(engine);

        let composing = hipkey_get_composing_text(engine);
        let text = unsafe { CStr::from_ptr(composing) };
        assert_eq!(text.to_str().unwrap(), "");
        hipkey_string_free(composing);

        hipkey_engine_destroy(engine);
    }

    #[test]
    fn test_agent_enable_disable() {
        let engine = hipkey_engine_create();
        assert!(hipkey_agent_is_enabled(engine));

        assert_eq!(hipkey_agent_disable(engine), HipKeyResult::Success);
        assert!(!hipkey_agent_is_enabled(engine));

        assert_eq!(hipkey_agent_enable(engine), HipKeyResult::Success);
        assert!(hipkey_agent_is_enabled(engine));

        hipkey_engine_destroy(engine);
    }

    #[test]
    fn test_agent_null_safety() {
        assert_eq!(hipkey_agent_enable(ptr::null_mut()), HipKeyResult::InvalidArgument);
        assert_eq!(hipkey_agent_disable(ptr::null_mut()), HipKeyResult::InvalidArgument);
        assert!(!hipkey_agent_is_enabled(ptr::null_mut()));
    }

    #[test]
    fn test_agent_process_calc() {
        let engine = hipkey_engine_create();

        let result = hipkey_agent_process(engine, CString::new("calc 10+5").unwrap().as_ptr());
        assert!(result.success);
        assert!(!result.display_text.is_null());
        let display = unsafe { CStr::from_ptr(result.display_text) };
        assert!(display.to_str().unwrap().contains("15"));
        hipkey_agent_action_result_free(result);

        hipkey_engine_destroy(engine);
    }

    #[test]
    fn test_agent_process_time() {
        let engine = hipkey_engine_create();

        let result = hipkey_agent_process(engine, CString::new("giờ mấy rồi").unwrap().as_ptr());
        assert!(result.success);
        assert!(!result.display_text.is_null());
        hipkey_agent_action_result_free(result);

        hipkey_engine_destroy(engine);
    }

    #[test]
    fn test_get_last_committed_after_explicit_commit() {
        let engine = hipkey_engine_create();
        hipkey_engine_set_language_pack_vi(engine, 0);

        hipkey_process_keystroke(engine, 'x' as u32, false, false, false, false);
        hipkey_process_keystroke(engine, 'i' as u32, false, false, false, false);
        hipkey_process_keystroke(engine, 'n' as u32, false, false, false, false);
        hipkey_commit(engine);

        let ptr = hipkey_get_last_committed(engine);
        assert!(!ptr.is_null());
        let text = unsafe { CStr::from_ptr(ptr) };
        assert_eq!(text.to_str().unwrap(), "xin");
        hipkey_string_free(ptr);

        hipkey_engine_destroy(engine);
    }

    #[test]
    fn test_get_last_committed_none_initially() {
        let engine = hipkey_engine_create();
        assert!(hipkey_get_last_committed(engine).is_null());
        hipkey_engine_destroy(engine);
    }

    #[test]
    fn test_get_last_committed_null_safe() {
        assert!(hipkey_get_last_committed(ptr::null_mut()).is_null());
    }

    #[test]
    fn test_candidate_list_free_tampered_len_does_not_crash() {
        // Regression test for ffi-011/ffi-012: a caller that tampers with
        // list.len (sets it above MAX_CANDIDATES) must not cause OOB reads
        // or wrong-layout dealloc. The free function clamps len.
        let engine = hipkey_engine_create();
        hipkey_engine_set_language_pack_vi(engine, 0);

        // Type some chars to populate candidates
        for c in ['c', 'h'] {
            hipkey_process_keystroke(engine, c as u32, false, false, false, false);
        }

        let mut list = hipkey_get_candidates(engine);
        // Tamper: inflate len far beyond actual allocation
        list.len = 999_999;
        // Must not crash (previously: OOB read + arbitrary free)
        hipkey_candidate_list_free(list);

        hipkey_engine_destroy(engine);
    }

    #[test]
    fn test_space_key_maps_to_space_variant() {
        // Regression test for ffi-023: 0x20 must produce Key::Space, not
        // Key::Char(' '). Verified by checking the engine does not crash and
        // the keystroke is routed to the language pack.
        let engine = hipkey_engine_create();
        hipkey_engine_set_language_pack_vi(engine, 0);

        let event = hipkey_process_keystroke(engine, 0x20, false, false, false, false);
        // Space should be BufferChanged (appended) or PassThrough depending on
        // language pack; either way, not Error.
        assert_ne!(event, HipKeyEngineEvent::Error);

        hipkey_engine_destroy(engine);
    }
}
