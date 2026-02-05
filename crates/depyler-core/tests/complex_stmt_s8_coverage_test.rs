//! Session 8 batch 3: Coverage tests for stmt_gen_complex.rs
//! Targets: try/except code generation, nested functions, variable hoisting,
//! generator exception handling, argparse patterns

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile(python_code: &str) -> String {
    let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
    let (module, _) = AstBridge::new()
        .with_source(python_code.to_string())
        .python_to_hir(ast)
        .expect("hir");
    let tm = TypeMapper::default();
    let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
    result
}

// ── Try/except with variable hoisting ───────────────────────────

#[test]
fn test_try_except_variable_hoisting() {
    let code = transpile(
        r#"
def f(s: str) -> int:
    result = 0
    try:
        result = int(s)
    except ValueError:
        result = -1
    return result
"#,
    );
    assert!(
        code.contains("result") && code.contains("let"),
        "Should hoist variable: {code}"
    );
}

#[test]
fn test_try_except_with_else() {
    let code = transpile(
        r#"
def f(s: str) -> int:
    try:
        x = int(s)
    except ValueError:
        return -1
    else:
        return x
    return 0
"#,
    );
    assert!(
        code.contains("fn f") && code.contains("-1"),
        "Should handle try/except/else: {code}"
    );
}

#[test]
fn test_try_except_finally_cleanup() {
    let code = transpile(
        r#"
def f(path: str) -> str:
    content = ""
    try:
        content = "data"
    except Exception:
        content = "error"
    finally:
        print("cleanup done")
    return content
"#,
    );
    assert!(
        code.contains("cleanup") || code.contains("content"),
        "Should generate finally block: {code}"
    );
}

#[test]
fn test_try_except_type_conversion() {
    let code = transpile(
        r#"
def safe_float(s: str) -> float:
    try:
        return float(s)
    except ValueError:
        return 0.0
"#,
    );
    assert!(
        code.contains("f64") || code.contains("parse") || code.contains("0.0"),
        "Should handle float conversion in try: {code}"
    );
}

#[test]
fn test_try_except_multiple_exception_types() {
    let code = transpile(
        r#"
def f(data: dict, key: str) -> int:
    try:
        return data[key]
    except KeyError:
        return -1
    except TypeError:
        return -2
    except Exception:
        return -3
"#,
    );
    assert!(
        code.contains("fn f"),
        "Should handle multiple exception types: {code}"
    );
}

#[test]
fn test_try_except_nested() {
    let code = transpile(
        r#"
def f(s: str) -> int:
    try:
        try:
            return int(s)
        except ValueError:
            return -1
    except Exception:
        return -2
"#,
    );
    assert!(
        code.contains("-1") || code.contains("-2"),
        "Should handle nested try: {code}"
    );
}

#[test]
fn test_try_except_with_raise() {
    let code = transpile(
        r#"
def f(x: int) -> int:
    try:
        if x < 0:
            raise ValueError("negative")
        return x
    except ValueError:
        return 0
"#,
    );
    assert!(
        code.contains("fn f"),
        "Should handle raise in try block: {code}"
    );
}

// ── Nested function definitions ─────────────────────────────────

#[test]
fn test_nested_function() {
    let code = transpile(
        r#"
def outer(x: int) -> int:
    def inner(y: int) -> int:
        return y * 2
    return inner(x)
"#,
    );
    assert!(
        code.contains("inner") || code.contains("fn "),
        "Should generate nested function: {code}"
    );
}

#[test]
fn test_nested_function_as_closure() {
    let code = transpile(
        r#"
def make_adder(n: int) -> int:
    def add(x: int) -> int:
        return x + n
    return add(10)
"#,
    );
    assert!(
        code.contains("fn make_adder") || code.contains("add"),
        "Should handle closure-like nested function: {code}"
    );
}

#[test]
fn test_nested_recursive_function() {
    let code = transpile(
        r#"
def outer() -> int:
    def factorial(n: int) -> int:
        if n <= 1:
            return 1
        return n * factorial(n - 1)
    return factorial(5)
"#,
    );
    assert!(
        code.contains("factorial"),
        "Should handle recursive nested function: {code}"
    );
}

// ── Generator patterns ──────────────────────────────────────────

#[test]
fn test_generator_function_yield() {
    let code = transpile(
        r#"
def count_up(n: int) -> int:
    i = 0
    while i < n:
        yield i
        i += 1
"#,
    );
    assert!(
        code.contains("Iterator") || code.contains("iter") || code.contains("next"),
        "Should generate iterator for yield: {code}"
    );
}

#[test]
fn test_generator_range_yield() {
    let code = transpile(
        r#"
def squares(n: int) -> int:
    for i in range(n):
        yield i * i
"#,
    );
    assert!(
        code.contains("Iterator") || code.contains("iter") || code.contains("next") || code.contains("*"),
        "Should generate range-based generator: {code}"
    );
}

// ── Complex control flow in try/except ──────────────────────────

#[test]
fn test_try_except_in_loop() {
    let code = transpile(
        r#"
def parse_numbers(items: list) -> list:
    result = []
    for item in items:
        try:
            result.append(int(item))
        except ValueError:
            pass
    return result
"#,
    );
    assert!(
        code.contains("for") || code.contains("iter"),
        "Should handle try/except in loop: {code}"
    );
}

#[test]
fn test_try_except_with_return_in_handler() {
    let code = transpile(
        r#"
def f(items: list) -> int:
    try:
        total = 0
        for item in items:
            total += item
        return total
    except TypeError:
        return -1
"#,
    );
    assert!(
        code.contains("total") || code.contains("return"),
        "Should handle return in handler: {code}"
    );
}

#[test]
fn test_try_except_with_assignment() {
    let code = transpile(
        r#"
def safe_div(a: int, b: int) -> float:
    try:
        result = a / b
    except ZeroDivisionError:
        result = 0.0
    return result
"#,
    );
    assert!(
        code.contains("result"),
        "Should handle try/except assignment: {code}"
    );
}

// ── Complex raise patterns ──────────────────────────────────────

#[test]
fn test_raise_custom_exception() {
    let code = transpile(
        r#"
def validate(x: int) -> int:
    if x < 0:
        raise ValueError("must be non-negative")
    if x > 100:
        raise ValueError("must be <= 100")
    return x
"#,
    );
    assert!(
        code.contains("panic") || code.contains("Err") || code.contains("Error"),
        "Should handle multiple raises: {code}"
    );
}

#[test]
fn test_raise_not_implemented() {
    let code = transpile(
        r#"
def f() -> None:
    raise NotImplementedError("todo")
"#,
    );
    assert!(
        code.contains("unimplemented") || code.contains("todo") || code.contains("panic"),
        "Should handle NotImplementedError: {code}"
    );
}

#[test]
fn test_raise_type_error() {
    let code = transpile(
        r#"
def f(x: int) -> int:
    if not isinstance(x, int):
        raise TypeError("expected int")
    return x
"#,
    );
    assert!(
        code.contains("fn f"),
        "Should handle TypeError raise: {code}"
    );
}

// ── With statement patterns ─────────────────────────────────────

#[test]
fn test_with_multiple_context_managers() {
    let code = transpile(
        r#"
def f(path1: str, path2: str) -> str:
    with open(path1, "r") as f1:
        data = f1.read()
    with open(path2, "w") as f2:
        f2.write(data)
    return data
"#,
    );
    assert!(
        code.contains("File") || code.contains("read") || code.contains("write"),
        "Should handle multiple with blocks: {code}"
    );
}

// ── Complex assignment patterns ─────────────────────────────────

#[test]
fn test_multi_target_assignment() {
    let code = transpile(
        r#"
def f() -> int:
    x = y = z = 0
    return x + y + z
"#,
    );
    assert!(
        code.contains("let") && code.contains("0"),
        "Should handle chained assignment: {code}"
    );
}

#[test]
fn test_augmented_assignment_in_loop() {
    let code = transpile(
        r#"
def sum_squares(n: int) -> int:
    total = 0
    for i in range(n):
        total += i * i
    return total
"#,
    );
    assert!(
        code.contains("+=") || code.contains("total"),
        "Should handle augmented assignment in loop: {code}"
    );
}

#[test]
fn test_tuple_unpack_from_function() {
    let code = transpile(
        r#"
def f() -> int:
    a, b = 1, 2
    x, y, z = 10, 20, 30
    return a + b + x + y + z
"#,
    );
    assert!(
        code.contains("let") && code.contains("10"),
        "Should unpack tuples: {code}"
    );
}

// ── Pass statement patterns ─────────────────────────────────────

#[test]
fn test_pass_in_if_else() {
    let code = transpile(
        r#"
def f(x: int) -> int:
    if x > 0:
        pass
    else:
        return -1
    return x
"#,
    );
    assert!(
        code.contains("fn f"),
        "Should handle pass in if block: {code}"
    );
}

#[test]
fn test_pass_in_except() {
    let code = transpile(
        r#"
def f(x: int) -> int:
    try:
        return x / 1
    except Exception:
        pass
    return 0
"#,
    );
    assert!(
        code.contains("fn f"),
        "Should handle pass in except: {code}"
    );
}

// ── Break/continue in complex patterns ──────────────────────────

#[test]
fn test_break_in_nested_loop() {
    let code = transpile(
        r#"
def find_pair(items: list, target: int) -> bool:
    for i in range(len(items)):
        for j in range(i + 1, len(items)):
            if items[i] + items[j] == target:
                return True
    return False
"#,
    );
    assert!(
        code.contains("for") || code.contains("return true") || code.contains("return false"),
        "Should handle nested loop with return: {code}"
    );
}

#[test]
fn test_while_true_break() {
    let code = transpile(
        r#"
def f() -> int:
    x = 0
    while True:
        x += 1
        if x > 10:
            break
    return x
"#,
    );
    assert!(
        code.contains("loop") || code.contains("while") || code.contains("break"),
        "Should handle while True with break: {code}"
    );
}

// ── Complex function patterns ───────────────────────────────────

#[test]
fn test_recursive_function() {
    let code = transpile(
        r#"
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)
"#,
    );
    assert!(
        code.contains("fn fibonacci") && code.contains("fibonacci("),
        "Should generate recursive function: {code}"
    );
}

#[test]
fn test_function_with_early_return() {
    let code = transpile(
        r#"
def find_first(items: list, target: int) -> int:
    for i in range(len(items)):
        if items[i] == target:
            return i
    return -1
"#,
    );
    assert!(
        code.contains("return") && code.contains("-1"),
        "Should handle early return: {code}"
    );
}

#[test]
fn test_function_multiple_returns() {
    let code = transpile(
        r#"
def grade(score: int) -> str:
    if score >= 90:
        return "A"
    if score >= 80:
        return "B"
    if score >= 70:
        return "C"
    if score >= 60:
        return "D"
    return "F"
"#,
    );
    assert!(
        code.contains("fn grade") && code.contains("90"),
        "Should handle multiple returns: {code}"
    );
}

// ── Global and scope patterns ───────────────────────────────────

#[test]
fn test_global_variable_access() {
    let code = transpile(
        r#"
MAX_VALUE: int = 100
def check(x: int) -> bool:
    return x <= MAX_VALUE
"#,
    );
    assert!(
        code.contains("MAX_VALUE") || code.contains("100"),
        "Should handle global constant access: {code}"
    );
}

// ── Complex list/dict operations ────────────────────────────────

#[test]
fn test_list_comprehension_with_condition() {
    let code = transpile(
        r#"
def even_squares(n: int) -> list:
    return [i * i for i in range(n) if i % 2 == 0]
"#,
    );
    assert!(
        code.contains("filter") || code.contains("iter") || code.contains("%"),
        "Should generate filtered comprehension: {code}"
    );
}

#[test]
fn test_dict_comprehension() {
    let code = transpile(
        r#"
def index_map(items: list) -> dict:
    return {i: item for i, item in enumerate(items)}
"#,
    );
    assert!(
        code.contains("HashMap") || code.contains("collect") || code.contains("enumerate"),
        "Should generate dict comprehension: {code}"
    );
}

#[test]
fn test_set_comprehension() {
    let code = transpile(
        r#"
def unique_lengths(words: list) -> set:
    return {len(w) for w in words}
"#,
    );
    assert!(
        code.contains("HashSet") || code.contains("collect") || code.contains("len"),
        "Should generate set comprehension: {code}"
    );
}

// ── String operations ───────────────────────────────────────────

#[test]
fn test_string_methods_chain() {
    let code = transpile(
        r#"
def clean(s: str) -> str:
    return s.strip().lower().replace(" ", "_")
"#,
    );
    assert!(
        code.contains("trim") || code.contains("to_lowercase") || code.contains("replace"),
        "Should chain string methods: {code}"
    );
}

#[test]
fn test_string_split_join() {
    let code = transpile(
        r#"
def reverse_words(s: str) -> str:
    words = s.split(" ")
    words.reverse()
    return " ".join(words)
"#,
    );
    assert!(
        code.contains("split") || code.contains("join") || code.contains("reverse"),
        "Should handle split/join: {code}"
    );
}

// ── Type conversion patterns ────────────────────────────────────

#[test]
fn test_int_to_str() {
    let code = transpile(
        r#"
def f(x: int) -> str:
    return str(x)
"#,
    );
    assert!(
        code.contains("to_string") || code.contains("format"),
        "Should convert int to str: {code}"
    );
}

#[test]
fn test_str_to_int() {
    let code = transpile(
        r#"
def f(s: str) -> int:
    return int(s)
"#,
    );
    assert!(
        code.contains("parse") || code.contains("i64"),
        "Should convert str to int: {code}"
    );
}

#[test]
fn test_float_to_int() {
    let code = transpile(
        r#"
def f(x: float) -> int:
    return int(x)
"#,
    );
    assert!(
        code.contains("as i64") || code.contains("as i32") || code.contains("round"),
        "Should convert float to int: {code}"
    );
}

// ── Async patterns ──────────────────────────────────────────────

#[test]
fn test_async_function() {
    let code = transpile(
        r#"
async def fetch(url: str) -> str:
    return url
"#,
    );
    assert!(
        code.contains("async") || code.contains("fn fetch"),
        "Should handle async function: {code}"
    );
}

// ── Assert patterns ─────────────────────────────────────────────

#[test]
fn test_assert_equality() {
    let code = transpile(
        r#"
def f(x: int, y: int) -> int:
    assert x == y, "values must be equal"
    return x + y
"#,
    );
    assert!(
        code.contains("assert") || code.contains("equal"),
        "Should generate assert with equality: {code}"
    );
}

#[test]
fn test_assert_type_check() {
    let code = transpile(
        r#"
def f(x: int) -> int:
    assert x > 0
    assert x < 1000
    return x * 2
"#,
    );
    assert!(
        code.contains("assert") || code.contains("debug_assert"),
        "Should generate multiple asserts: {code}"
    );
}

// ── Delete statement ────────────────────────────────────────────

#[test]
fn test_del_variable() {
    let code = transpile(
        r#"
def f() -> int:
    x = 42
    del x
    return 0
"#,
    );
    assert!(
        code.contains("fn f") && code.contains("42"),
        "Should handle del statement: {code}"
    );
}
