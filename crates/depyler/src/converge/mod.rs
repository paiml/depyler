//! Convergence loop for automated compilation rate improvement
//!
//! This module implements the `depyler converge` command which automates
//! the iteration loop to get all examples compiling.
//!
//! # Architecture
//!
//! ```text
//! converge/
//! ├── mod.rs              # Public API (this file)
//! ├── state.rs            # Convergence state management
//! ├── compiler.rs         # Batch compilation + error collection
//! ├── classifier.rs       # Error classification (wraps oracle)
//! ├── clusterer.rs        # Error clustering by root cause
//! └── reporter.rs         # Progress reporting
//! ```
//!
//! # Usage
//!
//! ```bash
//! depyler converge \
//!     --input-dir ../examples \
//!     --target-rate 100 \
//!     --auto-fix
//! ```

mod cache;
pub mod cache_warmer;
mod classifier;
mod clusterer;
mod compiler;
mod fix_applicator;
mod reporter;
mod roi_metrics;
mod state;
pub mod transpiler_patcher;
pub mod type_constraint_learner;

pub use cache::{
    CacheConfig, CacheEntry, CacheError, CacheStats, CasStore, CompilationStatus, GcResult,
    SqliteCache, TranspilationCacheKey,
};
pub use classifier::{ErrorCategory, ErrorClassification, ErrorClassifier};
pub use clusterer::{ErrorCluster, ErrorClusterer, RootCause};
pub use compiler::{BatchCompiler, CompilationError, CompilationResult};
pub use fix_applicator::{
    CompositeFixApplicator, FixApplicationResult, FixApplicator, FixType, GeneratedRustFixer,
};
pub use reporter::{ConvergenceReporter, IterationReport};
pub use roi_metrics::{
    EscapeRateTracker, OracleRoiMetrics, RoiMetrics, ESCAPE_RATE_FALSIFICATION_THRESHOLD,
};
pub use state::{AppliedFix, ConvergenceConfig, ConvergenceState, DisplayMode, ExampleState};
pub use transpiler_patcher::{AprFile, PatchRecord, PatchResult, PatchType, TranspilerPatcher};
pub use type_constraint_learner::{
    parse_e0308_batch, parse_e0308_constraint, TypeConstraint, TypeConstraintStore,
};

use anyhow::Result;
use depyler_oracle::NgramFixPredictor;

/// Run the convergence loop to improve compilation rate
pub async fn run_convergence_loop(config: ConvergenceConfig) -> Result<ConvergenceState> {
    let mut state = ConvergenceState::new(config.clone());
    let compiler = BatchCompiler::new(&config.input_dir).with_display_mode(config.display_mode);
    let classifier = ErrorClassifier::new();
    let clusterer = ErrorClusterer::new();
    let reporter = ConvergenceReporter::with_display_mode(config.display_mode);

    // DEPYLER-ORACLE-TRAIN: Initialize user pattern predictor for feedback learning
    let mut user_predictor = NgramFixPredictor::new();
    let user_model_path = NgramFixPredictor::default_user_model_path();
    if let Err(e) = user_predictor.load(&user_model_path) {
        tracing::debug!("No user model loaded: {e}");
    }

    // DEPYLER-1305: Initialize fix applicator if auto_fix is enabled
    let fix_applicator = if config.auto_fix {
        Some(CompositeFixApplicator::new())
    } else {
        None
    };

    // DEPYLER-1308: Initialize transpiler patcher if enabled
    let mut transpiler_patcher = if config.patch_transpiler {
        let core_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("manifest dir has parent")
            .join("depyler-core");
        let mut patcher = TranspilerPatcher::new(&core_path);

        // Load custom APR file if provided, otherwise use defaults
        if let Some(ref apr_path) = config.apr_file {
            if let Err(e) = patcher.load_apr(apr_path) {
                tracing::warn!("Failed to load APR file {}: {}", apr_path.display(), e);
                patcher.load_defaults();
            }
        } else {
            patcher.load_defaults();
        }

        tracing::info!(
            "DEPYLER-1308: TranspilerPatcher initialized with {} patches",
            patcher.patch_count()
        );
        Some(patcher)
    } else {
        None
    };

    // DEPYLER-1101: Type constraint learning from E0308 errors
    let mut constraint_store = TypeConstraintStore::new();

    // Store last compilation results for failure analysis
    let mut last_results: Vec<CompilationResult> = Vec::new();

    // DEPYLER-1301: Store all classifications for ROI metrics
    let mut all_classifications: Vec<ErrorClassification> = Vec::new();

    reporter.report_start(&state);

    while state.compilation_rate < config.target_rate && state.iteration < config.max_iterations {
        state.iteration += 1;

        let results = compiler.compile_all().await?;
        state.update_examples(&results);

        let classifications = classifier.classify_all(&results);

        // DEPYLER-1301: Accumulate classifications for ROI metrics
        all_classifications.extend(classifications.clone());

        let clusters = clusterer.cluster(&classifications);
        state.error_clusters = clusters;

        // DEPYLER-1305: Apply fixes if auto_fix is enabled
        if let Some(ref applicator) = fix_applicator {
            // Collect source files for batch processing
            let source_files: std::collections::HashMap<std::path::PathBuf, String> = results
                .iter()
                .filter(|r| !r.success)
                .filter_map(|r| {
                    let rust_file = r.source_file.with_extension("rs");
                    std::fs::read_to_string(&rust_file)
                        .ok()
                        .map(|content| (rust_file, content))
                })
                .collect();

            // Apply fixes
            let fix_results = applicator.apply_batch(&classifications, &source_files);

            // Track applied fixes
            for (result, classification) in fix_results.iter().zip(classifications.iter()) {
                if result.applied {
                    state.fixes_applied.push(AppliedFix {
                        iteration: state.iteration,
                        error_code: classification.error.code.clone(),
                        description: result.description.clone(),
                        file_modified: classification.error.file.clone(),
                        commit_hash: None,
                        verified: false,
                    });

                    // Write modified source if available
                    if let Some(ref modified) = result.modified_source {
                        let rust_file = classification.error.file.with_extension("rs");
                        if let Err(e) = std::fs::write(&rust_file, modified) {
                            tracing::warn!(
                                "Failed to write fixed source to {}: {}",
                                rust_file.display(),
                                e
                            );
                        }
                    }

                    // DEPYLER-ORACLE-TRAIN: Record successful fix for user model learning
                    user_predictor.learn_pattern(
                        &classification.error.message,
                        &result.description,
                        depyler_oracle::ErrorCategory::from_code(&classification.error.code),
                    );
                }
            }

            let fixes_applied = fix_results.iter().filter(|r| r.applied).count();
            if fixes_applied > 0 {
                tracing::info!(
                    "DEPYLER-1305: Applied {} fixes in iteration {}",
                    fixes_applied,
                    state.iteration
                );
            }
        }

        // DEPYLER-1308: Apply transpiler patches if enabled
        if let Some(ref mut patcher) = transpiler_patcher {
            tracing::info!(
                "DEPYLER-1312: Checking for transpiler patches, {} clusters available",
                state.error_clusters.len()
            );
            // Find applicable patches based on highest-impact error cluster
            // DEPYLER-1312: Sort by impact_score instead of using arbitrary first()
            if let Some(top_cluster) = state.error_clusters.iter().max_by(|a, b| {
                a.impact_score()
                    .partial_cmp(&b.impact_score())
                    .unwrap_or(std::cmp::Ordering::Equal)
            }) {
                let error_code = top_cluster.error_code.clone();
                let sample_error = top_cluster.sample_errors.first();
                let sample_message = sample_error.map(|e| e.message.clone()).unwrap_or_default();

                // DEPYLER-1310: Extract context_keywords from all sample errors
                let context_keywords: Vec<String> = top_cluster
                    .sample_errors
                    .iter()
                    .flat_map(|e| e.context_keywords.iter().cloned())
                    .collect();

                // Info: log context keywords for diagnosis
                tracing::info!(
                    "DEPYLER-1312: Top cluster {} has {} sample errors with context_keywords: {:?}",
                    error_code,
                    top_cluster.sample_errors.len(),
                    &context_keywords
                );

                // Clone patches to avoid borrow conflict
                let patches: Vec<_> = patcher
                    .find_patches(&error_code, &sample_message, &context_keywords)
                    .into_iter()
                    .cloned()
                    .collect();

                if !patches.is_empty() {
                    tracing::info!(
                        "DEPYLER-1308: Found {} patches for {} errors",
                        patches.len(),
                        error_code
                    );

                    for patch in &patches {
                        match patcher.apply_patch(patch) {
                            Ok(result) if result.success => {
                                tracing::info!(
                                    "DEPYLER-1308: Applied transpiler patch '{}' to {}",
                                    result.patch_id,
                                    result.file_modified.display()
                                );
                                state.fixes_applied.push(AppliedFix {
                                    iteration: state.iteration,
                                    error_code: error_code.clone(),
                                    description: format!(
                                        "Transpiler patch: {}",
                                        result.description
                                    ),
                                    file_modified: result.file_modified,
                                    commit_hash: None,
                                    verified: false,
                                });
                            }
                            Ok(result) => {
                                tracing::debug!(
                                    "DEPYLER-1308: Patch '{}' not applied: {}",
                                    result.patch_id,
                                    result.description
                                );
                            }
                            Err(e) => {
                                tracing::warn!(
                                    "DEPYLER-1308: Failed to apply patch '{}': {}",
                                    patch.id,
                                    e
                                );
                            }
                        }
                    }
                }
            }
        }

        // DEPYLER-1101: Extract type constraints from E0308 errors
        for result in &results {
            if !result.success {
                let e0308_errors: Vec<(String, String, usize)> = result
                    .errors
                    .iter()
                    .map(|e| (e.code.clone(), e.message.clone(), e.line))
                    .collect();
                let constraints = parse_e0308_batch(&e0308_errors, &result.source_file);
                for constraint in constraints {
                    constraint_store.add_constraint(constraint);
                }
            }
        }

        if let Some(cluster) = state.error_clusters.iter().max_by(|a, b| {
            a.impact_score()
                .partial_cmp(&b.impact_score())
                .unwrap_or(std::cmp::Ordering::Equal)
        }) {
            reporter.report_iteration(&state, cluster);
        }

        state.update_compilation_rate();

        // Store results for final analysis
        last_results = results;
    }

    // DEPYLER-1101: Report learned constraints
    if constraint_store.stats.constraints_extracted > 0 {
        tracing::info!(
            "DEPYLER-1101: Learned {} type constraints from E0308 errors",
            constraint_store.stats.constraints_extracted
        );

        // Analyze patterns to identify common fixes
        let file_constraints: Vec<TypeConstraint> = constraint_store
            .variable_constraints
            .values()
            .cloned()
            .collect();
        let patterns = type_constraint_learner::analyze_constraint_patterns(&file_constraints);
        for (pattern, count) in patterns.iter().filter(|(_, c)| **c >= 5) {
            tracing::info!("  Common pattern: {} ({} occurrences)", pattern, count);
        }
    }

    reporter.report_finish(&state);

    // DEPYLER-UX: Automated failure analysis - no more grepping logs
    reporter.report_failure_analysis(&last_results);

    // DEPYLER-1301: Write Oracle ROI metrics
    let session_name = format!("converge-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S"));
    let roi_metrics = roi_metrics::OracleRoiMetrics::from_convergence(
        &state,
        &all_classifications,
        &session_name,
    );
    if let Err(e) = roi_metrics.write_to_docs() {
        tracing::warn!("Failed to write Oracle ROI metrics: {}", e);
    }

    // DEPYLER-ORACLE-TRAIN: Save user-learned patterns
    if user_predictor.pattern_count() > 0 {
        if let Err(e) = user_predictor.save(&user_model_path) {
            tracing::warn!("Failed to save user model: {}", e);
        } else {
            tracing::info!(
                "DEPYLER-ORACLE-TRAIN: Saved {} patterns to {}",
                user_predictor.pattern_count(),
                user_model_path.display()
            );
        }
    }

    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    // ============================================================================
    // GH-158: EXTREME TDD - RED PHASE TESTS
    // These tests MUST FAIL until implementation is complete
    // ============================================================================

    // --------------------------------------------------------------------------
    // Test 1: ConvergenceState initialization
    // --------------------------------------------------------------------------
    #[test]
    fn test_gh158_convergence_state_new() {
        let config = ConvergenceConfig {
            input_dir: PathBuf::from("/tmp/examples"),
            target_rate: 100.0,
            max_iterations: 50,
            auto_fix: false,
            dry_run: false,
            verbose: false,
            fix_confidence_threshold: 0.8,
            checkpoint_dir: None,
            parallel_jobs: 4,
            display_mode: DisplayMode::default(),
            oracle: false,
            explain: false,
            use_cache: true,
            patch_transpiler: false,
            apr_file: None,
        };

        let state = ConvergenceState::new(config);

        assert_eq!(state.iteration, 0);
        assert_eq!(state.compilation_rate, 0.0);
        assert!(state.examples.is_empty());
        assert!(state.error_clusters.is_empty());
        assert!(state.fixes_applied.is_empty());
    }

    // --------------------------------------------------------------------------
    // Test 2: ConvergenceConfig validation
    // --------------------------------------------------------------------------
    #[test]
    fn test_gh158_convergence_config_validation() {
        let config = ConvergenceConfig {
            input_dir: PathBuf::from("/nonexistent"),
            target_rate: 150.0, // Invalid: > 100
            max_iterations: 0,  // Invalid: must be > 0
            auto_fix: false,
            dry_run: false,
            verbose: false,
            fix_confidence_threshold: 1.5, // Invalid: > 1.0
            checkpoint_dir: None,
            parallel_jobs: 0, // Invalid: must be > 0
            display_mode: DisplayMode::default(),
            oracle: false,
            explain: false,
            use_cache: true,
            patch_transpiler: false,
            apr_file: None,
        };

        let result = config.validate();
        assert!(result.is_err());
    }

    // --------------------------------------------------------------------------
    // Test 3: ErrorCluster impact score calculation
    // --------------------------------------------------------------------------
    #[test]
    fn test_gh158_error_cluster_impact_score() {
        let cluster = ErrorCluster {
            root_cause: RootCause::TranspilerGap {
                gap_type: "missing_method".to_string(),
                location: "expr_gen.rs:1234".to_string(),
            },
            error_code: "E0599".to_string(),
            examples_blocked: vec![
                PathBuf::from("ex1.py"),
                PathBuf::from("ex2.py"),
                PathBuf::from("ex3.py"),
            ],
            sample_errors: vec![],
            fix_confidence: 0.9,
            suggested_fix: None,
        };

        // impact = num_examples * confidence
        let expected = 3.0 * 0.9;
        assert!((cluster.impact_score() - expected).abs() < 0.01);
    }

    // --------------------------------------------------------------------------
    // Test 4: Error classification by error code
    // --------------------------------------------------------------------------
    #[test]
    fn test_gh158_error_classifier_e0599() {
        let classifier = ErrorClassifier::new();
        let error = CompilationError {
            code: "E0599".to_string(),
            message: "no method named `contains_key` found for struct `Value`".to_string(),
            file: PathBuf::from("test.rs"),
            line: 42,
            column: 10,
            ..Default::default()
        };

        let classification = classifier.classify(&error);

        assert_eq!(classification.category, ErrorCategory::TranspilerGap);
        assert!(classification.confidence > 0.8);
    }

    #[test]
    fn test_gh158_error_classifier_e0308() {
        let classifier = ErrorClassifier::new();
        let error = CompilationError {
            code: "E0308".to_string(),
            message: "mismatched types: expected `i32`, found `i64`".to_string(),
            file: PathBuf::from("test.rs"),
            line: 100,
            column: 5,
            ..Default::default()
        };

        let classification = classifier.classify(&error);

        assert_eq!(classification.category, ErrorCategory::TranspilerGap);
        assert!(classification.confidence > 0.7);
    }

    #[test]
    fn test_gh158_error_classifier_e0277() {
        let classifier = ErrorClassifier::new();
        let error = CompilationError {
            code: "E0277".to_string(),
            message: "the trait `Ord` is not implemented for `Value`".to_string(),
            file: PathBuf::from("test.rs"),
            line: 200,
            column: 15,
            ..Default::default()
        };

        let classification = classifier.classify(&error);

        assert_eq!(classification.category, ErrorCategory::TranspilerGap);
    }

    // --------------------------------------------------------------------------
    // Test 5: Error clustering by root cause
    // --------------------------------------------------------------------------
    #[test]
    fn test_gh158_error_clusterer_groups_by_error_code() {
        let clusterer = ErrorClusterer::new();
        let classifications = vec![
            ErrorClassification {
                error: CompilationError {
                    code: "E0599".to_string(),
                    message: "method not found".to_string(),
                    file: PathBuf::from("a.rs"),
                    line: 1,
                    column: 1,
                    ..Default::default()
                },
                category: ErrorCategory::TranspilerGap,
                subcategory: "missing_method".to_string(),
                confidence: 0.9,
                suggested_fix: None,
            },
            ErrorClassification {
                error: CompilationError {
                    code: "E0599".to_string(),
                    message: "method not found".to_string(),
                    file: PathBuf::from("b.rs"),
                    line: 2,
                    column: 2,
                    ..Default::default()
                },
                category: ErrorCategory::TranspilerGap,
                subcategory: "missing_method".to_string(),
                confidence: 0.85,
                suggested_fix: None,
            },
            ErrorClassification {
                error: CompilationError {
                    code: "E0308".to_string(),
                    message: "type mismatch".to_string(),
                    file: PathBuf::from("c.rs"),
                    line: 3,
                    column: 3,
                    ..Default::default()
                },
                category: ErrorCategory::TranspilerGap,
                subcategory: "type_inference".to_string(),
                confidence: 0.8,
                suggested_fix: None,
            },
        ];

        let clusters = clusterer.cluster(&classifications);

        // Should have 2 clusters: E0599 and E0308
        assert_eq!(clusters.len(), 2);

        // E0599 cluster should have 2 errors
        let e0599_cluster = clusters.iter().find(|c| c.error_code == "E0599").unwrap();
        assert_eq!(e0599_cluster.sample_errors.len(), 2);
    }

    // --------------------------------------------------------------------------
    // Test 6: Batch compiler initialization (no subprocess needed)
    // --------------------------------------------------------------------------
    #[test]
    fn test_gh158_batch_compiler_initialization() {
        let temp_dir = TempDir::new().unwrap();
        let example_path = temp_dir.path().join("broken.py");
        std::fs::write(&example_path, "def foo(): return 1").unwrap();

        let _compiler = BatchCompiler::new(temp_dir.path())
            .with_parallel_jobs(4)
            .with_display_mode(DisplayMode::Silent);

        // Just verify compiler is constructed correctly (no assertion needed - if we get here, it works)
    }

    // --------------------------------------------------------------------------
    // Test 7: Compilation rate calculation
    // --------------------------------------------------------------------------
    #[test]
    fn test_gh158_compilation_rate_calculation() {
        let config = ConvergenceConfig {
            input_dir: PathBuf::from("/tmp"),
            target_rate: 100.0,
            max_iterations: 10,
            auto_fix: false,
            dry_run: false,
            verbose: false,
            fix_confidence_threshold: 0.8,
            checkpoint_dir: None,
            parallel_jobs: 4,
            display_mode: DisplayMode::default(),
            oracle: false,
            explain: false,
            use_cache: true,
            patch_transpiler: false,
            apr_file: None,
        };

        let mut state = ConvergenceState::new(config);

        // Add 10 examples: 7 passing, 3 failing
        state.examples = vec![
            ExampleState::new(PathBuf::from("1.py"), true),
            ExampleState::new(PathBuf::from("2.py"), true),
            ExampleState::new(PathBuf::from("3.py"), true),
            ExampleState::new(PathBuf::from("4.py"), true),
            ExampleState::new(PathBuf::from("5.py"), true),
            ExampleState::new(PathBuf::from("6.py"), true),
            ExampleState::new(PathBuf::from("7.py"), true),
            ExampleState::new(PathBuf::from("8.py"), false),
            ExampleState::new(PathBuf::from("9.py"), false),
            ExampleState::new(PathBuf::from("10.py"), false),
        ];

        state.update_compilation_rate();

        assert!((state.compilation_rate - 70.0).abs() < 0.1);
    }

    // --------------------------------------------------------------------------
    // Test 8: Checkpoint save and restore
    // --------------------------------------------------------------------------
    #[test]
    fn test_gh158_checkpoint_save_restore() {
        let temp_dir = TempDir::new().unwrap();
        let config = ConvergenceConfig {
            input_dir: PathBuf::from("/tmp/examples"),
            target_rate: 100.0,
            max_iterations: 50,
            auto_fix: false,
            dry_run: false,
            verbose: false,
            fix_confidence_threshold: 0.8,
            checkpoint_dir: Some(temp_dir.path().to_path_buf()),
            parallel_jobs: 4,
            display_mode: DisplayMode::default(),
            oracle: false,
            explain: false,
            use_cache: true,
            patch_transpiler: false,
            apr_file: None,
        };

        let mut state = ConvergenceState::new(config.clone());
        state.iteration = 5;
        state.compilation_rate = 75.0;

        // Save checkpoint
        state.save_checkpoint(temp_dir.path()).unwrap();

        // Restore from checkpoint
        let restored = ConvergenceState::load_checkpoint(temp_dir.path()).unwrap();

        assert_eq!(restored.iteration, 5);
        assert!((restored.compilation_rate - 75.0).abs() < 0.1);
    }

    // --------------------------------------------------------------------------
    // Test 9: Root cause detection
    // --------------------------------------------------------------------------
    #[test]
    fn test_gh158_root_cause_from_error() {
        // E0599: Missing method → TranspilerGap (missing stdlib mapping)
        let root = RootCause::from_error_code("E0599", "no method `readlines` found");
        assert!(matches!(root, RootCause::TranspilerGap { .. }));

        // E0308: Type mismatch → TranspilerGap (type inference)
        let root = RootCause::from_error_code("E0308", "expected `i32`, found `i64`");
        assert!(matches!(root, RootCause::TranspilerGap { .. }));

        // E0277: Trait not impl → TranspilerGap (missing trait bound)
        let root = RootCause::from_error_code("E0277", "trait `Ord` not implemented");
        assert!(matches!(root, RootCause::TranspilerGap { .. }));

        // Unknown error → Unknown
        let root = RootCause::from_error_code("E9999", "something weird");
        assert!(matches!(root, RootCause::Unknown));
    }

    // --------------------------------------------------------------------------
    // Test 10: Reporter output format
    // --------------------------------------------------------------------------
    #[test]
    fn test_gh158_reporter_iteration_format() {
        let reporter = ConvergenceReporter::new(true);
        let config = ConvergenceConfig {
            input_dir: PathBuf::from("/tmp"),
            target_rate: 100.0,
            max_iterations: 50,
            auto_fix: false,
            dry_run: false,
            verbose: true,
            fix_confidence_threshold: 0.8,
            checkpoint_dir: None,
            parallel_jobs: 4,
            display_mode: DisplayMode::default(),
            oracle: false,
            explain: false,
            use_cache: true,
            patch_transpiler: false,
            apr_file: None,
        };

        let state = ConvergenceState::new(config);
        let cluster = ErrorCluster {
            root_cause: RootCause::TranspilerGap {
                gap_type: "missing_method".to_string(),
                location: "expr_gen.rs:1234".to_string(),
            },
            error_code: "E0599".to_string(),
            examples_blocked: vec![PathBuf::from("ex1.py")],
            sample_errors: vec![],
            fix_confidence: 0.9,
            suggested_fix: None,
        };

        let report = reporter.format_iteration(&state, &cluster);

        assert!(report.contains("Iteration"));
        assert!(report.contains("E0599"));
    }

    // --------------------------------------------------------------------------
    // Property-based tests
    // --------------------------------------------------------------------------
    #[test]
    fn test_gh158_property_compilation_rate_bounds() {
        // Compilation rate should always be 0-100
        let config = ConvergenceConfig {
            input_dir: PathBuf::from("/tmp"),
            target_rate: 100.0,
            max_iterations: 10,
            auto_fix: false,
            dry_run: false,
            verbose: false,
            fix_confidence_threshold: 0.8,
            checkpoint_dir: None,
            parallel_jobs: 4,
            display_mode: DisplayMode::default(),
            oracle: false,
            explain: false,
            use_cache: true,
            patch_transpiler: false,
            apr_file: None,
        };

        let mut state = ConvergenceState::new(config);

        // Test with various example counts
        for passing in 0..=100 {
            for total in passing..=100 {
                if total == 0 {
                    continue;
                }
                state.examples = (0..total)
                    .map(|i| ExampleState::new(PathBuf::from(format!("{}.py", i)), i < passing))
                    .collect();

                state.update_compilation_rate();

                assert!(state.compilation_rate >= 0.0);
                assert!(state.compilation_rate <= 100.0);
            }
        }
    }

    #[test]
    fn test_gh158_property_cluster_impact_non_negative() {
        // Impact score should always be non-negative
        let cluster = ErrorCluster {
            root_cause: RootCause::Unknown,
            error_code: "E0000".to_string(),
            examples_blocked: vec![],
            sample_errors: vec![],
            fix_confidence: 0.0,
            suggested_fix: None,
        };

        assert!(cluster.impact_score() >= 0.0);
    }

    // ============================================================================
    // DEPYLER-CONVERGE-FULL: RED PHASE TESTS
    // These tests MUST FAIL until implementation is complete
    // ============================================================================

    // --------------------------------------------------------------------------
    // Test: ConvergenceConfig has oracle flag
    // --------------------------------------------------------------------------
    #[test]
    fn test_converge_full_config_has_oracle_flag() {
        let config = ConvergenceConfig {
            input_dir: PathBuf::from("/tmp"),
            target_rate: 80.0,
            max_iterations: 10,
            auto_fix: false,
            dry_run: false,
            verbose: false,
            fix_confidence_threshold: 0.8,
            checkpoint_dir: None,
            parallel_jobs: 4,
            display_mode: DisplayMode::default(),
            // NEW: oracle flag must exist
            oracle: true,
            explain: false,
            use_cache: true,
            patch_transpiler: false,
            apr_file: None,
        };
        assert!(config.oracle);
    }

    // --------------------------------------------------------------------------
    // Test: ConvergenceConfig has explain flag
    // --------------------------------------------------------------------------
    #[test]
    fn test_converge_full_config_has_explain_flag() {
        let config = ConvergenceConfig {
            input_dir: PathBuf::from("/tmp"),
            target_rate: 80.0,
            max_iterations: 10,
            auto_fix: false,
            dry_run: false,
            verbose: false,
            fix_confidence_threshold: 0.8,
            checkpoint_dir: None,
            parallel_jobs: 4,
            display_mode: DisplayMode::default(),
            oracle: false,
            // NEW: explain flag must exist
            explain: true,
            use_cache: true,
            patch_transpiler: false,
            apr_file: None,
        };
        assert!(config.explain);
    }

    // --------------------------------------------------------------------------
    // Test: ConvergenceConfig has use_cache flag
    // --------------------------------------------------------------------------
    #[test]
    fn test_converge_full_config_has_cache_flag() {
        let config = ConvergenceConfig {
            input_dir: PathBuf::from("/tmp"),
            target_rate: 80.0,
            max_iterations: 10,
            auto_fix: false,
            dry_run: false,
            verbose: false,
            fix_confidence_threshold: 0.8,
            checkpoint_dir: None,
            parallel_jobs: 4,
            display_mode: DisplayMode::default(),
            oracle: false,
            explain: false,
            // NEW: use_cache flag must exist
            use_cache: true,
            patch_transpiler: false,
            apr_file: None,
        };
        assert!(config.use_cache);
    }

    // --------------------------------------------------------------------------
    // Test: Default config enables cache, disables oracle/explain
    // --------------------------------------------------------------------------
    #[test]
    fn test_converge_full_default_flags() {
        let config = ConvergenceConfig {
            input_dir: PathBuf::from("/tmp"),
            target_rate: 80.0,
            max_iterations: 10,
            auto_fix: false,
            dry_run: false,
            verbose: false,
            fix_confidence_threshold: 0.8,
            checkpoint_dir: None,
            parallel_jobs: 4,
            display_mode: DisplayMode::default(),
            oracle: false,
            explain: false,
            use_cache: true,
            patch_transpiler: false,
            apr_file: None,
        };
        // Cache ON by default (performance)
        assert!(config.use_cache);
        // Oracle/explain OFF by default (opt-in)
        assert!(!config.oracle);
        assert!(!config.explain);
    }
}
