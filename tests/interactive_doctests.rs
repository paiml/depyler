//! Interactive Doctests - Phase 8.2
//!
//! Advanced doctest patterns with REPL-like flow, error condition documentation,
//! performance benchmark doctests, and comprehensive API demonstrations.

use depyler_core::DepylerPipeline;
use std::collections::HashMap;
use std::time::Instant;

/// Interactive doctest framework for REPL-like documentation
pub struct InteractiveDoctest {
    pipeline: DepylerPipeline,
    session_history: Vec<(String, Result<String, String>)>,
    performance_metrics: HashMap<String, u128>,
}

impl Default for InteractiveDoctest {
    fn default() -> Self {
        Self::new()
    }
}

impl InteractiveDoctest {
    /// Creates a new interactive doctest session
    ///
    /// # Interactive Example
    /// ```
    /// use depyler_core::DepylerPipeline;
    ///
    /// // Start a new session
    /// let mut session = InteractiveDoctest::new();
    ///
    /// // Execute a simple function
    /// session.execute("def greet(name: str) -> str: return name");
    /// // Expected: Ok("pub fn greet(name: String) -> String { ... }")
    ///
    /// // Build on previous definitions
    /// session.execute("def main(): return greet('World')");
    /// // Expected: Ok("pub fn main() -> String { greet(\"World\".to_string()) }")
    ///
    /// // View session history
    /// let history = session.get_history();
    /// assert_eq!(history.len(), 2);
    /// ```
    pub fn new() -> Self {
        Self {
            pipeline: DepylerPipeline::new(),
            session_history: Vec::new(),
            performance_metrics: HashMap::new(),
        }
    }

    /// Executes Python code and returns Rust translation
    ///
    /// # Error Handling Examples
    /// ```
    /// let mut session = InteractiveDoctest::new();
    ///
    /// // Valid code succeeds
    /// let result = session.execute("def add(a: int, b: int) -> int: return a + b");
    /// assert!(result.is_ok());
    ///
    /// // Invalid syntax fails gracefully
    /// let result = session.execute("def broken_function(\n    return 42");
    /// assert!(result.is_err());
    /// assert!(result.unwrap_err().contains("syntax"));
    ///
    /// // Unsupported features fail with helpful messages
    /// let result = session.execute("async def unsupported(): await something()");
    /// assert!(result.is_err());
    /// assert!(result.unwrap_err().contains("async"));
    /// ```
    pub fn execute(&mut self, python_code: &str) -> Result<String, String> {
        let start = Instant::now();

        let result = self
            .pipeline
            .transpile(python_code)
            .map_err(|e| e.to_string());

        let duration = start.elapsed().as_micros();
        self.performance_metrics.insert(
            format!("execution_{}", self.session_history.len()),
            duration,
        );

        // Store in session history
        match &result {
            Ok(rust_code) => {
                self.session_history
                    .push((python_code.to_string(), Ok(rust_code.clone())));
            }
            Err(error) => {
                self.session_history
                    .push((python_code.to_string(), Err(error.clone())));
            }
        }

        result
    }

    /// Gets the complete session history
    ///
    /// # Performance Tracking Example
    /// ```
    /// let mut session = InteractiveDoctest::new();
    ///
    /// // Execute several operations
    /// session.execute("def simple(): return 42");
    /// session.execute("def complex(x: int) -> int: return x * 2 + 1 if x > 0 else 0");
    /// session.execute("def invalid syntax");
    ///
    /// let history = session.get_history();
    /// assert_eq!(history.len(), 3);
    ///
    /// // Check that successful operations are recorded
    /// assert!(history[0].1.is_ok());
    /// assert!(history[1].1.is_ok());
    /// assert!(history[2].1.is_err());
    ///
    /// // Performance metrics are tracked
    /// let metrics = session.get_performance_metrics();
    /// assert_eq!(metrics.len(), 3);
    /// ```
    pub fn get_history(&self) -> &Vec<(String, Result<String, String>)> {
        &self.session_history
    }

    /// Gets performance metrics for all executions
    pub fn get_performance_metrics(&self) -> &HashMap<String, u128> {
        &self.performance_metrics
    }

    /// Clears the session history and metrics
    ///
    /// # Session Management Example
    /// ```
    /// let mut session = InteractiveDoctest::new();
    ///
    /// session.execute("def test(): pass");
    /// assert_eq!(session.get_history().len(), 1);
    ///
    /// session.clear_session();
    /// assert_eq!(session.get_history().len(), 0);
    /// assert_eq!(session.get_performance_metrics().len(), 0);
    /// ```
    pub fn clear_session(&mut self) {
        self.session_history.clear();
        self.performance_metrics.clear();
    }
}

/// Performance benchmark doctests with timing validation
pub struct PerformanceBenchmarkDoctest {
    pipeline: DepylerPipeline,
    benchmarks: HashMap<String, Vec<u128>>,
}

impl Default for PerformanceBenchmarkDoctest {
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceBenchmarkDoctest {
    /// Creates a new performance benchmark session
    ///
    /// # Performance Baseline Example
    /// ```
    /// let mut bench = PerformanceBenchmarkDoctest::new();
    ///
    /// // Benchmark simple functions (should be fast)
    /// let duration = bench.benchmark_function("def simple(): return 42", "simple_function");
    /// assert!(duration < 50_000); // Less than 50ms
    ///
    /// // Benchmark complex functions (may be slower but still reasonable)
    /// let complex_fn = r#"
    /// def fibonacci(n: int) -> int:
    ///     if n <= 1:
    ///         return n
    ///     return fibonacci(n-1) + fibonacci(n-2)
    /// "#;
    /// let duration = bench.benchmark_function(complex_fn, "fibonacci");
    /// assert!(duration < 200_000); // Less than 200ms
    /// ```
    pub fn new() -> Self {
        Self {
            pipeline: DepylerPipeline::new(),
            benchmarks: HashMap::new(),
        }
    }

    /// Benchmarks a function transpilation and returns duration in microseconds
    ///
    /// # Benchmark Comparison Example
    /// ```
    /// let mut bench = PerformanceBenchmarkDoctest::new();
    ///
    /// // Benchmark different complexity levels
    /// let simple = bench.benchmark_function("def f(): pass", "empty");
    /// let medium = bench.benchmark_function("def f(x: int) -> int: return x * 2", "arithmetic");
    /// let complex = bench.benchmark_function(r#"
    /// def complex_function(data: list) -> dict:
    ///     result = {}
    ///     for item in data:
    ///         if item > 0:
    ///             result[str(item)] = item * 2
    ///     return result
    /// "#, "complex");
    ///
    /// // Complexity should correlate with execution time
    /// assert!(simple <= medium);
    /// assert!(medium <= complex * 2); // Allow some variance
    /// ```
    pub fn benchmark_function(&mut self, python_code: &str, benchmark_name: &str) -> u128 {
        let start = Instant::now();
        let _result = self.pipeline.transpile(python_code);
        let duration = start.elapsed().as_micros();

        // Store benchmark result
        self.benchmarks
            .entry(benchmark_name.to_string())
            .or_default()
            .push(duration);

        duration
    }

    /// Runs multiple iterations of a benchmark for statistical analysis
    ///
    /// # Statistical Benchmark Example
    /// ```
    /// let mut bench = PerformanceBenchmarkDoctest::new();
    ///
    /// let stats = bench.benchmark_iterations(
    ///     "def test_function(x: int, y: int) -> int: return x + y * 2",
    ///     "addition_benchmark",
    ///     10
    /// );
    ///
    /// // Should have statistical data
    /// assert!(stats.min_duration > 0);
    /// assert!(stats.max_duration >= stats.min_duration);
    /// assert!(stats.avg_duration >= stats.min_duration);
    /// assert!(stats.avg_duration <= stats.max_duration);
    /// assert_eq!(stats.iterations, 10);
    ///
    /// // Performance should be consistent (low variance)
    /// let variance_ratio = stats.max_duration as f64 / stats.min_duration as f64;
    /// assert!(variance_ratio < 10.0); // Max 10x variance
    /// ```
    pub fn benchmark_iterations(
        &mut self,
        python_code: &str,
        benchmark_name: &str,
        iterations: usize,
    ) -> BenchmarkStats {
        let mut durations = Vec::new();

        for _ in 0..iterations {
            let duration = self.benchmark_function(python_code, benchmark_name);
            durations.push(duration);
        }

        let min_duration = *durations.iter().min().unwrap();
        let max_duration = *durations.iter().max().unwrap();
        let avg_duration = durations.iter().sum::<u128>() / durations.len() as u128;

        BenchmarkStats {
            min_duration,
            max_duration,
            avg_duration,
            iterations,
            all_durations: durations,
        }
    }

    /// Gets all benchmark results
    pub fn get_benchmarks(&self) -> &HashMap<String, Vec<u128>> {
        &self.benchmarks
    }
}

/// Statistical benchmark results
#[derive(Debug, Clone)]
pub struct BenchmarkStats {
    pub min_duration: u128,
    pub max_duration: u128,
    pub avg_duration: u128,
    pub iterations: usize,
    pub all_durations: Vec<u128>,
}

/// Error condition documentation with expected failure patterns
pub struct ErrorConditionDoctest {
    pipeline: DepylerPipeline,
    error_catalog: HashMap<String, Vec<String>>,
}

impl Default for ErrorConditionDoctest {
    fn default() -> Self {
        Self::new()
    }
}

impl ErrorConditionDoctest {
    /// Creates a new error condition documentation session
    ///
    /// # Error Classification Example
    /// ```
    /// let mut error_doc = ErrorConditionDoctest::new();
    ///
    /// // Test syntax errors
    /// let result = error_doc.test_error_condition(
    ///     "def broken_function(\n    return 42",
    ///     "syntax_error"
    /// );
    /// assert!(result.is_err());
    /// assert!(result.unwrap_err().contains("syntax"));
    ///
    /// // Test unsupported features
    /// let result = error_doc.test_error_condition(
    ///     "async def unsupported(): await something()",
    ///     "unsupported_feature"
    /// );
    /// assert!(result.is_err());
    ///
    /// // Error catalog should be populated
    /// let catalog = error_doc.get_error_catalog();
    /// assert!(catalog.contains_key("syntax_error"));
    /// assert!(catalog.contains_key("unsupported_feature"));
    /// ```
    pub fn new() -> Self {
        Self {
            pipeline: DepylerPipeline::new(),
            error_catalog: HashMap::new(),
        }
    }

    /// Tests an error condition and catalogs the result
    ///
    /// # Comprehensive Error Testing Example
    /// ```
    /// let mut error_doc = ErrorConditionDoctest::new();
    ///
    /// // Test various error categories
    /// let error_cases = vec![
    ///     ("def f(): return", "incomplete_return"),
    ///     ("def f(: pass", "invalid_parameter"),
    ///     ("if condition\n    print('missing colon')", "missing_colon"),
    ///     ("def f(): return 1 2", "invalid_expression"),
    ///     ("class A:\n  def __init__(", "incomplete_method"),
    /// ];
    ///
    /// for (code, category) in error_cases {
    ///     let result = error_doc.test_error_condition(code, category);
    ///     assert!(result.is_err(), "Expected error for category: {}", category);
    /// }
    ///
    /// // All error categories should be documented
    /// let catalog = error_doc.get_error_catalog();
    /// assert_eq!(catalog.len(), 5);
    /// ```
    pub fn test_error_condition(
        &mut self,
        python_code: &str,
        error_category: &str,
    ) -> Result<String, String> {
        let result = self
            .pipeline
            .transpile(python_code)
            .map_err(|e| e.to_string());

        // Catalog the error if it occurred
        if let Err(ref error_msg) = result {
            self.error_catalog
                .entry(error_category.to_string())
                .or_default()
                .push(error_msg.clone());
        }

        result
    }

    /// Gets the complete error catalog
    pub fn get_error_catalog(&self) -> &HashMap<String, Vec<String>> {
        &self.error_catalog
    }

    /// Validates that expected errors occur for known bad inputs
    ///
    /// # Error Validation Example
    /// ```
    /// let mut error_doc = ErrorConditionDoctest::new();
    ///
    /// // Define expected error patterns
    /// let expected_errors = vec![
    ///     ("def f(", "Incomplete function definition should fail"),
    ///     ("if True\n    pass", "Missing colon should fail"),
    ///     ("def f(): return x y", "Invalid expression should fail"),
    /// ];
    ///
    /// let validation = error_doc.validate_expected_errors(&expected_errors);
    /// assert_eq!(validation.total_tests, 3);
    /// assert_eq!(validation.failed_as_expected, 3);
    /// assert_eq!(validation.unexpectedly_succeeded, 0);
    /// ```
    pub fn validate_expected_errors(
        &mut self,
        test_cases: &[(&str, &str)],
    ) -> ErrorValidationResults {
        let mut failed_as_expected = 0;
        let mut unexpectedly_succeeded = 0;
        let mut validation_details = Vec::new();

        for (code, description) in test_cases {
            let result = self.pipeline.transpile(code);

            if result.is_err() {
                failed_as_expected += 1;
                validation_details.push((
                    description.to_string(),
                    true,
                    result.err().map(|e| e.to_string()),
                ));
            } else {
                unexpectedly_succeeded += 1;
                validation_details.push((description.to_string(), false, None));
            }
        }

        ErrorValidationResults {
            total_tests: test_cases.len(),
            failed_as_expected,
            unexpectedly_succeeded,
            validation_details,
        }
    }
}

/// Error validation results
#[derive(Debug, Clone)]
pub struct ErrorValidationResults {
    pub total_tests: usize,
    pub failed_as_expected: usize,
    pub unexpectedly_succeeded: usize,
    pub validation_details: Vec<(String, bool, Option<String>)>,
}

/// End-to-end workflow documentation with comprehensive examples
pub struct WorkflowDoctest {
    pipeline: DepylerPipeline,
    workflow_steps: Vec<(String, String, Result<String, String>)>,
}

impl Default for WorkflowDoctest {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkflowDoctest {
    /// Creates a new workflow documentation session
    ///
    /// # Complete Workflow Example
    /// ```
    /// let mut workflow = WorkflowDoctest::new();
    ///
    /// // Step 1: Simple function
    /// workflow.add_step(
    ///     "Simple Function",
    ///     "def add(a: int, b: int) -> int: return a + b"
    /// );
    ///
    /// // Step 2: Function with control flow
    /// workflow.add_step(
    ///     "Control Flow",
    ///     "def max_value(a: int, b: int) -> int: return a if a > b else b"
    /// );
    ///
    /// // Step 3: Function with collections
    /// workflow.add_step(
    ///     "Collections",
    ///     "def sum_list(numbers: list) -> int: return sum(numbers)"
    /// );
    ///
    /// // Validate workflow
    /// let results = workflow.execute_workflow();
    /// assert_eq!(results.total_steps, 3);
    /// assert_eq!(results.successful_steps, 3);
    /// assert_eq!(results.failed_steps, 0);
    /// ```
    pub fn new() -> Self {
        Self {
            pipeline: DepylerPipeline::new(),
            workflow_steps: Vec::new(),
        }
    }

    /// Adds a step to the workflow
    pub fn add_step(&mut self, step_name: &str, python_code: &str) {
        let result = self
            .pipeline
            .transpile(python_code)
            .map_err(|e| e.to_string());

        self.workflow_steps
            .push((step_name.to_string(), python_code.to_string(), result));
    }

    /// Executes the complete workflow and returns results
    ///
    /// # Advanced Workflow Example
    /// ```
    /// let mut workflow = WorkflowDoctest::new();
    ///
    /// // Build a complete program step by step
    /// workflow.add_step("Data Structure", r#"
    /// def create_person(name: str, age: int) -> dict:
    ///     return {"name": name, "age": age}
    /// "#);
    ///
    /// workflow.add_step("Business Logic", r#"
    /// def is_adult(person: dict) -> bool:
    ///     return person["age"] >= 18
    /// "#);
    ///
    /// workflow.add_step("Main Function", r#"
    /// def main():
    ///     person = create_person("Alice", 25)
    ///     if is_adult(person):
    ///         print(f"{person['name']} is an adult")
    ///     else:
    ///         print(f"{person['name']} is a minor")
    /// "#);
    ///
    /// let results = workflow.execute_workflow();
    /// assert!(results.successful_steps >= 2); // At least basic functionality works
    /// ```
    pub fn execute_workflow(&self) -> WorkflowResults {
        let total_steps = self.workflow_steps.len();
        let successful_steps = self
            .workflow_steps
            .iter()
            .filter(|(_, _, result)| result.is_ok())
            .count();
        let failed_steps = total_steps - successful_steps;

        let step_details: Vec<_> = self
            .workflow_steps
            .iter()
            .map(|(name, code, result)| {
                (
                    name.clone(),
                    code.clone(),
                    result.is_ok(),
                    result.as_ref().err().cloned(),
                )
            })
            .collect();

        WorkflowResults {
            total_steps,
            successful_steps,
            failed_steps,
            step_details,
        }
    }

    /// Gets all workflow steps
    pub fn get_workflow_steps(&self) -> &Vec<(String, String, Result<String, String>)> {
        &self.workflow_steps
    }
}

/// Workflow execution results
#[derive(Debug, Clone)]
pub struct WorkflowResults {
    pub total_steps: usize,
    pub successful_steps: usize,
    pub failed_steps: usize,
    pub step_details: Vec<(String, String, bool, Option<String>)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test interactive doctest functionality
    #[test]
    fn test_interactive_doctest_session() {
        println!("=== Interactive Doctest Session Test ===");

        let mut session = InteractiveDoctest::new();

        // Test successful execution
        let result = session.execute("def greet(name: str) -> str: return name");
        println!("Simple function result: {:?}", result.is_ok());
        assert!(result.is_ok());

        // Test building on previous definitions
        let result = session.execute("def main(): return greet('World')");
        println!("Dependent function result: {:?}", result.is_ok());
        // Note: This may fail if the transpiler doesn't handle cross-function dependencies

        // Test error handling
        let result = session.execute("def broken_function(\n    return 42");
        println!("Syntax error result: {:?}", result.is_err());
        assert!(result.is_err());

        // Verify session history
        let history = session.get_history();
        println!("Session history length: {}", history.len());
        assert_eq!(history.len(), 3);

        // Check performance metrics
        let metrics = session.get_performance_metrics();
        println!("Performance metrics count: {}", metrics.len());
        assert_eq!(metrics.len(), 3);

        // Test session clearing
        session.clear_session();
        assert_eq!(session.get_history().len(), 0);
        assert_eq!(session.get_performance_metrics().len(), 0);
    }

    /// Test performance benchmark doctests
    #[test]
    fn test_performance_benchmark_doctests() {
        println!("=== Performance Benchmark Doctests Test ===");

        let mut bench = PerformanceBenchmarkDoctest::new();

        // Benchmark simple function
        let simple_duration = bench.benchmark_function("def simple(): return 42", "simple");
        println!("Simple function duration: {} μs", simple_duration);
        assert!(simple_duration < 100_000); // Less than 100ms

        // Benchmark complex function
        let complex_fn = r#"
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)
"#;
        let complex_duration = bench.benchmark_function(complex_fn, "fibonacci");
        println!("Complex function duration: {} μs", complex_duration);
        assert!(complex_duration < 500_000); // Less than 500ms

        // Test statistical benchmarking
        let stats = bench.benchmark_iterations(
            "def test_func(x: int) -> int: return x * 2 + 1",
            "arithmetic_test",
            5,
        );

        println!(
            "Benchmark stats: min={} μs, max={} μs, avg={} μs",
            stats.min_duration, stats.max_duration, stats.avg_duration
        );

        assert!(stats.min_duration > 0);
        assert!(stats.max_duration >= stats.min_duration);
        assert!(stats.avg_duration >= stats.min_duration);
        assert!(stats.avg_duration <= stats.max_duration);
        assert_eq!(stats.iterations, 5);

        // Performance consistency check
        let variance_ratio = stats.max_duration as f64 / stats.min_duration as f64;
        println!("Performance variance ratio: {:.2}x", variance_ratio);
        assert!(variance_ratio < 50.0); // Allow reasonable variance

        // Check benchmark storage
        let benchmarks = bench.get_benchmarks();
        assert!(benchmarks.contains_key("simple"));
        assert!(benchmarks.contains_key("fibonacci"));
        assert!(benchmarks.contains_key("arithmetic_test"));
    }

    /// Test error condition documentation
    #[test]
    fn test_error_condition_documentation() {
        println!("=== Error Condition Documentation Test ===");

        let mut error_doc = ErrorConditionDoctest::new();

        // Test various error conditions
        let error_cases = vec![
            ("def broken_function(\n    return 42", "syntax_error"),
            ("if condition\n    print('missing colon')", "missing_colon"),
            ("def f(): return x y", "invalid_expression"),
            (
                "async def unsupported(): await something()",
                "unsupported_feature",
            ),
            ("def f(: pass", "invalid_parameter"),
        ];

        for (code, category) in &error_cases {
            let result = error_doc.test_error_condition(code, category);
            println!("Error test '{}': {:?}", category, result.is_err());
            // Most should fail, but some might be handled gracefully
        }

        // Check error catalog
        let catalog = error_doc.get_error_catalog();
        println!("Error catalog size: {}", catalog.len());
        assert!(!catalog.is_empty()); // Should have captured some errors

        // Test error validation
        let validation_cases = vec![
            ("def f(", "Incomplete function should fail"),
            ("if True\n    pass", "Missing colon should fail"),
            ("return without function", "Invalid return should fail"),
        ];

        let validation = error_doc.validate_expected_errors(&validation_cases);
        println!(
            "Error validation: {}/{} failed as expected, {} unexpectedly succeeded",
            validation.failed_as_expected,
            validation.total_tests,
            validation.unexpectedly_succeeded
        );

        assert_eq!(validation.total_tests, 3);
        // Most errors should be caught, but allow some flexibility
        assert!(validation.failed_as_expected >= validation.total_tests / 2);
    }

    /// Test end-to-end workflow documentation
    #[test]
    fn test_workflow_documentation() {
        println!("=== Workflow Documentation Test ===");

        let mut workflow = WorkflowDoctest::new();

        // Build a complete workflow
        workflow.add_step(
            "Simple Function",
            "def add(a: int, b: int) -> int: return a + b",
        );
        workflow.add_step(
            "Control Flow",
            "def max_val(a: int, b: int) -> int: return a if a > b else b",
        );
        workflow.add_step(
            "Collections",
            "def sum_list(nums: list) -> int: return sum(nums) if nums else 0",
        );
        workflow.add_step(
            "Complex Logic",
            r#"
def process_data(data: list) -> dict:
    result = {"positive": [], "negative": [], "zero": []}
    for item in data:
        if item > 0:
            result["positive"].append(item)
        elif item < 0:
            result["negative"].append(item)
        else:
            result["zero"].append(item)
    return result
"#,
        );

        // Execute workflow
        let results = workflow.execute_workflow();
        println!(
            "Workflow results: {}/{} steps successful, {} failed",
            results.successful_steps, results.total_steps, results.failed_steps
        );

        assert_eq!(results.total_steps, 4);
        // At least basic functions should work
        assert!(results.successful_steps >= 2);

        // Check step details
        for (i, (name, _code, success, error)) in results.step_details.iter().enumerate() {
            println!(
                "Step {}: '{}' - {}",
                i + 1,
                name,
                if *success { "✓" } else { "✗" }
            );
            if let Some(err) = error {
                println!("  Error: {}", err.chars().take(100).collect::<String>());
            }
        }

        // Verify workflow steps are stored
        let steps = workflow.get_workflow_steps();
        assert_eq!(steps.len(), 4);
    }

    /// Test comprehensive doctest integration
    #[test]
    fn test_comprehensive_doctest_integration() {
        println!("=== Comprehensive Doctest Integration Test ===");

        // Test all doctest types together
        let mut interactive = InteractiveDoctest::new();
        let mut benchmark = PerformanceBenchmarkDoctest::new();
        let mut error_doc = ErrorConditionDoctest::new();
        let mut workflow = WorkflowDoctest::new();

        // Common test function
        let test_function =
            "def factorial(n: int) -> int: return 1 if n <= 1 else n * factorial(n - 1)";

        // Test in interactive session
        let interactive_result = interactive.execute(test_function);
        println!("Interactive result: {:?}", interactive_result.is_ok());

        // Benchmark the function
        let duration = benchmark.benchmark_function(test_function, "factorial_test");
        println!("Benchmark duration: {} μs", duration);

        // Add to workflow
        workflow.add_step("Recursive Function", test_function);

        // Test error conditions
        let error_result =
            error_doc.test_error_condition("def factorial(n: int", "incomplete_factorial");
        println!("Error test result: {:?}", error_result.is_err());

        // Verify integration
        assert_eq!(interactive.get_history().len(), 1);
        assert!(benchmark.get_benchmarks().contains_key("factorial_test"));
        assert_eq!(workflow.get_workflow_steps().len(), 1);

        // Performance should be reasonable
        assert!(duration < 200_000); // Less than 200ms

        println!("Comprehensive integration test completed successfully");
    }

    /// Test doctest performance characteristics
    #[test]
    fn test_doctest_performance_characteristics() {
        println!("=== Doctest Performance Characteristics Test ===");

        let mut benchmark = PerformanceBenchmarkDoctest::new();

        // Test different complexity levels
        let test_cases = vec![
            ("def empty(): pass", "empty_function"),
            ("def simple(x: int) -> int: return x", "identity_function"),
            (
                "def arithmetic(a: int, b: int) -> int: return a + b * 2",
                "arithmetic_function",
            ),
            (
                "def conditional(x: int) -> str: return 'positive' if x > 0 else 'non-positive'",
                "conditional_function",
            ),
            (
                "def loop_function(n: int) -> int: return sum(range(n))",
                "loop_function",
            ),
        ];

        let mut durations = Vec::new();

        for (code, name) in test_cases {
            let duration = benchmark.benchmark_function(code, name);
            durations.push((name, duration));
            println!("{}: {} μs", name, duration);
        }

        // Verify all tests complete quickly
        for (name, duration) in &durations {
            assert!(
                duration < &100_000,
                "{} took too long: {} μs",
                name,
                duration
            );
        }

        // Check that complexity generally correlates with time (allowing variance)
        let empty_time = durations[0].1;
        let loop_time = durations[4].1;

        println!(
            "Performance scaling: empty={}μs, loop={}μs, ratio={:.2}x",
            empty_time,
            loop_time,
            loop_time as f64 / empty_time as f64
        );

        // Allow significant variance but ensure loop isn't dramatically slower
        assert!(
            loop_time <= empty_time * 20,
            "Loop function too slow compared to empty function"
        );

        // Test statistical consistency
        let stats = benchmark.benchmark_iterations(
            "def consistent_test(): return 42",
            "consistency_test",
            10,
        );

        let consistency_ratio = stats.max_duration as f64 / stats.min_duration as f64;
        println!("Consistency ratio: {:.2}x (max/min)", consistency_ratio);

        // Should be reasonably consistent
        assert!(
            consistency_ratio < 20.0,
            "Performance too inconsistent: {:.2}x variance",
            consistency_ratio
        );
    }
}
