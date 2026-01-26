//! DEPYLER-ORACLE-TRAIN: User corpus training example.
//!
//! Demonstrates the Oracle's user training feedback loop:
//! 1. Load existing user model (if any)
//! 2. Learn patterns from error→fix pairs
//! 3. Save updated model for future runs
//!
//! Usage:
//!   cargo run --example oracle_user_training -p depyler-oracle

use depyler_oracle::ngram::NgramFixPredictor;
use depyler_oracle::ErrorCategory;

fn main() -> anyhow::Result<()> {
    println!("=== Depyler Oracle User Training Demo ===\n");

    // Step 1: Create predictor and load existing user model
    let mut predictor = NgramFixPredictor::new();
    let model_path = NgramFixPredictor::default_user_model_path();

    println!("Model path: {}", model_path.display());

    match predictor.load(&model_path) {
        Ok(()) => println!("Loaded existing model ({} patterns)", predictor.pattern_count()),
        Err(e) => println!("No existing model: {}", e),
    }

    let initial_count = predictor.pattern_count();

    // Step 2: Simulate learning from converge fixes
    // These represent error→fix pairs discovered during convergence
    let training_pairs = [
        // E0308: Type mismatch errors
        (
            "error[E0308]: mismatched types - expected `i32`, found `&str`",
            "Use .parse::<i32>() to convert string to integer",
            ErrorCategory::TypeMismatch,
        ),
        (
            "error[E0308]: mismatched types - expected `String`, found `&str`",
            "Use .to_string() to convert &str to String",
            ErrorCategory::TypeMismatch,
        ),
        (
            "error[E0308]: mismatched types - expected `&str`, found `String`",
            "Use .as_str() or &value to borrow String as &str",
            ErrorCategory::TypeMismatch,
        ),
        // E0382: Borrow checker errors
        (
            "error[E0382]: borrow of moved value",
            "Use .clone() before the move, or restructure to avoid double use",
            ErrorCategory::BorrowChecker,
        ),
        (
            "error[E0502]: cannot borrow as mutable because also borrowed as immutable",
            "Separate the immutable borrow scope from the mutable borrow",
            ErrorCategory::BorrowChecker,
        ),
        // E0433: Missing imports
        (
            "error[E0433]: failed to resolve: use of undeclared crate or module",
            "Add `use` statement or check Cargo.toml dependencies",
            ErrorCategory::MissingImport,
        ),
        // E0599: Method not found (trait bound)
        (
            "error[E0599]: no method named `iter` found for type `HashMap`",
            "Use .iter() for references, .into_iter() for owned values",
            ErrorCategory::TraitBound,
        ),
    ];

    println!("\nLearning {} error→fix patterns...", training_pairs.len());

    for (error_msg, fix_template, category) in &training_pairs {
        predictor.learn_pattern(error_msg, fix_template, *category);
        println!("  Learned: {:?} → {}", category, &fix_template[..40.min(fix_template.len())]);
    }

    // Step 3: Fit the vectorizer for similarity matching
    predictor.fit()?;

    // Step 4: Test prediction on similar errors
    println!("\n=== Testing Predictions ===\n");

    let test_errors = [
        "error[E0308]: mismatched types - expected `u64`, found `&str`",
        "error[E0382]: use of moved value: `data`",
        "error[E0433]: failed to resolve: use of undeclared type `HashMap`",
    ];

    for test_error in &test_errors {
        println!("Query: {}", &test_error[..60.min(test_error.len())]);
        let suggestions = predictor.predict_fixes(test_error, 2);

        if suggestions.is_empty() {
            println!("  No suggestions (min_similarity threshold not met)\n");
        } else {
            for (i, s) in suggestions.iter().enumerate() {
                println!(
                    "  {}. [{:.2}] {:?}: {}",
                    i + 1,
                    s.confidence,
                    s.category,
                    &s.fix[..50.min(s.fix.len())]
                );
            }
            println!();
        }
    }

    // Step 5: Save updated model
    println!("=== Saving Model ===\n");

    predictor.save(&model_path)?;
    println!(
        "Saved {} patterns to {}",
        predictor.pattern_count(),
        model_path.display()
    );
    println!(
        "New patterns this session: {}",
        predictor.pattern_count() - initial_count
    );

    // Step 6: Show statistics by category
    println!("\n=== Pattern Statistics ===\n");

    for category in [
        ErrorCategory::TypeMismatch,
        ErrorCategory::BorrowChecker,
        ErrorCategory::MissingImport,
        ErrorCategory::LifetimeError,
        ErrorCategory::TraitBound,
        ErrorCategory::Other,
    ] {
        let patterns = predictor.patterns_for_category(category);
        if !patterns.is_empty() {
            println!("{:?}: {} patterns", category, patterns.len());
        }
    }

    println!("\n=== Training Complete ===");
    println!("Future converge runs will load and apply these patterns.");

    Ok(())
}
