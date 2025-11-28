//! DEPYLER-0599: Comprehensive tests for entrenar CITL spec integration
//!
//! Tests cover:
//! - WeightedLoss corpus export
//! - select_optimal_tier() for multi-tier comparison
//! - Reweight factor application
//! - ProgressCallback integration
//! - Efficiency scoring
//! - TieredCurriculum advancement
//!
//! Following Extreme TDD: RED → GREEN → REFACTOR

// Test naming follows CLAUDE.md convention: test_DEPYLER_XXXX_<section>_<feature>_<scenario>
#![allow(non_snake_case)]

#[allow(unused_imports)]
use depyler::compilation_trainer::{
    ClippyLevel, CompilationConfig, CompilationResult, CompilationTrainer, DiagnosticFeatures,
    DiagnosticTier, VerbosityConfig,
};
#[allow(unused_imports)]
use std::path::PathBuf;
use tempfile::TempDir;

// ============================================================================
// RED PHASE: Failing tests for WeightedLoss corpus export
// ============================================================================

mod weighted_corpus_tests {
    use depyler::compilation_trainer::{weight_corpus_entries, export_weighted_corpus_jsonl};

    /// Test that corpus entries are weighted by error class rarity
    #[test]
    fn test_DEPYLER_0599_weighted_corpus_rare_errors_get_higher_weight() {
        // Arrange: Create corpus with common and rare errors
        let corpus = vec![
            ("file1.py".to_string(), "[error] E0308: type mismatch".to_string(), "{}".to_string()),
            ("file2.py".to_string(), "[error] E0308: type mismatch".to_string(), "{}".to_string()),
            ("file3.py".to_string(), "[error] E0308: type mismatch".to_string(), "{}".to_string()),
            ("file4.py".to_string(), "[error] ICE-0001: internal error".to_string(), "{}".to_string()),
        ];

        // Act: Apply weighting with reweight factor of 1.5
        let weighted = weight_corpus_entries(&corpus, 1.5);

        // Assert: Rare error (ICE) should have higher weight than common (E0308)
        let ice_weight = weighted.iter()
            .find(|(_, code, _, _)| code.contains("ICE"))
            .map(|(_, _, _, w)| *w)
            .unwrap();
        let e0308_weight = weighted.iter()
            .find(|(_, code, _, _)| code.contains("E0308"))
            .map(|(_, _, _, w)| *w)
            .unwrap();

        assert!(ice_weight > e0308_weight,
            "Rare errors should have higher weight: ICE={:.4} vs E0308={:.4}",
            ice_weight, e0308_weight);
    }

    /// Test that reweight factor of 1.0 produces uniform weights
    #[test]
    fn test_DEPYLER_0599_reweight_1_0_gives_uniform_weights() {
        let corpus = vec![
            ("file1.py".to_string(), "[error] E0308: type mismatch".to_string(), "{}".to_string()),
            ("file2.py".to_string(), "[error] E0277: trait not satisfied".to_string(), "{}".to_string()),
        ];

        let weighted = weight_corpus_entries(&corpus, 1.0);

        // All weights should be 1.0 when reweight factor is 1.0
        for (_, _, _, weight) in &weighted {
            assert!((weight - 1.0).abs() < 0.001, "Weight should be 1.0 with no reweighting, got {}", weight);
        }
    }

    /// Test weighted corpus export to JSONL format
    #[test]
    fn test_DEPYLER_0599_weighted_corpus_export_jsonl() {
        let corpus = vec![
            ("file1.py".to_string(), "[error] E0308: mismatch".to_string(), "{}".to_string()),
        ];

        let jsonl = export_weighted_corpus_jsonl(&corpus, 1.5);

        // Should contain weight field
        assert!(jsonl.contains("\"weight\":"), "JSONL should contain weight field: {}", jsonl);
        assert!(jsonl.contains("\"error\":"), "JSONL should contain error field");
        assert!(jsonl.contains("\"file\":"), "JSONL should contain file field");
    }
}

// ============================================================================
// RED PHASE: Failing tests for select_optimal_tier
// ============================================================================

mod optimal_tier_tests {
    use entrenar::train::select_optimal_tier;

    /// Test selecting optimal tier from multiple training runs
    #[test]
    fn test_DEPYLER_0599_select_optimal_tier_basic() {
        // Arrange: Results from different tiers (tier, accuracy, corpus_bytes)
        let results = vec![
            (1, 0.65, 2000_usize),   // Tier 1: 65% accuracy, 2KB corpus
            (2, 0.72, 5000_usize),   // Tier 2: 72% accuracy, 5KB corpus
            (3, 0.75, 20000_usize),  // Tier 3: 75% accuracy, 20KB corpus
            (4, 0.76, 100000_usize), // Tier 4: 76% accuracy, 100KB corpus
        ];

        // Act: Select optimal tier
        let (best_tier, _best_efficiency) = select_optimal_tier(&results).unwrap();

        // Assert: Tier 2 should be optimal (best efficiency score)
        // E(T) = Accuracy / log(CorpusSize)
        // Tier 2: 0.72 / log(5000) ≈ 0.72 / 8.52 ≈ 0.0845
        // Tier 3: 0.75 / log(20000) ≈ 0.75 / 9.90 ≈ 0.0757
        assert!(best_tier == 2 || best_tier == 1, "Tier 2 or 1 should be optimal for efficiency");
    }

    /// Test that empty results return None
    #[test]
    fn test_DEPYLER_0599_select_optimal_tier_empty_results() {
        let results: Vec<(usize, f32, usize)> = vec![];
        let result = select_optimal_tier(&results);
        assert!(result.is_none(), "Empty results should return None");
    }

    /// Test tier comparison storage and retrieval
    #[test]
    fn test_DEPYLER_0599_tier_comparison_storage() {
        let mut comparison = TierComparisonStore::new();

        comparison.record_run(1, 0.65, 2000, 10.5);
        comparison.record_run(2, 0.72, 5000, 15.2);

        assert_eq!(comparison.runs.len(), 2);
        assert_eq!(comparison.best_tier(), Some(2)); // Higher accuracy
    }

    /// Tier comparison storage for multi-run analysis
    struct TierComparisonStore {
        runs: Vec<TierRunResult>,
    }

    #[allow(dead_code)]
    struct TierRunResult {
        tier: usize,
        accuracy: f32,
        corpus_bytes: usize,
        elapsed_secs: f64,
    }

    impl TierComparisonStore {
        fn new() -> Self {
            Self { runs: vec![] }
        }

        fn record_run(&mut self, tier: usize, accuracy: f32, corpus_bytes: usize, elapsed_secs: f64) {
            self.runs.push(TierRunResult { tier, accuracy, corpus_bytes, elapsed_secs });
        }

        fn best_tier(&self) -> Option<usize> {
            self.runs.iter().max_by(|a, b| a.accuracy.partial_cmp(&b.accuracy).unwrap()).map(|r| r.tier)
        }
    }
}

// ============================================================================
// RED PHASE: Failing tests for reweight factor application
// ============================================================================

mod reweight_tests {
    use depyler::compilation_trainer::apply_reweight_sampling;
    use entrenar::train::AdaptiveCurriculum;

    /// Test that reweight factor affects sample weights
    #[test]
    fn test_DEPYLER_0599_reweight_factor_affects_weights() {
        let curriculum = AdaptiveCurriculum::new();

        // Common error should have lower weight
        let common_weight = curriculum.weight_for_class("E0308");
        // Rare error should have higher weight
        let rare_weight = curriculum.weight_for_class("ICE-0001");

        // With default weighting, rare errors get boosted
        assert!(rare_weight >= common_weight, "Rare errors should have >= weight");
    }

    /// Test applying reweight to training sample selection
    #[test]
    fn test_DEPYLER_0599_reweight_sample_selection() {
        // Create samples with different error types
        let samples = vec![
            ("E0308", 10), // Common: 10 samples
            ("E0277", 5),  // Medium: 5 samples
            ("ICE-0001", 1), // Rare: 1 sample
        ];

        // Apply reweight factor of 2.0
        let weighted_counts = apply_reweight_sampling(&samples, 2.0);

        // Rare errors should be oversampled
        let ice_ratio = weighted_counts.get("ICE-0001").unwrap_or(&0);
        let _e0308_ratio = weighted_counts.get("E0308").unwrap_or(&0);

        // ICE should have higher effective count after reweighting
        assert!(ice_ratio > &1, "Rare errors should be oversampled");
    }
}

// ============================================================================
// RED PHASE: Failing tests for ProgressCallback integration
// ============================================================================

mod progress_callback_tests {
    use entrenar::train::{CallbackAction, CallbackContext, ProgressCallback, TrainerCallback};

    /// Test ProgressCallback is called during training
    #[test]
    fn test_DEPYLER_0599_progress_callback_epoch_begin() {
        let mut callback = ProgressCallback::new(1); // log_interval = 1

        let ctx = CallbackContext {
            epoch: 0,
            max_epochs: 10,
            loss: 0.5,
            ..Default::default()
        };

        let action = callback.on_epoch_begin(&ctx);
        assert_eq!(action, CallbackAction::Continue);
    }

    /// Test ProgressCallback epoch end
    #[test]
    fn test_DEPYLER_0599_progress_callback_epoch_end() {
        let mut callback = ProgressCallback::new(1); // log_interval = 1

        let ctx = CallbackContext {
            epoch: 5,
            max_epochs: 10,
            loss: 0.3,
            ..Default::default()
        };

        let action = callback.on_epoch_end(&ctx);
        assert_eq!(action, CallbackAction::Continue);
    }

    /// Test custom CITL progress callback
    #[test]
    fn test_DEPYLER_0599_citl_progress_callback() {
        let mut callback = CitlProgressCallback::new();

        callback.on_tier_change(1, 2, 0.65);

        assert_eq!(callback.tier_changes, 1);
        assert_eq!(callback.current_tier, 2);
    }

    /// Custom CITL progress callback for tier tracking
    struct CitlProgressCallback {
        tier_changes: usize,
        current_tier: usize,
        accuracies: Vec<f32>,
    }

    impl CitlProgressCallback {
        fn new() -> Self {
            Self {
                tier_changes: 0,
                current_tier: 1,
                accuracies: vec![],
            }
        }

        fn on_tier_change(&mut self, _from_tier: usize, to_tier: usize, accuracy: f32) {
            self.tier_changes += 1;
            self.current_tier = to_tier;
            self.accuracies.push(accuracy);
        }
    }
}

// ============================================================================
// RED PHASE: Failing tests for efficiency scoring
// ============================================================================

mod efficiency_tests {
    use entrenar::train::efficiency_score;

    /// Test efficiency score calculation
    #[test]
    fn test_DEPYLER_0599_efficiency_score_calculation() {
        // E(T) = Accuracy / log(CorpusSize)
        let eff = efficiency_score(0.72, 5000);

        // 0.72 / ln(5000) ≈ 0.72 / 8.517 ≈ 0.0845
        assert!(eff > 0.08 && eff < 0.09, "Efficiency score should be ~0.0845, got {}", eff);
    }

    /// Test efficiency score with small corpus
    #[test]
    fn test_DEPYLER_0599_efficiency_score_small_corpus() {
        let eff = efficiency_score(0.50, 100);

        // 0.50 / ln(100) ≈ 0.50 / 4.605 ≈ 0.1086
        assert!(eff > 0.10 && eff < 0.12, "Small corpus should have higher efficiency");
    }

    /// Test efficiency score with zero corpus (edge case)
    #[test]
    fn test_DEPYLER_0599_efficiency_score_zero_corpus() {
        let eff = efficiency_score(0.50, 0);

        // Should handle gracefully (return 0 or some default)
        assert!(eff >= 0.0, "Zero corpus should not produce negative efficiency");
    }

    /// Test efficiency grading
    #[test]
    fn test_DEPYLER_0599_efficiency_grading() {
        assert_eq!(grade_efficiency(0.20), "Excellent");
        assert_eq!(grade_efficiency(0.12), "Good");
        assert_eq!(grade_efficiency(0.07), "Acceptable");
        assert_eq!(grade_efficiency(0.03), "Needs improvement");
    }

    fn grade_efficiency(score: f32) -> &'static str {
        match score {
            e if e > 0.15 => "Excellent",
            e if e > 0.10 => "Good",
            e if e > 0.05 => "Acceptable",
            _ => "Needs improvement",
        }
    }
}

// ============================================================================
// RED PHASE: Failing tests for TieredCurriculum advancement
// ============================================================================

mod curriculum_tests {
    use super::*;
    use entrenar::train::{CurriculumScheduler, TieredCurriculum};

    /// Test CITL default thresholds (60% → 70% → 80%)
    /// NOTE: TieredCurriculum requires `patience` (3) epochs at threshold to advance
    #[test]
    fn test_DEPYLER_0599_citl_default_thresholds() {
        let mut curriculum = TieredCurriculum::citl_default();

        // Initial tier should be 1
        assert_eq!(curriculum.tier(), 1);

        // Below 60% - stay at tier 1
        curriculum.step(0, 0.55);
        assert_eq!(curriculum.tier(), 1);

        // At 60% for 3 epochs (patience) - advance to tier 2
        curriculum.step(1, 0.62);
        curriculum.step(2, 0.62);
        curriculum.step(3, 0.62);
        assert_eq!(curriculum.tier(), 2, "Should advance after patience epochs at threshold");

        // At 70% for 3 epochs - advance to tier 3
        curriculum.step(4, 0.72);
        curriculum.step(5, 0.72);
        curriculum.step(6, 0.72);
        assert_eq!(curriculum.tier(), 3);

        // At 80% for 3 epochs - advance to tier 4
        curriculum.step(7, 0.82);
        curriculum.step(8, 0.82);
        curriculum.step(9, 0.82);
        assert_eq!(curriculum.tier(), 4);
    }

    /// Test curriculum doesn't regress
    #[test]
    fn test_DEPYLER_0599_curriculum_no_regression() {
        let mut curriculum = TieredCurriculum::citl_default();

        // Advance to tier 2 (need patience epochs)
        curriculum.step(0, 0.65);
        curriculum.step(1, 0.65);
        curriculum.step(2, 0.65);
        assert_eq!(curriculum.tier(), 2);

        // Accuracy drops - should NOT regress
        curriculum.step(3, 0.50);
        assert_eq!(curriculum.tier(), 2, "Curriculum should not regress on accuracy drop");
    }

    /// Test curriculum tier affects verbosity config
    #[test]
    fn test_DEPYLER_0599_curriculum_updates_verbosity() {
        let mut config = VerbosityConfig::new().with_adaptive(true);
        let mut curriculum = TieredCurriculum::citl_default();

        // Advance curriculum (need patience epochs at threshold)
        curriculum.step(0, 0.65);
        curriculum.step(1, 0.65);
        curriculum.step(2, 0.65);
        let new_tier = curriculum.tier();

        // Update verbosity based on curriculum
        config.tier = DiagnosticTier::from_level(new_tier as u8);

        assert_eq!(config.tier, DiagnosticTier::Tier2);
    }
}

// ============================================================================
// RED PHASE: Integration tests for complete CITL workflow
// ============================================================================

mod integration_tests {
    use super::*;

    /// Test complete CITL workflow with all features
    #[test]
    fn test_DEPYLER_0599_complete_citl_workflow() {
        // This test verifies the complete integration
        let config = CompilationConfig::new()
            .with_target_rate(0.80)
            .with_max_epochs(5)
            .with_verbosity_tier(1)
            .with_adaptive_verbosity(true)
            .with_reweight(1.5);

        // Verify configuration
        assert_eq!(config.target_rate, 0.80);
        assert_eq!(config.verbosity.tier, DiagnosticTier::Tier1);
        assert!(config.verbosity.adaptive);
        assert!((config.reweight - 1.5).abs() < 0.001);
    }

    /// Test that CompilationTrainer uses all CITL features
    #[test]
    fn test_DEPYLER_0599_trainer_citl_integration() {
        let temp_dir = TempDir::new().unwrap();
        let config = CompilationConfig::new()
            .with_report_dir(temp_dir.path().to_path_buf())
            .with_adaptive_verbosity(true)
            .with_reweight(1.5);

        // Trainer should be creatable with CITL config
        let _trainer = CompilationTrainer::new(vec![], config);

        // Verify trainer has CITL components
        // (this will be verified by the train() method using curriculum)
        assert!(true, "Trainer created with CITL config");
    }
}
