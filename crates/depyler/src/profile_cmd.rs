//! Profiling command for performance analysis
//!
//! This module provides the CLI command for profiling Python code and
//! analyzing performance characteristics of the transpiled Rust code.

use anyhow::Result;
use clap::Args;
use depyler_core::{
    hir::HirProgram,
    profiling::{ProfileConfig, Profiler},
    DepylerPipeline,
};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct ProfileArgs {
    /// Path to Python file to profile
    pub file: PathBuf,

    /// Enable instruction counting
    #[arg(long, default_value = "true")]
    pub count_instructions: bool,

    /// Enable memory allocation tracking
    #[arg(long, default_value = "true")]
    pub track_allocations: bool,

    /// Enable hot path detection
    #[arg(long, default_value = "true")]
    pub detect_hot_paths: bool,

    /// Minimum samples for hot path detection
    #[arg(long, default_value = "100")]
    pub hot_path_threshold: usize,

    /// Generate flame graph data
    #[arg(long)]
    pub flamegraph: bool,

    /// Include performance hints
    #[arg(long, default_value = "true")]
    pub hints: bool,

    /// Output flamegraph data to file
    #[arg(long)]
    pub flamegraph_output: Option<PathBuf>,

    /// Output perf annotations to file
    #[arg(long)]
    pub perf_output: Option<PathBuf>,
}

pub fn handle_profile_command(args: ProfileArgs) -> Result<()> {
    // Read the Python source file
    let source = fs::read_to_string(&args.file)?;

    // Create pipeline and parse to HIR
    let pipeline = DepylerPipeline::new();
    let hir = pipeline.parse_to_hir(&source)?;

    // Convert to HirProgram for profiling
    let hir_program = HirProgram {
        functions: hir.functions,
        classes: hir.classes,
        imports: hir.imports,
    };

    // Create profiler with configuration
    let config = ProfileConfig {
        count_instructions: args.count_instructions,
        track_allocations: args.track_allocations,
        detect_hot_paths: args.detect_hot_paths,
        hot_path_threshold: args.hot_path_threshold,
        generate_flamegraph: args.flamegraph,
        include_hints: args.hints,
    };

    let mut profiler = Profiler::new(config);

    // Analyze the program
    let report = profiler.analyze_program(&hir_program);

    // Display the main report
    println!("{}", report.format_report());

    // Generate flamegraph data if requested
    if args.flamegraph {
        let flamegraph_data = report.generate_flamegraph_data();

        if let Some(output_path) = args.flamegraph_output {
            fs::write(output_path, flamegraph_data)?;
            println!("\n🔥 Flamegraph data written to file");
        } else {
            println!("\n🔥 Flamegraph Data (collapsed format):");
            println!("{}", flamegraph_data);
        }
    }

    // Generate perf annotations if requested
    if let Some(output_path) = args.perf_output {
        let perf_annotations = report.generate_perf_annotations();
        fs::write(output_path, perf_annotations)?;
        println!("\n📊 Perf annotations written to file");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_profile_command_basic() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.py");

        let python_code = r#"
def compute_fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return compute_fibonacci(n - 1) + compute_fibonacci(n - 2)

def main():
    for i in range(10):
        result = compute_fibonacci(i)
        print(result)
"#;

        fs::write(&file_path, python_code).unwrap();

        let args = ProfileArgs {
            file: file_path,
            count_instructions: true,
            track_allocations: true,
            detect_hot_paths: true,
            hot_path_threshold: 100,
            flamegraph: false,
            hints: true,
            flamegraph_output: None,
            perf_output: None,
        };

        let result = handle_profile_command(args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_profile_with_flamegraph() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.py");
        let flamegraph_path = dir.path().join("flamegraph.txt");

        let python_code = r#"
def hot_function():
    total = 0
    for i in range(1000):
        for j in range(1000):
            total += i * j
    return total
"#;

        fs::write(&file_path, python_code).unwrap();

        let args = ProfileArgs {
            file: file_path,
            count_instructions: true,
            track_allocations: true,
            detect_hot_paths: true,
            hot_path_threshold: 10,
            flamegraph: true,
            hints: true,
            flamegraph_output: Some(flamegraph_path.clone()),
            perf_output: None,
        };

        let result = handle_profile_command(args);
        assert!(result.is_ok());

        // Check that flamegraph file was created
        assert!(flamegraph_path.exists());
        let content = fs::read_to_string(&flamegraph_path).unwrap();
        assert!(content.contains("hot_function"));
    }
}
