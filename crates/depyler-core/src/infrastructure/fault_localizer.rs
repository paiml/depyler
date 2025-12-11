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
