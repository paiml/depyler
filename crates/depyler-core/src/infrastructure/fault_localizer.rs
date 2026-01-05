//! Fault Localizer using Tarantula Algorithm (DEPYLER-0925)
//!
//! Implements spectrum-based fault localization to identify suspicious
//! transpiler decisions that correlate with compilation failures.
//!
//! ## Algorithm
//!
//! Tarantula suspiciousness formula (Jones & Harrold 2005):
//! ```text
//! suspiciousness = (failed/total_failed) / (failed/total_failed + passed/total_passed)
//! ```
//!
//! Higher suspiciousness indicates the decision is more likely to be faulty.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Source location in original Python file
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SourceLocation {
    pub file: String,
    pub line: u32,
    pub column: u32,
}

/// Types of decisions the transpiler can make
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecisionType {
    /// Type inference decision
    TypeInference {
        inferred: String,
        constraints: Vec<String>,
    },
    /// Ownership/borrowing strategy
    OwnershipChoice {
        strategy: String,
        reason: String,
    },
    /// Library mapping (Python API → Rust API)
    LibraryMapping {
        python_api: String,
        rust_api: String,
    },
    /// Lifetime elision/annotation
    LifetimeElision {
        pattern: String,
    },
    /// Trait bound selection
    TraitBoundSelection {
        trait_name: String,
        impl_strategy: String,
    },
}

/// A single transpiler decision captured during code generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranspilerDecision {
    pub id: u64,
    pub location: SourceLocation,
    pub decision_type: DecisionType,
    pub input_context: String,
    pub output_generated: String,
    pub confidence: f32,
    pub timestamp_ns: u64,
}

/// Tarantula-style fault localizer for transpiler decisions
pub struct FaultLocalizer {
    decisions: HashMap<u64, TranspilerDecision>,
    pass_count: HashMap<u64, u32>,
    fail_count: HashMap<u64, u32>,
    total_passed: u32,
    total_failed: u32,
}

impl FaultLocalizer {
    /// Create a new fault localizer
    pub fn new() -> Self {
        Self {
            decisions: HashMap::new(),
            pass_count: HashMap::new(),
            fail_count: HashMap::new(),
            total_passed: 0,
            total_failed: 0,
        }
    }

    /// Record a transpiler decision
    pub fn record_decision(&mut self, decision: TranspilerDecision) {
        self.decisions.insert(decision.id, decision);
    }

    /// Get decision count
    pub fn decision_count(&self) -> usize {
        self.decisions.len()
    }

    /// Get a decision by ID
    pub fn get_decision(&self, id: u64) -> Option<&TranspilerDecision> {
        self.decisions.get(&id)
    }

    /// Record that a decision appeared in a passing test
    pub fn record_pass(&mut self, decision_id: u64) {
        *self.pass_count.entry(decision_id).or_insert(0) += 1;
    }

    /// Record that a decision appeared in a failing test
    pub fn record_fail(&mut self, decision_id: u64) {
        *self.fail_count.entry(decision_id).or_insert(0) += 1;
    }

    /// Set total pass/fail counts
    pub fn set_totals(&mut self, failed: u32, passed: u32) {
        self.total_failed = failed;
        self.total_passed = passed;
    }

    /// Calculate Tarantula suspiciousness for a decision
    ///
    /// Formula: (failed/total_failed) / (failed/total_failed + passed/total_passed)
    pub fn suspiciousness(&self, decision_id: u64) -> f64 {
        if self.total_failed == 0 {
            return 0.0;
        }

        let failed = *self.fail_count.get(&decision_id).unwrap_or(&0) as f64;
        let passed = *self.pass_count.get(&decision_id).unwrap_or(&0) as f64;
        let total_failed = self.total_failed as f64;
        let total_passed = self.total_passed as f64;

        let fail_ratio = failed / total_failed;
        let pass_ratio = if total_passed > 0.0 {
            passed / total_passed
        } else {
            0.0
        };

        fail_ratio / (fail_ratio + pass_ratio + f64::EPSILON)
    }

    /// Rank all recorded decisions by suspiciousness
    pub fn rank_decisions(&self) -> Vec<(u64, f64)> {
        let mut all_ids: Vec<u64> = self
            .fail_count
            .keys()
            .chain(self.pass_count.keys())
            .copied()
            .collect();
        all_ids.sort();
        all_ids.dedup();

        let mut ranked: Vec<_> = all_ids
            .into_iter()
            .map(|id| (id, self.suspiciousness(id)))
            .collect();

        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        ranked
    }
}

impl Default for FaultLocalizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_location(line: u32) -> SourceLocation {
        SourceLocation {
            file: "test.py".to_string(),
            line,
            column: 0,
        }
    }

    fn make_decision(id: u64, line: u32) -> TranspilerDecision {
        TranspilerDecision {
            id,
            location: make_location(line),
            decision_type: DecisionType::TypeInference {
                inferred: "i32".to_string(),
                constraints: vec![],
            },
            input_context: "x = 1".to_string(),
            output_generated: "let x: i32 = 1;".to_string(),
            confidence: 0.9,
            timestamp_ns: 0,
        }
    }

    // ========================================================================
    // SourceLocation tests
    // ========================================================================

    #[test]
    fn test_source_location_new() {
        let loc = SourceLocation {
            file: "main.py".to_string(),
            line: 42,
            column: 10,
        };
        assert_eq!(loc.file, "main.py");
        assert_eq!(loc.line, 42);
        assert_eq!(loc.column, 10);
    }

    #[test]
    fn test_source_location_clone() {
        let loc = make_location(100);
        let cloned = loc.clone();
        assert_eq!(loc, cloned);
    }

    #[test]
    fn test_source_location_debug() {
        let loc = make_location(50);
        let debug_str = format!("{:?}", loc);
        assert!(debug_str.contains("SourceLocation"));
        assert!(debug_str.contains("50"));
    }

    #[test]
    fn test_source_location_serialize() {
        let loc = make_location(25);
        let json = serde_json::to_string(&loc).unwrap();
        assert!(json.contains("test.py"));
        assert!(json.contains("25"));
    }

    // ========================================================================
    // DecisionType tests
    // ========================================================================

    #[test]
    fn test_decision_type_type_inference() {
        let dt = DecisionType::TypeInference {
            inferred: "String".to_string(),
            constraints: vec!["FromStr".to_string()],
        };
        assert!(matches!(dt, DecisionType::TypeInference { .. }));
    }

    #[test]
    fn test_decision_type_ownership_choice() {
        let dt = DecisionType::OwnershipChoice {
            strategy: "clone".to_string(),
            reason: "used after move".to_string(),
        };
        assert!(matches!(dt, DecisionType::OwnershipChoice { .. }));
    }

    #[test]
    fn test_decision_type_library_mapping() {
        let dt = DecisionType::LibraryMapping {
            python_api: "json.dumps".to_string(),
            rust_api: "serde_json::to_string".to_string(),
        };
        assert!(matches!(dt, DecisionType::LibraryMapping { .. }));
    }

    #[test]
    fn test_decision_type_lifetime_elision() {
        let dt = DecisionType::LifetimeElision {
            pattern: "input-output".to_string(),
        };
        assert!(matches!(dt, DecisionType::LifetimeElision { .. }));
    }

    #[test]
    fn test_decision_type_trait_bound_selection() {
        let dt = DecisionType::TraitBoundSelection {
            trait_name: "Display".to_string(),
            impl_strategy: "derive".to_string(),
        };
        assert!(matches!(dt, DecisionType::TraitBoundSelection { .. }));
    }

    #[test]
    fn test_decision_type_clone() {
        let dt = DecisionType::TypeInference {
            inferred: "f64".to_string(),
            constraints: vec![],
        };
        let cloned = dt.clone();
        assert!(matches!(cloned, DecisionType::TypeInference { .. }));
    }

    // ========================================================================
    // TranspilerDecision tests
    // ========================================================================

    #[test]
    fn test_transpiler_decision_new() {
        let decision = make_decision(1, 10);
        assert_eq!(decision.id, 1);
        assert_eq!(decision.location.line, 10);
        assert_eq!(decision.confidence, 0.9);
    }

    #[test]
    fn test_transpiler_decision_clone() {
        let decision = make_decision(42, 100);
        let cloned = decision.clone();
        assert_eq!(decision.id, cloned.id);
        assert_eq!(decision.location.line, cloned.location.line);
    }

    #[test]
    fn test_transpiler_decision_debug() {
        let decision = make_decision(99, 200);
        let debug_str = format!("{:?}", decision);
        assert!(debug_str.contains("TranspilerDecision"));
        assert!(debug_str.contains("99"));
    }

    #[test]
    fn test_transpiler_decision_serialize() {
        let decision = make_decision(5, 50);
        let json = serde_json::to_string(&decision).unwrap();
        let deserialized: TranspilerDecision = serde_json::from_str(&json).unwrap();
        assert_eq!(decision.id, deserialized.id);
    }

    // ========================================================================
    // FaultLocalizer tests
    // ========================================================================

    #[test]
    fn test_fault_localizer_new() {
        let fl = FaultLocalizer::new();
        assert_eq!(fl.decision_count(), 0);
    }

    #[test]
    fn test_fault_localizer_default() {
        let fl = FaultLocalizer::default();
        assert_eq!(fl.decision_count(), 0);
    }

    #[test]
    fn test_record_decision() {
        let mut fl = FaultLocalizer::new();
        fl.record_decision(make_decision(1, 10));
        fl.record_decision(make_decision(2, 20));
        assert_eq!(fl.decision_count(), 2);
    }

    #[test]
    fn test_get_decision() {
        let mut fl = FaultLocalizer::new();
        fl.record_decision(make_decision(42, 100));

        let decision = fl.get_decision(42);
        assert!(decision.is_some());
        assert_eq!(decision.unwrap().location.line, 100);

        let missing = fl.get_decision(999);
        assert!(missing.is_none());
    }

    #[test]
    fn test_record_pass() {
        let mut fl = FaultLocalizer::new();
        fl.record_pass(1);
        fl.record_pass(1);
        fl.record_pass(2);

        // Can't directly access pass_count, but suspiciousness should reflect it
        fl.set_totals(1, 3);
        // Decision 1 has 2 passes, so lower suspiciousness
        let susp = fl.suspiciousness(1);
        assert!(susp < 0.5);
    }

    #[test]
    fn test_record_fail() {
        let mut fl = FaultLocalizer::new();
        fl.record_fail(1);
        fl.record_fail(1);
        fl.set_totals(2, 0);

        let susp = fl.suspiciousness(1);
        assert!(susp > 0.9); // High suspiciousness (all failures)
    }

    #[test]
    fn test_set_totals() {
        let mut fl = FaultLocalizer::new();
        fl.set_totals(5, 10);

        // With 0 recorded passes/fails for any decision, suspiciousness should be 0
        let susp = fl.suspiciousness(999);
        assert_eq!(susp, 0.0);
    }

    // ========================================================================
    // Tarantula suspiciousness tests
    // ========================================================================

    #[test]
    fn test_suspiciousness_no_failures() {
        let fl = FaultLocalizer::new();
        // total_failed = 0, should return 0
        let susp = fl.suspiciousness(1);
        assert_eq!(susp, 0.0);
    }

    #[test]
    fn test_suspiciousness_only_failures() {
        let mut fl = FaultLocalizer::new();
        fl.record_fail(1);
        fl.record_fail(1);
        fl.set_totals(2, 0);

        let susp = fl.suspiciousness(1);
        // fail_ratio = 2/2 = 1.0, pass_ratio = 0
        // susp = 1.0 / (1.0 + 0 + epsilon) ≈ 1.0
        assert!(susp > 0.99);
    }

    #[test]
    fn test_suspiciousness_only_passes() {
        let mut fl = FaultLocalizer::new();
        fl.record_pass(1);
        fl.record_pass(1);
        fl.set_totals(1, 2);

        let susp = fl.suspiciousness(1);
        // fail_ratio = 0/1 = 0, pass_ratio = 2/2 = 1.0
        // susp = 0 / (0 + 1.0 + epsilon) ≈ 0
        assert!(susp < 0.01);
    }

    #[test]
    fn test_suspiciousness_mixed() {
        let mut fl = FaultLocalizer::new();
        fl.record_fail(1);
        fl.record_pass(1);
        fl.set_totals(2, 2);

        let susp = fl.suspiciousness(1);
        // fail_ratio = 1/2 = 0.5, pass_ratio = 1/2 = 0.5
        // susp = 0.5 / (0.5 + 0.5 + epsilon) ≈ 0.5
        assert!(susp > 0.45 && susp < 0.55);
    }

    #[test]
    fn test_suspiciousness_high_fail_ratio() {
        let mut fl = FaultLocalizer::new();
        fl.record_fail(1);
        fl.record_fail(1);
        fl.record_pass(1);
        fl.set_totals(2, 10);

        let susp = fl.suspiciousness(1);
        // fail_ratio = 2/2 = 1.0, pass_ratio = 1/10 = 0.1
        // susp = 1.0 / (1.0 + 0.1) ≈ 0.909
        assert!(susp > 0.9);
    }

    #[test]
    fn test_suspiciousness_unknown_decision() {
        let mut fl = FaultLocalizer::new();
        fl.set_totals(5, 5);

        // Decision 999 was never recorded
        let susp = fl.suspiciousness(999);
        assert_eq!(susp, 0.0);
    }

    // ========================================================================
    // Rank decisions tests
    // ========================================================================

    #[test]
    fn test_rank_decisions_empty() {
        let fl = FaultLocalizer::new();
        let ranked = fl.rank_decisions();
        assert!(ranked.is_empty());
    }

    #[test]
    fn test_rank_decisions_single() {
        let mut fl = FaultLocalizer::new();
        fl.record_fail(1);
        fl.set_totals(1, 0);

        let ranked = fl.rank_decisions();
        assert_eq!(ranked.len(), 1);
        assert_eq!(ranked[0].0, 1);
    }

    #[test]
    fn test_rank_decisions_ordered_by_suspiciousness() {
        let mut fl = FaultLocalizer::new();

        // Decision 1: all failures (highest suspiciousness)
        fl.record_fail(1);
        fl.record_fail(1);

        // Decision 2: mixed (medium suspiciousness)
        fl.record_fail(2);
        fl.record_pass(2);

        // Decision 3: all passes (lowest suspiciousness)
        fl.record_pass(3);
        fl.record_pass(3);

        fl.set_totals(3, 3);

        let ranked = fl.rank_decisions();
        assert_eq!(ranked.len(), 3);

        // Most suspicious first
        assert_eq!(ranked[0].0, 1);
        assert!(ranked[0].1 > ranked[1].1);
        assert!(ranked[1].1 > ranked[2].1);
    }

    #[test]
    fn test_rank_decisions_deduplicates() {
        let mut fl = FaultLocalizer::new();
        fl.record_fail(1);
        fl.record_pass(1);
        fl.set_totals(1, 1);

        let ranked = fl.rank_decisions();
        // Decision 1 appears in both pass and fail, but should only appear once
        assert_eq!(ranked.len(), 1);
    }

    // ========================================================================
    // Integration tests
    // ========================================================================

    #[test]
    fn test_full_workflow() {
        let mut fl = FaultLocalizer::new();

        // Record some decisions
        fl.record_decision(make_decision(1, 10));
        fl.record_decision(make_decision(2, 20));
        fl.record_decision(make_decision(3, 30));

        // Simulate test runs
        // Test 1 failed: involved decisions 1, 2
        fl.record_fail(1);
        fl.record_fail(2);

        // Test 2 passed: involved decisions 2, 3
        fl.record_pass(2);
        fl.record_pass(3);

        // Test 3 passed: involved decision 3
        fl.record_pass(3);

        fl.set_totals(1, 2);

        // Decision 1 only appears in failures - most suspicious
        // Decision 2 appears in both - medium
        // Decision 3 only in passes - least suspicious
        let ranked = fl.rank_decisions();

        assert_eq!(ranked[0].0, 1); // Most suspicious
        assert_eq!(ranked[2].0, 3); // Least suspicious
    }

    #[test]
    fn test_complex_decision_types() {
        let mut fl = FaultLocalizer::new();

        // Test different decision types
        let d1 = TranspilerDecision {
            id: 1,
            location: make_location(10),
            decision_type: DecisionType::OwnershipChoice {
                strategy: "move".to_string(),
                reason: "transferred".to_string(),
            },
            input_context: "x".to_string(),
            output_generated: "x".to_string(),
            confidence: 0.8,
            timestamp_ns: 100,
        };

        let d2 = TranspilerDecision {
            id: 2,
            location: make_location(20),
            decision_type: DecisionType::LibraryMapping {
                python_api: "os.path.join".to_string(),
                rust_api: "Path::join".to_string(),
            },
            input_context: "os.path.join(a, b)".to_string(),
            output_generated: "Path::new(a).join(b)".to_string(),
            confidence: 0.95,
            timestamp_ns: 200,
        };

        fl.record_decision(d1);
        fl.record_decision(d2);

        assert_eq!(fl.decision_count(), 2);
        assert!(fl.get_decision(1).is_some());
        assert!(fl.get_decision(2).is_some());
    }
}
