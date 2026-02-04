//! Coverage tests for codegen.rs
//!
//! DEPYLER-99MODE-001: Targets codegen.rs (2,879 lines)
//! Covers: hir_to_rust, generate_rust, function conversion,
//! collection type detection, import generation, code formatting.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

fn transpile(code: &str) -> String {
    DepylerPipeline::new()
        .transpile(code)
        .unwrap_or_else(|e| panic!("Transpilation failed: {e}"))
}

// ============================================================================
// Basic code generation
// ============================================================================

#[test]
fn test_codegen_simple_function() {
    let rust = transpile("def f() -> int:\n    return 42\n");
    assert!(rust.contains("fn f()"));
    assert!(rust.contains("42"));
}

#[test]
fn test_codegen_function_with_params() {
    let rust = transpile("def add(a: int, b: int) -> int:\n    return a + b\n");
    assert!(rust.contains("fn add"));
}

#[test]
fn test_codegen_multiple_functions() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b

def sub(a: int, b: int) -> int:
    return a - b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn add"));
    assert!(rust.contains("fn sub"));
}

#[test]
fn test_codegen_void_function() {
    let code = "def f(x: int):\n    print(x)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_codegen_return_string() {
    let rust = transpile("def f() -> str:\n    return \"hello\"\n");
    assert!(rust.contains("hello"));
}

// ============================================================================
// Import generation
// ============================================================================

#[test]
fn test_codegen_hashmap_import() {
    let code = "def f() -> dict:\n    return {\"a\": 1}\n";
    let rust = transpile(code);
    assert!(rust.contains("HashMap") || rust.contains("hash_map") || rust.contains("dict"));
}

#[test]
fn test_codegen_hashset_import() {
    let code = "def f() -> set:\n    return {1, 2, 3}\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Collection type detection
// ============================================================================

#[test]
fn test_codegen_list_of_int() {
    let code = "def f() -> list:\n    return [1, 2, 3]\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_codegen_list_of_str() {
    let code = r#"
def f() -> list:
    return ["a", "b", "c"]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_codegen_dict_str_int() {
    let code = r#"
def f() -> dict:
    return {"x": 1, "y": 2}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_codegen_nested_list() {
    let code = r#"
def f() -> list:
    return [[1, 2], [3, 4]]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Code formatting
// ============================================================================

#[test]
fn test_codegen_formatted_output() {
    let rust = transpile("def f() -> int:\n    return 1\n");
    assert!(!rust.is_empty());
}

#[test]
fn test_codegen_multiline_function() {
    let code = r#"
def process(items: list) -> int:
    total = 0
    for item in items:
        total += item
    return total
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn process"));
}

// ============================================================================
// Complex patterns
// ============================================================================

#[test]
fn test_codegen_class_to_struct() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_codegen_class_with_method() {
    let code = r#"
class Counter:
    def __init__(self):
        self.count = 0

    def increment(self):
        self.count += 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_codegen_recursive_function() {
    let code = r#"
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_codegen_generator_function() {
    let code = r#"
def gen(n: int):
    for i in range(n):
        yield i
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_codegen_async_function() {
    let code = "async def f() -> int:\n    return 42\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_codegen_exception_handling() {
    let code = r#"
def safe_div(a: int, b: int) -> int:
    try:
        return a // b
    except:
        return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_codegen_with_statement() {
    let code = r#"
def read_file(path: str) -> str:
    with open(path) as f:
        return f.read()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_codegen_list_comprehension() {
    let code = "def f() -> list:\n    return [x * 2 for x in range(10)]\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_codegen_lambda() {
    let code = r#"
def f(items: list) -> list:
    return sorted(items, key=lambda x: -x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_codegen_fstring() {
    let code = r#"
def f(name: str) -> str:
    return f"Hello {name}"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_codegen_default_params() {
    let code = r#"
def f(x: int = 0, y: str = "hi") -> str:
    return y + str(x)
"#;
    assert!(transpile_ok(code));
}
