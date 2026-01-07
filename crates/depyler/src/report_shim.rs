//! Report Command Shim - pure logic separated from I/O
//!
//! Extracts testable logic from report_cmd/mod.rs

use std::collections::HashMap;

/// Compilation result for a single file (pure data)
#[derive(Debug, Clone)]
pub struct CompileResult {
    pub name: String,
    pub success: bool,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub python_source: Option<String>,
}

impl CompileResult {
    /// Create a successful result
    pub fn success(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            success: true,
            error_code: None,
            error_message: None,
            python_source: None,
        }
    }

    /// Create a failed result
    pub fn failure(name: impl Into<String>, code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            success: false,
            error_code: Some(code.into()),
            error_message: Some(message.into()),
            python_source: None,
        }
    }

    /// Add Python source
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.python_source = Some(source.into());
        self
    }
}

/// Error taxonomy entry
#[derive(Debug, Clone, Default)]
pub struct ErrorTaxonomy {
    pub count: usize,
    pub samples: Vec<String>,
}

impl ErrorTaxonomy {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a sample, keeping max 3
    pub fn add_sample(&mut self, sample: impl Into<String>) {
        self.count += 1;
        if self.samples.len() < 3 {
            self.samples.push(sample.into());
        }
    }

    /// Get percentage of total
    pub fn percentage(&self, total: usize) -> f64 {
        if total == 0 {
            0.0
        } else {
            (self.count as f64 / total as f64) * 100.0
        }
    }
}

/// Analyze compilation results (pure function)
pub fn analyze_results(results: &[CompileResult]) -> (usize, usize, HashMap<String, ErrorTaxonomy>) {
    let mut pass = 0;
    let mut fail = 0;
    let mut taxonomy: HashMap<String, ErrorTaxonomy> = HashMap::new();

    for result in results {
        if result.success {
            pass += 1;
        } else {
            fail += 1;
            let error_code = result.error_code.as_deref().unwrap_or("unknown");
            let entry = taxonomy.entry(error_code.to_string()).or_default();
            entry.add_sample(&result.name);
        }
    }

    (pass, fail, taxonomy)
}

/// Calculate compilation rate
pub fn calculate_rate(pass: usize, fail: usize) -> f64 {
    let total = pass + fail;
    if total == 0 {
        0.0
    } else {
        (pass as f64 / total as f64) * 100.0
    }
}

/// Check if target rate is achieved
pub fn target_achieved(pass: usize, fail: usize, target_rate: f64) -> bool {
    calculate_rate(pass, fail) >= target_rate * 100.0
}

/// Extract error code and message from compiler output
pub fn extract_error(stderr: &str) -> (String, String) {
    // Look for "error[E0XXX]:" pattern
    let code = stderr
        .lines()
        .find(|line| line.contains("error[E"))
        .and_then(|line| {
            let start = line.find("error[E")? + 6;
            let end = line[start..].find(']')? + start;
            Some(line[start..=end].to_string())
        })
        .unwrap_or_else(|| "unknown".to_string());

    // Extract message (first line after error code)
    let message = stderr
        .lines()
        .find(|line| line.contains("error[E") || line.starts_with("error:"))
        .map(|line| {
            if let Some(pos) = line.find("]: ") {
                line[pos + 3..].to_string()
            } else if let Some(pos) = line.find("error: ") {
                line[pos + 7..].to_string()
            } else {
                line.to_string()
            }
        })
        .unwrap_or_else(|| "compilation error".to_string());

    (code, message)
}

/// Filter statistics
#[derive(Debug, Clone, Default)]
pub struct FilterStats {
    pub total_files: usize,
    pub files_after_pattern: usize,
    pub files_after_tags: usize,
    pub files_after_limit: usize,
    pub files_after_sample: usize,
}

impl FilterStats {
    pub fn new(total: usize) -> Self {
        Self {
            total_files: total,
            files_after_pattern: total,
            files_after_tags: total,
            files_after_limit: total,
            files_after_sample: total,
        }
    }

    /// Calculate reduction percentage
    pub fn reduction_percent(&self) -> f64 {
        if self.total_files == 0 {
            0.0
        } else {
            let final_count = self.files_after_sample;
            (1.0 - (final_count as f64 / self.total_files as f64)) * 100.0
        }
    }

    /// Get final file count
    pub fn final_count(&self) -> usize {
        self.files_after_sample
    }
}

/// Bisection state for delta debugging
#[derive(Debug, Clone)]
pub struct BisectionState {
    pub files: Vec<String>,
    pub low: usize,
    pub high: usize,
    pub iteration: usize,
    pub max_iterations: usize,
    pub result: Option<Vec<String>>,
}

impl BisectionState {
    pub fn new(files: Vec<String>) -> Self {
        let len = files.len();
        Self {
            files,
            low: 0,
            high: len,
            iteration: 0,
            max_iterations: ((len as f64).log2().ceil() as usize) + 5,
            result: None,
        }
    }

    /// Check if bisection is complete
    pub fn is_complete(&self) -> bool {
        self.result.is_some() || self.iteration >= self.max_iterations
    }

    /// Get current test set (first half)
    pub fn current_test_set(&self) -> Vec<&String> {
        let mid = (self.low + self.high) / 2;
        self.files[self.low..mid].iter().collect()
    }

    /// Advance bisection based on test result
    pub fn advance(&mut self, has_failure: bool) {
        self.iteration += 1;
        let mid = (self.low + self.high) / 2;

        if has_failure {
            // Failure in first half, narrow to first half
            self.high = mid;
        } else {
            // No failure in first half, narrow to second half
            self.low = mid;
        }

        // Check if we've isolated a single file or small set
        if self.high - self.low <= 1 {
            self.result = Some(self.files[self.low..self.high].to_vec());
        }
    }

    /// Get bisection result
    pub fn get_result(&self) -> Option<Vec<&String>> {
        self.result.as_ref().map(|r| r.iter().collect())
    }

    /// Get remaining search space size
    pub fn remaining_space(&self) -> usize {
        self.high - self.low
    }

    /// Calculate progress percentage
    pub fn progress_percent(&self) -> f64 {
        if self.files.is_empty() {
            100.0
        } else {
            let initial = self.files.len();
            let remaining = self.remaining_space();
            (1.0 - (remaining as f64 / initial as f64)) * 100.0
        }
    }
}

/// Report summary statistics
#[derive(Debug, Clone)]
pub struct ReportSummary {
    pub total_files: usize,
    pub passed: usize,
    pub failed: usize,
    pub pass_rate: f64,
    pub target_rate: f64,
    pub target_achieved: bool,
    pub top_errors: Vec<(String, usize)>,
}

impl ReportSummary {
    pub fn from_results(results: &[CompileResult], target_rate: f64) -> Self {
        let (passed, failed, taxonomy) = analyze_results(results);
        let rate = calculate_rate(passed, failed);

        let mut top_errors: Vec<_> = taxonomy.iter()
            .map(|(k, v)| (k.clone(), v.count))
            .collect();
        top_errors.sort_by(|a, b| b.1.cmp(&a.1));
        top_errors.truncate(5);

        Self {
            total_files: passed + failed,
            passed,
            failed,
            pass_rate: rate,
            target_rate,
            target_achieved: rate >= target_rate * 100.0,
            top_errors,
        }
    }

    /// Get improvement needed to reach target
    pub fn improvement_needed(&self) -> f64 {
        if self.target_achieved {
            0.0
        } else {
            (self.target_rate * 100.0) - self.pass_rate
        }
    }

    /// Get files needed to fix to reach target
    pub fn files_to_fix(&self) -> usize {
        if self.target_achieved {
            0
        } else {
            let target_passed = (self.total_files as f64 * self.target_rate).ceil() as usize;
            target_passed.saturating_sub(self.passed)
        }
    }
}

/// Semantic tag for file classification
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SemanticTag {
    Dict,
    List,
    Tuple,
    Set,
    String,
    Numeric,
    IO,
    Argparse,
    Regex,
    Json,
    Async,
    Class,
    Generic,
    Other(String),
}

impl SemanticTag {
    /// Parse from string
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "dict" | "dictionary" => Self::Dict,
            "list" | "array" => Self::List,
            "tuple" => Self::Tuple,
            "set" => Self::Set,
            "string" | "str" => Self::String,
            "numeric" | "int" | "float" => Self::Numeric,
            "io" | "file" => Self::IO,
            "argparse" | "cli" | "args" => Self::Argparse,
            "regex" | "re" => Self::Regex,
            "json" => Self::Json,
            "async" | "await" => Self::Async,
            "class" | "oop" => Self::Class,
            "generic" | "typing" => Self::Generic,
            _ => Self::Other(s.to_string()),
        }
    }

    /// Check if file content matches this tag
    pub fn matches_content(&self, content: &str) -> bool {
        match self {
            Self::Dict => content.contains("dict") || content.contains("Dict") || content.contains("{:"),
            Self::List => content.contains("list") || content.contains("List") || content.contains("["),
            Self::Tuple => content.contains("tuple") || content.contains("Tuple"),
            Self::Set => content.contains("set") || content.contains("Set"),
            Self::String => content.contains("str") || content.contains("String"),
            Self::Numeric => content.contains("int") || content.contains("float"),
            Self::IO => content.contains("open(") || content.contains("read(") || content.contains("write("),
            Self::Argparse => content.contains("argparse") || content.contains("ArgumentParser"),
            Self::Regex => content.contains("import re") || content.contains("re."),
            Self::Json => content.contains("import json") || content.contains("json."),
            Self::Async => content.contains("async ") || content.contains("await "),
            Self::Class => content.contains("class "),
            Self::Generic => content.contains("TypeVar") || content.contains("Generic"),
            Self::Other(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_result_success() {
        let result = CompileResult::success("test.py");
        assert!(result.success);
        assert!(result.error_code.is_none());
        assert_eq!(result.name, "test.py");
    }

    #[test]
    fn test_compile_result_failure() {
        let result = CompileResult::failure("test.py", "E0308", "type mismatch");
        assert!(!result.success);
        assert_eq!(result.error_code, Some("E0308".to_string()));
        assert_eq!(result.error_message, Some("type mismatch".to_string()));
    }

    #[test]
    fn test_compile_result_with_source() {
        let result = CompileResult::success("test.py")
            .with_source("print('hello')");
        assert_eq!(result.python_source, Some("print('hello')".to_string()));
    }

    #[test]
    fn test_error_taxonomy_add_sample() {
        let mut taxonomy = ErrorTaxonomy::new();
        taxonomy.add_sample("file1.py");
        taxonomy.add_sample("file2.py");
        taxonomy.add_sample("file3.py");
        taxonomy.add_sample("file4.py"); // Should not be added

        assert_eq!(taxonomy.count, 4);
        assert_eq!(taxonomy.samples.len(), 3);
    }

    #[test]
    fn test_error_taxonomy_percentage() {
        let mut taxonomy = ErrorTaxonomy::new();
        taxonomy.count = 25;

        assert!((taxonomy.percentage(100) - 25.0).abs() < 0.001);
        assert!((taxonomy.percentage(50) - 50.0).abs() < 0.001);
        assert_eq!(taxonomy.percentage(0), 0.0);
    }

    #[test]
    fn test_analyze_results() {
        let results = vec![
            CompileResult::success("a.py"),
            CompileResult::success("b.py"),
            CompileResult::failure("c.py", "E0308", "type mismatch"),
            CompileResult::failure("d.py", "E0308", "type mismatch"),
            CompileResult::failure("e.py", "E0277", "trait not satisfied"),
        ];

        let (pass, fail, taxonomy) = analyze_results(&results);

        assert_eq!(pass, 2);
        assert_eq!(fail, 3);
        assert_eq!(taxonomy.len(), 2);
        assert_eq!(taxonomy.get("E0308").unwrap().count, 2);
        assert_eq!(taxonomy.get("E0277").unwrap().count, 1);
    }

    #[test]
    fn test_calculate_rate() {
        assert!((calculate_rate(80, 20) - 80.0).abs() < 0.001);
        assert!((calculate_rate(0, 100) - 0.0).abs() < 0.001);
        assert!((calculate_rate(100, 0) - 100.0).abs() < 0.001);
        assert_eq!(calculate_rate(0, 0), 0.0);
    }

    #[test]
    fn test_target_achieved() {
        assert!(target_achieved(95, 5, 0.90));
        assert!(target_achieved(90, 10, 0.90));
        assert!(!target_achieved(89, 11, 0.90));
        assert!(!target_achieved(80, 20, 0.90));
    }

    #[test]
    fn test_extract_error() {
        let stderr = r#"error[E0308]: mismatched types
  --> src/main.rs:10:5
   |
10 |     let x: i32 = "hello";
   |            --- ^^^^^^^^^ expected `i32`, found `&str`
"#;

        let (code, message) = extract_error(stderr);
        assert_eq!(code, "E0308]");
        assert!(message.contains("mismatched types"));
    }

    #[test]
    fn test_extract_error_unknown() {
        let (code, message) = extract_error("some other error");
        assert_eq!(code, "unknown");
        assert_eq!(message, "compilation error");
    }

    #[test]
    fn test_filter_stats() {
        let mut stats = FilterStats::new(100);
        stats.files_after_pattern = 80;
        stats.files_after_tags = 60;
        stats.files_after_limit = 50;
        stats.files_after_sample = 25;

        assert_eq!(stats.final_count(), 25);
        assert!((stats.reduction_percent() - 75.0).abs() < 0.001);
    }

    #[test]
    fn test_bisection_state_new() {
        let files = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()];
        let state = BisectionState::new(files);

        assert_eq!(state.low, 0);
        assert_eq!(state.high, 4);
        assert_eq!(state.iteration, 0);
        assert!(!state.is_complete());
    }

    #[test]
    fn test_bisection_state_advance() {
        let files: Vec<String> = (0..8).map(|i| format!("file{}.py", i)).collect();
        let mut state = BisectionState::new(files);

        // First half has failure
        state.advance(true);
        assert_eq!(state.high, 4);
        assert_eq!(state.low, 0);

        // First half still has failure
        state.advance(true);
        assert_eq!(state.high, 2);
    }

    #[test]
    fn test_bisection_state_complete() {
        let files = vec!["a".to_string(), "b".to_string()];
        let mut state = BisectionState::new(files);

        state.advance(true); // Narrow to first half
        assert!(state.is_complete());
        assert!(state.get_result().is_some());
    }

    #[test]
    fn test_bisection_progress() {
        let files: Vec<String> = (0..16).map(|i| format!("file{}.py", i)).collect();
        let mut state = BisectionState::new(files);

        assert_eq!(state.progress_percent(), 0.0);

        state.advance(true); // 16 -> 8
        assert!((state.progress_percent() - 50.0).abs() < 0.001);

        state.advance(true); // 8 -> 4
        assert!((state.progress_percent() - 75.0).abs() < 0.001);
    }

    #[test]
    fn test_report_summary() {
        let results = vec![
            CompileResult::success("a.py"),
            CompileResult::success("b.py"),
            CompileResult::failure("c.py", "E0308", "error"),
        ];

        let summary = ReportSummary::from_results(&results, 0.80);

        assert_eq!(summary.total_files, 3);
        assert_eq!(summary.passed, 2);
        assert_eq!(summary.failed, 1);
        assert!((summary.pass_rate - 66.67).abs() < 0.1);
        assert!(!summary.target_achieved);
    }

    #[test]
    fn test_report_summary_improvement_needed() {
        let results = vec![
            CompileResult::success("a.py"),
            CompileResult::failure("b.py", "E0308", "error"),
        ];

        let summary = ReportSummary::from_results(&results, 0.80);
        assert!((summary.improvement_needed() - 30.0).abs() < 0.1);
    }

    #[test]
    fn test_report_summary_files_to_fix() {
        let results: Vec<_> = (0..100).map(|i| {
            if i < 70 {
                CompileResult::success(format!("file{}.py", i))
            } else {
                CompileResult::failure(format!("file{}.py", i), "E0308", "error")
            }
        }).collect();

        let summary = ReportSummary::from_results(&results, 0.80);
        assert_eq!(summary.files_to_fix(), 10); // Need 80 passed, have 70
    }

    #[test]
    fn test_semantic_tag_from_str() {
        assert_eq!(SemanticTag::from_str("dict"), SemanticTag::Dict);
        assert_eq!(SemanticTag::from_str("Dict"), SemanticTag::Dict);
        assert_eq!(SemanticTag::from_str("list"), SemanticTag::List);
        assert_eq!(SemanticTag::from_str("argparse"), SemanticTag::Argparse);
        assert!(matches!(SemanticTag::from_str("unknown"), SemanticTag::Other(_)));
    }

    #[test]
    fn test_semantic_tag_matches_content() {
        assert!(SemanticTag::Dict.matches_content("x: Dict[str, int] = {}"));
        assert!(SemanticTag::List.matches_content("items: List[int] = []"));
        assert!(SemanticTag::Argparse.matches_content("import argparse"));
        assert!(SemanticTag::Async.matches_content("async def main():"));
        assert!(SemanticTag::Class.matches_content("class Foo:"));
        assert!(!SemanticTag::Dict.matches_content("x = 42"));
    }

    #[test]
    fn test_semantic_tag_io() {
        assert!(SemanticTag::IO.matches_content("with open('file.txt') as f:"));
        assert!(SemanticTag::IO.matches_content("f.read()"));
        assert!(SemanticTag::IO.matches_content("f.write('data')"));
    }

    #[test]
    fn test_semantic_tag_json_regex() {
        assert!(SemanticTag::Json.matches_content("import json\ndata = json.loads(s)"));
        assert!(SemanticTag::Regex.matches_content("import re\nre.match(r'\\d+', s)"));
    }

    // =====================================================
    // Additional CompileResult Tests
    // =====================================================

    #[test]
    fn test_compile_result_debug() {
        let result = CompileResult::success("test.py");
        let debug = format!("{:?}", result);
        assert!(debug.contains("test.py"));
    }

    #[test]
    fn test_compile_result_clone() {
        let result = CompileResult::failure("a.py", "E0308", "error");
        let cloned = result.clone();
        assert_eq!(result.name, cloned.name);
        assert_eq!(result.error_code, cloned.error_code);
    }

    #[test]
    fn test_compile_result_chain() {
        let result = CompileResult::failure("x.py", "E0001", "msg")
            .with_source("def foo(): pass");
        assert!(!result.success);
        assert!(result.python_source.is_some());
    }

    // =====================================================
    // Additional ErrorTaxonomy Tests
    // =====================================================

    #[test]
    fn test_error_taxonomy_default() {
        let taxonomy = ErrorTaxonomy::default();
        assert_eq!(taxonomy.count, 0);
        assert!(taxonomy.samples.is_empty());
    }

    #[test]
    fn test_error_taxonomy_percentage_100() {
        let mut taxonomy = ErrorTaxonomy::new();
        taxonomy.count = 100;
        assert!((taxonomy.percentage(100) - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_error_taxonomy_single_sample() {
        let mut taxonomy = ErrorTaxonomy::new();
        taxonomy.add_sample("only_one.py");
        assert_eq!(taxonomy.count, 1);
        assert_eq!(taxonomy.samples.len(), 1);
    }

    // =====================================================
    // Additional analyze_results Tests
    // =====================================================

    #[test]
    fn test_analyze_results_all_success() {
        let results = vec![
            CompileResult::success("a.py"),
            CompileResult::success("b.py"),
        ];
        let (pass, fail, taxonomy) = analyze_results(&results);
        assert_eq!(pass, 2);
        assert_eq!(fail, 0);
        assert!(taxonomy.is_empty());
    }

    #[test]
    fn test_analyze_results_all_failure() {
        let results = vec![
            CompileResult::failure("a.py", "E0308", "e1"),
            CompileResult::failure("b.py", "E0308", "e2"),
        ];
        let (pass, fail, taxonomy) = analyze_results(&results);
        assert_eq!(pass, 0);
        assert_eq!(fail, 2);
        assert_eq!(taxonomy.get("E0308").unwrap().count, 2);
    }

    #[test]
    fn test_analyze_results_empty() {
        let results: Vec<CompileResult> = vec![];
        let (pass, fail, taxonomy) = analyze_results(&results);
        assert_eq!(pass, 0);
        assert_eq!(fail, 0);
        assert!(taxonomy.is_empty());
    }

    #[test]
    fn test_analyze_results_unknown_error() {
        let mut result = CompileResult::failure("a.py", "E0000", "unknown");
        result.error_code = None;
        let results = vec![result];
        let (_, fail, taxonomy) = analyze_results(&results);
        assert_eq!(fail, 1);
        assert!(taxonomy.contains_key("unknown"));
    }

    // =====================================================
    // Additional extract_error Tests
    // =====================================================

    #[test]
    fn test_extract_error_simple_error() {
        let stderr = "error: linking with `cc` failed";
        let (code, message) = extract_error(stderr);
        assert_eq!(code, "unknown");
        assert!(message.contains("linking"));
    }

    #[test]
    fn test_extract_error_multiple_errors() {
        let stderr = r#"error[E0308]: mismatched types
error[E0277]: trait bound not satisfied"#;
        let (code, _) = extract_error(stderr);
        assert!(code.contains("E0308"));
    }

    #[test]
    fn test_extract_error_empty() {
        let (code, message) = extract_error("");
        assert_eq!(code, "unknown");
        assert_eq!(message, "compilation error");
    }

    // =====================================================
    // Additional FilterStats Tests
    // =====================================================

    #[test]
    fn test_filter_stats_default() {
        let stats = FilterStats::default();
        assert_eq!(stats.total_files, 0);
    }

    #[test]
    fn test_filter_stats_no_reduction() {
        let stats = FilterStats::new(100);
        assert_eq!(stats.final_count(), 100);
        assert_eq!(stats.reduction_percent(), 0.0);
    }

    #[test]
    fn test_filter_stats_full_reduction() {
        let mut stats = FilterStats::new(100);
        stats.files_after_sample = 0;
        assert_eq!(stats.reduction_percent(), 100.0);
    }

    #[test]
    fn test_filter_stats_clone() {
        let stats = FilterStats::new(50);
        let cloned = stats.clone();
        assert_eq!(stats.total_files, cloned.total_files);
    }

    // =====================================================
    // Additional BisectionState Tests
    // =====================================================

    #[test]
    fn test_bisection_state_empty() {
        let state = BisectionState::new(vec![]);
        assert_eq!(state.remaining_space(), 0);
        assert_eq!(state.progress_percent(), 100.0);
    }

    #[test]
    fn test_bisection_state_single_file() {
        let files = vec!["only.py".to_string()];
        let mut state = BisectionState::new(files);
        state.advance(true);
        assert!(state.is_complete());
    }

    #[test]
    fn test_bisection_state_advance_no_failure() {
        let files: Vec<String> = (0..8).map(|i| format!("file{}.py", i)).collect();
        let mut state = BisectionState::new(files);
        state.advance(false); // No failure in first half
        assert_eq!(state.low, 4);
        assert_eq!(state.high, 8);
    }

    #[test]
    fn test_bisection_state_current_test_set() {
        let files: Vec<String> = (0..4).map(|i| format!("f{}.py", i)).collect();
        let state = BisectionState::new(files);
        let test_set = state.current_test_set();
        assert_eq!(test_set.len(), 2);
    }

    #[test]
    fn test_bisection_state_max_iterations() {
        let files: Vec<String> = (0..16).map(|i| format!("f{}.py", i)).collect();
        let state = BisectionState::new(files);
        assert!(state.max_iterations > 4);
    }

    #[test]
    fn test_bisection_state_clone() {
        let files = vec!["a".to_string()];
        let state = BisectionState::new(files);
        let cloned = state.clone();
        assert_eq!(state.low, cloned.low);
    }

    // =====================================================
    // Additional ReportSummary Tests
    // =====================================================

    #[test]
    fn test_report_summary_target_achieved() {
        let results = vec![
            CompileResult::success("a.py"),
            CompileResult::success("b.py"),
            CompileResult::success("c.py"),
            CompileResult::success("d.py"),
            CompileResult::failure("e.py", "E0001", "err"),
        ];
        let summary = ReportSummary::from_results(&results, 0.80);
        assert!(summary.target_achieved);
        assert_eq!(summary.improvement_needed(), 0.0);
    }

    #[test]
    fn test_report_summary_empty_results() {
        let results: Vec<CompileResult> = vec![];
        let summary = ReportSummary::from_results(&results, 0.80);
        assert_eq!(summary.total_files, 0);
        assert_eq!(summary.pass_rate, 0.0);
    }

    #[test]
    fn test_report_summary_top_errors_limit() {
        let results: Vec<_> = (0..10)
            .map(|i| CompileResult::failure(format!("f{}.py", i), format!("E{:04}", i), "err"))
            .collect();
        let summary = ReportSummary::from_results(&results, 0.80);
        assert!(summary.top_errors.len() <= 5);
    }

    #[test]
    fn test_report_summary_files_to_fix_achieved() {
        let results: Vec<_> = (0..100)
            .map(|i| CompileResult::success(format!("f{}.py", i)))
            .collect();
        let summary = ReportSummary::from_results(&results, 0.80);
        assert_eq!(summary.files_to_fix(), 0);
    }

    #[test]
    fn test_report_summary_clone() {
        let results = vec![CompileResult::success("a.py")];
        let summary = ReportSummary::from_results(&results, 0.80);
        let cloned = summary.clone();
        assert_eq!(summary.total_files, cloned.total_files);
    }

    // =====================================================
    // Additional SemanticTag Tests
    // =====================================================

    #[test]
    fn test_semantic_tag_tuple() {
        assert_eq!(SemanticTag::from_str("tuple"), SemanticTag::Tuple);
        assert!(SemanticTag::Tuple.matches_content("x: Tuple[int, str]"));
    }

    #[test]
    fn test_semantic_tag_set() {
        assert_eq!(SemanticTag::from_str("set"), SemanticTag::Set);
        assert!(SemanticTag::Set.matches_content("items: Set[int] = set()"));
    }

    #[test]
    fn test_semantic_tag_string() {
        assert_eq!(SemanticTag::from_str("str"), SemanticTag::String);
        assert!(SemanticTag::String.matches_content("x: str = ''"));
    }

    #[test]
    fn test_semantic_tag_numeric() {
        assert_eq!(SemanticTag::from_str("int"), SemanticTag::Numeric);
        assert_eq!(SemanticTag::from_str("float"), SemanticTag::Numeric);
        assert!(SemanticTag::Numeric.matches_content("x: int = 42"));
    }

    #[test]
    fn test_semantic_tag_generic() {
        assert_eq!(SemanticTag::from_str("typing"), SemanticTag::Generic);
        assert!(SemanticTag::Generic.matches_content("T = TypeVar('T')"));
    }

    #[test]
    fn test_semantic_tag_cli() {
        assert_eq!(SemanticTag::from_str("cli"), SemanticTag::Argparse);
        assert_eq!(SemanticTag::from_str("args"), SemanticTag::Argparse);
    }

    #[test]
    fn test_semantic_tag_other_matches_nothing() {
        let tag = SemanticTag::Other("custom".to_string());
        assert!(!tag.matches_content("anything"));
    }

    #[test]
    fn test_semantic_tag_dictionary() {
        assert_eq!(SemanticTag::from_str("dictionary"), SemanticTag::Dict);
    }

    #[test]
    fn test_semantic_tag_array() {
        assert_eq!(SemanticTag::from_str("array"), SemanticTag::List);
    }

    #[test]
    fn test_semantic_tag_file() {
        assert_eq!(SemanticTag::from_str("file"), SemanticTag::IO);
    }

    #[test]
    fn test_semantic_tag_await() {
        assert_eq!(SemanticTag::from_str("await"), SemanticTag::Async);
    }

    #[test]
    fn test_semantic_tag_oop() {
        assert_eq!(SemanticTag::from_str("oop"), SemanticTag::Class);
    }

    #[test]
    fn test_semantic_tag_re() {
        assert_eq!(SemanticTag::from_str("re"), SemanticTag::Regex);
    }

    #[test]
    fn test_semantic_tag_eq() {
        assert_eq!(SemanticTag::Dict, SemanticTag::Dict);
        assert_ne!(SemanticTag::Dict, SemanticTag::List);
    }

    #[test]
    fn test_semantic_tag_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(SemanticTag::Dict);
        set.insert(SemanticTag::List);
        set.insert(SemanticTag::Dict);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_semantic_tag_debug() {
        let tag = SemanticTag::Dict;
        let debug = format!("{:?}", tag);
        assert!(debug.contains("Dict"));
    }

    #[test]
    fn test_semantic_tag_clone() {
        let tag = SemanticTag::Async;
        let cloned = tag.clone();
        assert_eq!(tag, cloned);
    }

    // =====================================================
    // Additional calculate_rate Tests
    // =====================================================

    #[test]
    fn test_calculate_rate_50_percent() {
        assert!((calculate_rate(50, 50) - 50.0).abs() < 0.001);
    }

    #[test]
    fn test_calculate_rate_large_numbers() {
        assert!((calculate_rate(999999, 1) - 99.9999).abs() < 0.01);
    }

    // =====================================================
    // target_achieved edge cases
    // =====================================================

    #[test]
    fn test_target_achieved_exact() {
        assert!(target_achieved(80, 20, 0.80));
    }

    #[test]
    fn test_target_achieved_zero_target() {
        assert!(target_achieved(0, 100, 0.0));
    }

    #[test]
    fn test_target_achieved_100_target() {
        assert!(target_achieved(100, 0, 1.0));
        assert!(!target_achieved(99, 1, 1.0));
    }
}
