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
}
