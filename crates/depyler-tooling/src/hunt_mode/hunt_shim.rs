//! Hunt Mode Shim - pure logic separated from I/O
//!
//! Extracts testable logic from hunt_mode components

use std::collections::HashMap;

/// Failure pattern classification
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternClass {
    TypeMismatch,
    MissingImport,
    BorrowError,
    LifetimeError,
    TraitBound,
    SyntaxError,
    Unknown,
}

impl PatternClass {
    /// Classify from error code
    pub fn from_error_code(code: &str) -> Self {
        match code {
            "E0308" => Self::TypeMismatch,
            "E0412" | "E0425" | "E0433" => Self::MissingImport,
            "E0502" | "E0503" | "E0505" | "E0382" => Self::BorrowError,
            "E0106" | "E0621" => Self::LifetimeError,
            "E0277" => Self::TraitBound,
            "E0061" => Self::SyntaxError,
            _ => Self::Unknown,
        }
    }

    /// Get priority for this pattern class
    pub fn priority(&self) -> u8 {
        match self {
            Self::TypeMismatch => 1,
            Self::TraitBound => 2,
            Self::BorrowError => 3,
            Self::LifetimeError => 4,
            Self::MissingImport => 5,
            Self::SyntaxError => 6,
            Self::Unknown => 10,
        }
    }

    /// Get repair difficulty estimate
    pub fn repair_difficulty(&self) -> f64 {
        match self {
            Self::TypeMismatch => 0.3,
            Self::MissingImport => 0.2,
            Self::SyntaxError => 0.4,
            Self::TraitBound => 0.6,
            Self::BorrowError => 0.7,
            Self::LifetimeError => 0.8,
            Self::Unknown => 1.0,
        }
    }
}

/// Kaizen metrics for tracking improvement
#[derive(Debug, Clone, Default)]
pub struct KaizenSnapshot {
    pub cycles_completed: u32,
    pub fixes_applied: u32,
    pub fixes_rejected: u32,
    pub compilation_rate: f64,
    pub improvement_delta: f64,
}

impl KaizenSnapshot {
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate success rate
    pub fn success_rate(&self) -> f64 {
        let total = self.fixes_applied + self.fixes_rejected;
        if total == 0 {
            0.0
        } else {
            self.fixes_applied as f64 / total as f64
        }
    }

    /// Check if plateaued (no improvement)
    pub fn is_plateaued(&self, threshold: f64) -> bool {
        self.improvement_delta.abs() < threshold
    }

    /// Record a cycle result
    pub fn record_cycle(&mut self, success: bool, new_rate: f64) {
        self.cycles_completed += 1;
        let old_rate = self.compilation_rate;
        self.compilation_rate = new_rate;
        self.improvement_delta = new_rate - old_rate;

        if success {
            self.fixes_applied += 1;
        } else {
            self.fixes_rejected += 1;
        }
    }
}

/// Five Whys analysis step
#[derive(Debug, Clone)]
pub struct WhyAnalysis {
    pub level: u8,
    pub question: String,
    pub answer: String,
    pub is_root_cause: bool,
}

impl WhyAnalysis {
    pub fn new(level: u8, question: impl Into<String>, answer: impl Into<String>) -> Self {
        Self {
            level,
            question: question.into(),
            answer: answer.into(),
            is_root_cause: false,
        }
    }

    pub fn mark_root_cause(mut self) -> Self {
        self.is_root_cause = true;
        self
    }
}

/// Root cause chain from Five Whys analysis
#[derive(Debug, Clone, Default)]
pub struct RootCauseAnalysis {
    pub steps: Vec<WhyAnalysis>,
    pub root_cause: Option<String>,
    pub suggested_fix: Option<String>,
}

impl RootCauseAnalysis {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_step(&mut self, step: WhyAnalysis) {
        if step.is_root_cause {
            self.root_cause = Some(step.answer.clone());
        }
        self.steps.push(step);
    }

    pub fn depth(&self) -> usize {
        self.steps.len()
    }

    pub fn has_root_cause(&self) -> bool {
        self.root_cause.is_some()
    }
}

/// Repair confidence levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConfidenceLevel {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}

impl ConfidenceLevel {
    pub fn from_score(score: f64) -> Self {
        if score >= 0.95 {
            Self::VeryHigh
        } else if score >= 0.85 {
            Self::High
        } else if score >= 0.70 {
            Self::Medium
        } else if score >= 0.50 {
            Self::Low
        } else {
            Self::VeryLow
        }
    }

    pub fn to_score(&self) -> f64 {
        match self {
            Self::VeryLow => 0.3,
            Self::Low => 0.5,
            Self::Medium => 0.7,
            Self::High => 0.85,
            Self::VeryHigh => 0.95,
        }
    }

    pub fn allows_auto_apply(&self, threshold: f64) -> bool {
        self.to_score() >= threshold
    }
}

/// Error cluster for pattern grouping
#[derive(Debug, Clone)]
pub struct ErrorClusterStats {
    pub pattern_class: PatternClass,
    pub count: usize,
    pub sample_codes: Vec<String>,
    pub affected_files: Vec<String>,
}

impl ErrorClusterStats {
    pub fn new(pattern_class: PatternClass) -> Self {
        Self {
            pattern_class,
            count: 0,
            sample_codes: Vec::new(),
            affected_files: Vec::new(),
        }
    }

    pub fn add_occurrence(&mut self, code: &str, file: &str) {
        self.count += 1;
        if self.sample_codes.len() < 5 && !self.sample_codes.contains(&code.to_string()) {
            self.sample_codes.push(code.to_string());
        }
        if self.affected_files.len() < 10 && !self.affected_files.contains(&file.to_string()) {
            self.affected_files.push(file.to_string());
        }
    }

    pub fn priority_score(&self) -> f64 {
        let base = self.pattern_class.priority() as f64;
        let count_factor = (self.count as f64).log2().max(1.0);
        count_factor / base
    }
}

/// Cluster errors by pattern class
pub fn cluster_errors(
    errors: &[(String, String, String)],
) -> HashMap<PatternClass, ErrorClusterStats> {
    let mut clusters: HashMap<PatternClass, ErrorClusterStats> = HashMap::new();

    for (code, _message, file) in errors {
        let class = PatternClass::from_error_code(code);
        let stats = clusters
            .entry(class.clone())
            .or_insert_with(|| ErrorClusterStats::new(class));
        stats.add_occurrence(code, file);
    }

    clusters
}

/// Select highest priority cluster
pub fn select_priority_cluster(
    clusters: &HashMap<PatternClass, ErrorClusterStats>,
) -> Option<&ErrorClusterStats> {
    clusters.values().max_by(|a, b| {
        a.priority_score()
            .partial_cmp(&b.priority_score())
            .unwrap_or(std::cmp::Ordering::Equal)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_class_from_error_code() {
        assert_eq!(
            PatternClass::from_error_code("E0308"),
            PatternClass::TypeMismatch
        );
        assert_eq!(
            PatternClass::from_error_code("E0412"),
            PatternClass::MissingImport
        );
        assert_eq!(
            PatternClass::from_error_code("E0502"),
            PatternClass::BorrowError
        );
        assert_eq!(
            PatternClass::from_error_code("E0106"),
            PatternClass::LifetimeError
        );
        assert_eq!(
            PatternClass::from_error_code("E0277"),
            PatternClass::TraitBound
        );
        assert_eq!(
            PatternClass::from_error_code("E0061"),
            PatternClass::SyntaxError
        );
        assert_eq!(
            PatternClass::from_error_code("E9999"),
            PatternClass::Unknown
        );
    }

    #[test]
    fn test_pattern_class_priority() {
        assert!(PatternClass::TypeMismatch.priority() < PatternClass::Unknown.priority());
        assert!(PatternClass::TraitBound.priority() < PatternClass::BorrowError.priority());
    }

    #[test]
    fn test_pattern_class_repair_difficulty() {
        assert!(
            PatternClass::MissingImport.repair_difficulty()
                < PatternClass::LifetimeError.repair_difficulty()
        );
    }

    #[test]
    fn test_kaizen_snapshot_new() {
        let snapshot = KaizenSnapshot::new();
        assert_eq!(snapshot.cycles_completed, 0);
        assert_eq!(snapshot.fixes_applied, 0);
        assert_eq!(snapshot.compilation_rate, 0.0);
    }

    #[test]
    fn test_kaizen_snapshot_success_rate() {
        let mut snapshot = KaizenSnapshot::new();
        snapshot.fixes_applied = 7;
        snapshot.fixes_rejected = 3;
        assert!((snapshot.success_rate() - 0.7).abs() < 0.001);
    }

    #[test]
    fn test_kaizen_snapshot_success_rate_zero() {
        let snapshot = KaizenSnapshot::new();
        assert_eq!(snapshot.success_rate(), 0.0);
    }

    #[test]
    fn test_kaizen_snapshot_record_cycle() {
        let mut snapshot = KaizenSnapshot::new();
        snapshot.record_cycle(true, 0.5);
        assert_eq!(snapshot.cycles_completed, 1);
        assert_eq!(snapshot.fixes_applied, 1);
        assert_eq!(snapshot.compilation_rate, 0.5);

        snapshot.record_cycle(false, 0.6);
        assert_eq!(snapshot.cycles_completed, 2);
        assert_eq!(snapshot.fixes_rejected, 1);
        assert!((snapshot.improvement_delta - 0.1).abs() < 0.001);
    }

    #[test]
    fn test_kaizen_snapshot_is_plateaued() {
        let mut snapshot = KaizenSnapshot::new();
        snapshot.improvement_delta = 0.001;
        assert!(snapshot.is_plateaued(0.01));
        assert!(!snapshot.is_plateaued(0.0001));
    }

    #[test]
    fn test_why_analysis() {
        let step = WhyAnalysis::new(1, "Why did compilation fail?", "Type mismatch");
        assert_eq!(step.level, 1);
        assert!(!step.is_root_cause);

        let root = step.clone().mark_root_cause();
        assert!(root.is_root_cause);
    }

    #[test]
    fn test_root_cause_analysis() {
        let mut analysis = RootCauseAnalysis::new();
        assert_eq!(analysis.depth(), 0);
        assert!(!analysis.has_root_cause());

        analysis.add_step(WhyAnalysis::new(1, "Why?", "Because"));
        assert_eq!(analysis.depth(), 1);

        analysis.add_step(WhyAnalysis::new(2, "Why?", "Root cause").mark_root_cause());
        assert!(analysis.has_root_cause());
        assert_eq!(analysis.root_cause, Some("Root cause".to_string()));
    }

    #[test]
    fn test_confidence_level_from_score() {
        assert_eq!(ConfidenceLevel::from_score(0.99), ConfidenceLevel::VeryHigh);
        assert_eq!(ConfidenceLevel::from_score(0.90), ConfidenceLevel::High);
        assert_eq!(ConfidenceLevel::from_score(0.75), ConfidenceLevel::Medium);
        assert_eq!(ConfidenceLevel::from_score(0.55), ConfidenceLevel::Low);
        assert_eq!(ConfidenceLevel::from_score(0.30), ConfidenceLevel::VeryLow);
    }

    #[test]
    fn test_confidence_level_to_score() {
        assert!((ConfidenceLevel::VeryHigh.to_score() - 0.95).abs() < 0.001);
        assert!((ConfidenceLevel::High.to_score() - 0.85).abs() < 0.001);
    }

    #[test]
    fn test_confidence_level_allows_auto_apply() {
        assert!(ConfidenceLevel::VeryHigh.allows_auto_apply(0.85));
        assert!(ConfidenceLevel::High.allows_auto_apply(0.85));
        assert!(!ConfidenceLevel::Medium.allows_auto_apply(0.85));
        assert!(!ConfidenceLevel::Low.allows_auto_apply(0.85));
    }

    #[test]
    fn test_error_cluster_stats() {
        let mut stats = ErrorClusterStats::new(PatternClass::TypeMismatch);
        stats.add_occurrence("E0308", "file1.rs");
        stats.add_occurrence("E0308", "file2.rs");
        stats.add_occurrence("E0308", "file1.rs"); // Duplicate file

        assert_eq!(stats.count, 3);
        assert_eq!(stats.sample_codes.len(), 1);
        assert_eq!(stats.affected_files.len(), 2);
    }

    #[test]
    fn test_error_cluster_stats_priority_score() {
        let mut stats = ErrorClusterStats::new(PatternClass::TypeMismatch);
        stats.count = 10;
        let score = stats.priority_score();
        assert!(score > 0.0);
    }

    #[test]
    fn test_cluster_errors() {
        let errors = vec![
            (
                "E0308".to_string(),
                "mismatch".to_string(),
                "a.rs".to_string(),
            ),
            (
                "E0308".to_string(),
                "mismatch".to_string(),
                "b.rs".to_string(),
            ),
            ("E0277".to_string(), "trait".to_string(), "c.rs".to_string()),
        ];

        let clusters = cluster_errors(&errors);
        assert_eq!(clusters.len(), 2);
        assert_eq!(clusters.get(&PatternClass::TypeMismatch).unwrap().count, 2);
        assert_eq!(clusters.get(&PatternClass::TraitBound).unwrap().count, 1);
    }

    #[test]
    fn test_select_priority_cluster() {
        let errors = vec![
            (
                "E0308".to_string(),
                "mismatch".to_string(),
                "a.rs".to_string(),
            ),
            (
                "E0308".to_string(),
                "mismatch".to_string(),
                "b.rs".to_string(),
            ),
            ("E0277".to_string(), "trait".to_string(), "c.rs".to_string()),
        ];

        let clusters = cluster_errors(&errors);
        let priority = select_priority_cluster(&clusters);
        assert!(priority.is_some());
    }

    #[test]
    fn test_select_priority_cluster_empty() {
        let clusters: HashMap<PatternClass, ErrorClusterStats> = HashMap::new();
        assert!(select_priority_cluster(&clusters).is_none());
    }
}
