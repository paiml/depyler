//! Transpilation shim - pure logic separated from I/O
//!
//! This module contains the testable core logic of transpilation,
//! extracted from the CLI command handlers.

use anyhow::Result;
use depyler_core::{debug::DebugConfig, debug::DebugLevel, DepylerPipeline};
use std::path::{Path, PathBuf};

/// Configuration for transpilation (pure data, no I/O)
#[derive(Debug, Clone, Default)]
pub struct TranspileConfig {
    pub verify: bool,
    pub gen_tests: bool,
    pub debug: bool,
    pub source_map: bool,
    pub trace: bool,
    pub explain: bool,
    pub audit_trail: bool,
    pub auto_fix: bool,
    pub async_mode: bool,
    pub suggest_fixes: bool,
    pub fix_confidence: f64,
    pub oracle: bool,
    pub max_retries: usize,
    pub llm_fallback: bool,
}

/// Strategy for auto-fix operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoFixStrategy {
    /// No auto-fix
    None,
    /// Synchronous ML oracle
    Synchronous,
    /// Async background processing
    Async,
    /// Suggest fixes only (no apply)
    SuggestOnly,
}

/// Confidence level for transpilation results
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfidenceLevel {
    /// High confidence (>= threshold)
    High,
    /// Low confidence (< threshold)
    Low,
    /// No confidence data available
    Unknown,
}

/// Oracle configuration derived from transpile config
#[derive(Debug, Clone)]
pub struct OracleConfig {
    pub enabled: bool,
    pub threshold: f64,
    pub max_retries: usize,
    pub llm_fallback: bool,
}

/// Output path configuration
#[derive(Debug, Clone)]
pub struct OutputPaths {
    pub main_output: PathBuf,
    pub autofix_output: Option<PathBuf>,
    pub test_output: Option<PathBuf>,
    pub cargo_toml: Option<PathBuf>,
    pub source_map: Option<PathBuf>,
}

/// Result of transpilation (pure data, no I/O)
#[derive(Debug, Clone)]
pub struct TranspileResult {
    pub rust_code: String,
    pub source_size: usize,
    pub output_size: usize,
    pub parse_time_ms: u64,
    pub transpile_time_ms: u64,
    pub verification_passed: Option<bool>,
    pub fixes_applied: usize,
}

/// Create a configured pipeline from TranspileConfig
pub fn create_pipeline(config: &TranspileConfig) -> DepylerPipeline {
    let mut pipeline = DepylerPipeline::new();

    if config.verify {
        pipeline = pipeline.with_verification();
    }

    if config.debug || config.source_map {
        let debug_config = DebugConfig {
            debug_level: if config.debug {
                DebugLevel::Full
            } else {
                DebugLevel::Basic
            },
            generate_source_map: config.source_map,
            preserve_symbols: true,
            debug_prints: config.debug,
            breakpoints: config.debug,
        };
        pipeline = pipeline.with_debug(debug_config);
    }

    pipeline
}

/// Transpile Python source to Rust (pure function, no I/O)
pub fn transpile_source(python_source: &str, config: &TranspileConfig) -> Result<TranspileResult> {
    let source_size = python_source.len();

    let parse_start = std::time::Instant::now();
    let pipeline = create_pipeline(config);
    let parse_time_ms = parse_start.elapsed().as_millis() as u64;

    let transpile_start = std::time::Instant::now();
    let rust_code = pipeline.transpile(python_source)?;
    let transpile_time_ms = transpile_start.elapsed().as_millis() as u64;

    let output_size = rust_code.len();

    Ok(TranspileResult {
        rust_code,
        source_size,
        output_size,
        parse_time_ms,
        transpile_time_ms,
        verification_passed: None,
        fixes_applied: 0,
    })
}

/// Generate trace output for a transpilation phase
pub fn format_trace_phase(phase: u8, name: &str, details: &[(&str, &str)]) -> String {
    let mut output = format!("Phase {}: {}\n", phase, name);
    for (key, value) in details {
        output.push_str(&format!("  - {}: {}\n", key, value));
    }
    output
}

/// Calculate transpilation metrics
pub fn calculate_metrics(result: &TranspileResult) -> TranspileMetrics {
    let compression_ratio = if result.source_size > 0 {
        result.output_size as f64 / result.source_size as f64
    } else {
        0.0
    };

    let total_time_ms = result.parse_time_ms + result.transpile_time_ms;
    let throughput = if total_time_ms > 0 {
        (result.source_size as f64 / 1024.0) / (total_time_ms as f64 / 1000.0)
    } else {
        0.0
    };

    TranspileMetrics {
        compression_ratio,
        total_time_ms,
        throughput_kb_per_sec: throughput,
    }
}

/// Transpilation metrics
#[derive(Debug, Clone)]
pub struct TranspileMetrics {
    pub compression_ratio: f64,
    pub total_time_ms: u64,
    pub throughput_kb_per_sec: f64,
}

/// Determine auto-fix strategy from config
pub fn determine_autofix_strategy(config: &TranspileConfig) -> AutoFixStrategy {
    if config.suggest_fixes && !config.auto_fix {
        AutoFixStrategy::SuggestOnly
    } else if config.async_mode && config.auto_fix {
        AutoFixStrategy::Async
    } else if config.auto_fix {
        AutoFixStrategy::Synchronous
    } else {
        AutoFixStrategy::None
    }
}

/// Evaluate confidence level based on threshold
pub fn evaluate_confidence(confidence: f64, threshold: f64) -> ConfidenceLevel {
    if confidence.is_nan() || threshold.is_nan() {
        ConfidenceLevel::Unknown
    } else if confidence >= threshold {
        ConfidenceLevel::High
    } else {
        ConfidenceLevel::Low
    }
}

/// Build oracle configuration from transpile config
pub fn build_oracle_config(config: &TranspileConfig) -> OracleConfig {
    OracleConfig {
        enabled: config.oracle,
        threshold: config.fix_confidence,
        max_retries: config.max_retries,
        llm_fallback: config.llm_fallback,
    }
}

/// Compute output path from input path
pub fn compute_output_path(input: &Path, output: Option<&Path>) -> PathBuf {
    match output {
        Some(path) => path.to_path_buf(),
        None => {
            let mut path = input.to_path_buf();
            path.set_extension("rs");
            path
        }
    }
}

/// Compute all output paths from config
pub fn compute_output_paths(
    input: &Path,
    output: Option<&Path>,
    config: &TranspileConfig,
    has_dependencies: bool,
) -> OutputPaths {
    let main_output = compute_output_path(input, output);

    let autofix_output = if config.auto_fix && config.async_mode {
        let stem = main_output.file_stem().unwrap_or_default().to_string_lossy();
        let mut p = main_output.clone();
        p.set_file_name(format!("{}.autofix.rs", stem));
        Some(p)
    } else {
        None
    };

    let test_output = if config.gen_tests {
        Some(main_output.with_extension("test.rs"))
    } else {
        None
    };

    let cargo_toml = if has_dependencies {
        let mut p = main_output.clone();
        p.set_file_name("Cargo.toml");
        Some(p)
    } else {
        None
    };

    let source_map = if config.source_map {
        let mut p = main_output.clone();
        let name = p.file_name().unwrap_or_default().to_string_lossy();
        p.set_file_name(format!("{}.sourcemap.json", name));
        Some(p)
    } else {
        None
    };

    OutputPaths {
        main_output,
        autofix_output,
        test_output,
        cargo_toml,
        source_map,
    }
}

/// Extract package name from output path
pub fn extract_package_name(output_path: &Path) -> String {
    output_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("transpiled_package")
        .to_string()
}

/// Extract source file name from output path
pub fn extract_source_file_name(output_path: &Path) -> String {
    output_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("main.rs")
        .to_string()
}

/// Format initialization trace details
pub fn format_init_trace(config: &TranspileConfig) -> Vec<(String, String)> {
    vec![
        (
            "Verification".to_string(),
            if config.verify { "enabled" } else { "disabled" }.to_string(),
        ),
        (
            "Debug mode".to_string(),
            if config.debug { "enabled" } else { "disabled" }.to_string(),
        ),
        (
            "Source map".to_string(),
            if config.source_map {
                "enabled"
            } else {
                "disabled"
            }
            .to_string(),
        ),
        (
            "Audit trail".to_string(),
            if config.audit_trail {
                "enabled (HashChainCollector)"
            } else {
                "disabled"
            }
            .to_string(),
        ),
    ]
}

/// Format parse trace details
pub fn format_parse_trace(source_size: usize, parse_time_ms: u64) -> Vec<(String, String)> {
    vec![
        ("Input size".to_string(), format!("{} bytes", source_size)),
        ("Parse time".to_string(), format!("{:.2}ms", parse_time_ms)),
    ]
}

/// Format codegen trace details
pub fn format_codegen_trace(output_size: usize, dependencies_count: usize) -> Vec<(String, String)> {
    vec![
        (
            "Generated Rust code".to_string(),
            format!("{} bytes", output_size),
        ),
        (
            "Dependencies detected".to_string(),
            format!("{}", dependencies_count),
        ),
        ("Generation".to_string(), "complete".to_string()),
    ]
}

/// Format oracle trace details
pub fn format_oracle_trace(oracle_config: &OracleConfig, pattern_path: Option<&Path>) -> Vec<(String, String)> {
    let mut details = vec![
        (
            "Threshold".to_string(),
            format!("{:.0}%", oracle_config.threshold * 100.0),
        ),
        (
            "Max retries".to_string(),
            format!("{}", oracle_config.max_retries),
        ),
        (
            "LLM fallback".to_string(),
            if oracle_config.llm_fallback {
                "enabled"
            } else {
                "disabled"
            }
            .to_string(),
        ),
    ];

    if let Some(path) = pattern_path {
        details.insert(
            0,
            ("Pattern file".to_string(), path.display().to_string()),
        );
    }

    details
}

/// Format summary output for transpilation
pub fn format_summary(
    input_path: &Path,
    output_path: &Path,
    source_size: usize,
    output_size: usize,
    parse_time_ms: u64,
    total_time_ms: u64,
    dependencies_count: usize,
    verified: bool,
) -> Vec<String> {
    let throughput = if parse_time_ms > 0 {
        (source_size as f64 / 1024.0) / (parse_time_ms as f64 / 1000.0)
    } else {
        0.0
    };

    let mut lines = vec![
        format!("📄 Source: {} ({} bytes)", input_path.display(), source_size),
        format!("📝 Output: {} ({} bytes)", output_path.display(), output_size),
    ];

    if dependencies_count > 0 {
        let mut cargo_path = output_path.to_path_buf();
        cargo_path.set_file_name("Cargo.toml");
        lines.push(format!(
            "📦 Cargo.toml: {} ({} dependencies)",
            cargo_path.display(),
            dependencies_count
        ));
    }

    lines.push(format!("⏱️  Parse time: {:.2}ms", parse_time_ms));
    lines.push(format!("📊 Throughput: {:.1} KB/s", throughput));
    lines.push(format!("⏱️  Total time: {:.2}ms", total_time_ms));

    if verified {
        lines.push("✓ Properties Verified".to_string());
    }

    lines
}

/// Validate confidence threshold is in valid range
pub fn validate_confidence_threshold(threshold: f64) -> Result<f64, &'static str> {
    if threshold.is_nan() {
        Err("Confidence threshold cannot be NaN")
    } else if threshold < 0.0 {
        Err("Confidence threshold cannot be negative")
    } else if threshold > 1.0 {
        Err("Confidence threshold cannot exceed 1.0")
    } else {
        Ok(threshold)
    }
}

/// Validate max retries is reasonable
pub fn validate_max_retries(max_retries: usize) -> Result<usize, &'static str> {
    if max_retries == 0 {
        Err("Max retries must be at least 1")
    } else if max_retries > 100 {
        Err("Max retries cannot exceed 100")
    } else {
        Ok(max_retries)
    }
}

/// Check if config requires oracle
pub fn requires_oracle(config: &TranspileConfig) -> bool {
    config.oracle || config.auto_fix || config.suggest_fixes
}

/// Determine debug level from config
pub fn determine_debug_level(config: &TranspileConfig) -> DebugLevel {
    if config.debug {
        DebugLevel::Full
    } else if config.source_map {
        DebugLevel::Basic
    } else {
        DebugLevel::None
    }
}

/// Format explanation output
pub fn format_explanation() -> Vec<String> {
    vec![
        "Transformation Decisions:".to_string(),
        "  1. Python AST -> HIR: Converted Python constructs to type-safe HIR".to_string(),
        "  2. HIR -> Rust: Generated idiomatic Rust code with:".to_string(),
        "     - Type inference for local variables".to_string(),
        "     - Ownership and borrowing semantics".to_string(),
        "     - Memory safety guarantees".to_string(),
        "  3. Module mapping: Applied Python->Rust standard library mappings".to_string(),
    ]
}

/// Format ML oracle result message
pub fn format_ml_result(strategy: &str, confidence: f64, is_auto_fix: bool, threshold: f64) -> String {
    let confidence_pct = confidence * 100.0;
    let is_high = confidence >= threshold;

    if is_auto_fix && is_high {
        format!("🔧 Auto-fix applied with {:.2}% confidence", confidence_pct)
    } else if is_high {
        format!(
            "✓ High confidence ({:.2}%) - auto-fix would apply",
            confidence_pct
        )
    } else {
        format!(
            "⚠ Low confidence ({:.2}%) - manual review recommended [strategy: {}]",
            confidence_pct, strategy
        )
    }
}

/// Determine if result should be used based on confidence
pub fn should_use_result(confidence: f64, threshold: f64) -> bool {
    !confidence.is_nan() && !threshold.is_nan() && confidence >= threshold
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    // =====================================================
    // TranspileConfig Tests
    // =====================================================

    #[test]
    fn test_transpile_config_default() {
        let config = TranspileConfig::default();
        assert!(!config.verify);
        assert!(!config.debug);
        assert!(!config.trace);
        assert!(!config.async_mode);
        assert_eq!(config.fix_confidence, 0.0);
        assert_eq!(config.max_retries, 0);
    }

    #[test]
    fn test_transpile_config_all_enabled() {
        let config = TranspileConfig {
            verify: true,
            gen_tests: true,
            debug: true,
            source_map: true,
            trace: true,
            explain: true,
            audit_trail: true,
            auto_fix: true,
            async_mode: true,
            suggest_fixes: true,
            fix_confidence: 0.8,
            oracle: true,
            max_retries: 5,
            llm_fallback: true,
        };
        assert!(config.verify);
        assert!(config.async_mode);
        assert_eq!(config.fix_confidence, 0.8);
    }

    // =====================================================
    // AutoFixStrategy Tests
    // =====================================================

    #[test]
    fn test_autofix_strategy_none() {
        let config = TranspileConfig::default();
        assert_eq!(determine_autofix_strategy(&config), AutoFixStrategy::None);
    }

    #[test]
    fn test_autofix_strategy_synchronous() {
        let config = TranspileConfig {
            auto_fix: true,
            ..Default::default()
        };
        assert_eq!(determine_autofix_strategy(&config), AutoFixStrategy::Synchronous);
    }

    #[test]
    fn test_autofix_strategy_async() {
        let config = TranspileConfig {
            auto_fix: true,
            async_mode: true,
            ..Default::default()
        };
        assert_eq!(determine_autofix_strategy(&config), AutoFixStrategy::Async);
    }

    #[test]
    fn test_autofix_strategy_suggest_only() {
        let config = TranspileConfig {
            suggest_fixes: true,
            ..Default::default()
        };
        assert_eq!(determine_autofix_strategy(&config), AutoFixStrategy::SuggestOnly);
    }

    #[test]
    fn test_autofix_strategy_autofix_overrides_suggest() {
        let config = TranspileConfig {
            auto_fix: true,
            suggest_fixes: true,
            ..Default::default()
        };
        assert_eq!(determine_autofix_strategy(&config), AutoFixStrategy::Synchronous);
    }

    // =====================================================
    // ConfidenceLevel Tests
    // =====================================================

    #[test]
    fn test_evaluate_confidence_high() {
        assert_eq!(evaluate_confidence(0.9, 0.8), ConfidenceLevel::High);
        assert_eq!(evaluate_confidence(0.8, 0.8), ConfidenceLevel::High);
        assert_eq!(evaluate_confidence(1.0, 0.5), ConfidenceLevel::High);
    }

    #[test]
    fn test_evaluate_confidence_low() {
        assert_eq!(evaluate_confidence(0.5, 0.8), ConfidenceLevel::Low);
        assert_eq!(evaluate_confidence(0.0, 0.1), ConfidenceLevel::Low);
        assert_eq!(evaluate_confidence(0.79, 0.8), ConfidenceLevel::Low);
    }

    #[test]
    fn test_evaluate_confidence_unknown_nan() {
        assert_eq!(evaluate_confidence(f64::NAN, 0.8), ConfidenceLevel::Unknown);
        assert_eq!(evaluate_confidence(0.9, f64::NAN), ConfidenceLevel::Unknown);
        assert_eq!(evaluate_confidence(f64::NAN, f64::NAN), ConfidenceLevel::Unknown);
    }

    // =====================================================
    // OracleConfig Tests
    // =====================================================

    #[test]
    fn test_build_oracle_config() {
        let config = TranspileConfig {
            oracle: true,
            fix_confidence: 0.75,
            max_retries: 3,
            llm_fallback: true,
            ..Default::default()
        };
        let oracle_config = build_oracle_config(&config);
        assert!(oracle_config.enabled);
        assert_eq!(oracle_config.threshold, 0.75);
        assert_eq!(oracle_config.max_retries, 3);
        assert!(oracle_config.llm_fallback);
    }

    #[test]
    fn test_build_oracle_config_disabled() {
        let config = TranspileConfig::default();
        let oracle_config = build_oracle_config(&config);
        assert!(!oracle_config.enabled);
        assert_eq!(oracle_config.threshold, 0.0);
        assert!(!oracle_config.llm_fallback);
    }

    // =====================================================
    // Output Path Tests
    // =====================================================

    #[test]
    fn test_compute_output_path_with_explicit() {
        let input = Path::new("/src/test.py");
        let output = Path::new("/out/result.rs");
        let result = compute_output_path(input, Some(output));
        assert_eq!(result, PathBuf::from("/out/result.rs"));
    }

    #[test]
    fn test_compute_output_path_auto() {
        let input = Path::new("/src/test.py");
        let result = compute_output_path(input, None);
        assert_eq!(result, PathBuf::from("/src/test.rs"));
    }

    #[test]
    fn test_compute_output_path_no_extension() {
        let input = Path::new("/src/myfile");
        let result = compute_output_path(input, None);
        assert_eq!(result, PathBuf::from("/src/myfile.rs"));
    }

    #[test]
    fn test_compute_output_paths_minimal() {
        let input = Path::new("/src/test.py");
        let config = TranspileConfig::default();
        let paths = compute_output_paths(input, None, &config, false);
        assert_eq!(paths.main_output, PathBuf::from("/src/test.rs"));
        assert!(paths.autofix_output.is_none());
        assert!(paths.test_output.is_none());
        assert!(paths.cargo_toml.is_none());
        assert!(paths.source_map.is_none());
    }

    #[test]
    fn test_compute_output_paths_with_tests() {
        let input = Path::new("/src/test.py");
        let config = TranspileConfig {
            gen_tests: true,
            ..Default::default()
        };
        let paths = compute_output_paths(input, None, &config, false);
        assert_eq!(paths.test_output, Some(PathBuf::from("/src/test.test.rs")));
    }

    #[test]
    fn test_compute_output_paths_with_dependencies() {
        let input = Path::new("/src/test.py");
        let config = TranspileConfig::default();
        let paths = compute_output_paths(input, None, &config, true);
        assert_eq!(paths.cargo_toml, Some(PathBuf::from("/src/Cargo.toml")));
    }

    #[test]
    fn test_compute_output_paths_with_source_map() {
        let input = Path::new("/src/test.py");
        let config = TranspileConfig {
            source_map: true,
            ..Default::default()
        };
        let paths = compute_output_paths(input, None, &config, false);
        assert_eq!(paths.source_map, Some(PathBuf::from("/src/test.rs.sourcemap.json")));
    }

    #[test]
    fn test_compute_output_paths_with_async_autofix() {
        let input = Path::new("/src/test.py");
        let config = TranspileConfig {
            auto_fix: true,
            async_mode: true,
            ..Default::default()
        };
        let paths = compute_output_paths(input, None, &config, false);
        assert_eq!(paths.autofix_output, Some(PathBuf::from("/src/test.autofix.rs")));
    }

    #[test]
    fn test_compute_output_paths_all_options() {
        let input = Path::new("/src/example.py");
        let output = Path::new("/out/result.rs");
        let config = TranspileConfig {
            gen_tests: true,
            source_map: true,
            auto_fix: true,
            async_mode: true,
            ..Default::default()
        };
        let paths = compute_output_paths(input, Some(output), &config, true);
        assert_eq!(paths.main_output, PathBuf::from("/out/result.rs"));
        assert!(paths.autofix_output.is_some());
        assert!(paths.test_output.is_some());
        assert!(paths.cargo_toml.is_some());
        assert!(paths.source_map.is_some());
    }

    // =====================================================
    // Package/File Name Extraction Tests
    // =====================================================

    #[test]
    fn test_extract_package_name() {
        assert_eq!(extract_package_name(Path::new("/src/my_project.rs")), "my_project");
        assert_eq!(extract_package_name(Path::new("example.rs")), "example");
        assert_eq!(extract_package_name(Path::new("/a/b/c/test_module.rs")), "test_module");
    }

    #[test]
    fn test_extract_package_name_no_stem() {
        assert_eq!(extract_package_name(Path::new("/")), "transpiled_package");
        assert_eq!(extract_package_name(Path::new("")), "transpiled_package");
    }

    #[test]
    fn test_extract_source_file_name() {
        assert_eq!(extract_source_file_name(Path::new("/src/main.rs")), "main.rs");
        assert_eq!(extract_source_file_name(Path::new("lib.rs")), "lib.rs");
    }

    #[test]
    fn test_extract_source_file_name_no_name() {
        assert_eq!(extract_source_file_name(Path::new("/")), "main.rs");
    }

    // =====================================================
    // Trace Formatting Tests
    // =====================================================

    #[test]
    fn test_format_init_trace_default() {
        let config = TranspileConfig::default();
        let trace = format_init_trace(&config);
        assert_eq!(trace.len(), 4);
        assert_eq!(trace[0].1, "disabled");
        assert_eq!(trace[1].1, "disabled");
    }

    #[test]
    fn test_format_init_trace_enabled() {
        let config = TranspileConfig {
            verify: true,
            debug: true,
            source_map: true,
            audit_trail: true,
            ..Default::default()
        };
        let trace = format_init_trace(&config);
        assert_eq!(trace[0].1, "enabled");
        assert_eq!(trace[1].1, "enabled");
        assert_eq!(trace[2].1, "enabled");
        assert!(trace[3].1.contains("HashChainCollector"));
    }

    #[test]
    fn test_format_parse_trace() {
        let trace = format_parse_trace(1024, 15);
        assert_eq!(trace.len(), 2);
        assert_eq!(trace[0].0, "Input size");
        assert!(trace[0].1.contains("1024"));
        assert!(trace[1].1.contains("15"));
    }

    #[test]
    fn test_format_codegen_trace() {
        let trace = format_codegen_trace(2048, 5);
        assert_eq!(trace.len(), 3);
        assert!(trace[0].1.contains("2048"));
        assert!(trace[1].1.contains("5"));
        assert_eq!(trace[2].1, "complete");
    }

    #[test]
    fn test_format_oracle_trace_without_pattern() {
        let oracle_config = OracleConfig {
            enabled: true,
            threshold: 0.75,
            max_retries: 3,
            llm_fallback: false,
        };
        let trace = format_oracle_trace(&oracle_config, None);
        assert_eq!(trace.len(), 3);
        assert!(trace[0].1.contains("75%"));
        assert!(trace[1].1.contains("3"));
        assert_eq!(trace[2].1, "disabled");
    }

    #[test]
    fn test_format_oracle_trace_with_pattern() {
        let oracle_config = OracleConfig {
            enabled: true,
            threshold: 0.8,
            max_retries: 5,
            llm_fallback: true,
        };
        let pattern = Path::new("/patterns/oracle.json");
        let trace = format_oracle_trace(&oracle_config, Some(pattern));
        assert_eq!(trace.len(), 4);
        assert!(trace[0].1.contains("oracle.json"));
    }

    // =====================================================
    // Summary Formatting Tests
    // =====================================================

    #[test]
    fn test_format_summary_basic() {
        let summary = format_summary(
            Path::new("/src/test.py"),
            Path::new("/out/test.rs"),
            1000,
            800,
            10,
            100,
            0,
            false,
        );
        assert!(summary.len() >= 4);
        assert!(summary[0].contains("test.py"));
        assert!(summary[1].contains("test.rs"));
        assert!(summary.iter().any(|s| s.contains("Parse time")));
    }

    #[test]
    fn test_format_summary_with_dependencies() {
        let summary = format_summary(
            Path::new("input.py"),
            Path::new("output.rs"),
            500,
            400,
            5,
            50,
            3,
            false,
        );
        assert!(summary.iter().any(|s| s.contains("Cargo.toml")));
        assert!(summary.iter().any(|s| s.contains("3 dependencies")));
    }

    #[test]
    fn test_format_summary_verified() {
        let summary = format_summary(
            Path::new("in.py"),
            Path::new("out.rs"),
            100,
            80,
            2,
            20,
            0,
            true,
        );
        assert!(summary.iter().any(|s| s.contains("Verified")));
    }

    #[test]
    fn test_format_summary_zero_parse_time() {
        let summary = format_summary(
            Path::new("in.py"),
            Path::new("out.rs"),
            100,
            80,
            0,
            10,
            0,
            false,
        );
        assert!(summary.iter().any(|s| s.contains("0.0 KB/s")));
    }

    // =====================================================
    // Validation Tests
    // =====================================================

    #[test]
    fn test_validate_confidence_threshold_valid() {
        assert_eq!(validate_confidence_threshold(0.0), Ok(0.0));
        assert_eq!(validate_confidence_threshold(0.5), Ok(0.5));
        assert_eq!(validate_confidence_threshold(1.0), Ok(1.0));
    }

    #[test]
    fn test_validate_confidence_threshold_negative() {
        assert!(validate_confidence_threshold(-0.1).is_err());
    }

    #[test]
    fn test_validate_confidence_threshold_too_high() {
        assert!(validate_confidence_threshold(1.1).is_err());
    }

    #[test]
    fn test_validate_confidence_threshold_nan() {
        assert!(validate_confidence_threshold(f64::NAN).is_err());
    }

    #[test]
    fn test_validate_max_retries_valid() {
        assert_eq!(validate_max_retries(1), Ok(1));
        assert_eq!(validate_max_retries(50), Ok(50));
        assert_eq!(validate_max_retries(100), Ok(100));
    }

    #[test]
    fn test_validate_max_retries_zero() {
        assert!(validate_max_retries(0).is_err());
    }

    #[test]
    fn test_validate_max_retries_too_high() {
        assert!(validate_max_retries(101).is_err());
    }

    // =====================================================
    // Requires Oracle Tests
    // =====================================================

    #[test]
    fn test_requires_oracle_disabled() {
        let config = TranspileConfig::default();
        assert!(!requires_oracle(&config));
    }

    #[test]
    fn test_requires_oracle_oracle_flag() {
        let config = TranspileConfig {
            oracle: true,
            ..Default::default()
        };
        assert!(requires_oracle(&config));
    }

    #[test]
    fn test_requires_oracle_auto_fix() {
        let config = TranspileConfig {
            auto_fix: true,
            ..Default::default()
        };
        assert!(requires_oracle(&config));
    }

    #[test]
    fn test_requires_oracle_suggest_fixes() {
        let config = TranspileConfig {
            suggest_fixes: true,
            ..Default::default()
        };
        assert!(requires_oracle(&config));
    }

    // =====================================================
    // Debug Level Tests
    // =====================================================

    #[test]
    fn test_determine_debug_level_none() {
        let config = TranspileConfig::default();
        assert_eq!(determine_debug_level(&config), DebugLevel::None);
    }

    #[test]
    fn test_determine_debug_level_basic() {
        let config = TranspileConfig {
            source_map: true,
            ..Default::default()
        };
        assert_eq!(determine_debug_level(&config), DebugLevel::Basic);
    }

    #[test]
    fn test_determine_debug_level_full() {
        let config = TranspileConfig {
            debug: true,
            ..Default::default()
        };
        assert_eq!(determine_debug_level(&config), DebugLevel::Full);
    }

    #[test]
    fn test_determine_debug_level_debug_overrides_source_map() {
        let config = TranspileConfig {
            debug: true,
            source_map: true,
            ..Default::default()
        };
        assert_eq!(determine_debug_level(&config), DebugLevel::Full);
    }

    // =====================================================
    // Explanation Tests
    // =====================================================

    #[test]
    fn test_format_explanation() {
        let exp = format_explanation();
        assert!(exp.len() >= 5);
        assert!(exp[0].contains("Transformation"));
        assert!(exp.iter().any(|s| s.contains("HIR")));
        assert!(exp.iter().any(|s| s.contains("Rust")));
    }

    // =====================================================
    // ML Result Formatting Tests
    // =====================================================

    #[test]
    fn test_format_ml_result_autofix_high() {
        let msg = format_ml_result("ast", 0.9, true, 0.8);
        assert!(msg.contains("Auto-fix applied"));
        assert!(msg.contains("90.00%"));
    }

    #[test]
    fn test_format_ml_result_high_no_autofix() {
        let msg = format_ml_result("ml", 0.85, false, 0.8);
        assert!(msg.contains("High confidence"));
        assert!(msg.contains("would apply"));
    }

    #[test]
    fn test_format_ml_result_low() {
        let msg = format_ml_result("fallback", 0.5, false, 0.8);
        assert!(msg.contains("Low confidence"));
        assert!(msg.contains("manual review"));
        assert!(msg.contains("fallback"));
    }

    #[test]
    fn test_format_ml_result_boundary() {
        let msg = format_ml_result("exact", 0.8, false, 0.8);
        assert!(msg.contains("High confidence"));
    }

    // =====================================================
    // Should Use Result Tests
    // =====================================================

    #[test]
    fn test_should_use_result_high() {
        assert!(should_use_result(0.9, 0.8));
        assert!(should_use_result(0.8, 0.8));
        assert!(should_use_result(1.0, 0.0));
    }

    #[test]
    fn test_should_use_result_low() {
        assert!(!should_use_result(0.5, 0.8));
        assert!(!should_use_result(0.0, 0.1));
    }

    #[test]
    fn test_should_use_result_nan() {
        assert!(!should_use_result(f64::NAN, 0.8));
        assert!(!should_use_result(0.9, f64::NAN));
    }

    // =====================================================
    // Pipeline Tests
    // =====================================================

    #[test]
    fn test_create_pipeline_basic() {
        let config = TranspileConfig::default();
        let _pipeline = create_pipeline(&config);
    }

    #[test]
    fn test_create_pipeline_with_verify() {
        let config = TranspileConfig {
            verify: true,
            ..Default::default()
        };
        let _pipeline = create_pipeline(&config);
    }

    #[test]
    fn test_create_pipeline_with_debug() {
        let config = TranspileConfig {
            debug: true,
            ..Default::default()
        };
        let _pipeline = create_pipeline(&config);
    }

    #[test]
    fn test_create_pipeline_with_source_map() {
        let config = TranspileConfig {
            source_map: true,
            ..Default::default()
        };
        let _pipeline = create_pipeline(&config);
    }

    #[test]
    fn test_create_pipeline_all_options() {
        let config = TranspileConfig {
            verify: true,
            debug: true,
            source_map: true,
            ..Default::default()
        };
        let _pipeline = create_pipeline(&config);
    }

    // =====================================================
    // Transpilation Tests
    // =====================================================

    #[test]
    fn test_transpile_simple() {
        let config = TranspileConfig::default();
        let source = "def add(a: int, b: int) -> int:\n    return a + b\n";
        let result = transpile_source(source, &config).unwrap();
        assert!(result.rust_code.contains("fn add"));
        assert_eq!(result.source_size, source.len());
        assert!(result.output_size > 0);
    }

    #[test]
    fn test_transpile_with_verify() {
        let config = TranspileConfig {
            verify: true,
            ..Default::default()
        };
        let source = "x: int = 42\n";
        let result = transpile_source(source, &config).unwrap();
        assert!(!result.rust_code.is_empty());
    }

    #[test]
    fn test_transpile_empty_source() {
        let config = TranspileConfig::default();
        let result = transpile_source("", &config);
        // Empty source may or may not error, but should not panic
        let _ = result;
    }

    // =====================================================
    // Trace Phase Formatting Tests
    // =====================================================

    #[test]
    fn test_format_trace_phase() {
        let trace = format_trace_phase(1, "Parsing", &[
            ("Input size", "1024 bytes"),
            ("Parser", "RustPython"),
        ]);
        assert!(trace.contains("Phase 1: Parsing"));
        assert!(trace.contains("Input size: 1024 bytes"));
        assert!(trace.contains("Parser: RustPython"));
    }

    #[test]
    fn test_format_trace_phase_empty_details() {
        let trace = format_trace_phase(2, "Generation", &[]);
        assert!(trace.contains("Phase 2: Generation"));
        assert!(!trace.contains("  - "));
    }

    // =====================================================
    // Metrics Tests
    // =====================================================

    #[test]
    fn test_calculate_metrics() {
        let result = TranspileResult {
            rust_code: "fn test() {}".to_string(),
            source_size: 1000,
            output_size: 500,
            parse_time_ms: 10,
            transpile_time_ms: 90,
            verification_passed: None,
            fixes_applied: 0,
        };
        let metrics = calculate_metrics(&result);
        assert!((metrics.compression_ratio - 0.5).abs() < 0.01);
        assert_eq!(metrics.total_time_ms, 100);
        assert!(metrics.throughput_kb_per_sec > 0.0);
    }

    #[test]
    fn test_calculate_metrics_empty_source() {
        let result = TranspileResult {
            rust_code: String::new(),
            source_size: 0,
            output_size: 0,
            parse_time_ms: 0,
            transpile_time_ms: 0,
            verification_passed: None,
            fixes_applied: 0,
        };
        let metrics = calculate_metrics(&result);
        assert_eq!(metrics.compression_ratio, 0.0);
        assert_eq!(metrics.throughput_kb_per_sec, 0.0);
    }

    #[test]
    fn test_calculate_metrics_expansion() {
        let result = TranspileResult {
            rust_code: "// expanded".to_string(),
            source_size: 100,
            output_size: 300,
            parse_time_ms: 5,
            transpile_time_ms: 5,
            verification_passed: Some(true),
            fixes_applied: 0,
        };
        let metrics = calculate_metrics(&result);
        assert!((metrics.compression_ratio - 3.0).abs() < 0.01);
    }

    // =====================================================
    // Result Fields Tests
    // =====================================================

    #[test]
    fn test_transpile_result_fields() {
        let result = TranspileResult {
            rust_code: "// test".to_string(),
            source_size: 100,
            output_size: 7,
            parse_time_ms: 5,
            transpile_time_ms: 15,
            verification_passed: Some(true),
            fixes_applied: 2,
        };
        assert_eq!(result.source_size, 100);
        assert_eq!(result.fixes_applied, 2);
        assert_eq!(result.verification_passed, Some(true));
    }

    #[test]
    fn test_transpile_result_verification_none() {
        let result = TranspileResult {
            rust_code: String::new(),
            source_size: 0,
            output_size: 0,
            parse_time_ms: 0,
            transpile_time_ms: 0,
            verification_passed: None,
            fixes_applied: 0,
        };
        assert!(result.verification_passed.is_none());
    }

    #[test]
    fn test_transpile_result_verification_false() {
        let result = TranspileResult {
            rust_code: "bad".to_string(),
            source_size: 10,
            output_size: 3,
            parse_time_ms: 1,
            transpile_time_ms: 1,
            verification_passed: Some(false),
            fixes_applied: 0,
        };
        assert_eq!(result.verification_passed, Some(false));
    }

    // =====================================================
    // AutoFixStrategy Enum Tests
    // =====================================================

    #[test]
    fn test_autofix_strategy_eq() {
        assert_eq!(AutoFixStrategy::None, AutoFixStrategy::None);
        assert_ne!(AutoFixStrategy::None, AutoFixStrategy::Async);
    }

    #[test]
    fn test_autofix_strategy_clone() {
        let strategy = AutoFixStrategy::Synchronous;
        let cloned = strategy;
        assert_eq!(strategy, cloned);
    }

    // =====================================================
    // ConfidenceLevel Enum Tests
    // =====================================================

    #[test]
    fn test_confidence_level_eq() {
        assert_eq!(ConfidenceLevel::High, ConfidenceLevel::High);
        assert_ne!(ConfidenceLevel::High, ConfidenceLevel::Low);
    }

    // =====================================================
    // OutputPaths Clone Tests
    // =====================================================

    #[test]
    fn test_output_paths_clone() {
        let paths = OutputPaths {
            main_output: PathBuf::from("/test.rs"),
            autofix_output: Some(PathBuf::from("/test.autofix.rs")),
            test_output: None,
            cargo_toml: None,
            source_map: None,
        };
        let cloned = paths.clone();
        assert_eq!(paths.main_output, cloned.main_output);
        assert_eq!(paths.autofix_output, cloned.autofix_output);
    }
}
