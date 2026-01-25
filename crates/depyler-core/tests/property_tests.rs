//! Property-based tests for transpiler coverage
//!
//! These tests use proptest to generate many random inputs and exercise
//! more code paths than deterministic integration tests.

use depyler_core::DepylerPipeline;
use proptest::prelude::*;

// Generate valid Python identifiers
fn identifier() -> impl Strategy<Value = String> {
    prop::string::string_regex("[a-z][a-z0-9_]{0,10}")
        .unwrap()
        .prop_filter("not keyword", |s| {
            !matches!(
                s.as_str(),
                "and"
                    | "as"
                    | "assert"
                    | "async"
                    | "await"
                    | "break"
                    | "class"
                    | "continue"
                    | "def"
                    | "del"
                    | "elif"
                    | "else"
                    | "except"
                    | "finally"
                    | "for"
                    | "from"
                    | "global"
                    | "if"
                    | "import"
                    | "in"
                    | "is"
                    | "lambda"
                    | "nonlocal"
                    | "not"
                    | "or"
                    | "pass"
                    | "raise"
                    | "return"
                    | "try"
                    | "while"
                    | "with"
                    | "yield"
                    | "None"
                    | "True"
                    | "False"
            )
        })
}

// Generate valid integers
fn py_int() -> impl Strategy<Value = i64> {
    prop::num::i64::ANY
}

// Generate valid floats (avoid NaN and inf)
fn py_float() -> impl Strategy<Value = f64> {
    prop::num::f64::NORMAL
}

// Generate simple Python expressions
fn simple_expr() -> impl Strategy<Value = String> {
    prop_oneof![
        py_int().prop_map(|n| n.to_string()),
        py_float().prop_map(|f| format!("{:.6}", f)),
        Just("True".to_string()),
        Just("False".to_string()),
        Just("None".to_string()),
        Just("\"hello\"".to_string()),
        Just("'world'".to_string()),
        Just("[]".to_string()),
        Just("{}".to_string()),
        Just("[1, 2, 3]".to_string()),
        Just("{'a': 1}".to_string()),
        Just("(1, 2)".to_string()),
    ]
}

// Generate binary operators
fn binop() -> impl Strategy<Value = &'static str> {
    prop_oneof![
        Just("+"),
        Just("-"),
        Just("*"),
        Just("/"),
        Just("//"),
        Just("%"),
        Just("**"),
        Just("&"),
        Just("|"),
        Just("^"),
        Just("<<"),
        Just(">>"),
        Just("and"),
        Just("or"),
        Just("=="),
        Just("!="),
        Just("<"),
        Just(">"),
        Just("<="),
        Just(">="),
    ]
}

// Generate comparison operators
#[allow(dead_code)] // Reserved for future comparison chain tests
fn cmpop() -> impl Strategy<Value = &'static str> {
    prop_oneof![
        Just("=="),
        Just("!="),
        Just("<"),
        Just(">"),
        Just("<="),
        Just(">="),
        Just("in"),
        Just("not in"),
        Just("is"),
        Just("is not"),
    ]
}

// Generate augmented assignment operators
fn augop() -> impl Strategy<Value = &'static str> {
    prop_oneof![
        Just("+="),
        Just("-="),
        Just("*="),
        Just("/="),
        Just("//="),
        Just("%="),
        Just("**="),
        Just("&="),
        Just("|="),
        Just("^="),
        Just("<<="),
        Just(">>="),
    ]
}

// Helper to check if transpilation succeeds
fn transpiles_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// PROPERTY-BASED TESTS
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    // Test that simple assignments transpile
    #[test]
    fn test_simple_assignment(name in identifier(), expr in simple_expr()) {
        let code = format!("{} = {}", name, expr);
        // We don't require success, just that it doesn't panic
        let _ = DepylerPipeline::new().transpile(&code);
    }

    // Test binary expressions
    #[test]
    fn test_binary_expr(left in py_int(), op in binop(), right in 1i64..100) {
        let code = format!("x = {} {} {}", left, op, right);
        let _ = DepylerPipeline::new().transpile(&code);
    }

    // Test augmented assignments
    #[test]
    fn test_augmented_assign(op in augop(), val in 1i64..100) {
        let code = format!("def f():\n    x = 10\n    x {} {}", op, val);
        let _ = DepylerPipeline::new().transpile(&code);
    }

    // Test simple functions
    #[test]
    fn test_simple_function(name in identifier(), ret in simple_expr()) {
        let code = format!("def {}():\n    return {}", name, ret);
        let _ = DepylerPipeline::new().transpile(&code);
    }

    // Test function with parameters
    #[test]
    fn test_function_params(
        name in identifier(),
        p1 in identifier(),
        p2 in identifier(),
    ) {
        if p1 != p2 && name != p1 && name != p2 {
            let code = format!("def {}({}, {}):\n    return {} + {}", name, p1, p2, p1, p2);
            let _ = DepylerPipeline::new().transpile(&code);
        }
    }

    // Test if statements
    #[test]
    fn test_if_stmt(cond in 0i64..10, then_val in py_int(), else_val in py_int()) {
        let code = format!(
            "def f(x):\n    if x > {}:\n        return {}\n    else:\n        return {}",
            cond, then_val, else_val
        );
        let _ = DepylerPipeline::new().transpile(&code);
    }

    // Test for loops with range
    #[test]
    fn test_for_range(start in 0i64..10, end in 10i64..20) {
        let code = format!(
            "def f():\n    total = 0\n    for i in range({}, {}):\n        total += i\n    return total",
            start, end
        );
        let _ = DepylerPipeline::new().transpile(&code);
    }

    // Test while loops
    #[test]
    fn test_while_loop(limit in 1i64..20) {
        let code = format!(
            "def f():\n    x = 0\n    while x < {}:\n        x += 1\n    return x",
            limit
        );
        let _ = DepylerPipeline::new().transpile(&code);
    }

    // Test list comprehensions
    #[test]
    fn test_list_comprehension(n in 1i64..20, mul in 1i64..5) {
        let code = format!("x = [i * {} for i in range({})]", mul, n);
        let _ = DepylerPipeline::new().transpile(&code);
    }

    // Test dict comprehensions
    #[test]
    fn test_dict_comprehension(n in 1i64..10) {
        let code = format!("x = {{i: i * 2 for i in range({})}}", n);
        let _ = DepylerPipeline::new().transpile(&code);
    }

    // Test try/except
    #[test]
    fn test_try_except(val in py_int(), default in py_int()) {
        let code = format!(
            "def f():\n    try:\n        return {}\n    except:\n        return {}",
            val, default
        );
        let _ = DepylerPipeline::new().transpile(&code);
    }

    // Test class with fields
    #[test]
    fn test_class_init(class_name in identifier(), field1 in identifier(), field2 in identifier()) {
        if class_name != field1 && class_name != field2 && field1 != field2 {
            let code = format!(
                "class {}:\n    def __init__(self, {}, {}):\n        self.{} = {}\n        self.{} = {}",
                class_name, field1, field2, field1, field1, field2, field2
            );
            let _ = DepylerPipeline::new().transpile(&code);
        }
    }

    // Test lambdas
    #[test]
    fn test_lambda_expr(mul in 1i64..10, add in 0i64..10) {
        let code = format!("f = lambda x: x * {} + {}", mul, add);
        let _ = DepylerPipeline::new().transpile(&code);
    }

    // Test ternary expressions
    #[test]
    fn test_ternary(threshold in 0i64..20, then_val in py_int(), else_val in py_int()) {
        let code = format!(
            "def f(x):\n    return {} if x > {} else {}",
            then_val, threshold, else_val
        );
        let _ = DepylerPipeline::new().transpile(&code);
    }

    // Test f-strings
    #[test]
    fn test_fstring(val in py_int()) {
        let code = format!("x = f'value: {{{}}}'", val);
        let _ = DepylerPipeline::new().transpile(&code);
    }

    // Test nested structures
    #[test]
    fn test_nested_list(depth in 1usize..4) {
        let inner = "1".to_string();
        let mut result = inner;
        for _ in 0..depth {
            result = format!("[{}]", result);
        }
        let code = format!("x = {}", result);
        let _ = DepylerPipeline::new().transpile(&code);
    }

    // Test method chains
    #[test]
    fn test_method_chain(n in 1usize..5) {
        let methods = ["strip", "lower", "upper"];
        let mut code = "'hello'".to_string();
        for i in 0..n {
            code = format!("{}.{}()", code, methods[i % methods.len()]);
        }
        let code = format!("x = {}", code);
        let _ = DepylerPipeline::new().transpile(&code);
    }

    // Test comparison chains
    #[test]
    fn test_comparison_chain(a in 0i64..10, b in 10i64..20, c in 20i64..30) {
        // Use b for a different chain pattern
        let code = format!("def f(x):\n    return {} < {} < x < {}", a, b, c);
        let _ = DepylerPipeline::new().transpile(&code);
    }

    // Test slicing
    #[test]
    fn test_slice(start in 0i64..5, end in 5i64..10) {
        let code = format!("def f(x):\n    return x[{}:{}]", start, end);
        let _ = DepylerPipeline::new().transpile(&code);
    }

    // Test generators
    #[test]
    fn test_generator(n in 1i64..20) {
        let code = format!(
            "def gen(n):\n    for i in range({}):\n        yield i",
            n
        );
        let _ = DepylerPipeline::new().transpile(&code);
    }

    // Test dataclass
    #[test]
    fn test_dataclass_fields(
        class_name in identifier(),
        f1 in identifier(),
        f2 in identifier(),
    ) {
        if class_name != f1 && class_name != f2 && f1 != f2 {
            let code = format!(
                "from dataclasses import dataclass\n\n@dataclass\nclass {}:\n    {}: int\n    {}: str",
                class_name, f1, f2
            );
            let _ = DepylerPipeline::new().transpile(&code);
        }
    }

    // Test match statements
    #[test]
    fn test_match_int(val1 in py_int(), val2 in py_int()) {
        let code = format!(
            "def f(x):\n    match x:\n        case {}:\n            return 'a'\n        case {}:\n            return 'b'\n        case _:\n            return 'c'",
            val1, val2
        );
        let _ = DepylerPipeline::new().transpile(&code);
    }

    // Test async functions
    #[test]
    fn test_async_function(name in identifier(), ret in py_int()) {
        let code = format!("async def {}():\n    return {}", name, ret);
        let _ = DepylerPipeline::new().transpile(&code);
    }
}

// Additional deterministic edge case tests
#[test]
fn test_empty_function() {
    assert!(transpiles_ok("def f(): pass"));
}

#[test]
fn test_empty_class() {
    assert!(transpiles_ok("class C: pass"));
}

#[test]
fn test_multiline_string() {
    assert!(transpiles_ok("x = '''multiline\nstring'''"));
}

#[test]
fn test_raw_string() {
    assert!(transpiles_ok("x = r'raw\\nstring'"));
}

#[test]
fn test_bytes_literal() {
    assert!(transpiles_ok("x = b'bytes'"));
}

#[test]
fn test_large_int() {
    assert!(transpiles_ok("x = 99999999999999999999"));
}

#[test]
fn test_scientific_notation() {
    assert!(transpiles_ok("x = 1e10"));
}

#[test]
fn test_negative_scientific() {
    assert!(transpiles_ok("x = -1.5e-10"));
}

#[test]
fn test_hex_literal() {
    assert!(transpiles_ok("x = 0xFF"));
}

#[test]
fn test_octal_literal() {
    assert!(transpiles_ok("x = 0o77"));
}

#[test]
fn test_binary_literal() {
    assert!(transpiles_ok("x = 0b1010"));
}

#[test]
fn test_underscore_number() {
    assert!(transpiles_ok("x = 1_000_000"));
}

#[test]
fn test_complex_literal() {
    // Complex numbers may or may not be supported
    let _ = DepylerPipeline::new().transpile("x = 1 + 2j");
}

#[test]
fn test_ellipsis() {
    assert!(transpiles_ok("x = ..."));
}

#[test]
fn test_walrus_in_while() {
    let _ = DepylerPipeline::new().transpile(
        "def f(items):\n    while (item := items.pop()) is not None:\n        print(item)",
    );
}

#[test]
fn test_deeply_nested_if() {
    let code = "def f(a, b, c, d):\n    if a:\n        if b:\n            if c:\n                if d:\n                    return 1\n    return 0";
    assert!(transpiles_ok(code));
}

#[test]
fn test_many_elif() {
    let code = "def f(x):\n    if x == 0:\n        return 'a'\n    elif x == 1:\n        return 'b'\n    elif x == 2:\n        return 'c'\n    elif x == 3:\n        return 'd'\n    elif x == 4:\n        return 'e'\n    else:\n        return 'f'";
    assert!(transpiles_ok(code));
}

#[test]
fn test_tuple_in_for() {
    assert!(transpiles_ok(
        "def f():\n    for a, b, c in [(1, 2, 3)]:\n        print(a, b, c)"
    ));
}

#[test]
fn test_star_import() {
    // Star imports may or may not be supported
    let _ = DepylerPipeline::new().transpile("from os import *");
}

#[test]
fn test_relative_import() {
    // Relative imports may or may not be supported
    let _ = DepylerPipeline::new().transpile("from . import module");
}

#[test]
fn test_decorator_with_args() {
    let code = "def deco(arg):\n    def wrapper(f):\n        return f\n    return wrapper\n\n@deco('test')\ndef f(): pass";
    let _ = DepylerPipeline::new().transpile(code);
}

#[test]
fn test_multiple_decorators() {
    let code = "@staticmethod\n@property\ndef f(): pass";
    let _ = DepylerPipeline::new().transpile(code);
}

#[test]
fn test_class_with_bases() {
    let code = "class A: pass\nclass B: pass\nclass C(A, B): pass";
    let _ = DepylerPipeline::new().transpile(code);
}

#[test]
fn test_metaclass() {
    let code = "class Meta(type): pass\nclass C(metaclass=Meta): pass";
    let _ = DepylerPipeline::new().transpile(code);
}
