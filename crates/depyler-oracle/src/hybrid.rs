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
        let mut config = Self::default();
        config.enable_api = std::env::var("ANTHROPIC_API_KEY").is_ok();
        config
    }

    /// Create config with local model
    pub fn with_local_model(path: &str) -> Self {
        let mut config = Self::default();
        config.enable_local_model = true;
        config.local_model_path = Some(path.to_string());
        config
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

    /// Try local fine-tuned model (placeholder for llama.cpp integration)
    fn try_local_model(&self, python_code: &str) -> Result<TranspileResult, TranspileError> {
        let model_path = self.config.local_model_path.as_ref()
            .ok_or(TranspileError::ModelNotLoaded)?;

        // TODO: Integrate llama-cpp-rs or similar
        // For now, check if model file exists
        if !std::path::Path::new(model_path).exists() {
            return Err(TranspileError::ModelNotLoaded);
        }

        // Placeholder - would use llama.cpp bindings here
        // let ctx = LlamaContext::load(model_path)?;
        // let prompt = format!("Convert Python to Rust:\n```python\n{}\n```\n\nRust:", python_code);
        // let response = ctx.generate(&prompt)?;

        let _ = python_code;
        Err(TranspileError::ModelFailed("Local model inference not yet implemented".to_string()))
    }

    /// Try API-based transpilation (Claude/OpenAI)
    fn try_api_transpile(&self, python_code: &str) -> Result<TranspileResult, TranspileError> {
        let endpoint = self.config.api_endpoint.as_ref()
            .ok_or(TranspileError::ApiNotConfigured)?;
        let api_key = self.config.api_key.as_ref()
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

        let response_json: serde_json::Value = response.into_json()
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
            ast_success_rate: safe_rate(self.pattern_stats.ast_success, self.pattern_stats.ast_failure),
            model_success_rate: safe_rate(self.pattern_stats.model_success, self.pattern_stats.model_failure),
            api_success_rate: safe_rate(self.pattern_stats.api_success, self.pattern_stats.api_failure),
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
    pub fn collect_from_transpiler(&mut self, transpiler: &mut HybridTranspiler, python_samples: &[&str]) {
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
        assert!(result.is_ok(), "Simple function should transpile: {:?}", result);
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
        assert_eq!(config.enable_api, std::env::var("ANTHROPIC_API_KEY").is_ok());
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
