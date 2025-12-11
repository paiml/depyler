//! Knowledge Distiller (DEPYLER-0925)
//!
//! Promotes high-confidence Oracle patterns to hardcoded transpiler rules.
//! This implements knowledge distillation from the pattern store to
//! permanent transpiler improvements.
//!
//! ## Graduation Criteria
//!
//! A pattern graduates when it meets ALL thresholds:
//! - Confidence >= 0.95
//! - Usage count >= 50
//! - Success rate >= 0.99
//!
//! Reference: Hinton et al. (2015) - Distilling the Knowledge in a Neural Network

use super::pattern_store::{PatternStore, TranspilationPattern};

/// Criteria for graduating a pattern to a hardcoded rule
#[derive(Debug, Clone)]
pub struct GraduationCriteria {
    pub min_confidence: f32,
    pub min_usage_count: u32,
    pub min_success_rate: f32,
    pub max_complexity: u32,
}

impl Default for GraduationCriteria {
    fn default() -> Self {
        Self {
            min_confidence: 0.95,
            min_usage_count: 50,
            min_success_rate: 0.99,
            max_complexity: 10,
        }
    }
}

/// Knowledge distiller for pattern graduation
pub struct KnowledgeDistiller {
    criteria: GraduationCriteria,
}

impl KnowledgeDistiller {
    /// Create a new distiller with given criteria
    pub fn new(criteria: GraduationCriteria) -> Self {
        Self { criteria }
    }

    /// Check if a pattern is ready for graduation
    pub fn ready_for_graduation(&self, pattern: &TranspilationPattern) -> bool {
        pattern.confidence >= self.criteria.min_confidence
            && pattern.usage_count >= self.criteria.min_usage_count
            && pattern.success_rate >= self.criteria.min_success_rate
    }

    /// Generate Rust code for a hardcoded rule
    ///
    /// Returns a function definition that can be added to direct_rules.rs
    pub fn generate_rule(&self, pattern: &TranspilationPattern) -> String {
        // Convert ID to valid Rust identifier (snake_case)
        let fn_name = pattern.id.replace(['-', '.'], "_");

        format!(
            r#"
// Auto-generated from pattern {} (confidence: {:.2}, uses: {})
// Original Python: {}
fn handle_pattern_{}(ctx: &mut CodegenContext, expr: &HirExpr) -> TokenStream {{
    // Generated output: {}
    quote! {{ {} }}
}}
"#,
            pattern.id,
            pattern.confidence,
            pattern.usage_count,
            pattern.python_pattern.lines().next().unwrap_or(""),
            fn_name,
            pattern.rust_output,
            pattern.rust_output
        )
    }

    /// Find all patterns ready for graduation
    pub fn find_graduation_candidates<'a>(
        &self,
        store: &'a PatternStore,
    ) -> Vec<&'a TranspilationPattern> {
        store
            .patterns()
            .filter(|p| self.ready_for_graduation(p))
            .collect()
    }

    /// Generate a report of graduation candidates
    pub fn graduation_report(&self, store: &PatternStore) -> String {
        let candidates = self.find_graduation_candidates(store);

        if candidates.is_empty() {
            return "No patterns ready for graduation.".to_string();
        }

        let mut report = format!("=== Graduation Candidates ({}) ===\n\n", candidates.len());

        for pattern in candidates {
            report.push_str(&format!(
                "Pattern: {}\n  Confidence: {:.2}%\n  Usage: {}\n  Success Rate: {:.2}%\n  Error Prevented: {}\n\n",
                pattern.id,
                pattern.confidence * 100.0,
                pattern.usage_count,
                pattern.success_rate * 100.0,
                pattern.error_prevented
            ));
        }

        report
    }
}

impl Default for KnowledgeDistiller {
    fn default() -> Self {
        Self::new(GraduationCriteria::default())
    }
}
