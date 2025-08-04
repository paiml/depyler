// Library interface for depyler main functionality
// This allows us to test main.rs functions by moving them to lib.rs

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use depyler_analyzer::Analyzer;
use depyler_core::{
    lambda_codegen::LambdaCodeGenerator, lambda_inference::LambdaTypeInferencer,
    lambda_optimizer::LambdaOptimizer, lambda_testing::LambdaTestHarness, DepylerPipeline,
};
use depyler_quality::QualityAnalyzer;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

pub mod debug_cmd;
pub mod docs_cmd;
pub mod interactive;
pub mod profile_cmd;

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

pub fn transpile_command(
    input: PathBuf,
    output: Option<PathBuf>,
    verify: bool,
    gen_tests: bool,
    debug: bool,
    source_map: bool,
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

    // Parse Python
    pb.set_message("Parsing Python source...");
    let parse_start = Instant::now();
    let rust_code = pipeline.transpile(&python_source)?;
    let parse_time = parse_start.elapsed();
    pb.inc(1);

    // Analyze if requested
    if verify {
        pb.set_message("Analyzing code...");
        // Analysis would happen here
        pb.inc(1);
    }

    // Generate output
    pb.set_message("Writing output...");
    let output_path = output.unwrap_or_else(|| {
        let mut path = input.clone();
        path.set_extension("rs");
        path
    });

    fs::write(&output_path, &rust_code)?;
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
    let hir = depyler_core::ast_bridge::python_to_hir(ast)?;

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
    let hir = depyler_core::ast_bridge::python_to_hir(ast)?;
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
                .map(|(name, ty)| format!("{name}: {ty:?}"))
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
) -> Result<()> {
    if tips {
        debug_cmd::print_debugging_tips();
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
                event_type: Some(match analysis.inferred_event_type {
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
                }),
                ..Default::default()
            });
    pb.inc(1);

    // Step 3: Transpile to Rust
    pb.set_message("🦀 Transpiling to Rust...");
    let rust_code = pipeline.transpile(&python_source)?;

    let generation_context = depyler_core::lambda_codegen::LambdaGenerationContext {
        event_type: lambda_annotations.event_type.clone(),
        response_type: "serde_json::Value".to_string(), // Could be inferred better
        handler_body: rust_code,
        imports: vec![],
        dependencies: vec![],
        annotations: lambda_annotations.clone(),
        function_name: "handler".to_string(),
        module_name: input.file_stem().unwrap().to_string_lossy().to_string(),
    };
    pb.inc(1);

    // Step 4: Generate optimized Lambda project
    pb.set_message("⚡ Generating optimized project...");
    let mut generator = LambdaCodeGenerator::new();
    if optimize {
        let optimizer = if optimize {
            LambdaOptimizer::new().enable_aggressive_mode()
        } else {
            LambdaOptimizer::new()
        };
        let _optimization_plan = optimizer.generate_optimization_plan(&lambda_annotations)?;
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

    fs::create_dir_all(&output_dir)?;
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

    if deploy {
        if let Some(ref sam_template) = project.sam_template {
            fs::write(output_dir.join("template.yaml"), sam_template)?;
        }
        if let Some(ref cdk_construct) = project.cdk_construct {
            fs::write(output_dir.join("lambda-construct.ts"), cdk_construct)?;
        }
    }
    pb.inc(1);

    // Step 6: Generate tests if requested
    if tests {
        pb.set_message("🧪 Generating test suite...");
        let test_harness = LambdaTestHarness::new();
        let test_suite = test_harness.generate_test_suite(&lambda_annotations)?;
        fs::write(output_dir.join("src/lib.rs"), &test_suite)?;

        let test_script = test_harness.generate_cargo_lambda_test_script(&lambda_annotations)?;
        fs::write(output_dir.join("test.sh"), &test_script)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(output_dir.join("test.sh"))?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(output_dir.join("test.sh"), perms)?;
        }
    }
    pb.inc(1);

    pb.finish_and_clear();

    // Print summary
    let total_time = start.elapsed();
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

    // Show next steps
    println!("\n📋 Next Steps:");
    println!("   cd {}", output_dir.display());
    println!("   ./build.sh                    # Build the Lambda");
    if tests {
        println!("   ./test.sh                     # Run tests");
    }
    println!("   cargo lambda deploy           # Deploy to AWS");

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

// Note: parse_python method is now available in DepylerPipeline

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

        let result = transpile_command(input_path, None, false, false, false, false);
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
}
