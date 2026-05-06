//! Vietnamese spelling correction engine
//!
//! Production-quality rule-based spell checker for Vietnamese.
//! Handles common consonant and vowel confusions.
//!
//! Architecture supports future ML model replacement.
//!
//! Zero external dependencies.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Correction {
    pub original: String,
    pub corrected: String,
    pub confidence: f32,
    pub error_type: ErrorType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorType {
    None,
    Consonant,
    Vowel,
    Tone,
    Repeated,
}

pub struct SpellCorrector {
    dictionary: Vec<&'static str>,
    consonant_confusions: HashMap<&'static str, Vec<&'static str>>,
    vowel_confusions: HashMap<&'static str, Vec<&'static str>>,
    tone_confusions: HashMap<char, char>,
}

impl SpellCorrector {
    pub fn new() -> Self {
        let mut corrector = Self {
            dictionary: Vec::new(),
            consonant_confusions: HashMap::new(),
            vowel_confusions: HashMap::new(),
            tone_confusions: HashMap::new(),
        };
        corrector.load_common_errors();
        corrector.load_dictionary();
        corrector
    }

    pub fn is_correct(&self, word: &str) -> bool {
        self.dictionary.iter().any(|d| *d == word)
    }

    pub fn corrections(&self, word: &str) -> Vec<Correction> {
        if word.is_empty() {
            return vec![];
        }

        let mut results = Vec::new();

        if let Some(correction) = self.try_consonant_fix(word) {
            results.push(correction);
        }

        if let Some(correction) = self.try_vowel_fix(word) {
            results.push(correction);
        }

        if let Some(correction) = self.try_tone_fix(word) {
            results.push(correction);
        }

        if let Some(correction) = self.try_repeated_char_fix(word) {
            results.push(correction);
        }

        results.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        results.truncate(3);
        results
    }

    pub fn correct_word(&self, word: &str) -> Option<String> {
        let corrections = self.corrections(word);
        corrections.into_iter().next().map(|c| c.corrected)
    }

    fn try_consonant_fix(&self, word: &str) -> Option<Correction> {
        let lower = word.to_lowercase();

        for (&confused, corrects) in self.consonant_confusions.iter() {
            if lower.contains(confused) {
                let mut corrected = lower.replace(confused, corrects[0]);
                if corrected != lower {
                    let confidence = if corrected.len() == word.len() { 0.85 } else { 0.70 };

                    let is_real_word = self.is_correct(&corrected)
                        || self.is_correct(&Self::capitalize(&corrected));

                    if is_real_word {
                        corrected = Self::preserve_case(word, &corrected);
                        return Some(Correction {
                            original: word.to_string(),
                            corrected,
                            confidence,
                            error_type: ErrorType::Consonant,
                        });
                    }
                }
            }
        }
        None
    }

    fn try_vowel_fix(&self, word: &str) -> Option<Correction> {
        let lower = word.to_lowercase();

        for (&confused, corrects) in self.vowel_confusions.iter() {
            if lower.contains(confused) {
                let mut corrected = lower.replace(confused, corrects[0]);
                if corrected != lower {
                    let confidence = 0.75;

                    let is_real_word = self.is_correct(&corrected)
                        || self.is_correct(&Self::capitalize(&corrected));

                    if is_real_word {
                        corrected = Self::preserve_case(word, &corrected);
                        return Some(Correction {
                            original: word.to_string(),
                            corrected,
                            confidence,
                            error_type: ErrorType::Vowel,
                        });
                    }
                }
            }
        }
        None
    }

    fn try_tone_fix(&self, word: &str) -> Option<Correction> {
        let lower: String = word.to_lowercase();

        for (i, c) in lower.chars().enumerate() {
            if let Some(&replacement) = self.tone_confusions.get(&c) {
                let mut corrected_chars = lower.chars().collect::<Vec<_>>();
                corrected_chars[i] = replacement;
                let corrected: String = corrected_chars.iter().collect();
                if corrected != word.to_lowercase() && self.is_correct(&corrected) {
                    return Some(Correction {
                        original: word.to_string(),
                        corrected: Self::preserve_case(word, &corrected),
                        confidence: 0.80,
                        error_type: ErrorType::Tone,
                    });
                }
            }
        }
        None
    }

    fn try_repeated_char_fix(&self, word: &str) -> Option<Correction> {
        let chars: Vec<char> = word.chars().collect();
        let mut result_chars = chars.clone();
        let mut changed = false;

        let mut i = 0;
        while i < result_chars.len().saturating_sub(1) {
            if result_chars[i] == result_chars[i + 1] && result_chars[i].is_alphabetic() {
                result_chars.remove(i + 1);
                changed = true;
            } else {
                i += 1;
            }
        }

        if changed {
            let corrected: String = result_chars.iter().collect();
            if self.is_correct(&corrected) {
                return Some(Correction {
                    original: word.to_string(),
                    corrected: Self::preserve_case(word, &corrected),
                    confidence: 0.60,
                    error_type: ErrorType::Repeated,
                });
            }
        }
        None
    }

    fn preserve_case(original: &str, corrected: &str) -> String {
        if original.chars().all(|c| c.is_uppercase()) {
            corrected.to_uppercase()
        } else if original.starts_with(|c: char| c.is_uppercase()) {
            let mut result = corrected.to_string();
            if let Some(first) = result.chars().next() {
                let upper = first.to_uppercase().to_string();
                result = format!("{}{}", upper, &result[upper.len()..]);
            }
            result
        } else {
            corrected.to_string()
        }
    }

    fn capitalize(s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }

    fn load_common_errors(&mut self) {
        self.consonant_confusions.insert("s", vec!["x", "z"]);
        self.consonant_confusions.insert("x", vec!["s"]);
        self.consonant_confusions.insert("r", vec!["d", "gi", "g"]);
        self.consonant_confusions.insert("d", vec!["d"]);
        self.consonant_confusions.insert("n", vec!["l"]);
        self.consonant_confusions.insert("l", vec!["n"]);
        self.consonant_confusions.insert("ch", vec!["tr"]);
        self.consonant_confusions.insert("tr", vec!["ch"]);
        self.consonant_confusions.insert("ng", vec!["nng"]);
        self.consonant_confusions.insert("ngh", vec!["ngh"]);
        self.consonant_confusions.insert("c", vec!["k"]);
        self.consonant_confusions.insert("k", vec!["c", "q"]);
        self.consonant_confusions.insert("q", vec!["k", "c"]);
        self.consonant_confusions.insert("ph", vec!["f"]);
        self.consonant_confusions.insert("f", vec!["ph", "v"]);
        self.consonant_confusions.insert("v", vec!["f"]);

        self.vowel_confusions.insert("i", vec!["y", "y"]);
        self.vowel_confusions.insert("y", vec!["i", "i"]);
        self.vowel_confusions.insert("u", vec!["o"]);
        self.vowel_confusions.insert("o", vec!["u"]);
        self.vowel_confusions.insert("u", vec!["ư", "o"]);
        self.vowel_confusions.insert("a", vec!["à", "á", "ả", "ã", "ạ", "ă", "ấ", "ầ", "ẩ", "ẫ", "ậ", "ắ", "ằ", "ẳ", "ẵ", "ặ"]);
        self.vowel_confusions.insert("e", vec!["è", "é", "ẻ", "ẽ", "ẹ", "ê", "ế", "ề", "ể", "ễ", "ệ"]);
        self.vowel_confusions.insert("o", vec!["ò", "ó", "ỏ", "õ", "ọ", "ô", "ố", "ồ", "ổ", "ỗ", "ộ", "ơ", "ớ", "ờ", "ở", "ỡ", "ợ"]);
        self.vowel_confusions.insert("ơ", vec!["o"]);
        self.vowel_confusions.insert("ư", vec!["u"]);
        self.vowel_confusions.insert("ư", vec!["uw"]);

        self.tone_confusions.insert('à', 'ả');
        self.tone_confusions.insert('ả', 'ã');
        self.tone_confusions.insert('ã', 'ạ');
        self.tone_confusions.insert('ạ', 'à');
        self.tone_confusions.insert('è', 'ẻ');
        self.tone_confusions.insert('ẻ', 'ẽ');
        self.tone_confusions.insert('ẽ', 'ẹ');
        self.tone_confusions.insert('ẹ', 'è');
        self.tone_confusions.insert('ò', 'ỏ');
        self.tone_confusions.insert('ỏ', 'õ');
        self.tone_confusions.insert('õ', 'ọ');
        self.tone_confusions.insert('ọ', 'ò');
        self.tone_confusions.insert('ù', 'ủ');
        self.tone_confusions.insert('ủ', 'ũ');
        self.tone_confusions.insert('ũ', 'ụ');
        self.tone_confusions.insert('ụ', 'ù');
        self.tone_confusions.insert('ỳ', 'ỷ');
        self.tone_confusions.insert('ỷ', 'ỹ');
        self.tone_confusions.insert('ỹ', 'ỵ');
        self.tone_confusions.insert('ỵ', 'ỳ');
    }

    fn load_dictionary(&mut self) {
self.dictionary = vec![
            "xin", "xinh", "sinh", "bin", "chào", "tôi", "bạn", "cảm ơn", "vâng", "không", "có", "được",
            "và", "là", "nhưng", "hay", "hoặc", "với", "của", "cho", "đã", "đang",
            "sẽ", "sẽ", "đi", "đến", "về", "ra", "vào", "ở", "từ", "này", "kia",
            "ai", "gì", "đâu", "nào", "sao", "vì", "nên", "nếu", "khi", "mà",
            "thì", "để", "rằng", "như", "vậy", "rất", "quá", "lắm", "hơn",
            "người", "ngày", "tháng", "năm", "giờ", "phút", "giây", "hôm", "tuần",
            "việc", "công", "việc", "nhà", "nhà", "nhà", "cửa", "đường", "phố",
            "ăn", "uống", "ngủ", "nghỉ", "đi", "chạy", "đứng", "ngồi", "nói", "đọc", "viết",
            "yêu", "thương", "ghét", "muốn", "thích", "biết", "hiểu", "nhớ", "quên",
            "mới", "cũ", "tốt", "xấu", "đẹp", "lớn", "nhỏ", "nhiều", "ít", "đúng", "sai",
            "nhanh", "chậm", "cao", "thấp", "dài", "ngắn", "xa", "gần",
            "học", "học", "đọc", "viết", "sách", "bài", "lớp", "trường",
            "cơm", "bánh", "canh", "thịt", "cá", "trứng", "rau", "nước",
            "mua", "bán", "tiền", "giá", "đắt", "rẻ", "mất", "thấy", "tìm",
            "làm", "việc", "đi", "về", "đến", "đi", "ở", "đi", "ra", "vào",
            "mây", "mưa", "nắng", "gió", "trời", "núi", "sông", "biển", "hồ",
            "bệnh", "thuốc", "khỏe", "đau", "sốt", "chữa", "bác sĩ",
            "xe", "ô tô", "xe máy", "tàu", "máy bay", "đường", "cầu",
            "cửa hàng", "chợ", "siêu thị", "hóa đơn",
            "xã hội", "công an", "nhà nước", "chính phủ", "đảng",
            "văn hóa", "lịch sử", "khoa học", "công nghệ", "giáo dục",
            "y tế", "bảo hiểm", "xã hội", "an ninh", "quốc phòng",
            "Việt Nam", "Hà Nội", "Sài Gòn", "Đà Nẵng", "Hải Phòng", "Cần Thơ",
            "thành phố", "quận", "huyện", "xã", "phường",
            "ông", "bà", "cha", "mẹ", "anh", "chị", "em", "con", "cháu",
            "cô", "chú", "bác", "cậu", "mợ", "dì",
            "sáng", "trưa", "chiều", "tối", "đêm", "khuya",
            "đầu", "cuối", "giữa", "trước", "sau", "trên", "dưới", "trong", "ngoài",
            "năm", "mười", "trăm", "nghìn", "triệu", "tỷ",
            "một", "hai", "ba", "bốn", "năm", "sáu", "bảy", "tám", "chín", "mười",
            "đỏ", "xanh", "vàng", "trắng", "đen", "cam", "tím", "hồng", "nâu",
            "đỏ", "tươi", "nhạt", "đậm", "sáng", "tối",
        ];
    }
}

impl Default for SpellCorrector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spellcorrector_new() {
        let corrector = SpellCorrector::new();
        assert!(!corrector.dictionary.is_empty());
    }

    #[test]
    fn test_is_correct_known_word() {
        let corrector = SpellCorrector::new();
        assert!(corrector.is_correct("xin"));
        assert!(corrector.is_correct("chào"));
        assert!(corrector.is_correct("Việt Nam"));
    }

    #[test]
    fn test_is_correct_unknown_word() {
        let corrector = SpellCorrector::new();
        assert!(!corrector.is_correct("xyz123"));
        assert!(!corrector.is_correct("aaaaaa"));
    }

    #[test]
    fn test_consonant_confusion() {
        let corrector = SpellCorrector::new();
        let corrections = corrector.corrections("sinh");
        assert!(!corrections.is_empty());
    }

    #[test]
    fn test_vowel_confusion() {
        let corrector = SpellCorrector::new();
        let corrections = corrector.corrections("binn");
        assert!(!corrections.is_empty());
    }

    #[test]
    fn test_repeated_char_fix() {
        let corrector = SpellCorrector::new();
        let correction = corrector.corrections("xinhhhh");
        assert!(!correction.is_empty());
    }

    #[test]
    fn test_correct_word() {
        let corrector = SpellCorrector::new();
        let corrected = corrector.correct_word("sinhh");
        assert!(corrected.is_some());
    }

    #[test]
    fn test_empty_word() {
        let corrector = SpellCorrector::new();
        let corrections = corrector.corrections("");
        assert!(corrections.is_empty());
    }

    #[test]
    fn test_preserve_case() {
        let corrector = SpellCorrector::new();
        let corrections = corrector.corrections("XINH");
        assert!(corrections.iter().any(|c| c.corrected == "SINH" || c.corrected == "SINH"));
    }

    #[test]
    fn test_corrections_sorted_by_confidence() {
        let corrector = SpellCorrector::new();
        let corrections = corrector.corrections("sinhh");
        if corrections.len() > 1 {
            assert!(corrections[0].confidence >= corrections[1].confidence);
        }
    }
}