//! CLI binary for corpus extraction
//!
//! Replaces bash script with type-safe Rust implementation.
//! Uses the corpus_extract module (TDD-developed).

use clap::Parser;
use depyler_oracle::corpus_extract::{TrainingCorpus, TrainingError};
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(name = "extract-training-data")]
#[command(about = "Extract and deduplicate training errors from transpilation")]
struct Args {
    /// Input directory with Python files
    #[arg(short, long, default_value = "target/verificar/corpus")]
    input_dir: PathBuf,

    /// Output directory for transpiled Rust files
    #[arg(short, long, default_value = "target/verificar/output")]
    output_dir: PathBuf,

    /// Main corpus file (JSONL)
    #[arg(short, long, default_value = "training_corpus/errors.jsonl")]
    corpus: PathBuf,

    /// Cycle number (for accumulation mode)
    #[arg(long, default_value = "0")]
    cycle: u32,

    /// Maximum files to process
    #[arg(long, default_value = "500")]
    max_files: usize,

    /// Depyler binary path
    #[arg(long, default_value = "target/release/depyler")]
    depyler: PathBuf,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!("🔍 Extracting training errors (Rust implementation)...");
    println!("   Input: {}", args.input_dir.display());
    println!("   Output: {}", args.output_dir.display());
    println!("   Corpus: {}", args.corpus.display());
    println!();

    // Ensure directories exist
    std::fs::create_dir_all(&args.output_dir)?;
    if let Some(parent) = args.corpus.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Load existing corpus (for deduplication)
    let mut corpus = TrainingCorpus::load(&args.corpus)?;
    let before_count = corpus.len();
    println!("📊 Existing corpus: {} unique errors", before_count);

    // Find Python files
    let py_files: Vec<PathBuf> = find_python_files(&args.input_dir, args.max_files)?;
    println!("📊 Found {} Python files to process", py_files.len());
    println!();

    let mut stats = ExtractionStats::default();

    for (i, py_file) in py_files.iter().enumerate() {
        let name = py_file
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        let rs_file = args.output_dir.join(format!("{}.rs", name));

        // Try to transpile
        let transpile_result = Command::new(&args.depyler)
            .args(["transpile", &py_file.to_string_lossy(), "-o", &rs_file.to_string_lossy()])
            .output();

        match transpile_result {
            Ok(output) if output.status.success() => {
                stats.transpile_success += 1;

                // Try to compile the generated Rust
                let compile_result = Command::new("rustc")
                    .args(["--edition", "2021", "--crate-type", "lib", &rs_file.to_string_lossy(), "-o", "/dev/null"])
                    .output();

                match compile_result {
                    Ok(output) if !output.status.success() => {
                        let error_msg = String::from_utf8_lossy(&output.stderr);
                        if !error_msg.is_empty() {
                            stats.compile_fail += 1;

                            let category = classify_error(&error_msg);
                            let truncated_error = truncate_error(&error_msg, 500);

                            let error = TrainingError::new(
                                extract_error_code(&error_msg),
                                truncated_error,
                                "", // context
                                py_file.to_string_lossy(),
                                args.cycle,
                            );

                            if corpus.insert(error) {
                                stats.errors_harvested += 1;
                                if args.verbose {
                                    println!("  ✓ {} -> {} ({})", name, category, stats.errors_harvested);
                                }
                            }
                        }
                    }
                    Ok(_) => stats.compile_success += 1,
                    Err(_) => stats.compile_fail += 1,
                }
            }
            _ => stats.transpile_fail += 1,
        }

        // Progress indicator
        if (i + 1) % 50 == 0 {
            println!(
                "   Processed {}/{} files, {} new errors...",
                i + 1,
                py_files.len(),
                stats.errors_harvested
            );
        }
    }

    // Save updated corpus
    corpus.save(&args.corpus)?;

    println!();
    println!("=== Extraction Complete ===");
    println!();
    println!("📊 Results:");
    println!("   Files processed: {}", stats.transpile_success + stats.transpile_fail);
    println!("   Transpile success: {}", stats.transpile_success);
    println!("   Transpile fail: {}", stats.transpile_fail);
    println!("   Compile success: {}", stats.compile_success);
    println!("   Compile fail: {}", stats.compile_fail);
    println!();
    println!("🎯 Errors harvested: {} new unique", stats.errors_harvested);
    println!("   Corpus before: {}", before_count);
    println!("   Corpus after: {}", corpus.len());
    println!();

    Ok(())
}

#[derive(Default)]
struct ExtractionStats {
    transpile_success: usize,
    transpile_fail: usize,
    compile_success: usize,
    compile_fail: usize,
    errors_harvested: usize,
}

fn find_python_files(dir: &PathBuf, max: usize) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    if dir.is_dir() {
        for entry in walkdir::WalkDir::new(dir)
            .max_depth(5)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.path().extension().map(|e| e == "py").unwrap_or(false) {
                files.push(entry.path().to_path_buf());
                if files.len() >= max {
                    break;
                }
            }
        }
    }

    Ok(files)
}

fn classify_error(error: &str) -> &'static str {
    if error.contains("cannot borrow") {
        "BorrowChecker"
    } else if error.contains("lifetime") {
        "LifetimeError"
    } else if error.contains("expected") && error.contains("found") || error.contains("mismatched") {
        "TypeMismatch"
    } else if error.contains("cannot find") {
        "MissingImport"
    } else if error.contains("trait") && error.contains("not implemented") {
        "TraitBound"
    } else if error.contains("syntax") {
        "SyntaxError"
    } else {
        "Other"
    }
}

fn extract_error_code(error: &str) -> String {
    // Extract error code like E0308, E0599, etc.
    let re = regex::Regex::new(r"E\d{4}").ok();
    re.and_then(|r| r.find(error).map(|m| m.as_str().to_string()))
        .unwrap_or_else(|| "UNKNOWN".to_string())
}

fn truncate_error(error: &str, max_len: usize) -> String {
    let single_line = error.lines().take(20).collect::<Vec<_>>().join(" ");
    if single_line.len() > max_len {
        single_line[..max_len].to_string()
    } else {
        single_line
    }
}
