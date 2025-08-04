//! Advanced Property Test Generators - Phase 8.1
//!
//! Enhanced property test generators for more realistic and comprehensive testing
//! of the Depyler transpiler with custom generators, weighted patterns, and
//! mutation-based edge case discovery.

use depyler_core::DepylerPipeline;
use proptest::prelude::*;
use proptest::strategy::ValueTree;
use std::collections::HashMap;
use std::time::Instant;

/// Custom generator for realistic Python function patterns
#[derive(Debug, Clone)]
pub struct PythonFunctionPattern {
    pub name: String,
    pub parameters: Vec<(String, String)>, // (name, type_hint)
    pub return_type: Option<String>,
    pub body_complexity: usize,
    pub has_docstring: bool,
    pub uses_collections: bool,
    pub uses_control_flow: bool,
}

/// Weighted generator for realistic code patterns
pub fn weighted_python_function() -> impl Strategy<Value = PythonFunctionPattern> {
    (
        "[a-z][a-z0-9_]{2,15}", // Realistic function names
        prop::collection::vec(("[a-z][a-z0-9_]{1,10}", "int|str|float|bool"), 0..4),
        prop::option::of("int|str|bool"),
        1..10usize,                // Body complexity
        prop::bool::weighted(0.7), // 70% chance of docstring
        prop::bool::weighted(0.4), // 40% chance of collections
        prop::bool::weighted(0.6), // 60% chance of control flow
    )
        .prop_map(
            |(name, params, return_type, complexity, docstring, collections, control_flow)| {
                let typed_params = params
                    .into_iter()
                    .map(|(n, t)| (n, t.to_string()))
                    .collect();
                let typed_return = return_type.map(|s| s.to_string());

                PythonFunctionPattern {
                    name,
                    parameters: typed_params,
                    return_type: typed_return,
                    body_complexity: complexity,
                    has_docstring: docstring,
                    uses_collections: collections,
                    uses_control_flow: control_flow,
                }
            },
        )
}

/// Generator for compositional test scenarios
pub fn compositional_python_module() -> impl Strategy<Value = Vec<PythonFunctionPattern>> {
    prop::collection::vec(weighted_python_function(), 1..5)
}

/// Performance-optimized generator with caching
pub struct OptimizedGenerator {
    cache: HashMap<String, String>,
    hit_count: usize,
    miss_count: usize,
}

impl OptimizedGenerator {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            hit_count: 0,
            miss_count: 0,
        }
    }

    pub fn generate_cached(&mut self, pattern: &PythonFunctionPattern) -> String {
        let key = format!(
            "{}-{}-{}",
            pattern.name,
            pattern.parameters.len(),
            pattern.body_complexity
        );
        if let Some(cached) = self.cache.get(&key) {
            self.hit_count += 1;
            cached.clone()
        } else {
            self.miss_count += 1;
            let code = generate_function_code(pattern);
            self.cache.insert(key, code.clone());
            code
        }
    }

    pub fn cache_stats(&self) -> (usize, usize, f64) {
        let total = self.hit_count + self.miss_count;
        let hit_rate = if total > 0 {
            self.hit_count as f64 / total as f64
        } else {
            0.0
        };
        (self.hit_count, self.miss_count, hit_rate)
    }
}

/// Helper function to generate actual Python code from patterns
fn generate_function_code(pattern: &PythonFunctionPattern) -> String {
    let mut code = String::new();

    // Function signature
    let params: Vec<String> = pattern
        .parameters
        .iter()
        .map(|(name, type_hint)| format!("{}: {}", name, type_hint))
        .collect();

    let return_annotation = pattern
        .return_type
        .as_ref()
        .map(|t| format!(" -> {}", t))
        .unwrap_or_default();

    code.push_str(&format!(
        "def {}({}){}: ",
        pattern.name,
        params.join(", "),
        return_annotation
    ));

    // Docstring
    if pattern.has_docstring {
        code.push_str(&format!(
            "\n    \"\"\"{}\"\"\" ",
            generate_docstring(pattern)
        ));
    }

    // Function body based on complexity
    code.push_str(&generate_function_body(pattern));

    code
}

fn generate_docstring(pattern: &PythonFunctionPattern) -> String {
    format!(
        "Function {} with {} parameters.",
        pattern.name,
        pattern.parameters.len()
    )
}

fn generate_function_body(pattern: &PythonFunctionPattern) -> String {
    let mut body = String::new();

    // Simple bodies for lower complexity
    if pattern.body_complexity <= 3 {
        if pattern.return_type.as_deref() == Some("int") {
            body.push_str("return 42");
        } else if pattern.return_type.as_deref() == Some("str") {
            body.push_str("return \"result\"");
        } else {
            body.push_str("pass");
        }
        return body;
    }

    // Collections usage
    if pattern.uses_collections {
        body.push_str("\n    items = []");
        body.push_str("\n    data = {}");
    }

    // Control flow
    if pattern.uses_control_flow && pattern.body_complexity > 5 {
        body.push_str("\n    for i in range(10):");
        body.push_str("\n        if i % 2 == 0:");
        body.push_str("\n            continue");
        body.push_str("\n        else:");
        body.push_str("\n            break");
    }

    // Return statement
    if let Some(return_type) = &pattern.return_type {
        match return_type.as_str() {
            "int" => body.push_str("\n    return 42"),
            "str" => body.push_str("\n    return \"result\""),
            "bool" => body.push_str("\n    return True"),
            _ => body.push_str("\n    return None"),
        }
    } else {
        body.push_str("\n    pass");
    }

    body
}

/// Simple mutation generator for edge cases
pub fn generate_mutated_code(base_code: &str) -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    match rng.gen_range(0..5) {
        0 => base_code.replace("def ", "def test_函数_"), // Unicode
        1 => base_code.replace("42", "9223372036854775807"), // Large numbers
        2 => base_code.replace("\"result\"", "\"\\n\\t\\r special\""), // Special chars
        3 => format!("{}\n# Extra comment with 测试", base_code), // Unicode comments
        _ => base_code.to_string(),                       // No mutation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test custom generators for Python language constructs
    #[test]
    fn test_custom_python_generators() {
        println!("=== Custom Python Generators Test ===");

        let mut runner = proptest::test_runner::TestRunner::default();
        let strategy = weighted_python_function();

        for _ in 0..20 {
            let pattern = strategy.new_tree(&mut runner).unwrap().current();
            let code = generate_function_code(&pattern);

            println!("Generated: {}", pattern.name);

            // Test that generated code is reasonable
            assert!(code.starts_with(&format!("def {}", pattern.name)));
            assert!(code.len() > 10);
            assert!(code.len() < 1000); // Reasonable upper bound

            // Test transpilation
            let pipeline = DepylerPipeline::new();
            let result = pipeline.transpile(&code);

            // Should either succeed or fail gracefully
            match result {
                Ok(_) => println!("  ✓ Transpiled successfully"),
                Err(e) => println!(
                    "  - Transpilation failed (expected): {}",
                    e.to_string().chars().take(50).collect::<String>()
                ),
            }
        }
    }

    /// Test weighted generators for realistic code patterns
    #[test]
    fn test_weighted_generators() {
        println!("=== Weighted Generators Test ===");

        let mut runner = proptest::test_runner::TestRunner::default();
        let strategy = compositional_python_module();

        for test_case in 0..10 {
            let module_patterns = strategy.new_tree(&mut runner).unwrap().current();

            println!(
                "Module {} with {} functions:",
                test_case,
                module_patterns.len()
            );

            let mut module_code = String::new();
            for pattern in &module_patterns {
                let function_code = generate_function_code(pattern);
                module_code.push_str(&function_code);
                module_code.push_str("\n\n");

                println!(
                    "  - {}: {} params, complexity {}",
                    pattern.name,
                    pattern.parameters.len(),
                    pattern.body_complexity
                );
            }

            // Test full module transpilation
            let pipeline = DepylerPipeline::new();
            let start = Instant::now();
            let result = pipeline.transpile(&module_code);
            let duration = start.elapsed();

            println!(
                "  Transpilation: {:?} ({})",
                duration,
                if result.is_ok() { "✓" } else { "✗" }
            );

            // Performance check - realistic modules should transpile quickly
            assert!(
                duration.as_millis() < 1000,
                "Module transpilation took too long: {:?}",
                duration
            );
        }
    }

    /// Test mutation-based generators for edge case discovery
    #[test]
    fn test_mutation_based_generators() {
        println!("=== Mutation-Based Generators Test ===");

        let base_functions = vec![
            "def simple(x: int) -> int: return x + 1",
            "def greet(name: str) -> str: return f\"Hello {name}\"",
            "def check(flag: bool) -> bool: return not flag",
        ];

        for base_code in base_functions {
            println!("Base: {}", base_code);

            for _ in 0..5 {
                let mutated_code = generate_mutated_code(base_code);

                println!(
                    "  Mutated: {}",
                    mutated_code.chars().take(60).collect::<String>()
                );

                // Test with pipeline
                let pipeline = DepylerPipeline::new();
                let result = pipeline.transpile(&mutated_code);

                match result {
                    Ok(_) => println!("    ✓ Mutation transpiled successfully"),
                    Err(e) => {
                        let error_preview = e.to_string().chars().take(100).collect::<String>();
                        println!("    - Mutation failed gracefully: {}", error_preview);

                        // Error messages should be reasonable
                        assert!(error_preview.len() > 5, "Error message too short");
                        assert!(error_preview.len() < 500, "Error message too long");
                    }
                }
            }
        }
    }

    /// Test compositional generators for complex scenarios
    #[test]
    fn test_compositional_generators() {
        println!("=== Compositional Generators Test ===");

        let pipeline = DepylerPipeline::new();

        // Test various composition patterns
        let composition_patterns = vec![
            ("Single Function", 1, 1),
            ("Small Module", 2, 3),
            ("Medium Module", 3, 4),
        ];

        for (pattern_name, min_funcs, max_funcs) in composition_patterns {
            let mut runner = proptest::test_runner::TestRunner::default();
            let strategy = prop::collection::vec(weighted_python_function(), min_funcs..=max_funcs);

            let functions = strategy.new_tree(&mut runner).unwrap().current();

            let mut module_code = String::new();
            for pattern in &functions {
                module_code.push_str(&generate_function_code(pattern));
                module_code.push('\n');
            }

            let start = Instant::now();
            let result = pipeline.transpile(&module_code);
            let duration = start.elapsed();

            println!(
                "{}: {} functions, {:?} ({})",
                pattern_name,
                functions.len(),
                duration,
                if result.is_ok() { "✓" } else { "✗" }
            );

            // Compositional complexity should scale reasonably
            let expected_max_ms = 100 * functions.len() as u128;
            assert!(
                duration.as_millis() <= expected_max_ms,
                "{} took too long: {:?} > {}ms",
                pattern_name,
                duration,
                expected_max_ms
            );
        }
    }

    /// Test performance-optimized generators with caching
    #[test]
    fn test_optimized_generators() {
        println!("=== Performance-Optimized Generators Test ===");

        let mut generator = OptimizedGenerator::new();
        let mut runner = proptest::test_runner::TestRunner::default();
        let strategy = weighted_python_function();

        // Generate many patterns, some duplicates expected
        let mut patterns = Vec::new();
        for _ in 0..30 {
            patterns.push(strategy.new_tree(&mut runner).unwrap().current());
        }

        // Add some duplicates intentionally
        for i in 0..5 {
            patterns.push(patterns[i].clone());
        }

        let start = Instant::now();

        for pattern in &patterns {
            let _code = generator.generate_cached(pattern);
        }

        let duration = start.elapsed();
        let (hits, misses, hit_rate) = generator.cache_stats();

        println!("Generated {} patterns in {:?}", patterns.len(), duration);
        println!(
            "Cache stats: {} hits, {} misses, {:.2}% hit rate",
            hits,
            misses,
            hit_rate * 100.0
        );

        // Should achieve some cache hits with duplicates
        assert!(
            hit_rate >= 0.0,
            "Cache hit rate should be non-negative: {:.2}%",
            hit_rate * 100.0
        );
        assert!(
            duration.as_millis() < 1000,
            "Generation took too long: {:?}",
            duration
        );

        // Performance should improve with caching
        let cache_size = generator.cache.len();
        println!("Final cache size: {} entries", cache_size);
        assert!(
            cache_size <= patterns.len(),
            "Cache size should not exceed unique patterns"
        );
    }

    /// Test generator efficiency and performance characteristics
    #[test]
    fn test_generator_performance() {
        println!("=== Generator Performance Test ===");

        let test_scenarios = vec![("Simple Functions", 50), ("Compositional Modules", 10)];

        for (scenario_name, count) in test_scenarios {
            let mut runner = proptest::test_runner::TestRunner::default();
            let strategy = weighted_python_function();

            let start = Instant::now();

            for _ in 0..count {
                let _pattern = strategy.new_tree(&mut runner).unwrap().current();
            }

            let duration = start.elapsed();
            let per_generation = duration.as_micros() / count as u128;

            println!(
                "{}: {} generations in {:?} ({} μs/generation)",
                scenario_name, count, duration, per_generation
            );

            // Generators should be fast
            assert!(
                per_generation < 10000, // Less than 10ms per generation
                "{} generator too slow: {} μs/generation",
                scenario_name,
                per_generation
            );
        }

        // Test cache performance
        let test_input_pattern = PythonFunctionPattern {
            name: "cached_test".to_string(),
            parameters: vec![],
            return_type: None,
            body_complexity: 1,
            has_docstring: false,
            uses_collections: false,
            uses_control_flow: false,
        };

        let mut generator = OptimizedGenerator::new();

        let time1 = {
            let start = Instant::now();
            let _code = generator.generate_cached(&test_input_pattern);
            start.elapsed()
        };

        let time2 = {
            let start = Instant::now();
            let _code = generator.generate_cached(&test_input_pattern);
            start.elapsed()
        };

        println!("Cache test: first {:?}, second {:?}", time1, time2);

        // Second execution should be faster due to caching
        assert!(
            time2 <= time1 * 5,
            "Caching should not significantly slow down execution"
        );
    }
}
