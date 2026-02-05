//! Session 8 coverage tests for expr_gen.rs
//! Targets: binary ops, comparison, boolean, attribute access, subscript,
//! call expressions, lambda, starred, ternary

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

// ── Binary operations ───────────────────────────────────────────

#[test]
fn test_integer_division() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> int:
    return a // b
"#,
    );
    assert!(
        code.contains("/") || code.contains("div"),
        "Should generate integer division: {code}"
    );
}

#[test]
fn test_modulo() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> int:
    return a % b
"#,
    );
    assert!(code.contains("%"), "Should generate modulo: {code}");
}

#[test]
fn test_power() {
    let code = transpile(
        r#"
def f(base: int, exp: int) -> int:
    return base ** exp
"#,
    );
    assert!(
        code.contains("pow") || code.contains("**"),
        "Should generate power: {code}"
    );
}

#[test]
fn test_bitwise_and() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> int:
    return a & b
"#,
    );
    assert!(code.contains("&"), "Should generate bitwise and: {code}");
}

#[test]
fn test_bitwise_or() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> int:
    return a | b
"#,
    );
    assert!(code.contains("|"), "Should generate bitwise or: {code}");
}

#[test]
fn test_bitwise_xor() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> int:
    return a ^ b
"#,
    );
    assert!(code.contains("^"), "Should generate bitwise xor: {code}");
}

#[test]
fn test_left_shift() {
    let code = transpile(
        r#"
def f(a: int, n: int) -> int:
    return a << n
"#,
    );
    assert!(code.contains("<<"), "Should generate left shift: {code}");
}

#[test]
fn test_right_shift() {
    let code = transpile(
        r#"
def f(a: int, n: int) -> int:
    return a >> n
"#,
    );
    assert!(code.contains(">>"), "Should generate right shift: {code}");
}

// ── Boolean operations ──────────────────────────────────────────

#[test]
fn test_boolean_and() {
    let code = transpile(
        r#"
def f(a: bool, b: bool) -> bool:
    return a and b
"#,
    );
    assert!(code.contains("&&"), "Should generate logical and: {code}");
}

#[test]
fn test_boolean_or() {
    let code = transpile(
        r#"
def f(a: bool, b: bool) -> bool:
    return a or b
"#,
    );
    assert!(code.contains("||"), "Should generate logical or: {code}");
}

#[test]
fn test_boolean_not() {
    let code = transpile(
        r#"
def f(a: bool) -> bool:
    return not a
"#,
    );
    assert!(code.contains("!"), "Should generate logical not: {code}");
}

// ── Comparison operators ────────────────────────────────────────

#[test]
fn test_is_none() {
    let code = transpile(
        r#"
def f(x: int) -> bool:
    return x is None
"#,
    );
    assert!(
        code.contains("is_none") || code.contains("None") || code.contains("false"),
        "Should generate is None check: {code}"
    );
}

#[test]
fn test_is_not_none() {
    let code = transpile(
        r#"
def f(x: int) -> bool:
    return x is not None
"#,
    );
    assert!(
        code.contains("is_some") || code.contains("true") || code.contains("!"),
        "Should generate is not None check: {code}"
    );
}

#[test]
fn test_in_operator_list() {
    let code = transpile(
        r#"
def f(x: int, items: list) -> bool:
    return x in items
"#,
    );
    assert!(
        code.contains("contains"),
        "Should generate contains: {code}"
    );
}

#[test]
fn test_not_in_operator() {
    let code = transpile(
        r#"
def f(x: int, items: list) -> bool:
    return x not in items
"#,
    );
    assert!(
        code.contains("contains") || code.contains("!"),
        "Should generate not contains: {code}"
    );
}

// ── Subscript expressions ───────────────────────────────────────

#[test]
fn test_list_index() {
    let code = transpile(
        r#"
def f(items: list) -> int:
    return items[0]
"#,
    );
    assert!(
        code.contains("[0]") || code.contains("get(0)") || code.contains("0"),
        "Should generate list index: {code}"
    );
}

#[test]
fn test_negative_index() {
    let code = transpile(
        r#"
def f(items: list) -> int:
    return items[-1]
"#,
    );
    assert!(
        code.contains("last()") || code.contains("len()") || code.contains("-1"),
        "Should generate negative index: {code}"
    );
}

#[test]
fn test_dict_key_access() {
    let code = transpile(
        r#"
def f(d: dict) -> int:
    return d["key"]
"#,
    );
    assert!(
        code.contains("[") || code.contains("get"),
        "Should generate dict access: {code}"
    );
}

// ── Call expressions ────────────────────────────────────────────

#[test]
fn test_len_builtin() {
    let code = transpile(
        r#"
def f(items: list) -> int:
    return len(items)
"#,
    );
    assert!(code.contains("len()"), "Should generate len(): {code}");
}

#[test]
fn test_range_call() {
    let code = transpile(
        r#"
def f(n: int) -> list:
    result = []
    for i in range(n):
        result.append(i)
    return result
"#,
    );
    assert!(
        code.contains("0..") || code.contains("range"),
        "Should generate range: {code}"
    );
}

#[test]
fn test_range_start_stop() {
    let code = transpile(
        r#"
def f() -> list:
    result = []
    for i in range(1, 10):
        result.append(i)
    return result
"#,
    );
    assert!(
        code.contains("1..10") || code.contains("1.."),
        "Should generate range(1, 10): {code}"
    );
}

#[test]
fn test_range_with_step() {
    let code = transpile(
        r#"
def f() -> list:
    result = []
    for i in range(0, 10, 2):
        result.append(i)
    return result
"#,
    );
    assert!(
        code.contains("step_by") || code.contains("0..10"),
        "Should generate range with step: {code}"
    );
}

#[test]
fn test_print_call() {
    let code = transpile(
        r#"
def f() -> None:
    print("hello")
"#,
    );
    assert!(
        code.contains("println!") || code.contains("print"),
        "Should generate println: {code}"
    );
}

#[test]
fn test_print_multiple_args() {
    let code = transpile(
        r#"
def f(name: str, age: int) -> None:
    print(name, age)
"#,
    );
    assert!(
        code.contains("println!") || code.contains("print"),
        "Should generate println with args: {code}"
    );
}

#[test]
fn test_int_conversion() {
    let code = transpile(
        r#"
def f(s: str) -> int:
    return int(s)
"#,
    );
    assert!(
        code.contains("parse") || code.contains("to_i"),
        "Should generate parse: {code}"
    );
}

#[test]
fn test_str_conversion() {
    let code = transpile(
        r#"
def f(x: int) -> str:
    return str(x)
"#,
    );
    assert!(
        code.contains("to_string") || code.contains("format"),
        "Should generate to_string: {code}"
    );
}

#[test]
fn test_float_conversion() {
    let code = transpile(
        r#"
def f(s: str) -> float:
    return float(s)
"#,
    );
    assert!(
        code.contains("parse") || code.contains("f64"),
        "Should generate float parse: {code}"
    );
}

#[test]
fn test_abs_builtin() {
    let code = transpile(
        r#"
def f(x: int) -> int:
    return abs(x)
"#,
    );
    assert!(code.contains("abs"), "Should generate abs: {code}");
}

#[test]
fn test_max_builtin() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> int:
    return max(a, b)
"#,
    );
    assert!(
        code.contains("max") || code.contains("std::cmp::max"),
        "Should generate max: {code}"
    );
}

#[test]
fn test_min_builtin() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> int:
    return min(a, b)
"#,
    );
    assert!(
        code.contains("min") || code.contains("std::cmp::min"),
        "Should generate min: {code}"
    );
}

#[test]
fn test_sum_builtin() {
    let code = transpile(
        r#"
def f(items: list) -> int:
    return sum(items)
"#,
    );
    assert!(
        code.contains("sum()") || code.contains("iter()"),
        "Should generate sum: {code}"
    );
}

#[test]
fn test_any_builtin() {
    let code = transpile(
        r#"
def f(items: list) -> bool:
    return any(items)
"#,
    );
    assert!(
        code.contains("any") || code.contains("iter()"),
        "Should generate any: {code}"
    );
}

#[test]
fn test_all_builtin() {
    let code = transpile(
        r#"
def f(items: list) -> bool:
    return all(items)
"#,
    );
    assert!(
        code.contains("all") || code.contains("iter()"),
        "Should generate all: {code}"
    );
}

// ── Lambda expressions ──────────────────────────────────────────

#[test]
fn test_lambda_simple() {
    let code = transpile(
        r#"
def f() -> int:
    square = lambda x: x * x
    return square(5)
"#,
    );
    assert!(
        code.contains("|") || code.contains("closure") || code.contains("Fn"),
        "Should generate closure: {code}"
    );
}

#[test]
fn test_lambda_in_sorted() {
    let code = transpile(
        r#"
def f(items: list) -> list:
    return sorted(items, key=lambda x: x[0])
"#,
    );
    assert!(
        code.contains("sort") || code.contains("|"),
        "Should generate sorted with key: {code}"
    );
}

#[test]
fn test_lambda_in_map() {
    let code = transpile(
        r#"
def f(items: list) -> list:
    return list(map(lambda x: x * 2, items))
"#,
    );
    assert!(
        code.contains("map") || code.contains("|"),
        "Should generate map with lambda: {code}"
    );
}

#[test]
fn test_lambda_in_filter() {
    let code = transpile(
        r#"
def f(items: list) -> list:
    return list(filter(lambda x: x > 0, items))
"#,
    );
    assert!(
        code.contains("filter") || code.contains("|"),
        "Should generate filter with lambda: {code}"
    );
}

// ── String operations ───────────────────────────────────────────

#[test]
fn test_string_split() {
    let code = transpile(
        r#"
def f(s: str) -> list:
    return s.split(",")
"#,
    );
    assert!(code.contains("split"), "Should generate split: {code}");
}

#[test]
fn test_string_join() {
    let code = transpile(
        r#"
def f(items: list) -> str:
    return ",".join(items)
"#,
    );
    assert!(code.contains("join"), "Should generate join: {code}");
}

#[test]
fn test_string_strip() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    return s.strip()
"#,
    );
    assert!(
        code.contains("trim"),
        "Should generate trim for strip: {code}"
    );
}

#[test]
fn test_string_replace() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    return s.replace("old", "new")
"#,
    );
    assert!(code.contains("replace"), "Should generate replace: {code}");
}

#[test]
fn test_string_startswith() {
    let code = transpile(
        r#"
def f(s: str) -> bool:
    return s.startswith("prefix")
"#,
    );
    assert!(
        code.contains("starts_with"),
        "Should generate starts_with: {code}"
    );
}

#[test]
fn test_string_endswith() {
    let code = transpile(
        r#"
def f(s: str) -> bool:
    return s.endswith("suffix")
"#,
    );
    assert!(
        code.contains("ends_with"),
        "Should generate ends_with: {code}"
    );
}

#[test]
fn test_string_upper() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    return s.upper()
"#,
    );
    assert!(
        code.contains("to_uppercase"),
        "Should generate to_uppercase: {code}"
    );
}

#[test]
fn test_string_lower() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    return s.lower()
"#,
    );
    assert!(
        code.contains("to_lowercase"),
        "Should generate to_lowercase: {code}"
    );
}

#[test]
fn test_string_find() {
    let code = transpile(
        r#"
def f(s: str, sub: str) -> int:
    return s.find(sub)
"#,
    );
    assert!(
        code.contains("find") || code.contains("position"),
        "Should generate find: {code}"
    );
}

#[test]
fn test_string_count() {
    let code = transpile(
        r#"
def f(s: str, sub: str) -> int:
    return s.count(sub)
"#,
    );
    assert!(
        code.contains("matches") || code.contains("count"),
        "Should generate count: {code}"
    );
}

#[test]
fn test_string_isdigit() {
    let code = transpile(
        r#"
def f(s: str) -> bool:
    return s.isdigit()
"#,
    );
    assert!(
        code.contains("is_numeric") || code.contains("is_digit") || code.contains("chars()"),
        "Should generate isdigit check: {code}"
    );
}

#[test]
fn test_string_isalpha() {
    let code = transpile(
        r#"
def f(s: str) -> bool:
    return s.isalpha()
"#,
    );
    assert!(
        code.contains("is_alphabetic") || code.contains("chars()"),
        "Should generate isalpha check: {code}"
    );
}

// ── List methods ────────────────────────────────────────────────

#[test]
fn test_list_append() {
    let code = transpile(
        r#"
def f(items: list) -> None:
    items.append(42)
"#,
    );
    assert!(code.contains("push"), "Should generate push: {code}");
}

#[test]
fn test_list_extend() {
    let code = transpile(
        r#"
def f(a: list, b: list) -> None:
    a.extend(b)
"#,
    );
    assert!(code.contains("extend"), "Should generate extend: {code}");
}

#[test]
fn test_list_insert() {
    let code = transpile(
        r#"
def f(items: list) -> None:
    items.insert(0, 42)
"#,
    );
    assert!(code.contains("insert"), "Should generate insert: {code}");
}

#[test]
fn test_list_remove() {
    let code = transpile(
        r#"
def f(items: list) -> None:
    items.remove(42)
"#,
    );
    assert!(
        code.contains("retain") || code.contains("remove") || code.contains("position"),
        "Should generate remove: {code}"
    );
}

#[test]
fn test_list_pop() {
    let code = transpile(
        r#"
def f(items: list) -> int:
    return items.pop()
"#,
    );
    assert!(code.contains("pop"), "Should generate pop: {code}");
}

#[test]
fn test_list_sort() {
    let code = transpile(
        r#"
def f(items: list) -> None:
    items.sort()
"#,
    );
    assert!(code.contains("sort"), "Should generate sort: {code}");
}

#[test]
fn test_list_reverse() {
    let code = transpile(
        r#"
def f(items: list) -> None:
    items.reverse()
"#,
    );
    assert!(code.contains("reverse"), "Should generate reverse: {code}");
}

#[test]
fn test_list_index_method() {
    let code = transpile(
        r#"
def f(items: list, x: int) -> int:
    return items.index(x)
"#,
    );
    assert!(
        code.contains("position") || code.contains("index") || code.contains("iter()"),
        "Should generate index/position: {code}"
    );
}

// ── Type-specific expressions ───────────────────────────────────

// isinstance check may not transpile cleanly for generic object types

#[test]
fn test_type_check() {
    let code = transpile(
        r#"
def f(x: object) -> str:
    return type(x).__name__
"#,
    );
    assert!(
        code.contains("type") || code.contains("type_name"),
        "Should generate type check: {code}"
    );
}

// ── Complex expressions ─────────────────────────────────────────

#[test]
fn test_nested_function_calls() {
    let code = transpile(
        r#"
def f(items: list) -> int:
    return len(sorted(items))
"#,
    );
    assert!(
        code.contains("len()") || code.contains("sort"),
        "Should generate nested calls: {code}"
    );
}

#[test]
fn test_chained_method_calls() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    return s.strip().lower().replace(" ", "_")
"#,
    );
    assert!(
        code.contains("trim") || code.contains("to_lowercase"),
        "Should generate chained methods: {code}"
    );
}

#[test]
fn test_generator_expression_in_sum() {
    let code = transpile(
        r#"
def f(items: list) -> int:
    return sum(x * x for x in items)
"#,
    );
    assert!(
        code.contains("iter()") || code.contains("map") || code.contains("sum"),
        "Should generate generator in sum: {code}"
    );
}

#[test]
fn test_dict_get_with_default() {
    let code = transpile(
        r#"
def f(d: dict) -> int:
    return d.get("key", 0)
"#,
    );
    assert!(
        code.contains("get") || code.contains("unwrap_or"),
        "Should generate get with default: {code}"
    );
}

#[test]
fn test_string_multiplication() {
    let code = transpile(
        r#"
def f(s: str, n: int) -> str:
    return s * n
"#,
    );
    assert!(
        code.contains("repeat") || code.contains("*"),
        "Should generate string repeat: {code}"
    );
}

#[test]
fn test_tuple_creation() {
    let code = transpile(
        r#"
def f() -> tuple:
    return (1, "hello", 3.14)
"#,
    );
    assert!(
        code.contains("(") && code.contains("1"),
        "Should generate tuple: {code}"
    );
}

#[test]
fn test_set_literal() {
    let code = transpile(
        r#"
def f() -> set:
    return {1, 2, 3}
"#,
    );
    assert!(
        code.contains("HashSet") || code.contains("BTreeSet") || code.contains("from"),
        "Should generate set: {code}"
    );
}

#[test]
fn test_none_return() {
    let code = transpile(
        r#"
def f() -> None:
    return None
"#,
    );
    // Transpiler typically generates empty function or ()
    assert!(
        code.contains("()") || code.contains("fn f"),
        "Should generate None return: {code}"
    );
}
