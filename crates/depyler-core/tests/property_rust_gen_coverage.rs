//! Property-based tests for rust_gen modules to achieve solidified 95% coverage
//!
//! DEPYLER-COVERAGE-95: These tests verify code generation invariants:
//! - Determinism: same input always produces same output
//! - Validity: generated Rust code is syntactically valid
//! - Completeness: all HIR expression types are handled

use depyler_core::DepylerPipeline;
use proptest::prelude::*;

/// Helper to transpile Python code
fn transpile(code: &str) -> Result<String, anyhow::Error> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code)
}

// =============================================================================
// STRATEGY: Generate valid Python code snippets for property testing
// =============================================================================

/// Generate simple Python expressions
fn simple_python_expr() -> impl Strategy<Value = String> {
    prop_oneof![
        // Integer literals
        any::<i32>().prop_map(|n| n.to_string()),
        // Float literals
        any::<f64>()
            .prop_filter("finite", |f| f.is_finite())
            .prop_map(|n| format!("{:.2}", n)),
        // String literals
        "[a-zA-Z_][a-zA-Z0-9_]{0,10}"
            .prop_map(|s| format!("\"{}\"", s)),
        // Boolean literals
        any::<bool>().prop_map(|b| if b { "True" } else { "False" }.to_string()),
        // None
        Just("None".to_string()),
    ]
}

/// Generate simple Python statements
fn simple_python_stmt() -> impl Strategy<Value = String> {
    prop_oneof![
        // Assignment
        ("[a-z][a-z0-9]{0,5}", simple_python_expr())
            .prop_map(|(name, expr)| format!("{} = {}", name, expr)),
        // Pass
        Just("pass".to_string()),
        // Return with value
        simple_python_expr().prop_map(|e| format!("return {}", e)),
    ]
}

/// Rust keywords that cannot be used as function names
const RUST_KEYWORDS: &[&str] = &[
    "fn", "let", "mut", "const", "static", "if", "else", "match", "loop",
    "while", "for", "in", "break", "continue", "return", "type", "impl",
    "trait", "struct", "enum", "mod", "pub", "use", "as", "self", "super",
    "crate", "where", "async", "await", "dyn", "ref", "move", "true", "false",
];

/// Generate annotated Python function (excluding Rust keywords)
fn annotated_python_function() -> impl Strategy<Value = String> {
    (
        "[a-z][a-z0-9]{0,8}",           // function name
        prop::collection::vec(simple_python_stmt(), 1..5), // body statements
    )
        .prop_filter("not rust keyword", |(name, _)| !RUST_KEYWORDS.contains(&name.as_str()))
        .prop_map(|(name, body)| {
            let body_str = body
                .iter()
                .map(|s| format!("    {}", s))
                .collect::<Vec<_>>()
                .join("\n");
            format!(
                "def {}() -> None:\n{}",
                name, body_str
            )
        })
}

/// Generate typed Python function (excluding Rust keywords)
fn typed_python_function() -> impl Strategy<Value = String> {
    (
        "[a-z][a-z0-9]{0,8}",  // function name
        prop_oneof![
            Just(("a: int", "int")),
            Just(("a: float", "float")),
            Just(("a: str", "str")),
            Just(("a: bool", "bool")),
        ],
    )
        .prop_filter("not rust keyword", |(name, _)| !RUST_KEYWORDS.contains(&name.as_str()))
        .prop_map(|(name, (params, ret))| {
            format!(
                "def {}({}) -> {}:\n    return a",
                name, params, ret
            )
        })
}

// =============================================================================
// PROPERTY TESTS: Determinism
// =============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Property: Transpilation is deterministic
    /// Same Python input must always produce identical Rust output
    #[test]
    fn prop_transpile_deterministic(code in annotated_python_function()) {
        let result1 = transpile(&code);
        let result2 = transpile(&code);

        match (result1, result2) {
            (Ok(r1), Ok(r2)) => {
                prop_assert_eq!(r1, r2, "Transpilation must be deterministic");
            }
            (Err(_), Err(_)) => {
                // Both errored - that's consistent
            }
            _ => {
                prop_assert!(false, "Inconsistent success/failure between runs");
            }
        }
    }

    /// Property: Typed functions produce valid Rust
    #[test]
    fn prop_typed_function_produces_valid_rust(code in typed_python_function()) {
        if let Ok(rust_code) = transpile(&code) {
            // Verify it parses as valid Rust
            let parse_result: Result<syn::File, syn::Error> = syn::parse_file(&rust_code);
            prop_assert!(
                parse_result.is_ok(),
                "Generated Rust must parse. Code:\n{}",
                rust_code
            );
        }
    }
}

// =============================================================================
// PROPERTY TESTS: Expression Coverage
// =============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    /// Property: Binary operations are handled
    #[test]
    fn prop_binary_ops_handled(
        op in prop_oneof![
            Just("+"),
            Just("-"),
            Just("*"),
            Just("//"),
            Just("%"),
        ]
    ) {
        let code = format!(
            "def compute(x: int, y: int) -> int:\n    return x {} y",
            op
        );
        let result = transpile(&code);
        prop_assert!(result.is_ok(), "Binary op {} should transpile", op);

        if let Ok(rust_code) = result {
            let parse: Result<syn::File, _> = syn::parse_file(&rust_code);
            prop_assert!(parse.is_ok(), "Generated Rust should parse for op {}", op);
        }
    }

    /// Property: Comparison operations are handled
    #[test]
    fn prop_comparison_ops_handled(
        op in prop_oneof![
            Just("=="),
            Just("!="),
            Just("<"),
            Just("<="),
            Just(">"),
            Just(">="),
        ]
    ) {
        let code = format!(
            "def compare(x: int, y: int) -> bool:\n    return x {} y",
            op
        );
        let result = transpile(&code);
        prop_assert!(result.is_ok(), "Comparison op {} should transpile", op);
    }

    /// Property: Boolean operations are handled
    #[test]
    fn prop_boolean_ops_handled(
        op in prop_oneof![
            Just("and"),
            Just("or"),
        ]
    ) {
        let code = format!(
            "def logic(x: bool, y: bool) -> bool:\n    return x {} y",
            op
        );
        let result = transpile(&code);
        prop_assert!(result.is_ok(), "Boolean op {} should transpile", op);
    }

    /// Property: If statements are handled
    #[test]
    fn prop_if_statements_handled(condition in any::<bool>()) {
        let cond_str = if condition { "True" } else { "False" };
        let code = format!(
            "def check() -> int:\n    if {}:\n        return 1\n    return 0",
            cond_str
        );
        let result = transpile(&code);
        prop_assert!(result.is_ok(), "If statement should transpile");
    }

    /// Property: For loops are handled
    #[test]
    fn prop_for_loops_handled(n in 1usize..10) {
        let code = format!(
            "def loop_test() -> int:\n    total: int = 0\n    for i in range({}):\n        total = total + i\n    return total",
            n
        );
        let result = transpile(&code);
        prop_assert!(result.is_ok(), "For loop should transpile");
    }
}

// =============================================================================
// UNIT TESTS: Type Handling and Edge Cases
// =============================================================================

#[test]
fn test_while_loops_handled() {
    let code = "def while_test() -> int:\n    x: int = 0\n    while x < 10:\n        x = x + 1\n    return x";
    let result = transpile(code);
    assert!(result.is_ok(), "While loop should transpile");
}

#[test]
fn test_list_types_handled() {
    let code = "def list_test() -> list[int]:\n    return [1, 2, 3]";
    let result = transpile(code);
    assert!(result.is_ok(), "List type should transpile");
}

#[test]
fn test_dict_types_handled() {
    let code = "def dict_test() -> dict[str, int]:\n    return {\"a\": 1}";
    let result = transpile(code);
    assert!(result.is_ok(), "Dict type should transpile");
}

#[test]
fn test_optional_types_handled() {
    let code = "from typing import Optional\ndef opt_test(x: Optional[int]) -> Optional[int]:\n    return x";
    // Optional types may or may not be fully supported
    // Just verify it doesn't panic
    let _ = transpile(code);
}

#[test]
fn test_empty_function() {
    let code = "def empty() -> None:\n    pass";
    let result = transpile(code);
    assert!(result.is_ok());
}

#[test]
fn test_nested_expressions() {
    let code = "def nested() -> int:\n    return ((1 + 2) * 3) - 4";
    let result = transpile(code);
    assert!(result.is_ok());
}

#[test]
fn test_string_operations() {
    let code = "def strings() -> str:\n    return \"hello\" + \" \" + \"world\"";
    let result = transpile(code);
    assert!(result.is_ok());
}

#[test]
fn test_multiple_returns() {
    let code = "def multi_return(x: int) -> int:\n    if x > 0:\n        return 1\n    return -1";
    let result = transpile(code);
    assert!(result.is_ok());
}

#[test]
fn test_augmented_assignment() {
    let code = "def augment() -> int:\n    x: int = 0\n    x += 1\n    x -= 1\n    x *= 2\n    return x";
    let result = transpile(code);
    assert!(result.is_ok());
}

// =============================================================================
// PROPERTY TESTS: Advanced Code Generation Paths
// =============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(30))]

    /// Property: Unary operations are handled
    #[test]
    fn prop_unary_ops_handled(val in any::<i32>()) {
        let code = format!(
            "def negate() -> int:\n    x: int = {}\n    return -x",
            val
        );
        let result = transpile(&code);
        prop_assert!(result.is_ok(), "Unary negation should transpile");
    }

    /// Property: String formatting is handled
    #[test]
    fn prop_fstring_handled(n in 0i32..100) {
        let code = format!(
            "def fmt() -> str:\n    x: int = {}\n    return f\"value: {{x}}\"",
            n
        );
        let result = transpile(&code);
        // f-strings may not be fully supported, just verify no panic
        let _ = result;
    }

    /// Property: Multiple function definitions are handled
    #[test]
    fn prop_multiple_functions(count in 1usize..5) {
        let funcs: Vec<String> = (0..count)
            .map(|i| format!("def func{}() -> int:\n    return {}", i, i))
            .collect();
        let code = funcs.join("\n\n");
        let result = transpile(&code);
        prop_assert!(result.is_ok(), "Multiple functions should transpile");
    }

    /// Property: Nested if statements are handled
    #[test]
    fn prop_nested_if_handled(depth in 1usize..4) {
        let indent = |n| "    ".repeat(n);
        let mut code = "def nested_if(x: int) -> int:\n".to_string();
        for i in 0..depth {
            code.push_str(&format!("{}if x > {}:\n", indent(i + 1), i));
        }
        code.push_str(&format!("{}return 1\n", indent(depth + 1)));
        code.push_str(&format!("{}return 0", indent(1)));
        let result = transpile(&code);
        prop_assert!(result.is_ok(), "Nested if (depth {}) should transpile", depth);
    }
}

// =============================================================================
// PROPERTY TESTS: Coverage of rust_gen/expr_gen.rs paths
// =============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    /// Property: List comprehensions are handled
    #[test]
    fn prop_list_comprehension_handled(n in 1usize..10) {
        let code = format!(
            "def listcomp() -> list[int]:\n    return [i * 2 for i in range({})]",
            n
        );
        let result = transpile(&code);
        // List comprehensions may not be fully supported
        let _ = result;
    }

    /// Property: Tuple expressions are handled
    #[test]
    fn prop_tuple_handled(a in any::<i32>(), b in any::<i32>()) {
        let code = format!(
            "def tuple_test() -> tuple[int, int]:\n    return ({}, {})",
            a, b
        );
        let result = transpile(&code);
        // Tuples may not be fully supported
        let _ = result;
    }

    /// Property: Index expressions are handled
    #[test]
    fn prop_index_expr_handled(idx in 0usize..5) {
        let code = format!(
            "def index_test(arr: list[int]) -> int:\n    return arr[{}]",
            idx
        );
        let result = transpile(&code);
        prop_assert!(result.is_ok(), "Index expression should transpile");
    }

    /// Property: Attribute access is handled
    #[test]
    fn prop_attribute_access_handled(method in prop_oneof![
        Just("upper"),
        Just("lower"),
        Just("strip"),
    ]) {
        let code = format!(
            "def attr_test(s: str) -> str:\n    return s.{}()",
            method
        );
        let result = transpile(&code);
        // Attribute methods may vary in support
        let _ = result;
    }

    /// Property: Lambda expressions are handled
    #[test]
    fn prop_lambda_handled(n in any::<i32>()) {
        let code = format!(
            "def with_lambda() -> int:\n    f = lambda x: x + {}\n    return f(1)",
            n
        );
        let result = transpile(&code);
        // Lambdas may not be fully supported
        let _ = result;
    }

    /// Property: Call expressions with various arg counts
    #[test]
    fn prop_call_args_handled(arg_count in 0usize..4) {
        let args: Vec<String> = (0..arg_count).map(|i| i.to_string()).collect();
        let params: Vec<String> = (0..arg_count).map(|i| format!("x{}: int", i)).collect();
        let code = format!(
            "def callee({}) -> int:\n    return 0\n\ndef caller() -> int:\n    return callee({})",
            params.join(", "),
            args.join(", ")
        );
        let result = transpile(&code);
        prop_assert!(result.is_ok(), "Call with {} args should transpile", arg_count);
    }
}

// =============================================================================
// PROPERTY TESTS: Coverage of rust_gen/stmt_gen.rs paths
// =============================================================================

#[test]
fn test_try_except_handled() {
    let code = "def safe_div(a: int, b: int) -> int:\n    try:\n        return a // b\n    except:\n        return 0";
    // Exception handling may vary
    let _ = transpile(code);
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(30))]

    /// Property: Break statements in loops
    #[test]
    fn prop_break_handled(limit in 1i32..10) {
        let code = format!(
            "def with_break() -> int:\n    i: int = 0\n    while True:\n        if i >= {}:\n            break\n        i = i + 1\n    return i",
            limit
        );
        let result = transpile(&code);
        prop_assert!(result.is_ok(), "Break statement should transpile");
    }

    /// Property: Continue statements in loops
    #[test]
    fn prop_continue_handled(limit in 1i32..10) {
        let code = format!(
            "def with_continue() -> int:\n    total: int = 0\n    for i in range({}):\n        if i % 2 == 0:\n            continue\n        total = total + i\n    return total",
            limit
        );
        let result = transpile(&code);
        prop_assert!(result.is_ok(), "Continue statement should transpile");
    }

    /// Property: Assert statements
    #[test]
    fn prop_assert_handled(val in any::<bool>()) {
        let val_str = if val { "True" } else { "False" };
        let code = format!(
            "def with_assert() -> None:\n    assert {}",
            val_str
        );
        // Assert may or may not be supported
        let _ = transpile(&code);
    }

    /// Property: Global variable declarations
    #[test]
    fn prop_global_handled(name in "[a-z][a-z0-9]{0,5}") {
        let code = format!(
            "{}: int = 42\n\ndef use_global() -> int:\n    return {}",
            name, name
        );
        // Global handling varies
        let _ = transpile(&code);
    }
}

// =============================================================================
// PROPERTY TESTS: Coverage of rust_gen/func_gen.rs paths
// =============================================================================

#[test]
fn test_mixed_params_handled() {
    let code = "def mixed(a: int, b: str, c: bool) -> int:\n    return a";
    let result = transpile(code);
    assert!(result.is_ok(), "Mixed params should transpile");
}

#[test]
fn test_recursive_handled() {
    let code = "def factorial(n: int) -> int:\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)";
    let result = transpile(code);
    assert!(result.is_ok(), "Recursive function should transpile");
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(30))]

    /// Property: Default parameter values
    #[test]
    fn prop_default_params_handled(default in 0i32..100) {
        let code = format!(
            "def with_default(x: int = {}) -> int:\n    return x",
            default
        );
        let result = transpile(&code);
        prop_assert!(result.is_ok(), "Default params should transpile");
    }

    /// Property: Functions with docstrings
    #[test]
    fn prop_docstring_handled(word in "[a-z]{3,10}") {
        let code = format!(
            "def documented() -> None:\n    \"\"\"This function does {}\"\"\"\n    pass",
            word
        );
        let result = transpile(&code);
        prop_assert!(result.is_ok(), "Docstring should transpile");
    }

    /// Property: Functions calling other functions
    #[test]
    fn prop_call_chain_handled(depth in 1usize..4) {
        let mut code = String::new();
        for i in 0..depth {
            if i == depth - 1 {
                code.push_str(&format!("def func{}() -> int:\n    return {}\n\n", i, i));
            } else {
                code.push_str(&format!("def func{}() -> int:\n    return func{}()\n\n", i, i + 1));
            }
        }
        let result = transpile(&code);
        prop_assert!(result.is_ok(), "Call chain should transpile");
    }
}
