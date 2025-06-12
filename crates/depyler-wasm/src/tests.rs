#![allow(dead_code)]

use super::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_depyler_wasm_initialization() {
    let engine = DepylerWasm::new();
    // Engine initializes successfully
    let _ = engine; // Suppress unused variable warning
}

#[wasm_bindgen_test]
fn test_simple_function_transpilation() {
    let engine = DepylerWasm::new();
    let options = WasmTranspileOptions::new();

    let python_code = r#"
def add(a: int, b: int) -> int:
    return a + b
"#;

    let result = engine.transpile(python_code, &options).unwrap();

    // Verify basic structure
    assert!(result.success());
    assert!(!result.rust_code().is_empty());
    assert!(result.rust_code().contains("fn add"));
    assert!(result.rust_code().contains("i32"));
    assert!(result.transpile_time_ms() >= 0.0);
}

#[wasm_bindgen_test]
fn test_energy_estimation_accuracy() {
    let engine = DepylerWasm::new();
    let options = WasmTranspileOptions::new();

    let python_code = r#"
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)
"#;

    let result = engine.transpile(python_code, &options).unwrap();

    // Verify energy estimation bounds
    assert!(result.success());
    let energy = &result.energy_estimate();

    // Energy should be positive and reasonable
    assert!(energy.joules() >= 0.0);
    assert!(energy.joules() < 1.0); // Should be small for simple transpilation
    assert!(energy.watts_average() > 0.0);
    assert!(energy.co2_grams() >= 0.0);
    assert!(energy.confidence() >= 0.0 && energy.confidence() <= 1.0);
}

#[wasm_bindgen_test]
fn test_error_handling_invalid_syntax() {
    let engine = DepylerWasm::new();
    let options = WasmTranspileOptions::new();

    let invalid_python = r#"
def broken_function(
    # Missing closing parenthesis and body
"#;

    let result = engine.transpile(invalid_python, &options).unwrap();

    // Should fail gracefully
    assert!(!result.success());
    assert!(!result.errors().is_empty());
    assert!(result.rust_code().is_empty());
}

#[wasm_bindgen_test]
fn test_quality_metrics_calculation() {
    let engine = DepylerWasm::new();
    let options = WasmTranspileOptions::new();

    let python_code = r#"
def complex_function(data: list) -> dict:
    result = {}
    for item in data:
        if isinstance(item, str):
            if len(item) > 5:
                result[item] = len(item)
            else:
                result[item] = 0
        elif isinstance(item, int):
            if item > 0:
                result[str(item)] = item * 2
    return result
"#;

    let result = engine.transpile(python_code, &options).unwrap();

    if result.success() {
        let metrics = &result.quality_metrics();

        // PMAT score bounds validation
        assert!(metrics.pmat_score() >= 0.0 && metrics.pmat_score() <= 1.0);
        assert!(metrics.productivity() >= 0.0 && metrics.productivity() <= 1.0);
        assert!(metrics.maintainability() >= 0.0 && metrics.maintainability() <= 1.0);
        assert!(metrics.accessibility() >= 0.0 && metrics.accessibility() <= 1.0);
        assert!(metrics.testability() >= 0.0 && metrics.testability() <= 1.0);

        // Complexity should be reasonable
        assert!(metrics.code_complexity() >= 0);
        assert!(metrics.cyclomatic_complexity() >= 0);
    }
}

#[wasm_bindgen_test]
fn test_transpilation_performance_target() {
    let engine = DepylerWasm::new();
    let options = WasmTranspileOptions::new();

    let simple_function = r#"
def add(a: int, b: int) -> int:
    return a + b
"#;

    let start_time = js_sys::Date::now();
    let result = engine.transpile(simple_function, &options).unwrap();
    let end_time = js_sys::Date::now();

    let actual_time = end_time - start_time;

    // Should meet performance target: <50ms for simple functions
    assert!(
        actual_time < 50.0,
        "Transpilation took {}ms, exceeds 50ms target",
        actual_time
    );
    assert!(result.success());
}

#[wasm_bindgen_test]
fn test_code_analysis_functionality() {
    let engine = DepylerWasm::new();

    let python_code = r#"
def example_function():
    eval("dangerous_code")
    for i in range(len(items)):
        print(i)
"#;

    let analysis_result = engine.analyze_code(python_code);
    assert!(analysis_result.is_ok());

    // Should detect anti-patterns
    let analysis = analysis_result.unwrap();
    // Analysis should return structured data about the code
    assert!(js_sys::JSON::stringify(&analysis).is_ok());
}

#[wasm_bindgen_test]
fn test_benchmark_functionality() {
    let engine = DepylerWasm::new();

    let test_code = r#"
def test_function(x: int) -> int:
    return x * 2
"#;

    let benchmark_result = engine.benchmark(test_code, 5);
    assert!(benchmark_result.is_ok());

    let result = benchmark_result.unwrap();
    // Should return structured benchmark data
    assert!(js_sys::JSON::stringify(&result).is_ok());
}

#[wasm_bindgen_test]
fn test_memory_usage_measurement() {
    let engine = DepylerWasm::new();
    let options = WasmTranspileOptions::new();

    let memory_intensive_code = r#"
def process_large_data():
    large_list = [i for i in range(1000)]
    result = []
    for item in large_list:
        result.append(item * 2)
    return result
"#;

    let result = engine.transpile(memory_intensive_code, &options).unwrap();

    // Memory usage should be measured
    assert!(result.memory_usage_mb >= 0.0);

    // For WASM environment, memory might be minimal but should be tracked
    if result.memory_usage_mb > 0.0 {
        assert!(result.memory_usage_mb < 100.0); // Reasonable upper bound
    }
}

#[wasm_bindgen_test]
fn test_annotation_handling() {
    let engine = DepylerWasm::new();
    let options = WasmTranspileOptions::new();

    let annotated_code = r#"
# @depyler: optimize_energy=true, string_strategy=zero_copy
def optimized_function(text: str) -> str:
    return text.upper()
"#;

    let result = engine.transpile(annotated_code, &options).unwrap();

    if result.success() {
        // Should handle annotations without errors
        assert!(!result.rust_code().is_empty());
        // Energy optimization should be reflected
        assert!(result.energy_estimate().joules() >= 0.0);
    }
}

#[wasm_bindgen_test]
fn test_deterministic_transpilation() {
    let engine = DepylerWasm::new();
    let options = WasmTranspileOptions::new();

    let python_code = r#"
def deterministic_test(x: int, y: int) -> int:
    return x + y
"#;

    // Run transpilation multiple times
    let result1 = engine.transpile(python_code, &options).unwrap();
    let result2 = engine.transpile(python_code, &options).unwrap();

    if result1.success() && result2.success() {
        // Results should be identical (deterministic)
        assert_eq!(result1.rust_code(), result2.rust_code());

        // Quality metrics should be consistent
        let metrics1 = result1.quality_metrics();
        let metrics2 = result2.quality_metrics();

        assert_eq!(metrics1.code_complexity(), metrics2.code_complexity());
        assert_eq!(
            metrics1.cyclomatic_complexity(),
            metrics2.cyclomatic_complexity()
        );
    }
}

// Performance stress test
#[wasm_bindgen_test]
fn test_large_function_handling() {
    let engine = DepylerWasm::new();
    let options = WasmTranspileOptions::new();

    // Generate a larger function
    let mut large_function =
        String::from("def large_function(data: list) -> int:\n    result = 0\n");
    for i in 0..50 {
        large_function.push_str(&format!(
            "    if len(data) > {}:\n        result += {}\n",
            i, i
        ));
    }
    large_function.push_str("    return result\n");

    let start_time = js_sys::Date::now();
    let result = engine.transpile(&large_function, &options).unwrap();
    let end_time = js_sys::Date::now();

    let actual_time = end_time - start_time;

    // Should handle complex functions within reasonable time
    assert!(
        actual_time < 1000.0,
        "Large function transpilation took {}ms, exceeds 1000ms target",
        actual_time
    );

    if result.success() {
        let metrics = result.quality_metrics();
        assert!(metrics.code_complexity() > 10); // Should detect complexity
    }
}
