//! EXTREME TDD tests for stmt_gen module
//! Tests edge cases, error paths, and boundary conditions using property-based testing

use depyler_core::ast_bridge::AstBridge;
use depyler_core::codegen::hir_to_rust;
use proptest::prelude::*;
use rustpython_ast::Suite;
use rustpython_parser::Parse;

// ============================================================================
// Helper Functions
// ============================================================================

/// Helper to create a ModModule from parsed code
fn make_module(ast: Suite) -> rustpython_ast::Mod {
    rustpython_ast::Mod::Module(rustpython_ast::ModModule {
        body: ast,
        range: Default::default(),
        type_ignores: vec![],
    })
}

/// Transpile Python code to Rust and return the result
fn transpile_code(python_code: &str) -> Option<String> {
    let ast = Suite::parse(python_code, "<test>").ok()?;
    let bridge = AstBridge::new().with_source(python_code.to_string());
    let (hir, _type_env) = bridge.python_to_hir(make_module(ast)).ok()?;
    let rust_code = hir_to_rust(&hir).ok()?;
    Some(rust_code)
}

/// Check if Python code transpiles successfully
fn transpile_succeeds(python_code: &str) -> bool {
    transpile_code(python_code).is_some()
}

// ============================================================================
// FALSIFICATION TESTS - Try to break statement generation
// ============================================================================

#[test]
fn test_empty_if_body() {
    let code = r#"
def check(x: int) -> None:
    if x > 0:
        pass
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_if_with_only_ellipsis() {
    let code = r#"
def check(x: int) -> None:
    if x > 0:
        ...
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_nested_if_statements() {
    let code = r#"
def nested(a: int, b: int, c: int) -> int:
    if a > 0:
        if b > 0:
            if c > 0:
                return a + b + c
    return 0
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_deeply_nested_if_else() {
    let code = r#"
def deep(x: int) -> str:
    if x == 0:
        return "zero"
    elif x == 1:
        return "one"
    elif x == 2:
        return "two"
    elif x == 3:
        return "three"
    else:
        return "many"
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_while_with_break() {
    let code = r#"
def find_first(n: int, target: int) -> int:
    i = 0
    while i < n:
        if i == target:
            break
        i = i + 1
    return i
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_while_with_continue() {
    let code = r#"
def sum_even(n: int) -> int:
    total = 0
    i = 0
    while i < n:
        i = i + 1
        if i % 2 != 0:
            continue
        total = total + i
    return total
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_for_with_enumerate() {
    let code = r#"
def print_indexed(n: int) -> int:
    total = 0
    for i, val in enumerate(range(n)):
        total = total + i + val
    return total
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_for_with_range_step() {
    let code = r#"
def every_third(n: int) -> int:
    result = 0
    for i in range(0, n, 3):
        result = result + i
    return result
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_for_with_negative_range() {
    let code = r#"
def countdown(n: int) -> int:
    result = 0
    for i in range(n, 0, -1):
        result = result + i
    return result
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_try_except_basic() {
    let code = r#"
def safe_div(a: int, b: int) -> int:
    try:
        return a // b
    except ZeroDivisionError:
        return 0
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_try_except_multiple_handlers() {
    let code = r#"
def safe_op(a: int, b: int) -> int:
    try:
        return a // b
    except ZeroDivisionError:
        return -1
    except ValueError:
        return -2
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_try_except_finally() {
    let code = r#"
def with_cleanup(x: int) -> int:
    result = 0
    try:
        result = x * 2
    finally:
        pass
    return result
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_raise_statement() {
    let code = r#"
def validate(x: int) -> int:
    if x < 0:
        raise ValueError("must be positive")
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_assert_statement() {
    let code = r#"
def check_positive(x: int) -> int:
    assert x > 0
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_assert_with_message() {
    let code = r#"
def check_positive(x: int) -> int:
    assert x > 0, "x must be positive"
    return x
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// EDGE CASE TESTS - Boundary conditions
// ============================================================================

#[test]
fn test_assignment_to_underscore() {
    let code = r#"
def ignore_result() -> int:
    _ = 42
    return 0
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_tuple_unpacking() {
    let code = r#"
def unpack() -> int:
    a, b = 1, 2
    return a + b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_augmented_assignment_add() {
    let code = r#"
def aug_add(x: int) -> int:
    x += 1
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_augmented_assignment_sub() {
    let code = r#"
def aug_sub(x: int) -> int:
    x -= 1
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_augmented_assignment_mul() {
    let code = r#"
def aug_mul(x: int) -> int:
    x *= 2
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_augmented_assignment_div() {
    let code = r#"
def aug_div(x: float) -> float:
    x /= 2.0
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_augmented_assignment_floor_div() {
    let code = r#"
def aug_floor_div(x: int) -> int:
    x //= 2
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_augmented_assignment_mod() {
    let code = r#"
def aug_mod(x: int) -> int:
    x %= 3
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_augmented_assignment_pow() {
    let code = r#"
def aug_pow(x: int) -> int:
    x **= 2
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_augmented_assignment_and() {
    let code = r#"
def aug_and(x: int) -> int:
    x &= 255
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_augmented_assignment_or() {
    let code = r#"
def aug_or(x: int) -> int:
    x |= 1
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_augmented_assignment_xor() {
    let code = r#"
def aug_xor(x: int) -> int:
    x ^= 255
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_augmented_assignment_lshift() {
    let code = r#"
def aug_lshift(x: int) -> int:
    x <<= 2
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_augmented_assignment_rshift() {
    let code = r#"
def aug_rshift(x: int) -> int:
    x >>= 2
    return x
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// RETURN STATEMENT TESTS
// ============================================================================

#[test]
fn test_return_none_explicit() {
    let code = r#"
def return_none() -> None:
    return None
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_return_none_implicit() {
    let code = r#"
def return_none() -> None:
    return
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_return_tuple() {
    let code = r#"
def return_tuple() -> tuple:
    return (1, 2, 3)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_return_list() {
    let code = r#"
def return_list() -> list:
    return [1, 2, 3]
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_return_dict() {
    let code = r#"
def return_dict() -> dict:
    return {"a": 1, "b": 2}
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_early_return() {
    let code = r#"
def early_return(x: int) -> int:
    if x < 0:
        return -1
    if x == 0:
        return 0
    return 1
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// COMPREHENSION TESTS
// ============================================================================

#[test]
fn test_list_comprehension_simple() {
    let code = r#"
def squares(n: int) -> list:
    return [x * x for x in range(n)]
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_list_comprehension_with_condition() {
    let code = r#"
def even_squares(n: int) -> list:
    return [x * x for x in range(n) if x % 2 == 0]
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_dict_comprehension() {
    let code = r#"
def square_dict(n: int) -> dict:
    return {x: x * x for x in range(n)}
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_set_comprehension() {
    let code = r#"
def unique_squares(n: int) -> set:
    return {x * x for x in range(n)}
"#;
    assert!(transpile_succeeds(code));
}

// Generator expressions are not yet supported - this tests that graceful failure
#[test]
fn test_generator_expression_unsupported() {
    let code = r#"
def sum_squares(n: int) -> int:
    return sum(x * x for x in range(n))
"#;
    // Generator expressions fail gracefully (return None, don't panic)
    let _result = transpile_code(code);
    // The important thing is no panic - the feature is not yet supported
}

// ============================================================================
// LAMBDA TESTS
// ============================================================================

#[test]
fn test_lambda_simple() {
    let code = r#"
def use_lambda() -> int:
    f = lambda x: x + 1
    return f(5)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_lambda_multiple_args() {
    let code = r#"
def use_lambda() -> int:
    f = lambda x, y: x + y
    return f(2, 3)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_lambda_no_args() {
    let code = r#"
def use_lambda() -> int:
    f = lambda: 42
    return f()
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// FLOAT INFERENCE TESTS
// ============================================================================

#[test]
fn test_float_literal() {
    let code = r#"
def get_float() -> float:
    return 3.14
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_float_from_div() {
    let code = r#"
def divide(a: int, b: int) -> float:
    return a / b
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("f64") || result.contains("as f64"));
}

#[test]
fn test_float_binary_ops() {
    let code = r#"
def float_ops(x: float, y: float) -> float:
    return x * y + x - y
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// MATCH STATEMENT TESTS
// ============================================================================

#[test]
fn test_match_simple() {
    let code = r#"
def match_num(x: int) -> str:
    match x:
        case 0:
            return "zero"
        case 1:
            return "one"
        case _:
            return "other"
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// GLOBAL AND NONLOCAL TESTS
// ============================================================================

#[test]
fn test_global_variable() {
    let code = r#"
COUNTER = 0

def increment() -> int:
    global COUNTER
    COUNTER = COUNTER + 1
    return COUNTER
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// PROPERTY-BASED TESTS
// ============================================================================

proptest! {
    /// Property: All augmented assignments should transpile
    #[test]
    fn prop_augmented_assignment_transpiles(x in -1000i64..1000) {
        let code = format!(r#"
def aug(n: int) -> int:
    n += {}
    return n
"#, x);
        prop_assert!(transpile_succeeds(&code));
    }

    /// Property: For loops over range should transpile
    #[test]
    fn prop_for_range_transpiles(end in 1usize..100) {
        let code = format!(r#"
def loop() -> int:
    total = 0
    for i in range({}):
        total += i
    return total
"#, end);
        prop_assert!(transpile_succeeds(&code));
    }

    /// Property: Binary operators with ints should transpile
    #[test]
    fn prop_binary_ops_transpile(a in -100i64..100, b in 1i64..100) {
        let code = format!(r#"
def binary_ops() -> int:
    return {} + {} - {} * {} // {}
"#, a, b, a, b, b);
        prop_assert!(transpile_succeeds(&code));
    }
}

// ============================================================================
// MUTATION-RESISTANT TESTS
// ============================================================================

#[test]
fn test_if_else_exact_branches() {
    let code = r#"
def branch(x: int) -> int:
    if x > 0:
        return 1
    else:
        return -1
"#;
    let result = transpile_code(code).unwrap();
    // Must have both if and else branches in output
    assert!(result.contains("if"));
    assert!(result.contains("else"));
}

#[test]
fn test_while_loop_has_condition() {
    let code = r#"
def countdown(n: int) -> int:
    while n > 0:
        n = n - 1
    return n
"#;
    let result = transpile_code(code).unwrap();
    // Must have while loop structure
    assert!(result.contains("while"));
}

#[test]
fn test_for_loop_has_iterator() {
    let code = r#"
def iterate() -> int:
    total = 0
    for i in range(10):
        total = total + i
    return total
"#;
    let result = transpile_code(code).unwrap();
    // Must have for loop structure
    assert!(result.contains("for"));
}

#[test]
fn test_return_preserves_value() {
    let code = r#"
def constant() -> int:
    return 42
"#;
    let result = transpile_code(code).unwrap();
    // Must preserve the return value
    assert!(result.contains("42"));
}

#[test]
fn test_assignment_preserves_name() {
    let code = r#"
def assign() -> int:
    my_special_variable = 100
    return my_special_variable
"#;
    let result = transpile_code(code).unwrap();
    // Must preserve variable name
    assert!(result.contains("my_special_variable"));
}
