//! DEPYLER-1318: Train Oracle on Ambiguity Corpus for Dict Key Paradox
//!
//! This example demonstrates training the Oracle model on a targeted synthetic
//! corpus designed to break dictionary type inference (the "Civil War" corpus).
//!
//! The corpus targets the Type System Schism between:
//!   - `type_tokens.rs` (expects `HashMap<String, V>`)
//!   - `type_mapper.rs` (infers `HashMap<DepylerValue, V>`)
//!
//! ## Prerequisites
//!
//! Generate the ambiguity corpus first:
//! ```bash
//! python scripts/generate_ambiguity_corpus.py \
//!     --output training_corpus/ambiguity_v1 \
//!     --count 2000
//! ```
//!
//! Then vectorize failures:
//! ```bash
//! cargo run --bin depyler -- graph vectorize \
//!     --corpus training_corpus/ambiguity_v1 \
//!     --output training_corpus/ambiguity_vectors.ndjson
//! ```
//!
//! ## Usage
//!
//! Train with default settings:
//! ```bash
//! cargo run --release --example train_ambiguity_corpus -p depyler-oracle
//! ```
//!
//! Train with custom output path:
//! ```bash
//! cargo run --release --example train_ambiguity_corpus -p depyler-oracle -- \
//!     --output ~/.depyler/depyler_oracle_v3.23.apr
//! ```
//!
//! ## Expected Output
//!
//! The trained model should achieve:
//! - >95% confidence on TypeMismatch: Dict Key predictions
//! - >14,000 failure vectors from 2,000 hostile Python files
//! - Improved handling of E0308 errors in dictionary-heavy code

use clap::Parser;
use depyler_oracle::unified_training::{build_unified_corpus, UnifiedTrainingConfig};
use std::path::PathBuf;

/// Default paths for the ambiguity corpus
const DEFAULT_VECTORS_PATH: &str = "training_corpus/ambiguity_vectors.ndjson";
const DEFAULT_OUTPUT_PATH: &str = "depyler_oracle_ambiguity.apr";

#[derive(Parser, Debug)]
#[command(name = "train_ambiguity_corpus")]
#[command(about = "Train Oracle model on Dict Key Ambiguity Corpus (DEPYLER-1318)")]
struct Args {
    /// Path to ambiguity vectors (from `depyler graph vectorize`)
    #[arg(long, default_value = DEFAULT_VECTORS_PATH)]
    vectors: PathBuf,

    /// Output path for trained model (.apr format)
    #[arg(short, long, default_value = DEFAULT_OUTPUT_PATH)]
    output: PathBuf,

    /// Random seed for reproducible training
    #[arg(long, default_value = "1318")]
    seed: u64,

    /// Number of additional synthetic samples to generate
    #[arg(long, default_value = "6000")]
    synthetic_samples: usize,

    /// Balance classes by limiting max samples per category
    #[arg(long, default_value = "true")]
    balance: bool,

    /// Maximum samples per class when balancing
    #[arg(long, default_value = "2500")]
    max_per_class: usize,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  DEPYLER-1318: Dict Key Ambiguity Training                 ║");
    println!("║  Target: TypeMismatch: Dict Key with >95% confidence       ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();

    // Verify vectors file exists
    if !args.vectors.exists() {
        eprintln!("Error: Vectors file not found: {}", args.vectors.display());
        eprintln!();
        eprintln!("Generate the ambiguity corpus first:");
        eprintln!("  1. python scripts/generate_ambiguity_corpus.py \\");
        eprintln!("       --output training_corpus/ambiguity_v1 --count 2000");
        eprintln!();
        eprintln!("  2. cargo run --bin depyler -- graph vectorize \\");
        eprintln!("       --corpus training_corpus/ambiguity_v1 \\");
        eprintln!("       --output training_corpus/ambiguity_vectors.ndjson");
        std::process::exit(1);
    }

    println!("Configuration:");
    println!("  Vectors input: {}", args.vectors.display());
    println!("  Model output:  {}", args.output.display());
    println!("  Seed:          {}", args.seed);
    println!("  Synthetic:     {} samples", args.synthetic_samples);
    println!("  Balance:       {}", args.balance);
    if args.balance {
        println!("  Max/class:     {}", args.max_per_class);
    }
    println!();

    // Configure training with focus on ambiguity corpus
    let config = UnifiedTrainingConfig {
        seed: args.seed,
        synthetic_samples: args.synthetic_samples,
        oip_data_path: None,
        real_errors_path: None,
        graph_corpus_path: Some(args.vectors.to_string_lossy().to_string()),
        balance_classes: args.balance,
        max_per_class: if args.balance {
            Some(args.max_per_class)
        } else {
            None
        },
    };

    // Build unified corpus
    println!("Building unified corpus from ambiguity vectors...");
    let result = build_unified_corpus(&config);

    // Print statistics
    println!();
    println!("═══ Training Data Statistics ═══");
    println!("Sources:");
    println!(
        "  Graph corpus: {:>6} samples (ambiguity vectors)",
        result.stats.graph_corpus_count
    );
    println!(
        "  Synthetic:    {:>6} samples (background data)",
        result.stats.synthetic_count
    );
    println!("  Depyler:      {:>6} samples", result.stats.depyler_count);
    println!(
        "  Verificar:    {:>6} samples",
        result.stats.verificar_count
    );
    println!("  ────────────────────────────");
    println!("  Before dedup: {:>6}", result.stats.total_before_dedupe);
    println!("  Duplicates:   {:>6}", result.stats.duplicates_removed);
    println!("  Final count:  {:>6}", result.stats.final_count);
    println!();

    // Category breakdown
    println!("By category:");
    let mut categories: Vec<_> = result.stats.by_category.iter().collect();
    categories.sort_by_key(|(_, count)| std::cmp::Reverse(*count));

    let type_mismatch_count = result
        .stats
        .by_category
        .get(&depyler_oracle::ErrorCategory::TypeMismatch)
        .copied()
        .unwrap_or(0);

    for (category, count) in &categories {
        let marker = if matches!(category, depyler_oracle::ErrorCategory::TypeMismatch) {
            " <-- Target"
        } else {
            ""
        };
        println!("  {:?}: {} samples{}", category, count, marker);
    }
    println!();

    // Train Oracle
    println!("Training Oracle model...");
    let oracle = depyler_oracle::Oracle::load_or_train()?;

    // Save model
    println!("Saving model to: {}", args.output.display());
    oracle.save(&args.output)?;

    // Calculate and report metrics
    let total = result.stats.final_count as f64;
    let type_mismatch_ratio = if total > 0.0 {
        (type_mismatch_count as f64 / total) * 100.0
    } else {
        0.0
    };

    println!();
    println!("═══ Training Complete ═══");
    println!("Model saved:        {}", args.output.display());
    println!("Total samples:      {}", result.stats.final_count);
    println!("TypeMismatch ratio: {:.1}%", type_mismatch_ratio);

    // Success criteria check
    if type_mismatch_count >= 1500 {
        println!();
        println!(
            "✓ Success: TypeMismatch samples ({}) >= 1500 threshold",
            type_mismatch_count
        );
    } else {
        println!();
        println!(
            "⚠ Warning: TypeMismatch samples ({}) < 1500 threshold",
            type_mismatch_count
        );
        println!("  Consider generating more hostile patterns in the corpus");
    }

    Ok(())
}
