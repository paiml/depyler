//! Unified training pipeline for merging all data sources deterministically.
//!
//! Issue #213: Aligned with aprender::online::corpus::CorpusMerger patterns.
//! Uses similar deduplication, weighting, and provenance tracking.
//!
//! Data sources (mapped to SampleSource-like enum):
//! 1. **Synthetic corpus**: Generated error patterns (SampleSource::Synthetic)
//! 2. **Depyler corpus**: Hand-crafted samples (SampleSource::HandCrafted)
//! 3. **Verificar corpus**: Extracted from verificar (SampleSource::External)
//! 4. **OIP GitHub corpus**: Mined from Git history (SampleSource::Production)
//! 5. **Real errors**: Actual compilation failures (SampleSource::Production)
//!
//! Pipeline (mirrors CorpusMerger flow):
//! ```text
//! ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐
//! │  Synthetic  │  │   Depyler   │  │  Verificar  │  │ OIP GitHub  │
//! │  (weight=1) │  │ (priority=2)│  │ (priority=1)│  │ (priority=0)│
//! └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘
//!        │                │                │                │
//!        └────────────────┴────────────────┴────────────────┘
//!                                  │
//!                                  ▼
//!                       ┌─────────────────────┐
//!                       │  CorpusMerger-style │
//!                       │  Merge & Dedupe     │
//!                       └──────────┬──────────┘
//!                                  │
//!                                  ▼
//!                       ┌─────────────────────┐
//!                       │  Reservoir Sampling │
//!                       │  (seed=42)          │
//!                       └──────────┬──────────┘
//!                                  │
//!                                  ▼
//!                       ┌─────────────────────┐
//!                       │   Train Oracle      │
//!                       └─────────────────────┘
//! ```

use crate::classifier::ErrorCategory;
use crate::depyler_training::build_depyler_corpus;
use crate::github_corpus::{convert_oip_to_depyler, load_oip_training_data};
use crate::synthetic::generate_synthetic_corpus_sized;
use crate::training::{TrainingDataset, TrainingSample};
use crate::verificar_integration::build_verificar_corpus;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::path::Path;

/// Sample source for provenance tracking (mirrors aprender::online::corpus::SampleSource).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TextSampleSource {
    /// Synthetic generated data
    Synthetic,
    /// Hand-crafted training samples (DEPYLER tickets)
    HandCrafted,
    /// External dataset (Verificar)
    External,
    /// Production data (OIP GitHub, real errors)
    #[default]
    Production,
}

/// A corpus source with weight and priority (mirrors aprender::online::corpus::CorpusSource).
#[derive(Debug, Clone)]
pub struct TextCorpusSource {
    /// Source name for provenance
    pub name: String,
    /// Samples from this source
    pub samples: Vec<TrainingSample>,
    /// Weight multiplier (1.0 = normal)
    pub weight: f64,
    /// Priority (higher = prefer in dedup)
    pub priority: u8,
    /// Source type for tracking
    pub source_type: TextSampleSource,
}

impl TextCorpusSource {
    /// Create a new corpus source.
    pub fn new(name: impl Into<String>, samples: Vec<TrainingSample>, source_type: TextSampleSource) -> Self {
        Self {
            name: name.into(),
            samples,
            weight: 1.0,
            priority: 0,
            source_type,
        }
    }

    /// Set weight multiplier.
    #[must_use]
    pub fn with_weight(mut self, weight: f64) -> Self {
        self.weight = weight;
        self
    }

    /// Set priority (higher = prefer in dedup).
    #[must_use]
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }
}

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

/// Provenance tracking for merged corpus (mirrors aprender::online::corpus::CorpusProvenance).
#[derive(Debug, Clone, Default)]
pub struct CorpusProvenance {
    /// Sources and their contributions: (original_count, effective_count)
    pub sources: HashMap<String, (usize, usize)>,
    /// Final merged size
    pub final_size: usize,
    /// Duplicates removed
    pub duplicates_removed: usize,
    /// By source type
    pub by_source_type: HashMap<TextSampleSource, usize>,
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
    /// Provenance tracking (Issue #213)
    pub provenance: CorpusProvenance,
}

/// Text corpus merger (mirrors aprender::online::corpus::CorpusMerger API).
///
/// Merges multiple text-based training sources with configurable weighting,
/// priority-based deduplication, and deterministic shuffling.
#[derive(Debug, Default)]
pub struct TextCorpusMerger {
    sources: Vec<TextCorpusSource>,
    deduplicate: bool,
    shuffle_seed: Option<u64>,
}

impl TextCorpusMerger {
    /// Create a new text corpus merger.
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            deduplicate: true,
            shuffle_seed: Some(42), // Deterministic by default
        }
    }

    /// Add a source to merge.
    pub fn add_source(&mut self, source: TextCorpusSource) -> &mut Self {
        self.sources.push(source);
        self
    }

    /// Set deduplication flag.
    #[must_use]
    pub fn deduplicate(mut self, enable: bool) -> Self {
        self.deduplicate = enable;
        self
    }

    /// Set shuffle seed.
    #[must_use]
    pub fn shuffle_seed(mut self, seed: u64) -> Self {
        self.shuffle_seed = Some(seed);
        self
    }

    /// Merge all sources into unified dataset.
    pub fn merge(&self) -> (Vec<TrainingSample>, CorpusProvenance) {
        let mut provenance = CorpusProvenance::default();
        let mut all_samples: Vec<(TrainingSample, u8, TextSampleSource)> = Vec::new();

        // Collect all samples with priority and source type
        for source in &self.sources {
            let original_count = source.samples.len();
            let effective_count = (original_count as f64 * source.weight).round() as usize;

            provenance.sources.insert(
                source.name.clone(),
                (original_count, effective_count),
            );

            // Apply weight (repeat samples if weight > 1)
            if source.weight >= 1.0 {
                let repeats = source.weight.floor() as usize;
                for sample in &source.samples {
                    for _ in 0..repeats {
                        all_samples.push((sample.clone(), source.priority, source.source_type));
                    }
                }
            } else {
                // Subsample if weight < 1
                let take = (source.samples.len() as f64 * source.weight).round() as usize;
                for sample in source.samples.iter().take(take) {
                    all_samples.push((sample.clone(), source.priority, source.source_type));
                }
            }
        }

        // Sort by priority (higher first) for deduplication
        all_samples.sort_by(|a, b| b.1.cmp(&a.1));

        // Deduplicate
        let mut result = Vec::new();
        let mut duplicates = 0;

        if self.deduplicate {
            let mut seen: HashSet<u64> = HashSet::new();
            for (sample, _, source_type) in all_samples {
                let hash = sample_hash(&sample);
                if seen.insert(hash) {
                    *provenance.by_source_type.entry(source_type).or_default() += 1;
                    result.push(sample);
                } else {
                    duplicates += 1;
                }
            }
        } else {
            for (sample, _, source_type) in all_samples {
                *provenance.by_source_type.entry(source_type).or_default() += 1;
                result.push(sample);
            }
        }

        provenance.duplicates_removed = duplicates;
        provenance.final_size = result.len();

        // Deterministic shuffle
        if let Some(seed) = self.shuffle_seed {
            result = deterministic_shuffle(result, seed);
        }

        (result, provenance)
    }
}

/// Result of the unified training pipeline.
pub struct UnifiedTrainingResult {
    pub dataset: TrainingDataset,
    pub stats: MergeStats,
}

/// Build a unified corpus from all available data sources.
///
/// Issue #213: Now uses TextCorpusMerger (aligned with aprender::online::corpus::CorpusMerger).
///
/// # Arguments
/// * `config` - Configuration for the training pipeline
///
/// # Returns
/// * `UnifiedTrainingResult` containing the merged dataset and statistics
pub fn build_unified_corpus(config: &UnifiedTrainingConfig) -> UnifiedTrainingResult {
    let mut stats = MergeStats::default();
    let mut merger = TextCorpusMerger::new().shuffle_seed(config.seed);

    // 1. Synthetic corpus (lowest priority - will be deduped if conflicts)
    let synthetic = generate_synthetic_corpus_sized(config.synthetic_samples);
    stats.synthetic_count = synthetic.samples().len();
    merger.add_source(
        TextCorpusSource::new("synthetic", synthetic.samples().to_vec(), TextSampleSource::Synthetic)
            .with_priority(0),
    );

    // 2. Depyler corpus (highest priority - hand-crafted from tickets)
    let depyler = build_depyler_corpus();
    stats.depyler_count = depyler.samples().len();
    merger.add_source(
        TextCorpusSource::new("depyler", depyler.samples().to_vec(), TextSampleSource::HandCrafted)
            .with_priority(2),
    );

    // 3. Verificar corpus (medium priority)
    let verificar = build_verificar_corpus();
    stats.verificar_count = verificar.samples().len();
    merger.add_source(
        TextCorpusSource::new("verificar", verificar.samples().to_vec(), TextSampleSource::External)
            .with_priority(1),
    );

    // 4. OIP GitHub corpus (if available)
    if let Some(ref oip_path) = config.oip_data_path {
        if let Ok(oip_data) = load_oip_training_data(Path::new(oip_path)) {
            let oip_corpus = convert_oip_to_depyler(&oip_data);
            stats.oip_count = oip_corpus.samples().len();
            merger.add_source(
                TextCorpusSource::new("oip_github", oip_corpus.samples().to_vec(), TextSampleSource::Production)
                    .with_priority(1),
            );
        }
    }

    // 5. Real errors file (if available)
    if let Some(ref real_path) = config.real_errors_path {
        let real_samples = load_real_errors_file(Path::new(real_path));
        stats.real_errors_count = real_samples.len();
        merger.add_source(
            TextCorpusSource::new("real_errors", real_samples, TextSampleSource::Production)
                .with_priority(2), // High priority for real errors
        );
    }

    // Merge using CorpusMerger-style API
    let (merged_samples, provenance) = merger.merge();

    stats.total_before_dedupe = stats.synthetic_count
        + stats.depyler_count
        + stats.verificar_count
        + stats.oip_count
        + stats.real_errors_count;
    stats.duplicates_removed = provenance.duplicates_removed;
    stats.provenance = provenance;

    // Optional class balancing
    let balanced = if config.balance_classes {
        balance_classes(merged_samples, config.max_per_class)
    } else {
        merged_samples
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

/// Print merge statistics with provenance (Issue #213).
pub fn print_merge_stats(stats: &MergeStats) {
    println!("╭─────────────────────────────────────────────────────╮");
    println!("│          Unified Corpus Statistics                  │");
    println!("├─────────────────────────────────────────────────────┤");
    println!("│  Data Sources:                                      │");
    println!("│    Synthetic:     {:>6} samples                     │", stats.synthetic_count);
    println!("│    Depyler:       {:>6} samples (priority=2)        │", stats.depyler_count);
    println!("│    Verificar:     {:>6} samples (priority=1)        │", stats.verificar_count);
    println!("│    OIP GitHub:    {:>6} samples                     │", stats.oip_count);
    println!("│    Real Errors:   {:>6} samples (priority=2)        │", stats.real_errors_count);
    println!("├─────────────────────────────────────────────────────┤");
    println!("│  Merge Results:                                     │");
    println!("│    Before dedupe: {:>6} samples                     │", stats.total_before_dedupe);
    println!("│    Duplicates:    {:>6} removed                     │", stats.duplicates_removed);
    println!("│    Final count:   {:>6} samples                     │", stats.final_count);
    println!("├─────────────────────────────────────────────────────┤");
    println!("│  By Source Type (Provenance):                       │");
    for (source_type, count) in &stats.provenance.by_source_type {
        let pct = (*count as f64 / stats.final_count.max(1) as f64) * 100.0;
        println!("│    {:?}: {} ({:.1}%)                    │", source_type, count, pct);
    }
    println!("├─────────────────────────────────────────────────────┤");
    println!("│  By Category:                                       │");
    let mut categories: Vec<_> = stats.by_category.iter().collect();
    categories.sort_by(|a, b| b.1.cmp(a.1));
    for (category, count) in categories {
        let pct = (*count as f64 / stats.final_count.max(1) as f64) * 100.0;
        println!("│    {:?}: {} ({:.1}%)             │", category, count, pct);
    }
    println!("╰─────────────────────────────────────────────────────╯");
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================
    // Issue #213: TextCorpusMerger Tests (mirrors CorpusMerger API)
    // ============================================================

    #[test]
    fn test_text_corpus_merger_basic() {
        let samples1 = vec![
            TrainingSample::new("error[E0308]: type mismatch", ErrorCategory::TypeMismatch),
        ];
        let samples2 = vec![
            TrainingSample::new("error[E0382]: moved value", ErrorCategory::BorrowChecker),
        ];

        let mut merger = TextCorpusMerger::new();
        merger.add_source(TextCorpusSource::new("src1", samples1, TextSampleSource::Synthetic));
        merger.add_source(TextCorpusSource::new("src2", samples2, TextSampleSource::HandCrafted));

        let (merged, provenance) = merger.merge();
        assert_eq!(merged.len(), 2);
        assert_eq!(provenance.final_size, 2);
        assert_eq!(provenance.sources.len(), 2);
    }

    #[test]
    fn test_text_corpus_merger_deduplication() {
        let samples1 = vec![
            TrainingSample::new("error[E0308]: type mismatch", ErrorCategory::TypeMismatch),
        ];
        let samples2 = vec![
            TrainingSample::new("error[E0308]: type mismatch", ErrorCategory::TypeMismatch), // Duplicate
        ];

        let mut merger = TextCorpusMerger::new();
        merger.add_source(
            TextCorpusSource::new("high_priority", samples1, TextSampleSource::HandCrafted)
                .with_priority(2)
        );
        merger.add_source(
            TextCorpusSource::new("low_priority", samples2, TextSampleSource::Synthetic)
                .with_priority(0)
        );

        let (merged, provenance) = merger.merge();
        assert_eq!(merged.len(), 1, "Duplicate should be removed");
        assert_eq!(provenance.duplicates_removed, 1);
    }

    #[test]
    fn test_text_corpus_merger_no_deduplication() {
        let samples1 = vec![
            TrainingSample::new("error[E0308]: type mismatch", ErrorCategory::TypeMismatch),
        ];
        let samples2 = vec![
            TrainingSample::new("error[E0308]: type mismatch", ErrorCategory::TypeMismatch),
        ];

        let mut merger = TextCorpusMerger::new().deduplicate(false);
        merger.add_source(TextCorpusSource::new("src1", samples1, TextSampleSource::Synthetic));
        merger.add_source(TextCorpusSource::new("src2", samples2, TextSampleSource::HandCrafted));

        let (merged, provenance) = merger.merge();
        assert_eq!(merged.len(), 2, "Should keep duplicates when dedup disabled");
        assert_eq!(provenance.duplicates_removed, 0);
    }

    #[test]
    fn test_text_corpus_merger_provenance_tracking() {
        let samples = vec![
            TrainingSample::new("a", ErrorCategory::TypeMismatch),
            TrainingSample::new("b", ErrorCategory::BorrowChecker),
        ];

        let mut merger = TextCorpusMerger::new();
        merger.add_source(TextCorpusSource::new("test_source", samples, TextSampleSource::Production));

        let (_, provenance) = merger.merge();

        assert!(provenance.sources.contains_key("test_source"));
        assert_eq!(provenance.sources["test_source"], (2, 2)); // (original, effective)
        assert_eq!(*provenance.by_source_type.get(&TextSampleSource::Production).unwrap_or(&0), 2);
    }

    #[test]
    fn test_text_sample_source_default() {
        assert_eq!(TextSampleSource::default(), TextSampleSource::Production);
    }

    #[test]
    fn test_sample_hash_normalization() {
        let s1 = TrainingSample::new("error[E0308]:   mismatched  types", ErrorCategory::TypeMismatch);
        let s2 = TrainingSample::new("error[E0308]: mismatched types", ErrorCategory::TypeMismatch);
        assert_eq!(sample_hash(&s1), sample_hash(&s2));
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

    // ============================================================
    // TextSampleSource Tests
    // ============================================================

    #[test]
    fn test_text_sample_source_variants() {
        let synthetic = TextSampleSource::Synthetic;
        let hand_crafted = TextSampleSource::HandCrafted;
        let external = TextSampleSource::External;
        let production = TextSampleSource::Production;

        assert!(format!("{:?}", synthetic).contains("Synthetic"));
        assert!(format!("{:?}", hand_crafted).contains("HandCrafted"));
        assert!(format!("{:?}", external).contains("External"));
        assert!(format!("{:?}", production).contains("Production"));
    }

    #[test]
    fn test_text_sample_source_clone() {
        let source = TextSampleSource::Synthetic;
        let cloned = source;
        assert_eq!(source, cloned);
    }

    #[test]
    fn test_text_sample_source_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(TextSampleSource::Synthetic);
        set.insert(TextSampleSource::HandCrafted);
        assert_eq!(set.len(), 2);
    }

    // ============================================================
    // TextCorpusSource Tests
    // ============================================================

    #[test]
    fn test_text_corpus_source_new() {
        let samples = vec![
            TrainingSample::new("error", ErrorCategory::TypeMismatch),
        ];
        let source = TextCorpusSource::new("test", samples, TextSampleSource::Synthetic);

        assert_eq!(source.name, "test");
        assert_eq!(source.samples.len(), 1);
        assert_eq!(source.weight, 1.0);
        assert_eq!(source.priority, 0);
        assert_eq!(source.source_type, TextSampleSource::Synthetic);
    }

    #[test]
    fn test_text_corpus_source_with_weight() {
        let source = TextCorpusSource::new("test", vec![], TextSampleSource::Synthetic)
            .with_weight(2.5);
        assert!((source.weight - 2.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_text_corpus_source_with_priority() {
        let source = TextCorpusSource::new("test", vec![], TextSampleSource::Synthetic)
            .with_priority(5);
        assert_eq!(source.priority, 5);
    }

    #[test]
    fn test_text_corpus_source_chained() {
        let source = TextCorpusSource::new("test", vec![], TextSampleSource::HandCrafted)
            .with_weight(1.5)
            .with_priority(3);

        assert_eq!(source.name, "test");
        assert!((source.weight - 1.5).abs() < f64::EPSILON);
        assert_eq!(source.priority, 3);
    }

    #[test]
    fn test_text_corpus_source_clone() {
        let samples = vec![TrainingSample::new("test", ErrorCategory::Other)];
        let source = TextCorpusSource::new("original", samples, TextSampleSource::Production);
        let cloned = source.clone();

        assert_eq!(source.name, cloned.name);
        assert_eq!(source.samples.len(), cloned.samples.len());
    }

    // ============================================================
    // UnifiedTrainingConfig Tests
    // ============================================================

    #[test]
    fn test_unified_training_config_default() {
        let config = UnifiedTrainingConfig::default();

        assert_eq!(config.seed, 42);
        assert_eq!(config.synthetic_samples, 12_000);
        assert!(config.oip_data_path.is_none());
        assert!(config.real_errors_path.is_none());
        assert!(!config.balance_classes);
        assert!(config.max_per_class.is_none());
    }

    #[test]
    fn test_unified_training_config_custom() {
        let config = UnifiedTrainingConfig {
            seed: 123,
            synthetic_samples: 5000,
            oip_data_path: Some("/path/to/oip".to_string()),
            real_errors_path: Some("/path/to/errors".to_string()),
            balance_classes: true,
            max_per_class: Some(100),
        };

        assert_eq!(config.seed, 123);
        assert_eq!(config.synthetic_samples, 5000);
        assert!(config.oip_data_path.is_some());
        assert!(config.balance_classes);
        assert_eq!(config.max_per_class, Some(100));
    }

    #[test]
    fn test_unified_training_config_clone() {
        let config = UnifiedTrainingConfig::default();
        let cloned = config.clone();
        assert_eq!(config.seed, cloned.seed);
    }

    // ============================================================
    // CorpusProvenance Tests
    // ============================================================

    #[test]
    fn test_corpus_provenance_default() {
        let provenance = CorpusProvenance::default();

        assert!(provenance.sources.is_empty());
        assert_eq!(provenance.final_size, 0);
        assert_eq!(provenance.duplicates_removed, 0);
        assert!(provenance.by_source_type.is_empty());
    }

    #[test]
    fn test_corpus_provenance_clone() {
        let mut provenance = CorpusProvenance::default();
        provenance.sources.insert("test".to_string(), (10, 10));
        provenance.final_size = 10;

        let cloned = provenance.clone();
        assert_eq!(provenance.final_size, cloned.final_size);
        assert_eq!(provenance.sources.len(), cloned.sources.len());
    }

    // ============================================================
    // MergeStats Tests
    // ============================================================

    #[test]
    fn test_merge_stats_default() {
        let stats = MergeStats::default();

        assert_eq!(stats.synthetic_count, 0);
        assert_eq!(stats.depyler_count, 0);
        assert_eq!(stats.verificar_count, 0);
        assert_eq!(stats.oip_count, 0);
        assert_eq!(stats.real_errors_count, 0);
        assert_eq!(stats.total_before_dedupe, 0);
        assert_eq!(stats.duplicates_removed, 0);
        assert_eq!(stats.final_count, 0);
        assert!(stats.by_category.is_empty());
    }

    // ============================================================
    // TextCorpusMerger Additional Tests
    // ============================================================

    #[test]
    fn test_text_corpus_merger_empty() {
        let merger = TextCorpusMerger::new();
        let (merged, provenance) = merger.merge();

        assert!(merged.is_empty());
        assert_eq!(provenance.final_size, 0);
    }

    #[test]
    fn test_text_corpus_merger_shuffle_seed() {
        let samples = vec![
            TrainingSample::new("a", ErrorCategory::TypeMismatch),
            TrainingSample::new("b", ErrorCategory::BorrowChecker),
            TrainingSample::new("c", ErrorCategory::Other),
        ];

        let mut merger1 = TextCorpusMerger::new().shuffle_seed(123);
        merger1.add_source(TextCorpusSource::new("src", samples.clone(), TextSampleSource::Synthetic));

        let mut merger2 = TextCorpusMerger::new().shuffle_seed(123);
        merger2.add_source(TextCorpusSource::new("src", samples, TextSampleSource::Synthetic));

        let (result1, _) = merger1.merge();
        let (result2, _) = merger2.merge();

        // Same seed = same order
        for (s1, s2) in result1.iter().zip(result2.iter()) {
            assert_eq!(s1.message, s2.message);
        }
    }

    #[test]
    fn test_text_corpus_merger_weight_multiply() {
        let samples = vec![
            TrainingSample::new("a", ErrorCategory::TypeMismatch),
        ];

        let mut merger = TextCorpusMerger::new().deduplicate(false);
        merger.add_source(
            TextCorpusSource::new("src", samples, TextSampleSource::Synthetic)
                .with_weight(3.0)
        );

        let (merged, _) = merger.merge();
        assert_eq!(merged.len(), 3); // Weight of 3 = 3 copies
    }

    #[test]
    fn test_text_corpus_merger_weight_subsample() {
        let samples = vec![
            TrainingSample::new("a", ErrorCategory::TypeMismatch),
            TrainingSample::new("b", ErrorCategory::BorrowChecker),
            TrainingSample::new("c", ErrorCategory::Other),
            TrainingSample::new("d", ErrorCategory::SyntaxError),
        ];

        let mut merger = TextCorpusMerger::new();
        merger.add_source(
            TextCorpusSource::new("src", samples, TextSampleSource::Synthetic)
                .with_weight(0.5)
        );

        let (merged, _) = merger.merge();
        assert_eq!(merged.len(), 2); // Weight of 0.5 = half (4 * 0.5 = 2)
    }

    #[test]
    fn test_text_corpus_merger_priority_dedup() {
        // High priority sample should be kept, low priority duplicate removed
        let high_priority = vec![
            TrainingSample::new("error[E0308]: type mismatch", ErrorCategory::TypeMismatch),
        ];
        let low_priority = vec![
            TrainingSample::new("error[E0308]: TYPE MISMATCH", ErrorCategory::TypeMismatch), // Same after normalization
        ];

        let mut merger = TextCorpusMerger::new();
        merger.add_source(
            TextCorpusSource::new("high", high_priority, TextSampleSource::HandCrafted)
                .with_priority(10)
        );
        merger.add_source(
            TextCorpusSource::new("low", low_priority, TextSampleSource::Synthetic)
                .with_priority(1)
        );

        let (merged, provenance) = merger.merge();
        assert_eq!(merged.len(), 1);
        assert_eq!(provenance.duplicates_removed, 1);
    }

    // ============================================================
    // Helper Function Tests
    // ============================================================

    #[test]
    fn test_sample_hash_different_messages() {
        let s1 = TrainingSample::new("first message", ErrorCategory::TypeMismatch);
        let s2 = TrainingSample::new("second message", ErrorCategory::TypeMismatch);
        assert_ne!(sample_hash(&s1), sample_hash(&s2));
    }

    #[test]
    fn test_sample_hash_case_insensitive() {
        let s1 = TrainingSample::new("ERROR MESSAGE", ErrorCategory::TypeMismatch);
        let s2 = TrainingSample::new("error message", ErrorCategory::TypeMismatch);
        assert_eq!(sample_hash(&s1), sample_hash(&s2));
    }

    #[test]
    fn test_deterministic_shuffle_empty() {
        let samples: Vec<TrainingSample> = vec![];
        let result = deterministic_shuffle(samples, 42);
        assert!(result.is_empty());
    }

    #[test]
    fn test_deterministic_shuffle_single() {
        let samples = vec![
            TrainingSample::new("only", ErrorCategory::TypeMismatch),
        ];
        let result = deterministic_shuffle(samples.clone(), 42);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].message, "only");
    }

    #[test]
    fn test_deterministic_shuffle_different_seeds() {
        let samples = vec![
            TrainingSample::new("a", ErrorCategory::TypeMismatch),
            TrainingSample::new("b", ErrorCategory::BorrowChecker),
            TrainingSample::new("c", ErrorCategory::Other),
            TrainingSample::new("d", ErrorCategory::SyntaxError),
        ];

        let result1 = deterministic_shuffle(samples.clone(), 1);
        let result2 = deterministic_shuffle(samples.clone(), 2);

        // Different seeds should produce different orders (most likely)
        let same_order = result1.iter().zip(result2.iter())
            .all(|(a, b)| a.message == b.message);
        // With 4 elements, probability of same order is very low
        assert!(!same_order);
    }

    #[test]
    fn test_balance_classes_no_limit() {
        let samples = vec![
            TrainingSample::new("a", ErrorCategory::TypeMismatch),
            TrainingSample::new("b", ErrorCategory::TypeMismatch),
            TrainingSample::new("c", ErrorCategory::BorrowChecker),
        ];

        let balanced = balance_classes(samples.clone(), None);
        assert_eq!(balanced.len(), 3); // No limit, keep all
    }

    #[test]
    fn test_balance_classes_empty() {
        let samples: Vec<TrainingSample> = vec![];
        let balanced = balance_classes(samples, Some(10));
        assert!(balanced.is_empty());
    }

    #[test]
    fn test_balance_classes_all_same_category() {
        let samples = vec![
            TrainingSample::new("a", ErrorCategory::TypeMismatch),
            TrainingSample::new("b", ErrorCategory::TypeMismatch),
            TrainingSample::new("c", ErrorCategory::TypeMismatch),
            TrainingSample::new("d", ErrorCategory::TypeMismatch),
        ];

        let balanced = balance_classes(samples, Some(2));
        assert_eq!(balanced.len(), 2);
    }

    // ============================================================
    // parse_category Tests
    // ============================================================

    #[test]
    fn test_parse_category_all_variants() {
        // TypeMismatch
        assert_eq!(parse_category("typemismatch"), ErrorCategory::TypeMismatch);
        assert_eq!(parse_category("TypeMismatch"), ErrorCategory::TypeMismatch);
        assert_eq!(parse_category("type"), ErrorCategory::TypeMismatch);

        // BorrowChecker
        assert_eq!(parse_category("borrowchecker"), ErrorCategory::BorrowChecker);
        assert_eq!(parse_category("BorrowChecker"), ErrorCategory::BorrowChecker);
        assert_eq!(parse_category("borrow_checker"), ErrorCategory::BorrowChecker);

        // MissingImport
        assert_eq!(parse_category("missingimport"), ErrorCategory::MissingImport);
        assert_eq!(parse_category("missing_import"), ErrorCategory::MissingImport);
        assert_eq!(parse_category("import"), ErrorCategory::MissingImport);

        // SyntaxError
        assert_eq!(parse_category("syntaxerror"), ErrorCategory::SyntaxError);
        assert_eq!(parse_category("syntax_error"), ErrorCategory::SyntaxError);
        assert_eq!(parse_category("syntax"), ErrorCategory::SyntaxError);

        // LifetimeError
        assert_eq!(parse_category("lifetimeerror"), ErrorCategory::LifetimeError);
        assert_eq!(parse_category("lifetime_error"), ErrorCategory::LifetimeError);
        assert_eq!(parse_category("lifetime"), ErrorCategory::LifetimeError);

        // TraitBound
        assert_eq!(parse_category("traitbound"), ErrorCategory::TraitBound);
        assert_eq!(parse_category("trait_bound"), ErrorCategory::TraitBound);
        assert_eq!(parse_category("trait"), ErrorCategory::TraitBound);
    }

    #[test]
    fn test_parse_category_whitespace() {
        assert_eq!(parse_category("  type  "), ErrorCategory::TypeMismatch);
    }

    #[test]
    fn test_parse_category_unknown() {
        assert_eq!(parse_category("random"), ErrorCategory::Other);
        assert_eq!(parse_category(""), ErrorCategory::Other);
    }

    // ============================================================
    // load_real_errors_file Tests
    // ============================================================

    #[test]
    fn test_load_real_errors_file_nonexistent() {
        let samples = load_real_errors_file(Path::new("/nonexistent/path"));
        assert!(samples.is_empty());
    }

    #[test]
    fn test_load_real_errors_file_temp() {
        use std::io::Write;

        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("errors.txt");

        {
            let mut file = std::fs::File::create(&file_path).unwrap();
            writeln!(file, "# Comment line").unwrap();
            writeln!(file, "").unwrap();
            writeln!(file, "E0308|mismatched types|type|add type annotation").unwrap();
            writeln!(file, "E0382|moved value|borrow").unwrap();
        }

        let samples = load_real_errors_file(&file_path);
        assert_eq!(samples.len(), 2);
        assert!(samples[0].message.contains("E0308"));
        assert_eq!(samples[0].category, ErrorCategory::TypeMismatch);
        assert!(samples[1].message.contains("E0382"));
        assert_eq!(samples[1].category, ErrorCategory::BorrowChecker);
    }

    // ============================================================
    // print_merge_stats Tests
    // ============================================================

    #[test]
    fn test_print_merge_stats_runs_without_error() {
        let mut stats = MergeStats::default();
        stats.synthetic_count = 100;
        stats.depyler_count = 50;
        stats.verificar_count = 30;
        stats.final_count = 180;
        stats.by_category.insert(ErrorCategory::TypeMismatch, 100);
        stats.by_category.insert(ErrorCategory::BorrowChecker, 80);
        stats.provenance.by_source_type.insert(TextSampleSource::Synthetic, 100);
        stats.provenance.by_source_type.insert(TextSampleSource::HandCrafted, 80);

        // Just ensure it doesn't panic
        print_merge_stats(&stats);
    }

    #[test]
    fn test_print_merge_stats_empty() {
        let stats = MergeStats::default();
        // Should not panic with zeros
        print_merge_stats(&stats);
    }

    // ============================================================
    // build_unified_corpus_with_oip Tests
    // ============================================================

    #[test]
    fn test_build_unified_corpus_with_oip_nonexistent() {
        // Should gracefully handle missing file
        let result = build_unified_corpus_with_oip("/nonexistent/oip/path");
        assert!(result.stats.oip_count == 0);
    }

    // ============================================================
    // UnifiedTrainingResult Tests
    // ============================================================

    #[test]
    fn test_unified_training_result_structure() {
        let config = UnifiedTrainingConfig {
            synthetic_samples: 50,
            balance_classes: true,
            max_per_class: Some(10),
            ..Default::default()
        };
        let result = build_unified_corpus(&config);

        assert!(result.stats.final_count > 0);
        // With balancing enabled, categories should be limited
    }
}
