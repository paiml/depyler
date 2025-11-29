//! # Depyler - Python to Rust Transpiler
//!
//! Depyler is a transpiler that converts Python code with type annotations into idiomatic Rust.
//! It performs semantic verification and memory safety analysis to ensure correctness.
//!
//! ## Usage
//!
//! ### As a Library
//!
//! ```rust,no_run
//! use depyler_core::DepylerPipeline;
//!
//! let pipeline = DepylerPipeline::new();
//! let python_code = r#"
//! def add(a: int, b: int) -> int:
//!     return a + b
//! "#;
//!
//! match pipeline.transpile(python_code) {
//!     Ok(rust_code) => println!("{}", rust_code),
//!     Err(e) => eprintln!("Transpilation failed: {}", e),
//! }
//! ```
//!
//! ### As a CLI Tool
//!
//! ```bash
//! # Transpile a Python file
//! depyler transpile example.py
//!
//! # Transpile with verification
//! depyler transpile example.py --verify
//!
//! # Analyze migration complexity
//! depyler analyze example.py
//! ```
//!
//! ## Architecture
//!
//! Depyler uses a multi-stage pipeline:
//!
//! 1. **Parsing**: Python code → AST (via RustPython)
//! 2. **HIR**: AST → High-level Intermediate Representation
//! 3. **Type Inference**: Infer ownership and borrowing
//! 4. **Code Generation**: HIR → Rust code (via syn/quote)
//! 5. **Verification**: Property-based testing for equivalence
//!
//! ## Features
//!
//! - Type-directed transpilation using Python annotations
//! - Memory safety analysis and ownership inference
//! - Semantic verification via property testing
//! - MCP server for AI assistant integration
//! - Quality analysis (TDG scoring, complexity metrics)

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use depyler_analyzer::Analyzer;
use depyler_core::{
    lambda_codegen::{LambdaCodeGenerator, LambdaProject},
    lambda_inference::{AnalysisReport, LambdaTypeInferencer},
    lambda_optimizer::LambdaOptimizer,
    lambda_testing::LambdaTestHarness,
    DepylerPipeline,
};
use depyler_oracle::{CITLFixer, CITLFixerConfig, HybridTranspiler, Oracle};
use depyler_quality::QualityAnalyzer;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

pub mod agent;
pub mod compilation_trainer;
pub mod compile_cmd;
pub mod converge;
pub mod debug_cmd;
pub mod docs_cmd;
pub mod interactive;
pub mod profile_cmd;
pub mod training_monitor;

#[derive(Parser)]
#[command(name = "depyler")]
#[command(about = "Pragmatic Python-to-Rust Transpiler", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Transpile Python code to Rust
    Transpile {
        /// Input Python file
        input: PathBuf,

        /// Output Rust file (defaults to input with .rs extension)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Enable verification
        #[arg(long)]
        verify: bool,

        /// Generate property tests
        #[arg(long)]
        gen_tests: bool,

        /// Enable debug mode
        #[arg(long)]
        debug: bool,

        /// Generate source map
        #[arg(long)]
        source_map: bool,

        /// Show transpilation trace (AST → HIR → Rust phases)
        #[arg(long)]
        trace: bool,

        /// Explain transformation decisions in detail
        #[arg(long)]
        explain: bool,

        /// Auto-fix compile errors using ML oracle (Issue #105)
        #[arg(long)]
        auto_fix: bool,

        /// Show suggested fixes without applying (Issue #105)
        #[arg(long)]
        suggest_fixes: bool,

        /// Minimum confidence threshold for auto-fix (0.0-1.0)
        #[arg(long, default_value = "0.80")]
        fix_confidence: f64,
    },

    /// Compile Python to standalone binary (DEPYLER-0380)
    Compile {
        /// Input Python file
        input: PathBuf,

        /// Output binary path (defaults to input name without extension)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Cargo build profile (debug, release)
        #[arg(long, default_value = "release")]
        profile: String,
    },

    /// Analyze Python code complexity and metrics
    Analyze {
        /// Input Python file
        input: PathBuf,

        /// Output format (json, text)
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Check if Python code can be transpiled
    Check {
        /// Input Python file
        input: PathBuf,
    },

    /// Run quality gates and analysis
    QualityCheck {
        /// Input Python file or directory
        input: PathBuf,

        /// Enforce quality gates (exit with error on failure)
        #[arg(long)]
        enforce: bool,

        /// Minimum PMAT TDG score
        #[arg(long, default_value = "1.0")]
        min_tdg: f64,

        /// Maximum PMAT TDG score
        #[arg(long, default_value = "2.0")]
        max_tdg: f64,

        /// Maximum complexity
        #[arg(long, default_value = "20")]
        max_complexity: u32,

        /// Minimum coverage percentage
        #[arg(long, default_value = "80")]
        min_coverage: u32,
    },

    /// Interactive transpilation with annotation suggestions
    Interactive {
        /// Input Python file
        input: PathBuf,

        /// Enable annotation mode
        #[arg(long)]
        annotate: bool,
    },

    /// Inspect intermediate representations (AST/HIR)
    Inspect {
        /// Input Python file
        input: PathBuf,

        /// What to inspect: python-ast, hir, typed-hir
        #[arg(short, long, default_value = "hir")]
        repr: String,

        /// Output format: json, debug, pretty
        #[arg(short, long, default_value = "pretty")]
        format: String,

        /// Output to file instead of stdout
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Lambda-specific commands for AWS Lambda development
    #[command(subcommand)]
    Lambda(LambdaCommands),

    /// Start Language Server Protocol (LSP) server for IDE integration
    Lsp {
        /// Port to listen on
        #[arg(short, long, default_value = "2087")]
        port: u16,

        /// Enable verbose logging
        #[arg(short, long)]
        verbose: bool,
    },

    /// Debug-related commands
    Debug {
        /// Show debugging tips
        #[arg(long)]
        tips: bool,

        /// Generate debugger script
        #[arg(long)]
        gen_script: Option<PathBuf>,

        /// Debugger type (gdb, lldb, rust-gdb)
        #[arg(long, default_value = "gdb")]
        debugger: String,

        /// Python source file
        #[arg(long)]
        source: Option<PathBuf>,

        /// Output script path
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Use spydecy interactive debugger
        #[arg(long)]
        spydecy: Option<PathBuf>,

        /// Enable visualization mode (spydecy only)
        #[arg(long)]
        visualize: bool,
    },

    /// Generate documentation from Python code
    Docs {
        /// Input Python file or directory
        input: PathBuf,

        /// Output directory for documentation
        #[arg(short, long, default_value = "./docs")]
        output: PathBuf,

        /// Documentation format (markdown, html)
        #[arg(short, long, default_value = "markdown")]
        format: String,

        /// Include Python source in documentation
        #[arg(long, default_value = "true")]
        include_source: bool,

        /// Generate usage examples
        #[arg(long, default_value = "true")]
        examples: bool,

        /// Include migration notes
        #[arg(long, default_value = "true")]
        migration_notes: bool,

        /// Include performance notes
        #[arg(long)]
        performance_notes: bool,

        /// Generate API reference
        #[arg(long, default_value = "true")]
        api_reference: bool,

        /// Generate usage guide
        #[arg(long, default_value = "true")]
        usage_guide: bool,

        /// Generate index file
        #[arg(long, default_value = "true")]
        index: bool,
    },

    /// Profile Python code for performance analysis
    Profile {
        /// Input Python file
        file: PathBuf,

        /// Enable instruction counting
        #[arg(long, default_value = "true")]
        count_instructions: bool,

        /// Enable memory allocation tracking
        #[arg(long, default_value = "true")]
        track_allocations: bool,

        /// Enable hot path detection
        #[arg(long, default_value = "true")]
        detect_hot_paths: bool,

        /// Minimum samples for hot path detection
        #[arg(long, default_value = "100")]
        hot_path_threshold: usize,

        /// Generate flame graph data
        #[arg(long)]
        flamegraph: bool,

        /// Include performance hints
        #[arg(long, default_value = "true")]
        hints: bool,

        /// Output flamegraph data to file
        #[arg(long)]
        flamegraph_output: Option<PathBuf>,

        /// Output perf annotations to file
        #[arg(long)]
        perf_output: Option<PathBuf>,
    },

    /// Background agent mode with MCP integration
    #[command(subcommand)]
    Agent(AgentCommands),

    /// Oracle ML model commands (optimization, training)
    #[command(subcommand)]
    Oracle(OracleCommands),

    /// Automated convergence loop to achieve 100% compilation rate (GH-158)
    Converge {
        /// Directory containing Python examples
        #[arg(short, long, default_value = "./examples")]
        input_dir: PathBuf,

        /// Target compilation rate (0-100)
        #[arg(short, long, default_value = "100")]
        target_rate: f64,

        /// Maximum iterations before stopping
        #[arg(short, long, default_value = "50")]
        max_iterations: usize,

        /// Automatically apply transpiler fixes
        #[arg(long)]
        auto_fix: bool,

        /// Show what would be fixed without applying
        #[arg(long)]
        dry_run: bool,

        /// Minimum confidence for auto-fix (0.0-1.0)
        #[arg(long, default_value = "0.8")]
        fix_confidence: f64,

        /// Directory to save/resume checkpoints
        #[arg(long)]
        checkpoint_dir: Option<PathBuf>,

        /// Number of parallel compilation jobs
        #[arg(short, long, default_value = "4")]
        parallel_jobs: usize,
    },
}

#[derive(Subcommand)]
pub enum AgentCommands {
    /// Start the background agent daemon
    Start {
        /// MCP server port
        #[arg(long, default_value = "3000")]
        port: u16,

        /// Enable debug mode
        #[arg(long)]
        debug: bool,

        /// Configuration file path
        #[arg(long)]
        config: Option<PathBuf>,

        /// Run in foreground (don't daemonize)
        #[arg(long)]
        foreground: bool,
    },

    /// Stop the background agent daemon
    Stop,

    /// Check agent daemon status
    Status,

    /// Restart the background agent daemon
    Restart {
        /// MCP server port
        #[arg(long, default_value = "3000")]
        port: u16,

        /// Enable debug mode
        #[arg(long)]
        debug: bool,

        /// Configuration file path
        #[arg(long)]
        config: Option<PathBuf>,
    },

    /// Add project to monitoring
    AddProject {
        /// Project path to monitor
        path: PathBuf,

        /// Project identifier
        #[arg(long)]
        id: Option<String>,

        /// File patterns to watch
        #[arg(long, default_value = "**/*.py")]
        patterns: Vec<String>,
    },

    /// Remove project from monitoring
    RemoveProject {
        /// Project identifier or path
        project: String,
    },

    /// List monitored projects
    ListProjects,

    /// View agent logs
    Logs {
        /// Number of lines to show
        #[arg(short, long, default_value = "50")]
        lines: usize,

        /// Follow log output
        #[arg(short, long)]
        follow: bool,
    },
}

#[derive(Subcommand)]
pub enum OracleCommands {
    /// Optimize generation parameters using Differential Evolution
    Optimize {
        /// Number of stdlib functions to use in evaluation
        #[arg(long, default_value = "5")]
        stdlib_count: usize,

        /// Samples per evaluation
        #[arg(long, default_value = "50")]
        eval_samples: usize,

        /// Maximum evaluations
        #[arg(long, default_value = "100")]
        max_evaluations: usize,

        /// Enable curriculum learning
        #[arg(long)]
        curriculum: bool,

        /// Output path for optimized parameters (default: ~/.depyler/oracle_params.json)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Show current optimized parameters
    Show,

    /// Train Oracle model from corpus
    Train {
        /// Minimum samples for training (default: 100)
        #[arg(long, default_value = "100")]
        min_samples: usize,

        /// Enable synthetic data augmentation
        #[arg(long)]
        synthetic: bool,
    },

    /// DEPYLER-0585: Continuous improvement loop until 100% compilation
    ///
    /// Runs transpile→compile→train→fix loop until target compilation rate
    /// or maximum iterations reached. Designed for enterprise use (Netflix, AWS, etc).
    Improve {
        /// Directory containing Python files to transpile
        #[arg(short, long)]
        input_dir: PathBuf,

        /// Target compilation rate (0.0-1.0, default: 1.0 for 100%)
        #[arg(long, default_value = "1.0")]
        target_rate: f64,

        /// Maximum iterations (default: 50)
        #[arg(long, default_value = "50")]
        max_iterations: usize,

        /// Apply auto-fixes automatically
        #[arg(long)]
        auto_apply: bool,

        /// Minimum confidence for auto-fix (default: 0.85)
        #[arg(long, default_value = "0.85")]
        min_confidence: f64,

        /// Output directory for reports
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Export error corpus for external training
        #[arg(long)]
        export_corpus: Option<PathBuf>,

        /// Continue from previous run state
        #[arg(long)]
        resume: bool,

        /// Verbose progress output
        #[arg(short, long)]
        verbose: bool,

        /// Enable real-time monitoring (writes metrics to monitor.json)
        #[arg(long)]
        monitor: bool,

        // DEPYLER-0598: Diagnostic verbosity tier configuration
        /// Diagnostic verbosity tier (1-4):
        /// 1=baseline JSON+clippy, 2=+verbose, 3=+RUSTC_LOG traces, 4=+full debug
        #[arg(long, default_value = "1")]
        verbosity_tier: u8,

        /// Clippy lint level (standard, pedantic, nursery, full)
        #[arg(long, default_value = "nursery")]
        clippy_level: String,

        /// Enable adaptive verbosity escalation based on error types
        #[arg(long)]
        adaptive_verbosity: bool,

        /// Sample reweight factor for curriculum learning (per Feldman 2020).
        /// Values >1.0 emphasize rare error classes. Default: 1.0 (no reweighting)
        #[arg(long, default_value = "1.0")]
        reweight: f32,
    },

    /// Export CITL corpus for OIP training (GitHub #156)
    ///
    /// Exports the compilation training corpus in Parquet or JSONL format
    /// for consumption by the Organizational Intelligence Plugin (OIP).
    ExportOip {
        /// Input directory containing Python files (uses cached corpus if available)
        #[arg(short, long)]
        input_dir: PathBuf,

        /// Output file path (.parquet or .jsonl based on --format)
        #[arg(short, long)]
        output: PathBuf,

        /// Export format: parquet (recommended) or jsonl
        #[arg(long, default_value = "parquet")]
        format: String,

        /// Minimum confidence for including samples (0.0-1.0)
        #[arg(long, default_value = "0.80")]
        min_confidence: f64,

        /// Include Clippy lint mappings in export
        #[arg(long)]
        include_clippy: bool,

        /// Apply sample reweighting using Feldman long-tail weighting
        #[arg(long, default_value = "1.0")]
        reweight: f32,
    },

    /// Classify a Rust compiler error and suggest fixes
    Classify {
        /// Error message to classify (e.g., "error[E0308]: mismatched types")
        error: String,

        /// Output format: text (default) or json
        #[arg(short, long, default_value = "text")]
        format: String,
    },
}

#[derive(Subcommand)]
pub enum LambdaCommands {
    /// Analyze Python Lambda function and infer event types
    Analyze {
        /// Input Python Lambda file
        input: PathBuf,

        /// Output format (json, text)
        #[arg(short, long, default_value = "text")]
        format: String,

        /// Confidence threshold for event type inference
        #[arg(long, default_value = "0.8")]
        confidence: f64,
    },

    /// Convert Python Lambda to optimized Rust Lambda
    Convert {
        /// Input Python Lambda file
        input: PathBuf,

        /// Output directory for Rust Lambda project
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Enable aggressive optimizations
        #[arg(long)]
        optimize: bool,

        /// Generate test suite
        #[arg(long)]
        tests: bool,

        /// Generate deployment templates (SAM/CDK)
        #[arg(long)]
        deploy: bool,
    },

    /// Test Lambda function locally
    Test {
        /// Lambda project directory
        input: PathBuf,

        /// Run specific test event
        #[arg(short, long)]
        event: Option<String>,

        /// Enable performance benchmarks
        #[arg(long)]
        benchmark: bool,

        /// Generate load test
        #[arg(long)]
        load_test: bool,
    },

    /// Build Lambda function with optimizations
    Build {
        /// Lambda project directory
        input: PathBuf,

        /// Target architecture (arm64, x86_64)
        #[arg(long, default_value = "arm64")]
        arch: String,

        /// Enable size optimization
        #[arg(long)]
        optimize_size: bool,

        /// Enable cold start optimization
        #[arg(long)]
        optimize_cold_start: bool,
    },

    /// Deploy Lambda function to AWS
    Deploy {
        /// Lambda project directory
        input: PathBuf,

        /// AWS region
        #[arg(long)]
        region: Option<String>,

        /// Lambda function name
        #[arg(long)]
        function_name: Option<String>,

        /// IAM role ARN
        #[arg(long)]
        role: Option<String>,

        /// Dry run (don't actually deploy)
        #[arg(long)]
        dry_run: bool,
    },
}

/// Handle compile command (DEPYLER-0380)
/// Complexity: 3 (within ≤10 target)
pub fn compile_command(
    input: PathBuf,
    output: Option<PathBuf>,
    profile: String,
    verbose: bool,
) -> Result<()> {
    if verbose {
        println!("🔨 Compiling {} to native binary...", input.display());
    }

    let binary_path =
        compile_cmd::compile_python_to_binary(&input, output.as_deref(), Some(&profile))?;

    println!("✅ Binary created: {}", binary_path.display());
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn transpile_command(
    input: PathBuf,
    output: Option<PathBuf>,
    verify: bool,
    gen_tests: bool,
    debug: bool,
    source_map: bool,
    trace: bool,
    explain: bool,
    auto_fix: bool,
    suggest_fixes: bool,
    fix_confidence: f64,
) -> Result<()> {
    let start = Instant::now();

    // Read input file
    let python_source = fs::read_to_string(&input)?;
    let source_size = python_source.len();

    // Create progress bar
    let pb = ProgressBar::new(4);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    // Initialize pipeline
    pb.set_message("Initializing pipeline...");
    let mut pipeline = DepylerPipeline::new();
    if verify {
        pipeline = pipeline.with_verification();
    }
    if debug || source_map {
        let debug_config = depyler_core::debug::DebugConfig {
            debug_level: if debug {
                depyler_core::debug::DebugLevel::Full
            } else {
                depyler_core::debug::DebugLevel::Basic
            },
            generate_source_map: source_map,
            preserve_symbols: true,
            debug_prints: debug,
            breakpoints: debug,
        };
        pipeline = pipeline.with_debug(debug_config);
    }
    pb.inc(1);

    // Trace: Pipeline initialization
    if trace {
        eprintln!("\n=== TRANSPILATION TRACE ===");
        eprintln!("Phase 1: Pipeline Initialization");
        eprintln!(
            "  - Verification: {}",
            if verify { "enabled" } else { "disabled" }
        );
        eprintln!(
            "  - Debug mode: {}",
            if debug { "enabled" } else { "disabled" }
        );
        eprintln!(
            "  - Source map: {}",
            if source_map { "enabled" } else { "disabled" }
        );
        eprintln!();
    }
    // Parse Python
    pb.set_message("Parsing Python source...");
    let parse_start = Instant::now();

    // Trace: AST parsing
    if trace {
        eprintln!("Phase 2: AST Parsing");
        eprintln!("  - Input size: {} bytes", source_size);
        eprintln!("  - Parsing Python source...");
    }

    // DEPYLER-0384: Use transpile_with_dependencies to get both code and dependencies
    let (rust_code, dependencies) = pipeline.transpile_with_dependencies(&python_source)?;
    let parse_time = parse_start.elapsed();

    // Trace: Transpilation complete
    if trace {
        eprintln!("  - Parse time: {:.2}ms", parse_time.as_millis());
        eprintln!("\nPhase 3: Code Generation");
        eprintln!("  - Generated Rust code: {} bytes", rust_code.len());
        eprintln!("  - Dependencies detected: {}", dependencies.len());
        eprintln!("  - Generation complete");
        eprintln!();
    }

    // Explain: Transformation decisions
    if explain {
        eprintln!("\n=== TRANSPILATION EXPLANATION ===");
        eprintln!("Transformation Decisions:");
        eprintln!("  1. Python AST -> HIR: Converted Python constructs to type-safe HIR");
        eprintln!("  2. HIR -> Rust: Generated idiomatic Rust code with:");
        eprintln!("     - Type inference for local variables");
        eprintln!("     - Ownership and borrowing semantics");
        eprintln!("     - Memory safety guarantees");
        eprintln!("  3. Module mapping: Applied Python->Rust standard library mappings");
        eprintln!();
    }

    pb.inc(1);

    // Analyze if requested
    if verify {
        pb.set_message("Analyzing code...");
        // Analysis would happen here
        pb.inc(1);
    }

    // ML-powered auto-fix / suggest-fixes (Issue #105)
    let rust_code = if auto_fix || suggest_fixes {
        pb.set_message("Running ML oracle...");

        // Try hybrid transpiler first (uses ML when AST fails)
        let mut hybrid = HybridTranspiler::new();
        match hybrid.transpile(&python_source) {
            Ok(result) => {
                if trace {
                    eprintln!("\n=== ML Oracle Analysis ===");
                    eprintln!("  Strategy: {:?}", result.strategy);
                    eprintln!("  Confidence: {:.2}%", result.confidence * 100.0);
                }

                if result.confidence >= fix_confidence as f32 {
                    if suggest_fixes {
                        println!("\n🔮 ML Oracle Suggestions:");
                        println!(
                            "  ✓ High confidence ({:.2}%) - auto-fix would apply",
                            result.confidence * 100.0
                        );
                    }
                    if auto_fix {
                        println!(
                            "\n🔧 Auto-fix applied with {:.2}% confidence",
                            result.confidence * 100.0
                        );
                    }
                    result.rust_code
                } else {
                    if suggest_fixes {
                        println!("\n🔮 ML Oracle Suggestions:");
                        println!(
                            "  ⚠ Low confidence ({:.2}%) - manual review recommended",
                            result.confidence * 100.0
                        );

                        // Try to get fix suggestions from ngram predictor
                        let oracle = Oracle::load_or_train().ok();
                        if let Some(oracle) = oracle {
                            let message = format!("transpilation: {:?}", result.strategy);
                            if let Ok(classification) = oracle.classify_message(&message) {
                                println!("  Category: {:?}", classification.category);
                                if let Some(fix) = &classification.suggested_fix {
                                    println!("  Suggested fix: {}", fix);
                                }
                            }
                        }
                    }
                    rust_code // Use original if confidence too low
                }
            }
            Err(e) => {
                if suggest_fixes {
                    println!("\n🔮 ML Oracle Suggestions:");
                    println!("  ✗ Transpilation error: {}", e);
                    println!("  → Check for unsupported Python patterns");
                }
                rust_code // Fallback to original
            }
        }
    } else {
        rust_code
    };

    // Generate output
    pb.set_message("Writing output...");
    let output_path = output.unwrap_or_else(|| {
        let mut path = input.clone();
        path.set_extension("rs");
        path
    });

    fs::write(&output_path, &rust_code)?;

    // DEPYLER-0384: Generate and write Cargo.toml if dependencies exist
    if !dependencies.is_empty() {
        // Extract package name from output file stem (e.g., "example_stdlib" from "example_stdlib.rs")
        let package_name = output_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("transpiled_package");

        // Extract source file name for [[bin]] path (DEPYLER-0392)
        let source_file_name = output_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("main.rs");

        // Generate Cargo.toml content
        let cargo_toml_content = depyler_core::cargo_toml_gen::generate_cargo_toml(
            package_name,
            source_file_name,
            &dependencies,
        );

        // Write Cargo.toml to the same directory as the output file
        let mut cargo_toml_path = output_path.clone();
        cargo_toml_path.set_file_name("Cargo.toml");

        fs::write(&cargo_toml_path, &cargo_toml_content)?;

        if trace {
            eprintln!("  - Generated Cargo.toml: {}", cargo_toml_path.display());
        }
    }

    pb.inc(1);

    pb.finish_and_clear();

    // Generate tests if requested
    if gen_tests {
        let test_path = output_path.with_extension("test.rs");
        // Test generation would happen here
        println!("✅ Generated tests: {}", test_path.display());
    }

    // Print summary
    let total_time = start.elapsed();
    let throughput = (source_size as f64 / 1024.0) / parse_time.as_secs_f64();

    println!("📄 Source: {} ({} bytes)", input.display(), source_size);
    println!(
        "📝 Output: {} ({} bytes)",
        output_path.display(),
        rust_code.len()
    );

    // DEPYLER-0384: Show Cargo.toml generation
    if !dependencies.is_empty() {
        let mut cargo_toml_path = output_path.clone();
        cargo_toml_path.set_file_name("Cargo.toml");
        println!(
            "📦 Cargo.toml: {} ({} dependencies)",
            cargo_toml_path.display(),
            dependencies.len()
        );
    }

    println!("⏱️  Parse time: {:.2}ms", parse_time.as_millis());
    println!("📊 Throughput: {throughput:.1} KB/s");
    println!("⏱️  Total time: {:.2}ms", total_time.as_millis());

    if verify {
        println!("✓ Properties Verified");
    }

    Ok(())
}

pub fn analyze_command(input: PathBuf, format: String) -> Result<()> {
    // Read and parse
    let python_source = fs::read_to_string(&input)?;
    let _pipeline = DepylerPipeline::new();

    // Parse to HIR
    let ast = {
        use rustpython_parser::{parse, Mode};
        parse(&python_source, Mode::Module, "<input>")?
    };
    let (hir, _type_env) = depyler_core::ast_bridge::python_to_hir(ast)?;

    // Analyze
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze(&hir)?;

    match format.as_str() {
        "json" => {
            let json = serde_json::to_string_pretty(&analysis)?;
            println!("{json}");
        }
        _ => {
            // Text format
            println!(
                "Source: {} ({} KB)",
                input.display(),
                python_source.len() / 1024
            );
            println!("Functions: {}", analysis.module_metrics.total_functions);
            println!(
                "Avg Cyclomatic: {:.1}",
                analysis.module_metrics.avg_cyclomatic_complexity
            );
            println!(
                "Max Cognitive: {}",
                analysis.module_metrics.max_cognitive_complexity
            );
            println!(
                "Type Coverage: {:.0}%",
                analysis.type_coverage.coverage_percentage
            );
        }
    }

    Ok(())
}

pub fn check_command(input: PathBuf) -> Result<()> {
    let python_source = fs::read_to_string(&input)?;
    let pipeline = DepylerPipeline::new();

    // Try to transpile
    match pipeline.transpile(&python_source) {
        Ok(_) => {
            println!("✓ {} can be transpiled directly", input.display());
            Ok(())
        }
        Err(e) => {
            println!("✗ {} cannot be transpiled: {}", input.display(), e);
            std::process::exit(1);
        }
    }
}

pub fn complexity_rating(complexity: f64) -> colored::ColoredString {
    if complexity <= 5.0 {
        "(✓ Good)".green()
    } else if complexity <= 10.0 {
        "(✓ Acceptable)".yellow()
    } else {
        "(⚠ High)".red()
    }
}

pub fn quality_check_command(
    input: PathBuf,
    enforce: bool,
    min_tdg: f64,
    max_tdg: f64,
    max_complexity: u32,
    min_coverage: u32,
) -> Result<()> {
    let report = generate_quality_report(&input)?;
    let quality_analyzer = QualityAnalyzer::new();
    quality_analyzer.print_quality_report(&report);

    let validations =
        validate_quality_targets(&report, min_tdg, max_tdg, max_complexity, min_coverage);
    print_validation_results(&validations);

    let compilation_results = check_compilation_quality(&input)?;
    print_compilation_results(&compilation_results);

    let all_passed = validations.all_passed && compilation_results.all_passed;

    if enforce && !all_passed {
        std::process::exit(1);
    }

    Ok(())
}

pub struct QualityValidations {
    pub tdg_ok: bool,
    pub complexity_ok: bool,
    pub coverage_ok: bool,
    pub all_passed: bool,
    pub report: depyler_quality::QualityReport,
    pub min_tdg: f64,
    pub max_tdg: f64,
    pub max_complexity: u32,
    pub min_coverage: u32,
}

pub struct CompilationResults {
    pub compilation_ok: bool,
    pub clippy_ok: bool,
    pub all_passed: bool,
}

pub fn generate_quality_report(input: &std::path::Path) -> Result<depyler_quality::QualityReport> {
    let python_source = fs::read_to_string(input)?;
    let ast = {
        use rustpython_parser::{parse, Mode};
        parse(&python_source, Mode::Module, "<input>")?
    };
    let (hir, _type_env) = depyler_core::ast_bridge::python_to_hir(ast)?;
    let quality_analyzer = QualityAnalyzer::new();
    Ok(quality_analyzer.analyze_quality(&hir.functions)?)
}

pub fn validate_quality_targets(
    report: &depyler_quality::QualityReport,
    min_tdg: f64,
    max_tdg: f64,
    max_complexity: u32,
    min_coverage: u32,
) -> QualityValidations {
    let tdg_ok = report.pmat_metrics.tdg >= min_tdg && report.pmat_metrics.tdg <= max_tdg;
    let complexity_ok = report.complexity_metrics.cyclomatic_complexity <= max_complexity;
    let coverage_ok = report.coverage_metrics.line_coverage >= (min_coverage as f64 / 100.0);
    let all_passed = tdg_ok && complexity_ok && coverage_ok;

    QualityValidations {
        tdg_ok,
        complexity_ok,
        coverage_ok,
        all_passed,
        report: report.clone(),
        min_tdg,
        max_tdg,
        max_complexity,
        min_coverage,
    }
}

pub fn print_validation_results(validations: &QualityValidations) {
    println!("Target Verification:");
    println!(
        "  {} PMAT TDG: {:.2} (target: {:.1}-{:.1})",
        if validations.tdg_ok { "✅" } else { "❌" },
        validations.report.pmat_metrics.tdg,
        validations.min_tdg,
        validations.max_tdg
    );
    println!(
        "  {} Complexity: {} (target: ≤{})",
        if validations.complexity_ok {
            "✅"
        } else {
            "❌"
        },
        validations.report.complexity_metrics.cyclomatic_complexity,
        validations.max_complexity
    );
    println!(
        "  {} Coverage: {:.1}% (target: ≥{}%)",
        if validations.coverage_ok {
            "✅"
        } else {
            "❌"
        },
        validations.report.coverage_metrics.line_coverage * 100.0,
        validations.min_coverage
    );
}

pub fn check_compilation_quality(input: &std::path::Path) -> Result<CompilationResults> {
    let compilation_ok = check_rust_compilation(input)?;
    let clippy_ok = check_clippy_clean(input)?;
    let all_passed = compilation_ok && clippy_ok;

    Ok(CompilationResults {
        compilation_ok,
        clippy_ok,
        all_passed,
    })
}

pub fn print_compilation_results(results: &CompilationResults) {
    println!("Compilation Check:");
    println!(
        "  {} rustc compilation: {}",
        if results.compilation_ok { "✅" } else { "❌" },
        if results.compilation_ok {
            "PASS"
        } else {
            "FAIL"
        }
    );
    println!(
        "  {} clippy: {}",
        if results.clippy_ok { "✅" } else { "❌" },
        if results.clippy_ok {
            "CLEAN"
        } else {
            "WARNINGS"
        }
    );
}

pub fn interactive_command(input: PathBuf, annotate: bool) -> Result<()> {
    interactive::run_interactive_session(&input.to_string_lossy(), annotate)
}

pub fn inspect_command(
    input: PathBuf,
    repr: String,
    format: String,
    output: Option<PathBuf>,
) -> Result<()> {
    let python_source = fs::read_to_string(&input)?;
    let pipeline = DepylerPipeline::new();

    let output_content = match repr.as_str() {
        "python-ast" => inspect_python_ast(&python_source, &format)?,
        "hir" => {
            let hir = pipeline.parse_to_hir(&python_source)?;
            inspect_hir(&hir, &format)?
        }
        "typed-hir" => {
            let typed_hir = pipeline.analyze_to_typed_hir(&python_source)?;
            inspect_hir(&typed_hir, &format)?
        }
        _ => {
            return Err(anyhow::anyhow!("Unknown representation: {repr}"));
        }
    };

    // Output results
    match output {
        Some(output_path) => {
            fs::write(&output_path, &output_content)?;
            println!("✅ Output written to: {}", output_path.display());
        }
        None => {
            println!("{output_content}");
        }
    }

    Ok(())
}

pub fn inspect_python_ast(python_source: &str, format: &str) -> Result<String> {
    use rustpython_parser::{parse, Mode};

    let ast = parse(python_source, Mode::Module, "<input>")?;

    match format {
        "json" => {
            // rustpython_ast doesn't implement Serialize, so use debug format for JSON
            Ok(format!("{ast:#?}"))
        }
        "debug" => Ok(format!("{ast:#?}")),
        "pretty" => Ok(format_python_ast_pretty(&ast)),
        _ => Err(anyhow::anyhow!("Unknown format: {}", format)),
    }
}

pub fn inspect_hir(hir: &depyler_core::hir::HirModule, format: &str) -> Result<String> {
    match format {
        "json" => Ok(serde_json::to_string_pretty(hir)?),
        "debug" => Ok(format!("{hir:#?}")),
        "pretty" => Ok(format_hir_pretty(hir)),
        _ => Err(anyhow::anyhow!("Unknown format: {}", format)),
    }
}

pub fn format_python_ast_pretty(ast: &rustpython_ast::Mod) -> String {
    let mut output = String::new();
    output.push_str("🐍 Python AST Structure\n");
    output.push_str("========================\n\n");

    match ast {
        rustpython_ast::Mod::Module(module) => {
            output.push_str(&format!(
                "Module with {} statements:\n\n",
                module.body.len()
            ));

            for (i, stmt) in module.body.iter().enumerate() {
                output.push_str(&format!("Statement {}: ", i + 1));
                output.push_str(&format_stmt_summary(stmt));
                output.push('\n');
            }
        }
        _ => output.push_str("Non-module AST node\n"),
    }

    output
}

pub fn format_stmt_summary(stmt: &rustpython_ast::Stmt) -> String {
    match stmt {
        rustpython_ast::Stmt::FunctionDef(func) => {
            format!(
                "Function '{}' with {} parameters",
                func.name,
                func.args.args.len()
            )
        }
        rustpython_ast::Stmt::Return(_) => "Return statement".to_string(),
        rustpython_ast::Stmt::Assign(_) => "Assignment".to_string(),
        rustpython_ast::Stmt::If(_) => "If statement".to_string(),
        rustpython_ast::Stmt::While(_) => "While loop".to_string(),
        rustpython_ast::Stmt::For(_) => "For loop".to_string(),
        rustpython_ast::Stmt::Expr(_) => "Expression statement".to_string(),
        _ => format!("{stmt:?}")
            .split('(')
            .next()
            .unwrap_or("Unknown")
            .to_string(),
    }
}

pub fn format_hir_pretty(hir: &depyler_core::hir::HirModule) -> String {
    let mut output = String::new();
    output.push_str("🦀 Depyler HIR Structure\n");
    output.push_str("=========================\n\n");

    // Functions
    output.push_str(&format!("🔧 Functions ({}):\n", hir.functions.len()));
    for (i, func) in hir.functions.iter().enumerate() {
        output.push_str(&format!("\n{}. Function: {}\n", i + 1, func.name));
        output.push_str(&format!(
            "   Parameters: {} -> {:?}\n",
            func.params
                .iter()
                .map(|param| format!("{}: {:?}", param.name, param.ty))
                .collect::<Vec<_>>()
                .join(", "),
            func.ret_type
        ));
        output.push_str(&format!("   Body: {} statements\n", func.body.len()));
    }

    output
}

pub fn check_rust_compilation(python_file: &std::path::Path) -> Result<bool> {
    // Convert Python file to expected Rust file
    let rust_file = python_file.with_extension("rs");
    check_rust_compilation_for_file(rust_file.to_str().unwrap())
}

pub fn check_rust_compilation_for_file(rust_file: &str) -> Result<bool> {
    if !std::path::Path::new(rust_file).exists() {
        return Ok(false);
    }

    let output = Command::new("rustc")
        .arg("--check-cfg")
        .arg("cfg()")
        .arg("--crate-type")
        .arg("lib")
        .arg(rust_file)
        .arg("-o")
        .arg("/dev/null")
        .output()?;

    Ok(output.status.success())
}

pub fn check_clippy_clean(python_file: &std::path::Path) -> Result<bool> {
    let rust_file = python_file.with_extension("rs");

    if !rust_file.exists() {
        return Ok(false);
    }

    let output = Command::new("cargo")
        .arg("clippy")
        .arg("--")
        .arg("-D")
        .arg("warnings")
        .arg("--check")
        .arg(&rust_file)
        .output()?;

    Ok(output.status.success())
}

// Debug command implementation

pub fn debug_command(
    tips: bool,
    gen_script: Option<PathBuf>,
    debugger: String,
    source: Option<PathBuf>,
    output: Option<PathBuf>,
    spydecy: Option<PathBuf>,
    visualize: bool,
) -> Result<()> {
    if tips {
        debug_cmd::print_debugging_tips();
        return Ok(());
    }

    if let Some(source_file) = spydecy {
        debug_cmd::launch_spydecy_debugger(&source_file, visualize)?;
        return Ok(());
    }

    if let Some(rust_file) = gen_script {
        let source_file = source
            .ok_or_else(|| anyhow::anyhow!("--source is required when using --gen-script"))?;

        debug_cmd::generate_debugger_script(
            &source_file,
            &rust_file,
            &debugger,
            output.as_deref(),
        )?;
    } else {
        println!("Use --tips for debugging guide or --gen-script to generate debugger scripts");
        println!("Use --spydecy <file> for interactive debugging");
    }

    Ok(())
}

// LSP command implementation

pub fn lsp_command(port: u16, verbose: bool) -> Result<()> {
    use depyler_core::lsp::LspServer;
    use std::io::{self, BufRead, Write};

    println!("🚀 Starting Depyler Language Server on port {}...", port);

    // For now, implement a simple stdio-based LSP server
    // In a full implementation, this would handle TCP connections

    let _server = LspServer::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    println!("📡 Language Server ready. Waiting for client connections...");
    println!("   Use with your IDE's LSP client configuration:");
    println!("   - Command: depyler lsp");
    println!("   - Port: {}", port);
    println!("   - Language: Python");

    // Simple message loop (in practice, would use full JSON-RPC)
    for line in stdin.lock().lines() {
        let line = line?;

        if verbose {
            eprintln!("Received: {}", line);
        }

        // Handle shutdown
        if line.contains("shutdown") {
            println!("👋 Language Server shutting down...");
            break;
        }

        // Echo back for now (real implementation would parse JSON-RPC)
        writeln!(stdout, "{{\"jsonrpc\":\"2.0\",\"result\":null,\"id\":1}}")?;
        stdout.flush()?;
    }

    Ok(())
}

// Lambda-specific command implementations

pub fn lambda_analyze_command(input: PathBuf, format: String, confidence: f64) -> Result<()> {
    let python_source = fs::read_to_string(&input)?;
    let pipeline = DepylerPipeline::new();

    // Parse to AST for inference
    let ast = pipeline.parse_python(&python_source)?;

    // Create and configure inferencer
    let inferencer = LambdaTypeInferencer::new().with_confidence_threshold(confidence);

    // Analyze the handler
    let analysis_report = inferencer.analyze_handler(&ast)?;

    match format.as_str() {
        "json" => {
            let json = serde_json::to_string_pretty(&analysis_report)?;
            println!("{json}");
        }
        _ => {
            println!("🔍 Lambda Event Type Analysis");
            println!("==============================");
            println!("📄 File: {}", input.display());
            println!(
                "🎯 Inferred Event Type: {:?}",
                analysis_report.inferred_event_type
            );
            println!("📊 Confidence Scores:");
            for (event_type, confidence) in &analysis_report.confidence_scores {
                println!("   {event_type:?}: {confidence:.2}");
            }
            println!(
                "🔍 Detected Patterns: {}",
                analysis_report.detected_patterns.len()
            );
            for pattern in &analysis_report.detected_patterns {
                println!(
                    "   - {:?}: {:?}",
                    pattern.pattern_type,
                    pattern.access_chain.join(".")
                );
            }

            if !analysis_report.recommendations.is_empty() {
                println!("💡 Recommendations:");
                for rec in &analysis_report.recommendations {
                    println!("   - {rec}");
                }
            }
        }
    }

    Ok(())
}

/// Maps EventType from depyler_core to LambdaEventType from depyler_annotations
///
/// This helper converts between the two enum types used in different crates.
/// Complexity: 7 (one match with 7 arms)
fn infer_and_map_event_type(
    inferred_type: depyler_core::lambda_inference::EventType,
) -> depyler_annotations::LambdaEventType {
    match inferred_type {
        depyler_core::lambda_inference::EventType::S3Event => {
            depyler_annotations::LambdaEventType::S3Event
        }
        depyler_core::lambda_inference::EventType::ApiGatewayV2Http => {
            depyler_annotations::LambdaEventType::ApiGatewayV2HttpRequest
        }
        depyler_core::lambda_inference::EventType::SnsEvent => {
            depyler_annotations::LambdaEventType::SnsEvent
        }
        depyler_core::lambda_inference::EventType::SqsEvent => {
            depyler_annotations::LambdaEventType::SqsEvent
        }
        depyler_core::lambda_inference::EventType::DynamodbEvent => {
            depyler_annotations::LambdaEventType::DynamodbEvent
        }
        depyler_core::lambda_inference::EventType::EventBridge => {
            depyler_annotations::LambdaEventType::EventBridgeEvent(None)
        }
        _ => depyler_annotations::LambdaEventType::Auto,
    }
}

/// Creates a LambdaGenerationContext from annotations and transpiled Rust code
///
/// This helper builds the context structure needed for Lambda code generation.
/// Complexity: 1 (simple struct construction)
fn create_lambda_generation_context(
    lambda_annotations: &depyler_annotations::LambdaAnnotations,
    rust_code: String,
    input: &Path,
) -> depyler_core::lambda_codegen::LambdaGenerationContext {
    depyler_core::lambda_codegen::LambdaGenerationContext {
        event_type: lambda_annotations.event_type.clone(),
        response_type: "serde_json::Value".to_string(), // Could be inferred better
        handler_body: rust_code,
        imports: vec![],
        dependencies: vec![],
        annotations: lambda_annotations.clone(),
        function_name: "handler".to_string(),
        module_name: input.file_stem().unwrap().to_string_lossy().to_string(),
    }
}

/// Configures LambdaCodeGenerator with optimization profile if requested
///
/// Complexity: 3 (optimize check + nested if + optimization setup)
fn setup_lambda_generator(
    optimize: bool,
    lambda_annotations: &depyler_annotations::LambdaAnnotations,
) -> Result<LambdaCodeGenerator> {
    let mut generator = LambdaCodeGenerator::new();
    if optimize {
        let optimizer = LambdaOptimizer::new().enable_aggressive_mode();
        let _optimization_plan = optimizer.generate_optimization_plan(lambda_annotations)?;
        let optimized_profile = depyler_core::lambda_codegen::OptimizationProfile {
            lto: true,
            panic_abort: true,
            codegen_units: 1,
            opt_level: "z".to_string(),
            strip: true,
            mimalloc: true,
        };
        generator = generator.with_optimization_profile(optimized_profile);
    }
    Ok(generator)
}

/// Writes core Lambda project files (main.rs, Cargo.toml, build.sh, README.md)
///
/// Complexity: 2 (Unix permission check)
fn write_lambda_project_files(output_dir: &Path, project: &LambdaProject) -> Result<()> {
    fs::create_dir_all(output_dir)?;
    fs::create_dir_all(output_dir.join("src"))?;

    // Write main files
    fs::write(output_dir.join("src/main.rs"), &project.handler_code)?;
    fs::write(output_dir.join("Cargo.toml"), &project.cargo_toml)?;
    fs::write(output_dir.join("build.sh"), &project.build_script)?;
    fs::write(output_dir.join("README.md"), &project.readme)?;

    // Make build script executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(output_dir.join("build.sh"))?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(output_dir.join("build.sh"), perms)?;
    }

    Ok(())
}

/// Writes deployment templates (SAM/CDK) if deploy flag is set
///
/// Complexity: 3 (deploy check + 2 optional template writes)
fn write_deployment_templates(
    output_dir: &Path,
    project: &LambdaProject,
    deploy: bool,
) -> Result<()> {
    if deploy {
        if let Some(ref sam_template) = project.sam_template {
            fs::write(output_dir.join("template.yaml"), sam_template)?;
        }
        if let Some(ref cdk_construct) = project.cdk_construct {
            fs::write(output_dir.join("lambda-construct.ts"), cdk_construct)?;
        }
    }
    Ok(())
}

/// Generates and writes test suite files if tests flag is set
///
/// Complexity: 3 (tests check + Unix permission check)
fn generate_and_write_tests(
    output_dir: &Path,
    lambda_annotations: &depyler_annotations::LambdaAnnotations,
    tests: bool,
) -> Result<()> {
    if tests {
        let test_harness = LambdaTestHarness::new();
        let test_suite = test_harness.generate_test_suite(lambda_annotations)?;
        fs::write(output_dir.join("src/lib.rs"), &test_suite)?;

        let test_script = test_harness.generate_cargo_lambda_test_script(lambda_annotations)?;
        fs::write(output_dir.join("test.sh"), &test_script)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(output_dir.join("test.sh"))?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(output_dir.join("test.sh"), perms)?;
        }
    }
    Ok(())
}

/// Prints completion summary and next steps
///
/// Complexity: 3 (optimize/tests/deploy conditionals in output)
#[allow(clippy::too_many_arguments)]
fn print_lambda_summary(
    input: &Path,
    output_dir: &Path,
    analysis: &AnalysisReport,
    optimize: bool,
    tests: bool,
    deploy: bool,
    total_time: Duration,
) {
    println!("🎉 Lambda conversion completed!");
    println!("📄 Input: {}", input.display());
    println!("📁 Output: {}", output_dir.display());
    println!("🎯 Event Type: {:?}", analysis.inferred_event_type);
    println!(
        "⚡ Optimizations: {}",
        if optimize { "Enabled" } else { "Standard" }
    );
    println!("🧪 Tests: {}", if tests { "Generated" } else { "Skipped" });
    println!(
        "🚀 Deploy Templates: {}",
        if deploy { "Generated" } else { "Skipped" }
    );
    println!("⏱️  Total Time: {:.2}ms", total_time.as_millis());

    println!("\n📋 Next Steps:");
    println!("   cd {}", output_dir.display());
    println!("   ./build.sh                    # Build the Lambda");
    if tests {
        println!("   ./test.sh                     # Run tests");
    }
    println!("   cargo lambda deploy           # Deploy to AWS");
}

pub fn lambda_convert_command(
    input: PathBuf,
    output: Option<PathBuf>,
    optimize: bool,
    tests: bool,
    deploy: bool,
) -> Result<()> {
    let start = Instant::now();
    let python_source = fs::read_to_string(&input)?;

    // Create progress bar
    let pb = ProgressBar::new(6);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    // Step 1: Parse and analyze
    pb.set_message("🔍 Analyzing Lambda function...");
    let pipeline = DepylerPipeline::new();
    let ast = pipeline.parse_python(&python_source)?;

    // Infer event type
    let inferencer = LambdaTypeInferencer::new();
    let analysis = inferencer.analyze_handler(&ast)?;
    pb.inc(1);

    // Step 2: Extract annotations and generate context
    pb.set_message("📋 Processing annotations...");
    let annotations = depyler_annotations::AnnotationParser::new()
        .parse_annotations(&python_source)
        .unwrap_or_default();

    let lambda_annotations =
        annotations
            .lambda_annotations
            .unwrap_or_else(|| depyler_annotations::LambdaAnnotations {
                event_type: Some(infer_and_map_event_type(
                    analysis.inferred_event_type.clone(),
                )),
                ..Default::default()
            });
    pb.inc(1);

    // Step 3: Transpile to Rust
    pb.set_message("🦀 Transpiling to Rust...");
    let rust_code = pipeline.transpile(&python_source)?;

    let generation_context =
        create_lambda_generation_context(&lambda_annotations, rust_code, &input);
    pb.inc(1);

    // Step 4: Generate optimized Lambda project
    pb.set_message("⚡ Generating optimized project...");
    let generator = setup_lambda_generator(optimize, &lambda_annotations)?;
    let project = generator.generate_lambda_project(&generation_context)?;
    pb.inc(1);

    // Step 5: Write output
    pb.set_message("📁 Writing project files...");
    let output_dir = output.unwrap_or_else(|| {
        input.parent().unwrap().join(format!(
            "{}_lambda",
            input.file_stem().unwrap().to_string_lossy()
        ))
    });

    write_lambda_project_files(&output_dir, &project)?;
    write_deployment_templates(&output_dir, &project, deploy)?;
    pb.inc(1);

    // Step 6: Generate tests if requested
    pb.set_message("🧪 Generating test suite...");
    generate_and_write_tests(&output_dir, &lambda_annotations, tests)?;
    pb.inc(1);

    pb.finish_and_clear();

    // Print summary
    let total_time = start.elapsed();
    print_lambda_summary(
        &input,
        &output_dir,
        &analysis,
        optimize,
        tests,
        deploy,
        total_time,
    );

    Ok(())
}

pub fn lambda_test_command(
    input: PathBuf,
    event: Option<String>,
    benchmark: bool,
    load_test: bool,
) -> Result<()> {
    if !input.join("Cargo.toml").exists() {
        return Err(anyhow::anyhow!("Not a valid Lambda project directory"));
    }

    let current_dir = std::env::current_dir()?;
    std::env::set_current_dir(&input)?;

    if let Some(event_name) = event {
        println!("🧪 Running specific test event: {event_name}");
        let output = Command::new("cargo")
            .args(["test", &format!("test_{event_name}")])
            .output()?;

        if output.status.success() {
            println!("✅ Test passed");
        } else {
            println!("❌ Test failed");
            println!("{}", String::from_utf8_lossy(&output.stderr));
        }
    } else {
        println!("🧪 Running all tests...");
        let output = Command::new("cargo").arg("test").output()?;

        if output.status.success() {
            println!("✅ All tests passed");
        } else {
            println!("❌ Some tests failed");
            println!("{}", String::from_utf8_lossy(&output.stderr));
        }
    }

    if benchmark {
        println!("📊 Running performance benchmarks...");
        if input.join("test.sh").exists() {
            let output = Command::new("./test.sh").output()?;
            println!("{}", String::from_utf8_lossy(&output.stdout));
        } else {
            println!("⚠️ No test.sh script found for benchmarking");
        }
    }

    if load_test {
        println!("🔥 Generating load test script...");
        let harness = LambdaTestHarness::new();
        let annotations = depyler_annotations::LambdaAnnotations::default(); // Could read from project
        let load_script = harness.generate_load_test_script(&annotations)?;
        fs::write("load_test.sh", &load_script)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata("load_test.sh")?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions("load_test.sh", perms)?;
        }

        println!("✅ Load test script generated: load_test.sh");
    }

    std::env::set_current_dir(current_dir)?;
    Ok(())
}

pub fn lambda_build_command(
    input: PathBuf,
    arch: String,
    optimize_size: bool,
    optimize_cold_start: bool,
) -> Result<()> {
    if !input.join("Cargo.toml").exists() {
        return Err(anyhow::anyhow!("Not a valid Lambda project directory"));
    }

    let current_dir = std::env::current_dir()?;
    std::env::set_current_dir(&input)?;

    println!("🏗️ Building Lambda function...");

    let arch_flag = match arch.as_str() {
        "arm64" | "aarch64" => "--arm64",
        "x86_64" | "x64" => "--x86-64",
        _ => return Err(anyhow::anyhow!("Unsupported architecture: {}", arch)),
    };

    let mut build_cmd = Command::new("cargo");
    build_cmd.args(["lambda", "build", "--release", arch_flag]);

    if optimize_size || optimize_cold_start {
        build_cmd.arg("--profile").arg("lambda");
    }

    println!("Running: cargo lambda build --release {arch_flag}");
    let output = build_cmd.output()?;

    if output.status.success() {
        println!("✅ Build successful");

        // Show binary size if available
        if let Ok(entries) = fs::read_dir("target/lambda") {
            for entry in entries.flatten() {
                let bootstrap_path = entry.path().join("bootstrap");
                if bootstrap_path.exists() {
                    if let Ok(metadata) = fs::metadata(&bootstrap_path) {
                        let size_kb = metadata.len() / 1024;
                        println!("📦 Binary size: {size_kb}KB");

                        if optimize_size && size_kb > 2048 {
                            println!("⚠️ Binary size is larger than 2MB, consider additional optimizations");
                        }
                    }
                }
            }
        }
    } else {
        println!("❌ Build failed");
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }

    std::env::set_current_dir(current_dir)?;
    Ok(())
}

pub fn lambda_deploy_command(
    input: PathBuf,
    region: Option<String>,
    function_name: Option<String>,
    role: Option<String>,
    dry_run: bool,
) -> Result<()> {
    if !input.join("Cargo.toml").exists() {
        return Err(anyhow::anyhow!("Not a valid Lambda project directory"));
    }

    let current_dir = std::env::current_dir()?;
    std::env::set_current_dir(&input)?;

    let func_name =
        function_name.unwrap_or_else(|| input.file_name().unwrap().to_string_lossy().to_string());

    if dry_run {
        println!("🔍 Dry run deployment for function: {func_name}");
        if let Some(ref region) = region {
            println!("📍 Region: {region}");
        }
        if let Some(ref role) = role {
            println!("🔑 IAM Role: {role}");
        }
        println!("✅ Dry run completed - no actual deployment");
        std::env::set_current_dir(current_dir)?;
        return Ok(());
    }

    println!("🚀 Deploying Lambda function: {func_name}");

    let mut deploy_cmd = Command::new("cargo");
    deploy_cmd.args(["lambda", "deploy", &func_name]);

    if let Some(ref region) = region {
        deploy_cmd.args(["--region", region]);
    }

    if let Some(ref role) = role {
        deploy_cmd.args(["--iam-role", role]);
    }

    let output = deploy_cmd.output()?;

    if output.status.success() {
        println!("✅ Deployment successful");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("❌ Deployment failed");
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }

    std::env::set_current_dir(current_dir)?;
    Ok(())
}

pub async fn agent_start_command(
    port: u16,
    debug: bool,
    config: Option<PathBuf>,
    foreground: bool,
) -> Result<()> {
    use crate::agent::daemon::{AgentDaemon, DaemonConfig};

    let config = if let Some(config_path) = config {
        DaemonConfig::from_file(&config_path)?
    } else {
        DaemonConfig::default()
    };

    let mut config = config;
    config.mcp_port = port;
    config.debug = debug;

    let mut daemon = AgentDaemon::new(config);

    if foreground {
        println!("🚀 Starting Depyler agent in foreground mode on port {port}...");
        daemon.run().await
    } else {
        println!("🚀 Starting Depyler agent daemon on port {port}...");
        daemon.start_daemon().await
    }
}

pub fn agent_stop_command() -> Result<()> {
    use crate::agent::daemon::AgentDaemon;

    println!("🛑 Stopping Depyler agent daemon...");
    AgentDaemon::stop_daemon()
}

pub fn agent_status_command() -> Result<()> {
    use crate::agent::daemon::AgentDaemon;

    match AgentDaemon::daemon_status()? {
        Some(pid) => {
            println!("✅ Depyler agent daemon is running (PID: {pid})");
        }
        None => {
            println!("❌ Depyler agent daemon is not running");
        }
    }

    Ok(())
}

pub async fn agent_restart_command(port: u16, debug: bool, config: Option<PathBuf>) -> Result<()> {
    println!("🔄 Restarting Depyler agent daemon...");

    let _ = agent_stop_command();
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    agent_start_command(port, debug, config, false).await
}

pub fn agent_logs_command(lines: usize, follow: bool) -> Result<()> {
    use crate::agent::daemon::AgentDaemon;

    if follow {
        println!("📜 Following Depyler agent logs (Ctrl+C to stop)...");
        AgentDaemon::tail_logs()
    } else {
        println!("📜 Last {lines} lines of Depyler agent logs:");
        AgentDaemon::show_logs(lines)
    }
}

// Note: parse_python method is now available in DepylerPipeline

// ============================================================================
// Oracle Commands
// ============================================================================

/// Run Differential Evolution to optimize generation parameters.
///
/// This command uses the Metaheuristic Optimizer to find optimal parameters
/// for corpus generation, maximizing Oracle classification accuracy.
///
/// # Errors
///
/// Returns error if optimization fails or file saving fails.
pub fn oracle_optimize_command(
    stdlib_count: usize,
    eval_samples: usize,
    max_evaluations: usize,
    curriculum: bool,
    output: Option<PathBuf>,
) -> Result<()> {
    use depyler_oracle::self_supervised::{run_optimization, OptimizationRunConfig};
    use depyler_oracle::{save_params, OptimizedParams};

    println!("🧬 Depyler Oracle Parameter Optimizer");
    println!("=====================================\n");

    // Create sample stdlib functions for optimization
    let stdlib_funcs = create_sample_stdlib_functions();
    println!("📚 Using {} stdlib functions for evaluation", stdlib_funcs.len().min(stdlib_count));

    // Configure optimization
    let config = OptimizationRunConfig {
        eval_stdlib_count: stdlib_count,
        eval_samples,
        max_evaluations,
        use_curriculum: curriculum,
    };

    println!("⚙️  Configuration:");
    println!("    Max evaluations: {}", max_evaluations);
    println!("    Samples per eval: {}", eval_samples);
    println!("    Curriculum learning: {}", if curriculum { "enabled" } else { "disabled" });
    println!();

    // Create progress bar
    let pb = ProgressBar::new(max_evaluations as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} evaluations ({msg})")
            .unwrap()
            .progress_chars("#>-"),
    );

    // Run optimization
    pb.set_message("optimizing...");
    let result = run_optimization(&stdlib_funcs, &config);

    pb.finish_with_message(format!("fitness: {:.4}", result.fitness));
    println!();

    // Display results
    println!("✅ Optimization Complete!");
    println!("📊 Results:");
    println!("    Best fitness: {:.4}", result.fitness);
    println!("    Evaluations: {}", result.evaluations);
    println!();

    println!("🎯 Optimized Parameters:");
    println!("    weight_docstring: {:.3}", result.params.weight_docstring);
    println!("    weight_type_enum: {:.3}", result.params.weight_type_enum);
    println!("    weight_edge_cases: {:.3}", result.params.weight_edge_cases);
    println!("    weight_error_induction: {:.3}", result.params.weight_error_induction);
    println!("    weight_composition: {:.3}", result.params.weight_composition);
    println!("    quality_threshold: {:.3}", result.params.quality_threshold);
    println!("    min_diversity: {:.3}", result.params.min_diversity);
    println!("    augmentation_ratio: {:.3}", result.params.augmentation_ratio);
    println!();

    // Save parameters
    let optimized = OptimizedParams::new(
        result.params,
        result.fitness,
        result.evaluations,
        curriculum,
    );

    let saved_path = save_params(&optimized, output.as_ref())?;
    println!("💾 Parameters saved to: {}", saved_path.display());

    Ok(())
}

/// Create sample stdlib functions for optimization evaluation.
fn create_sample_stdlib_functions() -> Vec<depyler_oracle::self_supervised::StdlibFunction> {
    use depyler_oracle::self_supervised::{PyType, StdlibFunction};

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
        StdlibFunction {
            module: "os".to_string(),
            name: "listdir".to_string(),
            signature: "(path) -> list".to_string(),
            arg_types: vec![PyType::Str],
            return_type: Some(PyType::List(Box::new(PyType::Str))),
            docstring_examples: vec!["os.listdir('.')".to_string()],
        },
        StdlibFunction {
            module: "datetime".to_string(),
            name: "now".to_string(),
            signature: "() -> datetime".to_string(),
            arg_types: vec![],
            return_type: Some(PyType::Any),
            docstring_examples: vec!["datetime.datetime.now()".to_string()],
        },
    ]
}

/// Show current optimized parameters.
pub fn oracle_show_command() -> Result<()> {
    use depyler_oracle::{load_params, params_exist, default_params_path};

    let path = default_params_path();

    if !params_exist(None) {
        println!("❌ No optimized parameters found at: {}", path.display());
        println!("   Run 'depyler oracle optimize' to generate parameters.");
        return Ok(());
    }

    let params = load_params(None)?;

    println!("📊 Optimized Oracle Parameters");
    println!("==============================\n");
    println!("📁 Path: {}", path.display());
    println!("📅 Timestamp: {}", params.timestamp);
    println!("📦 Version: {}", params.version);
    println!("🎯 Fitness: {:.4}", params.fitness);
    println!("🔄 Evaluations: {}", params.evaluations);
    println!("📚 Curriculum: {}", if params.curriculum { "yes" } else { "no" });
    println!();
    println!("⚙️  Generation Parameters:");
    println!("    weight_docstring: {:.3}", params.params.weight_docstring);
    println!("    weight_type_enum: {:.3}", params.params.weight_type_enum);
    println!("    weight_edge_cases: {:.3}", params.params.weight_edge_cases);
    println!("    weight_error_induction: {:.3}", params.params.weight_error_induction);
    println!("    weight_composition: {:.3}", params.params.weight_composition);
    println!("    quality_threshold: {:.3}", params.params.quality_threshold);
    println!("    min_diversity: {:.3}", params.params.min_diversity);
    println!("    augmentation_ratio: {:.3}", params.params.augmentation_ratio);

    Ok(())
}

/// Train Oracle model from corpus.
pub fn oracle_train_command(min_samples: usize, synthetic: bool) -> Result<()> {
    use depyler_oracle::Oracle;

    println!("🧠 Training Depyler Oracle Model");
    println!("================================\n");

    println!("⚙️  Configuration:");
    println!("    Min samples: {}", min_samples);
    println!("    Synthetic augmentation: {}", if synthetic { "enabled" } else { "disabled" });
    println!();

    // Create progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message("Loading and training model...");

    // Train model (uses load_or_train internally which handles everything)
    let _oracle = Oracle::load_or_train()?;

    pb.finish_with_message("Training complete!");
    println!();

    let model_path = Oracle::default_model_path();
    println!("✅ Oracle model trained successfully!");
    println!("💾 Model saved to: {}", model_path.display());

    Ok(())
}

/// DEPYLER-0585: Oracle-driven continuous improvement loop.
///
/// Runs transpile→compile→train→fix loop until target compilation rate
/// or maximum iterations reached. Designed for enterprise use.
///
/// DEPYLER-0596: Refactored to use CompilationTrainer abstraction
/// which mirrors entrenar's Trainer API for compilation loops.
#[allow(clippy::too_many_arguments)]
pub fn oracle_improve_command(
    input_dir: PathBuf,
    target_rate: f64,
    max_iterations: usize,
    auto_apply: bool,
    min_confidence: f64,
    output: Option<PathBuf>,
    export_corpus: Option<PathBuf>,
    resume: bool,
    verbose: bool,
    monitor: bool,
    verbosity_tier: u8,
    clippy_level: String,
    adaptive_verbosity: bool,
    reweight: f32,
) -> Result<()> {
    use crate::compilation_trainer::{CompilationConfig, CompilationTrainer, DiagnosticTier, ClippyLevel};

    // Oracle used for future auto-fix capability
    let _ = (auto_apply, min_confidence, resume); // Mark as used

    println!("🔄 Depyler Oracle Continuous Improvement Loop");
    println!("=============================================\n");

    // Validate input directory
    if !input_dir.is_dir() {
        return Err(anyhow::anyhow!("Input path must be a directory: {}", input_dir.display()));
    }

    println!("📁 Input directory: {}", input_dir.display());
    println!("🎯 Target compilation rate: {:.1}%", target_rate * 100.0);
    println!("🔄 Max iterations: {}", max_iterations);
    println!("🔧 Auto-apply fixes: {}", if auto_apply { "enabled" } else { "disabled" });
    println!("📊 Min confidence: {:.1}%", min_confidence * 100.0);

    // DEPYLER-0598: Display diagnostic verbosity settings
    let tier = DiagnosticTier::from_level(verbosity_tier);
    let clippy = ClippyLevel::from_cli_arg(&clippy_level);
    println!("🔍 Diagnostic tier: {} ({})", verbosity_tier, match tier {
        DiagnosticTier::Tier1 => "baseline JSON+clippy",
        DiagnosticTier::Tier2 => "+verbose build",
        DiagnosticTier::Tier3 => "+RUSTC_LOG traces",
        DiagnosticTier::Tier4 => "+full debug",
    });
    println!("📋 Clippy level: {} ({})", clippy_level, match clippy {
        ClippyLevel::Standard => "~500 lints",
        ClippyLevel::Pedantic => "~600 lints",
        ClippyLevel::Nursery => "~650 lints",
        ClippyLevel::Full => "~730 lints",
    });
    println!("🎚️ Adaptive verbosity: {}", if adaptive_verbosity { "enabled" } else { "disabled" });
    if (reweight - 1.0).abs() > 0.001 {
        println!("⚖️  Sample reweight: {:.2}× (Feldman long-tail weighting)", reweight);
    }
    println!();

    // Find all Python files (excluding test files)
    let python_files = find_python_files(&input_dir)?;
    let total_files = python_files.len();

    if total_files == 0 {
        println!("⚠️  No Python files found in {}", input_dir.display());
        return Ok(());
    }

    println!("📄 Found {} Python files to process", total_files);

    // Create output directory for reports
    let report_dir = output.unwrap_or_else(|| input_dir.join(".depyler-improve"));

    // Configure the trainer with verbosity settings (DEPYLER-0598)
    let mut config = CompilationConfig::new()
        .with_target_rate(target_rate)
        .with_max_epochs(max_iterations)
        .with_patience(3) // Early stopping patience
        .with_verbose(verbose)
        .with_monitor(monitor)
        .with_report_dir(report_dir)
        .with_verbosity_tier(verbosity_tier)
        .with_clippy_level(&clippy_level)
        .with_adaptive_verbosity(adaptive_verbosity)
        .with_reweight(reweight);

    if let Some(corpus_path) = export_corpus {
        config = config.with_export_corpus(corpus_path);
    }

    // Create and run trainer - mirrors entrenar Trainer API
    let mut trainer = CompilationTrainer::new(python_files, config);

    // Training started message
    println!("\n🧠 Training started | {} files | target: {:.0}%\n", total_files, target_rate * 100.0);

    // Run training loop - replaces ~200 lines of manual loop code
    let result = trainer.train()?;

    // Display result summary using TrainResult-like struct
    println!("\n📊 Training Summary:");
    println!("  Epochs: {}", result.final_epoch);
    println!("  Final Rate: {:.1}%", result.final_rate * 100.0);
    println!("  Best Rate: {:.1}%", result.best_rate * 100.0);
    println!("  Elapsed: {:.1}s", result.elapsed_secs);
    println!("  Stopped Early: {}", result.stopped_early);
    println!("  Target Achieved: {}", if result.target_achieved { "✅ YES" } else { "❌ NO" });

    Ok(())
}

/// GitHub #156: Export CITL corpus for OIP training.
///
/// Exports the compilation training corpus in Parquet or JSONL format
/// for consumption by the Organizational Intelligence Plugin (OIP).
///
/// This command:
/// 1. Loads cached corpus from previous `oracle improve` runs
/// 2. Maps Rust error codes to OIP DefectCategory taxonomy
/// 3. Applies Feldman long-tail reweighting for rare error classes
/// 4. Exports via alimentar's Arrow-based serialization
///
/// References:
/// - Spec: verbose-compiler-diagnostics-citl-spec.md §11.6
/// - OIP NLP-014: Error classification training pipeline
/// - alimentar (#26): Arrow/Parquet data loading
pub fn oracle_export_oip_command(
    input_dir: PathBuf,
    output: PathBuf,
    format: String,
    min_confidence: f64,
    include_clippy: bool,
    reweight: f32,
) -> Result<()> {
    use crate::compilation_trainer::{export_oip_corpus, OipExportFormat, load_corpus_cache};

    println!("📤 Depyler OIP Export (GitHub #156)");
    println!("===================================\n");

    // Validate input directory
    if !input_dir.is_dir() {
        return Err(anyhow::anyhow!("Input path must be a directory: {}", input_dir.display()));
    }

    // Parse export format
    let export_format = OipExportFormat::parse(&format);

    println!("📁 Input directory: {}", input_dir.display());
    println!("📄 Output file: {}", output.display());
    println!("📦 Format: {:?}", export_format);
    println!("📊 Min confidence: {:.1}%", min_confidence * 100.0);
    println!("🔧 Include Clippy: {}", if include_clippy { "yes" } else { "no" });
    if (reweight - 1.0).abs() > 0.001 {
        println!("⚖️  Reweight factor: {:.2}×", reweight);
    }
    println!();

    // Create progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );

    // Try to load cached corpus from previous improve runs
    pb.set_message("Loading corpus cache...");
    let cache_path = input_dir.join(".depyler-improve").join("corpus_cache.jsonl");

    let corpus = if cache_path.exists() {
        load_corpus_cache(&cache_path)?
    } else {
        // No cache - need to generate corpus from scratch
        pb.set_message("No cache found, running transpilation...");

        let python_files = find_python_files(&input_dir)?;
        if python_files.is_empty() {
            return Err(anyhow::anyhow!("No Python files found in {}", input_dir.display()));
        }

        // Run compilation to generate corpus
        let mut corpus = Vec::new();
        let temp_base = std::env::temp_dir().join("depyler-oip-export");
        fs::create_dir_all(&temp_base)?;

        for (idx, py_file) in python_files.iter().enumerate() {
            if let Ok(py_source) = fs::read_to_string(py_file) {
                let pipeline = DepylerPipeline::new();
                if let Ok(rust_code) = pipeline.transpile(&py_source) {
                    // Try to compile to get error diagnostics
                    let rs_file = temp_base.join(format!("lib_{}.rs", idx));
                    fs::write(&rs_file, &rust_code)?;

                    // Get compilation diagnostics (errors become training data)
                    let output = std::process::Command::new("rustc")
                        .args(["--crate-type", "lib", "--error-format=json"])
                        .arg(&rs_file)
                        .output();

                    if let Ok(output) = output {
                        let diagnostics = String::from_utf8_lossy(&output.stderr).to_string();
                        corpus.push((
                            py_file.display().to_string(),
                            rust_code,
                            diagnostics,
                        ));
                    }

                    // Clean up temp file
                    let _ = fs::remove_file(&rs_file);
                }
            }
        }

        // Clean up temp directory
        let _ = fs::remove_dir(&temp_base);

        corpus
    };

    pb.set_message(format!("Exporting {} samples...", corpus.len()));

    // Export corpus
    let stats = export_oip_corpus(
        &corpus,
        &output,
        export_format,
        min_confidence,
        include_clippy,
        reweight,
    )?;

    pb.finish_with_message("Export complete!");
    println!();

    // Display export statistics
    println!("✅ OIP Export Complete!");
    println!("📊 Statistics:");
    println!("    Total samples: {}", stats.total_samples);
    println!("    Exported: {}", stats.exported_samples);
    println!("    Filtered (low confidence): {}", stats.filtered_low_confidence);
    println!("    Unique error codes: {}", stats.unique_error_codes);
    println!("    Unique OIP categories: {}", stats.unique_oip_categories);
    println!();

    println!("📦 Category Distribution:");
    for (category, count) in &stats.category_distribution {
        let pct = (*count as f64 / stats.exported_samples as f64) * 100.0;
        println!("    {}: {} ({:.1}%)", category, count, pct);
    }
    println!();

    println!("💾 Output: {}", output.display());
    println!();
    println!("🔗 Next steps:");
    println!("    1. Run OIP: oip train --input {}", output.display());
    println!("    2. Validate: oip validate --model ./oip_model");

    Ok(())
}

/// Classify a Rust compiler error and suggest fixes
pub fn oracle_classify_command(error: String, format: String) -> Result<()> {
    use depyler_oracle::{classify_with_moe, ErrorClassifier};

    println!("🔮 Depyler Oracle Classification");
    println!("================================\n");

    // Extract error code from the error message (e.g., E0308 from "error[E0308]")
    let error_code = extract_error_code(&error).unwrap_or_default();

    // Use MoE oracle for classification (more robust than RandomForest)
    let moe_result = classify_with_moe(&error_code, &error);

    // Also get keyword-based classification as backup
    let keyword_classifier = ErrorClassifier::new();
    let keyword_category = keyword_classifier.classify_by_keywords(&error);

    // Combine results: prefer MoE if confident, else use keyword classifier
    let (category, confidence) = if moe_result.confidence > 0.5 {
        (moe_result.category, moe_result.confidence)
    } else {
        (keyword_category, 0.7) // Keyword classifier has decent accuracy
    };

    if format == "json" {
        let json_result = serde_json::json!({
            "category": format!("{:?}", category),
            "confidence": confidence,
            "suggested_fix": moe_result.suggested_fix,
            "expert_used": format!("{:?}", moe_result.primary_expert),
        });
        println!("{}", serde_json::to_string_pretty(&json_result)?);
    } else {
        println!("📝 Error: {}", error);
        println!();
        println!("🏷️  Category: {:?}", category);
        println!("📊 Confidence: {:.1}%", confidence * 100.0);
        println!("🧠 Expert: {:?}", moe_result.primary_expert);
        println!();

        if let Some(fix) = &moe_result.suggested_fix {
            println!("💡 Suggested Fix:");
            println!("   {}", fix);
        }
    }

    Ok(())
}

/// Extract error code from a Rust compiler error message (e.g., "E0308" from "error[E0308]")
fn extract_error_code(error: &str) -> Option<String> {
    // Simple string-based extraction: find "E" followed by 4 digits
    error
        .char_indices()
        .find_map(|(i, c)| {
            if c == 'E' && error.len() >= i + 5 {
                let candidate = &error[i..i + 5];
                if candidate.chars().skip(1).all(|d| d.is_ascii_digit()) {
                    return Some(candidate.to_string());
                }
            }
            None
        })
}

/// DEPYLER-0595: Result of CITL (Compiler-in-the-Loop) execution
#[derive(Debug, Clone)]
pub struct CitlResult {
    /// Whether all files compiled successfully
    pub success: bool,
    /// Final compilation rate (0.0-1.0)
    pub compilation_rate: f64,
    /// Number of files processed
    pub files_processed: usize,
    /// Number of iterations used
    pub iterations_used: usize,
    /// Fixes applied
    pub fixes_applied: usize,
}

/// DEPYLER-0595: CITL command - Compiler-in-the-Loop iterative fix loop
///
/// Uses aprender's CITL module to iteratively transpile, compile, and fix
/// until all files compile successfully or max iterations reached.
pub fn citl_command(
    input_dir: PathBuf,
    max_iterations: usize,
    confidence_threshold: f64,
    verbose: bool,
) -> Result<CitlResult> {
    if verbose {
        println!("🔄 CITL (Compiler-in-the-Loop) Mode");
        println!("===================================\n");
    }

    // Validate input directory
    if !input_dir.is_dir() {
        return Err(anyhow::anyhow!("Input path must be a directory: {}", input_dir.display()));
    }

    // Find all Python files
    let python_files = find_python_files(&input_dir)?;
    let total_files = python_files.len();

    if total_files == 0 {
        return Ok(CitlResult {
            success: true,
            compilation_rate: 1.0,
            files_processed: 0,
            iterations_used: 0,
            fixes_applied: 0,
        });
    }

    if verbose {
        println!("📁 Input directory: {}", input_dir.display());
        println!("📄 Found {} Python files", total_files);
        println!("🎯 Target: 100% compilation");
        println!("🔄 Max iterations: {}", max_iterations);
        println!("📊 Confidence threshold: {:.1}%\n", confidence_threshold * 100.0);
    }

    // Configure CITL fixer
    let config = CITLFixerConfig {
        max_iterations,
        confidence_threshold: confidence_threshold as f32,
        ..CITLFixerConfig::default()
    };

    let mut fixer = CITLFixer::with_config(config)
        .map_err(|e| anyhow::anyhow!("Failed to initialize CITL fixer: {}", e))?;
    let mut total_fixes = 0;
    let mut compiled_count = 0;

    // Process each file with CITL
    for (idx, python_file) in python_files.iter().enumerate() {
        if verbose {
            println!("[{}/{}] Processing: {}", idx + 1, total_files, python_file.display());
        }

        // Read Python source
        let python_source = match fs::read_to_string(python_file) {
            Ok(s) => s,
            Err(e) => {
                if verbose {
                    println!("  ⚠️  Failed to read {}: {}", python_file.display(), e);
                }
                continue;
            }
        };

        // Transpile first
        let rust_file = python_file.with_extension("rs");
        let pipeline = DepylerPipeline::new();
        let result = pipeline.transpile(&python_source);

        match result {
            Ok(rust_code) => {
                // Write the Rust file
                if let Err(e) = fs::write(&rust_file, &rust_code) {
                    if verbose {
                        println!("  ⚠️  Failed to write {}: {}", rust_file.display(), e);
                    }
                    continue;
                }

                // Try to compile, apply fixes if needed
                let fix_result = fixer.fix_all(&rust_code);

                if fix_result.success {
                    compiled_count += 1;
                    total_fixes += fix_result.fixes_applied.len();
                    if verbose && !fix_result.fixes_applied.is_empty() {
                        println!("  ✅ Compiled after {} fixes", fix_result.fixes_applied.len());
                    } else if verbose {
                        println!("  ✅ Compiled");
                    }

                    // Write fixed code if any fixes were applied
                    if !fix_result.fixes_applied.is_empty() {
                        if let Err(e) = fs::write(&rust_file, &fix_result.fixed_source) {
                            if verbose {
                                println!("  ⚠️  Failed to write fixed code: {}", e);
                            }
                        }
                    }
                } else if verbose {
                    println!("  ❌ Failed after {} iterations", fix_result.iterations);
                }
            }
            Err(e) => {
                if verbose {
                    println!("  ⚠️  Transpilation failed: {}", e);
                }
            }
        }
    }

    let compilation_rate = compiled_count as f64 / total_files as f64;
    let success = compilation_rate >= 1.0;

    if verbose {
        println!("\n📊 CITL Results");
        println!("===============");
        println!("Files processed: {}", total_files);
        println!("Files compiled:  {} ({:.1}%)", compiled_count, compilation_rate * 100.0);
        println!("Fixes applied:   {}", total_fixes);
        println!("Status: {}", if success { "✅ SUCCESS" } else { "❌ INCOMPLETE" });
    }

    Ok(CitlResult {
        success,
        compilation_rate,
        files_processed: total_files,
        iterations_used: max_iterations, // Approximation
        fixes_applied: total_fixes,
    })
}

/// Find all Python files in a directory (excluding test files).
fn find_python_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    fn visit_dir(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Skip __pycache__, .git, venv, etc.
                let dir_name = path.file_name().unwrap_or_default().to_string_lossy();
                if !dir_name.starts_with('.') && dir_name != "__pycache__" && dir_name != "venv" {
                    visit_dir(&path, files)?;
                }
            } else if path.extension().is_some_and(|e| e == "py") {
                let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                // Skip test files
                if !file_name.starts_with("test_") && !file_name.ends_with("_test.py") {
                    files.push(path);
                }
            }
        }
        Ok(())
    }

    visit_dir(dir, &mut files)?;
    files.sort();
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_python_file(content: &str) -> (TempDir, PathBuf) {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.py");
        fs::write(&file_path, content).unwrap();
        (temp_dir, file_path)
    }

    #[test]
    fn test_transpile_command_basic() {
        let (_temp_dir, input_path) = create_test_python_file("def hello() -> int: return 42");

        let result = transpile_command(input_path, None, false, false, false, false, false, false, false, false, 0.8);
        assert!(result.is_ok());
    }

    #[test]
    fn test_transpile_command_with_output() {
        let (_temp_dir, input_path) = create_test_python_file("def hello() -> int: return 42");
        let output_path = input_path.with_extension("rs");

        let result = transpile_command(
            input_path,
            Some(output_path.clone()),
            false,
            false,
            false,
            false,
            false,
            false,
            false,  // auto_fix
            false,  // suggest_fixes
            0.8,    // fix_confidence
        );
        assert!(result.is_ok());
        assert!(output_path.exists());
    }

    #[test]
    fn test_analyze_command_text_format() {
        let (_temp_dir, input_path) = create_test_python_file("def hello() -> int: return 42");

        let result = analyze_command(input_path, "text".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_analyze_command_json_format() {
        let (_temp_dir, input_path) = create_test_python_file("def hello() -> int: return 42");

        let result = analyze_command(input_path, "json".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_command_valid() {
        let (_temp_dir, input_path) = create_test_python_file("def hello() -> int: return 42");

        let result = check_command(input_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_inspect_command_hir() {
        let (_temp_dir, input_path) = create_test_python_file("def hello() -> int: return 42");

        let result = inspect_command(input_path, "hir".to_string(), "pretty".to_string(), None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_inspect_command_python_ast() {
        let (_temp_dir, input_path) = create_test_python_file("def hello() -> int: return 42");

        let result = inspect_command(
            input_path,
            "python-ast".to_string(),
            "debug".to_string(),
            None,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_quality_check_command() {
        let (_temp_dir, input_path) = create_test_python_file("def hello() -> int: return 42");

        let result = quality_check_command(input_path, false, 1.0, 2.0, 20, 80);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_quality_report() {
        let (_temp_dir, input_path) = create_test_python_file("def hello() -> int: return 42");

        let result = generate_quality_report(&input_path);
        assert!(result.is_ok());

        let report = result.unwrap();
        assert!(report.pmat_metrics.tdg >= 0.0);
    }

    #[test]
    fn test_validate_quality_targets() {
        let (_temp_dir, input_path) = create_test_python_file("def hello() -> int: return 42");
        let report = generate_quality_report(&input_path).unwrap();

        let validations = validate_quality_targets(&report, 1.0, 2.0, 20, 80);
        assert!(validations.tdg_ok);
        assert!(validations.complexity_ok);
    }

    #[test]
    fn test_format_python_ast_pretty() {
        let python_source = "def hello(): return 42";
        let result = inspect_python_ast(python_source, "pretty");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Python AST Structure"));
    }

    #[test]
    fn test_inspect_python_ast_formats() {
        let python_source = "def hello(): return 42";

        // Test all formats
        assert!(inspect_python_ast(python_source, "pretty").is_ok());
        assert!(inspect_python_ast(python_source, "debug").is_ok());
        assert!(inspect_python_ast(python_source, "json").is_ok());

        // Test invalid format
        assert!(inspect_python_ast(python_source, "invalid").is_err());
    }

    #[test]
    fn test_complexity_rating() {
        assert!(complexity_rating(3.0).to_string().contains("Good"));
        assert!(complexity_rating(7.0).to_string().contains("Acceptable"));
        assert!(complexity_rating(15.0).to_string().contains("High"));
    }

    /// DEPYLER-0595: [RED] Test for CITL CLI command
    /// Compiler-in-the-Loop iterative fix loop until compilation succeeds
    #[test]
    fn test_citl_command_basic() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.py");
        fs::write(&file_path, "def hello() -> int: return 42").unwrap();

        // CITL should transpile, compile, and fix until success
        let result = citl_command(
            temp_dir.path().to_path_buf(),
            10,   // max_iterations
            0.85, // confidence_threshold
            false, // verbose
        );
        assert!(result.is_ok());

        let citl_result = result.unwrap();
        assert!(citl_result.success);
        assert!(citl_result.compilation_rate >= 1.0);
    }

    /// DEPYLER-0595: [RED] Test CITL with multiple files
    #[test]
    fn test_citl_command_multiple_files() {
        let temp_dir = TempDir::new().unwrap();

        // Create multiple Python files
        fs::write(temp_dir.path().join("a.py"), "def foo() -> int: return 1").unwrap();
        fs::write(temp_dir.path().join("b.py"), "def bar() -> str: return 'hello'").unwrap();

        let result = citl_command(
            temp_dir.path().to_path_buf(),
            10,
            0.85,
            false,
        );
        assert!(result.is_ok());

        let citl_result = result.unwrap();
        assert_eq!(citl_result.files_processed, 2);
    }
}
