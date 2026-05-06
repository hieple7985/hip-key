//! Agent system for action automation
//!
//! Detects intent from composing text and executes actions.
//! Zero external dependencies - simple pattern matching.

use std::collections::HashMap;

pub trait AgentAction: Send + Sync {
    fn name(&self) -> &str;
    fn detect(&self, text: &str) -> Option<Intent>;
    fn execute(&self, intent: &Intent) -> ActionResult;
}

#[derive(Debug, Clone)]
pub struct Intent {
    pub action_name: String,
    pub params: HashMap<String, String>,
    pub raw_text: String,
    pub confidence: f32,
}

#[derive(Debug, Clone)]
pub struct ActionResult {
    pub success: bool,
    pub display_text: String,
    pub commit_text: Option<String>,
    pub should_commit: bool,
}

impl ActionResult {
    pub fn none() -> Self {
        Self {
            success: false,
            display_text: String::new(),
            commit_text: None,
            should_commit: false,
        }
    }

    pub fn show(text: &str) -> Self {
        Self {
            success: true,
            display_text: text.to_string(),
            commit_text: None,
            should_commit: false,
        }
    }

    pub fn commit(text: &str) -> Self {
        Self {
            success: true,
            display_text: text.to_string(),
            commit_text: Some(text.to_string()),
            should_commit: true,
        }
    }

    pub fn error(msg: &str) -> Self {
        Self {
            success: false,
            display_text: msg.to_string(),
            commit_text: None,
            should_commit: false,
        }
    }
}

pub struct Agent {
    actions: Vec<Box<dyn AgentAction>>,
    enabled: bool,
}

impl Agent {
    pub fn new() -> Self {
        let mut agent = Self {
            actions: Vec::new(),
            enabled: true,
        };
        agent.register_default_actions();
        agent
    }

    pub fn with_actions(actions: Vec<Box<dyn AgentAction>>) -> Self {
        Self {
            actions,
            enabled: true,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn process(&self, text: &str) -> Option<ActionResult> {
        if !self.enabled || text.is_empty() {
            return None;
        }

        for action in &self.actions {
            if let Some(intent) = action.detect(text) {
                let result = action.execute(&intent);
                if result.success || result.should_commit {
                    return Some(result);
                }
            }
        }
        None
    }

    pub fn detect_intent(&self, text: &str) -> Option<Intent> {
        if !self.enabled || text.is_empty() {
            return None;
        }

        let mut best_intent: Option<Intent> = None;
        let mut best_confidence = 0.0;

        for action in &self.actions {
            if let Some(intent) = action.detect(text) {
                if intent.confidence > best_confidence {
                    best_confidence = intent.confidence;
                    best_intent = Some(intent);
                }
            }
        }
        best_intent
    }

    pub fn actions(&self) -> Vec<&str> {
        self.actions.iter().map(|a| a.name()).collect()
    }

    fn register_default_actions(&mut self) {
        self.actions.push(Box::new(TimeAction));
        self.actions.push(Box::new(DateAction));
        self.actions.push(Box::new(CalcAction));
    }
}

impl Default for Agent {
    fn default() -> Self {
        Self::new()
    }
}

struct TimeAction;

impl AgentAction for TimeAction {
    fn name(&self) -> &str {
        "time"
    }

    fn detect(&self, text: &str) -> Option<Intent> {
        let lower = text.to_lowercase();
        let triggers = [
            "giờ mấy", "giờ bao", "mấy giờ", "what time", "time now",
            "@time", "gio may", "gio bao",
        ];
        for t in &triggers {
            if lower.contains(t) {
                return Some(Intent {
                    action_name: self.name().to_string(),
                    params: HashMap::new(),
                    raw_text: text.to_string(),
                    confidence: 0.9,
                });
            }
        }
        None
    }

    fn execute(&self, _intent: &Intent) -> ActionResult {
        let now = std::time::SystemTime::now();
        let secs = match now.duration_since(std::time::UNIX_EPOCH) {
            Ok(s) => s,
            Err(_) => return ActionResult::error("Time error"),
        };
        let hours = (secs.as_secs() / 3600 % 24 + 7) % 24;
        let minutes = (secs.as_secs() / 60 % 60) as u32;

        let display = format!("🕐 {:02}:{:02}", hours, minutes);
        ActionResult::show(&display)
    }
}

struct DateAction;

impl AgentAction for DateAction {
    fn name(&self) -> &str {
        "date"
    }

    fn detect(&self, text: &str) -> Option<Intent> {
        let lower = text.to_lowercase();
        let triggers = [
            "ngày bao", "ngày mấy", "hôm nay", "hôm qua", "ngày mai",
            "what date", "date now", "@date", "ngay may", "ngay bao",
        ];
        for t in &triggers {
            if lower.contains(t) {
                return Some(Intent {
                    action_name: self.name().to_string(),
                    params: HashMap::new(),
                    raw_text: text.to_string(),
                    confidence: 0.9,
                });
            }
        }
        None
    }

    fn execute(&self, _intent: &Intent) -> ActionResult {
        let now = std::time::SystemTime::now();
        let secs = match now.duration_since(std::time::UNIX_EPOCH) {
            Ok(s) => s,
            Err(_) => return ActionResult::error("Time error"),
        };
        let days = secs.as_secs() / 86400;
        let base = days.saturating_sub(19755);
        let vietnamese_days = ["CN", "T2", "T3", "T4", "T5", "T6", "T7"];
        let day_idx = (base % 7) as usize;
        let day_name = vietnamese_days.get(day_idx).unwrap_or(&"??");

        let day_num = ((days + 4) % 30) + 1;
        let month = ((days / 30 + 3) % 12) + 1;
        let year = 1970 + (days / 365) as i32;

        let display = format!("📅 {} {}/{:02}/{}", day_name, day_num, month, year);
        ActionResult::show(&display)
    }
}

struct CalcAction;

impl AgentAction for CalcAction {
    fn name(&self) -> &str {
        "calculator"
    }

    fn detect(&self, text: &str) -> Option<Intent> {
        let lower = text.to_lowercase().trim().to_string();
        let triggers = ["calc ", "tính ", "calculate "];

        for t in &triggers {
            if lower.starts_with(t) {
                return Some(Intent {
                    action_name: self.name().to_string(),
                    params: HashMap::from([("expr".to_string(), lower[t.len()..].to_string())]),
                    raw_text: text.to_string(),
                    confidence: 0.95,
                });
            }
        }

        let expr_triggers = ["+", "-", "*", "/"];
        for t in &expr_triggers {
            if lower.contains(t) && lower.len() < 30 && lower.chars().all(|c| c.is_ascii_digit() || " +-*/.".contains(c)) {
                return Some(Intent {
                    action_name: self.name().to_string(),
                    params: HashMap::from([("expr".to_string(), text.to_string())]),
                    raw_text: text.to_string(),
                    confidence: 0.8,
                });
            }
        }
        None
    }

    fn execute(&self, intent: &Intent) -> ActionResult {
        let expr = intent.params.get("expr").map(|s| s.as_str()).unwrap_or("");
        match simple_calc(expr) {
            Ok(result) => {
                let display = format!("🔢 {} = {}", expr, result);
                ActionResult::show(&display)
            }
            Err(_) => ActionResult::error("Không tính được"),
        }
    }
}

fn simple_calc(expr: &str) -> Result<String, ()> {
    let expr = expr.replace(" ", "");
    let chars: Vec<char> = expr.chars().collect();
    let mut num_str = String::new();
    let mut nums: Vec<f64> = vec![];
    let mut ops: Vec<char> = vec![];

    for (i, c) in chars.iter().enumerate() {
        if c.is_ascii_digit() || *c == '.' {
            num_str.push(*c);
        } else if *c == '(' {
            if !num_str.is_empty() {
                return Err(());
            }
            ops.push(*c);
        } else if *c == ')' {
            if !num_str.is_empty() {
                nums.push(num_str.parse().map_err(|_| ())?);
                num_str.clear();
            }
            while ops.last() != Some(&'(') {
                let op = ops.pop().ok_or(())?;
                let b = nums.pop().ok_or(())?;
                let a = nums.pop().ok_or(())?;
                nums.push(apply_op(a, op, b));
            }
            ops.pop();
        } else if "+-*/".contains(*c) {
            if !num_str.is_empty() {
                nums.push(num_str.parse().map_err(|_| ())?);
                num_str.clear();
            }
            while let Some(&last) = ops.last() {
                if last == '(' { break; }
                let precedence = |op: char| if op == '+' || op == '-' { 1 } else { 2 };
                if precedence(last) >= precedence(*c) {
                    let op = ops.pop().ok_or(())?;
                    let b = nums.pop().ok_or(())?;
                    let a = nums.pop().ok_or(())?;
                    nums.push(apply_op(a, op, b));
                } else {
                    break;
                }
            }
            ops.push(*c);
        }
    }

    if !num_str.is_empty() {
        nums.push(num_str.parse().map_err(|_| ())?);
    }

    while let Some(op) = ops.pop() {
        if op == '(' { return Err(()); }
        let b = nums.pop().ok_or(())?;
        let a = nums.pop().ok_or(())?;
        nums.push(apply_op(a, op, b));
    }

    let result = nums.pop().ok_or(())?;

    if result.is_nan() || result.is_infinite() {
        return Err(());
    }

    Ok(if result.fract() == 0.0 {
        format!("{}", result as i64)
    } else {
        format!("{:.2}", result)
    })
}

fn apply_op(left: f64, op: char, right: f64) -> f64 {
    match op {
        '+' => left + right,
        '-' => left - right,
        '*' => left * right,
        '/' => if right != 0.0 { left / right } else { 0.0 },
        _ => right,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestAction(&'static str, &'static str);

    impl AgentAction for TestAction {
        fn name(&self) -> &str { self.0 }
        fn detect(&self, text: &str) -> Option<Intent> {
            if text.contains(self.1) {
                Some(Intent {
                    action_name: self.name().to_string(),
                    params: HashMap::new(),
                    raw_text: text.to_string(),
                    confidence: 0.9,
                })
            } else {
                None
            }
        }
        fn execute(&self, _intent: &Intent) -> ActionResult {
            ActionResult::show(self.name())
        }
    }

    #[test]
    fn test_agent_disabled() {
        let mut agent = Agent::new();
        agent.disable();
        assert!(agent.process("giờ mấy rồi").is_none());
    }

    #[test]
    fn test_agent_enabled() {
        let agent = Agent::new();
        assert!(agent.is_enabled());
    }

    #[test]
    fn test_agent_empty() {
        let agent = Agent::new();
        assert!(agent.process("").is_none());
    }

    #[test]
    fn test_action_result_show() {
        let result = ActionResult::show("test");
        assert!(result.success);
        assert_eq!(result.display_text, "test");
        assert!(!result.should_commit);
    }

    #[test]
    fn test_action_result_commit() {
        let result = ActionResult::commit("test");
        assert!(result.success);
        assert!(result.should_commit);
        assert_eq!(result.commit_text, Some("test".to_string()));
    }

    #[test]
    fn test_action_result_none() {
        let result = ActionResult::none();
        assert!(!result.success);
    }

    #[test]
    fn test_agent_actions_list() {
        let agent = Agent::new();
        let actions = agent.actions();
        assert!(actions.contains(&"time"));
        assert!(actions.contains(&"date"));
        assert!(actions.contains(&"calculator"));
    }

    #[test]
    fn test_time_action() {
        let agent = Agent::new();
        let result = agent.process("giờ mấy rồi");
        assert!(result.is_some());
        assert!(result.unwrap().display_text.starts_with("🕐"));
    }

    #[test]
    fn test_date_action() {
        let agent = Agent::new();
        let result = agent.process("hôm nay là ngày bao nhiêu");
        assert!(result.is_some());
        assert!(result.unwrap().display_text.starts_with("📅"));
    }

    #[test]
    fn test_calc_action() {
        let agent = Agent::new();
        let result = agent.process("calc 2 + 2");
        assert!(result.is_some());
        assert!(result.unwrap().display_text.contains("4"));
    }

    #[test]
    fn test_calc_inline() {
        let agent = Agent::new();
        let result = agent.process("10+5");
        assert!(result.is_some());
        assert!(result.unwrap().display_text.contains("15"));
    }

    #[test]
    fn test_calc_complex() {
        let agent = Agent::new();
        let result = agent.process("10+2*5");
        assert!(result.is_some());
        assert!(result.unwrap().display_text.contains("20"));
    }

    #[test]
    fn test_simple_calc() {
        assert_eq!(simple_calc("2+2").unwrap(), "4");
        assert_eq!(simple_calc("10-3").unwrap(), "7");
        assert_eq!(simple_calc("3*4").unwrap(), "12");
        assert_eq!(simple_calc("15/3").unwrap(), "5");
        assert_eq!(simple_calc("10+2*5").unwrap(), "20");
        assert_eq!(simple_calc("(10+2)*5").unwrap(), "60");
        assert_eq!(simple_calc("10.5+2.5").unwrap(), "13");
    }
}