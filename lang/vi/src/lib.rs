//! Vietnamese language pack for hip-key
//!
//! Input methods: Telex, VNI (extensible)

use hip_key_core::{Keystroke, LanguagePack, ProcessResult, CandidateList, Key};

/// Vietnamese input method type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMethod {
    /// Telex input (e.g., aw -> ă, aa -> â)
    Telex,
    /// VNI input (e.g., a8 -> ă, a6 -> â)
    VNI,
}

impl Default for InputMethod {
    fn default() -> Self {
        Self::Telex
    }
}

/// Vietnamese language pack
pub struct Vietnamese {
    method: InputMethod,
}

impl Vietnamese {
    pub fn new() -> Self {
        Self {
            method: InputMethod::default(),
        }
    }

    pub fn with_method(method: InputMethod) -> Self {
        Self { method }
    }

    /// Convert a Telex string to Vietnamese
    ///
    /// This is a convenience function for testing.
    /// The real engine uses process() for keystroke-by-keystroke handling.
    pub fn convert_telex(&self, input: &str) -> String {
        let mut result = String::with_capacity(input.len());

        // Process chars and handle digraphs
        let input_chars: Vec<char> = input.chars().collect();
        let mut i = 0;

        while i < input_chars.len() {
            let c = input_chars[i];

            // Check for digraphs
            if i + 1 < input_chars.len() {
                let next = input_chars[i + 1];
                let replacement = match (c, next) {
                    ('a', 'w') => Some('ă'),
                    ('a', 'a') => Some('â'),
                    ('o', 'w') => Some('ơ'),
                    ('o', 'o') => Some('ô'),
                    ('u', 'w') => Some('ư'),
                    ('d', 'd') => Some('đ'),
                    ('e', 'e') => Some('ê'),
                    _ => None,
                };

                if let Some(replaced) = replacement {
                    result.push(replaced);
                    i += 2;
                    continue;
                }
            }

            // Single character
            result.push(c);
            i += 1;
        }

        result
    }

    /// Process Telex input
    fn process_telex(&self, keystroke: &Keystroke, buffer: &str) -> ProcessResult {
        if let Keystroke { key: Key::Char(c), .. } = keystroke {
            // Telex vowel modifications
            let result = match (buffer.chars().last(), c) {
                // aw -> ă
                (Some('a'), 'w') => Some("ă"),
                // aa -> â
                (Some('a'), 'a') => Some("â"),
                // ow -> ơ
                (Some('o'), 'w') => Some("ơ"),
                // oo -> ô
                (Some('o'), 'o') => Some("ô"),
                // uw -> ư
                (Some('u'), 'w') => Some("ư"),
                // dd -> đ
                (Some('d'), 'd') => Some("đ"),
                // e + e -> ê
                (Some('e'), 'e') => Some("ê"),
                _ => None,
            };

            if let Some(replacement) = result {
                let mut new_buffer = buffer.to_string();
                new_buffer.pop();
                new_buffer.push_str(replacement);
                return ProcessResult::ReadyToCommit(new_buffer);
            }
        }
        ProcessResult::Consumed
    }
}

impl Default for Vietnamese {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguagePack for Vietnamese {
    fn process(&self, keystroke: &Keystroke, buffer: &str) -> ProcessResult {
        match self.method {
            InputMethod::Telex => self.process_telex(keystroke, buffer),
            InputMethod::VNI => {
                // TODO: Implement VNI
                ProcessResult::Consumed
            }
        }
    }

    fn generate_candidates(&self, _buffer: &str) -> CandidateList {
        // TODO: Implement dictionary-based candidates
        vec![]
    }

    fn is_valid_composition(&self, buffer: &str) -> bool {
        // Valid if contains printable Vietnamese-friendly characters
        // Includes: ASCII letters, spaces, Vietnamese specific chars
        buffer.chars().all(|c| {
            c.is_ascii_alphanumeric() || c.is_ascii_whitespace() ||
            matches!(c, 'ă'|'â'|'ê'|'ô'|'ơ'|'ư'|'đ'|
                     'Ă'|'Â'|'Ê'|'Ô'|'Ơ'|'Ư'|'Đ'|
                     // Allow accented characters (Latin Extended)
                     'à'|'á'|'ả'|'ã'|'ạ'|'ằ'|'ắ'|'ẳ'|'ẵ'|'ặ'|'ầ'|'ấ'|'ẩ'|'ẫ'|'ậ'|
                     'À'|'Á'|'Ả'|'Ã'|'Ạ'|'Ằ'|'Ắ'|'Ẳ'|'Ẵ'|'Ặ'|'Ầ'|'Ấ'|'Ẩ'|'Ẫ'|'Ậ'|
                     'è'|'é'|'ẻ'|'ẽ'|'ẹ'|'ề'|'ế'|'ể'|'ễ'|'ệ'|
                     'È'|'É'|'Ẻ'|'Ẽ'|'Ẹ'|'Ề'|'Ế'|'Ể'|'Ễ'|'Ệ'|
                     'ì'|'í'|'ỉ'|'ĩ'|'ị'|'Ì'|'Í'|'Ỉ'|'Ĩ'|'Ị'|
                     'ò'|'ó'|'ỏ'|'õ'|'ọ'|'ồ'|'ố'|'ổ'|'ỗ'|'ộ'|'ờ'|'ớ'|'ở'|'ỡ'|'ợ'|
                     'Ò'|'Ó'|'Ỏ'|'Õ'|'Ọ'|'Ồ'|'Ố'|'Ổ'|'Ỗ'|'Ộ'|'Ờ'|'Ớ'|'Ở'|'Ỡ'|'Ợ'|
                     'ù'|'ú'|'ủ'|'ũ'|'ụ'|'ừ'|'ứ'|'ử'|'ữ'|'ự'|
                     'Ù'|'Ú'|'Ủ'|'Ũ'|'Ụ'|'Ừ'|'Ứ'|'Ử'|'Ữ'|'Ự'|
                     'ỳ'|'ý'|'ỷ'|'ỹ'|'ỵ'|'Ỳ'|'Ý'|'Ỷ'|'Ỹ'|'Ỵ')
        })
    }

    fn id(&self) -> &str {
        "vi"
    }

    fn name(&self) -> &str {
        "Vietnamese"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vietnamese_id() {
        let vi = Vietnamese::new();
        assert_eq!(vi.id(), "vi");
        assert_eq!(vi.name(), "Vietnamese");
    }

    #[test]
    fn test_telex_aw() {
        let vi = Vietnamese::with_method(InputMethod::Telex);
        let result = vi.process(&Keystroke::char('w'), "a");
        assert_eq!(result, ProcessResult::ReadyToCommit(String::from("ă")));
    }

    #[test]
    fn test_telex_aa() {
        let vi = Vietnamese::with_method(InputMethod::Telex);
        let result = vi.process(&Keystroke::char('a'), "a");
        assert_eq!(result, ProcessResult::ReadyToCommit(String::from("â")));
    }

    #[test]
    fn test_telex_dd() {
        let vi = Vietnamese::with_method(InputMethod::Telex);
        let result = vi.process(&Keystroke::char('d'), "d");
        assert_eq!(result, ProcessResult::ReadyToCommit(String::from("đ")));
    }

    #[test]
    fn test_is_valid_composition() {
        let vi = Vietnamese::new();
        assert!(vi.is_valid_composition("xin chào"));
        assert!(vi.is_valid_composition("ăâêôơưđ"));
        assert!(!vi.is_valid_composition("hello@")); // @ not valid
    }
}
