//! Depyler Corpus Analysis CLI
//!
//! Deterministic scientific corpus analysis for Python-to-Rust transpilation.

use clap::{Parser, Subcommand, ValueEnum};
use colored::Colorize;
use depyler_corpus::{config::CorpusConfig, CorpusAnalyzer};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "depyler-corpus")]
#[command(about = "Deterministic scientific corpus analysis for Python-to-Rust transpilation")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run full corpus analysis
    Analyze {
        /// Path to the corpus directory
        #[arg(short, long)]
        corpus: Option<PathBuf>,

        /// Output format
        #[arg(short, long, default_value = "terminal")]
        format: OutputFormat,

        /// Output directory for reports
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Skip artifact cleaning phase
        #[arg(long)]
        skip_clean: bool,

        /// Target single-shot compilation rate (0.0-1.0)
        #[arg(long, default_value = "0.8")]
        target_rate: f64,

        /// Path to depyler binary
        #[arg(long)]
        depyler_bin: Option<PathBuf>,
    },

    /// Clean corpus artifacts only (5S methodology)
    Clean {
        /// Path to the corpus directory
        #[arg(short, long)]
        corpus: Option<PathBuf>,

        /// Dry run (show what would be deleted)
        #[arg(long)]
        dry_run: bool,
    },

    /// Show corpus statistics
    Stats {
        /// Path to the corpus directory
        #[arg(short, long)]
        corpus: Option<PathBuf>,
    },
}

#[derive(Clone, ValueEnum)]
enum OutputFormat {
    /// Terminal output with colors
    Terminal,
    /// JSON format
    Json,
    /// Markdown format
    Markdown,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze {
            corpus,
            format,
            output,
            skip_clean,
            target_rate,
            depyler_bin,
        } => {
            let corpus_path = corpus.unwrap_or_else(default_corpus_path);

            println!(
                "{} Corpus analysis for: {}",
                "[DEPYLER-CORPUS]".blue().bold(),
                corpus_path.display()
            );

            let mut config = CorpusConfig::default()
                .with_corpus_path(corpus_path.clone())
                .with_skip_clean(skip_clean)
                .with_target_rate(target_rate);

            if let Some(bin) = depyler_bin {
                config = config.with_depyler_path(bin);
            }

            if let Some(out) = &output {
                config = config.with_output_dir(out.clone());
            }

            let analyzer = CorpusAnalyzer::new(config);

            println!("{} Phase 1: Cleaning artifacts (5S)", "  ->".cyan());
            println!("{} Phase 2: Batch transpilation", "  ->".cyan());
            println!("{} Phase 3: Compilation verification", "  ->".cyan());
            println!("{} Phase 4: Error taxonomy analysis", "  ->".cyan());
            println!("{} Phase 5: Report generation", "  ->".cyan());

            // For now, show a placeholder since we need depyler-core integration
            println!(
                "\n{} Full analysis requires integration with depyler-core.",
                "[INFO]".yellow()
            );
            println!(
                "{} Corpus path: {}",
                "[INFO]".yellow(),
                analyzer.config().corpus_path.display()
            );
            println!(
                "{} Target rate: {:.1}%",
                "[INFO]".yellow(),
                analyzer.config().target_rate
            );

            match format {
                OutputFormat::Terminal => {
                    println!("{} Output: Terminal", "[INFO]".yellow());
                }
                OutputFormat::Json => {
                    println!("{} Output: JSON", "[INFO]".yellow());
                    if let Some(out) = output {
                        println!("{} Output dir: {}", "[INFO]".yellow(), out.display());
                    }
                }
                OutputFormat::Markdown => {
                    println!("{} Output: Markdown", "[INFO]".yellow());
                    if let Some(out) = output {
                        println!("{} Output dir: {}", "[INFO]".yellow(), out.display());
                    }
                }
            }
        }

        Commands::Clean { corpus, dry_run } => {
            let corpus_path = corpus.unwrap_or_else(default_corpus_path);

            println!(
                "{} Cleaning artifacts in: {}",
                "[CLEAN]".blue().bold(),
                corpus_path.display()
            );

            if dry_run {
                println!(
                    "{} Dry run mode - no files will be deleted",
                    "[INFO]".yellow()
                );
            }

            let cleaner = depyler_corpus::cleaner::ArtifactCleaner::new(&corpus_path);

            // Find artifacts
            let rs_files = cleaner.find_rs_files()?;
            let cargo_files = cleaner.find_cargo_tomls()?;
            let target_dirs = cleaner.find_target_dirs()?;

            println!("{} Found {} .rs files", "  ->".cyan(), rs_files.len());
            println!(
                "{} Found {} Cargo.toml files",
                "  ->".cyan(),
                cargo_files.len()
            );
            println!(
                "{} Found {} target/ directories",
                "  ->".cyan(),
                target_dirs.len()
            );

            if !dry_run {
                let summary = cleaner.clean()?;
                println!(
                    "\n{} Cleaned: {} .rs, {} Cargo.toml, {} Cargo.lock, {} target/",
                    "[DONE]".green().bold(),
                    summary.rs_files_removed,
                    summary.cargo_toml_removed,
                    summary.cargo_lock_removed,
                    summary.target_dirs_removed
                );
            }
        }

        Commands::Stats { corpus } => {
            let corpus_path = corpus.unwrap_or_else(default_corpus_path);

            println!(
                "{} Statistics for: {}",
                "[STATS]".blue().bold(),
                corpus_path.display()
            );

            let config = CorpusConfig::default().with_corpus_path(corpus_path);
            let runner = depyler_corpus::transpiler::TranspileRunner::new(&config);

            let python_files = runner.find_python_files()?;
            println!("{} Python files: {}", "  ->".cyan(), python_files.len());

            // Count by pattern
            let cli_files = python_files
                .iter()
                .filter(|p| p.to_string_lossy().contains("_cli.py"))
                .count();
            let test_files = python_files
                .iter()
                .filter(|p| {
                    p.file_name()
                        .is_some_and(|n| n.to_string_lossy().starts_with("test_"))
                })
                .count();
            let tool_files = python_files
                .iter()
                .filter(|p| p.to_string_lossy().contains("_tool.py"))
                .count();

            println!("{} CLI files (*_cli.py): {}", "  ->".cyan(), cli_files);
            println!("{} Test files (test_*.py): {}", "  ->".cyan(), test_files);
            println!("{} Tool files (*_tool.py): {}", "  ->".cyan(), tool_files);
            println!(
                "{} Other files: {}",
                "  ->".cyan(),
                python_files.len() - cli_files - test_files - tool_files
            );
        }
    }

    Ok(())
}

/// Default corpus path (reprorusted-python-cli)
fn default_corpus_path() -> PathBuf {
    // Check common locations
    let candidates = [
        PathBuf::from("../reprorusted-python-cli/examples"),
        PathBuf::from("/home/noah/src/reprorusted-python-cli/examples"),
        PathBuf::from("examples"),
    ];

    for path in &candidates {
        if path.exists() {
            return path.clone();
        }
    }

    // Fallback
    PathBuf::from(".")
}
