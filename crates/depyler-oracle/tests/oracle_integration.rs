//! Integration tests for the Self-Supervised Oracle Pipeline.
//!
//! These tests validate the end-to-end corpus generation, optimization,
//! and autofixer training workflow per Phase 2 spec.

use aprender::synthetic::{SyntheticConfig, SyntheticGenerator};
use depyler_oracle::self_supervised::{
    run_optimization, CorpusConfig, CorpusFixPredictor, CurriculumScheduler, DifficultyLevel,
    EvaluationConfig, EvaluationMetrics, Evaluator, GenerationParams, OptimizationRunConfig,
    PyType, PythonExampleGenerator, SelfSupervisedCorpusGenerator, StdlibFunction, TranspileResult,
};
use depyler_oracle::{ErrorCategory, TrainingDataset, TrainingSample};

// ============================================================================
// Phase 2.2: Integration Tests (EXTREME TDD - RED PHASE)
// ============================================================================

/// Sample stdlib functions for testing.
fn sample_stdlib_functions() -> Vec<StdlibFunction> {
    vec![
        StdlibFunction {
            module: "os.path".to_string(),
            name: "join".to_string(),
            signature: "(path, *paths) -> str".to_string(),
            arg_types: vec![PyType::Str, PyType::Str],
            return_type: Some(PyType::Str),
            docstring_examples: vec!["os.path.join('/home', 'user')".to_string()],
        },
        StdlibFunction {
            module: "os.path".to_string(),
            name: "exists".to_string(),
            signature: "(path) -> bool".to_string(),
            arg_types: vec![PyType::Str],
            return_type: Some(PyType::Bool),
            docstring_examples: vec!["os.path.exists('/tmp')".to_string()],
        },
        StdlibFunction {
            module: "json".to_string(),
            name: "loads".to_string(),
            signature: "(s) -> Any".to_string(),
            arg_types: vec![PyType::Str],
            return_type: Some(PyType::Any),
            docstring_examples: vec!["json.loads('{\"key\": \"value\"}')".to_string()],
        },
    ]
}

// ============================================================================
// Test 1: End-to-End Corpus Generation
// ============================================================================

#[test]
fn test_end_to_end_corpus_generation() {
    // 1. Parse stdlib functions (simulated)
    let stdlib_funcs = sample_stdlib_functions();
    assert!(stdlib_funcs.len() >= 3, "Need at least 3 stdlib functions");

    // 2. Generate examples using PythonExampleGenerator
    let generator = PythonExampleGenerator::new(stdlib_funcs.clone());
    let config = SyntheticConfig::default();
    let examples = generator
        .generate(&stdlib_funcs, &config)
        .expect("Generation should succeed");

    // Should generate examples (at least some)
    assert!(!examples.is_empty(), "Should generate at least one example");

    // 3. Create corpus generator and add results
    let mut corpus_gen =
        SelfSupervisedCorpusGenerator::new(stdlib_funcs.clone(), CorpusConfig::default());

    // Simulate transpilation results
    for (i, example) in examples.iter().take(10).enumerate() {
        // Mock: 80% success rate
        let success = i % 5 != 0;
        let result = TranspileResult {
            python_source: example.source.clone(),
            rust_output: if success {
                Some("fn foo() {}".to_string())
            } else {
                None
            },
            transpile_error: if success {
                None
            } else {
                Some("mock error".to_string())
            },
            compile_errors: vec![],
            content_hash: i as u64,
        };
        corpus_gen.add_result(&result);
    }

    // 4. Verify metrics
    let metrics = corpus_gen.metrics();
    assert!(
        metrics.acceptance_rate() >= 0.0 && metrics.acceptance_rate() <= 1.0,
        "Acceptance rate should be valid"
    );
}

// ============================================================================
// Test 2: Optimizer Execution
// ============================================================================

#[test]
fn test_optimizer_execution_integration() {
    let stdlib_funcs = sample_stdlib_functions();

    // Configure for fast test execution
    let config = OptimizationRunConfig {
        eval_stdlib_count: 2,
        eval_samples: 5,
        max_evaluations: 20,
        use_curriculum: false,
    };

    // Run optimization
    let result = run_optimization(&stdlib_funcs, &config);

    // Verify result structure
    assert!(result.fitness >= 0.0, "Fitness should be non-negative");
    assert!(result.evaluations > 0, "Should have evaluations");
    assert!(
        result.params.quality_threshold >= 0.1,
        "Quality threshold should be valid"
    );
}

#[test]
fn test_optimizer_with_curriculum_integration() {
    let stdlib_funcs = sample_stdlib_functions();

    // Enable curriculum learning
    let config = OptimizationRunConfig {
        eval_stdlib_count: 2,
        eval_samples: 5,
        max_evaluations: 20,
        use_curriculum: true,
    };

    let result = run_optimization(&stdlib_funcs, &config);

    // Curriculum should produce valid results
    assert!(result.fitness >= 0.0, "Fitness should be non-negative");
    assert!(
        !result.history.is_empty(),
        "Should have optimization history"
    );
}

// ============================================================================
// Test 3: Autofixer Training Integration
// ============================================================================

#[test]
fn test_autofixer_training_from_corpus() {
    // 1. Create training dataset
    let mut dataset = TrainingDataset::new();

    // Add samples covering multiple error categories
    dataset.add(TrainingSample {
        message: "mismatched types: expected `i32`, found `String`".to_string(),
        category: ErrorCategory::TypeMismatch,
        fix: Some("use .parse::<i32>() to convert".to_string()),
    });

    dataset.add(TrainingSample {
        message: "cannot borrow `x` as mutable, as it is not declared as mutable".to_string(),
        category: ErrorCategory::BorrowChecker,
        fix: Some("declare with `let mut x`".to_string()),
    });

    dataset.add(TrainingSample {
        message: "cannot find value `HashMap` in this scope".to_string(),
        category: ErrorCategory::MissingImport,
        fix: Some("add `use std::collections::HashMap;`".to_string()),
    });

    dataset.add(TrainingSample {
        message: "lifetime may not live long enough".to_string(),
        category: ErrorCategory::LifetimeError,
        fix: Some("add lifetime parameter 'a".to_string()),
    });

    dataset.add(TrainingSample {
        message: "the trait `Clone` is not implemented for `MyStruct`".to_string(),
        category: ErrorCategory::TraitBound,
        fix: Some("derive or implement Clone".to_string()),
    });

    // 2. Train autofixer predictor
    let mut predictor = CorpusFixPredictor::new();
    predictor.train_from_corpus(&dataset);

    // 3. Verify patterns were extracted
    assert!(
        predictor.pattern_count() >= 5,
        "Should extract patterns from all categories"
    );

    // 4. Test predictions
    let type_fix = predictor.predict(ErrorCategory::TypeMismatch);
    assert!(type_fix.is_some(), "Should predict TypeMismatch fix");

    let borrow_fix = predictor.predict(ErrorCategory::BorrowChecker);
    assert!(borrow_fix.is_some(), "Should predict BorrowChecker fix");
}

// ============================================================================
// Test 4: Curriculum Progression
// ============================================================================

#[test]
fn test_curriculum_progression_integration() {
    let samples_per_level = 10;
    let mut scheduler = CurriculumScheduler::new(samples_per_level);

    // Track progression
    let mut levels_visited = vec![scheduler.current_level()];

    // Simulate generating samples through all difficulty levels
    for _ in 0..50 {
        scheduler.record_sample();

        if scheduler.try_advance() {
            levels_visited.push(scheduler.current_level());
        }
    }

    // Should progress through all 4 levels
    assert!(
        levels_visited.contains(&DifficultyLevel::Basic),
        "Should visit Basic"
    );
    assert!(
        levels_visited.contains(&DifficultyLevel::Intermediate),
        "Should visit Intermediate"
    );
    assert!(
        levels_visited.contains(&DifficultyLevel::Advanced),
        "Should visit Advanced"
    );
    assert!(
        levels_visited.contains(&DifficultyLevel::Expert),
        "Should visit Expert"
    );
}

// ============================================================================
// Test 5: Evaluation Metrics Integration
// ============================================================================

#[test]
fn test_evaluation_metrics_integration() {
    let config = EvaluationConfig::default();
    let mut evaluator = Evaluator::new(config);

    // Add benchmark results from different configurations
    evaluator.add_result(depyler_oracle::self_supervised::BenchmarkResult::new(
        "Default Params",
        GenerationParams::default(),
        EvaluationMetrics {
            corpus_size: 500,
            uniqueness_rate: 0.95,
            class_balance: 0.85,
            category_coverage: 0.85,
            diversity_score: 0.75,
            estimated_accuracy: 0.88,
            macro_f1: 0.86,
        },
        10.0,
        5.0,
    ));

    evaluator.add_result(depyler_oracle::self_supervised::BenchmarkResult::new(
        "Optimized Params",
        GenerationParams::default(),
        EvaluationMetrics {
            corpus_size: 1000,
            uniqueness_rate: 0.98,
            class_balance: 0.90,
            category_coverage: 0.95,
            diversity_score: 0.85,
            estimated_accuracy: 0.92,
            macro_f1: 0.90,
        },
        15.0,
        8.0,
    ));

    // Best result should be "Optimized Params"
    let best = evaluator.best_result().expect("Should have best result");
    assert_eq!(best.name, "Optimized Params");

    // Check improvement over baseline
    assert!(
        evaluator.improves_over_baseline(&best.metrics),
        "Optimized should improve over baseline"
    );

    // Generate summary report
    let report = evaluator.summary_report();
    assert!(report.contains("Optimized Params"));
    assert!(report.contains("92.00%")); // Accuracy
}

// ============================================================================
// Test 6: Full Pipeline Integration (Slow - runs actual optimization)
// ============================================================================

#[test]
#[ignore] // Enable with --ignored for full integration test
fn test_full_pipeline_integration() {
    let stdlib_funcs = sample_stdlib_functions();

    // Full optimization run
    let config = OptimizationRunConfig {
        eval_stdlib_count: 5,
        eval_samples: 50,
        max_evaluations: 100,
        use_curriculum: true,
    };

    let result = run_optimization(&stdlib_funcs, &config);

    // Should find reasonable parameters
    assert!(result.fitness > 0.3, "Should achieve >30% fitness");

    // Use optimized params to generate corpus
    let generator = PythonExampleGenerator::new(stdlib_funcs.clone());
    let examples = generator
        .generate(&stdlib_funcs, &SyntheticConfig::default())
        .expect("Generation should succeed");

    // Corpus should be non-empty
    assert!(!examples.is_empty());

    // Train autofixer from generated examples
    let mut dataset = TrainingDataset::new();
    for (i, example) in examples.iter().enumerate().take(20) {
        dataset.add(TrainingSample {
            message: format!("error from {}", example.target_function),
            category: if i % 2 == 0 {
                ErrorCategory::TypeMismatch
            } else {
                ErrorCategory::BorrowChecker
            },
            fix: Some(format!("fix for {}", example.target_function)),
        });
    }

    let mut predictor = CorpusFixPredictor::new();
    predictor.train_from_corpus(&dataset);

    assert!(
        predictor.pattern_count() > 0,
        "Should extract patterns from corpus"
    );
}
