//! Depyler-specific training data for error classification.
//!
//! Combines:
//! - Real fixes from DEPYLER-0551 through DEPYLER-0555+
//! - Synthetic patterns from verificar corpus
//!
//! Uses aprender's model evaluation for cross-validation.

use crate::classifier::ErrorCategory;
use crate::training::{TrainingDataset, TrainingSample};
use crate::verificar_integration;

/// Build depyler-specific training dataset from actual fixes.
#[must_use]
pub fn build_depyler_corpus() -> TrainingDataset {
    let mut dataset = TrainingDataset::new();

    // DEPYLER-0551: Error types + PathBuf methods
    add_pathbuf_samples(&mut dataset);

    // DEPYLER-0552: Dict access type inference
    add_dict_inference_samples(&mut dataset);

    // DEPYLER-0553: datetime.datetime chain + instance methods
    add_datetime_samples(&mut dataset);

    // DEPYLER-0554: Function return type + if/else returns
    add_return_type_samples(&mut dataset);

    // DEPYLER-0555: hashlib/file read patterns
    add_file_io_samples(&mut dataset);

    // Type inference: serde_json::Value defaults
    add_type_inference_samples(&mut dataset);

    dataset
}

fn add_pathbuf_samples(dataset: &mut TrainingDataset) {
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0599]: no method named `exists` found for type `String`",
            ErrorCategory::TraitBound,
            "Use std::path::PathBuf::from(&path).exists() instead of String.exists()",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `is_file` found for type `String`",
            ErrorCategory::TraitBound,
            "Convert to PathBuf: std::path::PathBuf::from(&path).is_file()",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `stat` found for type `PathBuf`",
            ErrorCategory::TraitBound,
            "Use path.metadata() instead of path.stat() - Rust uses metadata()",
        ),
        TrainingSample::with_fix(
            "error[E0277]: the trait bound `PathBuf: From<Option<String>>` is not satisfied",
            ErrorCategory::TypeMismatch,
            "Unwrap Option before PathBuf conversion: path.map(PathBuf::from)",
        ),
    ]);
}

fn add_dict_inference_samples(dataset: &mut TrainingDataset) {
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `&String`, found `&&serde_json::Value`",
            ErrorCategory::TypeMismatch,
            "Fix type inference: parameter should be String/&str not serde_json::Value",
        ),
        TrainingSample::with_fix(
            "error[E0277]: the trait bound `String: Borrow<&str>` is not satisfied",
            ErrorCategory::TraitBound,
            "HashMap key type mismatch: use &str or String consistently",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `get` found for enum `serde_json::Value`",
            ErrorCategory::TypeMismatch,
            "Type should be HashMap not Value - fix dict type inference",
        ),
        TrainingSample::with_fix(
            "expected `HashMap<String, String>`, found `HashMap<String, serde_json::Value>`",
            ErrorCategory::TypeMismatch,
            "Dict value type inference: propagate concrete type from usage",
        ),
    ]);
}

fn add_datetime_samples(dataset: &mut TrainingDataset) {
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0433]: failed to resolve: use of undeclared type `DateTime`",
            ErrorCategory::MissingImport,
            "datetime.datetime.fromtimestamp() → chrono::DateTime::from_timestamp()",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `isoformat` found",
            ErrorCategory::TraitBound,
            "dt.isoformat() → dt.to_string() for chrono DateTime",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `strftime` found for struct `NaiveDateTime`",
            ErrorCategory::TraitBound,
            "dt.strftime(fmt) → dt.format(fmt).to_string() for chrono",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `timestamp` found for struct `NaiveDateTime`",
            ErrorCategory::TraitBound,
            "dt.timestamp() → dt.and_utc().timestamp() as f64",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `fromtimestamp`",
            ErrorCategory::MissingImport,
            "datetime.datetime.fromtimestamp → chrono::DateTime::from_timestamp",
        ),
    ]);
}

fn add_return_type_samples(dataset: &mut TrainingDataset) {
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `()`, found `String`",
            ErrorCategory::TypeMismatch,
            "Function missing return type: infer -> String from if/else branches",
        ),
        TrainingSample::with_fix(
            "error[E0308]: mismatched types expected `()`, found `i32`",
            ErrorCategory::TypeMismatch,
            "Function missing return type: add return type annotation",
        ),
        TrainingSample::with_fix(
            "missing `return` keyword in if branch",
            ErrorCategory::SyntaxError,
            "If branches need explicit return when not final expression",
        ),
        TrainingSample::with_fix(
            "error[E0308]: `if` missing an `else` clause",
            ErrorCategory::TypeMismatch,
            "If expression needs else clause for type inference",
        ),
    ]);
}

fn add_file_io_samples(dataset: &mut TrainingDataset) {
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0308]: expected `&mut [u8]`, found integer",
            ErrorCategory::TypeMismatch,
            "Python f.read(8192) → Rust requires buffer: let mut buf = vec![0u8; 8192]",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `hexdigest` found for struct `String`",
            ErrorCategory::TraitBound,
            "hashlib.hexdigest() → use sha2/md5 crate with .finalize() and hex encoding",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `update` found for struct `String`",
            ErrorCategory::TraitBound,
            "hasher.update(chunk) → use Digest trait from sha2 crate",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `is_empty` found for enum `Result`",
            ErrorCategory::TypeMismatch,
            "Walrus operator pattern: while chunk := f.read() needs different Rust idiom",
        ),
    ]);
}

fn add_type_inference_samples(dataset: &mut TrainingDataset) {
    dataset.add_many(vec![
        TrainingSample::with_fix(
            "error[E0606]: casting `&serde_json::Value` as `i64` is invalid",
            ErrorCategory::TypeMismatch,
            "Parameter type should be f64 not Value - infer from cast usage",
        ),
        TrainingSample::with_fix(
            "error[E0308]: expected `f64`, found `&serde_json::Value`",
            ErrorCategory::TypeMismatch,
            "Numeric parameter defaulted to Value - propagate type from arithmetic",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `to_uppercase` found for enum `serde_json::Value`",
            ErrorCategory::TypeMismatch,
            "String method on Value - parameter should be String not Value",
        ),
        TrainingSample::with_fix(
            "error[E0599]: no method named `len` found for reference `&serde_json::Value`",
            ErrorCategory::TypeMismatch,
            "Collection method on Value - infer Vec/String from .len() usage",
        ),
        TrainingSample::with_fix(
            "error[E0599]: the method `join` exists but trait bounds not satisfied",
            ErrorCategory::TraitBound,
            "Vec<Value> should be Vec<String> for join() - propagate element type",
        ),
        TrainingSample::with_fix(
            "error[E0282]: type annotations needed",
            ErrorCategory::TypeMismatch,
            "Insufficient type context - add explicit annotation or infer from usage",
        ),
    ]);
}

/// Build combined corpus from real fixes + synthetic verificar patterns.
#[must_use]
pub fn build_combined_corpus() -> TrainingDataset {
    let mut real = build_depyler_corpus();
    let synthetic = verificar_integration::build_verificar_corpus();

    // Merge synthetic samples into real corpus
    for sample in synthetic.samples() {
        real.add(sample.clone());
    }

    real
}

/// Get error-fix pairs formatted for NgramFixPredictor training.
/// Uses combined corpus (real + synthetic).
#[must_use]
pub fn get_training_pairs() -> Vec<(String, String, ErrorCategory)> {
    build_combined_corpus().error_fix_pairs()
}

/// Get pairs from real fixes only (for evaluation baseline).
#[must_use]
pub fn get_real_training_pairs() -> Vec<(String, String, ErrorCategory)> {
    build_depyler_corpus().error_fix_pairs()
}

/// Category distribution for combined corpus (real + synthetic).
#[must_use]
pub fn corpus_stats() -> Vec<(ErrorCategory, usize)> {
    let dataset = build_combined_corpus();
    vec![
        (ErrorCategory::TypeMismatch, dataset.samples_for_category(ErrorCategory::TypeMismatch).len()),
        (ErrorCategory::TraitBound, dataset.samples_for_category(ErrorCategory::TraitBound).len()),
        (ErrorCategory::MissingImport, dataset.samples_for_category(ErrorCategory::MissingImport).len()),
        (ErrorCategory::BorrowChecker, dataset.samples_for_category(ErrorCategory::BorrowChecker).len()),
        (ErrorCategory::SyntaxError, dataset.samples_for_category(ErrorCategory::SyntaxError).len()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depyler_corpus_not_empty() {
        let corpus = build_depyler_corpus();
        assert!(corpus.len() >= 20, "Corpus should have at least 20 samples");
    }

    #[test]
    fn test_all_samples_have_fixes() {
        let corpus = build_depyler_corpus();
        let pairs = corpus.error_fix_pairs();
        assert_eq!(pairs.len(), corpus.len(), "All samples should have fixes");
    }

    #[test]
    fn test_category_distribution() {
        let stats = corpus_stats();
        let total: usize = stats.iter().map(|(_, c)| c).sum();
        assert!(total >= 20);

        // TypeMismatch should be the largest category (our main issue)
        let type_mismatch_count = stats.iter()
            .find(|(cat, _)| *cat == ErrorCategory::TypeMismatch)
            .map(|(_, c)| *c)
            .unwrap_or(0);
        assert!(type_mismatch_count >= 8, "TypeMismatch should have most samples");
    }

    #[test]
    fn test_training_pairs_format() {
        let pairs = get_training_pairs();
        for (error, fix, _category) in &pairs {
            assert!(!error.is_empty(), "Error should not be empty");
            assert!(!fix.is_empty(), "Fix should not be empty");
        }
    }
}
