//! Trie data structure for prefix-based word lookup
//!
//! Zero external dependencies. Supports Unicode (Vietnamese) characters.

use std::collections::BTreeMap;

#[derive(Debug)]
pub struct TrieNode {
    children: BTreeMap<char, TrieNode>,
    is_word: bool,
    frequency: u32,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: BTreeMap::new(),
            is_word: false,
            frequency: 0,
        }
    }
}

#[derive(Debug)]
pub struct Trie {
    root: TrieNode,
    len: usize,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
            len: 0,
        }
    }

    pub fn insert(&mut self, word: &str, frequency: u32) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::new);
        }
        if !node.is_word {
            self.len += 1;
        }
        node.is_word = true;
        node.frequency = frequency;
    }

    pub fn contains(&self, word: &str) -> bool {
        self.get_node(word).map_or(false, |n| n.is_word)
    }

    pub fn get_frequency(&self, word: &str) -> u32 {
        self.get_node(word)
            .map_or(0, |n| if n.is_word { n.frequency } else { 0 })
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn starts_with(&self, prefix: &str) -> Vec<(String, u32)> {
        let mut results = Vec::new();
        if let Some(node) = self.get_node(prefix) {
            self.collect_words(node, prefix.to_string(), &mut results);
        }
        results.sort_by(|a, b| b.1.cmp(&a.1));
        results
    }

    fn get_node(&self, key: &str) -> Option<&TrieNode> {
        let mut node = &self.root;
        for ch in key.chars() {
            match node.children.get(&ch) {
                Some(child) => node = child,
                None => return None,
            }
        }
        Some(node)
    }

    fn collect_words(&self, node: &TrieNode, prefix: String, results: &mut Vec<(String, u32)>) {
        if node.is_word {
            results.push((prefix.clone(), node.frequency));
        }
        for (&ch, child) in &node.children {
            let mut new_prefix = prefix.clone();
            new_prefix.push(ch);
            self.collect_words(child, new_prefix, results);
        }
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_insert_and_contains() {
        let mut trie = Trie::new();
        trie.insert("xin", 100);
        trie.insert("chào", 200);
        trie.insert("việt", 150);

        assert!(trie.contains("xin"));
        assert!(trie.contains("chào"));
        assert!(trie.contains("việt"));
        assert!(!trie.contains("nam"));
    }

    #[test]
    fn test_trie_frequency() {
        let mut trie = Trie::new();
        trie.insert("xin", 100);
        trie.insert("chào", 200);

        assert_eq!(trie.get_frequency("xin"), 100);
        assert_eq!(trie.get_frequency("chào"), 200);
        assert_eq!(trie.get_frequency("unknown"), 0);
    }

    #[test]
    fn test_trie_starts_with() {
        let mut trie = Trie::new();
        trie.insert("xin", 100);
        trie.insert("xinh", 80);
        trie.insert("xứng", 50);
        trie.insert("chào", 200);
        trie.insert("cha", 30);

        let results = trie.starts_with("xi");
        assert_eq!(results.len(), 2);

        let words: Vec<&str> = results.iter().map(|(w, _)| w.as_str()).collect();
        assert!(words.contains(&"xin"));
        assert!(words.contains(&"xinh"));
    }

    #[test]
    fn test_trie_prefix_sorted_by_frequency() {
        let mut trie = Trie::new();
        trie.insert("ab", 10);
        trie.insert("abc", 50);
        trie.insert("abd", 30);

        let results = trie.starts_with("ab");
        assert_eq!(results[0].0, "abc");
        assert_eq!(results[1].0, "abd");
        assert_eq!(results[2].0, "ab");
    }

    #[test]
    fn test_trie_empty_prefix() {
        let mut trie = Trie::new();
        trie.insert("a", 1);
        trie.insert("b", 2);

        let results = trie.starts_with("");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_trie_len() {
        let mut trie = Trie::new();
        assert!(trie.is_empty());

        trie.insert("hello", 1);
        assert_eq!(trie.len(), 1);

        trie.insert("world", 1);
        assert_eq!(trie.len(), 2);

        trie.insert("hello", 99);
        assert_eq!(trie.len(), 2);
    }

    #[test]
    fn test_trie_unicode_vietnamese() {
        let mut trie = Trie::new();
        trie.insert("Việt", 100);
        trie.insert("Việc", 80);
        trie.insert("Viền", 10);

        assert!(trie.contains("Việt"));
        let results = trie.starts_with("Vi");
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].0, "Việt");
    }
}
