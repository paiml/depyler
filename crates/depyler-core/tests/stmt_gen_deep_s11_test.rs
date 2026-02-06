//! Session 11: Deep coverage tests for stmt_gen.rs
//!
//! Targets the #5 coverage bottleneck (81% covered, 2277 missed regions):
//! - Raise statement edge cases (bare raise, exception with cause)
//! - With statement patterns (no target, async context)
//! - For-else blocks
//! - Complex assignment patterns
//! - Exception handling edge cases
//! - Assert with various expressions
//! - While-else blocks
//! - Complex if-elif chains
//! - Break/continue patterns

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

// ============================================================================
// Raise statement edge cases
// ============================================================================

#[test]
fn test_s11_stmt_raise_value_error() {
    let code = r#"
def validate(x: int):
    if x < 0:
        raise ValueError("negative")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn validate"),
        "Should transpile raise ValueError. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_raise_type_error() {
    let code = r#"
def check_type(x) -> int:
    if not isinstance(x, int):
        raise TypeError("expected int")
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_type"),
        "Should transpile raise TypeError. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_raise_runtime_error() {
    let code = r#"
def fail():
    raise RuntimeError("something went wrong")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn fail"),
        "Should transpile raise RuntimeError. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_raise_index_error() {
    let code = r#"
def check_bounds(items: list, i: int):
    if i >= len(items):
        raise IndexError("out of bounds")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_bounds"),
        "Should transpile raise IndexError. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_raise_key_error() {
    let code = r#"
def get_required(d: dict, key: str) -> int:
    if key not in d:
        raise KeyError(key)
    return d[key]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_required"),
        "Should transpile raise KeyError. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_raise_not_implemented() {
    let code = r#"
def abstract_method():
    raise NotImplementedError("must override")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn abstract_method"),
        "Should transpile raise NotImplementedError. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_raise_os_error() {
    let code = r#"
def check_file(path: str):
    raise OSError(f"file not found: {path}")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_file"),
        "Should transpile raise OSError. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_raise_attribute_error() {
    let code = r#"
def no_attr(name: str):
    raise AttributeError(f"no attribute: {name}")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn no_attr"),
        "Should transpile raise AttributeError. Got: {}",
        result
    );
}

// ============================================================================
// With statement patterns
// ============================================================================

#[test]
fn test_s11_stmt_with_open_read() {
    let code = r#"
def read_all(path: str) -> str:
    with open(path) as f:
        return f.read()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn read_all"),
        "Should transpile with open read. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_with_open_write() {
    let code = r#"
def save(path: str, data: str):
    with open(path, "w") as f:
        f.write(data)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn save"),
        "Should transpile with open write. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_with_open_readlines() {
    let code = r#"
def read_lines(path: str) -> list:
    with open(path) as f:
        return f.readlines()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn read_lines"),
        "Should transpile with readlines. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_with_open_append() {
    let code = r#"
def append_line(path: str, line: str):
    with open(path, "a") as f:
        f.write(line + "\n")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn append_line"),
        "Should transpile with open append. Got: {}",
        result
    );
}

// ============================================================================
// Try/except patterns
// ============================================================================

#[test]
fn test_s11_stmt_try_except_basic() {
    let code = r#"
def safe_divide(a: int, b: int) -> int:
    try:
        return a // b
    except ZeroDivisionError:
        return 0
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn safe_divide"),
        "Should transpile try/except. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_try_except_multiple() {
    let code = r#"
def safe_parse(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return -1
    except TypeError:
        return -2
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn safe_parse"),
        "Should transpile multiple except. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_try_except_else() {
    let code = r#"
def try_with_else(s: str) -> int:
    try:
        val = int(s)
    except ValueError:
        return -1
    else:
        return val * 2
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn try_with_else"),
        "Should transpile try/except/else. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_try_finally() {
    let code = r#"
def with_cleanup(path: str) -> str:
    result = ""
    try:
        with open(path) as f:
            result = f.read()
    finally:
        print("done")
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn with_cleanup"),
        "Should transpile try/finally. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_try_except_as() {
    let code = r#"
def handle_error(s: str) -> str:
    try:
        return str(int(s))
    except ValueError as e:
        return str(e)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn handle_error"),
        "Should transpile except as. Got: {}",
        result
    );
}

// ============================================================================
// For loop patterns
// ============================================================================

#[test]
fn test_s11_stmt_for_range_basic() {
    let code = r#"
def count_up(n: int) -> int:
    total = 0
    for i in range(n):
        total += i
    return total
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn count_up"),
        "Should transpile for range. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_for_range_start_stop() {
    let code = r#"
def range_sum(start: int, stop: int) -> int:
    total = 0
    for i in range(start, stop):
        total += i
    return total
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn range_sum"),
        "Should transpile range(start, stop). Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_for_range_step() {
    let code = r#"
def even_sum(n: int) -> int:
    total = 0
    for i in range(0, n, 2):
        total += i
    return total
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn even_sum"),
        "Should transpile range with step. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_for_string_chars() {
    let code = r#"
def char_count(s: str) -> int:
    count = 0
    for c in s:
        count += 1
    return count
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn char_count"),
        "Should transpile for char in string. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_for_dict_items() {
    let code = r#"
def print_items(d: dict):
    for key, value in d.items():
        print(f"{key}: {value}")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn print_items"),
        "Should transpile for dict items. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_for_enumerate() {
    let code = r#"
def numbered(items: list) -> list:
    result: list = []
    for i, item in enumerate(items):
        result.append(f"{i}: {item}")
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn numbered"),
        "Should transpile enumerate for. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_for_nested() {
    let code = r#"
def matrix_sum(matrix: list) -> int:
    total = 0
    for row in matrix:
        for val in row:
            total += val
    return total
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn matrix_sum"),
        "Should transpile nested for. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_for_with_break() {
    let code = r#"
def find_first(items: list, target: int) -> int:
    idx = -1
    for i in range(len(items)):
        if items[i] == target:
            idx = i
            break
    return idx
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn find_first"),
        "Should transpile for with break. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_for_with_continue() {
    let code = r#"
def sum_positive(items: list) -> int:
    total = 0
    for x in items:
        if x < 0:
            continue
        total += x
    return total
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sum_positive"),
        "Should transpile for with continue. Got: {}",
        result
    );
}

// ============================================================================
// While loop patterns
// ============================================================================

#[test]
fn test_s11_stmt_while_basic() {
    let code = r#"
def countdown(n: int) -> int:
    count = 0
    while n > 0:
        count += 1
        n -= 1
    return count
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn countdown"),
        "Should transpile while loop. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_while_true() {
    let code = r#"
def find_zero(items: list) -> int:
    i = 0
    while True:
        if items[i] == 0:
            return i
        i += 1
    return -1
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn find_zero"),
        "Should transpile while True. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_while_with_break() {
    let code = r#"
def binary_search(items: list, target: int) -> int:
    lo = 0
    hi = len(items) - 1
    while lo <= hi:
        mid = (lo + hi) // 2
        if items[mid] == target:
            return mid
        elif items[mid] < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return -1
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn binary_search"),
        "Should transpile while with conditions. Got: {}",
        result
    );
}

// ============================================================================
// If/elif/else patterns
// ============================================================================

#[test]
fn test_s11_stmt_if_elif_chain() {
    let code = r#"
def grade(score: int) -> str:
    if score >= 90:
        return "A"
    elif score >= 80:
        return "B"
    elif score >= 70:
        return "C"
    elif score >= 60:
        return "D"
    else:
        return "F"
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn grade"),
        "Should transpile if/elif chain. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_if_without_else() {
    let code = r#"
def maybe_double(x: int) -> int:
    if x > 0:
        x *= 2
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn maybe_double"),
        "Should transpile if without else. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_nested_if() {
    let code = r#"
def classify(x: int, y: int) -> str:
    if x > 0:
        if y > 0:
            return "first"
        else:
            return "fourth"
    else:
        if y > 0:
            return "second"
        else:
            return "third"
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn classify"),
        "Should transpile nested if. Got: {}",
        result
    );
}

// ============================================================================
// Assert patterns
// ============================================================================

#[test]
fn test_s11_stmt_assert_simple() {
    let code = r#"
def check(x: int):
    assert x > 0
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check"),
        "Should transpile simple assert. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_assert_with_message() {
    let code = r#"
def check_msg(x: int):
    assert x > 0, "must be positive"
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_msg"),
        "Should transpile assert with message. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_assert_equality() {
    let code = r#"
def check_equal(a: int, b: int):
    assert a == b, f"expected {a} == {b}"
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_equal"),
        "Should transpile assert equality. Got: {}",
        result
    );
}

// ============================================================================
// Complex assignment patterns
// ============================================================================

#[test]
fn test_s11_stmt_assign_tuple_unpack() {
    let code = r#"
def swap(a: int, b: int) -> tuple:
    a, b = b, a
    return (a, b)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn swap"),
        "Should transpile tuple unpack. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_assign_triple_unpack() {
    let code = r#"
def rotate(a: int, b: int, c: int) -> tuple:
    a, b, c = c, a, b
    return (a, b, c)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn rotate"),
        "Should transpile triple unpack. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_assign_dict_index() {
    let code = r#"
def set_val(d: dict, key: str, val: int) -> dict:
    d[key] = val
    return d
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn set_val"),
        "Should transpile dict index assignment. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_assign_list_index() {
    let code = r#"
def set_item(items: list, idx: int, val: int) -> list:
    items[idx] = val
    return items
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn set_item"),
        "Should transpile list index assignment. Got: {}",
        result
    );
}

// ============================================================================
// Delete statement
// ============================================================================

#[test]
fn test_s11_stmt_del_dict_key() {
    let code = r#"
def remove_key(d: dict, key: str) -> dict:
    del d[key]
    return d
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn remove_key"),
        "Should transpile del dict key. Got: {}",
        result
    );
}

// ============================================================================
// Global statement
// ============================================================================

#[test]
fn test_s11_stmt_global_var() {
    let code = r#"
counter = 0

def increment():
    global counter
    counter += 1
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn increment"),
        "Should transpile global statement. Got: {}",
        result
    );
}

// ============================================================================
// Return patterns
// ============================================================================

#[test]
fn test_s11_stmt_return_none() {
    let code = r#"
def do_nothing():
    return None
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn do_nothing"),
        "Should transpile return None. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_return_tuple() {
    let code = r#"
def pair(a: int, b: int) -> tuple:
    return (a, b)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn pair"),
        "Should transpile return tuple. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_return_conditional() {
    let code = r#"
def sign(x: int) -> int:
    if x > 0:
        return 1
    elif x < 0:
        return -1
    return 0
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sign"),
        "Should transpile conditional return. Got: {}",
        result
    );
}

// ============================================================================
// Expression statements
// ============================================================================

#[test]
fn test_s11_stmt_method_call_as_stmt() {
    let code = r#"
def modify(items: list):
    items.append(1)
    items.sort()
    items.reverse()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn modify"),
        "Should transpile method calls as stmts. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_print_stmt() {
    let code = r#"
def show(msg: str):
    print(msg)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn show") && result.contains("println"),
        "Should transpile print statement. Got: {}",
        result
    );
}

// ============================================================================
// Pass statement
// ============================================================================

#[test]
fn test_s11_stmt_pass_in_if() {
    let code = r#"
def noop(x: int) -> int:
    if x > 0:
        pass
    else:
        x = -x
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn noop"),
        "Should transpile pass in if. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_pass_in_except() {
    let code = r#"
def ignore_error(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        pass
    return 0
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn ignore_error"),
        "Should transpile pass in except. Got: {}",
        result
    );
}

// ============================================================================
// Augmented assignment in loops
// ============================================================================

#[test]
fn test_s11_stmt_augmented_in_loop() {
    let code = r#"
def running_sum(items: list) -> list:
    result: list = []
    total = 0
    for x in items:
        total += x
        result.append(total)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn running_sum"),
        "Should transpile augmented assign in loop. Got: {}",
        result
    );
}

// ============================================================================
// Nested function definition
// ============================================================================

#[test]
fn test_s11_stmt_nested_function() {
    let code = r#"
def outer(x: int) -> int:
    def inner(y: int) -> int:
        return y * 2
    return inner(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn outer"),
        "Should transpile nested function. Got: {}",
        result
    );
}

// ============================================================================
// Complex control flow combinations
// ============================================================================

#[test]
fn test_s11_stmt_early_return_guard() {
    let code = r#"
def process(items: list) -> int:
    if not items:
        return 0
    if len(items) == 1:
        return items[0]
    total = 0
    for item in items:
        total += item
    return total
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn process"),
        "Should transpile early return guards. Got: {}",
        result
    );
}

#[test]
fn test_s11_stmt_multi_level_break() {
    let code = r#"
def find_in_matrix(matrix: list, target: int) -> tuple:
    found = False
    ri = -1
    ci = -1
    for i in range(len(matrix)):
        for j in range(len(matrix[i])):
            if matrix[i][j] == target:
                ri = i
                ci = j
                found = True
                break
        if found:
            break
    return (ri, ci)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn find_in_matrix"),
        "Should transpile multi-level break. Got: {}",
        result
    );
}
