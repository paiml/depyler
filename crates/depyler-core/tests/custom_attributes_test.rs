//! Integration tests for custom Rust attributes feature
//!
//! Tests that custom Rust attributes specified via @depyler: custom_attribute
//! are correctly parsed and emitted in the generated Rust code.

use depyler_core::DepylerPipeline;

#[test]
fn test_single_custom_attribute() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
# @depyler: custom_attribute = "inline"
def add(a: int, b: int) -> int:
    return a + b
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();

    // Verify the custom attribute is present
    assert!(
        rust_code.contains("#[inline]"),
        "Should contain #[inline] attribute"
    );
    assert!(
        rust_code.contains("pub fn add"),
        "Should contain function definition"
    );
}

#[test]
fn test_multiple_custom_attributes() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
# @depyler: custom_attribute = "inline"
# @depyler: custom_attribute = "must_use"
def calculate(x: int) -> int:
    return x * 2
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();

    // Verify both custom attributes are present
    assert!(
        rust_code.contains("#[inline]"),
        "Should contain #[inline] attribute"
    );
    assert!(
        rust_code.contains("#[must_use]"),
        "Should contain #[must_use] attribute"
    );
    assert!(
        rust_code.contains("pub fn calculate"),
        "Should contain function definition"
    );
}

#[test]
fn test_custom_attribute_with_args() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
# @depyler: custom_attribute = "inline(always)"
def fast_function(n: int) -> int:
    return n + 1
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();

    // Verify the attribute with arguments is present
    assert!(
        rust_code.contains("#[inline(always)]"),
        "Should contain #[inline(always)] attribute"
    );
    assert!(
        rust_code.contains("pub fn fast_function"),
        "Should contain function definition"
    );
}

#[test]
fn test_custom_attributes_with_other_annotations() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
# @depyler: optimization_level = "aggressive"
# @depyler: custom_attribute = "inline"
# @depyler: performance_critical = "true"
def hot_path(items: list[int]) -> int:
    total = 0
    for item in items:
        total += item
    return total
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();

    // Verify the custom attribute is present alongside other optimizations
    assert!(
        rust_code.contains("#[inline]"),
        "Should contain #[inline] attribute"
    );
    assert!(
        rust_code.contains("pub fn hot_path"),
        "Should contain function definition"
    );
}

#[test]
fn test_no_custom_attributes() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def normal_function(x: int) -> int:
    return x * 2
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();

    // Verify function exists but no custom attributes (except maybe doc comments)
    assert!(
        rust_code.contains("pub fn normal_function"),
        "Should contain function definition"
    );
}

#[test]
fn test_custom_attribute_cold() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
# @depyler: custom_attribute = "cold"
def error_handler(msg: str) -> None:
    print(msg)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();

    // Verify the cold attribute is present
    assert!(
        rust_code.contains("#[cold]"),
        "Should contain #[cold] attribute"
    );
    assert!(
        rust_code.contains("pub fn error_handler"),
        "Should contain function definition"
    );
}

#[test]
fn test_custom_attribute_repr() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
# @depyler: custom_attribute = "repr(C)"
def get_layout() -> int:
    return 42
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();

    // Verify the repr attribute is present
    assert!(
        rust_code.contains("#[repr(C)]"),
        "Should contain #[repr(C)] attribute"
    );
    assert!(
        rust_code.contains("pub fn get_layout"),
        "Should contain function definition"
    );
}

#[test]
fn test_multiple_functions_different_attributes() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
# @depyler: custom_attribute = "inline"
def fast_func(x: int) -> int:
    return x + 1

# @depyler: custom_attribute = "cold"
def slow_func(x: int) -> int:
    return x - 1
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();

    // Verify both functions have their respective attributes
    assert!(
        rust_code.contains("#[inline]"),
        "Should contain #[inline] attribute"
    );
    assert!(
        rust_code.contains("#[cold]"),
        "Should contain #[cold] attribute"
    );
    assert!(
        rust_code.contains("pub fn fast_func"),
        "Should contain fast_func definition"
    );
    assert!(
        rust_code.contains("pub fn slow_func"),
        "Should contain slow_func definition"
    );
}

#[test]
fn test_custom_attribute_with_docstring() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
# @depyler: custom_attribute = "inline"
def documented_function(x: int) -> int:
    """This is a documented function."""
    return x * 2
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();

    // Verify both docstring and custom attribute are present
    assert!(
        rust_code.contains("#[inline]"),
        "Should contain #[inline] attribute"
    );
    assert!(
        rust_code.contains("pub fn documented_function"),
        "Should contain function definition"
    );
    // Doc comment format may vary, so we check for the content
    assert!(
        rust_code.contains("documented function")
            || rust_code.contains("This is a documented function"),
        "Should contain docstring content"
    );
}

#[test]
fn test_parse_to_hir_preserves_custom_attributes() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
# @depyler: custom_attribute = "inline"
# @depyler: custom_attribute = "must_use"
def test_func(x: int) -> int:
    return x
"#;

    let hir = pipeline.parse_to_hir(python_code).unwrap();

    // Verify the HIR preserves custom attributes
    assert_eq!(hir.functions.len(), 1);
    let func = &hir.functions[0];
    assert_eq!(func.annotations.custom_attributes.len(), 2);
    assert_eq!(func.annotations.custom_attributes[0], "inline");
    assert_eq!(func.annotations.custom_attributes[1], "must_use");
}
