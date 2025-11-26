//! Hybrid transpilation with ML fallback
//!
//! Strategy:
//! 1. AST-based transpilation (fast, deterministic) - 90%+ cases
//! 2. Small fine-tuned model fallback (Qwen2.5-Coder-1.5B) - edge cases
//! 3. API fallback (Claude/GPT) - complex cases
//!
//! # Example
//!
//! ```ignore
//! use depyler_oracle::hybrid::{HybridTranspiler, TranspileResult};
//!
//! let transpiler = HybridTranspiler::new();
//! let result = transpiler.transpile("def add(a: int, b: int) -> int: return a + b")?;
//! println!("Rust: {}", result.rust_code);
//! ```

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Transpilation strategy used
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Strategy {
    /// AST-based rule transpilation
    Ast,
    /// Local fine-tuned model
    LocalModel,
    /// Remote API call
    Api,
}

/// Result of hybrid transpilation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranspileResult {
    /// Generated Rust code
    pub rust_code: String,
    /// Strategy that succeeded
    pub strategy: Strategy,
    /// Confidence score (0.0-1.0)
    pub confidence: f32,
    /// Latency in milliseconds
    pub latency_ms: u64,
    /// Warnings or notes
    pub warnings: Vec<String>,
}

/// Configuration for hybrid transpiler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridConfig {
    /// Enable local model fallback
    pub enable_local_model: bool,
    /// Enable API fallback
    pub enable_api: bool,
    /// API endpoint (if enabled)
    pub api_endpoint: Option<String>,
    /// API timeout
    pub api_timeout: Duration,
    /// Minimum confidence to accept AST result
    pub ast_confidence_threshold: f32,
    /// Maximum retries for API
    pub max_api_retries: u32,
}

impl Default for HybridConfig {
    fn default() -> Self {
        Self {
            enable_local_model: true,
            enable_api: false, // Disabled by default
            api_endpoint: None,
            api_timeout: Duration::from_secs(30),
            ast_confidence_threshold: 0.8,
            max_api_retries: 2,
        }
    }
}

/// Patterns that AST transpilation handles well
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternComplexity {
    /// Simple: arithmetic, basic types, control flow
    Simple,
    /// Medium: classes, generics, iterators
    Medium,
    /// Complex: metaclasses, decorators, dynamic typing
    Complex,
    /// Unsupported: exec, eval, runtime introspection
    Unsupported,
}

/// Hybrid transpiler with multi-strategy fallback
pub struct HybridTranspiler {
    config: HybridConfig,
    /// Pattern classifier for routing
    pattern_stats: PatternStats,
}

/// Statistics for pattern-based routing
#[derive(Debug, Default)]
struct PatternStats {
    ast_success: u64,
    ast_failure: u64,
    model_success: u64,
    model_failure: u64,
    api_success: u64,
    api_failure: u64,
}

impl HybridTranspiler {
    /// Create with default config
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(HybridConfig::default())
    }

    /// Create with custom config
    #[must_use]
    pub fn with_config(config: HybridConfig) -> Self {
        Self {
            config,
            pattern_stats: PatternStats::default(),
        }
    }

    /// Analyze Python code complexity
    #[must_use]
    pub fn analyze_complexity(&self, python_code: &str) -> PatternComplexity {
        // Quick heuristics for routing
        let code = python_code.to_lowercase();

        // Unsupported patterns
        if code.contains("exec(") || code.contains("eval(") || code.contains("__import__") {
            return PatternComplexity::Unsupported;
        }

        // Complex patterns (check before medium since metaclass contains "class")
        if code.contains("metaclass")
            || code.contains("__new__")
            || code.contains("__getattr__")
            || code.contains("globals(")
            || code.contains("locals(")
            || (code.contains("class ") && code.contains("(type)"))
        {
            return PatternComplexity::Complex;
        }

        // Medium patterns
        if code.contains("class ")
            || code.contains("@")
            || code.contains("yield")
            || code.contains("async ")
            || code.contains("lambda")
            || code.contains("type(")
        {
            return PatternComplexity::Medium;
        }

        PatternComplexity::Simple
    }

    /// Transpile Python to Rust using hybrid strategy
    pub fn transpile(&mut self, python_code: &str) -> Result<TranspileResult, TranspileError> {
        let start = std::time::Instant::now();
        let complexity = self.analyze_complexity(python_code);

        // Route based on complexity
        match complexity {
            PatternComplexity::Simple | PatternComplexity::Medium => {
                // Try AST first
                match self.try_ast_transpile(python_code) {
                    Ok(result) if result.confidence >= self.config.ast_confidence_threshold => {
                        self.pattern_stats.ast_success += 1;
                        return Ok(TranspileResult {
                            latency_ms: start.elapsed().as_millis() as u64,
                            ..result
                        });
                    }
                    Ok(_) | Err(_) => {
                        self.pattern_stats.ast_failure += 1;
                        // Fall through to model
                    }
                }
            }
            PatternComplexity::Complex => {
                // Skip AST, go straight to model
            }
            PatternComplexity::Unsupported => {
                return Err(TranspileError::UnsupportedPattern(
                    "Dynamic Python features not supported".to_string(),
                ));
            }
        }

        // Try local model if enabled
        if self.config.enable_local_model {
            match self.try_local_model(python_code) {
                Ok(result) => {
                    self.pattern_stats.model_success += 1;
                    return Ok(TranspileResult {
                        latency_ms: start.elapsed().as_millis() as u64,
                        ..result
                    });
                }
                Err(_) => {
                    self.pattern_stats.model_failure += 1;
                }
            }
        }

        // Try API if enabled
        if self.config.enable_api {
            match self.try_api_transpile(python_code) {
                Ok(result) => {
                    self.pattern_stats.api_success += 1;
                    return Ok(TranspileResult {
                        latency_ms: start.elapsed().as_millis() as u64,
                        ..result
                    });
                }
                Err(_) => {
                    self.pattern_stats.api_failure += 1;
                }
            }
        }

        Err(TranspileError::AllStrategiesFailed)
    }

    /// Try AST-based transpilation
    fn try_ast_transpile(&self, python_code: &str) -> Result<TranspileResult, TranspileError> {
        // This would integrate with depyler-core's existing transpiler
        // For now, stub that returns success for simple patterns

        let complexity = self.analyze_complexity(python_code);
        let confidence = match complexity {
            PatternComplexity::Simple => 0.95,
            PatternComplexity::Medium => 0.75,
            PatternComplexity::Complex => 0.4,
            PatternComplexity::Unsupported => 0.0,
        };

        // TODO: Call actual depyler-core transpiler
        // depyler_core::transpile(python_code)

        Ok(TranspileResult {
            rust_code: format!("// AST-transpiled from Python\n// TODO: integrate depyler-core\n{}",
                stub_transpile(python_code)),
            strategy: Strategy::Ast,
            confidence,
            latency_ms: 0,
            warnings: vec![],
        })
    }

    /// Try local fine-tuned model
    fn try_local_model(&self, python_code: &str) -> Result<TranspileResult, TranspileError> {
        // TODO: Integrate with local Qwen2.5-Coder-1.5B via llama.cpp or similar
        // For now, return error to fall through to API

        // Would use something like:
        // let model = LocalModel::load("qwen2.5-coder-1.5b-q4_0.gguf")?;
        // let rust_code = model.generate(format!("Translate Python to Rust:\n{}", python_code))?;

        Err(TranspileError::ModelNotLoaded)
    }

    /// Try API-based transpilation
    fn try_api_transpile(&self, python_code: &str) -> Result<TranspileResult, TranspileError> {
        let endpoint = self.config.api_endpoint.as_ref()
            .ok_or(TranspileError::ApiNotConfigured)?;

        // TODO: Make actual API call
        // For now, stub
        let _ = endpoint;
        let _ = python_code;

        Err(TranspileError::ApiNotConfigured)
    }

    /// Get transpilation statistics
    #[must_use]
    pub fn stats(&self) -> TranspileStats {
        let total = self.pattern_stats.ast_success
            + self.pattern_stats.ast_failure
            + self.pattern_stats.model_success
            + self.pattern_stats.model_failure
            + self.pattern_stats.api_success
            + self.pattern_stats.api_failure;

        TranspileStats {
            total_attempts: total,
            ast_success_rate: if self.pattern_stats.ast_success + self.pattern_stats.ast_failure > 0 {
                self.pattern_stats.ast_success as f32
                    / (self.pattern_stats.ast_success + self.pattern_stats.ast_failure) as f32
            } else {
                0.0
            },
            model_success_rate: if self.pattern_stats.model_success + self.pattern_stats.model_failure > 0 {
                self.pattern_stats.model_success as f32
                    / (self.pattern_stats.model_success + self.pattern_stats.model_failure) as f32
            } else {
                0.0
            },
            api_success_rate: if self.pattern_stats.api_success + self.pattern_stats.api_failure > 0 {
                self.pattern_stats.api_success as f32
                    / (self.pattern_stats.api_success + self.pattern_stats.api_failure) as f32
            } else {
                0.0
            },
        }
    }
}

impl Default for HybridTranspiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Transpilation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranspileStats {
    pub total_attempts: u64,
    pub ast_success_rate: f32,
    pub model_success_rate: f32,
    pub api_success_rate: f32,
}

/// Transpilation errors
#[derive(Debug, thiserror::Error)]
pub enum TranspileError {
    #[error("Unsupported pattern: {0}")]
    UnsupportedPattern(String),

    #[error("AST transpilation failed: {0}")]
    AstFailed(String),

    #[error("Local model not loaded")]
    ModelNotLoaded,

    #[error("Model inference failed: {0}")]
    ModelFailed(String),

    #[error("API not configured")]
    ApiNotConfigured,

    #[error("API call failed: {0}")]
    ApiFailed(String),

    #[error("All strategies failed")]
    AllStrategiesFailed,
}

/// Simple stub transpilation for testing
fn stub_transpile(python: &str) -> String {
    // Very basic pattern matching for demo
    let mut rust = String::new();

    for line in python.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("def ") {
            // def foo(a: int, b: int) -> int:
            rust.push_str(&trimmed
                .replace("def ", "fn ")
                .replace(":", " {")
                .replace("int", "i32")
                .replace("str", "&str")
                .replace("float", "f64")
                .replace("bool", "bool")
                .replace(" -> ", " -> "));
            rust.push('\n');
        } else if trimmed.starts_with("return ") {
            rust.push_str(&format!("    {}\n}}\n", trimmed.replace("return", "return")));
        } else if !trimmed.is_empty() {
            rust.push_str(&format!("    // {}\n", trimmed));
        }
    }

    rust
}

/// Training data collector for fine-tuning
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TrainingDataCollector {
    /// Successful Python→Rust pairs
    pairs: Vec<TranslationPair>,
}

/// A Python→Rust translation pair for training
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationPair {
    pub python: String,
    pub rust: String,
    pub verified: bool,
    pub source: String,
}

impl TrainingDataCollector {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a verified translation pair
    pub fn add_pair(&mut self, python: String, rust: String, source: &str) {
        self.pairs.push(TranslationPair {
            python,
            rust,
            verified: true,
            source: source.to_string(),
        });
    }

    /// Export to JSONL for fine-tuning
    pub fn export_jsonl(&self) -> String {
        self.pairs
            .iter()
            .filter(|p| p.verified)
            .map(|p| {
                serde_json::json!({
                    "messages": [
                        {"role": "user", "content": format!("Translate to Rust:\n```python\n{}\n```", p.python)},
                        {"role": "assistant", "content": format!("```rust\n{}\n```", p.rust)}
                    ]
                })
                .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Load existing pairs from test corpus
    pub fn load_from_test_corpus(&mut self, corpus_path: &std::path::Path) -> std::io::Result<usize> {
        // TODO: Parse depyler test corpus for verified pairs
        let _ = corpus_path;
        Ok(0)
    }

    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complexity_simple() {
        let t = HybridTranspiler::new();
        assert_eq!(
            t.analyze_complexity("def add(a, b): return a + b"),
            PatternComplexity::Simple
        );
    }

    #[test]
    fn test_complexity_medium() {
        let t = HybridTranspiler::new();
        assert_eq!(
            t.analyze_complexity("class Foo: pass"),
            PatternComplexity::Medium
        );
        assert_eq!(
            t.analyze_complexity("@decorator\ndef foo(): pass"),
            PatternComplexity::Medium
        );
    }

    #[test]
    fn test_complexity_complex() {
        let t = HybridTranspiler::new();
        assert_eq!(
            t.analyze_complexity("class Meta(type): pass"),
            PatternComplexity::Complex
        );
    }

    #[test]
    fn test_complexity_unsupported() {
        let t = HybridTranspiler::new();
        assert_eq!(
            t.analyze_complexity("eval('x + 1')"),
            PatternComplexity::Unsupported
        );
    }

    #[test]
    fn test_transpile_simple() {
        let mut t = HybridTranspiler::new();
        let result = t.transpile("def add(a: int, b: int) -> int:\n    return a + b");
        assert!(result.is_ok());
        let r = result.unwrap();
        assert_eq!(r.strategy, Strategy::Ast);
        assert!(r.confidence > 0.8);
    }

    #[test]
    fn test_transpile_unsupported() {
        let mut t = HybridTranspiler::new();
        let result = t.transpile("exec('print(1)')");
        assert!(matches!(result, Err(TranspileError::UnsupportedPattern(_))));
    }

    #[test]
    fn test_stats() {
        let mut t = HybridTranspiler::new();
        let _ = t.transpile("def foo(): return 1");
        let _ = t.transpile("def bar(): return 2");
        let stats = t.stats();
        assert_eq!(stats.total_attempts, 2);
    }

    #[test]
    fn test_training_collector() {
        let mut collector = TrainingDataCollector::new();
        collector.add_pair(
            "def add(a, b): return a + b".to_string(),
            "fn add(a: i32, b: i32) -> i32 { a + b }".to_string(),
            "test",
        );
        assert_eq!(collector.len(), 1);

        let jsonl = collector.export_jsonl();
        assert!(jsonl.contains("Translate to Rust"));
    }

    #[test]
    fn test_config_default() {
        let config = HybridConfig::default();
        assert!(config.enable_local_model);
        assert!(!config.enable_api);
        assert!(config.ast_confidence_threshold > 0.5);
    }
}

#[cfg(test)]
mod proptests {
    use super::{
        HybridTranspiler, Strategy as TranspileStrategy, TrainingDataCollector, TranspileError,
    };
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(200))]

        #[test]
        fn prop_complexity_deterministic(code in "[a-z]+\\([a-z, ]*\\): [a-z]+") {
            let t = HybridTranspiler::new();
            let c1 = t.analyze_complexity(&code);
            let c2 = t.analyze_complexity(&code);
            prop_assert_eq!(c1, c2);
        }

        #[test]
        fn prop_simple_code_high_confidence(
            func_name in "[a-z]{3,10}",
            param in "[a-z]"
        ) {
            let code = format!("def {}({}: int) -> int:\n    return {} + 1", func_name, param, param);
            let mut t = HybridTranspiler::new();
            if let Ok(result) = t.transpile(&code) {
                prop_assert!(result.confidence >= 0.8);
                prop_assert_eq!(result.strategy, TranspileStrategy::Ast);
            }
        }

        #[test]
        fn prop_unsupported_always_fails(evil in "(exec|eval)\\('[^']*'\\)") {
            let mut t = HybridTranspiler::new();
            let result = t.transpile(&evil);
            prop_assert!(matches!(result, Err(TranspileError::UnsupportedPattern(_))));
        }

        #[test]
        fn prop_stats_consistent(n in 1usize..10) {
            let mut t = HybridTranspiler::new();
            for i in 0..n {
                let _ = t.transpile(&format!("def f{}(): return {}", i, i));
            }
            let stats = t.stats();
            prop_assert!(stats.total_attempts <= n as u64 * 2); // May retry
        }

        #[test]
        fn prop_training_pair_roundtrip(
            py in "[a-z ]+",
            rs in "[a-z ]+",
        ) {
            let mut collector = TrainingDataCollector::new();
            collector.add_pair(py.clone(), rs.clone(), "proptest");
            prop_assert_eq!(collector.len(), 1);

            let jsonl = collector.export_jsonl();
            prop_assert!(jsonl.contains(&py) || py.is_empty());
        }
    }
}
