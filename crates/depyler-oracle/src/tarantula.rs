//! Tarantula Fault Localization for Depyler Transpiler
//!
//! Identifies which transpiler decisions (codegen rules) are most likely
//! to cause compilation failures using the Tarantula algorithm.
//!
//! # References
//!
//! - Jones & Harrold (2005): Tarantula Fault Localization
//! - docs/specifications/single-shot-80-percentage-review.md#102-strategy-1

use entrenar::citl::{
    CITLConfig, CompilationOutcome, DecisionCITL, DecisionStats, DecisionTrace, SourceSpan,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Decision types tracked by the Depyler transpiler
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TranspilerDecision {
    TypeInference,
    OwnershipInference,
    LifetimeInference,
    ModuleMapping,
    MethodTranslation,
    ReturnTypeInference,
    IteratorTransform,
    ErrorHandling,
    ContainerMapping,
    FunctionSignature,
    ImportGeneration,
    StringFormatting,
    NumericType,
    BooleanExpression,
    ControlFlow,
}

impl TranspilerDecision {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TypeInference => "type_inference",
            Self::OwnershipInference => "ownership_inference",
            Self::LifetimeInference => "lifetime_inference",
            Self::ModuleMapping => "module_mapping",
            Self::MethodTranslation => "method_translation",
            Self::ReturnTypeInference => "return_type_inference",
            Self::IteratorTransform => "iterator_transform",
            Self::ErrorHandling => "error_handling",
            Self::ContainerMapping => "container_mapping",
            Self::FunctionSignature => "function_signature",
            Self::ImportGeneration => "import_generation",
            Self::StringFormatting => "string_formatting",
            Self::NumericType => "numeric_type",
            Self::BooleanExpression => "boolean_expression",
            Self::ControlFlow => "control_flow",
        }
    }

    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "type_inference" => Some(Self::TypeInference),
            "ownership_inference" => Some(Self::OwnershipInference),
            "lifetime_inference" => Some(Self::LifetimeInference),
            "module_mapping" => Some(Self::ModuleMapping),
            "method_translation" => Some(Self::MethodTranslation),
            "return_type_inference" => Some(Self::ReturnTypeInference),
            "iterator_transform" => Some(Self::IteratorTransform),
            "error_handling" => Some(Self::ErrorHandling),
            "container_mapping" => Some(Self::ContainerMapping),
            "function_signature" => Some(Self::FunctionSignature),
            "import_generation" => Some(Self::ImportGeneration),
            "string_formatting" => Some(Self::StringFormatting),
            "numeric_type" => Some(Self::NumericType),
            "boolean_expression" => Some(Self::BooleanExpression),
            "control_flow" => Some(Self::ControlFlow),
            _ => None,
        }
    }

    #[must_use]
    pub fn all() -> &'static [Self] {
        &[
            Self::TypeInference,
            Self::OwnershipInference,
            Self::LifetimeInference,
            Self::ModuleMapping,
            Self::MethodTranslation,
            Self::ReturnTypeInference,
            Self::IteratorTransform,
            Self::ErrorHandling,
            Self::ContainerMapping,
            Self::FunctionSignature,
            Self::ImportGeneration,
            Self::StringFormatting,
            Self::NumericType,
            Self::BooleanExpression,
            Self::ControlFlow,
        ]
    }
}

impl std::fmt::Display for TranspilerDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A recorded transpiler decision with context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranspilerDecisionRecord {
    pub decision_type: TranspilerDecision,
    pub description: String,
    pub python_line: Option<u32>,
    pub python_node: Option<String>,
    pub rust_snippet: Option<String>,
    pub context: HashMap<String, String>,
}

impl TranspilerDecisionRecord {
    #[must_use]
    pub fn new(decision_type: TranspilerDecision, description: impl Into<String>) -> Self {
        Self {
            decision_type,
            description: description.into(),
            python_line: None,
            python_node: None,
            rust_snippet: None,
            context: HashMap::new(),
        }
    }

    #[must_use]
    pub fn with_python_line(mut self, line: u32) -> Self {
        self.python_line = Some(line);
        self
    }

    #[must_use]
    pub fn with_rust_snippet(mut self, snippet: impl Into<String>) -> Self {
        let s: String = snippet.into();
        self.rust_snippet = Some(if s.len() > 100 {
            format!("{}...", &s[..97])
        } else {
            s
        });
        self
    }

    #[must_use]
    pub fn to_trace(&self, id: &str, file: &str) -> DecisionTrace {
        let mut trace =
            DecisionTrace::new(id, self.decision_type.as_str(), self.description.clone());
        if let Some(line) = self.python_line {
            trace = trace.with_span(SourceSpan::line(file, line));
        }
        trace
    }
}

/// Priority for fixing a suspicious decision
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FixPriority {
    Critical,
    High,
    Medium,
    Low,
}

impl FixPriority {
    #[must_use]
    pub fn from_failure_rate(rate: f32) -> Self {
        if rate > 0.20 {
            Self::Critical
        } else if rate > 0.10 {
            Self::High
        } else if rate > 0.05 {
            Self::Medium
        } else {
            Self::Low
        }
    }
}

/// A suspicious transpiler decision with analysis
#[derive(Debug, Clone)]
pub struct SuspiciousTranspilerDecision {
    pub decision_type: TranspilerDecision,
    pub suspiciousness: f32,
    pub fail_count: u32,
    pub success_count: u32,
    pub associated_errors: Vec<String>,
    pub priority: FixPriority,
}

/// Result of Tarantula analysis
#[derive(Debug, Clone)]
pub struct TarantulaResult {
    pub suspicious: Vec<SuspiciousTranspilerDecision>,
    pub total_success: usize,
    pub total_failure: usize,
    pub stats: HashMap<TranspilerDecision, DecisionStats>,
}

impl TarantulaResult {
    #[must_use]
    pub fn top(&self, n: usize) -> Vec<&SuspiciousTranspilerDecision> {
        self.suspicious.iter().take(n).collect()
    }

    #[must_use]
    pub fn is_suspicious(&self, decision: TranspilerDecision, threshold: f32) -> bool {
        self.suspicious
            .iter()
            .find(|s| s.decision_type == decision)
            .is_some_and(|s| s.suspiciousness > threshold)
    }

    #[must_use]
    pub fn score(&self, decision: TranspilerDecision) -> Option<f32> {
        self.suspicious
            .iter()
            .find(|s| s.decision_type == decision)
            .map(|s| s.suspiciousness)
    }
}

/// Tarantula analyzer for Depyler transpiler decisions
pub struct TarantulaAnalyzer {
    citl: DecisionCITL,
    session_id: u64,
    error_associations: HashMap<String, Vec<TranspilerDecision>>,
}

impl TarantulaAnalyzer {
    /// Create a new analyzer
    pub fn new() -> Result<Self, crate::OracleError> {
        let config = CITLConfig {
            max_suggestions: 10,
            min_suspiciousness: 0.1,
            enable_dependency_graph: true,
        };
        let citl = DecisionCITL::with_config(config)
            .map_err(|e| crate::OracleError::Model(e.to_string()))?;
        Ok(Self {
            citl,
            session_id: 0,
            error_associations: HashMap::new(),
        })
    }

    /// Record a successful transpilation session
    pub fn record_success(
        &mut self,
        file: &str,
        decisions: Vec<TranspilerDecisionRecord>,
    ) -> Result<(), crate::OracleError> {
        self.session_id += 1;
        let traces = self.decisions_to_traces(file, &decisions);
        self.citl
            .ingest_session(traces, CompilationOutcome::success(), None)
            .map_err(|e| crate::OracleError::Model(e.to_string()))
    }

    /// Record a failed transpilation session
    pub fn record_failure(
        &mut self,
        file: &str,
        decisions: Vec<TranspilerDecisionRecord>,
        error_codes: Vec<String>,
        error_messages: Vec<String>,
    ) -> Result<(), crate::OracleError> {
        self.session_id += 1;
        let traces = self.decisions_to_traces(file, &decisions);

        for code in &error_codes {
            let entry = self.error_associations.entry(code.clone()).or_default();
            for decision in &decisions {
                if !entry.contains(&decision.decision_type) {
                    entry.push(decision.decision_type);
                }
            }
        }

        let outcome = CompilationOutcome::failure(error_codes, vec![], error_messages);
        self.citl
            .ingest_session(traces, outcome, None)
            .map_err(|e| crate::OracleError::Model(e.to_string()))
    }

    fn decisions_to_traces(
        &self,
        file: &str,
        decisions: &[TranspilerDecisionRecord],
    ) -> Vec<DecisionTrace> {
        decisions
            .iter()
            .enumerate()
            .map(|(i, d)| d.to_trace(&format!("s{}_d{}", self.session_id, i), file))
            .collect()
    }

    /// Run Tarantula analysis
    #[must_use]
    pub fn analyze(&self) -> TarantulaResult {
        let entrenar_stats = self.citl.decision_stats();
        let total_success = self.citl.success_count();
        let total_failure = self.citl.failure_count();
        let mut stats: HashMap<TranspilerDecision, DecisionStats> = HashMap::new();
        let mut suspicious: Vec<SuspiciousTranspilerDecision> = Vec::new();

        for (decision_str, decision_stats) in entrenar_stats {
            if let Some(decision_type) = TranspilerDecision::parse(decision_str) {
                stats.insert(decision_type, decision_stats.clone());
                let score = decision_stats.tarantula_score();
                let failure_rate = if total_failure > 0 {
                    decision_stats.fail_count as f32 / total_failure as f32
                } else {
                    0.0
                };
                let associated_errors: Vec<String> = self
                    .error_associations
                    .iter()
                    .filter(|(_, decisions)| decisions.contains(&decision_type))
                    .map(|(code, _)| code.clone())
                    .collect();

                suspicious.push(SuspiciousTranspilerDecision {
                    decision_type,
                    suspiciousness: score,
                    fail_count: decision_stats.fail_count,
                    success_count: decision_stats.success_count,
                    associated_errors,
                    priority: FixPriority::from_failure_rate(failure_rate),
                });
            }
        }

        suspicious.sort_by(|a, b| {
            b.suspiciousness
                .partial_cmp(&a.suspiciousness)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        TarantulaResult {
            suspicious,
            total_success,
            total_failure,
            stats,
        }
    }

    /// Get session counts (success, failure)
    #[must_use]
    pub fn session_counts(&self) -> (usize, usize) {
        (self.citl.success_count(), self.citl.failure_count())
    }
}

impl std::fmt::Debug for TarantulaAnalyzer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TarantulaAnalyzer")
            .field("sessions", &self.session_id)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpiler_decision_roundtrip() {
        for decision in TranspilerDecision::all() {
            assert_eq!(
                TranspilerDecision::parse(decision.as_str()),
                Some(*decision)
            );
        }
    }

    #[test]
    fn test_fix_priority_from_failure_rate() {
        assert_eq!(FixPriority::from_failure_rate(0.25), FixPriority::Critical);
        assert_eq!(FixPriority::from_failure_rate(0.15), FixPriority::High);
        assert_eq!(FixPriority::from_failure_rate(0.07), FixPriority::Medium);
        assert_eq!(FixPriority::from_failure_rate(0.03), FixPriority::Low);
    }

    #[test]
    fn test_analyzer_new() {
        let analyzer = TarantulaAnalyzer::new().unwrap();
        assert_eq!(analyzer.session_counts(), (0, 0));
    }

    #[test]
    fn test_analyzer_record_success() {
        let mut analyzer = TarantulaAnalyzer::new().unwrap();
        let decisions = vec![TranspilerDecisionRecord::new(
            TranspilerDecision::TypeInference,
            "Inferred i32",
        )];
        analyzer.record_success("test.py", decisions).unwrap();
        assert_eq!(analyzer.session_counts(), (1, 0));
    }

    #[test]
    fn test_analyzer_record_failure() {
        let mut analyzer = TarantulaAnalyzer::new().unwrap();
        let decisions = vec![TranspilerDecisionRecord::new(
            TranspilerDecision::TypeInference,
            "wrong",
        )];
        analyzer
            .record_failure(
                "test.py",
                decisions,
                vec!["E0308".into()],
                vec!["mismatch".into()],
            )
            .unwrap();
        assert_eq!(analyzer.session_counts(), (0, 1));
    }

    #[test]
    fn test_analyzer_tarantula_analysis() {
        let mut analyzer = TarantulaAnalyzer::new().unwrap();

        // Record successes with TypeInference
        for _ in 0..3 {
            analyzer
                .record_success(
                    "good.py",
                    vec![TranspilerDecisionRecord::new(
                        TranspilerDecision::TypeInference,
                        "good",
                    )],
                )
                .unwrap();
        }

        // Record failures with ModuleMapping
        for _ in 0..5 {
            analyzer
                .record_failure(
                    "bad.py",
                    vec![TranspilerDecisionRecord::new(
                        TranspilerDecision::ModuleMapping,
                        "bad",
                    )],
                    vec!["E0433".into()],
                    vec![],
                )
                .unwrap();
        }

        let result = analyzer.analyze();
        assert_eq!(result.total_success, 3);
        assert_eq!(result.total_failure, 5);

        // ModuleMapping should be more suspicious than TypeInference
        let module_score = result.score(TranspilerDecision::ModuleMapping);
        let type_score = result.score(TranspilerDecision::TypeInference);
        assert!(module_score.unwrap() > type_score.unwrap());
    }

    #[test]
    fn test_tarantula_result_top() {
        let mut analyzer = TarantulaAnalyzer::new().unwrap();

        for _ in 0..5 {
            analyzer
                .record_failure(
                    "test.py",
                    vec![
                        TranspilerDecisionRecord::new(TranspilerDecision::ModuleMapping, "a"),
                        TranspilerDecisionRecord::new(TranspilerDecision::TypeInference, "b"),
                    ],
                    vec!["E0001".to_string()],
                    vec![],
                )
                .unwrap();
        }

        let result = analyzer.analyze();
        let top2 = result.top(2);
        assert_eq!(top2.len(), 2);
    }

    // ============================================================
    // Tarantula Scoring Formula Tests
    // ============================================================
    //
    // The Tarantula formula is:
    //   suspiciousness = (failed(e) / total_failed) /
    //                    ((failed(e) / total_failed) + (passed(e) / total_passed))
    //
    // Score ranges from 0.0 (never fails) to 1.0 (always fails)
    // ============================================================

    #[test]
    fn test_tarantula_all_pass_zero_suspiciousness() {
        // Edge case: When a decision ONLY appears in successful sessions,
        // its suspiciousness should be 0.0 (or very low)
        let mut analyzer = TarantulaAnalyzer::new().unwrap();

        // Record 10 successful sessions with TypeInference
        for i in 0..10 {
            analyzer
                .record_success(
                    &format!("success_{}.py", i),
                    vec![TranspilerDecisionRecord::new(
                        TranspilerDecision::TypeInference,
                        "Inferred correct type",
                    )],
                )
                .unwrap();
        }

        let result = analyzer.analyze();
        assert_eq!(result.total_success, 10);
        assert_eq!(result.total_failure, 0);

        // With no failures, the score should be 0.0 or not present
        let score = result.score(TranspilerDecision::TypeInference);
        if let Some(s) = score {
            assert!(
                s <= 0.01,
                "Expected near-zero suspiciousness for all-pass, got {}",
                s
            );
        }
        // None is also acceptable - decision not tracked as suspicious
    }

    #[test]
    fn test_tarantula_all_fail_max_suspiciousness() {
        // Edge case: When a decision ONLY appears in failed sessions,
        // its suspiciousness should be 1.0 (maximum)
        let mut analyzer = TarantulaAnalyzer::new().unwrap();

        // Record 10 failed sessions with ModuleMapping
        for i in 0..10 {
            analyzer
                .record_failure(
                    &format!("failure_{}.py", i),
                    vec![TranspilerDecisionRecord::new(
                        TranspilerDecision::ModuleMapping,
                        "Failed module import",
                    )],
                    vec!["E0433".to_string()],
                    vec!["unresolved import".to_string()],
                )
                .unwrap();
        }

        let result = analyzer.analyze();
        assert_eq!(result.total_success, 0);
        assert_eq!(result.total_failure, 10);

        // With only failures, the score should be 1.0 (all decisions in failures)
        let score = result.score(TranspilerDecision::ModuleMapping);
        assert!(score.is_some(), "ModuleMapping should have a score");
        assert!(
            (score.unwrap() - 1.0).abs() < 0.01,
            "Expected max suspiciousness (1.0) for all-fail, got {}",
            score.unwrap()
        );
    }

    #[test]
    fn test_tarantula_mixed_results_suspiciousness_ordering() {
        // Test: Decisions that appear more in failures should have higher scores
        let mut analyzer = TarantulaAnalyzer::new().unwrap();

        // Decision A: 8 failures, 2 successes (high suspiciousness)
        for i in 0..8 {
            analyzer
                .record_failure(
                    &format!("fail_a_{}.py", i),
                    vec![TranspilerDecisionRecord::new(
                        TranspilerDecision::OwnershipInference,
                        "Failed ownership",
                    )],
                    vec!["E0382".to_string()],
                    vec![],
                )
                .unwrap();
        }
        for i in 0..2 {
            analyzer
                .record_success(
                    &format!("success_a_{}.py", i),
                    vec![TranspilerDecisionRecord::new(
                        TranspilerDecision::OwnershipInference,
                        "Correct ownership",
                    )],
                )
                .unwrap();
        }

        // Decision B: 2 failures, 8 successes (low suspiciousness)
        for i in 0..2 {
            analyzer
                .record_failure(
                    &format!("fail_b_{}.py", i),
                    vec![TranspilerDecisionRecord::new(
                        TranspilerDecision::LifetimeInference,
                        "Failed lifetime",
                    )],
                    vec!["E0106".to_string()],
                    vec![],
                )
                .unwrap();
        }
        for i in 0..8 {
            analyzer
                .record_success(
                    &format!("success_b_{}.py", i),
                    vec![TranspilerDecisionRecord::new(
                        TranspilerDecision::LifetimeInference,
                        "Correct lifetime",
                    )],
                )
                .unwrap();
        }

        let result = analyzer.analyze();
        assert_eq!(result.total_failure, 10);
        assert_eq!(result.total_success, 10);

        let ownership_score = result
            .score(TranspilerDecision::OwnershipInference)
            .unwrap();
        let lifetime_score = result.score(TranspilerDecision::LifetimeInference).unwrap();

        // OwnershipInference (80% fail) should be MORE suspicious than
        // LifetimeInference (20% fail)
        assert!(
            ownership_score > lifetime_score,
            "OwnershipInference ({}) should be more suspicious than LifetimeInference ({})",
            ownership_score,
            lifetime_score
        );
    }

    #[test]
    fn test_tarantula_equal_distribution_neutral_score() {
        // Test: When a decision appears equally in pass and fail,
        // suspiciousness should be around 0.5
        let mut analyzer = TarantulaAnalyzer::new().unwrap();

        // Decision appears in 5 failures and 5 successes
        for i in 0..5 {
            analyzer
                .record_failure(
                    &format!("fail_{}.py", i),
                    vec![TranspilerDecisionRecord::new(
                        TranspilerDecision::MethodTranslation,
                        "Method translation",
                    )],
                    vec!["E0599".to_string()],
                    vec![],
                )
                .unwrap();
        }
        for i in 0..5 {
            analyzer
                .record_success(
                    &format!("success_{}.py", i),
                    vec![TranspilerDecisionRecord::new(
                        TranspilerDecision::MethodTranslation,
                        "Method translation",
                    )],
                )
                .unwrap();
        }

        let result = analyzer.analyze();
        let score = result.score(TranspilerDecision::MethodTranslation).unwrap();

        // Equal distribution should yield ~0.5 suspiciousness
        assert!(
            (0.4..=0.6).contains(&score),
            "Expected neutral suspiciousness (~0.5) for equal distribution, got {}",
            score
        );
    }

    #[test]
    fn test_tarantula_is_suspicious_threshold() {
        let mut analyzer = TarantulaAnalyzer::new().unwrap();

        // Create a highly suspicious decision (all failures)
        for _ in 0..5 {
            analyzer
                .record_failure(
                    "fail.py",
                    vec![TranspilerDecisionRecord::new(
                        TranspilerDecision::ErrorHandling,
                        "Bad error handling",
                    )],
                    vec!["E0277".to_string()],
                    vec![],
                )
                .unwrap();
        }

        // Create a low-suspicion decision (all successes)
        for _ in 0..5 {
            analyzer
                .record_success(
                    "success.py",
                    vec![TranspilerDecisionRecord::new(
                        TranspilerDecision::ContainerMapping,
                        "Good container",
                    )],
                )
                .unwrap();
        }

        let result = analyzer.analyze();

        // ErrorHandling should be suspicious at threshold 0.5
        assert!(
            result.is_suspicious(TranspilerDecision::ErrorHandling, 0.5),
            "ErrorHandling should be suspicious at threshold 0.5"
        );

        // ContainerMapping should NOT be suspicious at threshold 0.5
        assert!(
            !result.is_suspicious(TranspilerDecision::ContainerMapping, 0.5),
            "ContainerMapping should not be suspicious at threshold 0.5"
        );
    }

    #[test]
    fn test_tarantula_multiple_decisions_per_session() {
        // Test: Multiple decisions in the same session should each get tracked
        let mut analyzer = TarantulaAnalyzer::new().unwrap();

        // Session with multiple decisions - some will be in both pass and fail
        for _ in 0..3 {
            analyzer
                .record_failure(
                    "complex_fail.py",
                    vec![
                        TranspilerDecisionRecord::new(
                            TranspilerDecision::TypeInference,
                            "Type inference",
                        ),
                        TranspilerDecisionRecord::new(
                            TranspilerDecision::IteratorTransform,
                            "Iterator transform",
                        ),
                        TranspilerDecisionRecord::new(
                            TranspilerDecision::FunctionSignature,
                            "Function signature",
                        ),
                    ],
                    vec!["E0308".to_string()],
                    vec![],
                )
                .unwrap();
        }

        for _ in 0..3 {
            analyzer
                .record_success(
                    "complex_success.py",
                    vec![
                        TranspilerDecisionRecord::new(
                            TranspilerDecision::TypeInference,
                            "Type inference",
                        ),
                        TranspilerDecisionRecord::new(
                            TranspilerDecision::ContainerMapping,
                            "Container mapping",
                        ),
                    ],
                )
                .unwrap();
        }

        let result = analyzer.analyze();

        // TypeInference appears in both pass and fail - should have moderate score
        let type_score = result.score(TranspilerDecision::TypeInference).unwrap();

        // IteratorTransform only in failures - should have high score
        let iter_score = result.score(TranspilerDecision::IteratorTransform).unwrap();

        // ContainerMapping only in successes - should have low score
        let container_score = result.score(TranspilerDecision::ContainerMapping);

        assert!(
            iter_score > type_score,
            "IteratorTransform (fail-only) should be more suspicious than TypeInference (mixed)"
        );

        if let Some(cs) = container_score {
            assert!(
                cs < type_score,
                "ContainerMapping (success-only) should be less suspicious than TypeInference"
            );
        }
    }

    #[test]
    fn test_tarantula_error_associations() {
        // Test: Error codes should be associated with decisions in failures
        let mut analyzer = TarantulaAnalyzer::new().unwrap();

        analyzer
            .record_failure(
                "test.py",
                vec![
                    TranspilerDecisionRecord::new(TranspilerDecision::TypeInference, "Bad type"),
                    TranspilerDecisionRecord::new(TranspilerDecision::ModuleMapping, "Bad module"),
                ],
                vec!["E0308".to_string(), "E0433".to_string()],
                vec!["type mismatch".to_string()],
            )
            .unwrap();

        let result = analyzer.analyze();

        // Find TypeInference in results and check error associations
        let type_decision = result
            .suspicious
            .iter()
            .find(|s| s.decision_type == TranspilerDecision::TypeInference);

        assert!(type_decision.is_some());
        let type_decision = type_decision.unwrap();
        assert!(
            type_decision
                .associated_errors
                .contains(&"E0308".to_string())
                || type_decision
                    .associated_errors
                    .contains(&"E0433".to_string()),
            "TypeInference should have associated error codes"
        );
    }

    #[test]
    fn test_tarantula_result_sorted_by_suspiciousness() {
        // Test: Results should be sorted by suspiciousness (highest first)
        let mut analyzer = TarantulaAnalyzer::new().unwrap();

        // Create decisions with varying suspiciousness
        // High suspiciousness: 9 failures, 1 success
        for _ in 0..9 {
            analyzer
                .record_failure(
                    "fail.py",
                    vec![TranspilerDecisionRecord::new(
                        TranspilerDecision::ImportGeneration,
                        "Import",
                    )],
                    vec!["E0432".to_string()],
                    vec![],
                )
                .unwrap();
        }
        analyzer
            .record_success(
                "success.py",
                vec![TranspilerDecisionRecord::new(
                    TranspilerDecision::ImportGeneration,
                    "Import",
                )],
            )
            .unwrap();

        // Medium suspiciousness: 5 failures, 5 successes
        for _ in 0..5 {
            analyzer
                .record_failure(
                    "fail.py",
                    vec![TranspilerDecisionRecord::new(
                        TranspilerDecision::StringFormatting,
                        "Format",
                    )],
                    vec!["E0277".to_string()],
                    vec![],
                )
                .unwrap();
        }
        for _ in 0..5 {
            analyzer
                .record_success(
                    "success.py",
                    vec![TranspilerDecisionRecord::new(
                        TranspilerDecision::StringFormatting,
                        "Format",
                    )],
                )
                .unwrap();
        }

        // Low suspiciousness: 1 failure, 9 successes
        analyzer
            .record_failure(
                "fail.py",
                vec![TranspilerDecisionRecord::new(
                    TranspilerDecision::NumericType,
                    "Numeric",
                )],
                vec!["E0308".to_string()],
                vec![],
            )
            .unwrap();
        for _ in 0..9 {
            analyzer
                .record_success(
                    "success.py",
                    vec![TranspilerDecisionRecord::new(
                        TranspilerDecision::NumericType,
                        "Numeric",
                    )],
                )
                .unwrap();
        }

        let result = analyzer.analyze();

        // Verify sorting: suspicious[0] should have highest score
        let scores: Vec<f32> = result.suspicious.iter().map(|s| s.suspiciousness).collect();

        for i in 1..scores.len() {
            assert!(
                scores[i - 1] >= scores[i],
                "Results not sorted: {} < {} at index {}",
                scores[i - 1],
                scores[i],
                i
            );
        }
    }

    #[test]
    fn test_tarantula_priority_assignment() {
        // Test: FixPriority should be assigned based on failure rate
        let mut analyzer = TarantulaAnalyzer::new().unwrap();

        // Create 100 total failures for clear percentage calculation
        // Critical: >20% failure rate
        for _ in 0..25 {
            analyzer
                .record_failure(
                    "critical.py",
                    vec![TranspilerDecisionRecord::new(
                        TranspilerDecision::ControlFlow,
                        "Control flow",
                    )],
                    vec!["E0001".to_string()],
                    vec![],
                )
                .unwrap();
        }

        // Fill remaining with other decisions to reach 100 failures
        for _ in 0..75 {
            analyzer
                .record_failure(
                    "other.py",
                    vec![TranspilerDecisionRecord::new(
                        TranspilerDecision::BooleanExpression,
                        "Boolean",
                    )],
                    vec!["E0002".to_string()],
                    vec![],
                )
                .unwrap();
        }

        let result = analyzer.analyze();

        let control_flow = result
            .suspicious
            .iter()
            .find(|s| s.decision_type == TranspilerDecision::ControlFlow);

        assert!(control_flow.is_some());
        assert_eq!(
            control_flow.unwrap().priority,
            FixPriority::Critical,
            "ControlFlow with 25% failure rate should be Critical priority"
        );
    }

    #[test]
    fn test_transpiler_decision_record_builder() {
        // Test the builder pattern for TranspilerDecisionRecord
        let record =
            TranspilerDecisionRecord::new(TranspilerDecision::TypeInference, "Inferred i32")
                .with_python_line(42)
                .with_rust_snippet("let x: i32 = 0;");

        assert_eq!(record.decision_type, TranspilerDecision::TypeInference);
        assert_eq!(record.description, "Inferred i32");
        assert_eq!(record.python_line, Some(42));
        assert_eq!(record.rust_snippet, Some("let x: i32 = 0;".to_string()));
    }

    #[test]
    fn test_transpiler_decision_record_long_snippet_truncation() {
        // Test that long rust snippets are truncated
        let long_snippet = "x".repeat(150);
        let record = TranspilerDecisionRecord::new(TranspilerDecision::TypeInference, "test")
            .with_rust_snippet(&long_snippet);

        assert!(record.rust_snippet.is_some());
        let snippet = record.rust_snippet.unwrap();
        assert!(snippet.len() <= 103); // 97 chars + "..."
        assert!(snippet.ends_with("..."));
    }

    #[test]
    fn test_transpiler_decision_display() {
        assert_eq!(
            format!("{}", TranspilerDecision::TypeInference),
            "type_inference"
        );
        assert_eq!(
            format!("{}", TranspilerDecision::OwnershipInference),
            "ownership_inference"
        );
    }

    #[test]
    fn test_transpiler_decision_parse_invalid() {
        assert_eq!(TranspilerDecision::parse("invalid_decision"), None);
        assert_eq!(TranspilerDecision::parse(""), None);
        assert_eq!(TranspilerDecision::parse("TYPE_INFERENCE"), None); // Case sensitive
    }
}
