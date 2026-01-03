//! Analysis shim - pure logic separated from I/O
//!
//! This module contains the testable core logic of code analysis,
//! extracted from the CLI command handlers.

use anyhow::Result;

/// Analysis configuration
#[derive(Debug, Clone, Default)]
pub struct AnalyzeConfig {
    pub format: String,
    pub include_complexity: bool,
    pub include_quality: bool,
    pub include_migration: bool,
}

/// Analysis result (pure data)
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub complexity_score: f64,
    pub quality_score: f64,
    pub tdg_score: Option<f64>,
    pub line_count: usize,
    pub function_count: usize,
    pub class_count: usize,
    pub issues: Vec<AnalysisIssue>,
}

/// An issue found during analysis
#[derive(Debug, Clone)]
pub struct AnalysisIssue {
    pub severity: IssueSeverity,
    pub message: String,
    pub line: Option<usize>,
}

/// Issue severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueSeverity {
    Info,
    Warning,
    Error,
}

/// Analyze Python source code (pure function)
pub fn analyze_source(source: &str, _config: &AnalyzeConfig) -> Result<AnalysisResult> {
    // Count basic metrics without external deps
    let line_count = source.lines().count();
    let function_count = source.matches("def ").count();
    let class_count = source.matches("class ").count();

    // Estimate complexity based on source structure
    let complexity_score = estimate_complexity(source);

    Ok(AnalysisResult {
        complexity_score,
        quality_score: 80.0, // Default quality score
        tdg_score: Some(75.0),
        line_count,
        function_count,
        class_count,
        issues: vec![],
    })
}

/// Estimate code complexity from source
fn estimate_complexity(source: &str) -> f64 {
    let mut score = 1.0;

    // Add complexity for control flow
    score += source.matches("if ").count() as f64 * 0.5;
    score += source.matches("for ").count() as f64 * 0.5;
    score += source.matches("while ").count() as f64 * 0.5;
    score += source.matches("try:").count() as f64 * 0.3;
    score += source.matches("except").count() as f64 * 0.3;

    // Add complexity for nesting (rough estimate)
    let max_indent = source.lines()
        .map(|l| l.len() - l.trim_start().len())
        .max()
        .unwrap_or(0);
    score += (max_indent / 4) as f64 * 0.2;

    score
}

/// Calculate migration complexity from analysis
pub fn calculate_migration_complexity(result: &AnalysisResult) -> MigrationComplexity {
    let score = result.complexity_score;
    let level = if score < 2.0 {
        ComplexityLevel::Low
    } else if score < 5.0 {
        ComplexityLevel::Medium
    } else if score < 10.0 {
        ComplexityLevel::High
    } else {
        ComplexityLevel::VeryHigh
    };

    let estimated_effort = match level {
        ComplexityLevel::Low => "< 1 hour",
        ComplexityLevel::Medium => "1-4 hours",
        ComplexityLevel::High => "1-2 days",
        ComplexityLevel::VeryHigh => "> 2 days",
    };

    MigrationComplexity {
        level,
        score,
        estimated_effort: estimated_effort.to_string(),
    }
}

/// Migration complexity assessment
#[derive(Debug, Clone)]
pub struct MigrationComplexity {
    pub level: ComplexityLevel,
    pub score: f64,
    pub estimated_effort: String,
}

/// Complexity level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplexityLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

impl std::fmt::Display for ComplexityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComplexityLevel::Low => write!(f, "Low"),
            ComplexityLevel::Medium => write!(f, "Medium"),
            ComplexityLevel::High => write!(f, "High"),
            ComplexityLevel::VeryHigh => write!(f, "Very High"),
        }
    }
}

/// Format analysis result as text
pub fn format_as_text(result: &AnalysisResult) -> String {
    let mut output = String::new();
    output.push_str("=== Code Analysis ===\n");
    output.push_str(&format!("Complexity Score: {:.2}\n", result.complexity_score));
    output.push_str(&format!("Quality Score: {:.2}\n", result.quality_score));
    if let Some(tdg) = result.tdg_score {
        output.push_str(&format!("TDG Score: {:.2}\n", tdg));
    }
    output.push_str(&format!("Lines: {}\n", result.line_count));
    output.push_str(&format!("Functions: {}\n", result.function_count));
    output.push_str(&format!("Classes: {}\n", result.class_count));
    output
}

/// Format analysis result as JSON
pub fn format_as_json(result: &AnalysisResult) -> Result<String> {
    Ok(serde_json::to_string_pretty(&serde_json::json!({
        "complexity_score": result.complexity_score,
        "quality_score": result.quality_score,
        "tdg_score": result.tdg_score,
        "line_count": result.line_count,
        "function_count": result.function_count,
        "class_count": result.class_count,
    }))?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_config_default() {
        let config = AnalyzeConfig::default();
        assert_eq!(config.format, "");
        assert!(!config.include_complexity);
    }

    #[test]
    fn test_analyze_simple_source() {
        let config = AnalyzeConfig::default();
        let source = "def hello():\n    print('Hello')\n";
        let result = analyze_source(source, &config).unwrap();
        assert!(result.complexity_score >= 0.0);
        assert_eq!(result.line_count, 2);
    }

    #[test]
    fn test_calculate_migration_complexity_low() {
        let result = AnalysisResult {
            complexity_score: 1.0,
            quality_score: 90.0,
            tdg_score: Some(85.0),
            line_count: 10,
            function_count: 1,
            class_count: 0,
            issues: vec![],
        };
        let complexity = calculate_migration_complexity(&result);
        assert_eq!(complexity.level, ComplexityLevel::Low);
        assert!(complexity.estimated_effort.contains("hour"));
    }

    #[test]
    fn test_calculate_migration_complexity_medium() {
        let result = AnalysisResult {
            complexity_score: 3.0,
            quality_score: 80.0,
            tdg_score: Some(75.0),
            line_count: 50,
            function_count: 5,
            class_count: 1,
            issues: vec![],
        };
        let complexity = calculate_migration_complexity(&result);
        assert_eq!(complexity.level, ComplexityLevel::Medium);
    }

    #[test]
    fn test_calculate_migration_complexity_high() {
        let result = AnalysisResult {
            complexity_score: 7.0,
            quality_score: 60.0,
            tdg_score: Some(55.0),
            line_count: 200,
            function_count: 20,
            class_count: 5,
            issues: vec![],
        };
        let complexity = calculate_migration_complexity(&result);
        assert_eq!(complexity.level, ComplexityLevel::High);
    }

    #[test]
    fn test_calculate_migration_complexity_very_high() {
        let result = AnalysisResult {
            complexity_score: 15.0,
            quality_score: 40.0,
            tdg_score: Some(35.0),
            line_count: 1000,
            function_count: 100,
            class_count: 20,
            issues: vec![],
        };
        let complexity = calculate_migration_complexity(&result);
        assert_eq!(complexity.level, ComplexityLevel::VeryHigh);
    }

    #[test]
    fn test_format_as_text() {
        let result = AnalysisResult {
            complexity_score: 5.5,
            quality_score: 85.0,
            tdg_score: Some(80.0),
            line_count: 100,
            function_count: 10,
            class_count: 2,
            issues: vec![],
        };
        let text = format_as_text(&result);
        assert!(text.contains("Complexity Score: 5.50"));
        assert!(text.contains("Quality Score: 85.00"));
        assert!(text.contains("Lines: 100"));
    }

    #[test]
    fn test_format_as_json() {
        let result = AnalysisResult {
            complexity_score: 5.5,
            quality_score: 85.0,
            tdg_score: Some(80.0),
            line_count: 100,
            function_count: 10,
            class_count: 2,
            issues: vec![],
        };
        let json = format_as_json(&result).unwrap();
        assert!(json.contains("\"complexity_score\""));
        assert!(json.contains("5.5"));
    }

    #[test]
    fn test_issue_severity() {
        let issue = AnalysisIssue {
            severity: IssueSeverity::Warning,
            message: "Test warning".to_string(),
            line: Some(10),
        };
        assert_eq!(issue.severity, IssueSeverity::Warning);
        assert_eq!(issue.line, Some(10));
    }

    #[test]
    fn test_complexity_level_display() {
        assert_eq!(format!("{}", ComplexityLevel::Low), "Low");
        assert_eq!(format!("{}", ComplexityLevel::Medium), "Medium");
        assert_eq!(format!("{}", ComplexityLevel::High), "High");
        assert_eq!(format!("{}", ComplexityLevel::VeryHigh), "Very High");
    }
}
