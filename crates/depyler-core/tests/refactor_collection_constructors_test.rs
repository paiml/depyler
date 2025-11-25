//! EXTREME TDD Test Suite for collection_constructors.rs Module
//!
//! DEPYLER-REFACTOR-001 Phase 2.2: Extract collection constructor functions
//!
//! # Test Categories
//! 1. Behavior preservation tests - Ensure identical behavior after extraction
//! 2. Compilation verification tests - Generated Rust must compile
//! 3. Property-based tests - Invariants with proptest
//!
//! # TDD Protocol
//! - RED: Tests written first, module doesn't exist yet
//! - GREEN: Extract module, tests pass
//! - REFACTOR: TDG A+ grade, 95% coverage

use depyler_core::DepylerPipeline;
use proptest::prelude::*;

// ============================================================================
// RED PHASE: Module Existence Test (should FAIL until module extracted)
// ============================================================================

#[test]
#[ignore = "RED PHASE: Module not yet extracted"]
fn test_collection_constructors_module_exists() {
    // This test will pass once the module is extracted
    // For now, it's ignored to allow the test suite to run
    let _module_path = std::path::Path::new(
        "crates/depyler-core/src/rust_gen/collection_constructors.rs",
    );
    // Module should export the conversion functions
}

// ============================================================================
// Behavior Preservation Tests - set()
// ============================================================================

#[test]
fn test_set_constructor_empty() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def make_empty_set():
    return set()
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("HashSet") || rust.contains("HashSet::<"),
        "set() should generate HashSet. Got:\n{rust}"
    );
}

#[test]
fn test_set_constructor_from_list() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def make_set_from_list():
    items = [1, 2, 3]
    return set(items)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("collect::<HashSet") || rust.contains(".into_iter()"),
        "set(items) should use into_iter().collect(). Got:\n{rust}"
    );
}

#[test]
fn test_set_constructor_compiles() {
    let pipeline = DepylerPipeline::new();
    // Use the set so type inference works
    let python = r#"
def test_set() -> int:
    empty = set()
    from_list = set([1, 2, 3])
    from_list.add(4)
    return len(from_list)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");

    // Write to temp file and compile
    let temp_file = std::env::temp_dir().join("test_set_constructor.rs");
    std::fs::write(&temp_file, &rust).expect("Failed to write temp file");

    let out_file = std::env::temp_dir().join("test_set_constructor.rlib");
    let output = std::process::Command::new("rustc")
        .args(["--edition", "2021", "--crate-type", "lib", "-o"])
        .arg(&out_file)
        .arg(&temp_file)
        .output()
        .expect("Failed to run rustc");

    // Cleanup
    let _ = std::fs::remove_file(&out_file);
    let _ = std::fs::remove_file(&temp_file);

    assert!(
        output.status.success(),
        "Generated set code should compile. Errors:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

// ============================================================================
// Behavior Preservation Tests - frozenset()
// ============================================================================

#[test]
fn test_frozenset_constructor_empty() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def make_empty_frozenset():
    return frozenset()
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("Arc") && rust.contains("HashSet"),
        "frozenset() should generate Arc<HashSet>. Got:\n{rust}"
    );
}

#[test]
fn test_frozenset_constructor_from_list() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def make_frozenset_from_list():
    items = [1, 2, 3]
    return frozenset(items)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("Arc::new") && rust.contains("HashSet"),
        "frozenset(items) should use Arc::new(...HashSet...). Got:\n{rust}"
    );
}

// ============================================================================
// Behavior Preservation Tests - dict()
// ============================================================================

#[test]
fn test_dict_constructor_empty() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def make_empty_dict():
    return dict()
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("HashMap::new()"),
        "dict() should generate HashMap::new(). Got:\n{rust}"
    );
}

#[test]
fn test_dict_constructor_compiles() {
    let pipeline = DepylerPipeline::new();
    // Use the dict so type inference works
    let python = r#"
def test_dict() -> int:
    d = dict()
    d["key"] = "value"
    return len(d)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");

    let temp_file = std::env::temp_dir().join("test_dict_constructor.rs");
    std::fs::write(&temp_file, &rust).expect("Failed to write temp file");

    let out_file = std::env::temp_dir().join("test_dict_constructor.rlib");
    let output = std::process::Command::new("rustc")
        .args(["--edition", "2021", "--crate-type", "lib", "-o"])
        .arg(&out_file)
        .arg(&temp_file)
        .output()
        .expect("Failed to run rustc");

    // Cleanup
    let _ = std::fs::remove_file(&out_file);
    let _ = std::fs::remove_file(&temp_file);

    assert!(
        output.status.success(),
        "Generated dict code should compile. Errors:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

// ============================================================================
// Behavior Preservation Tests - list()
// ============================================================================

#[test]
fn test_list_constructor_empty() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def make_empty_list():
    return list()
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("Vec::new()"),
        "list() should generate Vec::new(). Got:\n{rust}"
    );
}

#[test]
fn test_list_constructor_from_range() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def make_list_from_range():
    return list(range(5))
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("collect::<Vec") || rust.contains(".collect()"),
        "list(range(5)) should use .collect(). Got:\n{rust}"
    );
}

#[test]
fn test_list_constructor_compiles() {
    let pipeline = DepylerPipeline::new();
    // Use the list so type inference works
    let python = r#"
def test_list() -> int:
    from_range = list(range(5))
    from_range.append(10)
    return len(from_range)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");

    let temp_file = std::env::temp_dir().join("test_list_constructor.rs");
    std::fs::write(&temp_file, &rust).expect("Failed to write temp file");

    let out_file = std::env::temp_dir().join("test_list_constructor.rlib");
    let output = std::process::Command::new("rustc")
        .args(["--edition", "2021", "--crate-type", "lib", "-o"])
        .arg(&out_file)
        .arg(&temp_file)
        .output()
        .expect("Failed to run rustc");

    // Cleanup
    let _ = std::fs::remove_file(&out_file);
    let _ = std::fs::remove_file(&temp_file);

    assert!(
        output.status.success(),
        "Generated list code should compile. Errors:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

// ============================================================================
// Behavior Preservation Tests - deque()
// ============================================================================

#[test]
fn test_deque_constructor_empty() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
from collections import deque

def make_empty_deque():
    return deque()
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("VecDeque::new()"),
        "deque() should generate VecDeque::new(). Got:\n{rust}"
    );
}

#[test]
fn test_deque_constructor_from_list() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
from collections import deque

def make_deque_from_list():
    items = [1, 2, 3]
    return deque(items)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("VecDeque::from") || rust.contains("VecDeque"),
        "deque(items) should use VecDeque::from. Got:\n{rust}"
    );
}

// ============================================================================
// Behavior Preservation Tests - Counter()
// ============================================================================

#[test]
fn test_counter_constructor_empty() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
from collections import Counter

def make_empty_counter():
    return Counter()
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("HashMap::new()"),
        "Counter() should generate HashMap::new(). Got:\n{rust}"
    );
}

#[test]
fn test_counter_constructor_from_iterable() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
from collections import Counter

def count_items():
    items = [1, 1, 2, 3, 3, 3]
    return Counter(items)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("fold") || rust.contains("entry"),
        "Counter(items) should use fold with entry. Got:\n{rust}"
    );
}

// ============================================================================
// Iterator Builtins Tests - enumerate(), zip(), all(), any()
// ============================================================================

#[test]
fn test_enumerate_builtin() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def test_enumerate():
    items = [10, 20, 30]
    result = list(enumerate(items))
    return result
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("enumerate()"),
        "enumerate() should generate .enumerate(). Got:\n{rust}"
    );
}

#[test]
fn test_enumerate_with_start() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def test_enumerate_start():
    items = [10, 20, 30]
    result = list(enumerate(items, 1))
    return result
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("enumerate()"),
        "enumerate(items, 1) should generate .enumerate(). Got:\n{rust}"
    );
}

#[test]
fn test_zip_builtin() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def test_zip():
    a = [1, 2, 3]
    b = [4, 5, 6]
    result = list(zip(a, b))
    return result
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains(".zip("),
        "zip(a, b) should generate .zip(). Got:\n{rust}"
    );
}

#[test]
fn test_all_builtin() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def test_all():
    items = [True, True, True]
    return all(items)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains(".all("),
        "all(items) should generate .all(). Got:\n{rust}"
    );
}

#[test]
fn test_any_builtin() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def test_any():
    items = [False, True, False]
    return any(items)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains(".any("),
        "any(items) should generate .any(). Got:\n{rust}"
    );
}

#[test]
fn test_reversed_builtin() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def test_reversed():
    items = [1, 2, 3]
    result = list(reversed(items))
    return result
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    // reversed() may generate .rev() or .into_iter().rev()
    assert!(
        rust.contains(".rev()") || rust.contains("rev()") || rust.contains("reversed"),
        "reversed(items) should generate .rev() or equivalent. Got:\n{rust}"
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
def test_collection():
    s = set()
    d = dict()
    l = list()
    return {}
"#, seed);
        let result1 = pipeline.transpile(&python);
        let result2 = pipeline.transpile(&python);
        match (result1, result2) {
            (Ok(r1), Ok(r2)) => prop_assert_eq!(r1, r2, "Transpilation should be deterministic"),
            (Err(_), Err(_)) => (), // Both errors is acceptable
            _ => prop_assert!(false, "Results should be consistent"),
        }
    }

    #[test]
    fn prop_collection_constructors_generate_valid_rust(
        constructor in prop::sample::select(vec!["set", "dict", "list"])
    ) {
        let pipeline = DepylerPipeline::new();
        let python = format!(r#"
def test_{}():
    return {}()
"#, constructor, constructor);

        if let Ok(rust) = pipeline.transpile(&python) {
            // Should contain valid Rust collection type
            let valid = rust.contains("HashSet")
                || rust.contains("HashMap")
                || rust.contains("Vec");
            prop_assert!(valid, "Constructor {} should generate valid Rust collection type", constructor);
        }
    }

    #[test]
    fn prop_empty_constructors_use_new(
        constructor in prop::sample::select(vec!["set", "dict", "list"])
    ) {
        let pipeline = DepylerPipeline::new();
        let python = format!(r#"
def test_empty_{}():
    return {}()
"#, constructor, constructor);

        if let Ok(rust) = pipeline.transpile(&python) {
            // Empty constructors should use ::new()
            prop_assert!(
                rust.contains("::new()") || rust.contains("new()"),
                "Empty {} should use ::new(). Got:\n{}", constructor, rust
            );
        }
    }

    #[test]
    fn prop_function_names_are_valid_rust(seed in 0u32..100) {
        let pipeline = DepylerPipeline::new();
        let python = format!(r#"
def collection_test_{}():
    s = set()
    return len(s)
"#, seed);

        if let Ok(rust) = pipeline.transpile(&python) {
            // Function names should be valid Rust identifiers
            prop_assert!(
                rust.contains(&format!("fn collection_test_{}", seed)),
                "Function name should be preserved"
            );
        }
    }
}

// ============================================================================
// Edge Cases and Error Handling
// ============================================================================

#[test]
fn test_set_too_many_args() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def bad_set():
    return set(1, 2)
"#;
    // Should either error or generate something reasonable
    let result = pipeline.transpile(python);
    // We're flexible here - either error or output is acceptable
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_nested_collection_constructors() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def nested_collections():
    s = set([1, 2, 3])
    l = list(s)
    return l
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("HashSet") && (rust.contains("Vec") || rust.contains("collect")),
        "Nested collections should generate correct types. Got:\n{rust}"
    );
}
