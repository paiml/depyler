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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_pattern(confidence: f32, usage_count: u32, success_rate: f32) -> TranspilationPattern {
        TranspilationPattern {
            id: "test-pattern-001".to_string(),
            python_pattern: "def foo(x): return x + 1".to_string(),
            rust_output: "fn foo(x: i64) -> i64 { x + 1 }".to_string(),
            error_prevented: "E0308".to_string(),
            confidence,
            usage_count,
            success_rate,
            embedding: vec![0.1, 0.2, 0.3],
        }
    }

    #[test]
    fn test_graduation_criteria_default() {
        let criteria = GraduationCriteria::default();
        assert!((criteria.min_confidence - 0.95).abs() < f32::EPSILON);
        assert_eq!(criteria.min_usage_count, 50);
        assert!((criteria.min_success_rate - 0.99).abs() < f32::EPSILON);
        assert_eq!(criteria.max_complexity, 10);
    }

    #[test]
    fn test_graduation_criteria_custom() {
        let criteria = GraduationCriteria {
            min_confidence: 0.90,
            min_usage_count: 100,
            min_success_rate: 0.95,
            max_complexity: 5,
        };
        assert!((criteria.min_confidence - 0.90).abs() < f32::EPSILON);
        assert_eq!(criteria.min_usage_count, 100);
    }

    #[test]
    fn test_graduation_criteria_clone() {
        let criteria = GraduationCriteria::default();
        let cloned = criteria.clone();
        assert!((cloned.min_confidence - criteria.min_confidence).abs() < f32::EPSILON);
    }

    #[test]
    fn test_graduation_criteria_debug() {
        let criteria = GraduationCriteria::default();
        let debug = format!("{:?}", criteria);
        assert!(debug.contains("GraduationCriteria"));
        assert!(debug.contains("min_confidence"));
    }

    #[test]
    fn test_knowledge_distiller_new() {
        let criteria = GraduationCriteria::default();
        let distiller = KnowledgeDistiller::new(criteria);
        // Just verify it creates successfully
        assert!(distiller.criteria.min_confidence > 0.0);
    }

    #[test]
    fn test_knowledge_distiller_default() {
        let distiller = KnowledgeDistiller::default();
        assert!((distiller.criteria.min_confidence - 0.95).abs() < f32::EPSILON);
    }

    #[test]
    fn test_ready_for_graduation_all_pass() {
        let distiller = KnowledgeDistiller::default();
        let pattern = create_test_pattern(0.98, 100, 0.995);
        assert!(distiller.ready_for_graduation(&pattern));
    }

    #[test]
    fn test_ready_for_graduation_low_confidence() {
        let distiller = KnowledgeDistiller::default();
        let pattern = create_test_pattern(0.80, 100, 0.995);
        assert!(!distiller.ready_for_graduation(&pattern));
    }

    #[test]
    fn test_ready_for_graduation_low_usage() {
        let distiller = KnowledgeDistiller::default();
        let pattern = create_test_pattern(0.98, 10, 0.995);
        assert!(!distiller.ready_for_graduation(&pattern));
    }

    #[test]
    fn test_ready_for_graduation_low_success_rate() {
        let distiller = KnowledgeDistiller::default();
        let pattern = create_test_pattern(0.98, 100, 0.80);
        assert!(!distiller.ready_for_graduation(&pattern));
    }

    #[test]
    fn test_ready_for_graduation_boundary_values() {
        let distiller = KnowledgeDistiller::default();
        // Exactly at threshold
        let pattern = create_test_pattern(0.95, 50, 0.99);
        assert!(distiller.ready_for_graduation(&pattern));

        // Just below threshold
        let pattern_below = create_test_pattern(0.949, 50, 0.99);
        assert!(!distiller.ready_for_graduation(&pattern_below));
    }

    #[test]
    fn test_generate_rule() {
        let distiller = KnowledgeDistiller::default();
        let pattern = create_test_pattern(0.98, 100, 0.995);
        let rule = distiller.generate_rule(&pattern);

        assert!(rule.contains("test_pattern_001")); // snake_case conversion
        assert!(rule.contains("0.98")); // confidence
        assert!(rule.contains("100")); // usage count
        assert!(rule.contains("fn foo")); // rust output
    }

    #[test]
    fn test_generate_rule_with_special_chars() {
        let distiller = KnowledgeDistiller::default();
        let mut pattern = create_test_pattern(0.98, 100, 0.995);
        pattern.id = "pattern-with.dots-and-dashes".to_string();
        let rule = distiller.generate_rule(&pattern);

        // Check that special chars are replaced with underscores
        assert!(rule.contains("pattern_with_dots_and_dashes"));
    }

    #[test]
    fn test_find_graduation_candidates_empty_store() {
        let distiller = KnowledgeDistiller::default();
        let store = PatternStore::new();
        let candidates = distiller.find_graduation_candidates(&store);
        assert!(candidates.is_empty());
    }

    #[test]
    fn test_find_graduation_candidates_with_qualifying() {
        let distiller = KnowledgeDistiller::default();
        let mut store = PatternStore::new();

        // Add a qualifying pattern
        let mut pattern = create_test_pattern(0.98, 100, 0.995);
        pattern.id = "qualifying-pattern".to_string();
        store.add_pattern(pattern);

        // Add a non-qualifying pattern
        let mut non_qual = create_test_pattern(0.50, 5, 0.50);
        non_qual.id = "non-qualifying".to_string();
        store.add_pattern(non_qual);

        let candidates = distiller.find_graduation_candidates(&store);
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].id, "qualifying-pattern");
    }

    #[test]
    fn test_graduation_report_empty() {
        let distiller = KnowledgeDistiller::default();
        let store = PatternStore::new();
        let report = distiller.graduation_report(&store);
        assert_eq!(report, "No patterns ready for graduation.");
    }

    #[test]
    fn test_graduation_report_with_candidates() {
        let distiller = KnowledgeDistiller::default();
        let mut store = PatternStore::new();

        let mut pattern = create_test_pattern(0.98, 100, 0.995);
        pattern.id = "grad-candidate".to_string();
        pattern.error_prevented = "E0308: type mismatch".to_string();
        store.add_pattern(pattern);

        let report = distiller.graduation_report(&store);
        assert!(report.contains("Graduation Candidates (1)"));
        assert!(report.contains("grad-candidate"));
        assert!(report.contains("98.00%")); // confidence as percentage
        assert!(report.contains("Usage: 100"));
        assert!(report.contains("E0308: type mismatch"));
    }

    #[test]
    fn test_graduation_report_multiple_candidates() {
        let distiller = KnowledgeDistiller::default();
        let mut store = PatternStore::new();

        for i in 1..=3 {
            let mut pattern = create_test_pattern(0.96 + (i as f32 * 0.01), 60 + i * 10, 0.995);
            pattern.id = format!("pattern-{}", i);
            store.add_pattern(pattern);
        }

        let report = distiller.graduation_report(&store);
        assert!(report.contains("Graduation Candidates (3)"));
        assert!(report.contains("pattern-1"));
        assert!(report.contains("pattern-2"));
        assert!(report.contains("pattern-3"));
    }
}
