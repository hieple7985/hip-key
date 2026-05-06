//! Vietnamese word dictionary with frequency data
//!
//! Embeds a common word list and uses a Trie for efficient prefix lookup.

use crate::trie::Trie;
use hip_key_core::{Candidate, CandidateList};

pub struct Dictionary {
    trie: Trie,
}

impl Dictionary {
    pub fn new() -> Self {
        let mut dict = Self {
            trie: Trie::new(),
        };
        dict.load_builtin_words();
        dict
    }

    pub fn insert(&mut self, word: &str, frequency: u32) {
        self.trie.insert(word, frequency);
    }

    pub fn contains(&self, word: &str) -> bool {
        self.trie.contains(word)
    }

    pub fn len(&self) -> usize {
        self.trie.len()
    }

    pub fn suggest(&self, prefix: &str, max: usize) -> CandidateList {
        let mut results = self.trie.starts_with(prefix);
        results.truncate(max);
        results
            .into_iter()
            .map(|(text, freq)| {
                let confidence = (freq as f32 / 1000.0).min(1.0);
                Candidate::new(text).with_confidence(confidence)
            })
            .collect()
    }

    fn load_builtin_words(&mut self) {
        let words: &[(&str, u32)] = &[
            ("của", 1000), ("và", 980), ("là", 950), ("không", 920),
            ("có", 900), ("được", 880), ("những", 850), ("cho", 830),
            ("này", 810), ("đã", 790), ("với", 770), ("đó", 750),
            ("một", 730), ("tôi", 710), ("anh", 700), ("em", 690),
            ("người", 680), ("nhà", 660), ("năm", 640), ("làm", 620),
            ("đi", 600), ("rằng", 590), ("nhiều", 580), ("về", 570),
            ("từ", 560), ("ra", 550), ("phải", 540), ("khi", 530),
            ("nếu", 520), ("rất", 510), ("mà", 500), ("chúng", 490),
            ("vẫn", 480), ("cũng", 470), ("nơi", 460), ("sẽ", 450),
            ("bị", 440), ("bởi", 430), ("nhưng", 420), ("vào", 410),
            ("lên", 400), ("xuống", 390), ("trên", 380), ("dưới", 370),
            ("sau", 360), ("trước", 350), ("giữa", 340), ("ngoài", 330),
            ("trong", 320), ("hay", 310), ("hoặc", 300), ("nào", 290),
            ("gì", 280), ("ai", 270), ("đâu", 260), ("khi nào", 250),
            ("bao giờ", 240), ("thế nào", 230), ("như thế nào", 220),
            ("tại sao", 210), ("xin", 200), ("chào", 200), ("cảm ơn", 195),
            ("tạm biệt", 190), ("xin lỗi", 185), ("vâng", 180), ("không", 175),
            ("được", 170), ("việc", 165), ("ngày", 160), ("tháng", 155),
            ("năm", 150), ("giờ", 145), ("phút", 140), ("giây", 135),
            ("hôm", 130), ("hôm nay", 125), ("ngày mai", 120), ("hôm qua", 115),
            ("tuần", 110), ("tháng", 105), ("năm", 100), ("thời gian", 100),
            ("thế giới", 95), ("quốc gia", 90), ("thành phố", 85),
            ("đường", 80), ("trường", 80), ("chợ", 80), ("bệnh viện", 75),
            ("sân bay", 70), ("nhà ga", 65), ("bến xe", 60), ("công viên", 55),
            ("biển", 50), ("núi", 50), ("sông", 50), ("hồ", 50),
            ("Việt Nam", 500), ("Hà Nội", 400), ("Sài Gòn", 380),
            ("Đà Nẵng", 350), ("Hải Phòng", 300), ("Cần Thơ", 280),
            ("ăn", 500), ("uống", 450), ("ngủ", 400), ("đi", 450),
            ("chạy", 350), ("đứng", 300), ("ngồi", 300), ("nói", 400),
            ("đọc", 350), ("viết", 350), ("nghe", 350), ("nhìn", 350),
            ("yêu", 400), ("thương", 350), ("ghét", 200), ("muốn", 380),
            ("thích", 360), ("biết", 370), ("hiểu", 350), ("nhớ", 340),
            ("quên", 330), ("nghĩ", 350), ("tin", 300), ("hy vọng", 250),
            ("sợ", 280), ("lo", 260), ("buồn", 300), ("vui", 320),
            ("giận", 250), ("tức", 240), ("hạnh phúc", 280), ("mừng", 200),
            ("tốt", 400), ("xấu", 300), ("đẹp", 380), ("lớn", 350),
            ("nhỏ", 350), ("dài", 300), ("ngắn", 280), ("cao", 300),
            ("thấp", 260), ("rộng", 280), ("hẹp", 240), ("nhanh", 320),
            ("chậm", 280), ("mới", 380), ("cũ", 280), ("nóng", 300),
            ("lạnh", 300), ("ấm", 260), ("mát", 260), ("sạch", 280),
            ("bẩn", 240), ("xa", 280), ("gần", 280), ("nhiều", 350),
            ("ít", 300), ("đúng", 350), ("sai", 300), ("dễ", 320),
            ("khó", 320), ("quan trọng", 340), ("đặc biệt", 280),
            ("bình thường", 260), ("thường", 300), ("hiếm", 200),
            ("mùa xuân", 250), ("mùa hạ", 240), ("mùa thu", 240),
            ("mùa đông", 240), ("mưa", 280), ("nắng", 280), ("gió", 260),
            ("bão", 200), ("mây", 220), ("trời", 300), ("mặt trời", 260),
            ("mặt trăng", 220), ("sao", 220), ("nước", 400),
            ("lửa", 300), ("đất", 300), ("không khí", 250), ("cây", 280),
            ("hoa", 280), ("lá", 240), ("quả", 240), ("rễ", 200),
            ("mèo", 300), ("chó", 300), ("con", 400), ("cá", 260),
            ("chim", 260), ("gà", 260), ("lợn", 240), ("bò", 240),
            ("trâu", 220), ("ngựa", 220), ("voi", 200), ("hổ", 200),
            ("rừng", 260), ("biển", 280), ("sông", 260), ("suối", 200),
            ("cha", 380), ("mẹ", 400), ("ông", 350), ("bà", 350),
            ("anh", 400), ("chị", 380), ("em", 400), ("con", 380),
            ("cháu", 300), ("cô", 300), ("chú", 300), ("bác", 280),
            ("vợ", 350), ("chồng", 350), ("bạn", 380), ("gia đình", 360),
            ("họ", 300), ("người", 450), ("mọi người", 300),
            ("màu", 280), ("đỏ", 280), ("xanh", 280), ("vàng", 280),
            ("trắng", 260), ("đen", 260), ("cam", 240), ("tím", 240),
            ("hồng", 240), ("nâu", 220), ("xám", 220),
            ("một", 500), ("hai", 450), ("ba", 450), ("bốn", 400),
            ("năm", 400), ("sáu", 380), ("bảy", 380), ("tám", 360),
            ("chín", 360), ("mười", 400), ("trăm", 300), ("nghìn", 300),
            ("triệu", 280), ("tỷ", 260), ("đầu tiên", 280), ("cuối cùng", 260),
            ("đến", 400), ("từ", 350), ("ở", 380), ("về", 350),
            ("qua", 300), ("lại", 300), ("mang", 260), ("đưa", 260),
            ("nhận", 260), ("gửi", 240), ("tìm", 280), ("thấy", 280),
            ("mất", 260), ("giữ", 240), ("để", 350), ("bằng", 300),
            ("như", 350), ("vậy", 340), ("thì", 340), ("mà", 330),
            ("nên", 320), ("vì", 310), ("do", 280), ("theo", 300),
            ("để", 290), ("cho", 380), ("để cho", 260),
            ("viết", 350), ("đọc", 350), ("học", 380), ("dạy", 300),
            ("thi", 280), ("trường học", 320), ("lớp học", 280),
            ("giáo viên", 300), ("học sinh", 300), ("sinh viên", 280),
            ("sách", 300), ("bài", 280), ("bút", 260), ("thước", 240),
            ("bảng", 240), ("ghế", 220), ("tủ", 220),
            ("bàn", 260), ("cửa", 260), ("cửa sổ", 240), ("tường", 240),
            ("trần", 220), ("sàn", 220), ("cầu thang", 200),
            ("bữa", 280), ("bữa sáng", 280), ("bữa trưa", 260),
            ("bữa tối", 260), ("bữa ăn", 260), ("cơm", 400),
            ("bánh", 300), ("phở", 300), ("bún", 260), ("miến", 200),
            ("cháo", 240), ("soup", 200), ("canh", 260), ("thịt", 300),
            ("cá", 260), ("trứng", 260), ("rau", 260), ("cải", 220),
            ("khoai", 220), ("cà chua", 240), ("hành", 220), ("tỏi", 200),
            ("gạo", 240), ("muối", 220), ("đường", 240), ("nước mắm", 240),
            ("tương", 200), ("dầu", 220), ("giấm", 200),
            ("bệnh", 300), ("thuốc", 300), ("bác sĩ", 320), ("y tá", 260),
            ("đau", 320), ("sốt", 280), ("ho", 260), ("cảm", 280),
            ("sức khỏe", 300), ("khỏe", 300), ("mệt", 260), ("thể thao", 260),
            ("tập", 260), ("chơi", 300), ("đá bóng", 240), ("bóng đá", 280),
            ("bóng rổ", 220), ("bóng chuyền", 220), ("cờ", 200),
            ("điện thoại", 360), ("máy tính", 340), ("internet", 300),
            ("web", 260), ("email", 240), ("tin nhắn", 280), ("gọi", 280),
            ("ảnh", 280), ("video", 260), ("âm nhạc", 280), ("phim", 260),
            ("báo", 260), ("tin tức", 280), ("truyền hình", 240), ("radio", 200),
            ("tiền", 380), ("giá", 300), ("mua", 350), ("bán", 340),
            ("chi phí", 260), ("đắt", 260), ("rẻ", 260), ("giảm giá", 240),
            ("tặng", 240), ("quà", 260), ("cửa hàng", 280), ("chợ", 260),
            ("siêu thị", 260), ("hóa đơn", 220),
            ("xe", 360), ("ô tô", 320), ("xe máy", 340), ("xe đạp", 280),
            ("tàu", 260), ("máy bay", 280), ("tàu thủy", 220), ("đường", 300),
            ("phố", 280), ("ngõ", 220), ("cầu", 240), ("bến", 220),
            ("công ty", 320), ("làm việc", 300), ("sếp", 280), ("nhân viên", 280),
            ("lương", 280), ("nghỉ", 260), ("kỳ nghỉ", 240),
            ("luật", 260), ("quyền", 260), ("trách nhiệm", 240),
            ("chính phủ", 240), ("bộ", 220), ("ủy ban", 220),
            ("văn hóa", 280), ("lịch sử", 280), ("địa lý", 240),
            ("khoa học", 260), ("công nghệ", 260), ("nghệ thuật", 240),
            ("âm nhạc", 260), ("hội họa", 220), ("văn học", 240),
            ("thơ", 240), ("truyện", 240), ("chuyện", 280),
            ("nghĩa", 240), ("tình", 260), ("trí", 220), ("tâm", 220),
            ("tài", 220), ("dũng", 200),
        ];
        for &(word, freq) in words {
            self.trie.insert(word, freq);
        }
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dictionary_new() {
        let dict = Dictionary::new();
        assert!(dict.len() > 100);
    }

    #[test]
    fn test_dictionary_contains() {
        let dict = Dictionary::new();
        assert!(dict.contains("xin"));
        assert!(dict.contains("chào"));
        assert!(dict.contains("Việt Nam"));
        assert!(dict.contains("cảm ơn"));
        assert!(!dict.contains("xyz123"));
    }

    #[test]
    fn test_dictionary_suggest() {
        let dict = Dictionary::new();
        let suggestions = dict.suggest("xin", 5);
        assert!(!suggestions.is_empty());

        let texts: Vec<&str> = suggestions.iter().map(|c| c.text.as_str()).collect();
        assert!(texts.contains(&"xin"));
    }

    #[test]
    fn test_dictionary_suggest_limit() {
        let dict = Dictionary::new();
        let suggestions = dict.suggest("", 3);
        assert!(suggestions.len() <= 3);
    }

    #[test]
    fn test_dictionary_suggest_prefix() {
        let dict = Dictionary::new();
        let suggestions = dict.suggest("m", 10);
        assert!(!suggestions.is_empty());

        for s in &suggestions {
            assert!(s.text.starts_with('m') || s.text.starts_with('M'));
        }
    }

    #[test]
    fn test_dictionary_custom_insert() {
        let mut dict = Dictionary::new();
        dict.insert("từ_mới", 999);
        assert!(dict.contains("từ_mới"));
        assert_eq!(dict.suggest("từ_m", 5)[0].text, "từ_mới");
    }

    #[test]
    fn test_dictionary_suggest_no_results() {
        let dict = Dictionary::new();
        let suggestions = dict.suggest("zzzzz", 5);
        assert!(suggestions.is_empty());
    }
}
