//! Comprehensive tests for HybridTranspiler module
//!
//! Coverage target: hybrid.rs from 70.42% to 95%+

use depyler_oracle::hybrid::{
    HybridConfig, HybridTranspiler, PatternComplexity, Strategy, TrainingDataCollector,
    TranslationPair, TranspileError, TranspileResult, TranspileStats,
};
use std::time::Duration;

mod hybrid_config_tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = HybridConfig::default();
        assert!(!config.enable_local_model);
        assert!(!config.enable_api);
        assert!(config.api_endpoint.is_some());
        assert_eq!(config.api_timeout, Duration::from_secs(30));
        assert_eq!(config.ast_confidence_threshold, 0.8);
        assert_eq!(config.max_api_retries, 2);
        assert!(config.local_model_path.is_none());
    }

    #[test]
    fn test_with_api_config() {
        // Clear any existing key for deterministic test
        let config = HybridConfig::with_api();
        // enable_api should depend on ANTHROPIC_API_KEY env var
        assert_eq!(
            config.enable_api,
            std::env::var("ANTHROPIC_API_KEY").is_ok()
        );
        assert!(!config.enable_local_model);
    }

    #[test]
    fn test_with_local_model_config() {
        let path = "/path/to/model.gguf";
        let config = HybridConfig::with_local_model(path);
        assert!(config.enable_local_model);
        assert_eq!(config.local_model_path, Some(path.to_string()));
        assert!(!config.enable_api);
    }

    #[test]
    fn test_config_serialization() {
        let config = HybridConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: HybridConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.enable_local_model, deserialized.enable_local_model);
        assert_eq!(config.enable_api, deserialized.enable_api);
    }

    #[test]
    fn test_config_clone() {
        let config = HybridConfig {
            enable_local_model: true,
            enable_api: true,
            api_endpoint: Some("https://example.com".to_string()),
            api_key: Some("test-key".to_string()),
            api_timeout: Duration::from_secs(60),
            api_model: "test-model".to_string(),
            ast_confidence_threshold: 0.9,
            max_api_retries: 3,
            local_model_path: Some("/path/to/model".to_string()),
        };
        let cloned = config.clone();
        assert_eq!(config.enable_local_model, cloned.enable_local_model);
        assert_eq!(config.api_endpoint, cloned.api_endpoint);
    }

    #[test]
    fn test_config_debug() {
        let config = HybridConfig::default();
        let debug = format!("{:?}", config);
        assert!(debug.contains("HybridConfig"));
    }
}

mod pattern_complexity_tests {
    use super::*;

    #[test]
    fn test_pattern_complexity_eq() {
        assert_eq!(PatternComplexity::Simple, PatternComplexity::Simple);
        assert_eq!(PatternComplexity::Medium, PatternComplexity::Medium);
        assert_eq!(PatternComplexity::Complex, PatternComplexity::Complex);
        assert_eq!(
            PatternComplexity::Unsupported,
            PatternComplexity::Unsupported
        );
    }

    #[test]
    fn test_pattern_complexity_ne() {
        assert_ne!(PatternComplexity::Simple, PatternComplexity::Medium);
        assert_ne!(PatternComplexity::Medium, PatternComplexity::Complex);
        assert_ne!(PatternComplexity::Complex, PatternComplexity::Unsupported);
    }

    #[test]
    fn test_pattern_complexity_clone() {
        let complexity = PatternComplexity::Complex;
        let cloned = complexity;
        assert_eq!(complexity, cloned);
    }

    #[test]
    fn test_pattern_complexity_copy() {
        let complexity = PatternComplexity::Simple;
        let copied: PatternComplexity = complexity;
        assert_eq!(complexity, copied);
    }

    #[test]
    fn test_pattern_complexity_debug() {
        let debug = format!("{:?}", PatternComplexity::Medium);
        assert!(debug.contains("Medium"));
    }
}

mod strategy_tests {
    use super::*;

    #[test]
    fn test_strategy_eq() {
        assert_eq!(Strategy::Ast, Strategy::Ast);
        assert_eq!(Strategy::LocalModel, Strategy::LocalModel);
        assert_eq!(Strategy::Api, Strategy::Api);
    }

    #[test]
    fn test_strategy_ne() {
        assert_ne!(Strategy::Ast, Strategy::LocalModel);
        assert_ne!(Strategy::LocalModel, Strategy::Api);
    }

    #[test]
    fn test_strategy_serialization() {
        let strategies = vec![Strategy::Ast, Strategy::LocalModel, Strategy::Api];
        for strategy in strategies {
            let json = serde_json::to_string(&strategy).unwrap();
            let deserialized: Strategy = serde_json::from_str(&json).unwrap();
            assert_eq!(strategy, deserialized);
        }
    }

    #[test]
    fn test_strategy_clone() {
        let strategy = Strategy::Api;
        assert_eq!(strategy, strategy.clone());
    }

    #[test]
    fn test_strategy_debug() {
        let debug = format!("{:?}", Strategy::Ast);
        assert!(debug.contains("Ast"));
    }
}

mod transpile_result_tests {
    use super::*;

    #[test]
    fn test_transpile_result_construction() {
        let result = TranspileResult {
            rust_code: "fn main() {}".to_string(),
            strategy: Strategy::Ast,
            confidence: 0.95,
            latency_ms: 10,
            warnings: vec!["test warning".to_string()],
        };
        assert_eq!(result.rust_code, "fn main() {}");
        assert_eq!(result.strategy, Strategy::Ast);
        assert!((result.confidence - 0.95).abs() < f32::EPSILON);
        assert_eq!(result.latency_ms, 10);
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_transpile_result_serialization() {
        let result = TranspileResult {
            rust_code: "let x = 1;".to_string(),
            strategy: Strategy::Api,
            confidence: 0.9,
            latency_ms: 100,
            warnings: vec![],
        };
        let json = serde_json::to_string(&result).unwrap();
        let deserialized: TranspileResult = serde_json::from_str(&json).unwrap();
        assert_eq!(result.rust_code, deserialized.rust_code);
        assert_eq!(result.strategy, deserialized.strategy);
    }

    #[test]
    fn test_transpile_result_clone() {
        let result = TranspileResult {
            rust_code: "fn test() {}".to_string(),
            strategy: Strategy::LocalModel,
            confidence: 0.85,
            latency_ms: 50,
            warnings: vec!["w1".to_string(), "w2".to_string()],
        };
        let cloned = result.clone();
        assert_eq!(result.rust_code, cloned.rust_code);
        assert_eq!(result.warnings.len(), cloned.warnings.len());
    }

    #[test]
    fn test_transpile_result_debug() {
        let result = TranspileResult {
            rust_code: "x".to_string(),
            strategy: Strategy::Ast,
            confidence: 1.0,
            latency_ms: 0,
            warnings: vec![],
        };
        let debug = format!("{:?}", result);
        assert!(debug.contains("TranspileResult"));
    }
}

mod transpile_stats_tests {
    use super::*;

    #[test]
    fn test_transpile_stats_construction() {
        let stats = TranspileStats {
            total_attempts: 100,
            ast_success_rate: 0.9,
            model_success_rate: 0.5,
            api_success_rate: 0.95,
        };
        assert_eq!(stats.total_attempts, 100);
        assert!((stats.ast_success_rate - 0.9).abs() < f32::EPSILON);
    }

    #[test]
    fn test_transpile_stats_serialization() {
        let stats = TranspileStats {
            total_attempts: 50,
            ast_success_rate: 0.8,
            model_success_rate: 0.0,
            api_success_rate: 0.7,
        };
        let json = serde_json::to_string(&stats).unwrap();
        let deserialized: TranspileStats = serde_json::from_str(&json).unwrap();
        assert_eq!(stats.total_attempts, deserialized.total_attempts);
    }

    #[test]
    fn test_transpile_stats_clone() {
        let stats = TranspileStats {
            total_attempts: 10,
            ast_success_rate: 0.5,
            model_success_rate: 0.3,
            api_success_rate: 0.8,
        };
        let cloned = stats.clone();
        assert_eq!(stats.total_attempts, cloned.total_attempts);
    }

    #[test]
    fn test_transpile_stats_debug() {
        let stats = TranspileStats {
            total_attempts: 0,
            ast_success_rate: 0.0,
            model_success_rate: 0.0,
            api_success_rate: 0.0,
        };
        let debug = format!("{:?}", stats);
        assert!(debug.contains("TranspileStats"));
    }
}

mod transpile_error_tests {
    use super::*;

    #[test]
    fn test_unsupported_pattern_error() {
        let err = TranspileError::UnsupportedPattern("eval detected".to_string());
        let msg = err.to_string();
        assert!(msg.contains("Unsupported pattern"));
        assert!(msg.contains("eval detected"));
    }

    #[test]
    fn test_ast_failed_error() {
        let err = TranspileError::AstFailed("parse error".to_string());
        let msg = err.to_string();
        assert!(msg.contains("AST transpilation failed"));
    }

    #[test]
    fn test_model_not_loaded_error() {
        let err = TranspileError::ModelNotLoaded;
        let msg = err.to_string();
        assert!(msg.contains("not loaded"));
    }

    #[test]
    fn test_model_failed_error() {
        let err = TranspileError::ModelFailed("inference error".to_string());
        let msg = err.to_string();
        assert!(msg.contains("inference failed"));
    }

    #[test]
    fn test_api_not_configured_error() {
        let err = TranspileError::ApiNotConfigured;
        let msg = err.to_string();
        assert!(msg.contains("not configured"));
    }

    #[test]
    fn test_api_failed_error() {
        let err = TranspileError::ApiFailed("network error".to_string());
        let msg = err.to_string();
        assert!(msg.contains("API call failed"));
    }

    #[test]
    fn test_all_strategies_failed_error() {
        let err = TranspileError::AllStrategiesFailed;
        let msg = err.to_string();
        assert!(msg.contains("All strategies failed"));
    }

    #[test]
    fn test_error_debug() {
        let err = TranspileError::UnsupportedPattern("test".to_string());
        let debug = format!("{:?}", err);
        assert!(debug.contains("UnsupportedPattern"));
    }
}

mod hybrid_transpiler_tests {
    use super::*;

    #[test]
    fn test_new() {
        let transpiler = HybridTranspiler::new();
        let stats = transpiler.stats();
        assert_eq!(stats.total_attempts, 0);
    }

    #[test]
    fn test_default() {
        let transpiler = HybridTranspiler::default();
        let stats = transpiler.stats();
        assert_eq!(stats.total_attempts, 0);
    }

    #[test]
    fn test_with_config() {
        let config = HybridConfig {
            enable_local_model: false,
            enable_api: false,
            ast_confidence_threshold: 0.5,
            ..HybridConfig::default()
        };
        let transpiler = HybridTranspiler::with_config(config);
        assert_eq!(transpiler.stats().total_attempts, 0);
    }

    #[test]
    fn test_analyze_complexity_simple() {
        let transpiler = HybridTranspiler::new();
        assert_eq!(
            transpiler.analyze_complexity("x = 1"),
            PatternComplexity::Simple
        );
        assert_eq!(
            transpiler.analyze_complexity("def foo(): pass"),
            PatternComplexity::Simple
        );
        assert_eq!(
            transpiler.analyze_complexity("if x: y = 1"),
            PatternComplexity::Simple
        );
    }

    #[test]
    fn test_analyze_complexity_medium() {
        let transpiler = HybridTranspiler::new();
        assert_eq!(
            transpiler.analyze_complexity("class Foo: pass"),
            PatternComplexity::Medium
        );
        assert_eq!(
            transpiler.analyze_complexity("@decorator\ndef foo(): pass"),
            PatternComplexity::Medium
        );
        assert_eq!(
            transpiler.analyze_complexity("yield x"),
            PatternComplexity::Medium
        );
        assert_eq!(
            transpiler.analyze_complexity("async def foo(): pass"),
            PatternComplexity::Medium
        );
        assert_eq!(
            transpiler.analyze_complexity("lambda x: x"),
            PatternComplexity::Medium
        );
        assert_eq!(
            transpiler.analyze_complexity("type(x)"),
            PatternComplexity::Medium
        );
    }

    #[test]
    fn test_analyze_complexity_complex() {
        let transpiler = HybridTranspiler::new();
        assert_eq!(
            transpiler.analyze_complexity("class Meta(type): pass"),
            PatternComplexity::Complex
        );
        assert_eq!(
            transpiler.analyze_complexity("class Foo(metaclass=Meta): pass"),
            PatternComplexity::Complex
        );
        assert_eq!(
            transpiler.analyze_complexity("def __new__(cls): pass"),
            PatternComplexity::Complex
        );
        assert_eq!(
            transpiler.analyze_complexity("def __getattr__(self, name): pass"),
            PatternComplexity::Complex
        );
        assert_eq!(
            transpiler.analyze_complexity("globals()"),
            PatternComplexity::Complex
        );
        assert_eq!(
            transpiler.analyze_complexity("locals()"),
            PatternComplexity::Complex
        );
    }

    #[test]
    fn test_analyze_complexity_unsupported() {
        let transpiler = HybridTranspiler::new();
        assert_eq!(
            transpiler.analyze_complexity("exec('code')"),
            PatternComplexity::Unsupported
        );
        assert_eq!(
            transpiler.analyze_complexity("eval('1 + 1')"),
            PatternComplexity::Unsupported
        );
        assert_eq!(
            transpiler.analyze_complexity("__import__('os')"),
            PatternComplexity::Unsupported
        );
    }

    #[test]
    fn test_transpile_simple() {
        let mut transpiler = HybridTranspiler::new();
        let result = transpiler.transpile("def add(a: int, b: int) -> int:\n    return a + b");
        assert!(result.is_ok());
        let r = result.unwrap();
        assert_eq!(r.strategy, Strategy::Ast);
        assert!(r.confidence >= 0.8);
    }

    #[test]
    fn test_transpile_unsupported() {
        let mut transpiler = HybridTranspiler::new();
        let result = transpiler.transpile("exec('print(1)')");
        assert!(matches!(result, Err(TranspileError::UnsupportedPattern(_))));
    }

    #[test]
    fn test_transpile_increments_stats() {
        let mut transpiler = HybridTranspiler::new();
        let _ = transpiler.transpile("def foo(): return 1");
        let _ = transpiler.transpile("def bar(): return 2");
        let stats = transpiler.stats();
        assert!(stats.total_attempts >= 2);
    }

    #[test]
    fn test_stats_initial() {
        let transpiler = HybridTranspiler::new();
        let stats = transpiler.stats();
        assert_eq!(stats.total_attempts, 0);
        assert_eq!(stats.ast_success_rate, 0.0);
        assert_eq!(stats.model_success_rate, 0.0);
        assert_eq!(stats.api_success_rate, 0.0);
    }

    #[test]
    fn test_stats_after_successful_transpile() {
        let mut transpiler = HybridTranspiler::new();
        let _ = transpiler.transpile("def foo(): return 1");
        let stats = transpiler.stats();
        assert!(stats.total_attempts >= 1);
        assert!(stats.ast_success_rate > 0.0);
    }
}

mod training_data_collector_tests {
    use super::*;

    #[test]
    fn test_new() {
        let collector = TrainingDataCollector::new();
        assert!(collector.is_empty());
        assert_eq!(collector.len(), 0);
    }

    #[test]
    fn test_default() {
        let collector = TrainingDataCollector::default();
        assert!(collector.is_empty());
    }

    #[test]
    fn test_add_pair() {
        let mut collector = TrainingDataCollector::new();
        collector.add_pair(
            "def add(a, b): return a + b".to_string(),
            "fn add(a: i32, b: i32) -> i32 { a + b }".to_string(),
            "test",
        );
        assert_eq!(collector.len(), 1);
        assert!(!collector.is_empty());
    }

    #[test]
    fn test_add_multiple_pairs() {
        let mut collector = TrainingDataCollector::new();
        for i in 0..10 {
            collector.add_pair(
                format!("def f{}(): return {}", i, i),
                format!("fn f{}() -> i32 {{ {} }}", i, i),
                "test",
            );
        }
        assert_eq!(collector.len(), 10);
    }

    #[test]
    fn test_collect_from_transpiler() {
        let mut collector = TrainingDataCollector::new();
        let mut transpiler = HybridTranspiler::new();
        let samples = vec![
            "def add(a: int, b: int) -> int:\n    return a + b",
            "def sub(x: int, y: int) -> int:\n    return x - y",
        ];
        collector.collect_from_transpiler(&mut transpiler, &samples);
        // Should have collected pairs from successful transpilations
        // Collection may or may not occur depending on confidence - test just verifies no panic
    }

    #[test]
    fn test_export_jsonl() {
        let mut collector = TrainingDataCollector::new();
        collector.add_pair(
            "def foo(): pass".to_string(),
            "fn foo() {}".to_string(),
            "test",
        );
        let jsonl = collector.export_jsonl();
        assert!(jsonl.contains("Convert to Rust"));
        assert!(jsonl.contains("def foo"));
        assert!(jsonl.contains("fn foo"));
    }

    #[test]
    fn test_export_jsonl_empty() {
        let collector = TrainingDataCollector::new();
        let jsonl = collector.export_jsonl();
        assert!(jsonl.is_empty());
    }

    #[test]
    fn test_export_alpaca() {
        let mut collector = TrainingDataCollector::new();
        collector.add_pair("x = 1".to_string(), "let x = 1;".to_string(), "test");
        let alpaca = collector.export_alpaca();
        assert!(alpaca.contains("instruction"));
        assert!(alpaca.contains("input"));
        assert!(alpaca.contains("output"));
    }

    #[test]
    fn test_export_alpaca_empty() {
        let collector = TrainingDataCollector::new();
        let alpaca = collector.export_alpaca();
        assert!(alpaca.is_empty());
    }

    #[test]
    fn test_serialization() {
        let mut collector = TrainingDataCollector::new();
        collector.add_pair("py".to_string(), "rs".to_string(), "test");
        let json = serde_json::to_string(&collector).unwrap();
        let deserialized: TrainingDataCollector = serde_json::from_str(&json).unwrap();
        assert_eq!(collector.len(), deserialized.len());
    }

    #[test]
    fn test_debug() {
        let collector = TrainingDataCollector::new();
        let debug = format!("{:?}", collector);
        assert!(debug.contains("TrainingDataCollector"));
    }
}

mod translation_pair_tests {
    use super::*;

    #[test]
    fn test_construction() {
        let pair = TranslationPair {
            python: "def foo(): pass".to_string(),
            rust: "fn foo() {}".to_string(),
            verified: true,
            source: "manual".to_string(),
        };
        assert_eq!(pair.python, "def foo(): pass");
        assert_eq!(pair.rust, "fn foo() {}");
        assert!(pair.verified);
        assert_eq!(pair.source, "manual");
    }

    #[test]
    fn test_clone() {
        let pair = TranslationPair {
            python: "x".to_string(),
            rust: "y".to_string(),
            verified: false,
            source: "test".to_string(),
        };
        let cloned = pair.clone();
        assert_eq!(pair.python, cloned.python);
        assert_eq!(pair.verified, cloned.verified);
    }

    #[test]
    fn test_serialization() {
        let pair = TranslationPair {
            python: "a".to_string(),
            rust: "b".to_string(),
            verified: true,
            source: "auto".to_string(),
        };
        let json = serde_json::to_string(&pair).unwrap();
        let deserialized: TranslationPair = serde_json::from_str(&json).unwrap();
        assert_eq!(pair.python, deserialized.python);
        assert_eq!(pair.rust, deserialized.rust);
    }

    #[test]
    fn test_debug() {
        let pair = TranslationPair {
            python: "p".to_string(),
            rust: "r".to_string(),
            verified: true,
            source: "s".to_string(),
        };
        let debug = format!("{:?}", pair);
        assert!(debug.contains("TranslationPair"));
    }
}

mod integration_tests {
    use super::*;

    #[test]
    fn test_full_workflow() {
        // Create transpiler
        let mut transpiler = HybridTranspiler::new();

        // Transpile multiple samples
        let samples = vec![
            "def add(a: int, b: int) -> int:\n    return a + b",
            "def mul(x: int, y: int) -> int:\n    return x * y",
            "def neg(n: int) -> int:\n    return -n",
        ];

        for sample in &samples {
            let result = transpiler.transpile(sample);
            assert!(result.is_ok());
        }

        // Check stats
        let stats = transpiler.stats();
        assert!(stats.total_attempts >= 3);
        assert!(stats.ast_success_rate > 0.0);

        // Collect training data
        let mut collector = TrainingDataCollector::new();
        collector.collect_from_transpiler(&mut transpiler, &samples);

        // Export formats
        let jsonl = collector.export_jsonl();
        let alpaca = collector.export_alpaca();

        // Both exports should work
        drop(jsonl);
        drop(alpaca);
    }

    #[test]
    fn test_mixed_complexity_samples() {
        let mut transpiler = HybridTranspiler::new();

        // Simple
        let r1 = transpiler.transpile("x = 1");
        assert!(r1.is_ok());

        // Medium (class) - may or may not succeed
        let _r2 = transpiler.transpile("class Foo: pass");

        // Unsupported
        let r3 = transpiler.transpile("eval('1')");
        assert!(matches!(r3, Err(TranspileError::UnsupportedPattern(_))));

        // Stats should reflect attempts
        let stats = transpiler.stats();
        assert!(stats.total_attempts >= 1);
    }
}
