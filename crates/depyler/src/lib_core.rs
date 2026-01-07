//! Pure core functions extracted from lib.rs for EXTREME TDD
//!
//! This module contains pure, side-effect-free functions that can be
//! thoroughly tested with unit tests. The main lib.rs becomes a thin
//! shim that calls these functions.
//!
//! DEPYLER-COVERAGE-95: Extracted for testability

use colored::{ColoredString, Colorize};

/// Rate code complexity and return colored rating string
///
/// # Thresholds
/// - Good: complexity <= 5
/// - Acceptable: complexity <= 10
/// - High: complexity > 10
pub fn complexity_rating(complexity: f64) -> ColoredString {
    if complexity <= 5.0 {
        "Good".green()
    } else if complexity <= 10.0 {
        "Acceptable".yellow()
    } else {
        "High".red()
    }
}

/// Complexity level enum for programmatic use
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplexityLevel {
    Good,
    Acceptable,
    High,
}

impl ComplexityLevel {
    /// Get complexity level from numeric value
    pub fn from_value(complexity: f64) -> Self {
        if complexity <= 5.0 {
            ComplexityLevel::Good
        } else if complexity <= 10.0 {
            ComplexityLevel::Acceptable
        } else {
            ComplexityLevel::High
        }
    }

    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            ComplexityLevel::Good => "Good",
            ComplexityLevel::Acceptable => "Acceptable",
            ComplexityLevel::High => "High",
        }
    }

    /// Check if complexity is acceptable (Good or Acceptable)
    pub fn is_acceptable(&self) -> bool {
        matches!(self, ComplexityLevel::Good | ComplexityLevel::Acceptable)
    }
}

/// Extract error code from rustc/cargo stderr output
///
/// Returns Some(code) if an error code like E0308 is found, None otherwise
pub fn extract_error_code(error: &str) -> Option<String> {
    // Pattern: error[EXXXX]:
    if let Some(start) = error.find("error[E") {
        let rest = &error[start + 6..]; // After "error["
        if let Some(end) = rest.find(']') {
            return Some(rest[..end].to_string());
        }
    }
    None
}

/// Parse line and column from rustc error output
///
/// Returns Some((line, column)) if found, None otherwise
pub fn parse_error_location(error: &str) -> Option<(usize, usize)> {
    // Pattern: --> path:line:column
    if let Some(arrow_pos) = error.find("-->") {
        let rest = &error[arrow_pos + 3..];
        // Find first colon (after path)
        if let Some(first_colon) = rest.find(':') {
            let after_path = &rest[first_colon + 1..];
            // Find line:column
            if let Some(second_colon) = after_path.find(':') {
                let line_str = &after_path[..second_colon];
                let col_rest = &after_path[second_colon + 1..];
                // Column ends at whitespace or newline
                let col_str: String = col_rest.chars().take_while(|c| c.is_ascii_digit()).collect();

                if let (Ok(line), Ok(col)) = (line_str.trim().parse(), col_str.parse()) {
                    return Some((line, col));
                }
            }
        }
    }
    None
}

/// Error category for classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    TypeMismatch,
    BorrowChecker,
    MissingImport,
    UndefinedVariable,
    MethodNotFound,
    TraitBound,
    Syntax,
    Transpiler,
    Unknown,
}

impl ErrorCategory {
    /// Classify error code into category
    pub fn from_code(code: &str) -> Self {
        match code {
            "E0308" => ErrorCategory::TypeMismatch,
            "E0382" | "E0499" | "E0502" | "E0505" | "E0507" | "E0515" => ErrorCategory::BorrowChecker,
            "E0432" | "E0433" => ErrorCategory::MissingImport,
            "E0425" => ErrorCategory::UndefinedVariable,
            "E0599" | "E0609" => ErrorCategory::MethodNotFound,
            "E0277" => ErrorCategory::TraitBound,
            "TRANSPILE" => ErrorCategory::Transpiler,
            _ => ErrorCategory::Unknown,
        }
    }

    /// Get human-readable name
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorCategory::TypeMismatch => "Type Mismatch",
            ErrorCategory::BorrowChecker => "Borrow Checker",
            ErrorCategory::MissingImport => "Missing Import",
            ErrorCategory::UndefinedVariable => "Undefined Variable",
            ErrorCategory::MethodNotFound => "Method Not Found",
            ErrorCategory::TraitBound => "Trait Bound",
            ErrorCategory::Syntax => "Syntax Error",
            ErrorCategory::Transpiler => "Transpiler Error",
            ErrorCategory::Unknown => "Unknown",
        }
    }
}

/// Quality validation result
#[derive(Debug, Clone)]
pub struct QualityCheck {
    pub tdg_ok: bool,
    pub complexity_ok: bool,
    pub coverage_ok: bool,
}

impl QualityCheck {
    /// Check if all quality gates pass
    pub fn all_passed(&self) -> bool {
        self.tdg_ok && self.complexity_ok && self.coverage_ok
    }

    /// Count passing checks
    pub fn passing_count(&self) -> usize {
        [self.tdg_ok, self.complexity_ok, self.coverage_ok]
            .iter()
            .filter(|&&x| x)
            .count()
    }

    /// Count failing checks
    pub fn failing_count(&self) -> usize {
        3 - self.passing_count()
    }
}

/// Validate TDG (Technical Debt Grade) score
pub fn validate_tdg(tdg: f64, min: f64, max: f64) -> bool {
    tdg >= min && tdg <= max
}

/// Validate complexity score
pub fn validate_complexity(complexity: f64, max: u32) -> bool {
    complexity <= max as f64
}

/// Validate coverage percentage
pub fn validate_coverage(coverage: f64, min: u32) -> bool {
    coverage >= min as f64
}

/// Format file size in human-readable format
pub fn format_file_size(bytes: usize) -> String {
    const KB: usize = 1024;
    const MB: usize = KB * 1024;
    const GB: usize = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

/// Format duration in human-readable format
pub fn format_duration_secs(secs: f64) -> String {
    if secs < 0.001 {
        format!("{:.0} μs", secs * 1_000_000.0)
    } else if secs < 1.0 {
        format!("{:.0} ms", secs * 1000.0)
    } else if secs < 60.0 {
        format!("{:.2} s", secs)
    } else if secs < 3600.0 {
        let mins = (secs / 60.0).floor();
        let remaining = secs - (mins * 60.0);
        format!("{}m {:.0}s", mins as u32, remaining)
    } else {
        let hours = (secs / 3600.0).floor();
        let remaining_mins = ((secs - (hours * 3600.0)) / 60.0).floor();
        format!("{}h {}m", hours as u32, remaining_mins as u32)
    }
}

/// Calculate percentage safely
pub fn calculate_percentage(part: usize, total: usize) -> f64 {
    if total == 0 {
        0.0
    } else {
        (part as f64 / total as f64) * 100.0
    }
}

/// Truncate string to max length with ellipsis
pub fn truncate_with_ellipsis(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else if max_len <= 3 {
        s.chars().take(max_len).collect()
    } else {
        let truncated: String = s.chars().take(max_len - 3).collect();
        format!("{}...", truncated)
    }
}

// ============================================================================
// EXTREME TDD: Comprehensive Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========== complexity_rating tests ==========

    #[test]
    fn test_complexity_rating_good_zero() {
        let rating = complexity_rating(0.0);
        assert!(rating.to_string().contains("Good"));
    }

    #[test]
    fn test_complexity_rating_good_mid() {
        let rating = complexity_rating(3.0);
        assert!(rating.to_string().contains("Good"));
    }

    #[test]
    fn test_complexity_rating_good_boundary() {
        let rating = complexity_rating(5.0);
        assert!(rating.to_string().contains("Good"));
    }

    #[test]
    fn test_complexity_rating_acceptable_low() {
        let rating = complexity_rating(5.1);
        assert!(rating.to_string().contains("Acceptable"));
    }

    #[test]
    fn test_complexity_rating_acceptable_mid() {
        let rating = complexity_rating(7.5);
        assert!(rating.to_string().contains("Acceptable"));
    }

    #[test]
    fn test_complexity_rating_acceptable_boundary() {
        let rating = complexity_rating(10.0);
        assert!(rating.to_string().contains("Acceptable"));
    }

    #[test]
    fn test_complexity_rating_high_low() {
        let rating = complexity_rating(10.1);
        assert!(rating.to_string().contains("High"));
    }

    #[test]
    fn test_complexity_rating_high_mid() {
        let rating = complexity_rating(15.0);
        assert!(rating.to_string().contains("High"));
    }

    #[test]
    fn test_complexity_rating_high_extreme() {
        let rating = complexity_rating(100.0);
        assert!(rating.to_string().contains("High"));
    }

    // ========== ComplexityLevel tests ==========

    #[test]
    fn test_complexity_level_good() {
        assert_eq!(ComplexityLevel::from_value(0.0), ComplexityLevel::Good);
        assert_eq!(ComplexityLevel::from_value(3.0), ComplexityLevel::Good);
        assert_eq!(ComplexityLevel::from_value(5.0), ComplexityLevel::Good);
    }

    #[test]
    fn test_complexity_level_acceptable() {
        assert_eq!(ComplexityLevel::from_value(5.1), ComplexityLevel::Acceptable);
        assert_eq!(ComplexityLevel::from_value(7.5), ComplexityLevel::Acceptable);
        assert_eq!(ComplexityLevel::from_value(10.0), ComplexityLevel::Acceptable);
    }

    #[test]
    fn test_complexity_level_high() {
        assert_eq!(ComplexityLevel::from_value(10.1), ComplexityLevel::High);
        assert_eq!(ComplexityLevel::from_value(15.0), ComplexityLevel::High);
        assert_eq!(ComplexityLevel::from_value(100.0), ComplexityLevel::High);
    }

    #[test]
    fn test_complexity_level_as_str() {
        assert_eq!(ComplexityLevel::Good.as_str(), "Good");
        assert_eq!(ComplexityLevel::Acceptable.as_str(), "Acceptable");
        assert_eq!(ComplexityLevel::High.as_str(), "High");
    }

    #[test]
    fn test_complexity_level_is_acceptable() {
        assert!(ComplexityLevel::Good.is_acceptable());
        assert!(ComplexityLevel::Acceptable.is_acceptable());
        assert!(!ComplexityLevel::High.is_acceptable());
    }

    // ========== extract_error_code tests ==========

    #[test]
    fn test_extract_error_code_e0308() {
        assert_eq!(
            extract_error_code("error[E0308]: mismatched types"),
            Some("E0308".to_string())
        );
    }

    #[test]
    fn test_extract_error_code_e0425() {
        assert_eq!(
            extract_error_code("error[E0425]: cannot find value"),
            Some("E0425".to_string())
        );
    }

    #[test]
    fn test_extract_error_code_e0277() {
        assert_eq!(
            extract_error_code("error[E0277]: trait bound not satisfied"),
            Some("E0277".to_string())
        );
    }

    #[test]
    fn test_extract_error_code_not_found() {
        assert_eq!(extract_error_code("warning: unused variable"), None);
    }

    #[test]
    fn test_extract_error_code_malformed() {
        assert_eq!(extract_error_code("error[E0308 no bracket"), None);
    }

    #[test]
    fn test_extract_error_code_empty() {
        assert_eq!(extract_error_code(""), None);
    }

    #[test]
    fn test_extract_error_code_multiline() {
        let error = "compiling...\nerror[E0599]: no method found\n   --> src/main.rs";
        assert_eq!(extract_error_code(error), Some("E0599".to_string()));
    }

    // ========== parse_error_location tests ==========

    #[test]
    fn test_parse_error_location_found() {
        let error = "  --> src/main.rs:42:15";
        assert_eq!(parse_error_location(error), Some((42, 15)));
    }

    #[test]
    fn test_parse_error_location_line_only() {
        let error = "  --> src/main.rs:100:1";
        assert_eq!(parse_error_location(error), Some((100, 1)));
    }

    #[test]
    fn test_parse_error_location_not_found() {
        let error = "error: something went wrong";
        assert_eq!(parse_error_location(error), None);
    }

    #[test]
    fn test_parse_error_location_empty() {
        assert_eq!(parse_error_location(""), None);
    }

    // ========== ErrorCategory tests ==========

    #[test]
    fn test_error_category_type_mismatch() {
        assert_eq!(ErrorCategory::from_code("E0308"), ErrorCategory::TypeMismatch);
    }

    #[test]
    fn test_error_category_borrow_checker() {
        assert_eq!(ErrorCategory::from_code("E0382"), ErrorCategory::BorrowChecker);
        assert_eq!(ErrorCategory::from_code("E0499"), ErrorCategory::BorrowChecker);
        assert_eq!(ErrorCategory::from_code("E0502"), ErrorCategory::BorrowChecker);
        assert_eq!(ErrorCategory::from_code("E0505"), ErrorCategory::BorrowChecker);
        assert_eq!(ErrorCategory::from_code("E0507"), ErrorCategory::BorrowChecker);
    }

    #[test]
    fn test_error_category_missing_import() {
        assert_eq!(ErrorCategory::from_code("E0432"), ErrorCategory::MissingImport);
        assert_eq!(ErrorCategory::from_code("E0433"), ErrorCategory::MissingImport);
    }

    #[test]
    fn test_error_category_undefined_variable() {
        assert_eq!(ErrorCategory::from_code("E0425"), ErrorCategory::UndefinedVariable);
    }

    #[test]
    fn test_error_category_method_not_found() {
        assert_eq!(ErrorCategory::from_code("E0599"), ErrorCategory::MethodNotFound);
        assert_eq!(ErrorCategory::from_code("E0609"), ErrorCategory::MethodNotFound);
    }

    #[test]
    fn test_error_category_trait_bound() {
        assert_eq!(ErrorCategory::from_code("E0277"), ErrorCategory::TraitBound);
    }

    #[test]
    fn test_error_category_transpiler() {
        assert_eq!(ErrorCategory::from_code("TRANSPILE"), ErrorCategory::Transpiler);
    }

    #[test]
    fn test_error_category_unknown() {
        assert_eq!(ErrorCategory::from_code("E9999"), ErrorCategory::Unknown);
        assert_eq!(ErrorCategory::from_code("RANDOM"), ErrorCategory::Unknown);
    }

    #[test]
    fn test_error_category_as_str() {
        assert_eq!(ErrorCategory::TypeMismatch.as_str(), "Type Mismatch");
        assert_eq!(ErrorCategory::BorrowChecker.as_str(), "Borrow Checker");
        assert_eq!(ErrorCategory::Unknown.as_str(), "Unknown");
    }

    // ========== QualityCheck tests ==========

    #[test]
    fn test_quality_check_all_pass() {
        let check = QualityCheck {
            tdg_ok: true,
            complexity_ok: true,
            coverage_ok: true,
        };
        assert!(check.all_passed());
        assert_eq!(check.passing_count(), 3);
        assert_eq!(check.failing_count(), 0);
    }

    #[test]
    fn test_quality_check_none_pass() {
        let check = QualityCheck {
            tdg_ok: false,
            complexity_ok: false,
            coverage_ok: false,
        };
        assert!(!check.all_passed());
        assert_eq!(check.passing_count(), 0);
        assert_eq!(check.failing_count(), 3);
    }

    #[test]
    fn test_quality_check_some_pass() {
        let check = QualityCheck {
            tdg_ok: true,
            complexity_ok: false,
            coverage_ok: true,
        };
        assert!(!check.all_passed());
        assert_eq!(check.passing_count(), 2);
        assert_eq!(check.failing_count(), 1);
    }

    // ========== validate_tdg tests ==========

    #[test]
    fn test_validate_tdg_in_range() {
        assert!(validate_tdg(1.5, 0.0, 2.0));
        assert!(validate_tdg(0.0, 0.0, 2.0));
        assert!(validate_tdg(2.0, 0.0, 2.0));
    }

    #[test]
    fn test_validate_tdg_out_of_range() {
        assert!(!validate_tdg(-0.1, 0.0, 2.0));
        assert!(!validate_tdg(2.1, 0.0, 2.0));
    }

    // ========== validate_complexity tests ==========

    #[test]
    fn test_validate_complexity_pass() {
        assert!(validate_complexity(5.0, 10));
        assert!(validate_complexity(10.0, 10));
    }

    #[test]
    fn test_validate_complexity_fail() {
        assert!(!validate_complexity(10.1, 10));
        assert!(!validate_complexity(15.0, 10));
    }

    // ========== validate_coverage tests ==========

    #[test]
    fn test_validate_coverage_pass() {
        assert!(validate_coverage(80.0, 80));
        assert!(validate_coverage(95.0, 80));
    }

    #[test]
    fn test_validate_coverage_fail() {
        assert!(!validate_coverage(79.9, 80));
        assert!(!validate_coverage(50.0, 80));
    }

    // ========== format_file_size tests ==========

    #[test]
    fn test_format_file_size_bytes() {
        assert_eq!(format_file_size(0), "0 bytes");
        assert_eq!(format_file_size(512), "512 bytes");
        assert_eq!(format_file_size(1023), "1023 bytes");
    }

    #[test]
    fn test_format_file_size_kb() {
        assert_eq!(format_file_size(1024), "1.00 KB");
        assert_eq!(format_file_size(2048), "2.00 KB");
        assert!(format_file_size(1536).contains("KB"));
    }

    #[test]
    fn test_format_file_size_mb() {
        assert_eq!(format_file_size(1024 * 1024), "1.00 MB");
        assert!(format_file_size(5 * 1024 * 1024).contains("MB"));
    }

    #[test]
    fn test_format_file_size_gb() {
        assert_eq!(format_file_size(1024 * 1024 * 1024), "1.00 GB");
    }

    // ========== format_duration_secs tests ==========

    #[test]
    fn test_format_duration_microseconds() {
        assert!(format_duration_secs(0.0001).contains("μs"));
        assert!(format_duration_secs(0.0005).contains("μs"));
    }

    #[test]
    fn test_format_duration_milliseconds() {
        assert!(format_duration_secs(0.001).contains("ms"));
        assert!(format_duration_secs(0.5).contains("ms"));
    }

    #[test]
    fn test_format_duration_seconds() {
        assert!(format_duration_secs(1.0).contains(" s"));
        assert!(format_duration_secs(30.5).contains(" s"));
    }

    #[test]
    fn test_format_duration_minutes() {
        let formatted = format_duration_secs(90.0);
        assert!(formatted.contains("m"));
    }

    #[test]
    fn test_format_duration_hours() {
        let formatted = format_duration_secs(3700.0);
        assert!(formatted.contains("h"));
    }

    // ========== calculate_percentage tests ==========

    #[test]
    fn test_calculate_percentage_zero() {
        assert_eq!(calculate_percentage(0, 0), 0.0);
        assert_eq!(calculate_percentage(0, 100), 0.0);
    }

    #[test]
    fn test_calculate_percentage_full() {
        assert_eq!(calculate_percentage(100, 100), 100.0);
    }

    #[test]
    fn test_calculate_percentage_half() {
        assert_eq!(calculate_percentage(50, 100), 50.0);
    }

    #[test]
    fn test_calculate_percentage_various() {
        assert!((calculate_percentage(1, 4) - 25.0).abs() < 0.01);
        assert!((calculate_percentage(3, 4) - 75.0).abs() < 0.01);
    }

    // ========== truncate_with_ellipsis tests ==========

    #[test]
    fn test_truncate_short_string() {
        assert_eq!(truncate_with_ellipsis("hello", 10), "hello");
    }

    #[test]
    fn test_truncate_exact_length() {
        assert_eq!(truncate_with_ellipsis("hello", 5), "hello");
    }

    #[test]
    fn test_truncate_long_string() {
        assert_eq!(truncate_with_ellipsis("hello world", 8), "hello...");
    }

    #[test]
    fn test_truncate_very_short_max() {
        assert_eq!(truncate_with_ellipsis("hello", 2), "he");
        assert_eq!(truncate_with_ellipsis("hello", 3), "hel");
    }

    #[test]
    fn test_truncate_empty() {
        assert_eq!(truncate_with_ellipsis("", 5), "");
    }

    #[test]
    fn test_truncate_zero_max() {
        assert_eq!(truncate_with_ellipsis("hello", 0), "");
    }
}
