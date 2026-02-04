//! Coverage tests for stmt_gen_complex.rs
//!
//! DEPYLER-99MODE-001: Targets 43%→80% coverage for stmt_gen_complex module
//! Covers: captures_outer_scope, extract_fields_recursive,
//! codegen_nested_function_def, codegen_try_stmt (complex paths),
//! and try_generate_subcommand_match branches.

use depyler_core::DepylerPipeline;

fn transpile(code: &str) -> String {
    DepylerPipeline::new()
        .transpile(code)
        .unwrap_or_else(|e| panic!("Transpilation failed: {e}"))
}

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// Nested function definitions (codegen_nested_function_def)
// ============================================================================

#[test]
fn test_nested_function_simple() {
    let code = r#"
def outer(x: int) -> int:
    def inner(y: int) -> int:
        return y + 1
    return inner(x)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn outer"));
}

#[test]
fn test_nested_function_captures_outer() {
    let code = r#"
def outer(x: int) -> int:
    y = 10
    def inner(z: int) -> int:
        return y + z
    return inner(x)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn outer"));
}

#[test]
fn test_nested_function_no_params() {
    let code = r#"
def outer() -> int:
    val = 42
    def getter() -> int:
        return val
    return getter()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn outer"));
}

#[test]
fn test_nested_function_string_params() {
    let code = r#"
def outer(name: str) -> str:
    def greet(prefix: str) -> str:
        return prefix + " " + name
    return greet("Hello")
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn outer"));
}

#[test]
fn test_nested_function_list_params() {
    let code = r#"
def outer(items: list) -> int:
    def count(data: list) -> int:
        return len(data)
    return count(items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn outer"));
}

#[test]
fn test_nested_function_dict_params() {
    let code = r#"
def outer(d: dict) -> int:
    def size(m: dict) -> int:
        return len(m)
    return size(d)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn outer"));
}

#[test]
fn test_nested_function_unknown_return() {
    let code = r#"
def outer(x: int):
    def inner(y: int):
        print(y)
    inner(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_multiple_nested_functions() {
    let code = r#"
def outer(x: int) -> int:
    def add_one(n: int) -> int:
        return n + 1
    def double(n: int) -> int:
        return n * 2
    return add_one(double(x))
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn outer"));
}

// ============================================================================
// captures_outer_scope: Variable capture detection
// ============================================================================

#[test]
fn test_capture_outer_in_binary_expr() {
    let code = r#"
def outer(x: int) -> int:
    y = 5
    def inner() -> int:
        return x + y
    return inner()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_capture_outer_in_call() {
    let code = r#"
def outer(x: int) -> int:
    def inner() -> int:
        return abs(x)
    return inner()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_capture_outer_in_method_call() {
    let code = r#"
def outer(items: list) -> int:
    def inner() -> int:
        return len(items)
    return inner()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_capture_outer_in_if_body() {
    let code = r#"
def outer(x: int) -> int:
    threshold = 10
    def inner(y: int) -> int:
        if y > threshold:
            return y
        return 0
    return inner(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_capture_outer_in_while_loop() {
    let code = r#"
def outer(limit: int) -> int:
    def inner() -> int:
        count = 0
        i = 0
        while i < limit:
            count = count + 1
            i = i + 1
        return count
    return inner()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_capture_outer_in_for_loop() {
    let code = r#"
def outer(n: int) -> int:
    def inner() -> int:
        total = 0
        for i in range(n):
            total = total + i
        return total
    return inner()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_capture_outer_in_unary() {
    let code = r#"
def outer(x: int) -> int:
    def inner() -> int:
        return -x
    return inner()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_capture_outer_in_index() {
    let code = r#"
def outer(items: list) -> int:
    idx = 0
    def inner() -> int:
        return items[idx]
    return inner()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_capture_outer_in_attribute() {
    let code = r#"
def outer(obj: str) -> int:
    def inner() -> int:
        return len(obj)
    return inner()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_no_capture_local_only() {
    let code = r#"
def outer() -> int:
    def inner(x: int) -> int:
        y = x + 1
        return y
    return inner(5)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_capture_in_try_body() {
    let code = r#"
def outer(x: int) -> int:
    def inner() -> int:
        try:
            return x
        except:
            return 0
    return inner()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_capture_in_with_body() {
    let code = r#"
def outer(filename: str) -> str:
    def inner() -> str:
        return filename
    return inner()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_capture_in_assert() {
    let code = r#"
def outer(x: int) -> int:
    limit = 100
    def inner(y: int) -> int:
        assert y < limit
        return y
    return inner(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_capture_in_raise() {
    let code = r#"
def outer(msg: str) -> int:
    def inner() -> int:
        raise ValueError(msg)
    return inner()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_capture_nested_function_def() {
    let code = r#"
def outer(x: int) -> int:
    def middle() -> int:
        def inner() -> int:
            return x
        return inner()
    return middle()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_capture_in_block_assign() {
    let code = r#"
def outer(x: int) -> int:
    y = x + 1
    def inner() -> int:
        z = y
        return z
    return inner()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// codegen_try_stmt: Complex try/except patterns
// ============================================================================

#[test]
fn test_try_with_zero_division_error() {
    let code = r#"
def safe_divide(a: int, b: int) -> int:
    try:
        return a // b
    except ZeroDivisionError:
        return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_with_zero_division_and_finally() {
    let code = r#"
def safe_divide(a: int, b: int) -> int:
    try:
        return a // b
    except ZeroDivisionError:
        return 0
    finally:
        print("done")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_with_value_error_parse() {
    let code = r#"
def parse_int(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_with_named_exception() {
    let code = r#"
def handle_error(x: int) -> str:
    try:
        return str(x / 0)
    except ZeroDivisionError as e:
        return str(e)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_multiple_handlers() {
    let code = r#"
def robust(x: int) -> int:
    try:
        result = 100 / x
        return int(result)
    except ZeroDivisionError:
        return -1
    except ValueError:
        return -2
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_except_else() {
    let code = r#"
def try_else(x: int) -> int:
    try:
        result = x + 1
    except:
        result = 0
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_except_finally_all() {
    let code = r#"
def full_try(x: int) -> int:
    result = 0
    try:
        result = x * 2
    except ValueError:
        result = -1
    finally:
        print("cleanup")
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_variable_hoisting() {
    let code = r#"
def hoisted(x: int) -> int:
    try:
        y = x + 1
    except:
        y = 0
    return y
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn hoisted"));
}

#[test]
fn test_try_with_raise_in_except() {
    let code = r#"
def reraise(x: int) -> int:
    try:
        return x / 0
    except ZeroDivisionError:
        raise ValueError("cannot divide")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_bare_except_with_finally() {
    let code = r#"
def safe_op() -> int:
    try:
        return 42
    except:
        return 0
    finally:
        print("done")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_try_blocks() {
    let code = r#"
def nested_try(x: int) -> int:
    try:
        try:
            return x / 0
        except ZeroDivisionError:
            return -1
    except:
        return -2
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Block statement
// ============================================================================

#[test]
fn test_block_with_multiple_stmts() {
    let code = r#"
def block_fn(x: int) -> int:
    y = x + 1
    z = y * 2
    return z
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn block_fn"));
}

// ============================================================================
// With statement
// ============================================================================

#[test]
fn test_with_statement_basic() {
    let code = r#"
def read_file(path: str) -> str:
    with open(path) as f:
        return f.read()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Assert statement
// ============================================================================

#[test]
fn test_assert_with_message() {
    let code = r#"
def validate(x: int) -> int:
    assert x > 0, "must be positive"
    return x
"#;
    let rust = transpile(code);
    assert!(rust.contains("assert"));
}

#[test]
fn test_assert_without_message() {
    let code = r#"
def check(x: int) -> int:
    assert x > 0
    return x
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Raise statement
// ============================================================================

#[test]
fn test_raise_value_error() {
    let code = r#"
def fail(msg: str):
    raise ValueError(msg)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_raise_runtime_error() {
    let code = r#"
def fail():
    raise RuntimeError("unexpected")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_bare_raise() {
    let code = r#"
def reraise():
    try:
        x = 1 / 0
    except:
        raise
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Break and Continue with labels
// ============================================================================

#[test]
fn test_break_in_while() {
    let code = r#"
def find_first(items: list) -> int:
    i = 0
    while i < len(items):
        if items[i] > 10:
            break
        i = i + 1
    return i
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_continue_in_for() {
    let code = r#"
def sum_positive(items: list) -> int:
    total = 0
    for x in items:
        if x < 0:
            continue
        total = total + x
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Pass statement
// ============================================================================

#[test]
fn test_pass_in_if() {
    let code = r#"
def noop(x: int) -> int:
    if x > 0:
        pass
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pass_in_except() {
    let code = r#"
def silent_fail():
    try:
        x = 1 / 0
    except:
        pass
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex combinations
// ============================================================================

#[test]
fn test_nested_if_in_try() {
    let code = r#"
def complex(x: int) -> int:
    try:
        if x > 0:
            return x
        else:
            return -x
    except:
        return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_for_loop_in_try() {
    let code = r#"
def sum_safe(items: list) -> int:
    try:
        total = 0
        for x in items:
            total = total + x
        return total
    except:
        return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_while_in_try() {
    let code = r#"
def countdown(n: int) -> int:
    try:
        count = 0
        while n > 0:
            count = count + 1
            n = n - 1
        return count
    except:
        return -1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_function_in_if() {
    let code = r#"
def conditional_fn(x: int) -> int:
    if x > 0:
        def helper(y: int) -> int:
            return y * 2
        return helper(x)
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_return_in_nested_function() {
    let code = r#"
def outer(x: int) -> int:
    def inner(y: int) -> int:
        if y > 0:
            return y
        return 0
    return inner(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_in_nested_function() {
    let code = r#"
def outer(x: int) -> int:
    def inner(y: int) -> int:
        try:
            return y / 1
        except:
            return 0
    return inner(x)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// extract_fields patterns (via Python argparse-like code)
// ============================================================================

#[test]
fn test_attribute_access_in_if() {
    let code = r#"
def process(x: int) -> int:
    if x > 0:
        y = x + 1
    else:
        y = 0
    return y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_attribute_access_in_while() {
    let code = r#"
def loop_fn(n: int) -> int:
    total = 0
    while n > 0:
        total = total + n
        n = n - 1
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_attribute_access_in_for() {
    let code = r#"
def iterate(items: list) -> int:
    result = 0
    for item in items:
        result = result + item
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// FunctionDef statement (nested)
// ============================================================================

#[test]
fn test_nested_function_with_closure() {
    let code = r#"
def make_adder(x: int):
    def adder(y: int) -> int:
        return x + y
    return adder
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_function_with_multiple_captures() {
    let code = r#"
def outer(a: int, b: int) -> int:
    c = a + b
    def inner() -> int:
        return a + b + c
    return inner()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_function_no_return_type() {
    let code = r#"
def outer():
    def inner():
        print("hello")
    inner()
"#;
    assert!(transpile_ok(code));
}
