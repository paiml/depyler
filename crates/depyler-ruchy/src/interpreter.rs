//! Ruchy interpreter integration module
//!
//! This module provides integration with the Ruchy v1.5.0+ interpreter,
//! leveraging the SELF-HOSTING capability and significant performance improvements.

use anyhow::{anyhow, Result};
use std::collections::HashMap;

#[cfg(feature = "interpreter")]
use ruchy::{compile, run_repl};

use crate::RuchyConfig;

/// Wrapper around the Ruchy interpreter with enhanced capabilities
pub struct RuchyInterpreter {
    /// Configuration for interpreter behavior
    #[allow(dead_code)]
    config: RuchyConfig,

    /// Runtime context for variable bindings
    context: HashMap<String, String>,

    /// Whether MCP integration is enabled
    #[cfg(feature = "interpreter")]
    #[allow(dead_code)]
    mcp_enabled: bool,
}

impl RuchyInterpreter {
    /// Creates a new Ruchy interpreter with default settings
    pub fn new() -> Self {
        Self {
            config: RuchyConfig::default(),
            context: HashMap::new(),
            #[cfg(feature = "interpreter")]
            mcp_enabled: false,
        }
    }

    /// Creates a Ruchy interpreter with custom configuration
    pub fn with_config(config: &RuchyConfig) -> Self {
        Self {
            config: config.clone(),
            context: HashMap::new(),
            #[cfg(feature = "interpreter")]
            mcp_enabled: config.enable_mcp,
        }
    }

    /// Executes Ruchy code and returns the result as a string
    #[cfg(feature = "interpreter")]
    pub fn execute(&self, code: &str) -> Result<String> {
        // Validate syntax first
        if !ruchy::is_valid_syntax(code) {
            if let Some(error) = ruchy::get_parse_error(code) {
                return Err(anyhow!("Parse error: {}", error));
            }
            return Err(anyhow!("Invalid Ruchy syntax"));
        }

        // Compile and execute
        match compile(code) {
            Ok(result) => Ok(result),
            Err(e) => Err(anyhow!("Execution error: {}", e)),
        }
    }

    /// Fallback execution for builds without interpreter feature
    #[cfg(not(feature = "interpreter"))]
    pub fn execute(&self, _code: &str) -> Result<String> {
        Err(anyhow!(
            "Interpreter feature not enabled. Rebuild with --features interpreter"
        ))
    }

    /// Validates Ruchy syntax
    #[cfg(feature = "interpreter")]
    pub fn validate_syntax(&self, code: &str) -> bool {
        ruchy::is_valid_syntax(code)
    }

    /// Fallback validation for builds without interpreter feature
    #[cfg(not(feature = "interpreter"))]
    pub fn validate_syntax(&self, _code: &str) -> bool {
        false
    }

    /// Sets a context variable for code execution
    pub fn set_context(&mut self, key: String, value: String) {
        self.context.insert(key, value);
    }

    /// Gets a context variable
    pub fn get_context(&self, key: &str) -> Option<&String> {
        self.context.get(key)
    }

    /// Clears all context variables
    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    /// Starts an interactive REPL session
    #[cfg(feature = "interpreter")]
    pub fn start_repl(&self) -> Result<()> {
        run_repl().map_err(|e| anyhow!("REPL error: {}", e))
    }

    /// Fallback REPL for builds without interpreter feature
    #[cfg(not(feature = "interpreter"))]
    pub fn start_repl(&self) -> Result<()> {
        Err(anyhow!(
            "REPL not available. Rebuild with --features interpreter"
        ))
    }

    /// Benchmarks the interpreter performance
    #[cfg(feature = "interpreter")]
    pub fn benchmark(&self, code: &str, iterations: usize) -> Result<BenchmarkResults> {
        use std::time::Instant;

        let start = Instant::now();

        for _ in 0..iterations {
            self.execute(code)?;
        }

        let duration = start.elapsed();

        Ok(BenchmarkResults {
            iterations,
            total_time_ms: duration.as_millis(),
            avg_time_ms: duration.as_millis() as f64 / iterations as f64,
            throughput_ops_per_sec: (iterations as f64 / duration.as_secs_f64()) as u64,
        })
    }
}

impl Default for RuchyInterpreter {
    fn default() -> Self {
        Self::new()
    }
}

/// Results from interpreter benchmarking
#[derive(Debug, Clone)]
pub struct BenchmarkResults {
    /// Number of iterations performed
    pub iterations: usize,

    /// Total execution time in milliseconds
    pub total_time_ms: u128,

    /// Average execution time per iteration in milliseconds
    pub avg_time_ms: f64,

    /// Throughput in operations per second
    pub throughput_ops_per_sec: u64,
}

impl std::fmt::Display for BenchmarkResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Benchmark Results:\n  Iterations: {}\n  Total time: {}ms\n  Avg time: {:.2}ms\n  Throughput: {} ops/sec",
            self.iterations,
            self.total_time_ms,
            self.avg_time_ms,
            self.throughput_ops_per_sec
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpreter_creation() {
        let interpreter = RuchyInterpreter::new();
        assert!(interpreter.context.is_empty());
    }

    #[test]
    fn test_interpreter_default() {
        let interpreter = RuchyInterpreter::default();
        assert!(interpreter.context.is_empty());
    }

    #[test]
    fn test_interpreter_with_config() {
        let config = RuchyConfig::default();
        let interpreter = RuchyInterpreter::with_config(&config);
        assert!(interpreter.context.is_empty());
    }

    #[test]
    #[cfg(feature = "interpreter")]
    fn test_interpreter_with_config_mcp_enabled() {
        let mut config = RuchyConfig::default();
        config.enable_mcp = true;
        let interpreter = RuchyInterpreter::with_config(&config);
        assert!(interpreter.context.is_empty());
    }

    #[test]
    fn test_context_management() {
        let mut interpreter = RuchyInterpreter::new();

        interpreter.set_context("x".to_string(), "42".to_string());
        assert_eq!(interpreter.get_context("x"), Some(&"42".to_string()));

        interpreter.clear_context();
        assert!(interpreter.context.is_empty());
    }

    #[test]
    fn test_context_get_nonexistent() {
        let interpreter = RuchyInterpreter::new();
        assert!(interpreter.get_context("nonexistent").is_none());
    }

    #[test]
    fn test_context_overwrite() {
        let mut interpreter = RuchyInterpreter::new();
        interpreter.set_context("key".to_string(), "old".to_string());
        interpreter.set_context("key".to_string(), "new".to_string());
        assert_eq!(interpreter.get_context("key"), Some(&"new".to_string()));
    }

    #[test]
    fn test_context_multiple_keys() {
        let mut interpreter = RuchyInterpreter::new();
        interpreter.set_context("a".to_string(), "1".to_string());
        interpreter.set_context("b".to_string(), "2".to_string());
        interpreter.set_context("c".to_string(), "3".to_string());
        assert_eq!(interpreter.get_context("a"), Some(&"1".to_string()));
        assert_eq!(interpreter.get_context("b"), Some(&"2".to_string()));
        assert_eq!(interpreter.get_context("c"), Some(&"3".to_string()));
    }

    #[cfg(not(feature = "interpreter"))]
    #[test]
    fn test_execute_without_feature() {
        let interpreter = RuchyInterpreter::new();
        let result = interpreter.execute("1 + 1");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Interpreter feature not enabled"));
    }

    #[cfg(not(feature = "interpreter"))]
    #[test]
    fn test_validate_syntax_without_feature() {
        let interpreter = RuchyInterpreter::new();
        assert!(!interpreter.validate_syntax("valid code"));
    }

    #[cfg(not(feature = "interpreter"))]
    #[test]
    fn test_start_repl_without_feature() {
        let interpreter = RuchyInterpreter::new();
        let result = interpreter.start_repl();
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("REPL not available"));
    }

    #[cfg(feature = "interpreter")]
    #[test]
    fn test_syntax_validation() {
        let interpreter = RuchyInterpreter::new();

        assert!(interpreter.validate_syntax("print(\"Hello\")"));
        assert!(!interpreter.validate_syntax("invalid syntax ("));
    }

    #[cfg(feature = "interpreter")]
    #[test]
    fn test_basic_execution() {
        let interpreter = RuchyInterpreter::new();

        // Test basic arithmetic
        let result = interpreter.execute("2 + 3");
        assert!(result.is_ok());

        // Test invalid syntax
        let result = interpreter.execute("invalid syntax (");
        assert!(result.is_err());
    }

    #[cfg(feature = "interpreter")]
    #[test]
    fn test_performance_benchmark() {
        let interpreter = RuchyInterpreter::new();
        let simple_code = "1 + 1";

        let results = interpreter.benchmark(simple_code, 100);
        assert!(results.is_ok());

        let bench = results.unwrap();
        assert_eq!(bench.iterations, 100);
        assert!(bench.throughput_ops_per_sec > 0);
    }

    #[test]
    fn test_benchmark_results_display() {
        let results = BenchmarkResults {
            iterations: 100,
            total_time_ms: 500,
            avg_time_ms: 5.0,
            throughput_ops_per_sec: 200,
        };
        let display = format!("{}", results);
        assert!(display.contains("Iterations: 100"));
        assert!(display.contains("Total time: 500ms"));
        assert!(display.contains("Avg time: 5.00ms"));
        assert!(display.contains("Throughput: 200 ops/sec"));
    }

    #[test]
    fn test_benchmark_results_clone() {
        let results = BenchmarkResults {
            iterations: 50,
            total_time_ms: 250,
            avg_time_ms: 5.0,
            throughput_ops_per_sec: 200,
        };
        let cloned = results.clone();
        assert_eq!(results.iterations, cloned.iterations);
        assert_eq!(results.total_time_ms, cloned.total_time_ms);
    }

    #[test]
    fn test_benchmark_results_debug() {
        let results = BenchmarkResults {
            iterations: 10,
            total_time_ms: 100,
            avg_time_ms: 10.0,
            throughput_ops_per_sec: 100,
        };
        let debug = format!("{:?}", results);
        assert!(debug.contains("BenchmarkResults"));
    }
}
