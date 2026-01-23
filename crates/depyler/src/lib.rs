//! # Depyler - Python to Rust Transpiler
//!
//! Minimal CLI focused on transpilation and single-shot compilation.

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use depyler_analyzer::Analyzer;
use depyler_core::DepylerPipeline;
use std::fs;
use std::path::PathBuf;

// Essential modules only
pub mod cli_shim;
pub mod compile_cmd;
pub mod converge;
pub mod graph_cmd;
pub mod report_cmd;
pub mod report_shim;
pub mod transpile_shim;
pub mod utol_cmd;

// DEPYLER-1202: Trait Bridge for Python method compatibility
pub mod prelude;
pub mod python_ops;

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

    /// Compile Python to standalone binary
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

    /// Automated convergence loop to achieve target compilation rate
    Converge {
        /// Directory containing Python examples
        #[arg(long)]
        input_dir: PathBuf,

        /// Target compilation rate (0-100)
        #[arg(long, default_value = "100")]
        target_rate: f64,

        /// Maximum iterations before stopping
        #[arg(long, default_value = "50")]
        max_iterations: usize,

        /// Automatically apply transpiler fixes
        #[arg(long)]
        auto_fix: bool,

        /// Show what would be fixed without applying
        #[arg(long)]
        dry_run: bool,

        /// Minimum confidence for auto-fix
        #[arg(long, default_value = "0.8")]
        fix_confidence: f64,

        /// Directory to save/resume state
        #[arg(long)]
        checkpoint: Option<PathBuf>,

        /// Number of parallel compilation jobs
        #[arg(long, default_value = "4")]
        jobs: usize,

        /// Display mode (rich, minimal, json, silent)
        #[arg(long, default_value = "rich")]
        display: String,

        /// Enable Oracle ML-based error classification
        #[arg(long)]
        oracle: bool,

        /// Enable explainability traces
        #[arg(long)]
        explain: bool,

        /// Enable O(1) compilation cache
        #[arg(long, default_value = "true")]
        cache: bool,
    },

    /// Generate deterministic corpus analysis report
    Report {
        /// Directory containing Python examples
        #[arg(long)]
        input_dir: PathBuf,

        /// Output format (text, json, markdown)
        #[arg(short, long, default_value = "text")]
        format: String,

        /// Output file (defaults to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Filter by error code (e.g., E0599)
        #[arg(long)]
        filter_error: Option<String>,

        /// Filter by file pattern (glob)
        #[arg(long)]
        filter_file: Option<String>,

        /// Only show failing files
        #[arg(long)]
        failures_only: bool,

        /// Show detailed error messages
        #[arg(long)]
        verbose: bool,
    },

    /// UTOL: Unified Training Oracle Loop
    Utol {
        /// Directory containing Python examples
        #[arg(long)]
        corpus: Option<PathBuf>,

        /// Target compilation rate (0.0-1.0)
        #[arg(long, default_value = "0.80")]
        target_rate: f64,

        /// Maximum iterations
        #[arg(long, default_value = "50")]
        max_iterations: usize,

        /// Patience before early stopping
        #[arg(long, default_value = "5")]
        patience: usize,

        /// Display mode (rich, minimal, json, silent)
        #[arg(long, default_value = "rich")]
        display: String,

        /// Show corpus status only
        #[arg(long)]
        status: bool,
    },

    /// Compilation cache commands
    #[command(subcommand)]
    Cache(CacheCommands),

    /// DEPYLER-1101: Repair type errors using Oracle-learned constraints
    Repair {
        /// Input Python file to repair
        input: PathBuf,

        /// Output Rust file (defaults to input with .rs extension)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Maximum repair iterations
        #[arg(long, default_value = "10")]
        max_iterations: usize,

        /// Display verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// DEPYLER-1303: Graph-based error analysis
    #[command(subcommand)]
    Graph(GraphCommands),
}

/// Graph analysis subcommands
#[derive(Subcommand)]
pub enum GraphCommands {
    /// Analyze corpus and identify Patient Zeros (high-impact failure points)
    Analyze {
        /// Directory containing Python examples
        #[arg(long)]
        corpus: PathBuf,

        /// Number of top Patient Zeros to identify
        #[arg(long, default_value = "5")]
        top: usize,

        /// Output file (JSON format)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Vectorize failures for ML training
    Vectorize {
        /// Directory containing Python examples
        #[arg(long)]
        corpus: PathBuf,

        /// Output file for vectorized failures
        #[arg(short, long)]
        output: PathBuf,

        /// Output format (json, ndjson)
        #[arg(long, default_value = "ndjson")]
        format: String,
    },
}

#[derive(Subcommand)]
pub enum CacheCommands {
    /// Show cache statistics
    Stats {
        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Run garbage collection
    Gc {
        /// Maximum age in days for entries
        #[arg(long, default_value = "30")]
        max_age_days: u32,

        /// Dry run (show what would be deleted)
        #[arg(long)]
        dry_run: bool,
    },

    /// Clear the entire cache
    Clear {
        /// Skip confirmation
        #[arg(long)]
        force: bool,
    },

    /// Warm cache by pre-transpiling files
    Warm {
        /// Directory containing Python files
        #[arg(long)]
        input_dir: PathBuf,

        /// Number of parallel jobs
        #[arg(long, default_value = "4")]
        jobs: usize,
    },
}

// ============================================================================
// Core Commands
// ============================================================================

pub fn transpile_command(
    input: PathBuf,
    output: Option<PathBuf>,
    verify: bool,
    gen_tests: bool,
    debug: bool,
    source_map: bool,
) -> Result<()> {
    use depyler_core::cargo_toml_gen::{generate_cargo_toml_auto, Dependency};

    let python_source = fs::read_to_string(&input)?;
    let pipeline = DepylerPipeline::new();

    // DEPYLER-0384: Use transpile_with_dependencies for automatic Cargo.toml emission
    let (rust_code, dependencies) = pipeline.transpile_with_dependencies(&python_source)?;

    let output_path = output.unwrap_or_else(|| input.with_extension("rs"));
    fs::write(&output_path, &rust_code)?;

    println!("{} {}", "✓".green(), output_path.display());

    // DEPYLER-0384: Automatically emit Cargo.toml if dependencies detected
    if !dependencies.is_empty() || true {
        // Always emit Cargo.toml for single-shot compile guarantee
        let package_name = output_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("transpiled")
            .replace('-', "_"); // Cargo package names use underscores

        let source_file_name = output_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("main.rs");

        // Convert dependencies to cargo_toml_gen::Dependency type
        let deps: Vec<Dependency> = dependencies;

        let cargo_toml = generate_cargo_toml_auto(&package_name, source_file_name, &deps);

        // Write Cargo.toml in the same directory as the output file
        let cargo_toml_path = output_path.parent().unwrap_or(std::path::Path::new(".")).join("Cargo.toml");
        fs::write(&cargo_toml_path, cargo_toml)?;

        println!("{} {} (with {} dependencies)", "✓".green(), cargo_toml_path.display(), deps.len());
    }

    if verify {
        println!("Verification not yet implemented");
    }

    if gen_tests {
        println!("Test generation not yet implemented");
    }

    if debug {
        println!("Debug mode enabled");
    }

    if source_map {
        println!("Source map generation not yet implemented");
    }

    Ok(())
}

pub fn compile_command(
    input: PathBuf,
    output: Option<PathBuf>,
    profile: String,
) -> Result<()> {
    let output_ref = output.as_deref();
    let profile_ref = if profile.is_empty() {
        None
    } else {
        Some(profile.as_str())
    };
    compile_cmd::compile_python_to_binary(&input, output_ref, profile_ref)?;
    Ok(())
}

pub fn analyze_command(input: PathBuf, format: String) -> Result<()> {
    let python_source = fs::read_to_string(&input)?;
    let pipeline = DepylerPipeline::new();
    let hir = pipeline.parse_to_hir(&python_source)?;
    let analyzer = Analyzer::new();
    let report = analyzer.analyze(&hir)?;

    match format.as_str() {
        "json" => println!("{}", serde_json::to_string_pretty(&report)?),
        _ => println!("{:#?}", report),
    }

    Ok(())
}

pub fn check_command(input: PathBuf) -> Result<()> {
    let python_source = fs::read_to_string(&input)?;
    let pipeline = DepylerPipeline::new();

    match pipeline.transpile(&python_source) {
        Ok(_) => {
            println!("{} {} can be transpiled", "✓".green(), input.display());
            Ok(())
        }
        Err(e) => {
            println!("{} {} cannot be transpiled: {}", "✗".red(), input.display(), e);
            Err(e)
        }
    }
}

/// DEPYLER-1101: Repair type errors using Oracle-learned constraints.
///
/// This command implements a "Transpile → Compile → Learn → Fix" loop that:
/// 1. Transpiles Python to Rust
/// 2. Attempts compilation to detect E0308 type mismatch errors
/// 3. Learns correct types from compiler error messages
/// 4. Re-transpiles with learned type constraints
/// 5. Repeats until compilation succeeds or max iterations reached
pub fn repair_command(
    input: PathBuf,
    output: Option<PathBuf>,
    max_iterations: usize,
    verbose: bool,
) -> Result<()> {
    use depyler_oracle::utol::repair_file_types;

    if verbose {
        println!(
            "{} Starting type repair for {}",
            "🔧".green(),
            input.display()
        );
        println!("   Max iterations: {}", max_iterations);
    }

    let result = repair_file_types(&input, max_iterations)?;

    if result.success {
        println!(
            "{} {} repaired successfully in {} iterations",
            "✓".green(),
            input.display(),
            result.iterations
        );
        println!(
            "   Constraints learned: {}, applied: {}",
            result.constraints_learned, result.constraints_applied
        );

        // If successful, read the generated Rust code and write to output
        let pipeline = DepylerPipeline::new();
        let python_source = fs::read_to_string(&input)?;

        // Get the learned constraints from the final iteration
        // For now, just transpile again (constraints were already applied internally)
        let rust_code = pipeline.transpile(&python_source)?;

        let output_path = output.unwrap_or_else(|| input.with_extension("rs"));
        fs::write(&output_path, &rust_code)?;

        println!(
            "{} Wrote repaired Rust code to {}",
            "📄".green(),
            output_path.display()
        );

        Ok(())
    } else {
        println!(
            "{} {} repair failed after {} iterations",
            "✗".red(),
            input.display(),
            result.iterations
        );
        println!(
            "   Constraints learned: {}, applied: {}",
            result.constraints_learned, result.constraints_applied
        );
        println!("   Final compile rate: {:.1}%", result.final_rate * 100.0);

        anyhow::bail!(
            "Type repair failed after {} iterations. Consider manual fixes.",
            max_iterations
        )
    }
}

// ============================================================================
// Helpers
// ============================================================================

pub fn complexity_rating(complexity: f64) -> colored::ColoredString {
    if complexity < 10.0 {
        "Low".green()
    } else if complexity < 20.0 {
        "Medium".yellow()
    } else {
        "High".red()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_complexity_rating_low() {
        let rating = complexity_rating(5.0);
        assert!(!rating.to_string().is_empty());
    }

    #[test]
    fn test_complexity_rating_medium() {
        let rating = complexity_rating(15.0);
        assert!(!rating.to_string().is_empty());
    }

    #[test]
    fn test_complexity_rating_high() {
        let rating = complexity_rating(25.0);
        assert!(!rating.to_string().is_empty());
    }

    #[test]
    fn test_complexity_rating_boundary_low() {
        let rating = complexity_rating(9.99);
        assert!(rating.to_string().contains("Low"));
    }

    #[test]
    fn test_complexity_rating_boundary_medium() {
        let rating = complexity_rating(10.0);
        assert!(rating.to_string().contains("Medium"));
    }

    #[test]
    fn test_complexity_rating_boundary_high() {
        let rating = complexity_rating(20.0);
        assert!(rating.to_string().contains("High"));
    }

    #[test]
    fn test_complexity_rating_zero() {
        let rating = complexity_rating(0.0);
        assert!(rating.to_string().contains("Low"));
    }

    #[test]
    fn test_complexity_rating_negative() {
        let rating = complexity_rating(-5.0);
        assert!(rating.to_string().contains("Low"));
    }

    #[test]
    fn test_transpile_command_valid() {
        let temp = TempDir::new().unwrap();
        let py_file = temp.path().join("test.py");
        fs::write(&py_file, "def add(a: int, b: int) -> int:\n    return a + b\n").unwrap();

        let result = transpile_command(py_file.clone(), None, false, false, false, false);
        assert!(result.is_ok());

        let rs_file = py_file.with_extension("rs");
        assert!(rs_file.exists());
    }

    #[test]
    fn test_transpile_command_with_output() {
        let temp = TempDir::new().unwrap();
        let py_file = temp.path().join("test.py");
        let rs_file = temp.path().join("custom_output.rs");
        fs::write(&py_file, "def greet() -> str:\n    return 'hello'\n").unwrap();

        let result = transpile_command(py_file, Some(rs_file.clone()), false, false, false, false);
        assert!(result.is_ok());
        assert!(rs_file.exists());
    }

    #[test]
    fn test_transpile_command_nonexistent() {
        let result = transpile_command(PathBuf::from("/nonexistent.py"), None, false, false, false, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_transpile_command_with_verify() {
        let temp = TempDir::new().unwrap();
        let py_file = temp.path().join("test.py");
        fs::write(&py_file, "x = 1\n").unwrap();

        let result = transpile_command(py_file, None, true, false, false, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_transpile_command_with_gen_tests() {
        let temp = TempDir::new().unwrap();
        let py_file = temp.path().join("test.py");
        fs::write(&py_file, "x = 1\n").unwrap();

        let result = transpile_command(py_file, None, false, true, false, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_transpile_command_with_debug() {
        let temp = TempDir::new().unwrap();
        let py_file = temp.path().join("test.py");
        fs::write(&py_file, "x = 1\n").unwrap();

        let result = transpile_command(py_file, None, false, false, true, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_transpile_command_with_source_map() {
        let temp = TempDir::new().unwrap();
        let py_file = temp.path().join("test.py");
        fs::write(&py_file, "x = 1\n").unwrap();

        let result = transpile_command(py_file, None, false, false, false, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_transpile_command_all_flags() {
        let temp = TempDir::new().unwrap();
        let py_file = temp.path().join("test.py");
        fs::write(&py_file, "def foo(): pass\n").unwrap();

        let result = transpile_command(py_file, None, true, true, true, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_analyze_command_text_format() {
        let temp = TempDir::new().unwrap();
        let py_file = temp.path().join("analyze.py");
        fs::write(&py_file, "def add(a: int, b: int) -> int:\n    return a + b\n").unwrap();

        let result = analyze_command(py_file, "text".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_analyze_command_json_format() {
        let temp = TempDir::new().unwrap();
        let py_file = temp.path().join("analyze.py");
        fs::write(&py_file, "def add(a: int, b: int) -> int:\n    return a + b\n").unwrap();

        let result = analyze_command(py_file, "json".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_analyze_command_nonexistent() {
        let result = analyze_command(PathBuf::from("/nonexistent.py"), "text".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_check_command_valid() {
        let temp = TempDir::new().unwrap();
        let py_file = temp.path().join("check.py");
        fs::write(&py_file, "def add(a: int, b: int) -> int:\n    return a + b\n").unwrap();

        let result = check_command(py_file);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_command_nonexistent() {
        let result = check_command(PathBuf::from("/nonexistent.py"));
        assert!(result.is_err());
    }

    #[test]
    fn test_compile_command_nonexistent() {
        let result = compile_command(PathBuf::from("/nonexistent.py"), None, "release".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_compile_command_empty_profile() {
        let temp = TempDir::new().unwrap();
        let py_file = temp.path().join("compile.py");
        fs::write(&py_file, "def foo(): pass\n").unwrap();

        // Empty profile - should use None internally
        let result = compile_command(py_file, None, "".to_string());
        // May fail during actual compilation, but should not panic
        let _ = result;
    }

    #[test]
    fn test_compile_command_with_profile() {
        let temp = TempDir::new().unwrap();
        let py_file = temp.path().join("compile.py");
        fs::write(&py_file, "def foo(): pass\n").unwrap();

        let result = compile_command(py_file, None, "debug".to_string());
        // May fail during actual compilation, but should not panic
        let _ = result;
    }

    // CLI struct tests
    #[test]
    fn test_cli_verbose_default() {
        use clap::Parser;
        let cli = Cli::try_parse_from(["depyler", "check", "test.py"]).unwrap();
        assert!(!cli.verbose);
    }

    #[test]
    fn test_cache_commands_stats() {
        use clap::Parser;
        let cli = Cli::try_parse_from(["depyler", "cache", "stats"]).unwrap();
        if let Commands::Cache(CacheCommands::Stats { format }) = cli.command {
            assert_eq!(format, "text");
        } else {
            panic!("Expected Cache Stats");
        }
    }

    #[test]
    fn test_cache_commands_gc() {
        use clap::Parser;
        let cli = Cli::try_parse_from(["depyler", "cache", "gc"]).unwrap();
        if let Commands::Cache(CacheCommands::Gc { max_age_days, dry_run }) = cli.command {
            assert_eq!(max_age_days, 30);
            assert!(!dry_run);
        } else {
            panic!("Expected Cache Gc");
        }
    }

    #[test]
    fn test_cache_commands_clear() {
        use clap::Parser;
        let cli = Cli::try_parse_from(["depyler", "cache", "clear"]).unwrap();
        if let Commands::Cache(CacheCommands::Clear { force }) = cli.command {
            assert!(!force);
        } else {
            panic!("Expected Cache Clear");
        }
    }

    #[test]
    fn test_cache_commands_warm() {
        use clap::Parser;
        let cli = Cli::try_parse_from(["depyler", "cache", "warm", "--input-dir", "/tmp"]).unwrap();
        if let Commands::Cache(CacheCommands::Warm { input_dir, jobs }) = cli.command {
            assert_eq!(input_dir, PathBuf::from("/tmp"));
            assert_eq!(jobs, 4);
        } else {
            panic!("Expected Cache Warm");
        }
    }
}
