//! Learning/ranking system for candidate suggestions
//!
//! Tracks user word selections locally and adjusts candidate ranking by frequency.
//! All data is stored in memory; persists to file on demand.

use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, Write};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct LearningStore {
    frequencies: HashMap<String, u32>,
    total_selections: u64,
    path: Option<std::path::PathBuf>,
}

impl LearningStore {
    pub fn new() -> Self {
        Self {
            frequencies: HashMap::new(),
            total_selections: 0,
            path: None,
        }
    }

    pub fn load_from_file(path: &Path) -> Result<Self, String> {
        let file = fs::File::open(path).map_err(|e| e.to_string())?;
        let mut store = Self::new();
        store.path = Some(path.to_path_buf());

        for line in std::io::BufReader::new(file).lines() {
            let line = line.map_err(|e| e.to_string())?;
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((word, freq_str)) = line.rsplit_once('\t') {
                if let Ok(freq) = freq_str.trim().parse::<u32>() {
                    store.frequencies.insert(word.trim().to_string(), freq);
                    store.total_selections += freq as u64;
                }
            }
        }

        Ok(store)
    }

    pub fn save(&self) -> Result<(), String> {
        if let Some(path) = &self.path {
            self.save_to(path)
        } else {
            Err("No path set".to_string())
        }
    }

    pub fn save_to(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let mut file = fs::File::create(path).map_err(|e| e.to_string())?;
        writeln!(file, "# hip-key learning data (tab-separated: word\\tfrequency)").map_err(|e| e.to_string())?;
        let mut entries: Vec<_> = self.frequencies.iter().collect();
        entries.sort_by(|a, b| b.1.cmp(a.1));
        for (word, freq) in entries {
            writeln!(file, "{}\t{}", word, freq).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn record_selection(&mut self, word: &str) {
        *self.frequencies.entry(word.to_string()).or_insert(0) += 1;
        self.total_selections += 1;
    }

    pub fn get_frequency(&self, word: &str) -> u32 {
        self.frequencies.get(word).copied().unwrap_or(0)
    }

    pub fn get_score(&self, word: &str) -> f32 {
        if self.total_selections == 0 {
            return 0.0;
        }
        let freq = self.frequencies.get(word).copied().unwrap_or(0) as f64;
        (freq / self.total_selections as f64) as f32
    }

    pub fn boost_candidates(
        &self,
        candidates: &mut Vec<crate::Candidate>,
    ) {
        if self.total_selections == 0 {
            return;
        }
        for candidate in candidates.iter_mut() {
            let learned_score = self.get_score(&candidate.text);
            candidate.confidence = (candidate.confidence * 0.6 + learned_score * 0.4).min(1.0);
        }
        candidates.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));
    }

    pub fn len(&self) -> usize {
        self.frequencies.len()
    }

    pub fn is_empty(&self) -> bool {
        self.frequencies.is_empty()
    }

    pub fn total_selections(&self) -> u64 {
        self.total_selections
    }

    pub fn top_words(&self, n: usize) -> Vec<(String, u32)> {
        let mut entries: Vec<_> = self.frequencies.iter().map(|(w, &f)| (w.clone(), f)).collect();
        entries.sort_by(|a, b| b.1.cmp(&a.1));
        entries.into_iter().take(n).collect()
    }

    pub fn decay(&mut self, factor: f32) {
        let factor = factor.clamp(0.0, 1.0);
        let mut total = 0u64;
        for freq in self.frequencies.values_mut() {
            *freq = (*freq as f32 * factor) as u32;
            total += *freq as u64;
        }
        self.frequencies.retain(|_, f| *f > 0);
        self.total_selections = total;
    }
}

impl Default for LearningStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Candidate;

    #[test]
    fn test_learning_new() {
        let store = LearningStore::new();
        assert!(store.is_empty());
        assert_eq!(store.total_selections(), 0);
    }

    #[test]
    fn test_record_selection() {
        let mut store = LearningStore::new();
        store.record_selection("xin");
        store.record_selection("chào");
        store.record_selection("xin");
        assert_eq!(store.get_frequency("xin"), 2);
        assert_eq!(store.get_frequency("chào"), 1);
        assert_eq!(store.get_frequency("unknown"), 0);
        assert_eq!(store.total_selections(), 3);
    }

    #[test]
    fn test_get_score() {
        let mut store = LearningStore::new();
        store.record_selection("xin");
        store.record_selection("xin");
        store.record_selection("chào");

        let xin_score = store.get_score("xin");
        let chao_score = store.get_score("chào");
        assert!(xin_score > chao_score);
        assert!(xin_score > 0.0);
    }

    #[test]
    fn test_boost_candidates() {
        let mut store = LearningStore::new();
        for _ in 0..20 {
            store.record_selection("chào");
        }

        let mut candidates = vec![
            Candidate::new("xin").with_confidence(0.5),
            Candidate::new("chào").with_confidence(0.1),
        ];

        store.boost_candidates(&mut candidates);
        assert_eq!(candidates[0].text, "chào");
    }

    #[test]
    fn test_top_words() {
        let mut store = LearningStore::new();
        store.record_selection("b");
        store.record_selection("a");
        store.record_selection("a");
        store.record_selection("c");
        store.record_selection("a");
        store.record_selection("b");

        let top = store.top_words(2);
        assert_eq!(top[0].0, "a");
        assert_eq!(top[0].1, 3);
        assert_eq!(top[1].0, "b");
        assert_eq!(top[1].1, 2);
    }

    #[test]
    fn test_decay() {
        let mut store = LearningStore::new();
        store.record_selection("word");
        assert_eq!(store.get_frequency("word"), 1);

        store.decay(0.5);
        assert_eq!(store.get_frequency("word"), 0);
    }

    #[test]
    fn test_save_load_roundtrip() {
        let dir = std::env::temp_dir().join("hip-key-test-learning");
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("learning.tsv");

        let mut store = LearningStore::new();
        store.record_selection("xin");
        store.record_selection("chào");
        store.record_selection("xin");
        store.save_to(&path).unwrap();

        let loaded = LearningStore::load_from_file(&path).unwrap();
        assert_eq!(loaded.get_frequency("xin"), 2);
        assert_eq!(loaded.get_frequency("chào"), 1);
        assert_eq!(loaded.total_selections(), 3);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_load_missing_file() {
        let result = LearningStore::load_from_file(Path::new("/nonexistent/path"));
        assert!(result.is_err());
    }

    #[test]
    fn test_save_no_path() {
        let store = LearningStore::new();
        assert!(store.save().is_err());
    }
}
