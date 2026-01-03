//! Transpilation shim - pure logic separated from I/O
//!
//! This module contains the testable core logic of transpilation,
//! extracted from the CLI command handlers.

use anyhow::Result;
use depyler_core::{debug::DebugConfig, debug::DebugLevel, DepylerPipeline};

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
    pub suggest_fixes: bool,
    pub fix_confidence: f64,
    pub oracle: bool,
    pub max_retries: usize,
    pub llm_fallback: bool,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpile_config_default() {
        let config = TranspileConfig::default();
        assert!(!config.verify);
        assert!(!config.debug);
        assert!(!config.trace);
        assert_eq!(config.fix_confidence, 0.0);
    }

    #[test]
    fn test_create_pipeline_basic() {
        let config = TranspileConfig::default();
        let _pipeline = create_pipeline(&config);
        // Pipeline created without panic
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
        assert!(result.rust_code.len() > 0);
    }

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
    }

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
}
