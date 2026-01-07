//! Pure quality validation functions - EXTREME TDD
//!
//! DEPYLER-COVERAGE-95: Extracted from lib.rs for testability
//! Contains validation logic for quality gates without side effects.

use serde::{Deserialize, Serialize};

/// Results of compilation quality checks
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompilationResults {
    pub compilation_ok: bool,
    pub clippy_ok: bool,
    pub all_passed: bool,
}

impl CompilationResults {
    /// Create a new CompilationResults
    pub fn new(compilation_ok: bool, clippy_ok: bool) -> Self {
        Self {
            compilation_ok,
            clippy_ok,
            all_passed: compilation_ok && clippy_ok,
        }
    }

    /// Create a passing result
    pub fn passing() -> Self {
        Self::new(true, true)
    }

    /// Create a failing result
    pub fn failing() -> Self {
        Self::new(false, false)
    }
}

/// Quality thresholds for validation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualityThresholds {
    pub min_tdg: f64,
    pub max_tdg: f64,
    pub max_complexity: u32,
    pub min_coverage: f64,
}

impl Default for QualityThresholds {
    fn default() -> Self {
        Self {
            min_tdg: 0.0,
            max_tdg: 2.0,
            max_complexity: 10,
            min_coverage: 80.0,
        }
    }
}

impl QualityThresholds {
    /// Create strict thresholds for high-quality code
    pub fn strict() -> Self {
        Self {
            min_tdg: 0.0,
            max_tdg: 1.5,
            max_complexity: 8,
            min_coverage: 90.0,
        }
    }

    /// Create relaxed thresholds for legacy code
    pub fn relaxed() -> Self {
        Self {
            min_tdg: 0.0,
            max_tdg: 3.0,
            max_complexity: 15,
            min_coverage: 60.0,
        }
    }

    /// Validate TDG score
    pub fn validate_tdg(&self, tdg: f64) -> bool {
        tdg >= self.min_tdg && tdg <= self.max_tdg
    }

    /// Validate complexity
    pub fn validate_complexity(&self, complexity: u32) -> bool {
        complexity <= self.max_complexity
    }

    /// Validate coverage (expects percentage 0-100)
    pub fn validate_coverage(&self, coverage: f64) -> bool {
        coverage >= self.min_coverage
    }
}

/// Quality metrics extracted from a report
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub tdg_score: f64,
    pub cyclomatic_complexity: u32,
    pub cognitive_complexity: u32,
    pub line_coverage: f64,
    pub function_coverage: f64,
}

impl Default for QualityMetrics {
    fn default() -> Self {
        Self {
            tdg_score: 0.0,
            cyclomatic_complexity: 0,
            cognitive_complexity: 0,
            line_coverage: 0.0,
            function_coverage: 0.0,
        }
    }
}

impl QualityMetrics {
    /// Create high-quality metrics
    pub fn excellent() -> Self {
        Self {
            tdg_score: 0.5,
            cyclomatic_complexity: 3,
            cognitive_complexity: 4,
            line_coverage: 95.0,
            function_coverage: 98.0,
        }
    }

    /// Create acceptable metrics
    pub fn acceptable() -> Self {
        Self {
            tdg_score: 1.5,
            cyclomatic_complexity: 8,
            cognitive_complexity: 10,
            line_coverage: 80.0,
            function_coverage: 85.0,
        }
    }

    /// Create poor metrics
    pub fn poor() -> Self {
        Self {
            tdg_score: 3.5,
            cyclomatic_complexity: 20,
            cognitive_complexity: 30,
            line_coverage: 40.0,
            function_coverage: 50.0,
        }
    }

    /// Get overall quality grade (A-F)
    pub fn grade(&self) -> char {
        let score = self.calculate_score();
        match score {
            90..=100 => 'A',
            80..=89 => 'B',
            70..=79 => 'C',
            60..=69 => 'D',
            _ => 'F',
        }
    }

    /// Calculate overall quality score (0-100)
    pub fn calculate_score(&self) -> u32 {
        let tdg_score = (100.0 - (self.tdg_score * 20.0).min(100.0)).max(0.0) as u32;
        let complexity_score = (100 - (self.cyclomatic_complexity * 5).min(100)).max(0);
        let coverage_score = self.line_coverage as u32;

        // Weighted average: coverage 40%, TDG 30%, complexity 30%
        (coverage_score * 40 + tdg_score * 30 + complexity_score * 30) / 100
    }
}

/// Validation result for a single check
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
}

impl ValidationResult {
    /// Create a passing result
    pub fn pass(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            message: format!("{} check passed", name),
            name,
            passed: true,
        }
    }

    /// Create a failing result
    pub fn fail(name: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            passed: false,
            message: reason.into(),
        }
    }
}

/// Validate quality metrics against thresholds
pub fn validate_metrics(
    metrics: &QualityMetrics,
    thresholds: &QualityThresholds,
) -> Vec<ValidationResult> {
    let mut results = Vec::new();

    // TDG validation
    if thresholds.validate_tdg(metrics.tdg_score) {
        results.push(ValidationResult::pass("TDG"));
    } else {
        results.push(ValidationResult::fail(
            "TDG",
            format!(
                "TDG score {:.2} outside range [{:.1}, {:.1}]",
                metrics.tdg_score, thresholds.min_tdg, thresholds.max_tdg
            ),
        ));
    }

    // Complexity validation
    if thresholds.validate_complexity(metrics.cyclomatic_complexity) {
        results.push(ValidationResult::pass("Complexity"));
    } else {
        results.push(ValidationResult::fail(
            "Complexity",
            format!(
                "Cyclomatic complexity {} exceeds max {}",
                metrics.cyclomatic_complexity, thresholds.max_complexity
            ),
        ));
    }

    // Coverage validation
    if thresholds.validate_coverage(metrics.line_coverage) {
        results.push(ValidationResult::pass("Coverage"));
    } else {
        results.push(ValidationResult::fail(
            "Coverage",
            format!(
                "Line coverage {:.1}% below min {:.1}%",
                metrics.line_coverage, thresholds.min_coverage
            ),
        ));
    }

    results
}

/// Check if all validations passed
pub fn all_validations_passed(results: &[ValidationResult]) -> bool {
    results.iter().all(|r| r.passed)
}

/// Count passing validations
pub fn count_passing(results: &[ValidationResult]) -> usize {
    results.iter().filter(|r| r.passed).count()
}

/// Count failing validations
pub fn count_failing(results: &[ValidationResult]) -> usize {
    results.iter().filter(|r| !r.passed).count()
}

/// Get all failure messages
pub fn get_failure_messages(results: &[ValidationResult]) -> Vec<String> {
    results
        .iter()
        .filter(|r| !r.passed)
        .map(|r| r.message.clone())
        .collect()
}

/// Format validation results for display
pub fn format_validation_summary(results: &[ValidationResult]) -> String {
    let total = results.len();
    let passed = count_passing(results);
    let failed = count_failing(results);

    let status = if all_validations_passed(results) {
        "PASSED"
    } else {
        "FAILED"
    };

    format!(
        "Quality Gate {}: {}/{} checks passed ({} failed)",
        status, passed, total, failed
    )
}

/// Calculate coverage improvement needed
pub fn coverage_improvement_needed(current: f64, target: f64) -> f64 {
    if current >= target {
        0.0
    } else {
        target - current
    }
}

/// Estimate lines to cover for target coverage
pub fn lines_to_cover(total_lines: u32, current_coverage: f64, target_coverage: f64) -> u32 {
    let current_covered = (total_lines as f64 * current_coverage / 100.0) as u32;
    let target_covered = (total_lines as f64 * target_coverage / 100.0) as u32;

    if target_covered > current_covered {
        target_covered - current_covered
    } else {
        0
    }
}

/// Calculate TDG improvement needed
pub fn tdg_improvement_needed(current: f64, max_target: f64) -> f64 {
    if current <= max_target {
        0.0
    } else {
        current - max_target
    }
}

/// Rate complexity level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplexityRating {
    Excellent,
    Good,
    Acceptable,
    High,
    VeryHigh,
}

impl ComplexityRating {
    /// Get rating from cyclomatic complexity
    pub fn from_cyclomatic(complexity: u32) -> Self {
        match complexity {
            0..=3 => ComplexityRating::Excellent,
            4..=6 => ComplexityRating::Good,
            7..=10 => ComplexityRating::Acceptable,
            11..=20 => ComplexityRating::High,
            _ => ComplexityRating::VeryHigh,
        }
    }

    /// Get rating from cognitive complexity
    pub fn from_cognitive(complexity: u32) -> Self {
        match complexity {
            0..=5 => ComplexityRating::Excellent,
            6..=10 => ComplexityRating::Good,
            11..=15 => ComplexityRating::Acceptable,
            16..=30 => ComplexityRating::High,
            _ => ComplexityRating::VeryHigh,
        }
    }

    /// Check if acceptable for production
    pub fn is_acceptable(&self) -> bool {
        matches!(
            self,
            ComplexityRating::Excellent | ComplexityRating::Good | ComplexityRating::Acceptable
        )
    }

    /// Get human-readable description
    pub fn as_str(&self) -> &'static str {
        match self {
            ComplexityRating::Excellent => "Excellent",
            ComplexityRating::Good => "Good",
            ComplexityRating::Acceptable => "Acceptable",
            ComplexityRating::High => "High",
            ComplexityRating::VeryHigh => "Very High",
        }
    }
}

/// TDG grade classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TdgGrade {
    APlus,
    A,
    AMinus,
    BPlus,
    B,
    BMinus,
    C,
    D,
    F,
}

impl TdgGrade {
    /// Get grade from TDG score
    pub fn from_score(tdg: f64) -> Self {
        match tdg {
            x if x <= 0.5 => TdgGrade::APlus,
            x if x <= 1.0 => TdgGrade::A,
            x if x <= 1.5 => TdgGrade::AMinus,
            x if x <= 2.0 => TdgGrade::BPlus,
            x if x <= 2.5 => TdgGrade::B,
            x if x <= 3.0 => TdgGrade::BMinus,
            x if x <= 3.5 => TdgGrade::C,
            x if x <= 4.0 => TdgGrade::D,
            _ => TdgGrade::F,
        }
    }

    /// Check if grade is passing (B- or better)
    pub fn is_passing(&self) -> bool {
        *self <= TdgGrade::BMinus
    }

    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            TdgGrade::APlus => "A+",
            TdgGrade::A => "A",
            TdgGrade::AMinus => "A-",
            TdgGrade::BPlus => "B+",
            TdgGrade::B => "B",
            TdgGrade::BMinus => "B-",
            TdgGrade::C => "C",
            TdgGrade::D => "D",
            TdgGrade::F => "F",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== CompilationResults tests ====================

    #[test]
    fn test_compilation_results_new_passing() {
        let results = CompilationResults::new(true, true);
        assert!(results.compilation_ok);
        assert!(results.clippy_ok);
        assert!(results.all_passed);
    }

    #[test]
    fn test_compilation_results_new_partial() {
        let results = CompilationResults::new(true, false);
        assert!(results.compilation_ok);
        assert!(!results.clippy_ok);
        assert!(!results.all_passed);
    }

    #[test]
    fn test_compilation_results_new_failing() {
        let results = CompilationResults::new(false, false);
        assert!(!results.compilation_ok);
        assert!(!results.clippy_ok);
        assert!(!results.all_passed);
    }

    #[test]
    fn test_compilation_results_passing() {
        let results = CompilationResults::passing();
        assert!(results.all_passed);
    }

    #[test]
    fn test_compilation_results_failing() {
        let results = CompilationResults::failing();
        assert!(!results.all_passed);
    }

    // ==================== QualityThresholds tests ====================

    #[test]
    fn test_thresholds_default() {
        let t = QualityThresholds::default();
        assert_eq!(t.min_tdg, 0.0);
        assert_eq!(t.max_tdg, 2.0);
        assert_eq!(t.max_complexity, 10);
        assert_eq!(t.min_coverage, 80.0);
    }

    #[test]
    fn test_thresholds_strict() {
        let t = QualityThresholds::strict();
        assert!(t.max_tdg < QualityThresholds::default().max_tdg);
        assert!(t.max_complexity < QualityThresholds::default().max_complexity);
        assert!(t.min_coverage > QualityThresholds::default().min_coverage);
    }

    #[test]
    fn test_thresholds_relaxed() {
        let t = QualityThresholds::relaxed();
        assert!(t.max_tdg > QualityThresholds::default().max_tdg);
        assert!(t.max_complexity > QualityThresholds::default().max_complexity);
        assert!(t.min_coverage < QualityThresholds::default().min_coverage);
    }

    #[test]
    fn test_validate_tdg_in_range() {
        let t = QualityThresholds::default();
        assert!(t.validate_tdg(0.0));
        assert!(t.validate_tdg(1.0));
        assert!(t.validate_tdg(2.0));
    }

    #[test]
    fn test_validate_tdg_out_of_range() {
        let t = QualityThresholds::default();
        assert!(!t.validate_tdg(-0.1));
        assert!(!t.validate_tdg(2.1));
    }

    #[test]
    fn test_validate_complexity_ok() {
        let t = QualityThresholds::default();
        assert!(t.validate_complexity(5));
        assert!(t.validate_complexity(10));
    }

    #[test]
    fn test_validate_complexity_too_high() {
        let t = QualityThresholds::default();
        assert!(!t.validate_complexity(11));
        assert!(!t.validate_complexity(100));
    }

    #[test]
    fn test_validate_coverage_ok() {
        let t = QualityThresholds::default();
        assert!(t.validate_coverage(80.0));
        assert!(t.validate_coverage(100.0));
    }

    #[test]
    fn test_validate_coverage_too_low() {
        let t = QualityThresholds::default();
        assert!(!t.validate_coverage(79.9));
        assert!(!t.validate_coverage(0.0));
    }

    // ==================== QualityMetrics tests ====================

    #[test]
    fn test_metrics_default() {
        let m = QualityMetrics::default();
        assert_eq!(m.tdg_score, 0.0);
        assert_eq!(m.cyclomatic_complexity, 0);
    }

    #[test]
    fn test_metrics_excellent() {
        let m = QualityMetrics::excellent();
        assert!(m.tdg_score < 1.0);
        assert!(m.line_coverage > 90.0);
    }

    #[test]
    fn test_metrics_poor() {
        let m = QualityMetrics::poor();
        assert!(m.tdg_score > 3.0);
        assert!(m.line_coverage < 50.0);
    }

    #[test]
    fn test_metrics_grade_a() {
        let m = QualityMetrics::excellent();
        assert_eq!(m.grade(), 'A');
    }

    #[test]
    fn test_metrics_grade_f() {
        let m = QualityMetrics::poor();
        assert!(m.grade() == 'D' || m.grade() == 'F');
    }

    #[test]
    fn test_metrics_score_range() {
        let m = QualityMetrics::default();
        let score = m.calculate_score();
        assert!(score <= 100);
    }

    // ==================== ValidationResult tests ====================

    #[test]
    fn test_validation_result_pass() {
        let r = ValidationResult::pass("Test");
        assert!(r.passed);
        assert_eq!(r.name, "Test");
    }

    #[test]
    fn test_validation_result_fail() {
        let r = ValidationResult::fail("Test", "Failed because X");
        assert!(!r.passed);
        assert!(r.message.contains("Failed"));
    }

    // ==================== validate_metrics tests ====================

    #[test]
    fn test_validate_metrics_all_pass() {
        let metrics = QualityMetrics::excellent();
        let thresholds = QualityThresholds::default();
        let results = validate_metrics(&metrics, &thresholds);
        assert!(all_validations_passed(&results));
    }

    #[test]
    fn test_validate_metrics_tdg_fail() {
        let mut metrics = QualityMetrics::excellent();
        metrics.tdg_score = 5.0;
        let thresholds = QualityThresholds::default();
        let results = validate_metrics(&metrics, &thresholds);
        assert!(!all_validations_passed(&results));
        assert!(results.iter().any(|r| r.name == "TDG" && !r.passed));
    }

    #[test]
    fn test_validate_metrics_complexity_fail() {
        let mut metrics = QualityMetrics::excellent();
        metrics.cyclomatic_complexity = 50;
        let thresholds = QualityThresholds::default();
        let results = validate_metrics(&metrics, &thresholds);
        assert!(!all_validations_passed(&results));
        assert!(results.iter().any(|r| r.name == "Complexity" && !r.passed));
    }

    #[test]
    fn test_validate_metrics_coverage_fail() {
        let mut metrics = QualityMetrics::excellent();
        metrics.line_coverage = 50.0;
        let thresholds = QualityThresholds::default();
        let results = validate_metrics(&metrics, &thresholds);
        assert!(!all_validations_passed(&results));
        assert!(results.iter().any(|r| r.name == "Coverage" && !r.passed));
    }

    // ==================== Helper function tests ====================

    #[test]
    fn test_count_passing() {
        let results = vec![
            ValidationResult::pass("A"),
            ValidationResult::pass("B"),
            ValidationResult::fail("C", "fail"),
        ];
        assert_eq!(count_passing(&results), 2);
    }

    #[test]
    fn test_count_failing() {
        let results = vec![
            ValidationResult::pass("A"),
            ValidationResult::fail("B", "fail"),
            ValidationResult::fail("C", "fail"),
        ];
        assert_eq!(count_failing(&results), 2);
    }

    #[test]
    fn test_get_failure_messages() {
        let results = vec![
            ValidationResult::pass("A"),
            ValidationResult::fail("B", "Error B"),
            ValidationResult::fail("C", "Error C"),
        ];
        let messages = get_failure_messages(&results);
        assert_eq!(messages.len(), 2);
        assert!(messages.contains(&"Error B".to_string()));
    }

    #[test]
    fn test_format_validation_summary_passed() {
        let results = vec![
            ValidationResult::pass("A"),
            ValidationResult::pass("B"),
        ];
        let summary = format_validation_summary(&results);
        assert!(summary.contains("PASSED"));
        assert!(summary.contains("2/2"));
    }

    #[test]
    fn test_format_validation_summary_failed() {
        let results = vec![
            ValidationResult::pass("A"),
            ValidationResult::fail("B", "fail"),
        ];
        let summary = format_validation_summary(&results);
        assert!(summary.contains("FAILED"));
    }

    // ==================== Coverage calculation tests ====================

    #[test]
    fn test_coverage_improvement_needed_below_target() {
        assert_eq!(coverage_improvement_needed(80.0, 95.0), 15.0);
    }

    #[test]
    fn test_coverage_improvement_needed_at_target() {
        assert_eq!(coverage_improvement_needed(95.0, 95.0), 0.0);
    }

    #[test]
    fn test_coverage_improvement_needed_above_target() {
        assert_eq!(coverage_improvement_needed(98.0, 95.0), 0.0);
    }

    #[test]
    fn test_lines_to_cover() {
        // 1000 lines, 80% covered, want 90%
        // Currently 800 covered, need 900, so 100 more lines
        assert_eq!(lines_to_cover(1000, 80.0, 90.0), 100);
    }

    #[test]
    fn test_lines_to_cover_already_met() {
        assert_eq!(lines_to_cover(1000, 95.0, 90.0), 0);
    }

    #[test]
    fn test_tdg_improvement_needed() {
        assert_eq!(tdg_improvement_needed(3.0, 2.0), 1.0);
        assert_eq!(tdg_improvement_needed(1.5, 2.0), 0.0);
    }

    // ==================== ComplexityRating tests ====================

    #[test]
    fn test_complexity_rating_excellent() {
        assert_eq!(ComplexityRating::from_cyclomatic(1), ComplexityRating::Excellent);
        assert_eq!(ComplexityRating::from_cyclomatic(3), ComplexityRating::Excellent);
    }

    #[test]
    fn test_complexity_rating_good() {
        assert_eq!(ComplexityRating::from_cyclomatic(5), ComplexityRating::Good);
    }

    #[test]
    fn test_complexity_rating_acceptable() {
        assert_eq!(ComplexityRating::from_cyclomatic(10), ComplexityRating::Acceptable);
    }

    #[test]
    fn test_complexity_rating_high() {
        assert_eq!(ComplexityRating::from_cyclomatic(15), ComplexityRating::High);
    }

    #[test]
    fn test_complexity_rating_very_high() {
        assert_eq!(ComplexityRating::from_cyclomatic(50), ComplexityRating::VeryHigh);
    }

    #[test]
    fn test_complexity_rating_cognitive() {
        assert_eq!(ComplexityRating::from_cognitive(3), ComplexityRating::Excellent);
        assert_eq!(ComplexityRating::from_cognitive(8), ComplexityRating::Good);
        assert_eq!(ComplexityRating::from_cognitive(12), ComplexityRating::Acceptable);
    }

    #[test]
    fn test_complexity_rating_is_acceptable() {
        assert!(ComplexityRating::Excellent.is_acceptable());
        assert!(ComplexityRating::Good.is_acceptable());
        assert!(ComplexityRating::Acceptable.is_acceptable());
        assert!(!ComplexityRating::High.is_acceptable());
        assert!(!ComplexityRating::VeryHigh.is_acceptable());
    }

    #[test]
    fn test_complexity_rating_as_str() {
        assert_eq!(ComplexityRating::Excellent.as_str(), "Excellent");
        assert_eq!(ComplexityRating::VeryHigh.as_str(), "Very High");
    }

    // ==================== TdgGrade tests ====================

    #[test]
    fn test_tdg_grade_a_plus() {
        assert_eq!(TdgGrade::from_score(0.0), TdgGrade::APlus);
        assert_eq!(TdgGrade::from_score(0.5), TdgGrade::APlus);
    }

    #[test]
    fn test_tdg_grade_a() {
        assert_eq!(TdgGrade::from_score(0.6), TdgGrade::A);
        assert_eq!(TdgGrade::from_score(1.0), TdgGrade::A);
    }

    #[test]
    fn test_tdg_grade_a_minus() {
        assert_eq!(TdgGrade::from_score(1.1), TdgGrade::AMinus);
        assert_eq!(TdgGrade::from_score(1.5), TdgGrade::AMinus);
    }

    #[test]
    fn test_tdg_grade_b_range() {
        assert_eq!(TdgGrade::from_score(1.6), TdgGrade::BPlus);
        assert_eq!(TdgGrade::from_score(2.2), TdgGrade::B);
        assert_eq!(TdgGrade::from_score(2.7), TdgGrade::BMinus);
    }

    #[test]
    fn test_tdg_grade_c_d_f() {
        assert_eq!(TdgGrade::from_score(3.2), TdgGrade::C);
        assert_eq!(TdgGrade::from_score(3.8), TdgGrade::D);
        assert_eq!(TdgGrade::from_score(5.0), TdgGrade::F);
    }

    #[test]
    fn test_tdg_grade_is_passing() {
        assert!(TdgGrade::APlus.is_passing());
        assert!(TdgGrade::A.is_passing());
        assert!(TdgGrade::BMinus.is_passing());
        assert!(!TdgGrade::C.is_passing());
        assert!(!TdgGrade::F.is_passing());
    }

    #[test]
    fn test_tdg_grade_as_str() {
        assert_eq!(TdgGrade::APlus.as_str(), "A+");
        assert_eq!(TdgGrade::AMinus.as_str(), "A-");
        assert_eq!(TdgGrade::BPlus.as_str(), "B+");
    }

    #[test]
    fn test_tdg_grade_ordering() {
        assert!(TdgGrade::APlus < TdgGrade::A);
        assert!(TdgGrade::A < TdgGrade::BMinus);
        assert!(TdgGrade::BMinus < TdgGrade::F);
    }

    // ==================== Edge cases ====================

    #[test]
    fn test_empty_validation_results() {
        let results: Vec<ValidationResult> = vec![];
        assert!(all_validations_passed(&results));
        assert_eq!(count_passing(&results), 0);
        assert_eq!(count_failing(&results), 0);
    }

    #[test]
    fn test_metrics_with_zero_values() {
        let m = QualityMetrics::default();
        let score = m.calculate_score();
        // Should handle zero values gracefully
        assert!(score <= 100);
    }

    #[test]
    fn test_thresholds_edge_values() {
        let t = QualityThresholds {
            min_tdg: 0.0,
            max_tdg: 0.0,
            max_complexity: 0,
            min_coverage: 100.0,
        };
        assert!(t.validate_tdg(0.0));
        assert!(!t.validate_tdg(0.1));
        assert!(t.validate_complexity(0));
        assert!(!t.validate_complexity(1));
        assert!(t.validate_coverage(100.0));
        assert!(!t.validate_coverage(99.9));
    }
}
