//! Candidate word/phrase suggestions

/// A candidate suggestion for completion/composition
///
/// Produced by language packs, consumed by UI layer.
#[derive(Debug, Clone)]
pub struct Candidate {
    /// Display text for this candidate
    pub text: String,
    /// Optional annotation (pinyin, pronunciation, etc.)
    pub annotation: Option<String>,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,
}

impl Candidate {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            annotation: None,
            confidence: 0.5,
        }
    }

    pub fn with_annotation(mut self, annotation: impl Into<String>) -> Self {
        self.annotation = Some(annotation.into());
        self
    }

    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Sort key: higher confidence first
    pub fn sort_key(&self) -> f32 {
        self.confidence
    }
}

impl From<String> for Candidate {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

impl From<&str> for Candidate {
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.annotation == other.annotation
        // Note: confidence is intentionally excluded from comparison
    }
}

impl Eq for Candidate {}

/// Collection of candidates with ordering
pub type CandidateList = Vec<Candidate>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candidate_new() {
        let c = Candidate::new("hello");
        assert_eq!(c.text, "hello");
        assert_eq!(c.confidence, 0.5);
        assert!(c.annotation.is_none());
    }

    #[test]
    fn test_candidate_builder() {
        let c = Candidate::new("xin chào")
            .with_annotation("hello")
            .with_confidence(0.9);
        assert_eq!(c.text, "xin chào");
        assert_eq!(c.annotation.as_deref(), Some("hello"));
        assert_eq!(c.confidence, 0.9);
    }

    #[test]
    fn test_candidate_confidence_clamp() {
        let c = Candidate::new("test").with_confidence(1.5);
        assert_eq!(c.confidence, 1.0);

        let c = Candidate::new("test").with_confidence(-0.5);
        assert_eq!(c.confidence, 0.0);
    }
}
