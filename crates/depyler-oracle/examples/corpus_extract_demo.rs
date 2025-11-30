//! Corpus Extraction Demo
//!
//! Demonstrates the Rust-based corpus extraction API for training data management.
//!
//! # Usage
//!
//! ```bash
//! cargo run --release -p depyler-oracle --example corpus_extract_demo
//! ```

use depyler_oracle::corpus_extract::{TrainingCorpus, TrainingError};

fn main() -> anyhow::Result<()> {
    println!("=== Corpus Extraction Demo ===\n");

    // Create a new corpus
    let mut corpus = TrainingCorpus::new();
    println!("Created empty corpus");

    // Add some training errors
    let errors = vec![
        ("E0308", "mismatched types: expected i32, found String", "type_conversion.py"),
        ("E0382", "use of moved value: `x`", "ownership.py"),
        ("E0433", "failed to resolve: use of undeclared crate or module", "imports.py"),
        ("E0308", "mismatched types: expected i32, found String", "another_file.py"), // Duplicate!
        ("E0599", "no method named `foo` found for type `Bar`", "methods.py"),
    ];

    println!("\nAdding {} errors...", errors.len());
    for (code, msg, file) in &errors {
        let error = TrainingError::new(*code, *msg, "", *file, 0);
        let inserted = corpus.insert(error);
        println!(
            "  {} {} - {}",
            if inserted { "✅" } else { "⏭️ " },
            code,
            if inserted { "added" } else { "duplicate, skipped" }
        );
    }

    println!("\nCorpus size: {} unique errors", corpus.len());

    // Demonstrate hash-based deduplication
    println!("\n--- Hash-Based Deduplication ---");
    let hash1 = TrainingError::compute_hash("E0308", "mismatched types");
    let hash2 = TrainingError::compute_hash("E0308", "mismatched types");
    let hash3 = TrainingError::compute_hash("E0308", "different message");

    println!("Hash('E0308', 'mismatched types'): {}", hash1);
    println!("Hash('E0308', 'mismatched types'): {} (same)", hash2);
    println!("Hash('E0308', 'different message'): {} (different)", hash3);
    println!("Hashes match: {}", hash1 == hash2);

    // Save to temp file
    let temp_dir = std::env::temp_dir();
    let corpus_path = temp_dir.join("demo_corpus.jsonl");

    println!("\n--- Save & Load Roundtrip ---");
    corpus.save(&corpus_path)?;
    println!("Saved to: {}", corpus_path.display());

    let loaded = TrainingCorpus::load(&corpus_path)?;
    println!("Loaded: {} errors", loaded.len());

    // Show corpus contents
    println!("\n--- Corpus Contents ---");
    for (i, error) in loaded.errors().iter().enumerate() {
        println!(
            "  [{}] {} | {} | {}",
            i + 1,
            error.error_code,
            &error.message[..error.message.len().min(40)],
            error.file
        );
    }

    // Demonstrate merge
    println!("\n--- Merge Demo ---");
    let mut corpus2 = TrainingCorpus::new();
    corpus2.insert(TrainingError::new("E0277", "trait bound not satisfied", "", "traits.py", 1));
    corpus2.insert(TrainingError::new("E0308", "mismatched types: expected i32, found String", "", "dup.py", 1)); // Duplicate content

    let mut merged = TrainingCorpus::load(&corpus_path)?;
    let before = merged.len();
    let new_count = merged.merge(corpus2);

    println!("Before merge: {} errors", before);
    println!("After merge: {} errors", merged.len());
    println!("New unique: {} errors", new_count);

    // Cleanup
    std::fs::remove_file(&corpus_path)?;
    println!("\n✅ Demo complete!");

    Ok(())
}
