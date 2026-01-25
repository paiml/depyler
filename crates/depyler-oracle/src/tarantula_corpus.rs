//! Corpus Analysis for Tarantula Fault Localization
//!
//! Processes transpilation results from the reprorusted-python-cli corpus
//! to identify which transpiler decisions cause the most compilation failures.
//!
//! # DEPYLER-0631: Strategy #1 Implementation

use crate::tarantula::{
    FixPriority, SuspiciousTranspilerDecision, TarantulaAnalyzer, TarantulaResult,
    TranspilerDecision, TranspilerDecisionRecord,
};
use crate::OracleError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Result of a single transpilation attempt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranspilationResult {
    /// Python source file path
    pub python_file: String,
    /// Whether transpilation succeeded (generated valid Rust)
    pub transpiled: bool,
    /// Whether the generated Rust compiled successfully
    pub compiled: bool,
    /// Rust error codes if compilation failed
    pub error_codes: Vec<String>,
    /// Rust error messages if compilation failed
    pub error_messages: Vec<String>,
    /// Decision trace from the transpiler
    pub decisions: Vec<TranspilerDecisionRecord>,
}

impl TranspilationResult {
    /// Create a successful result
    #[must_use]
    pub fn success(
        python_file: impl Into<String>,
        decisions: Vec<TranspilerDecisionRecord>,
    ) -> Self {
        Self {
            python_file: python_file.into(),
            transpiled: true,
            compiled: true,
            error_codes: vec![],
            error_messages: vec![],
            decisions,
        }
    }

    /// Create a compile failure result
    #[must_use]
    pub fn compile_failure(
        python_file: impl Into<String>,
        decisions: Vec<TranspilerDecisionRecord>,
        error_codes: Vec<String>,
        error_messages: Vec<String>,
    ) -> Self {
        Self {
            python_file: python_file.into(),
            transpiled: true,
            compiled: false,
            error_codes,
            error_messages,
            decisions,
        }
    }

    /// Create a transpilation failure result
    #[must_use]
    pub fn transpile_failure(python_file: impl Into<String>) -> Self {
        Self {
            python_file: python_file.into(),
            transpiled: false,
            compiled: false,
            error_codes: vec![],
            error_messages: vec![],
            decisions: vec![],
        }
    }
}

/// Corpus analyzer for batch Tarantula analysis
pub struct CorpusAnalyzer {
    analyzer: TarantulaAnalyzer,
    results: Vec<TranspilationResult>,
}

impl CorpusAnalyzer {
    /// Create a new corpus analyzer
    pub fn new() -> Result<Self, OracleError> {
        Ok(Self {
            analyzer: TarantulaAnalyzer::new()?,
            results: Vec::new(),
        })
    }

    /// Add a transpilation result to the corpus
    pub fn add_result(&mut self, result: TranspilationResult) -> Result<(), OracleError> {
        if result.compiled {
            self.analyzer
                .record_success(&result.python_file, result.decisions.clone())?;
        } else if result.transpiled {
            self.analyzer.record_failure(
                &result.python_file,
                result.decisions.clone(),
                result.error_codes.clone(),
                result.error_messages.clone(),
            )?;
        }
        // Skip files that failed to transpile (no decisions to analyze)

        self.results.push(result);
        Ok(())
    }

    /// Process multiple results in batch
    pub fn add_results(&mut self, results: Vec<TranspilationResult>) -> Result<(), OracleError> {
        for result in results {
            self.add_result(result)?;
        }
        Ok(())
    }

    /// Run Tarantula analysis on the corpus
    #[must_use]
    pub fn analyze(&self) -> CorpusAnalysisReport {
        let tarantula = self.analyzer.analyze();
        let (success, failure) = self.analyzer.session_counts();

        let transpile_success = self.results.iter().filter(|r| r.transpiled).count();
        let compile_success = self.results.iter().filter(|r| r.compiled).count();

        // Compute correlation BEFORE moving suspicious_decisions
        let decision_error_correlation = self.compute_decision_error_correlation(&tarantula);

        CorpusAnalysisReport {
            total_files: self.results.len(),
            transpile_success,
            compile_success,
            transpile_rate: if self.results.is_empty() {
                0.0
            } else {
                transpile_success as f32 / self.results.len() as f32 * 100.0
            },
            single_shot_rate: if self.results.is_empty() {
                0.0
            } else {
                compile_success as f32 / self.results.len() as f32 * 100.0
            },
            tarantula_success: success,
            tarantula_failure: failure,
            suspicious_decisions: tarantula.suspicious,
            error_frequency: self.compute_error_frequency(),
            decision_error_correlation,
        }
    }

    /// Compute error code frequency
    fn compute_error_frequency(&self) -> HashMap<String, usize> {
        let mut freq: HashMap<String, usize> = HashMap::new();
        for result in &self.results {
            for code in &result.error_codes {
                *freq.entry(code.clone()).or_default() += 1;
            }
        }
        freq
    }

    /// Compute correlation between decisions and error codes
    fn compute_decision_error_correlation(
        &self,
        tarantula: &TarantulaResult,
    ) -> Vec<DecisionErrorCorrelation> {
        let mut correlations = Vec::new();

        for suspicious in &tarantula.suspicious {
            if suspicious.suspiciousness > 0.3 {
                correlations.push(DecisionErrorCorrelation {
                    decision_type: suspicious.decision_type,
                    suspiciousness: suspicious.suspiciousness,
                    associated_errors: suspicious.associated_errors.clone(),
                    priority: suspicious.priority,
                    recommended_action: recommend_action(suspicious),
                });
            }
        }

        correlations
    }

    /// Get the top N suspicious decisions (returns owned data)
    #[must_use]
    pub fn top_suspicious(&self, n: usize) -> Vec<SuspiciousTranspilerDecision> {
        let result = self.analyzer.analyze();
        result.suspicious.into_iter().take(n).collect()
    }

    /// Clear the corpus
    pub fn clear(&mut self) -> Result<(), OracleError> {
        self.results.clear();
        self.analyzer = TarantulaAnalyzer::new()?;
        Ok(())
    }
}

impl std::fmt::Debug for CorpusAnalyzer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CorpusAnalyzer")
            .field("results", &self.results.len())
            .finish()
    }
}

/// Comprehensive corpus analysis report
#[derive(Debug, Clone)]
pub struct CorpusAnalysisReport {
    /// Total files in corpus
    pub total_files: usize,
    /// Files that transpiled successfully
    pub transpile_success: usize,
    /// Files that compiled successfully (single-shot)
    pub compile_success: usize,
    /// Transpilation success rate (%)
    pub transpile_rate: f32,
    /// Single-shot compile rate (%) - the key metric
    pub single_shot_rate: f32,
    /// Successful sessions in Tarantula
    pub tarantula_success: usize,
    /// Failed sessions in Tarantula
    pub tarantula_failure: usize,
    /// Suspicious decisions sorted by score
    pub suspicious_decisions: Vec<SuspiciousTranspilerDecision>,
    /// Error code frequency
    pub error_frequency: HashMap<String, usize>,
    /// Decision-error correlations
    pub decision_error_correlation: Vec<DecisionErrorCorrelation>,
}

impl CorpusAnalysisReport {
    /// Get top N suspicious decisions
    #[must_use]
    pub fn top_suspicious(&self, n: usize) -> Vec<&SuspiciousTranspilerDecision> {
        self.suspicious_decisions.iter().take(n).collect()
    }

    /// Get top N error codes by frequency
    #[must_use]
    pub fn top_errors(&self, n: usize) -> Vec<(&String, &usize)> {
        let mut errors: Vec<_> = self.error_frequency.iter().collect();
        errors.sort_by(|a, b| b.1.cmp(a.1));
        errors.into_iter().take(n).collect()
    }

    /// Check if we've reached the 80% single-shot target
    #[must_use]
    pub fn reached_target(&self) -> bool {
        self.single_shot_rate >= 80.0
    }

    /// Format as markdown report
    #[must_use]
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();

        md.push_str("# Corpus Analysis Report\n\n");
        md.push_str("## Summary\n\n");
        md.push_str("| Metric | Value |\n");
        md.push_str("|--------|-------|\n");
        md.push_str(&format!("| Total Files | {} |\n", self.total_files));
        md.push_str(&format!(
            "| Transpile Success | {} |\n",
            self.transpile_success
        ));
        md.push_str(&format!("| Compile Success | {} |\n", self.compile_success));
        md.push_str(&format!(
            "| Transpile Rate | {:.1}% |\n",
            self.transpile_rate
        ));
        md.push_str(&format!(
            "| **Single-Shot Rate** | **{:.1}%** |\n",
            self.single_shot_rate
        ));
        md.push_str(&format!(
            "| Target (80%) | {} |\n",
            if self.reached_target() { "✅" } else { "❌" }
        ));

        md.push_str("\n## Top Suspicious Decisions\n\n");
        md.push_str("| Decision | Score | Fail | Success | Priority | Errors |\n");
        md.push_str("|----------|-------|------|---------|----------|--------|\n");

        for s in self.top_suspicious(10) {
            let errors = if s.associated_errors.is_empty() {
                "-".to_string()
            } else {
                s.associated_errors.join(", ")
            };
            md.push_str(&format!(
                "| {} | {:.2} | {} | {} | {:?} | {} |\n",
                s.decision_type,
                s.suspiciousness,
                s.fail_count,
                s.success_count,
                s.priority,
                errors
            ));
        }

        md.push_str("\n## Top Error Codes\n\n");
        md.push_str("| Error Code | Count |\n");
        md.push_str("|------------|-------|\n");

        for (code, count) in self.top_errors(10) {
            md.push_str(&format!("| {} | {} |\n", code, count));
        }

        md.push_str("\n## Recommended Actions\n\n");
        for corr in &self.decision_error_correlation {
            if corr.priority == FixPriority::Critical || corr.priority == FixPriority::High {
                md.push_str(&format!(
                    "- **{:?}** ({:.2}): {}\n",
                    corr.decision_type, corr.suspiciousness, corr.recommended_action
                ));
            }
        }

        md
    }
}

/// Correlation between a decision type and error codes
#[derive(Debug, Clone)]
pub struct DecisionErrorCorrelation {
    /// The decision type
    pub decision_type: TranspilerDecision,
    /// Suspiciousness score
    pub suspiciousness: f32,
    /// Associated error codes
    pub associated_errors: Vec<String>,
    /// Fix priority
    pub priority: FixPriority,
    /// Recommended action to fix
    pub recommended_action: String,
}

/// Generate recommended action for a suspicious decision
fn recommend_action(decision: &SuspiciousTranspilerDecision) -> String {
    match decision.decision_type {
        TranspilerDecision::TypeInference => {
            "Review type inference logic for common Python types".to_string()
        }
        TranspilerDecision::ModuleMapping => {
            "Add missing module mappings or improve stdlib coverage".to_string()
        }
        TranspilerDecision::MethodTranslation => {
            "Review method-to-function translation for Python builtins".to_string()
        }
        TranspilerDecision::OwnershipInference => {
            "Improve borrow vs clone vs move inference".to_string()
        }
        TranspilerDecision::ReturnTypeInference => {
            "Fix return type inference for fallible functions".to_string()
        }
        TranspilerDecision::ErrorHandling => "Review Result/Option wrapping strategy".to_string(),
        TranspilerDecision::ContainerMapping => {
            "Review list/dict/set container type mappings".to_string()
        }
        TranspilerDecision::ImportGeneration => {
            "Fix use statement generation for external crates".to_string()
        }
        _ => format!("Review {} decisions", decision.decision_type),
    }
}

/// Simulate decisions for a Python file based on common patterns
///
/// This is used when we don't have actual decision traces from the transpiler.
#[must_use]
pub fn simulate_decisions_from_errors(
    error_codes: &[String],
    error_messages: &[String],
) -> Vec<TranspilerDecisionRecord> {
    let mut decisions = Vec::new();

    for code in error_codes {
        match code.as_str() {
            "E0308" => {
                // Type mismatch
                decisions.push(TranspilerDecisionRecord::new(
                    TranspilerDecision::TypeInference,
                    "Type mismatch detected",
                ));
            }
            "E0433" => {
                // Unresolved import
                decisions.push(TranspilerDecisionRecord::new(
                    TranspilerDecision::ModuleMapping,
                    "Unresolved module import",
                ));
                decisions.push(TranspilerDecisionRecord::new(
                    TranspilerDecision::ImportGeneration,
                    "Missing use statement",
                ));
            }
            "E0599" => {
                // Method not found
                decisions.push(TranspilerDecisionRecord::new(
                    TranspilerDecision::MethodTranslation,
                    "Method not found on type",
                ));
            }
            "E0277" => {
                // Trait bound not satisfied
                decisions.push(TranspilerDecisionRecord::new(
                    TranspilerDecision::TypeInference,
                    "Trait bound not satisfied",
                ));
            }
            "E0425" => {
                // Unresolved name
                decisions.push(TranspilerDecisionRecord::new(
                    TranspilerDecision::ImportGeneration,
                    "Unresolved identifier",
                ));
            }
            "E0382" | "E0505" | "E0507" => {
                // Borrow checker errors
                decisions.push(TranspilerDecisionRecord::new(
                    TranspilerDecision::OwnershipInference,
                    "Borrow checker violation",
                ));
            }
            "E0106" => {
                // Missing lifetime
                decisions.push(TranspilerDecisionRecord::new(
                    TranspilerDecision::LifetimeInference,
                    "Missing lifetime specifier",
                ));
            }
            _ => {
                // Generic type inference issue
                decisions.push(TranspilerDecisionRecord::new(
                    TranspilerDecision::TypeInference,
                    format!("Error {}", code),
                ));
            }
        }
    }

    // Also check error messages for patterns
    for msg in error_messages {
        let msg_lower = msg.to_lowercase();
        if msg_lower.contains("subprocess") || msg_lower.contains("command") {
            decisions.push(TranspilerDecisionRecord::new(
                TranspilerDecision::ModuleMapping,
                "subprocess module usage",
            ));
        }
        if msg_lower.contains("datetime") || msg_lower.contains("time") {
            decisions.push(TranspilerDecisionRecord::new(
                TranspilerDecision::ModuleMapping,
                "datetime module usage",
            ));
        }
        if msg_lower.contains("os.") || msg_lower.contains("os::") {
            decisions.push(TranspilerDecisionRecord::new(
                TranspilerDecision::ModuleMapping,
                "os module usage",
            ));
        }
    }

    decisions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpilation_result_success() {
        let result = TranspilationResult::success("test.py", vec![]);
        assert!(result.transpiled);
        assert!(result.compiled);
    }

    #[test]
    fn test_transpilation_result_compile_failure() {
        let result = TranspilationResult::compile_failure(
            "test.py",
            vec![],
            vec!["E0308".to_string()],
            vec!["type mismatch".to_string()],
        );
        assert!(result.transpiled);
        assert!(!result.compiled);
        assert_eq!(result.error_codes, vec!["E0308"]);
    }

    #[test]
    fn test_corpus_analyzer_new() {
        let analyzer = CorpusAnalyzer::new().unwrap();
        assert_eq!(analyzer.results.len(), 0);
    }

    #[test]
    fn test_corpus_analyzer_add_success() {
        let mut analyzer = CorpusAnalyzer::new().unwrap();
        let result = TranspilationResult::success(
            "test.py",
            vec![TranspilerDecisionRecord::new(
                TranspilerDecision::TypeInference,
                "i32",
            )],
        );
        analyzer.add_result(result).unwrap();
        assert_eq!(analyzer.results.len(), 1);
    }

    #[test]
    fn test_corpus_analyzer_analysis() {
        let mut analyzer = CorpusAnalyzer::new().unwrap();

        // Add some successes
        for i in 0..3 {
            analyzer
                .add_result(TranspilationResult::success(
                    format!("good_{}.py", i),
                    vec![TranspilerDecisionRecord::new(
                        TranspilerDecision::TypeInference,
                        "good",
                    )],
                ))
                .unwrap();
        }

        // Add some failures
        for i in 0..5 {
            analyzer
                .add_result(TranspilationResult::compile_failure(
                    format!("bad_{}.py", i),
                    vec![TranspilerDecisionRecord::new(
                        TranspilerDecision::ModuleMapping,
                        "bad module",
                    )],
                    vec!["E0433".to_string()],
                    vec!["unresolved import".to_string()],
                ))
                .unwrap();
        }

        let report = analyzer.analyze();
        assert_eq!(report.total_files, 8);
        assert_eq!(report.transpile_success, 8);
        assert_eq!(report.compile_success, 3);
        assert!((report.single_shot_rate - 37.5).abs() < 0.1);
    }

    #[test]
    fn test_corpus_analysis_report_markdown() {
        let mut analyzer = CorpusAnalyzer::new().unwrap();
        analyzer
            .add_result(TranspilationResult::success("test.py", vec![]))
            .unwrap();
        let report = analyzer.analyze();
        let md = report.to_markdown();
        assert!(md.contains("Corpus Analysis Report"));
        assert!(md.contains("Single-Shot Rate"));
    }

    #[test]
    fn test_simulate_decisions_from_errors() {
        let decisions = simulate_decisions_from_errors(
            &["E0308".to_string(), "E0433".to_string()],
            &["subprocess not found".to_string()],
        );
        assert!(!decisions.is_empty());
        assert!(decisions
            .iter()
            .any(|d| d.decision_type == TranspilerDecision::TypeInference));
        assert!(decisions
            .iter()
            .any(|d| d.decision_type == TranspilerDecision::ModuleMapping));
    }

    #[test]
    fn test_report_reached_target() {
        let mut analyzer = CorpusAnalyzer::new().unwrap();

        // Add 80+ successes
        for i in 0..80 {
            analyzer
                .add_result(TranspilationResult::success(
                    format!("good_{}.py", i),
                    vec![],
                ))
                .unwrap();
        }

        // Add 20 failures
        for i in 0..20 {
            analyzer
                .add_result(TranspilationResult::compile_failure(
                    format!("bad_{}.py", i),
                    vec![],
                    vec!["E0001".to_string()],
                    vec![],
                ))
                .unwrap();
        }

        let report = analyzer.analyze();
        assert!(report.reached_target());
    }

    // ============================================================
    // Additional TranspilationResult Tests
    // ============================================================

    #[test]
    fn test_transpilation_result_transpile_failure() {
        let result = TranspilationResult::transpile_failure("broken.py");
        assert!(!result.transpiled);
        assert!(!result.compiled);
        assert!(result.error_codes.is_empty());
        assert!(result.decisions.is_empty());
    }

    #[test]
    fn test_transpilation_result_with_decisions() {
        let decisions = vec![
            TranspilerDecisionRecord::new(TranspilerDecision::TypeInference, "i32"),
            TranspilerDecisionRecord::new(TranspilerDecision::ModuleMapping, "std::collections"),
        ];
        let result = TranspilationResult::success("test.py", decisions.clone());
        assert_eq!(result.decisions.len(), 2);
        assert_eq!(
            result.decisions[0].decision_type,
            TranspilerDecision::TypeInference
        );
    }

    // ============================================================
    // CorpusAnalyzer Tests
    // ============================================================

    #[test]
    fn test_corpus_analyzer_add_transpile_failure() {
        let mut analyzer = CorpusAnalyzer::new().unwrap();
        let result = TranspilationResult::transpile_failure("broken.py");
        analyzer.add_result(result).unwrap();
        assert_eq!(analyzer.results.len(), 1);

        // Analyze should work even with transpile failures
        let report = analyzer.analyze();
        assert_eq!(report.total_files, 1);
        assert_eq!(report.transpile_success, 0);
    }

    #[test]
    fn test_corpus_analyzer_add_results() {
        let mut analyzer = CorpusAnalyzer::new().unwrap();
        let results = vec![
            TranspilationResult::success("a.py", vec![]),
            TranspilationResult::success("b.py", vec![]),
            TranspilationResult::compile_failure("c.py", vec![], vec!["E0001".to_string()], vec![]),
        ];
        analyzer.add_results(results).unwrap();
        assert_eq!(analyzer.results.len(), 3);
    }

    #[test]
    fn test_corpus_analyzer_clear() {
        let mut analyzer = CorpusAnalyzer::new().unwrap();
        analyzer
            .add_result(TranspilationResult::success("test.py", vec![]))
            .unwrap();
        assert_eq!(analyzer.results.len(), 1);

        analyzer.clear().unwrap();
        assert_eq!(analyzer.results.len(), 0);
    }

    #[test]
    fn test_corpus_analyzer_top_suspicious() {
        let mut analyzer = CorpusAnalyzer::new().unwrap();
        for i in 0..5 {
            analyzer
                .add_result(TranspilationResult::compile_failure(
                    format!("fail_{}.py", i),
                    vec![TranspilerDecisionRecord::new(
                        TranspilerDecision::TypeInference,
                        "bad",
                    )],
                    vec!["E0308".to_string()],
                    vec![],
                ))
                .unwrap();
        }

        let suspicious = analyzer.top_suspicious(3);
        assert!(suspicious.len() <= 3);
    }

    #[test]
    fn test_corpus_analyzer_debug() {
        let analyzer = CorpusAnalyzer::new().unwrap();
        let debug_str = format!("{:?}", analyzer);
        assert!(debug_str.contains("CorpusAnalyzer"));
        assert!(debug_str.contains("results"));
    }

    // ============================================================
    // CorpusAnalysisReport Tests
    // ============================================================

    #[test]
    fn test_report_top_suspicious() {
        let mut analyzer = CorpusAnalyzer::new().unwrap();
        analyzer
            .add_result(TranspilationResult::compile_failure(
                "test.py",
                vec![TranspilerDecisionRecord::new(
                    TranspilerDecision::TypeInference,
                    "bad",
                )],
                vec!["E0308".to_string()],
                vec![],
            ))
            .unwrap();

        let report = analyzer.analyze();
        let top = report.top_suspicious(5);
        assert!(top.len() <= 5);
    }

    #[test]
    fn test_report_top_errors() {
        let mut analyzer = CorpusAnalyzer::new().unwrap();
        for _ in 0..3 {
            analyzer
                .add_result(TranspilationResult::compile_failure(
                    "test.py",
                    vec![],
                    vec!["E0308".to_string()],
                    vec![],
                ))
                .unwrap();
        }
        analyzer
            .add_result(TranspilationResult::compile_failure(
                "test2.py",
                vec![],
                vec!["E0433".to_string()],
                vec![],
            ))
            .unwrap();

        let report = analyzer.analyze();
        let top_errors = report.top_errors(2);
        assert!(!top_errors.is_empty());
        // E0308 should be first with 3 occurrences
        assert_eq!(*top_errors[0].0, "E0308");
    }

    #[test]
    fn test_report_not_reached_target() {
        let mut analyzer = CorpusAnalyzer::new().unwrap();
        for i in 0..10 {
            analyzer
                .add_result(TranspilationResult::compile_failure(
                    format!("fail_{}.py", i),
                    vec![],
                    vec!["E0001".to_string()],
                    vec![],
                ))
                .unwrap();
        }

        let report = analyzer.analyze();
        assert!(!report.reached_target());
    }

    #[test]
    fn test_report_empty_corpus() {
        let analyzer = CorpusAnalyzer::new().unwrap();
        let report = analyzer.analyze();
        assert_eq!(report.total_files, 0);
        assert_eq!(report.transpile_rate, 0.0);
        assert_eq!(report.single_shot_rate, 0.0);
    }

    #[test]
    fn test_report_markdown_with_errors() {
        let mut analyzer = CorpusAnalyzer::new().unwrap();
        analyzer
            .add_result(TranspilationResult::compile_failure(
                "test.py",
                vec![TranspilerDecisionRecord::new(
                    TranspilerDecision::ModuleMapping,
                    "missing",
                )],
                vec!["E0433".to_string()],
                vec!["unresolved import".to_string()],
            ))
            .unwrap();

        let report = analyzer.analyze();
        let md = report.to_markdown();
        assert!(md.contains("E0433"));
        assert!(md.contains("Top Error Codes"));
    }

    // ============================================================
    // simulate_decisions_from_errors Tests
    // ============================================================

    #[test]
    fn test_simulate_decisions_e0308() {
        let decisions = simulate_decisions_from_errors(&["E0308".to_string()], &[]);
        assert!(decisions
            .iter()
            .any(|d| d.decision_type == TranspilerDecision::TypeInference));
    }

    #[test]
    fn test_simulate_decisions_e0433() {
        let decisions = simulate_decisions_from_errors(&["E0433".to_string()], &[]);
        assert!(decisions
            .iter()
            .any(|d| d.decision_type == TranspilerDecision::ModuleMapping));
        assert!(decisions
            .iter()
            .any(|d| d.decision_type == TranspilerDecision::ImportGeneration));
    }

    #[test]
    fn test_simulate_decisions_e0599() {
        let decisions = simulate_decisions_from_errors(&["E0599".to_string()], &[]);
        assert!(decisions
            .iter()
            .any(|d| d.decision_type == TranspilerDecision::MethodTranslation));
    }

    #[test]
    fn test_simulate_decisions_e0277() {
        let decisions = simulate_decisions_from_errors(&["E0277".to_string()], &[]);
        assert!(decisions
            .iter()
            .any(|d| d.decision_type == TranspilerDecision::TypeInference));
    }

    #[test]
    fn test_simulate_decisions_e0425() {
        let decisions = simulate_decisions_from_errors(&["E0425".to_string()], &[]);
        assert!(decisions
            .iter()
            .any(|d| d.decision_type == TranspilerDecision::ImportGeneration));
    }

    #[test]
    fn test_simulate_decisions_borrow_checker() {
        let decisions = simulate_decisions_from_errors(&["E0382".to_string()], &[]);
        assert!(decisions
            .iter()
            .any(|d| d.decision_type == TranspilerDecision::OwnershipInference));

        let decisions = simulate_decisions_from_errors(&["E0505".to_string()], &[]);
        assert!(decisions
            .iter()
            .any(|d| d.decision_type == TranspilerDecision::OwnershipInference));

        let decisions = simulate_decisions_from_errors(&["E0507".to_string()], &[]);
        assert!(decisions
            .iter()
            .any(|d| d.decision_type == TranspilerDecision::OwnershipInference));
    }

    #[test]
    fn test_simulate_decisions_e0106() {
        let decisions = simulate_decisions_from_errors(&["E0106".to_string()], &[]);
        assert!(decisions
            .iter()
            .any(|d| d.decision_type == TranspilerDecision::LifetimeInference));
    }

    #[test]
    fn test_simulate_decisions_unknown() {
        let decisions = simulate_decisions_from_errors(&["E9999".to_string()], &[]);
        assert!(decisions
            .iter()
            .any(|d| d.decision_type == TranspilerDecision::TypeInference));
    }

    #[test]
    fn test_simulate_decisions_message_subprocess() {
        let decisions =
            simulate_decisions_from_errors(&[], &["subprocess module not found".to_string()]);
        assert!(decisions
            .iter()
            .any(|d| d.decision_type == TranspilerDecision::ModuleMapping));
    }

    #[test]
    fn test_simulate_decisions_message_datetime() {
        let decisions =
            simulate_decisions_from_errors(&[], &["datetime import failed".to_string()]);
        assert!(decisions
            .iter()
            .any(|d| d.decision_type == TranspilerDecision::ModuleMapping));
    }

    #[test]
    fn test_simulate_decisions_message_os() {
        let decisions = simulate_decisions_from_errors(&[], &["os.path not found".to_string()]);
        assert!(decisions
            .iter()
            .any(|d| d.decision_type == TranspilerDecision::ModuleMapping));
    }

    #[test]
    fn test_simulate_decisions_message_time() {
        let decisions = simulate_decisions_from_errors(&[], &["time module issue".to_string()]);
        assert!(decisions
            .iter()
            .any(|d| d.decision_type == TranspilerDecision::ModuleMapping));
    }

    #[test]
    fn test_simulate_decisions_message_command() {
        let decisions = simulate_decisions_from_errors(&[], &["Command not found".to_string()]);
        assert!(decisions
            .iter()
            .any(|d| d.decision_type == TranspilerDecision::ModuleMapping));
    }

    // ============================================================
    // recommend_action Tests
    // ============================================================

    #[test]
    fn test_recommend_action_type_inference() {
        let decision = SuspiciousTranspilerDecision {
            decision_type: TranspilerDecision::TypeInference,
            suspiciousness: 0.8,
            fail_count: 10,
            success_count: 2,
            associated_errors: vec![],
            priority: FixPriority::Critical,
        };
        let action = recommend_action(&decision);
        assert!(action.contains("type inference"));
    }

    #[test]
    fn test_recommend_action_module_mapping() {
        let decision = SuspiciousTranspilerDecision {
            decision_type: TranspilerDecision::ModuleMapping,
            suspiciousness: 0.7,
            fail_count: 8,
            success_count: 2,
            associated_errors: vec![],
            priority: FixPriority::High,
        };
        let action = recommend_action(&decision);
        assert!(action.contains("module mapping"));
    }

    #[test]
    fn test_recommend_action_method_translation() {
        let decision = SuspiciousTranspilerDecision {
            decision_type: TranspilerDecision::MethodTranslation,
            suspiciousness: 0.6,
            fail_count: 6,
            success_count: 3,
            associated_errors: vec![],
            priority: FixPriority::Medium,
        };
        let action = recommend_action(&decision);
        assert!(action.contains("method"));
    }

    #[test]
    fn test_recommend_action_ownership() {
        let decision = SuspiciousTranspilerDecision {
            decision_type: TranspilerDecision::OwnershipInference,
            suspiciousness: 0.5,
            fail_count: 5,
            success_count: 5,
            associated_errors: vec![],
            priority: FixPriority::Medium,
        };
        let action = recommend_action(&decision);
        assert!(action.contains("borrow"));
    }

    #[test]
    fn test_recommend_action_return_type() {
        let decision = SuspiciousTranspilerDecision {
            decision_type: TranspilerDecision::ReturnTypeInference,
            suspiciousness: 0.4,
            fail_count: 4,
            success_count: 6,
            associated_errors: vec![],
            priority: FixPriority::Low,
        };
        let action = recommend_action(&decision);
        assert!(action.contains("return type"));
    }

    #[test]
    fn test_recommend_action_error_handling() {
        let decision = SuspiciousTranspilerDecision {
            decision_type: TranspilerDecision::ErrorHandling,
            suspiciousness: 0.4,
            fail_count: 4,
            success_count: 6,
            associated_errors: vec![],
            priority: FixPriority::Low,
        };
        let action = recommend_action(&decision);
        assert!(action.contains("Result") || action.contains("Option"));
    }

    #[test]
    fn test_recommend_action_container() {
        let decision = SuspiciousTranspilerDecision {
            decision_type: TranspilerDecision::ContainerMapping,
            suspiciousness: 0.4,
            fail_count: 4,
            success_count: 6,
            associated_errors: vec![],
            priority: FixPriority::Low,
        };
        let action = recommend_action(&decision);
        assert!(action.contains("container"));
    }

    #[test]
    fn test_recommend_action_import() {
        let decision = SuspiciousTranspilerDecision {
            decision_type: TranspilerDecision::ImportGeneration,
            suspiciousness: 0.4,
            fail_count: 4,
            success_count: 6,
            associated_errors: vec![],
            priority: FixPriority::Low,
        };
        let action = recommend_action(&decision);
        assert!(action.contains("use statement"));
    }

    #[test]
    fn test_recommend_action_other() {
        let decision = SuspiciousTranspilerDecision {
            decision_type: TranspilerDecision::LifetimeInference,
            suspiciousness: 0.4,
            fail_count: 4,
            success_count: 6,
            associated_errors: vec![],
            priority: FixPriority::Low,
        };
        let action = recommend_action(&decision);
        assert!(action.contains("Review"));
    }
}
