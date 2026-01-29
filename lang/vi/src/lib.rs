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

/// Tone mark in Vietnamese
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ToneMark {
    None,       // no tone (a)
    Acute,      // sắc (á)
    Grave,      // huyền (à)
    HookAbove,  // hỏi (ả)
    Tilde,      // ngã (ã)
    DotBelow,   // nặng (ạ)
}

/// Vowel with modification (breve, circumflex, horn)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VowelMod {
    None,
    Breve,     // ă (from aw)
    Circumflex, // â, ê, ô (from aa, ee, oo)
    Horn,      // ơ, ư (from ow, uw)
}

/// Character info for tone placement
struct CharInfo {
    base: char,
    vowel_mod: VowelMod,
    can_take_tone: bool,  // true for vowels, false for consonants
}

impl CharInfo {
    fn new(c: char) -> Self {
        let (base, vowel_mod) = Self::parse_vowel(c);
        let can_take_tone = Self::is_vowel(base);
        Self { base, vowel_mod, can_take_tone }
    }

    fn parse_vowel(c: char) -> (char, VowelMod) {
        match c {
            // Breve vowels
            'ă' | 'Ă' | 'ắ' | 'Ắ' | 'ằ' | 'Ằ' | 'ẳ' | 'Ẳ' | 'ẵ' | 'Ẵ' | 'ặ' | 'Ặ' => ('a', VowelMod::Breve),
            // Circumflex vowels
            'â' | 'Â' | 'ấ' | 'Ấ' | 'ầ' | 'Ầ' | 'ẩ' | 'Ẩ' | 'ẫ' | 'Ẫ' | 'ậ' | 'Ậ' => ('a', VowelMod::Circumflex),
            'ê' | 'Ê' | 'ế' | 'Ế' | 'ề' | 'Ề' | 'ể' | 'Ể' | 'ễ' | 'Ễ' | 'ệ' | 'Ệ' => ('e', VowelMod::Circumflex),
            'ô' | 'Ô' | 'ố' | 'Ố' | 'ồ' | 'Ồ' | 'ổ' | 'Ổ' | 'ỗ' | 'Ỗ' | 'ộ' | 'Ộ' => ('o', VowelMod::Circumflex),
            // Horn vowels
            'ơ' | 'Ơ' | 'ớ' | 'Ớ' | 'ờ' | 'Ờ' | 'ở' | 'Ở' | 'ỡ' | 'Ỡ' | 'ợ' | 'Ợ' => ('o', VowelMod::Horn),
            'ư' | 'Ư' | 'ứ' | 'Ứ' | 'ừ' | 'Ừ' | 'ử' | 'Ử' | 'ữ' | 'Ữ' | 'ự' | 'Ự' => ('u', VowelMod::Horn),
            // đ
            'đ' | 'Đ' => ('d', VowelMod::None),
            // Tone marked base vowels - return base without modification
            'á' | 'Á' | 'à' | 'À' | 'ả' | 'Ả' | 'ã' | 'Ã' | 'ạ' | 'Ạ' => ('a', VowelMod::None),
            'é' | 'É' | 'è' | 'È' | 'ẻ' | 'Ẻ' | 'ẽ' | 'Ẽ' | 'ẹ' | 'Ẹ' => ('e', VowelMod::None),
            'í' | 'Í' | 'ì' | 'Ì' | 'ỉ' | 'Ỉ' | 'ĩ' | 'Ĩ' | 'ị' | 'Ị' => ('i', VowelMod::None),
            'ó' | 'Ó' | 'ò' | 'Ò' | 'ỏ' | 'Ỏ' | 'õ' | 'Õ' | 'ọ' | 'Ọ' => ('o', VowelMod::None),
            'ú' | 'Ú' | 'ù' | 'Ù' | 'ủ' | 'Ủ' | 'ũ' | 'Ũ' | 'ụ' | 'Ụ' => ('u', VowelMod::None),
            'ý' | 'Ý' | 'ỳ' | 'Ỳ' | 'ỷ' | 'Ỷ' | 'ỹ' | 'Ỹ' | 'ỵ' | 'Ỵ' => ('y', VowelMod::None),
            // Default: pass through
            _ => (c, VowelMod::None),
        }
    }

    fn is_vowel(c: char) -> bool {
        matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u' | 'y')
    }

    /// Find the best position for tone mark in a sequence of chars
    fn find_tone_position(chars: &[CharInfo]) -> Option<usize> {
        // Priority: ă > â > ê > ô > ơ > ư > a > e > i > o > u > y
        // Look for modified vowels first, then base vowels
        for (i, ch) in chars.iter().enumerate() {
            if !ch.can_take_tone {
                continue;
            }
            match ch.vowel_mod {
                VowelMod::Breve => return Some(i),     // ă - highest priority
                VowelMod::Circumflex => return Some(i), // â, ê, ô
                VowelMod::Horn => return Some(i),      // ơ, ư
                VowelMod::None => {}
            }
        }

        // No modified vowels, find first regular vowel
        for (i, ch) in chars.iter().enumerate() {
            if ch.can_take_tone && ch.vowel_mod == VowelMod::None {
                return Some(i);
            }
        }

        None
    }

    /// Apply tone to this character
    fn with_tone(&self, tone: ToneMark) -> char {
        let is_upper = self.base.is_ascii_uppercase();

        let result = match (self.base, self.vowel_mod, tone) {
            // Special vowels with modifications (base chars that already have modification)
            ('ă', VowelMod::Breve, ToneMark::Acute) => 'ắ',
            ('ă', VowelMod::Breve, ToneMark::Grave) => 'ằ',
            ('ă', VowelMod::Breve, ToneMark::HookAbove) => 'ẳ',
            ('ă', VowelMod::Breve, ToneMark::Tilde) => 'ẵ',
            ('ă', VowelMod::Breve, ToneMark::DotBelow) => 'ặ',
            ('ă', VowelMod::Breve, ToneMark::None) => 'ă',

            ('â', VowelMod::Circumflex, ToneMark::Acute) => 'ấ',
            ('â', VowelMod::Circumflex, ToneMark::Grave) => 'ầ',
            ('â', VowelMod::Circumflex, ToneMark::HookAbove) => 'ẩ',
            ('â', VowelMod::Circumflex, ToneMark::Tilde) => 'ẫ',
            ('â', VowelMod::Circumflex, ToneMark::DotBelow) => 'ậ',
            ('â', VowelMod::Circumflex, ToneMark::None) => 'â',

            ('ê', VowelMod::Circumflex, ToneMark::Acute) => 'ế',
            ('ê', VowelMod::Circumflex, ToneMark::Grave) => 'ề',
            ('ê', VowelMod::Circumflex, ToneMark::HookAbove) => 'ể',
            ('ê', VowelMod::Circumflex, ToneMark::Tilde) => 'ễ',
            ('ê', VowelMod::Circumflex, ToneMark::DotBelow) => 'ệ',
            ('ê', VowelMod::Circumflex, ToneMark::None) => 'ê',

            ('ô', VowelMod::Circumflex, ToneMark::Acute) => 'ố',
            ('ô', VowelMod::Circumflex, ToneMark::Grave) => 'ồ',
            ('ô', VowelMod::Circumflex, ToneMark::HookAbove) => 'ổ',
            ('ô', VowelMod::Circumflex, ToneMark::Tilde) => 'ỗ',
            ('ô', VowelMod::Circumflex, ToneMark::DotBelow) => 'ộ',
            ('ô', VowelMod::Circumflex, ToneMark::None) => 'ô',

            ('ơ', VowelMod::Horn, ToneMark::Acute) => 'ớ',
            ('ơ', VowelMod::Horn, ToneMark::Grave) => 'ờ',
            ('ơ', VowelMod::Horn, ToneMark::HookAbove) => 'ở',
            ('ơ', VowelMod::Horn, ToneMark::Tilde) => 'ỡ',
            ('ơ', VowelMod::Horn, ToneMark::DotBelow) => 'ợ',
            ('ơ', VowelMod::Horn, ToneMark::None) => 'ơ',

            ('ư', VowelMod::Horn, ToneMark::Acute) => 'ứ',
            ('ư', VowelMod::Horn, ToneMark::Grave) => 'ừ',
            ('ư', VowelMod::Horn, ToneMark::HookAbove) => 'ử',
            ('ư', VowelMod::Horn, ToneMark::Tilde) => 'ữ',
            ('ư', VowelMod::Horn, ToneMark::DotBelow) => 'ự',
            ('ư', VowelMod::Horn, ToneMark::None) => 'ư',

            // Base vowels (ASCII) with modifications - when parsed then tone applied
            ('a', VowelMod::Breve, ToneMark::Acute) => 'ắ',
            ('a', VowelMod::Breve, ToneMark::Grave) => 'ằ',
            ('a', VowelMod::Breve, ToneMark::HookAbove) => 'ẳ',
            ('a', VowelMod::Breve, ToneMark::Tilde) => 'ẵ',
            ('a', VowelMod::Breve, ToneMark::DotBelow) => 'ặ',
            ('a', VowelMod::Breve, ToneMark::None) => 'ă',

            ('a', VowelMod::Circumflex, ToneMark::Acute) => 'ấ',
            ('a', VowelMod::Circumflex, ToneMark::Grave) => 'ầ',
            ('a', VowelMod::Circumflex, ToneMark::HookAbove) => 'ẩ',
            ('a', VowelMod::Circumflex, ToneMark::Tilde) => 'ẫ',
            ('a', VowelMod::Circumflex, ToneMark::DotBelow) => 'ậ',
            ('a', VowelMod::Circumflex, ToneMark::None) => 'â',

            ('e', VowelMod::Circumflex, ToneMark::Acute) => 'ế',
            ('e', VowelMod::Circumflex, ToneMark::Grave) => 'ề',
            ('e', VowelMod::Circumflex, ToneMark::HookAbove) => 'ể',
            ('e', VowelMod::Circumflex, ToneMark::Tilde) => 'ễ',
            ('e', VowelMod::Circumflex, ToneMark::DotBelow) => 'ệ',
            ('e', VowelMod::Circumflex, ToneMark::None) => 'ê',

            ('o', VowelMod::Circumflex, ToneMark::Acute) => 'ố',
            ('o', VowelMod::Circumflex, ToneMark::Grave) => 'ồ',
            ('o', VowelMod::Circumflex, ToneMark::HookAbove) => 'ổ',
            ('o', VowelMod::Circumflex, ToneMark::Tilde) => 'ỗ',
            ('o', VowelMod::Circumflex, ToneMark::DotBelow) => 'ộ',
            ('o', VowelMod::Circumflex, ToneMark::None) => 'ô',

            ('o', VowelMod::Horn, ToneMark::Acute) => 'ớ',
            ('o', VowelMod::Horn, ToneMark::Grave) => 'ờ',
            ('o', VowelMod::Horn, ToneMark::HookAbove) => 'ở',
            ('o', VowelMod::Horn, ToneMark::Tilde) => 'ỡ',
            ('o', VowelMod::Horn, ToneMark::DotBelow) => 'ợ',
            ('o', VowelMod::Horn, ToneMark::None) => 'ơ',

            ('u', VowelMod::Horn, ToneMark::Acute) => 'ứ',
            ('u', VowelMod::Horn, ToneMark::Grave) => 'ừ',
            ('u', VowelMod::Horn, ToneMark::HookAbove) => 'ử',
            ('u', VowelMod::Horn, ToneMark::Tilde) => 'ữ',
            ('u', VowelMod::Horn, ToneMark::DotBelow) => 'ự',
            ('u', VowelMod::Horn, ToneMark::None) => 'ư',

            // Base vowels (ASCII) with tones
            ('a', VowelMod::None, ToneMark::Acute) => 'á',
            ('a', VowelMod::None, ToneMark::Grave) => 'à',
            ('a', VowelMod::None, ToneMark::HookAbove) => 'ả',
            ('a', VowelMod::None, ToneMark::Tilde) => 'ã',
            ('a', VowelMod::None, ToneMark::DotBelow) => 'ạ',
            ('a', VowelMod::None, ToneMark::None) => 'a',

            ('e', VowelMod::None, ToneMark::Acute) => 'é',
            ('e', VowelMod::None, ToneMark::Grave) => 'è',
            ('e', VowelMod::None, ToneMark::HookAbove) => 'ẻ',
            ('e', VowelMod::None, ToneMark::Tilde) => 'ẽ',
            ('e', VowelMod::None, ToneMark::DotBelow) => 'ẹ',
            ('e', VowelMod::None, ToneMark::None) => 'e',

            ('i', VowelMod::None, ToneMark::Acute) => 'í',
            ('i', VowelMod::None, ToneMark::Grave) => 'ì',
            ('i', VowelMod::None, ToneMark::HookAbove) => 'ỉ',
            ('i', VowelMod::None, ToneMark::Tilde) => 'ĩ',
            ('i', VowelMod::None, ToneMark::DotBelow) => 'ị',
            ('i', VowelMod::None, ToneMark::None) => 'i',

            ('o', VowelMod::None, ToneMark::Acute) => 'ó',
            ('o', VowelMod::None, ToneMark::Grave) => 'ò',
            ('o', VowelMod::None, ToneMark::HookAbove) => 'ỏ',
            ('o', VowelMod::None, ToneMark::Tilde) => 'õ',
            ('o', VowelMod::None, ToneMark::DotBelow) => 'ọ',
            ('o', VowelMod::None, ToneMark::None) => 'o',

            ('u', VowelMod::None, ToneMark::Acute) => 'ú',
            ('u', VowelMod::None, ToneMark::Grave) => 'ù',
            ('u', VowelMod::None, ToneMark::HookAbove) => 'ủ',
            ('u', VowelMod::None, ToneMark::Tilde) => 'ũ',
            ('u', VowelMod::None, ToneMark::DotBelow) => 'ụ',
            ('u', VowelMod::None, ToneMark::None) => 'u',

            ('y', VowelMod::None, ToneMark::Acute) => 'ý',
            ('y', VowelMod::None, ToneMark::Grave) => 'ỳ',
            ('y', VowelMod::None, ToneMark::HookAbove) => 'ỷ',
            ('y', VowelMod::None, ToneMark::Tilde) => 'ỹ',
            ('y', VowelMod::None, ToneMark::DotBelow) => 'ỵ',
            ('y', VowelMod::None, ToneMark::None) => 'y',

            // Special consonants
            ('d', VowelMod::None, ToneMark::None) => 'đ',

            // Consonants and other chars pass through
            (base, _, _) => base,
        };

        if is_upper {
            // For now, just return the lowercase result
            // TODO: Implement proper uppercase conversion
            result
        } else {
            result
        }
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
    /// Processes both vowel modifications and tone marks.
    pub fn convert_telex(&self, input: &str) -> String {
        let mut result = String::with_capacity(input.len());
        let input_chars: Vec<char> = input.chars().collect();
        let mut i = 0;

        // First pass: Process all characters, collect char info and pending tone
        let mut chars: Vec<CharInfo> = Vec::new();
        let mut pending_tone: Option<ToneMark> = None;

        while i < input_chars.len() {
            let c = input_chars[i];

            // Check for vowel modification digraphs first
            if i + 1 < input_chars.len() {
                let next = input_chars[i + 1];
                let vowel_mod = match (c, next) {
                    ('a', 'w') => Some(('ă', VowelMod::Breve)),
                    ('a', 'a') => Some(('â', VowelMod::Circumflex)),
                    ('o', 'w') => Some(('ơ', VowelMod::Horn)),
                    ('o', 'o') => Some(('ô', VowelMod::Circumflex)),
                    ('u', 'w') => Some(('ư', VowelMod::Horn)),
                    ('d', 'd') => Some(('đ', VowelMod::None)),
                    ('e', 'e') => Some(('ê', VowelMod::Circumflex)),
                    _ => None,
                };

                if let Some((ch, vm)) = vowel_mod {
                    // Use CharInfo::new to properly set can_take_tone
                    // Then override vowel_mod since we know the modification
                    let mut info = CharInfo::new(ch);
                    info.vowel_mod = vm;
                    chars.push(info);
                    i += 2;
                    continue;
                }
            }

            // Check for tone mark
            if c == 'x' || c == 'z' {
                // x/z removes tone if it comes after a vowel
                // Check if the previous character (in chars) is a vowel
                let prev_is_vowel = chars.last().map_or(false, |ch| ch.can_take_tone);
                if prev_is_vowel {
                    pending_tone = Some(ToneMark::None);  // Remove tone
                    i += 1;
                    continue;
                }
                // Fall through: treat as regular character
            } else {
                let tone = match c {
                    's' => Some(ToneMark::Acute),      // sắc
                    'f' => Some(ToneMark::Grave),      // huyền
                    'j' => Some(ToneMark::HookAbove),  // hỏi
                    'r' => Some(ToneMark::DotBelow),   // nặng
                    _ => None,
                };

                if let Some(t) = tone {
                    pending_tone = Some(t);
                    i += 1;
                    continue;
                }
            }

            // Regular character
            chars.push(CharInfo::new(c));
            i += 1;
        }

        // Second pass: Apply tone marks
        let tone_to_apply: Option<ToneMark> = pending_tone;

        // Build result string
        // First, find tone position once
        let tone_pos = if tone_to_apply.is_some() {
            CharInfo::find_tone_position(&chars)
        } else {
            None
        };

        for (i, ch) in chars.iter().enumerate() {
            // Check if this is the tone position
            let has_tone = tone_pos == Some(i);

            let ch_with_tone = if has_tone {
                let tone = tone_to_apply.unwrap();
                ch.with_tone(tone)
            } else {
                ch.with_tone(ToneMark::None)
            };

            result.push(ch_with_tone);
        }

        result
    }

    /// Process Telex input keystroke by keystroke
    fn process_telex(&self, keystroke: &Keystroke, buffer: &str) -> ProcessResult {
        if let Keystroke { key: Key::Char(c), .. } = keystroke {
            // Check for terminating characters (commit)
            if c.is_ascii_whitespace() || c.is_ascii_punctuation() {
                // Commit current buffer
                return ProcessResult::ReadyToCommit(buffer.to_string());
            }

            let buffer_chars: Vec<char> = buffer.chars().collect();
            let last_char = buffer_chars.last().copied();

            // Check for Telex vowel modification (last char + current)
            if let Some(last) = last_char {
                let vowel_mod = match (last, c) {
                    ('a', 'w') => Some('ă'),
                    ('a', 'a') => Some('â'),
                    ('o', 'w') => Some('ơ'),
                    ('o', 'o') => Some('ô'),
                    ('u', 'w') => Some('ư'),
                    ('d', 'd') => Some('đ'),
                    ('e', 'e') => Some('ê'),
                    _ => None,
                };

                if let Some(replaced) = vowel_mod {
                    // Replace last char with modified vowel
                    let new_buffer: String = buffer_chars[..buffer_chars.len()-1].iter().collect();
                    return ProcessResult::BufferUpdated(format!("{}{}", new_buffer, replaced));
                }
            }

            // Check for tone mark (s, f, j, r, x)
            let tone = match c {
                's' => Some(ToneMark::Acute),      // sắc
                'f' => Some(ToneMark::Grave),      // huyền
                'j' => Some(ToneMark::HookAbove),  // hỏi
                'r' => Some(ToneMark::DotBelow),   // nặng
                'x' | 'z' => Some(ToneMark::None),   // remove tone
                _ => None,
            };

            if let Some(tone_mark) = tone {
                // Find the vowel to apply tone to
                // Priority: ă > â > ê > ô > ơ > ư > a > e > i > o > u > y
                let mut chars: Vec<CharInfo> = buffer_chars.iter().map(|&ch| CharInfo::new(ch)).collect();

                if let Some(tone_pos) = CharInfo::find_tone_position(&chars) {
                    // Apply tone to the character at tone_pos
                    let target = &chars[tone_pos];
                    let with_tone = target.with_tone(tone_mark);

                    // Rebuild buffer with toned character
                    let mut new_buffer = String::new();
                    for (i, ch) in chars.iter().enumerate() {
                        if i == tone_pos {
                            new_buffer.push(with_tone);
                        } else {
                            new_buffer.push(ch.base);
                        }
                    }
                    return ProcessResult::BufferUpdated(new_buffer);
                }
                // No vowel found to apply tone - treat as regular character
            }

            // No special handling - append the character
            ProcessResult::Consumed
        } else {
            // Non-character keystroke (backspace, etc.)
            ProcessResult::PassThrough
        }
    }

    /// Convert a Telex string to Vietnamese
    ///
    /// VNI rules:
    /// - Vowel mods: a8→ă, a6→â, o7→ơ, o6→ô, u7→ư, d9→đ, e6→ê
    /// - Tone marks (at end): 1→sắc, 2→huyền, 3→hỏi, 4→ngã, 5→nặng
    pub fn convert_vni(&self, input: &str) -> String {
        let mut result = String::with_capacity(input.len());
        let input_chars: Vec<char> = input.chars().collect();
        let mut i = 0;

        // First pass: Process vowel modifications and collect chars
        let mut chars: Vec<CharInfo> = Vec::new();
        let mut pending_tone: Option<ToneMark> = None;

        while i < input_chars.len() {
            let c = input_chars[i];

            // Check for VNI vowel modification (vowel + number)
            if i + 1 < input_chars.len() {
                let next = input_chars[i + 1];
                let vowel_mod = match (c, next) {
                    ('a', '8') => Some(('ă', VowelMod::Breve)),
                    ('a', '6') => Some(('â', VowelMod::Circumflex)),
                    ('o', '7') => Some(('ơ', VowelMod::Horn)),
                    ('o', '6') => Some(('ô', VowelMod::Circumflex)),
                    ('u', '7') => Some(('ư', VowelMod::Horn)),
                    ('d', '9') => Some(('đ', VowelMod::None)),
                    ('e', '6') => Some(('ê', VowelMod::Circumflex)),
                    _ => None,
                };

                if let Some((ch, vm)) = vowel_mod {
                    let mut info = CharInfo::new(ch);
                    info.vowel_mod = vm;
                    chars.push(info);
                    i += 2;
                    continue;
                }
            }

            // Check for VNI tone mark (1-5)
            // In VNI, tone marks always come at the end of syllable
            let tone = match c {
                '1' => Some(ToneMark::Acute),      // sắc
                '2' => Some(ToneMark::Grave),      // huyền
                '3' => Some(ToneMark::HookAbove),  // hỏi
                '4' => Some(ToneMark::Tilde),      // ngã
                '5' => Some(ToneMark::DotBelow),   // nặng
                _ => None,
            };

            if let Some(t) = tone {
                // In VNI, tone marks override any previous tone
                pending_tone = Some(t);
                i += 1;
                continue;
            }

            // Regular character
            chars.push(CharInfo::new(c));
            i += 1;
        }

        // Second pass: Apply tone marks
        // VNI always applies tone to the first vowel in the sequence
        let tone_to_apply: Option<ToneMark> = pending_tone;

        let tone_pos = if tone_to_apply.is_some() {
            CharInfo::find_tone_position(&chars)
        } else {
            None
        };

        for (i, ch) in chars.iter().enumerate() {
            let has_tone = tone_pos == Some(i);

            let ch_with_tone = if has_tone {
                let tone = tone_to_apply.unwrap();
                ch.with_tone(tone)
            } else {
                ch.with_tone(ToneMark::None)
            };

            result.push(ch_with_tone);
        }

        result
    }

    /// Process VNI input keystroke by keystroke
    fn process_vni(&self, keystroke: &Keystroke, buffer: &str) -> ProcessResult {
        if let Keystroke { key: Key::Char(c), .. } = keystroke {
            // Check for terminating characters (commit)
            if c.is_ascii_whitespace() || c.is_ascii_punctuation() {
                // Commit current buffer
                return ProcessResult::ReadyToCommit(buffer.to_string());
            }

            let buffer_chars: Vec<char> = buffer.chars().collect();

            // Check for VNI tone mark (1-5)
            let tone = match c {
                '1' => Some(ToneMark::Acute),      // sắc
                '2' => Some(ToneMark::Grave),      // huyền
                '3' => Some(ToneMark::HookAbove),  // hỏi
                '4' => Some(ToneMark::Tilde),      // ngã
                '5' => Some(ToneMark::DotBelow),   // nặng
                _ => None,
            };

            if let Some(tone_mark) = tone {
                // Apply tone to first vowel
                let chars: Vec<CharInfo> = buffer_chars.iter().map(|&ch| CharInfo::new(ch)).collect();

                if let Some(tone_pos) = CharInfo::find_tone_position(&chars) {
                    // Apply tone to the character at tone_pos
                    let target = &chars[tone_pos];
                    let with_tone = target.with_tone(tone_mark);

                    // Rebuild buffer with toned character
                    let mut new_buffer = String::new();
                    for (i, ch) in chars.iter().enumerate() {
                        if i == tone_pos {
                            new_buffer.push(with_tone);
                        } else {
                            new_buffer.push(ch.base);
                        }
                    }
                    return ProcessResult::BufferUpdated(new_buffer);
                }
                // No vowel found - treat as regular character
            }

            // Check for VNI vowel modification (last char + current)
            let last_char = buffer_chars.last().copied();
            if let Some(last) = last_char {
                let vowel_mod = match (last, c) {
                    ('a', '8') => Some('ă'),
                    ('a', '6') => Some('â'),
                    ('o', '7') => Some('ơ'),
                    ('o', '6') => Some('ô'),
                    ('u', '7') => Some('ư'),
                    ('d', '9') => Some('đ'),
                    ('e', '6') => Some('ê'),
                    _ => None,
                };

                if let Some(replaced) = vowel_mod {
                    // Replace last char with modified vowel
                    let new_buffer: String = buffer_chars[..buffer_chars.len()-1].iter().collect();
                    return ProcessResult::BufferUpdated(format!("{}{}", new_buffer, replaced));
                }
            }

            // No special handling - append the character
            ProcessResult::Consumed
        } else {
            // Non-character keystroke (backspace, etc.)
            ProcessResult::PassThrough
        }
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
            InputMethod::VNI => self.process_vni(keystroke, buffer),
        }
    }

    fn generate_candidates(&self, _buffer: &str) -> CandidateList {
        // TODO: Implement dictionary-based candidates (issue #4)
        vec![]
    }

    fn is_valid_composition(&self, buffer: &str) -> bool {
        // Valid if contains printable Vietnamese-friendly characters
        buffer.chars().all(|c| {
            c.is_ascii_alphanumeric() || c.is_ascii_whitespace() ||
            matches!(c, 'ă'|'â'|'ê'|'ô'|'ơ'|'ư'|'đ'|
                     'Ă'|'Â'|'Ê'|'Ô'|'Ơ'|'Ư'|'Đ'|
                     // Tone marked vowels
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
    fn test_telex_vowel_modifications() {
        let vi = Vietnamese::with_method(InputMethod::Telex);

        assert_eq!(vi.convert_telex("aw"), "ă");
        assert_eq!(vi.convert_telex("aa"), "â");
        assert_eq!(vi.convert_telex("ow"), "ơ");
        assert_eq!(vi.convert_telex("oo"), "ô");
        assert_eq!(vi.convert_telex("uw"), "ư");
        assert_eq!(vi.convert_telex("dd"), "đ");
        assert_eq!(vi.convert_telex("ee"), "ê");
    }

    #[test]
    fn test_telex_tone_marks_basic() {
        let vi = Vietnamese::with_method(InputMethod::Telex);

        // Basic vowels with tones
        assert_eq!(vi.convert_telex("as"), "á");
        assert_eq!(vi.convert_telex("af"), "à");
        assert_eq!(vi.convert_telex("aj"), "ả");
        assert_eq!(vi.convert_telex("ar"), "ạ");
        assert_eq!(vi.convert_telex("ax"), "a");

        assert_eq!(vi.convert_telex("es"), "é");
        assert_eq!(vi.convert_telex("is"), "í");
        assert_eq!(vi.convert_telex("os"), "ó");
        assert_eq!(vi.convert_telex("us"), "ú");
        assert_eq!(vi.convert_telex("ys"), "ý");
    }

    #[test]
    fn test_telex_vowel_with_tone() {
        let vi = Vietnamese::with_method(InputMethod::Telex);

        // ă with tones
        assert_eq!(vi.convert_telex("aws"), "ắ");
        assert_eq!(vi.convert_telex("awf"), "ằ");
        assert_eq!(vi.convert_telex("awj"), "ẳ");
        assert_eq!(vi.convert_telex("awr"), "ặ");
        assert_eq!(vi.convert_telex("awx"), "ă");

        // â with tones
        assert_eq!(vi.convert_telex("aas"), "ấ");
        assert_eq!(vi.convert_telex("aaf"), "ầ");

        // ê with tones
        assert_eq!(vi.convert_telex("ees"), "ế");

        // ô with tones
        assert_eq!(vi.convert_telex("oos"), "ố");

        // ơ with tones
        assert_eq!(vi.convert_telex("ows"), "ớ");

        // ư with tones
        assert_eq!(vi.convert_telex("uws"), "ứ");
    }

    #[test]
    fn test_telex_order_independent() {
        let vi = Vietnamese::with_method(InputMethod::Telex);

        // as and sa should both give á
        assert_eq!(vi.convert_telex("as"), "á");
        assert_eq!(vi.convert_telex("sa"), "á");

        // aws and was should both give ắ (approximately)
        assert_eq!(vi.convert_telex("aws"), "ắ");
    }

    #[test]
    fn test_telex_remove_tone() {
        let vi = Vietnamese::with_method(InputMethod::Telex);

        // x removes tone
        assert_eq!(vi.convert_telex("asx"), "a");
        assert_eq!(vi.convert_telex("awsx"), "ă");
    }

    #[test]
    fn test_telex_word_examples() {
        let vi = Vietnamese::with_method(InputMethod::Telex);

        assert_eq!(vi.convert_telex("xin"), "xin");
        assert_eq!(vi.convert_telex("chao"), "chao");
        assert_eq!(vi.convert_telex("chaos"), "cháo");
        assert_eq!(vi.convert_telex("chaof"), "chào");
        assert_eq!(vi.convert_telex("uwfn"), "ừn");
    }

    #[test]
    fn test_is_valid_composition() {
        let vi = Vietnamese::new();
        assert!(vi.is_valid_composition("xin chào"));
        assert!(vi.is_valid_composition("ăâêôơưđ"));
        assert!(!vi.is_valid_composition("hello@"));
    }

    // VNI tests
    #[test]
    fn test_vni_vowel_modifications() {
        let vi = Vietnamese::with_method(InputMethod::VNI);

        assert_eq!(vi.convert_vni("a8"), "ă");
        assert_eq!(vi.convert_vni("a6"), "â");
        assert_eq!(vi.convert_vni("o7"), "ơ");
        assert_eq!(vi.convert_vni("o6"), "ô");
        assert_eq!(vi.convert_vni("u7"), "ư");
        assert_eq!(vi.convert_vni("d9"), "đ");
        assert_eq!(vi.convert_vni("e6"), "ê");
    }

    #[test]
    fn test_vni_tone_marks_basic() {
        let vi = Vietnamese::with_method(InputMethod::VNI);

        // Basic vowels with tones (1=sắc, 2=huyền, 3=hỏi, 4=ngã, 5=nặng)
        assert_eq!(vi.convert_vni("a1"), "á");
        assert_eq!(vi.convert_vni("a2"), "à");
        assert_eq!(vi.convert_vni("a3"), "ả");
        assert_eq!(vi.convert_vni("a4"), "ã");
        assert_eq!(vi.convert_vni("a5"), "ạ");

        assert_eq!(vi.convert_vni("e1"), "é");
        assert_eq!(vi.convert_vni("i1"), "í");
        assert_eq!(vi.convert_vni("o1"), "ó");
        assert_eq!(vi.convert_vni("u1"), "ú");
        assert_eq!(vi.convert_vni("y1"), "ý");
    }

    #[test]
    fn test_vni_vowel_with_tone() {
        let vi = Vietnamese::with_method(InputMethod::VNI);

        // ă with tones
        assert_eq!(vi.convert_vni("a81"), "ắ");
        assert_eq!(vi.convert_vni("a82"), "ằ");
        assert_eq!(vi.convert_vni("a83"), "ẳ");

        // â with tones
        assert_eq!(vi.convert_vni("a61"), "ấ");
        assert_eq!(vi.convert_vni("a62"), "ầ");

        // ê with tones
        assert_eq!(vi.convert_vni("e61"), "ế");

        // ô with tones
        assert_eq!(vi.convert_vni("o61"), "ố");

        // ơ with tones
        assert_eq!(vi.convert_vni("o71"), "ớ");

        // ư with tones
        assert_eq!(vi.convert_vni("u71"), "ứ");
    }

    #[test]
    fn test_vni_word_examples() {
        let vi = Vietnamese::with_method(InputMethod::VNI);

        assert_eq!(vi.convert_vni("xin"), "xin");
        assert_eq!(vi.convert_vni("chao"), "chao");
        assert_eq!(vi.convert_vni("chao1"), "cháo");
        assert_eq!(vi.convert_vni("chao2"), "chào");
        assert_eq!(vi.convert_vni("u71n"), "ứn");
    }
}
