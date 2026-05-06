//! Configuration system for hip-key
//!
//! Simple key-value config with TOML-like syntax.
//! Zero external dependencies.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Config {
    values: HashMap<String, String>,
    path: Option<PathBuf>,
}

impl Config {
    pub fn new() -> Self {
        let mut config = Self {
            values: HashMap::new(),
            path: None,
        };
        config.set_defaults();
        config
    }

    pub fn load_from_file(path: &Path) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path).map_err(|e| ConfigError::Io(e.to_string()))?;
        let mut config = Self {
            values: parse_config(&content),
            path: Some(path.to_path_buf()),
        };
        config.set_defaults_for_missing();
        Ok(config)
    }

    pub fn load_default() -> Self {
        if let Some(path) = Self::default_config_path() {
            if path.exists() {
                return Self::load_from_file(&path).unwrap_or_default();
            }
        }
        Self::new()
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        if let Some(path) = &self.path {
            let content = self.serialize();
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(|e| ConfigError::Io(e.to_string()))?;
            }
            fs::write(path, content).map_err(|e| ConfigError::Io(e.to_string()))?;
        }
        Ok(())
    }

    pub fn save_to(&mut self, path: &Path) -> Result<(), ConfigError> {
        self.path = Some(path.to_path_buf());
        self.save()
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|s| s.as_str())
    }

    pub fn get_or<'a>(&'a self, key: &str, default: &'a str) -> &'a str {
        self.values.get(key).map(|s| s.as_str()).unwrap_or(default)
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.values.get(key).and_then(|v| match v.as_str() {
            "true" | "yes" | "1" => Some(true),
            "false" | "no" | "0" => Some(false),
            _ => None,
        })
    }

    pub fn get_u32(&self, key: &str) -> Option<u32> {
        self.values.get(key).and_then(|v| v.parse().ok())
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.values.insert(key.to_string(), value.to_string());
    }

    pub fn remove(&mut self, key: &str) {
        self.values.remove(key);
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.values.keys()
    }

    pub fn contains(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }

    fn default_config_path() -> Option<PathBuf> {
        if cfg!(target_os = "macos") {
            Some(PathBuf::from(
                dirs_cache("macOS").unwrap_or_else(|| "~/Library/Application Support/hip-key/config.toml".to_string()),
            ))
        } else if cfg!(target_os = "linux") {
            Some(PathBuf::from("~/.config/hip-key/config.toml"))
        } else if cfg!(target_os = "windows") {
            Some(PathBuf::from("%APPDATA%\\hip-key\\config.toml"))
        } else {
            None
        }
    }

    fn set_defaults(&mut self) {
        self.values.insert("input_method".to_string(), "telex".to_string());
        self.values.insert("auto_commit".to_string(), "false".to_string());
        self.values.insert("max_candidates".to_string(), "9".to_string());
        self.values.insert("enable_suggestions".to_string(), "true".to_string());
        self.values.insert("enable_macros".to_string(), "false".to_string());
    }

    fn set_defaults_for_missing(&mut self) {
        let defaults = Self::new();
        for (key, value) in defaults.values {
            if !self.values.contains_key(&key) {
                self.values.insert(key, value);
            }
        }
    }

    fn serialize(&self) -> String {
        let mut out = String::from("# hip-key configuration\n\n");

        let sections = [
            ("input", vec!["input_method", "auto_commit"]),
            ("suggestions", vec!["enable_suggestions", "max_candidates"]),
            ("macros", vec!["enable_macros"]),
        ];

        for (section, keys) in &sections {
            out.push_str(&format!("[{}]\n", section));
            for key in keys {
                if let Some(value) = self.values.get(*key) {
                    out.push_str(&format!("{} = {}\n", key, format_value(value)));
                }
            }
            out.push('\n');
        }

        let known_keys: Vec<&str> = sections.iter().flat_map(|(_, ks)| ks.iter().copied()).collect();
        let custom: Vec<(&String, &String)> = self.values.iter()
            .filter(|(k, _)| !known_keys.contains(&k.as_str()))
            .collect();

        if !custom.is_empty() {
            out.push_str("[custom]\n");
            for (key, value) in custom {
                out.push_str(&format!("{} = {}\n", key, format_value(value)));
            }
        }

        out
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

fn format_value(v: &str) -> String {
    if v.parse::<u32>().is_ok() || v.parse::<bool>().is_ok() {
        v.to_string()
    } else {
        format!("\"{}\"", v)
    }
}

fn dirs_cache(_os: &str) -> Option<String> {
    None
}

fn parse_config(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().to_string();
            let value = value.trim().trim_matches('"').trim().to_string();
            map.insert(key, value);
        }
    }
    map
}

#[derive(Debug)]
pub enum ConfigError {
    Io(String),
    Parse(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Io(msg) => write!(f, "IO error: {}", msg),
            ConfigError::Parse(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        let config = Config::new();
        assert_eq!(config.get("input_method"), Some("telex"));
        assert_eq!(config.get_bool("auto_commit"), Some(false));
        assert_eq!(config.get_u32("max_candidates"), Some(9));
    }

    #[test]
    fn test_config_set_get() {
        let mut config = Config::new();
        config.set("input_method", "vni");
        assert_eq!(config.get("input_method"), Some("vni"));

        config.set("custom_key", "custom_value");
        assert_eq!(config.get("custom_key"), Some("custom_value"));
    }

    #[test]
    fn test_config_get_or() {
        let config = Config::new();
        assert_eq!(config.get_or("nonexistent", "fallback"), "fallback");
        assert_eq!(config.get_or("input_method", "fallback"), "telex");
    }

    #[test]
    fn test_config_get_bool() {
        let mut config = Config::new();
        config.set("flag1", "true");
        config.set("flag2", "false");
        config.set("flag3", "yes");
        config.set("flag4", "no");
        config.set("flag5", "1");
        config.set("flag6", "0");
        config.set("notbool", "hello");

        assert_eq!(config.get_bool("flag1"), Some(true));
        assert_eq!(config.get_bool("flag2"), Some(false));
        assert_eq!(config.get_bool("flag3"), Some(true));
        assert_eq!(config.get_bool("flag4"), Some(false));
        assert_eq!(config.get_bool("flag5"), Some(true));
        assert_eq!(config.get_bool("flag6"), Some(false));
        assert_eq!(config.get_bool("notbool"), None);
    }

    #[test]
    fn test_config_get_u32() {
        let mut config = Config::new();
        config.set("num", "42");
        config.set("notnum", "hello");

        assert_eq!(config.get_u32("num"), Some(42));
        assert_eq!(config.get_u32("notnum"), None);
    }

    #[test]
    fn test_config_remove() {
        let mut config = Config::new();
        assert!(config.contains("input_method"));
        config.remove("input_method");
        assert!(!config.contains("input_method"));
    }

    #[test]
    fn test_parse_config() {
        let content = r#"
# This is a comment
[input]
input_method = "vni"
auto_commit = true

[suggestions]
max_candidates = 5
enable_suggestions = yes
"#;
        let map = parse_config(content);
        assert_eq!(map.get("input_method").unwrap(), "vni");
        assert_eq!(map.get("auto_commit").unwrap(), "true");
        assert_eq!(map.get("max_candidates").unwrap(), "5");
        assert_eq!(map.get("enable_suggestions").unwrap(), "yes");
    }

    #[test]
    fn test_config_serialize() {
        let config = Config::new();
        let serialized = config.serialize();
        assert!(serialized.contains("input_method"));
        assert!(serialized.contains("telex"));
        assert!(serialized.contains("[input]"));
        assert!(serialized.contains("[suggestions]"));
    }

    #[test]
    fn test_config_save_load_roundtrip() {
        let dir = std::env::temp_dir().join("hip-key-test-config");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("config.toml");

        let mut config = Config::new();
        config.set("input_method", "vni");
        config.set("max_candidates", "5");
        config.save_to(&path).unwrap();

        let loaded = Config::load_from_file(&path).unwrap();
        assert_eq!(loaded.get("input_method"), Some("vni"));
        assert_eq!(loaded.get_u32("max_candidates"), Some(5));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_config_defaults_fill_missing() {
        let content = "input_method = \"vni\"\n";
        let map = parse_config(content);
        assert!(!map.contains_key("auto_commit"));

        let mut config = Config {
            values: map,
            path: None,
        };
        config.set_defaults_for_missing();
        assert_eq!(config.get("auto_commit"), Some("false"));
        assert_eq!(config.get("input_method"), Some("vni"));
    }
}
