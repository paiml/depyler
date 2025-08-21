//! Ruchy interpreter integration module
//!
//! This module provides integration with the Ruchy v0.9.1+ interpreter,
//! leveraging the significant performance and reliability improvements.

use anyhow::{Result, anyhow};
use std::collections::HashMap;

#[cfg(feature = "interpreter")]
use ruchy::{compile, run_repl};

use crate::RuchyConfig;

/// Wrapper around the Ruchy interpreter with enhanced capabilities
pub struct RuchyInterpreter {
    /// Configuration for interpreter behavior
    config: RuchyConfig,
    
    /// Runtime context for variable bindings
    context: HashMap<String, String>,
    
    /// Whether MCP integration is enabled
    #[cfg(feature = "interpreter")]
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
    pub fn execute(&self, code: &str) -> Result<String> {
        Err(anyhow!("Interpreter feature not enabled. Rebuild with --features interpreter"))
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
        Err(anyhow!("REPL not available. Rebuild with --features interpreter"))
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
    fn test_context_management() {
        let mut interpreter = RuchyInterpreter::new();
        
        interpreter.set_context("x".to_string(), "42".to_string());
        assert_eq!(interpreter.get_context("x"), Some(&"42".to_string()));
        
        interpreter.clear_context();
        assert!(interpreter.context.is_empty());
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
}