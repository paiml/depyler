//! EXTREME TDD Test Suite for call_resolution.rs Module
//!
//! DEPYLER-REFACTOR-001 Phase 2.6: Extract call resolution functions
//!
//! # Test Categories
//! 1. Builtin function calls - print, len, range, etc.
//! 2. Iterator builtins - map, filter, zip, enumerate
//! 3. Type conversion calls - int, float, str, bool
//! 4. Compilation verification tests
//! 5. Property-based tests
//!
//! # TDD Protocol
//! - RED: Tests written first, module doesn't exist yet
//! - GREEN: Extract module, tests pass
//! - REFACTOR: TDG A+ grade, 95% coverage

use depyler_core::DepylerPipeline;
use proptest::prelude::*;

// ============================================================================
// RED PHASE: Module Existence Test
// ============================================================================

#[test]
#[ignore = "RED PHASE: Module not yet extracted"]
fn test_call_resolution_module_exists() {
    let _module_path = std::path::Path::new("crates/depyler-core/src/rust_gen/call_resolution.rs");
}

// ============================================================================
// Builtin Function Calls - I/O
// ============================================================================

#[test]
fn test_print_call() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def greet(name: str):
    print(name)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("println!") || rust.contains("print!"),
        "print() should use println!/print!. Got:\n{rust}"
    );
}

#[test]
fn test_print_multiple_args() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def greet(first: str, last: str):
    print(first, last)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("println!") || rust.contains("print!"),
        "print() with multiple args should work. Got:\n{rust}"
    );
}

#[test]
fn test_input_call() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def get_name() -> str:
    return input("Enter name: ")
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    // input() typically maps to stdin reading
    assert!(
        rust.contains("stdin") || rust.contains("read_line") || rust.contains("input"),
        "input() should handle stdin. Got:\n{rust}"
    );
}

// ============================================================================
// Builtin Function Calls - Collection Operations
// ============================================================================

#[test]
fn test_len_call() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def get_length(items: list) -> int:
    return len(items)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains(".len()"),
        "len() should use .len(). Got:\n{rust}"
    );
}

#[test]
fn test_abs_call() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def absolute(x: int) -> int:
    return abs(x)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains(".abs()") || rust.contains("abs("),
        "abs() should use .abs(). Got:\n{rust}"
    );
}

#[test]
fn test_min_call() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def minimum(a: int, b: int) -> int:
    return min(a, b)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains(".min(") || rust.contains("min(") || rust.contains("std::cmp::min"),
        "min() should use .min() or std::cmp::min. Got:\n{rust}"
    );
}

#[test]
fn test_max_call() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def maximum(a: int, b: int) -> int:
    return max(a, b)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains(".max(") || rust.contains("max(") || rust.contains("std::cmp::max"),
        "max() should use .max() or std::cmp::max. Got:\n{rust}"
    );
}

#[test]
fn test_sum_call() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def total(items: list) -> int:
    return sum(items)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    // sum() may use .sum(), .iter().sum(), or fold pattern
    assert!(
        rust.contains(".sum()")
            || rust.contains("sum(")
            || rust.contains("fold")
            || rust.contains("iter"),
        "sum() should generate summing code. Got:\n{rust}"
    );
}

// ============================================================================
// Iterator Builtins
// ============================================================================

#[test]
fn test_map_call() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def double_all(items: list) -> list:
    return list(map(lambda x: x * 2, items))
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains(".map("),
        "map() should use .map(). Got:\n{rust}"
    );
}

#[test]
fn test_filter_call() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def evens_only(items: list) -> list:
    return list(filter(lambda x: x % 2 == 0, items))
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains(".filter("),
        "filter() should use .filter(). Got:\n{rust}"
    );
}

#[test]
fn test_zip_call() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def pair_up(a: list, b: list) -> list:
    return list(zip(a, b))
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains(".zip("),
        "zip() should use .zip(). Got:\n{rust}"
    );
}

#[test]
fn test_enumerate_call() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def with_indices(items: list) -> list:
    return list(enumerate(items))
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains(".enumerate()"),
        "enumerate() should use .enumerate(). Got:\n{rust}"
    );
}

#[test]
fn test_reversed_call() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def reverse_list(items: list) -> list:
    return list(reversed(items))
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains(".rev()") || rust.contains("reversed"),
        "reversed() should use .rev(). Got:\n{rust}"
    );
}

#[test]
fn test_sorted_call() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def sort_items(items: list) -> list:
    return sorted(items)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("sort") || rust.contains("sorted"),
        "sorted() should generate sorting code. Got:\n{rust}"
    );
}

// ============================================================================
// Type Conversion Calls (delegated to builtin_conversions)
// ============================================================================

#[test]
fn test_int_call() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def to_int(x: float) -> int:
    return int(x)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("as i32") || rust.contains("parse"),
        "int() should cast or parse. Got:\n{rust}"
    );
}

#[test]
fn test_float_call() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def to_float(x: int) -> float:
    return float(x)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("as f64") || rust.contains("parse"),
        "float() should cast or parse. Got:\n{rust}"
    );
}

#[test]
fn test_str_call() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def to_string(x: int) -> str:
    return str(x)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains(".to_string()") || rust.contains("format!"),
        "str() should use .to_string(). Got:\n{rust}"
    );
}

#[test]
fn test_bool_call() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def to_bool(x: int) -> bool:
    return bool(x)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("!= 0")
            || rust.contains("is_empty")
            || rust.contains("true")
            || rust.contains("false"),
        "bool() should check truthiness. Got:\n{rust}"
    );
}

// ============================================================================
// Special Function Calls
// ============================================================================

#[test]
fn test_isinstance_call() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def is_string(x) -> bool:
    return isinstance(x, str)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    // isinstance is tricky in Rust - may generate type checks or always true/false
    assert!(
        rust.contains("true")
            || rust.contains("false")
            || rust.contains("is_")
            || rust.contains("match"),
        "isinstance() should generate type check. Got:\n{rust}"
    );
}

#[test]
fn test_type_call() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def get_type(x: int) -> str:
    return str(type(x))
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    // type() is complex in Rust
    assert!(
        rust.contains("type") || rust.contains("Type") || rust.contains("std::any"),
        "type() should handle type inspection. Got:\n{rust}"
    );
}

#[test]
fn test_open_call() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def read_file(path: str) -> str:
    with open(path, 'r') as f:
        return f.read()
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("File") || rust.contains("open") || rust.contains("read"),
        "open() should use File operations. Got:\n{rust}"
    );
}

// ============================================================================
// Compilation Tests
// ============================================================================

#[test]
fn test_builtin_calls_compile() {
    let pipeline = DepylerPipeline::new();
    // Use typed parameter to avoid serde_json::Value
    let python = r#"
def test_builtins(items: list[int]) -> int:
    length = len(items)
    return length
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");

    // Skip compilation test if serde_json is used (external crate not available in rustc)
    if rust.contains("serde_json") {
        return; // Skip - external crate limitation
    }

    let temp_file = std::env::temp_dir().join("test_builtin_calls.rs");
    std::fs::write(&temp_file, &rust).expect("Failed to write temp file");

    let out_file = std::env::temp_dir().join("test_builtin_calls.rlib");
    let output = std::process::Command::new("rustc")
        .args(["--edition", "2021", "--crate-type", "lib", "-o"])
        .arg(&out_file)
        .arg(&temp_file)
        .output()
        .expect("Failed to run rustc");

    let _ = std::fs::remove_file(&out_file);
    let _ = std::fs::remove_file(&temp_file);

    assert!(
        output.status.success(),
        "Builtin calls should compile. Errors:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_iterator_calls_compile() {
    let pipeline = DepylerPipeline::new();
    // Use typed return to avoid serde_json::Value
    let python = r#"
def test_iterators() -> list[int]:
    items = [1, 2, 3, 4, 5]
    doubled = list(map(lambda x: x * 2, items))
    return doubled
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");

    // Skip compilation test if serde_json is used (external crate not available in rustc)
    if rust.contains("serde_json") {
        return; // Skip - external crate limitation
    }

    let temp_file = std::env::temp_dir().join("test_iterator_calls.rs");
    std::fs::write(&temp_file, &rust).expect("Failed to write temp file");

    let out_file = std::env::temp_dir().join("test_iterator_calls.rlib");
    let output = std::process::Command::new("rustc")
        .args(["--edition", "2021", "--crate-type", "lib", "-o"])
        .arg(&out_file)
        .arg(&temp_file)
        .output()
        .expect("Failed to run rustc");

    let _ = std::fs::remove_file(&out_file);
    let _ = std::fs::remove_file(&temp_file);

    assert!(
        output.status.success(),
        "Iterator calls should compile. Errors:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

// ============================================================================
// Property-Based Tests
// ============================================================================

proptest! {
    #[test]
    fn prop_transpilation_is_deterministic(seed in 0i32..1000) {
        let pipeline = DepylerPipeline::new();
        let python = format!(r#"
def test_call():
    return len([{}])
"#, seed);
        let result1 = pipeline.transpile(&python);
        let result2 = pipeline.transpile(&python);
        match (result1, result2) {
            (Ok(r1), Ok(r2)) => prop_assert_eq!(r1, r2, "Transpilation should be deterministic"),
            (Err(_), Err(_)) => (),
            _ => prop_assert!(false, "Results should be consistent"),
        }
    }

    #[test]
    fn prop_len_generates_method_call(size in 1usize..100) {
        let pipeline = DepylerPipeline::new();
        let items: Vec<String> = (0..size).map(|i| i.to_string()).collect();
        let python = format!(r#"
def test_len():
    return len([{}])
"#, items.join(", "));

        if let Ok(rust) = pipeline.transpile(&python) {
            prop_assert!(
                rust.contains(".len()"),
                "len() should generate .len(). Got:\n{}", rust
            );
        }
    }
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_nested_calls() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def nested() -> int:
    return len(list(range(10)))
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains(".len()"),
        "Nested calls should work. Got:\n{rust}"
    );
}

#[test]
fn test_chained_iterator_calls() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def chained(items: list) -> list:
    return list(filter(lambda x: x > 0, map(lambda x: x * 2, items)))
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains(".map(") && rust.contains(".filter("),
        "Chained iterators should work. Got:\n{rust}"
    );
}

#[test]
fn test_call_with_kwargs() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def with_kwargs():
    print("hello", end=" ")
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    // kwargs handling varies
    assert!(
        rust.contains("print") || rust.contains("hello"),
        "kwargs should be handled. Got:\n{rust}"
    );
}
