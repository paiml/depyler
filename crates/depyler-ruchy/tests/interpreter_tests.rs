//! Comprehensive tests for RuchyInterpreter module
//!
//! Coverage target: interpreter.rs from 62.84% to 95%+

use depyler_ruchy::interpreter::RuchyInterpreter;
use depyler_ruchy::RuchyConfig;

mod interpreter_creation {
    use super::*;

    #[test]
    fn test_new_creates_empty_context() {
        let interpreter = RuchyInterpreter::new();
        assert!(interpreter.get_context("nonexistent").is_none());
    }

    #[test]
    fn test_default_impl_same_as_new() {
        let from_new = RuchyInterpreter::new();
        let from_default = RuchyInterpreter::default();

        // Both should have empty contexts
        assert!(from_new.get_context("test").is_none());
        assert!(from_default.get_context("test").is_none());
    }

    #[test]
    fn test_with_config_custom() {
        let config = RuchyConfig::default();
        let interpreter = RuchyInterpreter::with_config(&config);
        // Should be created with custom config
        assert!(interpreter.get_context("test").is_none());
    }

    #[test]
    fn test_with_config_default() {
        let config = RuchyConfig::default();
        let interpreter = RuchyInterpreter::with_config(&config);
        assert!(interpreter.get_context("key").is_none());
    }
}

mod context_management {
    use super::*;

    #[test]
    fn test_set_and_get_context() {
        let mut interpreter = RuchyInterpreter::new();

        interpreter.set_context("x".to_string(), "42".to_string());
        assert_eq!(interpreter.get_context("x"), Some(&"42".to_string()));
    }

    #[test]
    fn test_get_nonexistent_context() {
        let interpreter = RuchyInterpreter::new();
        assert!(interpreter.get_context("nonexistent").is_none());
    }

    #[test]
    fn test_clear_context() {
        let mut interpreter = RuchyInterpreter::new();

        interpreter.set_context("a".to_string(), "1".to_string());
        interpreter.set_context("b".to_string(), "2".to_string());

        interpreter.clear_context();

        assert!(interpreter.get_context("a").is_none());
        assert!(interpreter.get_context("b").is_none());
    }

    #[test]
    fn test_overwrite_context() {
        let mut interpreter = RuchyInterpreter::new();

        interpreter.set_context("key".to_string(), "old".to_string());
        assert_eq!(interpreter.get_context("key"), Some(&"old".to_string()));

        interpreter.set_context("key".to_string(), "new".to_string());
        assert_eq!(interpreter.get_context("key"), Some(&"new".to_string()));
    }

    #[test]
    fn test_multiple_context_entries() {
        let mut interpreter = RuchyInterpreter::new();

        for i in 0..100 {
            interpreter.set_context(format!("key_{}", i), format!("value_{}", i));
        }

        for i in 0..100 {
            assert_eq!(
                interpreter.get_context(&format!("key_{}", i)),
                Some(&format!("value_{}", i))
            );
        }
    }

    #[test]
    fn test_empty_string_key() {
        let mut interpreter = RuchyInterpreter::new();
        interpreter.set_context(String::new(), "value".to_string());
        assert_eq!(interpreter.get_context(""), Some(&"value".to_string()));
    }

    #[test]
    fn test_empty_string_value() {
        let mut interpreter = RuchyInterpreter::new();
        interpreter.set_context("key".to_string(), String::new());
        assert_eq!(interpreter.get_context("key"), Some(&String::new()));
    }

    #[test]
    fn test_unicode_context() {
        let mut interpreter = RuchyInterpreter::new();
        interpreter.set_context("日本語".to_string(), "🎉".to_string());
        assert_eq!(interpreter.get_context("日本語"), Some(&"🎉".to_string()));
    }
}

mod execution_without_interpreter_feature {
    #[allow(unused_imports)]
    use super::*;

    #[cfg(not(feature = "interpreter"))]
    #[test]
    fn test_execute_returns_error_without_feature() {
        let interpreter = RuchyInterpreter::new();
        let result = interpreter.execute("1 + 1");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Interpreter feature not enabled"));
    }

    #[cfg(not(feature = "interpreter"))]
    #[test]
    fn test_validate_syntax_returns_false_without_feature() {
        let interpreter = RuchyInterpreter::new();
        assert!(!interpreter.validate_syntax("print('hello')"));
        assert!(!interpreter.validate_syntax("invalid syntax"));
    }

    #[cfg(not(feature = "interpreter"))]
    #[test]
    fn test_start_repl_returns_error_without_feature() {
        let interpreter = RuchyInterpreter::new();
        let result = interpreter.start_repl();
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("REPL not available"));
    }
}

mod benchmark_results {
    use depyler_ruchy::interpreter::BenchmarkResults;

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
    fn test_benchmark_results_debug() {
        let results = BenchmarkResults {
            iterations: 50,
            total_time_ms: 250,
            avg_time_ms: 5.0,
            throughput_ops_per_sec: 200,
        };

        let debug = format!("{:?}", results);
        assert!(debug.contains("BenchmarkResults"));
        assert!(debug.contains("iterations: 50"));
    }

    #[test]
    fn test_benchmark_results_clone() {
        let results = BenchmarkResults {
            iterations: 10,
            total_time_ms: 100,
            avg_time_ms: 10.0,
            throughput_ops_per_sec: 100,
        };

        let cloned = results.clone();
        assert_eq!(results.iterations, cloned.iterations);
        assert_eq!(results.total_time_ms, cloned.total_time_ms);
        assert!((results.avg_time_ms - cloned.avg_time_ms).abs() < f64::EPSILON);
        assert_eq!(
            results.throughput_ops_per_sec,
            cloned.throughput_ops_per_sec
        );
    }

    #[test]
    fn test_benchmark_results_zero_values() {
        let results = BenchmarkResults {
            iterations: 0,
            total_time_ms: 0,
            avg_time_ms: 0.0,
            throughput_ops_per_sec: 0,
        };

        let display = format!("{}", results);
        assert!(display.contains("Iterations: 0"));
        assert!(display.contains("Total time: 0ms"));
    }

    #[test]
    fn test_benchmark_results_large_values() {
        let results = BenchmarkResults {
            iterations: 1_000_000,
            total_time_ms: 10_000_000,
            avg_time_ms: 10.0,
            throughput_ops_per_sec: 100_000,
        };

        let display = format!("{}", results);
        assert!(display.contains("1000000"));
        assert!(display.contains("100000"));
    }

    #[test]
    fn test_benchmark_results_fractional_avg() {
        let results = BenchmarkResults {
            iterations: 1000,
            total_time_ms: 1234,
            avg_time_ms: 1.234,
            throughput_ops_per_sec: 810,
        };

        let display = format!("{}", results);
        assert!(display.contains("1.23")); // Format uses 2 decimal places
    }
}

mod ruchy_config {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = RuchyConfig::default();
        // Config should have sensible defaults
        assert!(!config.use_pipelines);
    }

    #[cfg(feature = "interpreter")]
    #[test]
    fn test_config_with_mcp_enabled() {
        let config = RuchyConfig {
            enable_mcp: true,
            ..Default::default()
        };
        assert!(config.enable_mcp);
    }

    #[test]
    fn test_config_clone() {
        let config = RuchyConfig::default();
        let cloned = config.clone();
        assert_eq!(config.use_pipelines, cloned.use_pipelines);
    }

    #[test]
    fn test_config_debug() {
        let config = RuchyConfig::default();
        let debug = format!("{:?}", config);
        assert!(debug.contains("RuchyConfig"));
    }
}

mod integration_scenarios {
    use super::*;

    #[test]
    fn test_interpreter_lifecycle() {
        // Create
        let mut interpreter = RuchyInterpreter::new();

        // Use context
        interpreter.set_context("state".to_string(), "initialized".to_string());
        assert_eq!(
            interpreter.get_context("state"),
            Some(&"initialized".to_string())
        );

        // Modify context
        interpreter.set_context("state".to_string(), "running".to_string());
        assert_eq!(
            interpreter.get_context("state"),
            Some(&"running".to_string())
        );

        // Add more context
        interpreter.set_context("counter".to_string(), "42".to_string());

        // Clear
        interpreter.clear_context();
        assert!(interpreter.get_context("state").is_none());
        assert!(interpreter.get_context("counter").is_none());
    }

    #[test]
    fn test_multiple_interpreters_isolated() {
        let mut interp1 = RuchyInterpreter::new();
        let mut interp2 = RuchyInterpreter::new();

        interp1.set_context("key".to_string(), "value1".to_string());
        interp2.set_context("key".to_string(), "value2".to_string());

        assert_eq!(interp1.get_context("key"), Some(&"value1".to_string()));
        assert_eq!(interp2.get_context("key"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_interpreter_with_various_configs() {
        let configs = vec![
            RuchyConfig::default(),
            RuchyConfig {
                use_pipelines: true,
                ..Default::default()
            },
            RuchyConfig {
                use_actors: true,
                ..Default::default()
            },
        ];

        for config in configs {
            let interpreter = RuchyInterpreter::with_config(&config);
            // All should have empty initial context
            assert!(interpreter.get_context("test").is_none());
        }
    }
}

mod property_tests {
    use super::*;
    use depyler_ruchy::interpreter::BenchmarkResults;
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_context_set_get_roundtrip(key in "[a-z]{1,20}", value in "[a-z0-9]{1,50}") {
            let mut interpreter = RuchyInterpreter::new();
            interpreter.set_context(key.clone(), value.clone());
            prop_assert_eq!(interpreter.get_context(&key), Some(&value));
        }

        #[test]
        fn prop_clear_removes_all(keys in proptest::collection::vec("[a-z]{1,10}", 1..10)) {
            let mut interpreter = RuchyInterpreter::new();
            for key in &keys {
                interpreter.set_context(key.clone(), "value".to_string());
            }
            interpreter.clear_context();
            for key in &keys {
                prop_assert!(interpreter.get_context(key).is_none());
            }
        }

        #[test]
        fn prop_overwrite_replaces_value(key in "[a-z]{1,10}", v1 in "[0-9]+", v2 in "[a-z]+") {
            let mut interpreter = RuchyInterpreter::new();
            interpreter.set_context(key.clone(), v1);
            interpreter.set_context(key.clone(), v2.clone());
            prop_assert_eq!(interpreter.get_context(&key), Some(&v2));
        }

        #[test]
        fn prop_benchmark_results_display_contains_values(
            iterations in 1u64..10000,
            total_time_ms in 1u128..100000,
        ) {
            let avg = total_time_ms as f64 / iterations as f64;
            let throughput = if total_time_ms > 0 {
                (iterations as f64 / (total_time_ms as f64 / 1000.0)) as u64
            } else {
                0
            };

            let results = BenchmarkResults {
                iterations: iterations as usize,
                total_time_ms,
                avg_time_ms: avg,
                throughput_ops_per_sec: throughput,
            };

            let display = format!("{}", results);
            prop_assert!(display.contains(&iterations.to_string()));
        }
    }
}
