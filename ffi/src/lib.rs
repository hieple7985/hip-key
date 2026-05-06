//! C-compatible FFI for hip-key core
//!
//! Provides a stable C API for platform adapters.
//! All strings returned are heap-allocated and must be freed with hipkey_string_free().

use hip_key_core::{
    Engine, EngineEvent, Key, Keystroke, LanguagePack, Modifiers,
};
use hip_key_core::keystroke::ArrowDirection;
#[cfg(test)]
use std::ffi::CStr;
use std::ffi::CString;
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

struct EngineState {
    engine: Engine,
    lang_pack: Option<Box<dyn LanguagePack>>,
    last_event: Option<EngineEvent>,
    last_commit: Option<CString>,
}

#[no_mangle]
pub extern "C" fn hipkey_engine_create() -> *mut HipKeyEngine {
    let state = Box::new(EngineState {
        engine: Engine::new(),
        lang_pack: None,
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

    let key = if key_code >= 0x20 && key_code <= 0x7E {
        Key::Char(key_code as u8 as char)
    } else {
        match key_code {
            0x08 => Key::Backspace,
            0x7F => Key::Delete,
            0x0D => Key::Enter,
            0x1B => Key::Escape,
            0x09 => Key::Tab,
            0x20 => Key::Space,
            0x11 => Key::Arrow(ArrowDirection::Up),
            0x12 => Key::Arrow(ArrowDirection::Down),
            0x13 => Key::Arrow(ArrowDirection::Left),
            0x14 => Key::Arrow(ArrowDirection::Right),
            _ => Key::Unknown(key_code),
        }
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
            let bytes = cstr.as_bytes_with_nul();
            let dup = unsafe { libc::malloc(bytes.len()) as *mut c_char };
            if dup.is_null() {
                return ptr::null_mut();
            }
            unsafe { ptr::copy_nonoverlapping(bytes.as_ptr() as *mut c_char, dup, bytes.len()) };
            dup
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
    let layout = std::alloc::Layout::array::<HipKeyCandidate>(count).unwrap();
    let c_ptr = unsafe { std::alloc::alloc(layout) as *mut HipKeyCandidate };

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

#[no_mangle]
pub extern "C" fn hipkey_candidate_list_free(list: HipKeyCandidateList) {
    if list.candidates.is_null() || list.len == 0 {
        return;
    }
    for i in 0..list.len {
        unsafe {
            let candidate = &*list.candidates.add(i);
            if !candidate.text.is_null() {
                drop(CString::from_raw(candidate.text));
            }
        }
    }
    let layout = std::alloc::Layout::array::<HipKeyCandidate>(list.len).unwrap();
    unsafe { std::alloc::dealloc(list.candidates as *mut u8, layout) };
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
        assert_eq!(hipkey_is_composing(ptr::null_mut()), false);
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
}
