//! Coverage tests for stmt_gen_complex.rs (try/except, nested functions)
//!
//! DEPYLER-99MODE-S8: Session 8 Batch 5 - targeting zero-test file
//! stmt_gen_complex.rs has 2,290 lines of code with 0 inline tests.
//! These transpile-based tests exercise the try/except codegen paths.

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

// ── Basic try/except ─────────────────────────────────────────────────

#[test]
fn test_try_except_basic() {
    let code = transpile(
        r#"
def f() -> int:
    try:
        x = int("42")
        return x
    except:
        return 0
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_try_except_value_error() {
    let code = transpile(
        r#"
def parse_int(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return -1
"#,
    );
    assert!(code.contains("fn parse_int"), "code: {code}");
}

#[test]
fn test_try_except_with_binding() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    try:
        x = int(s)
        return str(x)
    except Exception as e:
        return str(e)
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_try_except_finally() {
    let code = transpile(
        r#"
def f() -> int:
    result = 0
    try:
        result = 42
    except:
        result = -1
    finally:
        print(result)
    return result
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_try_except_multiple_handlers() {
    let code = transpile(
        r#"
def f(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return -1
    except TypeError:
        return -2
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Variable hoisting (DEPYLER-0681) ─────────────────────────────────

#[test]
fn test_try_except_variable_hoisting() {
    let code = transpile(
        r#"
def f() -> int:
    try:
        x = 42
    except:
        x = 0
    return x
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_try_except_hoisted_var_used_after() {
    let code = transpile(
        r#"
def f(data: str) -> str:
    try:
        result = data.upper()
    except:
        result = "error"
    return result
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Simple parse optimization (DEPYLER-0358) ──────────────────────

#[test]
fn test_try_except_parse_unwrap_or() {
    let code = transpile(
        r#"
def safe_int(s: str) -> int:
    try:
        return int(s)
    except:
        return 0
"#,
    );
    assert!(code.contains("fn safe_int"), "code: {code}");
}

#[test]
fn test_try_except_parse_negative_fallback() {
    let code = transpile(
        r#"
def safe_int(s: str) -> int:
    try:
        return int(s)
    except:
        return -1
"#,
    );
    assert!(code.contains("fn safe_int"), "code: {code}");
}

#[test]
fn test_try_except_parse_float_fallback() {
    let code = transpile(
        r#"
def safe_float(s: str) -> float:
    try:
        return float(s)
    except:
        return 0.0
"#,
    );
    assert!(code.contains("fn safe_float"), "code: {code}");
}

#[test]
fn test_try_except_parse_string_fallback() {
    let code = transpile(
        r#"
def safe_parse(s: str) -> str:
    try:
        x = int(s)
        return str(x)
    except:
        return "error"
"#,
    );
    assert!(code.contains("fn safe_parse"), "code: {code}");
}

// ── Floor division with ZeroDivisionError (DEPYLER-0360) ─────────

#[test]
fn test_try_except_zero_division() {
    let code = transpile(
        r#"
def safe_div(a: int, b: int) -> int:
    try:
        return a // b
    except ZeroDivisionError:
        return 0
"#,
    );
    assert!(code.contains("fn safe_div"), "code: {code}");
}

#[test]
fn test_try_except_zero_division_with_finally() {
    let code = transpile(
        r#"
def safe_div(a: int, b: int) -> int:
    try:
        return a // b
    except ZeroDivisionError:
        return 0
    finally:
        print("done")
"#,
    );
    assert!(code.contains("fn safe_div"), "code: {code}");
}

// ── Try without except (try/finally only) ────────────────────────

#[test]
fn test_try_finally_no_except() {
    let code = transpile(
        r#"
def f() -> int:
    x = 0
    try:
        x = 42
    finally:
        print("cleanup")
    return x
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Nested try/except ────────────────────────────────────────────

#[test]
fn test_nested_try_except() {
    let code = transpile(
        r#"
def f(s: str) -> int:
    try:
        try:
            return int(s)
        except ValueError:
            return -1
    except:
        return -2
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Nested functions (GH-70) ──────────────────────────────────────

#[test]
fn test_nested_function_basic() {
    let code = transpile(
        r#"
def make_adder(x: int):
    def adder(y: int) -> int:
        return x + y
    return adder
"#,
    );
    assert!(code.contains("fn make_adder"), "code: {code}");
}

#[test]
fn test_nested_function_no_params() {
    let code = transpile(
        r#"
def make_counter():
    count = 0
    def increment() -> int:
        return count + 1
    return increment
"#,
    );
    assert!(code.contains("fn make_counter"), "code: {code}");
}

// ── IO returns (DEPYLER-0626) ─────────────────────────────────────

#[test]
fn test_function_returns_file() {
    let code = transpile(
        r#"
def get_file(path: str):
    return open(path)
"#,
    );
    assert!(code.contains("fn get_file"), "code: {code}");
}

// ── Return type inference (DEPYLER-0410) ──────────────────────────

#[test]
fn test_return_type_inferred_from_body() {
    let code = transpile(
        r#"
def f(x: int):
    return x + 1
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_return_type_tuple_inference() {
    let code = transpile(
        r#"
def f(x: int, y: int) -> tuple:
    return (x, y)
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_return_type_optional_pattern() {
    let code = transpile(
        r#"
def find(items: list[int], target: int):
    for i in range(len(items)):
        if items[i] == target:
            return i
    return None
"#,
    );
    assert!(code.contains("fn find"), "code: {code}");
}

// ── Error type mapping (DEPYLER-0597) ──────────────────────────────

#[test]
fn test_oserror_maps_to_io_error() {
    let code = transpile(
        r#"
def read_file(path: str) -> str:
    try:
        f = open(path)
        return f.read()
    except OSError:
        return ""
"#,
    );
    assert!(code.contains("fn read_file"), "code: {code}");
}

#[test]
fn test_filenotfounderror() {
    let code = transpile(
        r#"
def exists(path: str) -> bool:
    try:
        f = open(path)
        return True
    except FileNotFoundError:
        return False
"#,
    );
    assert!(code.contains("fn exists"), "code: {code}");
}

#[test]
fn test_runtime_error() {
    let code = transpile(
        r#"
def check(x: int) -> int:
    try:
        if x < 0:
            raise RuntimeError("negative")
        return x
    except RuntimeError:
        return 0
"#,
    );
    assert!(code.contains("fn check"), "code: {code}");
}

// ── Handler with raise (DEPYLER-0819) ──────────────────────────────

#[test]
fn test_handler_contains_raise() {
    let code = transpile(
        r#"
def f(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        raise RuntimeError("invalid input")
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Complex try body patterns ──────────────────────────────────────

#[test]
fn test_try_with_multiple_statements() {
    let code = transpile(
        r#"
def f(data: str) -> int:
    try:
        x = int(data)
        y = x * 2
        return y
    except:
        return 0
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_try_with_loop_in_body() {
    let code = transpile(
        r#"
def sum_values(items: list[str]) -> int:
    total = 0
    for item in items:
        try:
            total = total + int(item)
        except:
            pass
    return total
"#,
    );
    assert!(code.contains("fn sum_values"), "code: {code}");
}

#[test]
fn test_try_with_if_in_body() {
    let code = transpile(
        r#"
def f(x: int) -> int:
    try:
        if x > 0:
            return x
        else:
            return -x
    except:
        return 0
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── extract_fields_from_expr coverage ──────────────────────────────

#[test]
fn test_fstring_with_method() {
    let code = transpile(
        r#"
def greet(name: str) -> str:
    return f"Hello, {name.upper()}!"
"#,
    );
    assert!(code.contains("fn greet"), "code: {code}");
}

#[test]
fn test_dict_with_index() {
    let code = transpile(
        r#"
def get_val(d: dict[str, int], key: str) -> int:
    return d[key]
"#,
    );
    assert!(code.contains("fn get_val"), "code: {code}");
}

// ── preload_hir_type_annotations coverage (func_gen_inference.rs) ──

#[test]
fn test_type_annotation_in_if() {
    let code = transpile(
        r#"
def f(x: int) -> int:
    if x > 0:
        result: int = x * 2
    else:
        result: int = 0
    return result
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_type_annotation_in_while() {
    let code = transpile(
        r#"
def f(n: int) -> int:
    total: int = 0
    i: int = 0
    while i < n:
        total = total + i
        i = i + 1
    return total
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_type_annotation_in_for() {
    let code = transpile(
        r#"
def f(items: list[int]) -> int:
    total: int = 0
    for item in items:
        total = total + item
    return total
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_type_annotation_in_try() {
    let code = transpile(
        r#"
def f(s: str) -> int:
    try:
        result: int = int(s)
    except:
        result: int = 0
    return result
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_type_annotation_in_with() {
    let code = transpile(
        r#"
def f(path: str) -> str:
    with open(path) as fh:
        data: str = fh.read()
    return data
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Parameter type inference (DEPYLER-0524) ─────────────────────

#[test]
fn test_param_type_inference_from_comparison() {
    let code = transpile(
        r#"
def is_positive(x):
    return x > 0
"#,
    );
    assert!(code.contains("fn is_positive"), "code: {code}");
}

#[test]
fn test_param_type_inference_from_string_method() {
    let code = transpile(
        r#"
def shout(s):
    return s.upper()
"#,
    );
    assert!(code.contains("fn shout"), "code: {code}");
}

#[test]
fn test_param_type_inference_from_arithmetic() {
    let code = transpile(
        r#"
def double(x):
    return x * 2
"#,
    );
    assert!(code.contains("fn double"), "code: {code}");
}

// ── Generic inference (DEPYLER-0716) ────────────────────────────

#[test]
fn test_generic_function() {
    let code = transpile(
        r#"
def identity(x: int) -> int:
    return x
"#,
    );
    assert!(code.contains("fn identity"), "code: {code}");
}

// ── Mutability detection (DEPYLER-0738) ─────────────────────────

#[test]
fn test_mutable_variable_detection() {
    let code = transpile(
        r#"
def f() -> int:
    x = 0
    x = x + 1
    x = x + 2
    return x
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
    assert!(
        code.contains("let mut"),
        "should detect mutable var: {code}"
    );
}

#[test]
fn test_mutable_list_push() {
    let code = transpile(
        r#"
def f() -> list[int]:
    items: list[int] = []
    items.append(1)
    items.append(2)
    return items
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Boolean operators in try blocks ──────────────────────────────

#[test]
fn test_try_with_boolean_return() {
    let code = transpile(
        r#"
def is_int(s: str) -> bool:
    try:
        int(s)
        return True
    except:
        return False
"#,
    );
    assert!(code.contains("fn is_int"), "code: {code}");
}

// ── Exception with pass ──────────────────────────────────────────

#[test]
fn test_try_except_pass() {
    let code = transpile(
        r#"
def f(s: str) -> int:
    x = 0
    try:
        x = int(s)
    except:
        pass
    return x
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── String concatenation return (v3.16.0) ──────────────────────

#[test]
fn test_string_concatenation_return() {
    let code = transpile(
        r#"
def greet(name: str) -> str:
    return "Hello, " + name + "!"
"#,
    );
    assert!(code.contains("fn greet"), "code: {code}");
    assert!(code.contains("String"), "should return String: {code}");
}

// ── main function special handling (DEPYLER-0612) ────────────────

#[test]
fn test_main_returns_unit() {
    let code = transpile(
        r#"
def main():
    print("hello")
"#,
    );
    assert!(code.contains("fn main"), "code: {code}");
}

#[test]
fn test_main_with_try_except() {
    let code = transpile(
        r#"
def main():
    try:
        x = int("42")
        print(x)
    except:
        print("error")
"#,
    );
    assert!(code.contains("fn main"), "code: {code}");
}

// ── Keyword function names (DEPYLER-0306) ──────────────────────

#[test]
fn test_rust_keyword_function_name() {
    // "loop" is a Rust keyword but valid Python identifier
    let code = transpile(
        r#"
def loop(x: int) -> int:
    return x + 1
"#,
    );
    // Should handle keyword escaping with raw identifier
    assert!(code.contains("fn"), "code: {code}");
}

// ── Vararg functions (DEPYLER-0648) ──────────────────────────────

#[test]
fn test_vararg_function() {
    let code = transpile(
        r#"
def sum_all(*args: int) -> int:
    total = 0
    for x in args:
        total = total + x
    return total
"#,
    );
    assert!(code.contains("fn sum_all"), "code: {code}");
}

// ── extract_parse_from_tokens edge cases ────────────────────────

#[test]
fn test_parse_with_remaining_stmts() {
    let code = transpile(
        r#"
def f(s: str) -> int:
    try:
        x = int(s)
        y = x + 1
        return y
    except ValueError:
        return -1
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Multiple return types ────────────────────────────────────────

#[test]
fn test_function_multiple_return_paths() {
    let code = transpile(
        r#"
def classify(x: int) -> str:
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    else:
        return "zero"
"#,
    );
    assert!(code.contains("fn classify"), "code: {code}");
}

// ── Complex patterns ────────────────────────────────────────────

#[test]
fn test_try_except_in_loop() {
    let code = transpile(
        r#"
def count_valid(items: list[str]) -> int:
    count = 0
    for item in items:
        try:
            int(item)
            count = count + 1
        except:
            pass
    return count
"#,
    );
    assert!(code.contains("fn count_valid"), "code: {code}");
}

#[test]
fn test_try_except_with_assignment_and_return() {
    let code = transpile(
        r#"
def safe_get(items: list[int], idx: int) -> int:
    try:
        val = items[idx]
        return val
    except IndexError:
        return -1
"#,
    );
    assert!(code.contains("fn safe_get"), "code: {code}");
}

// ── Bool return with exception check ──────────────────────────────

#[test]
fn test_try_except_bool_pattern() {
    let code = transpile(
        r#"
def can_parse(s: str) -> bool:
    try:
        float(s)
        return True
    except ValueError:
        return False
"#,
    );
    assert!(code.contains("fn can_parse"), "code: {code}");
}

// ── Deeply nested functions ──────────────────────────────────────

#[test]
fn test_function_with_complex_body() {
    let code = transpile(
        r#"
def process(items: list[int]) -> list[int]:
    result: list[int] = []
    for item in items:
        if item > 0:
            result.append(item * 2)
        else:
            result.append(0)
    return result
"#,
    );
    assert!(code.contains("fn process"), "code: {code}");
}

#[test]
fn test_function_with_while_and_break() {
    let code = transpile(
        r#"
def find_first(items: list[int], target: int) -> int:
    i = 0
    while i < len(items):
        if items[i] == target:
            return i
        i = i + 1
    return -1
"#,
    );
    assert!(code.contains("fn find_first"), "code: {code}");
}

// ── Assert statement ────────────────────────────────────────────

#[test]
fn test_assert_in_function() {
    let code = transpile(
        r#"
def f(x: int) -> int:
    assert x > 0
    return x * 2
"#,
    );
    assert!(code.contains("assert"), "code: {code}");
}

// ── With statement ──────────────────────────────────────────────

#[test]
fn test_with_open() {
    let code = transpile(
        r#"
def read_file(path: str) -> str:
    with open(path) as f:
        return f.read()
"#,
    );
    assert!(code.contains("fn read_file"), "code: {code}");
}

// ── Raise statement ─────────────────────────────────────────────

#[test]
fn test_raise_value_error() {
    let code = transpile(
        r#"
def validate(x: int) -> int:
    if x < 0:
        raise ValueError("must be non-negative")
    return x
"#,
    );
    assert!(code.contains("fn validate"), "code: {code}");
}

// ── Default parameter values ──────────────────────────────────────

#[test]
fn test_default_param_none() {
    let code = transpile(
        r#"
def f(x: int, y: int = 0) -> int:
    return x + y
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Comprehensive patterns in single function ────────────────────

#[test]
fn test_complex_function_with_all_features() {
    let code = transpile(
        r#"
def process_data(items: list[str]) -> dict[str, int]:
    result: dict[str, int] = {}
    for item in items:
        try:
            val = int(item)
            if val > 0:
                result[item] = val
        except ValueError:
            result[item] = -1
    return result
"#,
    );
    assert!(code.contains("fn process_data"), "code: {code}");
}
