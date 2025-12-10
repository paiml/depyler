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
mod reporter;
mod state;

pub use cache::{
    CacheConfig, CacheEntry, CacheError, CacheStats, CasStore, CompilationStatus, GcResult,
    SqliteCache, TranspilationCacheKey,
};
pub use classifier::{ErrorCategory, ErrorClassification, ErrorClassifier};
pub use clusterer::{ErrorCluster, ErrorClusterer, RootCause};
pub use compiler::{BatchCompiler, CompilationError, CompilationResult};
pub use reporter::{ConvergenceReporter, IterationReport};
pub use state::{AppliedFix, ConvergenceConfig, ConvergenceState, DisplayMode, ExampleState};

use anyhow::Result;

/// Run the convergence loop to improve compilation rate
///
/// # Arguments
///
/// * `config` - Convergence configuration
///
/// # Returns
///
/// Final convergence state with results
pub async fn run_convergence_loop(config: ConvergenceConfig) -> Result<ConvergenceState> {
    let mut state = ConvergenceState::new(config.clone());
    let compiler = BatchCompiler::new(&config.input_dir).with_display_mode(config.display_mode);
    let classifier = ErrorClassifier::new();
    let clusterer = ErrorClusterer::new();
    let reporter = ConvergenceReporter::with_display_mode(config.display_mode);

    reporter.report_start(&state);

    while state.compilation_rate < config.target_rate && state.iteration < config.max_iterations {
        state.iteration += 1;

        // Step 1: Compile all examples
        let results = compiler.compile_all().await?;
        state.update_examples(&results);

        // Step 2: Classify errors
        let classifications = classifier.classify_all(&results);

        // Step 3: Cluster by root cause
        let clusters = clusterer.cluster(&classifications);
        state.error_clusters = clusters;

        // Step 4: Prioritize and select top cluster
        let top_cluster = state
            .error_clusters
            .iter()
            .max_by(|a, b| {
                a.impact_score()
                    .partial_cmp(&b.impact_score())
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

        if let Some(cluster) = top_cluster {
            reporter.report_iteration(&state, cluster);

            // Step 5: Fix (if auto-fix enabled)
            if config.auto_fix && cluster.fix_confidence >= config.fix_confidence_threshold {
                if let Some(fix) = &cluster.suggested_fix {
                    // Apply fix and verify
                    let applied = fix.apply()?;
                    if applied.verified {
                        state.fixes_applied.push(applied);
                    }
                }
            }
        }

        // Step 6: Update compilation rate
        state.update_compilation_rate();

        // Step 7: Checkpoint (if enabled)
        if let Some(ref checkpoint_dir) = config.checkpoint_dir {
            state.save_checkpoint(checkpoint_dir)?;
        }
    }

    reporter.report_finish(&state);
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
    // Test 6: Batch compiler collects errors
    // --------------------------------------------------------------------------
    #[tokio::test]
    async fn test_gh158_batch_compiler_collects_errors() {
        let temp_dir = TempDir::new().unwrap();
        let example_path = temp_dir.path().join("broken.py");
        std::fs::write(
            &example_path,
            r#"
def broken_function():
    result = undefined_variable  # Will fail
    return result
"#,
        )
        .unwrap();

        let compiler = BatchCompiler::new(temp_dir.path());
        let results = compiler.compile_all().await.unwrap();

        // Should have at least one error
        assert!(!results.is_empty());
        let first = &results[0];
        assert!(!first.errors.is_empty() || !first.success);
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
}
