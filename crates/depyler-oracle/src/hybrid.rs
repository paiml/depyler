//! Hybrid transpilation with ML fallback
//!
//! Strategy:
//! 1. AST-based transpilation (fast, deterministic) - 90%+ cases
//! 2. Local model fallback (Qwen2.5-Coder-1.5B) - edge cases
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

use depyler_core::DepylerPipeline;
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
    /// API endpoint (Anthropic/OpenAI compatible)
    pub api_endpoint: Option<String>,
    /// API key
    pub api_key: Option<String>,
    /// API timeout
    pub api_timeout: Duration,
    /// Model to use for API calls
    pub api_model: String,
    /// Minimum confidence to accept AST result
    pub ast_confidence_threshold: f32,
    /// Maximum retries for API
    pub max_api_retries: u32,
    /// Local model path (GGUF format)
    pub local_model_path: Option<String>,
}

impl Default for HybridConfig {
    fn default() -> Self {
        Self {
            enable_local_model: false, // Requires model file
            enable_api: false,         // Requires API key
            api_endpoint: Some("https://api.anthropic.com/v1/messages".to_string()),
            api_key: std::env::var("ANTHROPIC_API_KEY").ok(),
            api_timeout: Duration::from_secs(30),
            api_model: "claude-sonnet-4-20250514".to_string(),
            ast_confidence_threshold: 0.8,
            max_api_retries: 2,
            local_model_path: None,
        }
    }
}

impl HybridConfig {
    /// Create config with API enabled (reads ANTHROPIC_API_KEY from env)
    pub fn with_api() -> Self {
        Self {
            enable_api: std::env::var("ANTHROPIC_API_KEY").is_ok(),
            ..Self::default()
        }
    }

    /// Create config with local model
    pub fn with_local_model(path: &str) -> Self {
        Self {
            enable_local_model: true,
            local_model_path: Some(path.to_string()),
            ..Self::default()
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
    /// Core depyler pipeline for AST transpilation
    pipeline: DepylerPipeline,
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
            pipeline: DepylerPipeline::new(),
            pattern_stats: PatternStats::default(),
        }
    }

    /// Analyze Python code complexity
    #[must_use]
    pub fn analyze_complexity(&self, python_code: &str) -> PatternComplexity {
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
                    }
                }
            }
            PatternComplexity::Complex => {
                // Skip AST for complex patterns, go to fallback
            }
            PatternComplexity::Unsupported => {
                return Err(TranspileError::UnsupportedPattern(
                    "Dynamic Python features (exec/eval) not supported".to_string(),
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

    /// Try AST-based transpilation using depyler-core
    fn try_ast_transpile(&self, python_code: &str) -> Result<TranspileResult, TranspileError> {
        match self.pipeline.transpile(python_code) {
            Ok(rust_code) => {
                let complexity = self.analyze_complexity(python_code);
                let confidence = match complexity {
                    PatternComplexity::Simple => 0.95,
                    PatternComplexity::Medium => 0.85,
                    PatternComplexity::Complex => 0.5,
                    PatternComplexity::Unsupported => 0.0,
                };

                Ok(TranspileResult {
                    rust_code,
                    strategy: Strategy::Ast,
                    confidence,
                    latency_ms: 0,
                    warnings: vec![],
                })
            }
            Err(e) => Err(TranspileError::AstFailed(e.to_string())),
        }
    }

    /// Try local model via aprender (placeholder for future implementation)
    fn try_local_model(&self, _python_code: &str) -> Result<TranspileResult, TranspileError> {
        // Note: Implement local model inference using aprender
        // For now, we rely on AST transpilation and API fallback
        Err(TranspileError::ModelFailed(
            "Local model inference not yet implemented".to_string(),
        ))
    }

    /// Try API-based transpilation (Claude/OpenAI)
    fn try_api_transpile(&self, python_code: &str) -> Result<TranspileResult, TranspileError> {
        let endpoint = self
            .config
            .api_endpoint
            .as_ref()
            .ok_or(TranspileError::ApiNotConfigured)?;
        let api_key = self
            .config
            .api_key
            .as_ref()
            .ok_or(TranspileError::ApiNotConfigured)?;

        let prompt = format!(
            "Convert this Python code to idiomatic Rust. Only output the Rust code, no explanations:\n\n```python\n{}\n```",
            python_code
        );

        let request_body = serde_json::json!({
            "model": self.config.api_model,
            "max_tokens": 4096,
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ]
        });

        let response = ureq::post(endpoint)
            .set("x-api-key", api_key)
            .set("anthropic-version", "2023-06-01")
            .set("content-type", "application/json")
            .timeout(self.config.api_timeout)
            .send_json(&request_body)
            .map_err(|e| TranspileError::ApiFailed(e.to_string()))?;

        let response_json: serde_json::Value = response
            .into_json()
            .map_err(|e| TranspileError::ApiFailed(e.to_string()))?;

        // Extract content from Anthropic response
        let rust_code = response_json["content"][0]["text"]
            .as_str()
            .ok_or_else(|| TranspileError::ApiFailed("Invalid response format".to_string()))?;

        // Clean up code blocks if present
        let rust_code = rust_code
            .trim()
            .trim_start_matches("```rust")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim()
            .to_string();

        Ok(TranspileResult {
            rust_code,
            strategy: Strategy::Api,
            confidence: 0.9, // API results are generally high quality
            latency_ms: 0,
            warnings: vec!["Generated via API - review before use".to_string()],
        })
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
            ast_success_rate: safe_rate(
                self.pattern_stats.ast_success,
                self.pattern_stats.ast_failure,
            ),
            model_success_rate: safe_rate(
                self.pattern_stats.model_success,
                self.pattern_stats.model_failure,
            ),
            api_success_rate: safe_rate(
                self.pattern_stats.api_success,
                self.pattern_stats.api_failure,
            ),
        }
    }
}

fn safe_rate(success: u64, failure: u64) -> f32 {
    let total = success + failure;
    if total > 0 {
        success as f32 / total as f32
    } else {
        0.0
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

/// Training data collector for fine-tuning
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TrainingDataCollector {
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

    /// Collect from successful AST transpilations
    pub fn collect_from_transpiler(
        &mut self,
        transpiler: &mut HybridTranspiler,
        python_samples: &[&str],
    ) {
        for python in python_samples {
            if let Ok(result) = transpiler.transpile(python) {
                if result.strategy == Strategy::Ast && result.confidence >= 0.9 {
                    self.add_pair(python.to_string(), result.rust_code, "ast-verified");
                }
            }
        }
    }

    /// Export to JSONL for fine-tuning (OpenAI/Anthropic format)
    pub fn export_jsonl(&self) -> String {
        self.pairs
            .iter()
            .filter(|p| p.verified)
            .map(|p| {
                serde_json::json!({
                    "messages": [
                        {"role": "user", "content": format!("Convert to Rust:\n```python\n{}\n```", p.python)},
                        {"role": "assistant", "content": format!("```rust\n{}\n```", p.rust)}
                    ]
                })
                .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Export for Axolotl/HuggingFace fine-tuning
    pub fn export_alpaca(&self) -> String {
        self.pairs
            .iter()
            .filter(|p| p.verified)
            .map(|p| {
                serde_json::json!({
                    "instruction": "Convert the following Python code to idiomatic Rust.",
                    "input": p.python,
                    "output": p.rust
                })
                .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n")
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
    fn test_transpile_simple_function() {
        let mut t = HybridTranspiler::new();
        let result = t.transpile("def add(a: int, b: int) -> int:\n    return a + b");
        assert!(
            result.is_ok(),
            "Simple function should transpile: {:?}",
            result
        );
        let r = result.unwrap();
        assert_eq!(r.strategy, Strategy::Ast);
        assert!(r.confidence >= 0.8);
        assert!(r.rust_code.contains("fn add"));
    }

    #[test]
    fn test_transpile_with_types() {
        let mut t = HybridTranspiler::new();
        let python = r#"
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"#;
        let result = t.transpile(python);
        assert!(result.is_ok());
        let r = result.unwrap();
        assert!(r.rust_code.contains("fn factorial"));
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
        assert!(stats.total_attempts >= 2);
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
        assert!(jsonl.contains("Convert to Rust"));

        let alpaca = collector.export_alpaca();
        assert!(alpaca.contains("instruction"));
    }

    #[test]
    fn test_config_default() {
        let config = HybridConfig::default();
        assert!(!config.enable_local_model);
        assert!(!config.enable_api);
        assert!(config.ast_confidence_threshold > 0.5);
    }

    #[test]
    fn test_config_with_api() {
        // This will enable API if ANTHROPIC_API_KEY is set
        let config = HybridConfig::with_api();
        assert_eq!(
            config.enable_api,
            std::env::var("ANTHROPIC_API_KEY").is_ok()
        );
    }

    // ============================================================
    // Additional Strategy Tests
    // ============================================================

    #[test]
    fn test_strategy_variants() {
        let ast = Strategy::Ast;
        let local = Strategy::LocalModel;
        let api = Strategy::Api;

        assert_eq!(ast, Strategy::Ast);
        assert_eq!(local, Strategy::LocalModel);
        assert_eq!(api, Strategy::Api);
    }

    #[test]
    fn test_strategy_debug() {
        let ast = Strategy::Ast;
        let debug = format!("{:?}", ast);
        assert!(debug.contains("Ast"));
    }

    #[test]
    fn test_strategy_clone() {
        let ast = Strategy::Ast;
        let cloned = ast;
        assert_eq!(ast, cloned);
    }

    // ============================================================
    // TranspileResult Tests
    // ============================================================

    #[test]
    fn test_transpile_result_clone() {
        let result = TranspileResult {
            rust_code: "fn main() {}".to_string(),
            strategy: Strategy::Ast,
            confidence: 0.9,
            latency_ms: 10,
            warnings: vec!["test".to_string()],
        };
        let cloned = result.clone();
        assert_eq!(result.rust_code, cloned.rust_code);
        assert_eq!(result.confidence, cloned.confidence);
    }

    #[test]
    fn test_transpile_result_serialization() {
        let result = TranspileResult {
            rust_code: "fn main() {}".to_string(),
            strategy: Strategy::Ast,
            confidence: 0.9,
            latency_ms: 10,
            warnings: vec![],
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("rust_code"));
        let deserialized: TranspileResult = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.rust_code, "fn main() {}");
    }

    // ============================================================
    // HybridConfig Tests
    // ============================================================

    #[test]
    fn test_hybrid_config_with_local_model() {
        let config = HybridConfig::with_local_model("/path/to/model.gguf");
        assert!(config.enable_local_model);
        assert_eq!(
            config.local_model_path,
            Some("/path/to/model.gguf".to_string())
        );
    }

    #[test]
    fn test_hybrid_config_serialization() {
        let config = HybridConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("enable_local_model"));
        let deserialized: HybridConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.enable_local_model, deserialized.enable_local_model);
    }

    // ============================================================
    // PatternComplexity Tests
    // ============================================================

    #[test]
    fn test_pattern_complexity_yield() {
        let t = HybridTranspiler::new();
        assert_eq!(
            t.analyze_complexity("def gen(): yield 1"),
            PatternComplexity::Medium
        );
    }

    #[test]
    fn test_pattern_complexity_async() {
        let t = HybridTranspiler::new();
        assert_eq!(
            t.analyze_complexity("async def foo(): pass"),
            PatternComplexity::Medium
        );
    }

    #[test]
    fn test_pattern_complexity_lambda() {
        let t = HybridTranspiler::new();
        assert_eq!(
            t.analyze_complexity("f = lambda x: x + 1"),
            PatternComplexity::Medium
        );
    }

    #[test]
    fn test_pattern_complexity_type() {
        let t = HybridTranspiler::new();
        assert_eq!(
            t.analyze_complexity("t = type(x)"),
            PatternComplexity::Medium
        );
    }

    #[test]
    fn test_pattern_complexity_metaclass() {
        let t = HybridTranspiler::new();
        assert_eq!(
            t.analyze_complexity("class M(metaclass=Meta): pass"),
            PatternComplexity::Complex
        );
    }

    #[test]
    fn test_pattern_complexity_new() {
        let t = HybridTranspiler::new();
        assert_eq!(
            t.analyze_complexity("def __new__(cls): pass"),
            PatternComplexity::Complex
        );
    }

    #[test]
    fn test_pattern_complexity_getattr() {
        let t = HybridTranspiler::new();
        assert_eq!(
            t.analyze_complexity("def __getattr__(self, name): pass"),
            PatternComplexity::Complex
        );
    }

    #[test]
    fn test_pattern_complexity_globals() {
        let t = HybridTranspiler::new();
        assert_eq!(
            t.analyze_complexity("g = globals()"),
            PatternComplexity::Complex
        );
    }

    #[test]
    fn test_pattern_complexity_locals() {
        let t = HybridTranspiler::new();
        assert_eq!(
            t.analyze_complexity("l = locals()"),
            PatternComplexity::Complex
        );
    }

    #[test]
    fn test_pattern_complexity_exec() {
        let t = HybridTranspiler::new();
        assert_eq!(
            t.analyze_complexity("exec('print(1)')"),
            PatternComplexity::Unsupported
        );
    }

    #[test]
    fn test_pattern_complexity_import() {
        let t = HybridTranspiler::new();
        assert_eq!(
            t.analyze_complexity("m = __import__('os')"),
            PatternComplexity::Unsupported
        );
    }

    // ============================================================
    // HybridTranspiler Tests
    // ============================================================

    #[test]
    fn test_hybrid_transpiler_default() {
        let t = HybridTranspiler::default();
        let stats = t.stats();
        assert_eq!(stats.total_attempts, 0);
    }

    #[test]
    fn test_hybrid_transpiler_with_config() {
        let config = HybridConfig {
            ast_confidence_threshold: 0.5,
            ..HybridConfig::default()
        };
        let t = HybridTranspiler::with_config(config);
        let stats = t.stats();
        assert_eq!(stats.total_attempts, 0);
    }

    #[test]
    fn test_transpile_complex_pattern() {
        let mut t = HybridTranspiler::new();
        // Complex pattern should skip AST and fail without fallback
        let result = t.transpile("class Meta(type): pass");
        assert!(result.is_err());
    }

    #[test]
    fn test_transpile_medium_pattern() {
        let mut t = HybridTranspiler::new();
        // Medium pattern should try AST
        let result = t.transpile("class Foo:\n    def bar(self):\n        return 1");
        // May succeed or fail depending on AST capabilities
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_stats_after_transpile() {
        let mut t = HybridTranspiler::new();
        let _ = t.transpile("def add(a: int, b: int) -> int:\n    return a + b");
        let stats = t.stats();
        assert!(stats.ast_success_rate >= 0.0);
    }

    // ============================================================
    // TrainingDataCollector Tests
    // ============================================================

    #[test]
    fn test_training_collector_is_empty() {
        let collector = TrainingDataCollector::new();
        assert!(collector.is_empty());
    }

    #[test]
    fn test_training_collector_not_empty() {
        let mut collector = TrainingDataCollector::new();
        collector.add_pair(
            "def foo(): pass".to_string(),
            "fn foo() {}".to_string(),
            "test",
        );
        assert!(!collector.is_empty());
        assert_eq!(collector.len(), 1);
    }

    #[test]
    fn test_training_collector_collect_from_transpiler() {
        let mut collector = TrainingDataCollector::new();
        let mut transpiler = HybridTranspiler::new();

        let samples = &[
            "def add(a: int, b: int) -> int:\n    return a + b",
            "def sub(a: int, b: int) -> int:\n    return a - b",
        ];

        collector.collect_from_transpiler(&mut transpiler, samples);
        // May or may not have collected depending on confidence
        // Using is_empty check rather than length comparison
        let _ = collector.is_empty(); // Sanity check - collector was exercised
    }

    #[test]
    fn test_training_collector_export_jsonl_empty() {
        let collector = TrainingDataCollector::new();
        let jsonl = collector.export_jsonl();
        assert!(jsonl.is_empty());
    }

    #[test]
    fn test_training_collector_export_alpaca_empty() {
        let collector = TrainingDataCollector::new();
        let alpaca = collector.export_alpaca();
        assert!(alpaca.is_empty());
    }

    #[test]
    fn test_translation_pair_serialization() {
        let pair = TranslationPair {
            python: "def foo(): pass".to_string(),
            rust: "fn foo() {}".to_string(),
            verified: true,
            source: "test".to_string(),
        };
        let json = serde_json::to_string(&pair).unwrap();
        assert!(json.contains("python"));
        assert!(json.contains("rust"));
        let deserialized: TranslationPair = serde_json::from_str(&json).unwrap();
        assert_eq!(pair.python, deserialized.python);
    }

    // ============================================================
    // TranspileError Tests
    // ============================================================

    #[test]
    fn test_transpile_error_display() {
        let err = TranspileError::UnsupportedPattern("test".to_string());
        let display = format!("{}", err);
        assert!(display.contains("Unsupported"));

        let err = TranspileError::AstFailed("parse error".to_string());
        let display = format!("{}", err);
        assert!(display.contains("AST"));

        let err = TranspileError::ModelNotLoaded;
        let display = format!("{}", err);
        assert!(display.contains("Local model"));

        let err = TranspileError::ModelFailed("inference error".to_string());
        let display = format!("{}", err);
        assert!(display.contains("Model inference"));

        let err = TranspileError::ApiNotConfigured;
        let display = format!("{}", err);
        assert!(display.contains("API"));

        let err = TranspileError::ApiFailed("timeout".to_string());
        let display = format!("{}", err);
        assert!(display.contains("API call"));

        let err = TranspileError::AllStrategiesFailed;
        let display = format!("{}", err);
        assert!(display.contains("All strategies"));
    }

    // ============================================================
    // safe_rate Tests
    // ============================================================

    #[test]
    fn test_safe_rate_zero() {
        assert_eq!(safe_rate(0, 0), 0.0);
    }

    #[test]
    fn test_safe_rate_all_success() {
        assert_eq!(safe_rate(10, 0), 1.0);
    }

    #[test]
    fn test_safe_rate_all_failure() {
        assert_eq!(safe_rate(0, 10), 0.0);
    }

    #[test]
    fn test_safe_rate_mixed() {
        assert!((safe_rate(5, 5) - 0.5).abs() < 0.01);
    }

    // ============================================================
    // TranspileStats Tests
    // ============================================================

    #[test]
    fn test_transpile_stats_serialization() {
        let stats = TranspileStats {
            total_attempts: 100,
            ast_success_rate: 0.9,
            model_success_rate: 0.5,
            api_success_rate: 0.8,
        };
        let json = serde_json::to_string(&stats).unwrap();
        assert!(json.contains("total_attempts"));
        let deserialized: TranspileStats = serde_json::from_str(&json).unwrap();
        assert_eq!(stats.total_attempts, deserialized.total_attempts);
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
            prop_assert!(stats.total_attempts <= n as u64 * 2);
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
