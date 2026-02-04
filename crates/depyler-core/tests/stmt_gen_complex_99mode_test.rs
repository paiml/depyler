//! Coverage tests for stmt_gen_complex.rs
//!
//! DEPYLER-99MODE-001: Targets stmt_gen_complex.rs (44.37% -> 65%+)
//! Covers: try/except variable hoisting, JSON stdin patterns,
//! nested function closures, capture detection, subcommand fields.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[allow(dead_code)]
fn transpile(code: &str) -> String {
    DepylerPipeline::new()
        .transpile(code)
        .unwrap_or_else(|e| panic!("Transpilation failed: {e}"))
}

// ============================================================================
// Try/except variable hoisting patterns
// ============================================================================

#[test]
fn test_try_except_var_hoist_int() {
    let code = r#"
def f(s: str) -> int:
    try:
        value = int(s)
    except ValueError:
        value = 0
    return value
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_except_var_hoist_str() {
    let code = r#"
def f(s: str) -> str:
    try:
        result = s.upper()
    except Exception:
        result = "error"
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_except_var_hoist_list() {
    let code = r#"
def f(data: str) -> list:
    try:
        items = data.split(",")
    except Exception:
        items = []
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_except_simple_return_pattern() {
    let code = r#"
def safe_int(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return -1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_except_with_named_exception() {
    let code = r#"
def f(s: str) -> str:
    try:
        x = int(s)
        return str(x)
    except ValueError as e:
        return str(e)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_except_multiple_handlers() {
    let code = r#"
def f(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return -1
    except TypeError:
        return -2
    except Exception:
        return -99
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_except_finally() {
    let code = r#"
def f(s: str) -> int:
    result = 0
    try:
        result = int(s)
    except ValueError:
        result = -1
    finally:
        print("done")
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_except_else() {
    let code = r#"
def f(x: int) -> int:
    try:
        result = 100 // x
    except ZeroDivisionError:
        result = 0
    else:
        result = result * 2
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_all_clauses() {
    let code = r#"
def f(s: str) -> int:
    result = 0
    try:
        result = int(s)
    except ValueError:
        result = -1
    else:
        result += 10
    finally:
        print("cleanup")
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_bare_except() {
    let code = r#"
def f(s: str) -> int:
    try:
        return int(s)
    except:
        return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_floor_div_zero() {
    let code = r#"
def safe_div(a: int, b: int) -> int:
    try:
        return a // b
    except ZeroDivisionError:
        return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_nested() {
    let code = r#"
def f(a: str, b: str) -> int:
    try:
        x = int(a)
        try:
            y = int(b)
        except ValueError:
            y = 0
        return x + y
    except ValueError:
        return -1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_with_complex_body() {
    let code = r#"
def process(items: list) -> int:
    total = 0
    for item in items:
        try:
            total += int(item)
        except ValueError:
            continue
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// JSON stdin patterns
// ============================================================================

#[test]
fn test_json_load_stdin() {
    let code = r#"
import json
import sys

def f() -> dict:
    data = json.load(sys.stdin)
    return data
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_json_loads_input() {
    let code = r#"
import json

def f(text: str) -> dict:
    return json.loads(text)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_json_dumps() {
    let code = r#"
import json

def f(data: dict) -> str:
    return json.dumps(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_json_parse_with_error() {
    let code = r#"
import json

def f(text: str) -> dict:
    try:
        return json.loads(text)
    except Exception:
        return {}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Nested function closures - parameter types
// ============================================================================

#[test]
fn test_nested_fn_int_param() {
    let code = r#"
def outer(x: int) -> int:
    def inner(y: int) -> int:
        return x + y
    return inner(10)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_fn_str_param() {
    let code = r#"
def outer(prefix: str) -> str:
    def format_line(text: str) -> str:
        return prefix + ": " + text
    return format_line("hello")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_fn_list_param() {
    let code = r#"
def outer(items: list) -> int:
    def count_positive(data: list) -> int:
        total = 0
        for x in data:
            if x > 0:
                total += 1
        return total
    return count_positive(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_fn_dict_param() {
    let code = r#"
def outer(config: dict) -> str:
    def get_value(d: dict, key: str) -> str:
        return d.get(key, "")
    return get_value(config, "name")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_fn_multiple_captures() {
    let code = r#"
def outer(low: int, high: int) -> int:
    def clamp(x: int) -> int:
        if x < low:
            return low
        if x > high:
            return high
        return x
    return clamp(50)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_fn_no_capture() {
    let code = r#"
def outer() -> int:
    def add(a: int, b: int) -> int:
        return a + b
    return add(3, 4)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_fn_recursive() {
    let code = r#"
def outer(n: int) -> int:
    def factorial(x: int) -> int:
        if x <= 1:
            return 1
        return x * factorial(x - 1)
    return factorial(n)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_sibling_nested_fns() {
    let code = r#"
def outer(x: int) -> int:
    def double(n: int) -> int:
        return n * 2
    def triple(n: int) -> int:
        return n * 3
    return double(x) + triple(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_fn_returns_closure() {
    let code = r#"
def make_adder(n: int):
    def add(x: int) -> int:
        return x + n
    return add
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_fn_with_default() {
    let code = r#"
def outer() -> int:
    def inner(x: int = 10) -> int:
        return x * 2
    return inner()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Capture detection - complex expressions
// ============================================================================

#[test]
fn test_capture_in_binary_expr() {
    let code = r#"
def outer(multiplier: int) -> int:
    def compute(x: int) -> int:
        return x * multiplier + 1
    return compute(5)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_capture_in_if_condition() {
    let code = r#"
def outer(threshold: int) -> int:
    def check(x: int) -> bool:
        return x > threshold
    if check(10):
        return 1
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_capture_in_list_comprehension() {
    let code = r#"
def outer(factor: int) -> list:
    def scale(items: list) -> list:
        return [x * factor for x in items]
    return scale([1, 2, 3])
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_capture_in_for_loop() {
    let code = r#"
def outer(offset: int) -> int:
    def sum_offset(items: list) -> int:
        total = 0
        for item in items:
            total += item + offset
        return total
    return sum_offset([1, 2, 3])
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_capture_in_while_loop() {
    let code = r#"
def outer(limit: int) -> int:
    def count_to_limit() -> int:
        n = 0
        while n < limit:
            n += 1
        return n
    return count_to_limit()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_capture_multiple_vars() {
    let code = r#"
def outer(a: int, b: int, c: int) -> int:
    def compute(x: int) -> int:
        return a * x * x + b * x + c
    return compute(5)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_no_capture_local_only() {
    let code = r#"
def outer() -> int:
    def inner() -> int:
        x = 10
        y = 20
        return x + y
    return inner()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Variable collection in complex statements
// ============================================================================

#[test]
fn test_vars_in_for_loop() {
    let code = r#"
def f() -> int:
    total = 0
    for i in range(10):
        temp = i * 2
        total += temp
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_vars_in_if_else() {
    let code = r#"
def f(x: int) -> str:
    if x > 0:
        label = "positive"
    elif x < 0:
        label = "negative"
    else:
        label = "zero"
    return label
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_vars_in_while() {
    let code = r#"
def f() -> int:
    count = 0
    n = 100
    while n > 1:
        n = n // 2
        count += 1
    return count
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_vars_in_try_except() {
    let code = r#"
def f(items: list) -> int:
    result = 0
    for item in items:
        try:
            value = int(item)
            result += value
        except ValueError:
            error_count = 1
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_vars_in_with() {
    let code = r#"
def f(path: str) -> str:
    with open(path) as file:
        content = file.read()
    return content
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_vars_in_nested_loops() {
    let code = r#"
def f(n: int) -> int:
    total = 0
    for i in range(n):
        for j in range(n):
            product = i * j
            total += product
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex try/except with various exception types
// ============================================================================

#[test]
fn test_try_index_error() {
    let code = r#"
def safe_get(items: list, idx: int) -> int:
    try:
        return items[idx]
    except IndexError:
        return -1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_key_error() {
    let code = r#"
def safe_lookup(d: dict, key: str) -> str:
    try:
        return d[key]
    except KeyError:
        return ""
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_attribute_error() {
    let code = r#"
def safe_method(obj: str) -> str:
    try:
        return obj.upper()
    except AttributeError:
        return ""
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_type_error() {
    let code = r#"
def safe_add(a: int, b: int) -> int:
    try:
        return a + b
    except TypeError:
        return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_io_error() {
    let code = r#"
def safe_read(path: str) -> str:
    try:
        with open(path) as f:
            return f.read()
    except IOError:
        return ""
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_os_error() {
    let code = r#"
def safe_open(path: str) -> str:
    try:
        with open(path) as f:
            return f.read()
    except OSError:
        return ""
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Raise statement in try/except
// ============================================================================

#[test]
fn test_re_raise() {
    let code = r#"
def f(x: int) -> int:
    try:
        return 100 // x
    except ZeroDivisionError:
        raise
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_raise_new_exception() {
    let code = r#"
def f(x: int) -> int:
    try:
        return int(x)
    except ValueError:
        raise RuntimeError("invalid input")
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex statement combinations hitting stmt_gen_complex paths
// ============================================================================

#[test]
fn test_try_in_for_loop() {
    let code = r#"
def parse_numbers(items: list) -> list:
    result = []
    for item in items:
        try:
            result.append(int(item))
        except ValueError:
            pass
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_try_except() {
    let code = r#"
def f(a: str, b: str) -> int:
    try:
        x = int(a)
    except ValueError:
        x = 0
    try:
        y = int(b)
    except ValueError:
        y = 0
    return x + y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_with_list_operations() {
    let code = r#"
def safe_process(data: list) -> list:
    results = []
    try:
        for item in data:
            results.append(item * 2)
    except Exception:
        results = []
    return results
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_with_dict_operations() {
    let code = r#"
def merge_configs(base: dict, override: dict) -> dict:
    result = base.copy()
    try:
        result.update(override)
    except Exception:
        pass
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_with_string_operations() {
    let code = r#"
def parse_header(line: str) -> tuple:
    try:
        key, value = line.split(":", 1)
        return (key.strip(), value.strip())
    except ValueError:
        return (line.strip(), "")
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Argparse-related patterns
// ============================================================================

#[test]
fn test_argparse_basic() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--name", type=str, default="World")
    args = parser.parse_args()
    print("Hello, " + args.name)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_with_flag() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--verbose", action="store_true")
    parser.add_argument("--count", type=int, default=1)
    args = parser.parse_args()
    if args.verbose:
        print("Verbose mode")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_positional() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("filename", type=str)
    args = parser.parse_args()
    print(args.filename)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// With statement patterns (context managers)
// ============================================================================

#[test]
fn test_with_open_read() {
    let code = r#"
def read_file(path: str) -> str:
    with open(path) as f:
        return f.read()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_with_open_write() {
    let code = r#"
def write_file(path: str, content: str):
    with open(path, "w") as f:
        f.write(content)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_with_open_readlines() {
    let code = r#"
def count_lines(path: str) -> int:
    with open(path) as f:
        lines = f.readlines()
    return len(lines)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_with_no_as() {
    let code = r#"
def f():
    with open("test.txt"):
        print("inside context")
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Mixed complex patterns
// ============================================================================

#[test]
fn test_try_in_nested_function() {
    let code = r#"
def outer() -> int:
    def safe_parse(s: str) -> int:
        try:
            return int(s)
        except ValueError:
            return 0
    return safe_parse("42") + safe_parse("bad")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_fn_with_try_and_capture() {
    let code = r#"
def processor(default: int) -> int:
    def parse(s: str) -> int:
        try:
            return int(s)
        except ValueError:
            return default
    return parse("hello")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_complex_error_handling() {
    let code = r#"
def process_data(raw: str) -> dict:
    result = {}
    lines = raw.strip().split("\n")
    for line in lines:
        try:
            key, value = line.split("=", 1)
            result[key.strip()] = value.strip()
        except ValueError:
            continue
    return result
"#;
    assert!(transpile_ok(code));
}
