//! Unified training pipeline for merging all data sources deterministically.
//!
//! Data sources:
//! 1. **Synthetic corpus**: Generated error patterns (10,000+ samples)
//! 2. **Depyler corpus**: Hand-crafted samples from DEPYLER tickets
//! 3. **Verificar corpus**: Extracted from verificar tool
//! 4. **OIP GitHub corpus**: Mined from Git commit history
//! 5. **Real errors**: Collected from actual compilation failures
//!
//! Pipeline:
//! ```text
//! ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐
//! │  Synthetic  │  │   Depyler   │  │  Verificar  │  │ OIP GitHub  │
//! │   Corpus    │  │   Corpus    │  │   Corpus    │  │   Corpus    │
//! └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘
//!        │                │                │                │
//!        └────────────────┴────────────────┴────────────────┘
//!                                  │
//!                                  ▼
//!                       ┌─────────────────────┐
//!                       │   Merge & Dedupe    │
//!                       │   (by error hash)   │
//!                       └──────────┬──────────┘
//!                                  │
//!                                  ▼
//!                       ┌─────────────────────┐
//!                       │  Deterministic      │
//!                       │  Shuffle (seed=42)  │
//!                       └──────────┬──────────┘
//!                                  │
//!                                  ▼
//!                       ┌─────────────────────┐
//!                       │   Train Oracle      │
//!                       │   (Random Forest)   │
//!                       └──────────┬──────────┘
//!                                  │
//!                                  ▼
//!                       ┌─────────────────────┐
//!                       │  Save Model (.apr)  │
//!                       └─────────────────────┘
//! ```

use crate::classifier::ErrorCategory;
use crate::depyler_training::build_depyler_corpus;
use crate::github_corpus::{load_oip_training_data, convert_oip_to_depyler};
use crate::synthetic::generate_synthetic_corpus_sized;
use crate::training::{TrainingDataset, TrainingSample};
use crate::verificar_integration::build_verificar_corpus;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::path::Path;

/// Configuration for the unified training pipeline.
#[derive(Debug, Clone)]
pub struct UnifiedTrainingConfig {
    /// Random seed for deterministic shuffling
    pub seed: u64,
    /// Number of synthetic samples to generate
    pub synthetic_samples: usize,
    /// Path to OIP training data (optional)
    pub oip_data_path: Option<String>,
    /// Path to real errors file (optional)
    pub real_errors_path: Option<String>,
    /// Whether to balance classes
    pub balance_classes: bool,
    /// Maximum samples per class (for balancing)
    pub max_per_class: Option<usize>,
}

impl Default for UnifiedTrainingConfig {
    fn default() -> Self {
        Self {
            seed: 42,
            synthetic_samples: 12_000,
            oip_data_path: None,
            real_errors_path: None,
            balance_classes: false,
            max_per_class: None,
        }
    }
}

/// Statistics about the merged corpus.
#[derive(Debug, Default)]
pub struct MergeStats {
    pub synthetic_count: usize,
    pub depyler_count: usize,
    pub verificar_count: usize,
    pub oip_count: usize,
    pub real_errors_count: usize,
    pub total_before_dedupe: usize,
    pub duplicates_removed: usize,
    pub final_count: usize,
    pub by_category: HashMap<ErrorCategory, usize>,
}

/// Result of the unified training pipeline.
pub struct UnifiedTrainingResult {
    pub dataset: TrainingDataset,
    pub stats: MergeStats,
}

/// Build a unified corpus from all available data sources.
///
/// This is the main entry point for deterministic training data preparation.
///
/// # Arguments
/// * `config` - Configuration for the training pipeline
///
/// # Returns
/// * `UnifiedTrainingResult` containing the merged dataset and statistics
pub fn build_unified_corpus(config: &UnifiedTrainingConfig) -> UnifiedTrainingResult {
    let mut stats = MergeStats::default();
    let mut all_samples: Vec<TrainingSample> = Vec::new();

    // 1. Synthetic corpus
    let synthetic = generate_synthetic_corpus_sized(config.synthetic_samples);
    stats.synthetic_count = synthetic.samples().len();
    all_samples.extend(synthetic.samples().iter().cloned());

    // 2. Depyler corpus (hand-crafted from tickets)
    let depyler = build_depyler_corpus();
    stats.depyler_count = depyler.samples().len();
    all_samples.extend(depyler.samples().iter().cloned());

    // 3. Verificar corpus
    let verificar = build_verificar_corpus();
    stats.verificar_count = verificar.samples().len();
    all_samples.extend(verificar.samples().iter().cloned());

    // 4. OIP GitHub corpus (if available)
    if let Some(ref oip_path) = config.oip_data_path {
        if let Ok(oip_data) = load_oip_training_data(Path::new(oip_path)) {
            let oip_corpus = convert_oip_to_depyler(&oip_data);
            stats.oip_count = oip_corpus.samples().len();
            all_samples.extend(oip_corpus.samples().iter().cloned());
        }
    }

    // 5. Real errors file (if available)
    if let Some(ref real_path) = config.real_errors_path {
        let real_samples = load_real_errors_file(Path::new(real_path));
        stats.real_errors_count = real_samples.len();
        all_samples.extend(real_samples);
    }

    stats.total_before_dedupe = all_samples.len();

    // Deduplicate by error message hash
    let deduped = deduplicate_samples(all_samples);
    stats.duplicates_removed = stats.total_before_dedupe - deduped.len();

    // Deterministic shuffle
    let shuffled = deterministic_shuffle(deduped, config.seed);

    // Optional class balancing
    let balanced = if config.balance_classes {
        balance_classes(shuffled, config.max_per_class)
    } else {
        shuffled
    };

    // Count by category
    for sample in &balanced {
        *stats.by_category.entry(sample.category).or_default() += 1;
    }

    stats.final_count = balanced.len();

    // Build final dataset
    let mut dataset = TrainingDataset::new();
    for sample in balanced {
        dataset.add(sample);
    }

    UnifiedTrainingResult { dataset, stats }
}

/// Compute hash of a sample for deduplication.
fn sample_hash(sample: &TrainingSample) -> u64 {
    let mut hasher = DefaultHasher::new();
    // Normalize: lowercase, remove extra whitespace
    let normalized = sample.message
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    normalized.hash(&mut hasher);
    hasher.finish()
}

/// Deduplicate samples by error message hash.
fn deduplicate_samples(samples: Vec<TrainingSample>) -> Vec<TrainingSample> {
    let mut seen: HashSet<u64> = HashSet::new();
    let mut result = Vec::new();

    for sample in samples {
        let hash = sample_hash(&sample);
        if seen.insert(hash) {
            result.push(sample);
        }
    }

    result
}

/// Deterministically shuffle samples using a seed.
fn deterministic_shuffle(mut samples: Vec<TrainingSample>, seed: u64) -> Vec<TrainingSample> {
    // Simple LCG-based shuffle for reproducibility
    let n = samples.len();
    if n <= 1 {
        return samples;
    }

    let mut state = seed;
    for i in (1..n).rev() {
        // LCG: state = (a * state + c) mod m
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let j = (state % (i as u64 + 1)) as usize;
        samples.swap(i, j);
    }

    samples
}

/// Balance classes by limiting samples per category.
fn balance_classes(samples: Vec<TrainingSample>, max_per_class: Option<usize>) -> Vec<TrainingSample> {
    let max = max_per_class.unwrap_or(usize::MAX);

    let mut by_category: HashMap<ErrorCategory, Vec<TrainingSample>> = HashMap::new();
    for sample in samples {
        by_category.entry(sample.category).or_default().push(sample);
    }

    let mut result = Vec::new();
    for (_, class_samples) in by_category {
        for sample in class_samples.into_iter().take(max) {
            result.push(sample);
        }
    }

    result
}

/// Load real errors from a file (error code, context, fix per line).
fn load_real_errors_file(path: &Path) -> Vec<TrainingSample> {
    let mut samples = Vec::new();

    if let Ok(content) = std::fs::read_to_string(path) {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Format: ERROR_CODE|context|category|fix
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 3 {
                let error_msg = format!("error[{}]: {}", parts[0], parts[1]);
                let category = parse_category(parts[2]);
                let fix = parts.get(3).map(|s| s.to_string());

                if let Some(fix) = fix {
                    samples.push(TrainingSample::with_fix(&error_msg, category, &fix));
                } else {
                    samples.push(TrainingSample::new(&error_msg, category));
                }
            }
        }
    }

    samples
}

/// Parse error category from string.
fn parse_category(s: &str) -> ErrorCategory {
    match s.trim().to_lowercase().as_str() {
        "typemismatch" | "type_mismatch" | "type" => ErrorCategory::TypeMismatch,
        "borrowchecker" | "borrow_checker" | "borrow" => ErrorCategory::BorrowChecker,
        "missingimport" | "missing_import" | "import" => ErrorCategory::MissingImport,
        "syntaxerror" | "syntax_error" | "syntax" => ErrorCategory::SyntaxError,
        "lifetimeerror" | "lifetime_error" | "lifetime" => ErrorCategory::LifetimeError,
        "traitbound" | "trait_bound" | "trait" => ErrorCategory::TraitBound,
        _ => ErrorCategory::Other,
    }
}

/// Convenience function to build unified corpus with default config.
#[must_use]
pub fn build_default_unified_corpus() -> UnifiedTrainingResult {
    build_unified_corpus(&UnifiedTrainingConfig::default())
}

/// Build unified corpus with OIP data included.
pub fn build_unified_corpus_with_oip(oip_path: &str) -> UnifiedTrainingResult {
    let config = UnifiedTrainingConfig {
        oip_data_path: Some(oip_path.to_string()),
        ..Default::default()
    };
    build_unified_corpus(&config)
}

/// Print merge statistics.
pub fn print_merge_stats(stats: &MergeStats) {
    println!("Unified Corpus Statistics:");
    println!("  Data Sources:");
    println!("    Synthetic:     {:>6} samples", stats.synthetic_count);
    println!("    Depyler:       {:>6} samples", stats.depyler_count);
    println!("    Verificar:     {:>6} samples", stats.verificar_count);
    println!("    OIP GitHub:    {:>6} samples", stats.oip_count);
    println!("    Real Errors:   {:>6} samples", stats.real_errors_count);
    println!("  Merge Results:");
    println!("    Before dedupe: {:>6} samples", stats.total_before_dedupe);
    println!("    Duplicates:    {:>6} removed", stats.duplicates_removed);
    println!("    Final count:   {:>6} samples", stats.final_count);
    println!("  By Category:");
    let mut categories: Vec<_> = stats.by_category.iter().collect();
    categories.sort_by(|a, b| b.1.cmp(a.1));
    for (category, count) in categories {
        let pct = (*count as f64 / stats.final_count as f64) * 100.0;
        println!("    {:?}: {} ({:.1}%)", category, count, pct);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_hash_normalization() {
        let s1 = TrainingSample::new("error[E0308]:   mismatched  types", ErrorCategory::TypeMismatch);
        let s2 = TrainingSample::new("error[E0308]: mismatched types", ErrorCategory::TypeMismatch);
        assert_eq!(sample_hash(&s1), sample_hash(&s2));
    }

    #[test]
    fn test_deduplicate_samples() {
        let samples = vec![
            TrainingSample::new("error[E0308]: type mismatch", ErrorCategory::TypeMismatch),
            TrainingSample::new("error[E0308]: type mismatch", ErrorCategory::TypeMismatch),
            TrainingSample::new("error[E0382]: moved value", ErrorCategory::BorrowChecker),
        ];
        let deduped = deduplicate_samples(samples);
        assert_eq!(deduped.len(), 2);
    }

    #[test]
    fn test_deterministic_shuffle() {
        let samples = vec![
            TrainingSample::new("a", ErrorCategory::TypeMismatch),
            TrainingSample::new("b", ErrorCategory::BorrowChecker),
            TrainingSample::new("c", ErrorCategory::MissingImport),
        ];

        let shuffled1 = deterministic_shuffle(samples.clone(), 42);
        let shuffled2 = deterministic_shuffle(samples.clone(), 42);

        // Same seed produces same order
        for (s1, s2) in shuffled1.iter().zip(shuffled2.iter()) {
            assert_eq!(s1.message, s2.message);
        }
    }

    #[test]
    fn test_balance_classes() {
        let samples = vec![
            TrainingSample::new("a", ErrorCategory::TypeMismatch),
            TrainingSample::new("b", ErrorCategory::TypeMismatch),
            TrainingSample::new("c", ErrorCategory::TypeMismatch),
            TrainingSample::new("d", ErrorCategory::BorrowChecker),
        ];

        let balanced = balance_classes(samples, Some(2));

        // TypeMismatch should be limited to 2
        let type_count = balanced.iter()
            .filter(|s| s.category == ErrorCategory::TypeMismatch)
            .count();
        assert_eq!(type_count, 2);
    }

    #[test]
    fn test_parse_category() {
        assert_eq!(parse_category("TypeMismatch"), ErrorCategory::TypeMismatch);
        assert_eq!(parse_category("type_mismatch"), ErrorCategory::TypeMismatch);
        assert_eq!(parse_category("borrow"), ErrorCategory::BorrowChecker);
        assert_eq!(parse_category("unknown"), ErrorCategory::Other);
    }

    #[test]
    fn test_build_default_unified_corpus() {
        // Use smaller synthetic corpus for test speed
        let config = UnifiedTrainingConfig {
            synthetic_samples: 100,
            ..Default::default()
        };
        let result = build_unified_corpus(&config);

        assert!(result.stats.final_count > 0);
        assert!(result.stats.synthetic_count > 0);
    }
}
