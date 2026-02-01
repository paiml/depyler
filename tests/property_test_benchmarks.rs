use depyler_core::DepylerPipeline;
use std::time::{Duration, Instant};

#[cfg(test)]
mod property_test_benchmarks {
    use super::*;

    /// Benchmark property test execution times
    #[test]
    fn benchmark_property_test_execution() {
        let pipeline = DepylerPipeline::new();

        // Test simple transpilation performance
        let simple_test_cases = [
            "def add(a: int, b: int) -> int: return a + b",
            "def multiply(x: int, y: int) -> int: return x * y",
            "def factorial(n: int) -> int: return 1 if n <= 1 else n * factorial(n - 1)",
            "def fibonacci(n: int) -> int: return n if n <= 1 else fibonacci(n-1) + fibonacci(n-2)",
        ];

        println!("=== Property Test Performance Benchmarks ===");

        for (i, test_case) in simple_test_cases.iter().enumerate() {
            let start = Instant::now();
            let result = pipeline.transpile(test_case);
            let duration = start.elapsed();

            println!(
                "Test Case {}: {:?} ({})",
                i + 1,
                duration,
                if result.is_ok() { "✓" } else { "✗" }
            );

            // Performance regression check - should complete within reasonable time
            // Threshold set to 3s to avoid flaky failures under system load
            assert!(
                duration < Duration::from_secs(3),
                "Transpilation should complete within 3 seconds, took {:?}",
                duration
            );
        }
    }

    /// Benchmark HIR parsing performance
    #[test]
    fn benchmark_hir_parsing_performance() {
        let pipeline = DepylerPipeline::new();

        let parsing_test_cases = [
            // Simple function
            "def simple() -> int: return 42",
            // Function with control flow
            r#"
def with_control_flow(x: int) -> int:
    if x > 0:
        return x * 2
    else:
        return 0
"#,
            // Function with loop
            r#"
def with_loop(n: int) -> int:
    total = 0
    for i in range(n):
        total += i
    return total
"#,
            // Multiple functions
            r#"
def func1(x: int) -> int:
    return x + 1

def func2(y: int) -> int:
    return func1(y) * 2

def func3(z: int) -> int:
    return func2(z) - 1
"#,
        ];

        println!("=== HIR Parsing Performance Benchmarks ===");

        for (i, test_case) in parsing_test_cases.iter().enumerate() {
            let start = Instant::now();
            let result = pipeline.parse_to_hir(test_case);
            let duration = start.elapsed();

            println!(
                "HIR Parse {}: {:?} ({})",
                i + 1,
                duration,
                if result.is_ok() { "✓" } else { "✗" }
            );

            // HIR parsing should be very fast
            assert!(
                duration < Duration::from_millis(100),
                "HIR parsing should complete within 100ms, took {:?}",
                duration
            );
        }
    }

    /// Benchmark property test generator performance
    #[test]
    #[ignore = "Causes stack overflow with large generated values - needs generator size limiting"]
    fn benchmark_property_generators() {
        use quickcheck::{quickcheck, TestResult};

        println!("=== Property Generator Performance ===");

        // Benchmark simple property test
        let start = Instant::now();
        quickcheck(|x: i32, y: i32| -> TestResult {
            let pipeline = DepylerPipeline::new();
            let test_code = format!(
                "def test(a: int, b: int) -> int: return {} + {}",
                x.abs() % 1000,
                y.abs() % 1000
            );

            match pipeline.transpile(&test_code) {
                Ok(_) => TestResult::passed(),
                Err(_) => TestResult::discard(),
            }
        } as fn(i32, i32) -> TestResult);
        let generator_duration = start.elapsed();

        println!("Property Generator: {:?}", generator_duration);

        // Property tests should complete within reasonable time
        assert!(
            generator_duration < Duration::from_secs(30),
            "Property generators should complete within 30 seconds, took {:?}",
            generator_duration
        );
    }

    /// Test memory usage patterns during property testing
    #[test]
    fn test_memory_usage_patterns() {
        let pipeline = DepylerPipeline::new();

        println!("=== Memory Usage Pattern Analysis ===");

        // Test increasing complexity to observe memory patterns
        let complexity_levels = vec![
            ("Simple", "def f(x: int) -> int: return x"),
            (
                "Medium",
                r#"
def medium_complexity(x: int, y: int) -> int:
    if x > y:
        return x + y
    else:
        return x * y
"#,
            ),
            (
                "Complex",
                r#"
def complex_function(n: int) -> int:
    total = 0
    for i in range(n):
        if i % 2 == 0:
            total += i * 2
        else:
            total -= i // 2
    return total
"#,
            ),
        ];

        for (name, code) in complexity_levels {
            let start = Instant::now();
            let result = pipeline.transpile(code);
            let duration = start.elapsed();

            println!(
                "{} Complexity: {:?} ({})",
                name,
                duration,
                if result.is_ok() { "✓" } else { "✗" }
            );
        }
    }

    /// Benchmark parallel property test execution
    #[test]
    fn benchmark_parallel_execution() {
        use std::sync::Arc;
        use std::thread;

        println!("=== Parallel Execution Benchmark ===");

        let pipeline = Arc::new(DepylerPipeline::new());
        let test_cases = vec![
            "def test1(x: int) -> int: return x + 1",
            "def test2(x: int) -> int: return x * 2",
            "def test3(x: int) -> int: return x - 1",
            "def test4(x: int) -> int: return x // 2",
        ];

        // Sequential execution
        let start = Instant::now();
        for test_case in &test_cases {
            let _ = pipeline.transpile(test_case);
        }
        let sequential_duration = start.elapsed();

        // Parallel execution
        let start = Instant::now();
        let handles: Vec<_> = test_cases
            .iter()
            .map(|&test_case| {
                let pipeline_clone = Arc::clone(&pipeline);
                thread::spawn(move || pipeline_clone.transpile(test_case))
            })
            .collect();

        for handle in handles {
            let _ = handle.join();
        }
        let parallel_duration = start.elapsed();

        println!("Sequential: {:?}", sequential_duration);
        println!("Parallel: {:?}", parallel_duration);

        // Parallel should be at least as fast (or close due to overhead)
        println!(
            "Speedup ratio: {:.2}x",
            sequential_duration.as_nanos() as f64 / parallel_duration.as_nanos() as f64
        );
    }

    /// Performance regression test - ensure no performance degradation
    #[test]
    #[ignore = "Timing-sensitive test - varies significantly with system load (67-78ms observed)"]
    fn performance_regression_test() {
        let pipeline = DepylerPipeline::new();

        println!("=== Performance Regression Tests ===");

        let regression_test_cases = vec![
            ("Basic Function", "def f(x: int) -> int: return x", Duration::from_millis(50)),
            ("With Control Flow", "def f(x: int) -> int: return x if x > 0 else 0", Duration::from_millis(100)),
            ("With Loop", "def f(n: int) -> int:\n    total = 0\n    for i in range(n):\n        total += i\n    return total", Duration::from_millis(150)),
        ];

        for (name, code, max_duration) in regression_test_cases {
            let start = Instant::now();
            let result = pipeline.transpile(code);
            let duration = start.elapsed();

            println!(
                "{}: {:?} (max: {:?}) {}",
                name,
                duration,
                max_duration,
                if result.is_ok() { "✓" } else { "✗" }
            );

            assert!(
                duration <= max_duration,
                "{} took {:?}, exceeded maximum {:?}",
                name,
                duration,
                max_duration
            );
        }
    }

    /// Test scalability with increasing input sizes
    #[test]
    #[ignore] // Timing-sensitive benchmark - flaky in CI environments
    fn test_scalability_patterns() {
        let pipeline = DepylerPipeline::new();

        println!("=== Scalability Pattern Analysis ===");

        // Test with increasing parameter counts
        let param_counts = vec![1, 2, 5, 10];

        for param_count in param_counts {
            let params: Vec<String> = (0..param_count).map(|i| format!("x{}: int", i)).collect();
            let args: Vec<String> = (0..param_count).map(|i| format!("x{}", i)).collect();

            let test_code = format!(
                "def test_func({}) -> int: return {}",
                params.join(", "),
                if args.is_empty() {
                    "0".to_string()
                } else {
                    args.join(" + ")
                }
            );

            let start = Instant::now();
            let result = pipeline.transpile(&test_code);
            let duration = start.elapsed();

            println!(
                "{} params: {:?} ({})",
                param_count,
                duration,
                if result.is_ok() { "✓" } else { "✗" }
            );

            // Should scale linearly or better
            assert!(
                duration < Duration::from_millis(200),
                "Scalability test with {} params should complete quickly, took {:?}",
                param_count,
                duration
            );
        }
    }
}
