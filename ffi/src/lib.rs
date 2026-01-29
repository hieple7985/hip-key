//! C-compatible FFI for hip-key core
//!
//! Provides a stable C API for platform adapters.


/// Opaque handle to Engine instance
#[repr(C)]
pub struct HipKeyEngine {
    _private: [u8; 0],
}

/// Result codes for FFI operations
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HipKeyResult {
    Success = 0,
    Error = -1,
    InvalidArgument = -2,
    NotReady = -3,
}

// TODO: Implement full C API
// - Engine creation/destruction
// - Keystroke processing
// - Buffer access
// - Candidate retrieval

#[no_mangle]
pub extern "C" fn hipkey_engine_create() -> *mut HipKeyEngine {
    // Placeholder: return null until implemented
    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn hipkey_engine_destroy(_engine: *mut HipKeyEngine) {
    // Placeholder
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_codes() {
        assert_eq!(HipKeyResult::Success as i32, 0);
        assert_eq!(HipKeyResult::Error as i32, -1);
    }
}
