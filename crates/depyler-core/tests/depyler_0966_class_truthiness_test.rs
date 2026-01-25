//! DEPYLER-0966: Class Field Truthiness Transformation Tests
//!
//! This test module validates the correct generation of Rust code for Python
//! classes with collection fields used in truthiness checks.
//!
//! Pattern: `if not self.collection:` should become `if self.collection.is_empty()`
//! NOT: `if !self.collection` which causes E0308 (mismatched types: expected bool, found Vec)

use depyler_core::DepylerPipeline;

#[test]
fn test_class_list_field_truthiness_negated() {
    // Python: Check if list field is empty with `not`
    let python = r#"
class MinHeap:
    def __init__(self):
        self.heap: list[int] = []

    def is_empty(self) -> bool:
        if not self.heap:
            return True
        return False
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();

    // Should use .is_empty() for collection truthiness check
    // NOT: !self.heap (which would fail with E0308)
    assert!(
        rust_code.contains(".is_empty()"),
        "Should use .is_empty() for collection truthiness check\n\nGenerated:\n{}",
        rust_code
    );

    // Should NOT use ! operator on a Vec
    let lines: Vec<&str> = rust_code.lines().collect();
    let has_invalid_not_on_vec = lines.iter().any(|line| {
        // Check for pattern like "if !self.heap" or "if !self.heap.clone()"
        // but NOT "if !self.heap.is_empty()" which is correct
        line.contains("!self.heap") && !line.contains("is_empty")
    });

    assert!(
        !has_invalid_not_on_vec,
        "Should NOT use ! operator directly on Vec field\n\nGenerated:\n{}",
        rust_code
    );
}

#[test]
#[ignore] // TODO: DEPYLER-0967 - Positive truthiness needs statement-level transformation
fn test_class_list_field_truthiness_positive() {
    // Python: Check if list field is non-empty (truthy)
    // NOTE: This test is ignored because positive truthiness (`if self.items:`)
    // requires statement-level transformation in stmt_gen.rs, not just unary not handling.
    // The negated case (`if not self.items:`) works via convert_unary().
    let python = r#"
class Queue:
    def __init__(self):
        self.items: list[str] = []

    def has_items(self) -> bool:
        if self.items:
            return True
        return False
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();

    // For positive truthiness check on collection, should use !.is_empty()
    // Pattern: `if self.items:` → `if !self.items.is_empty():`
    assert!(
        rust_code.contains("is_empty()"),
        "Should use is_empty() for collection truthiness check\n\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_class_dict_field_truthiness() {
    // Python: Check if dict field is empty
    let python = r#"
class Cache:
    def __init__(self):
        self.data: dict[str, int] = {}

    def is_populated(self) -> bool:
        if not self.data:
            return False
        return True
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();

    // Should use .is_empty() for dict truthiness
    assert!(
        rust_code.contains("is_empty()"),
        "Should use is_empty() for dict truthiness check\n\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_class_optional_field_truthiness() {
    // Python: Check if Optional field is None
    let python = r#"
class Container:
    def __init__(self):
        self.cached: int | None = None

    def has_cache(self) -> bool:
        if not self.cached:
            return False
        return True
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();

    // Should use .is_none() for Optional truthiness with negation
    assert!(
        rust_code.contains("is_none()") || rust_code.contains("is_some()"),
        "Should use is_none() or is_some() for Optional truthiness check\n\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_heap_pop_pattern() {
    // The specific pattern from example_heap_queue that was failing
    let python = r#"
class MinHeap:
    def __init__(self):
        self.heap: list[int] = []

    def pop(self) -> int | None:
        if not self.heap:
            return None
        return self.heap[0]
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();

    // Should generate is_empty() check
    assert!(
        rust_code.contains("is_empty()"),
        "Should use is_empty() for heap truthiness check\n\nGenerated:\n{}",
        rust_code
    );

    // The condition should be something like:
    // `if self.heap.clone().is_empty()` or `if self.heap.is_empty()`
    // NOT: `if !self.heap.clone()`
    let lines: Vec<&str> = rust_code.lines().collect();
    let has_correct_pattern = lines
        .iter()
        .any(|line| line.contains("self.heap") && line.contains("is_empty()"));

    assert!(
        has_correct_pattern,
        "Should have self.heap with is_empty() pattern\n\nGenerated:\n{}",
        rust_code
    );
}
