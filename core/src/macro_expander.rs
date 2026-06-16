//! Macro/abbreviation expansion system
//!
//! Allows users to define short abbreviations that expand to longer text.
//! Macros are stored in config and loaded at startup.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MacroExpander {
    macros: HashMap<String, String>,
    enabled: bool,
}

impl MacroExpander {
    pub fn new() -> Self {
        let mut expander = Self {
            macros: HashMap::new(),
            enabled: false,
        };
        expander.load_defaults();
        expander
    }

    pub fn with_macros(macros: HashMap<String, String>) -> Self {
        let mut expander = Self::new();
        for (abbr, expansion) in macros {
            expander.add(abbr, expansion);
        }
        expander
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn add(&mut self, abbreviation: String, expansion: String) {
        self.macros.insert(abbreviation, expansion);
    }

    pub fn remove(&mut self, abbreviation: &str) -> bool {
        self.macros.remove(abbreviation).is_some()
    }

    pub fn expand(&self, text: &str) -> Option<String> {
        if !self.enabled {
            return None;
        }
        self.macros.get(text).cloned()
    }

    pub fn expand_last_word(&self, text: &str) -> (String, bool) {
        if !self.enabled || text.is_empty() {
            return (text.to_string(), false);
        }

        let last_space = text.rfind(' ');
        let last_word = if let Some(pos) = last_space {
            &text[pos + 1..]
        } else {
            text
        };

        if let Some(expansion) = self.macros.get(last_word) {
            let prefix = if let Some(pos) = last_space {
                &text[..=pos]
            } else {
                ""
            };
            return (format!("{}{}", prefix, expansion), true);
        }

        (text.to_string(), false)
    }

    pub fn list(&self) -> impl Iterator<Item = (&String, &String)> {
        self.macros.iter()
    }

    pub fn len(&self) -> usize {
        self.macros.len()
    }

    pub fn is_empty(&self) -> bool {
        self.macros.is_empty()
    }

    fn load_defaults(&mut self) {
        let defaults: &[(&str, &str)] = &[
            ("vn", "Việt Nam"),
            ("hn", "Hà Nội"),
            ("sg", "Sài Gòn"),
            ("dn", "Đà Nẵng"),
            ("hp", "Hải Phòng"),
            ("ct", "Cần Thơ"),
            ("tks", "cảm ơn"),
            ("plz", "làm ơn"),
            ("sry", "xin lỗi"),
            ("hi", "xin chào"),
            ("bye", "tạm biệt"),
        ];
        for &(abbr, expansion) in defaults {
            self.macros.insert(abbr.to_string(), expansion.to_string());
        }
    }
}

impl Default for MacroExpander {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_new() {
        let expander = MacroExpander::new();
        assert!(!expander.enabled());
        assert!(!expander.is_empty());
    }

    #[test]
    fn test_macro_add_remove() {
        let mut expander = MacroExpander::new();
        expander.set_enabled(true);
        expander.add("test".to_string(), "test expansion".to_string());
        assert_eq!(expander.expand("test"), Some("test expansion".to_string()));

        assert!(expander.remove("test"));
        assert_eq!(expander.expand("test"), None);
    }

    #[test]
    fn test_macro_expand_disabled() {
        let mut expander = MacroExpander::new();
        expander.add("vn".to_string(), "Việt Nam".to_string());
        assert!(!expander.enabled());
        assert_eq!(expander.expand("vn"), None);
    }

    #[test]
    fn test_macro_expand_enabled() {
        let mut expander = MacroExpander::new();
        expander.set_enabled(true);
        assert_eq!(expander.expand("vn"), Some("Việt Nam".to_string()));
        assert_eq!(expander.expand("xyz"), None);
    }

    #[test]
    fn test_macro_expand_last_word() {
        let mut expander = MacroExpander::new();
        expander.set_enabled(true);

        let (result, expanded) = expander.expand_last_word("xin vn");
        assert!(expanded);
        assert_eq!(result, "xin Việt Nam");
    }

    #[test]
    fn test_macro_expand_last_word_single() {
        let mut expander = MacroExpander::new();
        expander.set_enabled(true);

        let (result, expanded) = expander.expand_last_word("vn");
        assert!(expanded);
        assert_eq!(result, "Việt Nam");
    }

    #[test]
    fn test_macro_expand_last_word_no_match() {
        let mut expander = MacroExpander::new();
        expander.set_enabled(true);

        let (result, expanded) = expander.expand_last_word("hello world");
        assert!(!expanded);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_macro_with_custom() {
        let mut macros = HashMap::new();
        macros.insert("cmd".to_string(), "command".to_string());
        let expander = MacroExpander::with_macros(macros);

        assert_eq!(expander.len(), 11 + 1); // defaults + 1 custom
    }

    #[test]
    fn test_macro_list() {
        let mut expander = MacroExpander::new();
        expander.add("custom".to_string(), "value".to_string());
        let count = expander.list().count();
        assert!(count > 0);
    }
}
