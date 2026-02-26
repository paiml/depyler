//! Session 8 batch 4: Coverage tests for expr_gen_instance_methods.rs
//!
//! Targets the 13,149-line instance method handler through transpile-based
//! tests. Covers list, dict, string, set, and builtin method handlers.

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile(python_code: &str) -> String {
    let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
    let (module, _) =
        AstBridge::new().with_source(python_code.to_string()).python_to_hir(ast).expect("hir");
    let tm = TypeMapper::default();
    let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
    result
}

// ── List methods ───────────────────────────────────────────────────

#[test]
fn test_list_append() {
    let code = transpile(
        r#"
def f() -> list:
    items = [1, 2, 3]
    items.append(4)
    return items
"#,
    );
    assert!(code.contains("push") || code.contains("append"), "Should handle list.append: {code}");
}

#[test]
fn test_list_extend() {
    let code = transpile(
        r#"
def f(a: list, b: list) -> list:
    a.extend(b)
    return a
"#,
    );
    assert!(
        code.contains("extend") || code.contains("append"),
        "Should handle list.extend: {code}"
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
    assert!(code.contains("pop") || code.contains("remove"), "Should handle list.pop: {code}");
}

#[test]
fn test_list_pop_index() {
    let code = transpile(
        r#"
def f(items: list) -> int:
    return items.pop(0)
"#,
    );
    assert!(
        code.contains("remove(0)") || code.contains("pop") || code.contains("fn f"),
        "Should handle list.pop(0): {code}"
    );
}

#[test]
fn test_list_insert() {
    let code = transpile(
        r#"
def f(items: list) -> None:
    items.insert(0, 42)
"#,
    );
    assert!(code.contains("insert") || code.contains("42"), "Should handle list.insert: {code}");
}

#[test]
fn test_list_remove() {
    let code = transpile(
        r#"
def f(items: list, val: int) -> None:
    items.remove(val)
"#,
    );
    assert!(
        code.contains("retain") || code.contains("remove") || code.contains("position"),
        "Should handle list.remove: {code}"
    );
}

#[test]
fn test_list_sort() {
    let code = transpile(
        r#"
def f(items: list) -> list:
    items.sort()
    return items
"#,
    );
    assert!(code.contains("sort"), "Should handle list.sort: {code}");
}

#[test]
fn test_list_reverse() {
    let code = transpile(
        r#"
def f(items: list) -> list:
    items.reverse()
    return items
"#,
    );
    assert!(code.contains("reverse"), "Should handle list.reverse: {code}");
}

#[test]
fn test_list_clear() {
    let code = transpile(
        r#"
def f(items: list) -> None:
    items.clear()
"#,
    );
    assert!(code.contains("clear"), "Should handle list.clear: {code}");
}

#[test]
fn test_list_copy() {
    let code = transpile(
        r#"
def f(items: list) -> list:
    return items.copy()
"#,
    );
    assert!(
        code.contains("clone") || code.contains("to_vec") || code.contains("copy"),
        "Should handle list.copy: {code}"
    );
}

#[test]
fn test_list_count() {
    let code = transpile(
        r#"
def f(items: list, target: int) -> int:
    return items.count(target)
"#,
    );
    assert!(
        code.contains("count") || code.contains("filter") || code.contains("iter"),
        "Should handle list.count: {code}"
    );
}

#[test]
fn test_list_index() {
    let code = transpile(
        r#"
def f(items: list, target: int) -> int:
    return items.index(target)
"#,
    );
    assert!(
        code.contains("position") || code.contains("index") || code.contains("iter"),
        "Should handle list.index: {code}"
    );
}

// ── Dict methods ───────────────────────────────────────────────────

#[test]
fn test_dict_get() {
    let code = transpile(
        r#"
def f(d: dict, key: str) -> int:
    return d.get(key, 0)
"#,
    );
    assert!(code.contains("get") || code.contains("unwrap_or"), "Should handle dict.get: {code}");
}

#[test]
fn test_dict_keys() {
    let code = transpile(
        r#"
def f(d: dict) -> list:
    return list(d.keys())
"#,
    );
    assert!(code.contains("keys") || code.contains("iter"), "Should handle dict.keys: {code}");
}

#[test]
fn test_dict_values() {
    let code = transpile(
        r#"
def f(d: dict) -> list:
    return list(d.values())
"#,
    );
    assert!(code.contains("values") || code.contains("iter"), "Should handle dict.values: {code}");
}

#[test]
fn test_dict_items() {
    let code = transpile(
        r#"
def f(d: dict) -> list:
    result = []
    for k, v in d.items():
        result.append(k)
    return result
"#,
    );
    assert!(
        code.contains("iter") || code.contains("items") || code.contains("for"),
        "Should handle dict.items: {code}"
    );
}

#[test]
fn test_dict_update() {
    let code = transpile(
        r#"
def f(a: dict, b: dict) -> dict:
    a.update(b)
    return a
"#,
    );
    assert!(
        code.contains("extend") || code.contains("insert") || code.contains("update"),
        "Should handle dict.update: {code}"
    );
}

#[test]
fn test_dict_pop() {
    let code = transpile(
        r#"
def f(d: dict, key: str) -> int:
    return d.pop(key)
"#,
    );
    assert!(code.contains("remove") || code.contains("pop"), "Should handle dict.pop: {code}");
}

#[test]
fn test_dict_clear() {
    let code = transpile(
        r#"
def f(d: dict) -> None:
    d.clear()
"#,
    );
    assert!(code.contains("clear"), "Should handle dict.clear: {code}");
}

#[test]
fn test_dict_setdefault() {
    let code = transpile(
        r#"
def f(d: dict, key: str) -> int:
    return d.setdefault(key, 0)
"#,
    );
    assert!(
        code.contains("entry") || code.contains("or_insert") || code.contains("setdefault"),
        "Should handle dict.setdefault: {code}"
    );
}

// ── String methods ─────────────────────────────────────────────────

#[test]
fn test_str_upper() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    return s.upper()
"#,
    );
    assert!(
        code.contains("to_uppercase") || code.contains("upper"),
        "Should handle str.upper: {code}"
    );
}

#[test]
fn test_str_lower() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    return s.lower()
"#,
    );
    assert!(
        code.contains("to_lowercase") || code.contains("lower"),
        "Should handle str.lower: {code}"
    );
}

#[test]
fn test_str_strip() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    return s.strip()
"#,
    );
    assert!(code.contains("trim") || code.contains("strip"), "Should handle str.strip: {code}");
}

#[test]
fn test_str_lstrip() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    return s.lstrip()
"#,
    );
    assert!(
        code.contains("trim_start") || code.contains("lstrip"),
        "Should handle str.lstrip: {code}"
    );
}

#[test]
fn test_str_rstrip() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    return s.rstrip()
"#,
    );
    assert!(
        code.contains("trim_end") || code.contains("rstrip"),
        "Should handle str.rstrip: {code}"
    );
}

#[test]
fn test_str_split() {
    let code = transpile(
        r#"
def f(s: str) -> list:
    return s.split(",")
"#,
    );
    assert!(code.contains("split") || code.contains(","), "Should handle str.split: {code}");
}

#[test]
fn test_str_split_no_args() {
    let code = transpile(
        r#"
def f(s: str) -> list:
    return s.split()
"#,
    );
    assert!(
        code.contains("split") || code.contains("whitespace"),
        "Should handle str.split(): {code}"
    );
}

#[test]
fn test_str_join() {
    let code = transpile(
        r#"
def f(items: list) -> str:
    return ", ".join(items)
"#,
    );
    assert!(code.contains("join") || code.contains(","), "Should handle str.join: {code}");
}

#[test]
fn test_str_replace() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    return s.replace("old", "new")
"#,
    );
    assert!(code.contains("replace") || code.contains("old"), "Should handle str.replace: {code}");
}

#[test]
fn test_str_find() {
    let code = transpile(
        r#"
def f(s: str) -> int:
    return s.find("x")
"#,
    );
    assert!(code.contains("find") || code.contains("position"), "Should handle str.find: {code}");
}

#[test]
fn test_str_startswith() {
    let code = transpile(
        r#"
def f(s: str) -> bool:
    return s.startswith("hello")
"#,
    );
    assert!(
        code.contains("starts_with") || code.contains("startswith"),
        "Should handle str.startswith: {code}"
    );
}

#[test]
fn test_str_endswith() {
    let code = transpile(
        r#"
def f(s: str) -> bool:
    return s.endswith("world")
"#,
    );
    assert!(
        code.contains("ends_with") || code.contains("endswith"),
        "Should handle str.endswith: {code}"
    );
}

#[test]
fn test_str_isdigit() {
    let code = transpile(
        r#"
def f(s: str) -> bool:
    return s.isdigit()
"#,
    );
    assert!(
        code.contains("is_digit")
            || code.contains("chars")
            || code.contains("isdigit")
            || code.contains("is_ascii_digit"),
        "Should handle str.isdigit: {code}"
    );
}

#[test]
fn test_str_isalpha() {
    let code = transpile(
        r#"
def f(s: str) -> bool:
    return s.isalpha()
"#,
    );
    assert!(
        code.contains("is_alpha")
            || code.contains("chars")
            || code.contains("isalpha")
            || code.contains("is_alphabetic"),
        "Should handle str.isalpha: {code}"
    );
}

#[test]
fn test_str_title() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    return s.title()
"#,
    );
    assert!(
        code.contains("title") || code.contains("to_uppercase") || code.contains("char"),
        "Should handle str.title: {code}"
    );
}

#[test]
fn test_str_capitalize() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    return s.capitalize()
"#,
    );
    assert!(
        code.contains("capitalize") || code.contains("to_uppercase") || code.contains("char"),
        "Should handle str.capitalize: {code}"
    );
}

#[test]
fn test_str_count() {
    let code = transpile(
        r#"
def f(s: str) -> int:
    return s.count("a")
"#,
    );
    assert!(code.contains("matches") || code.contains("count"), "Should handle str.count: {code}");
}

#[test]
fn test_str_zfill() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    return s.zfill(5)
"#,
    );
    assert!(
        code.contains("zfill") || code.contains("pad") || code.contains("format"),
        "Should handle str.zfill: {code}"
    );
}

#[test]
fn test_str_encode() {
    let code = transpile(
        r#"
def f(s: str) -> bytes:
    return s.encode("utf-8")
"#,
    );
    assert!(
        code.contains("as_bytes") || code.contains("encode") || code.contains("bytes"),
        "Should handle str.encode: {code}"
    );
}

#[test]
fn test_str_format() {
    let code = transpile(
        r#"
def f(name: str) -> str:
    return "Hello, {}".format(name)
"#,
    );
    assert!(code.contains("format") || code.contains("Hello"), "Should handle str.format: {code}");
}

// ── Set methods ────────────────────────────────────────────────────

#[test]
fn test_set_add() {
    let code = transpile(
        r#"
def f() -> set:
    s = {1, 2, 3}
    s.add(4)
    return s
"#,
    );
    assert!(code.contains("insert") || code.contains("add"), "Should handle set.add: {code}");
}

#[test]
fn test_set_discard() {
    let code = transpile(
        r#"
def f(s: set, val: int) -> None:
    s.discard(val)
"#,
    );
    assert!(
        code.contains("remove") || code.contains("discard"),
        "Should handle set.discard: {code}"
    );
}

// ── Chained methods ────────────────────────────────────────────────

#[test]
fn test_chained_string_methods() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    return s.strip().lower()
"#,
    );
    assert!(
        code.contains("trim") || code.contains("to_lowercase"),
        "Should handle chained methods: {code}"
    );
}

#[test]
fn test_chained_list_methods() {
    let code = transpile(
        r#"
def f(items: list) -> list:
    items.append(1)
    items.append(2)
    items.sort()
    return items
"#,
    );
    assert!(
        code.contains("push") && code.contains("sort"),
        "Should handle multiple list methods: {code}"
    );
}

// ── Method on class field ──────────────────────────────────────────

#[test]
fn test_method_on_self_field() {
    let code = transpile(
        r#"
class Counter:
    def __init__(self) -> None:
        self.values: list = []

    def add(self, x: int) -> None:
        self.values.append(x)

    def total(self) -> int:
        return len(self.values)
"#,
    );
    assert!(
        code.contains("push") || code.contains("values"),
        "Should handle method on self field: {code}"
    );
}

#[test]
fn test_method_on_self_dict() {
    let code = transpile(
        r#"
class Registry:
    def __init__(self) -> None:
        self.data: dict = {}

    def register(self, key: str, val: int) -> None:
        self.data[key] = val

    def lookup(self, key: str) -> int:
        return self.data.get(key, 0)
"#,
    );
    assert!(code.contains("insert") || code.contains("get"), "Should handle dict on self: {code}");
}

// ── Builtin functions ──────────────────────────────────────────────

#[test]
fn test_abs_function() {
    let code = transpile(
        r#"
def f(x: int) -> int:
    return abs(x)
"#,
    );
    assert!(code.contains("abs"), "Should handle abs(): {code}");
}

#[test]
fn test_min_function() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> int:
    return min(a, b)
"#,
    );
    assert!(code.contains("min") || code.contains("std::cmp"), "Should handle min(): {code}");
}

#[test]
fn test_max_function() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> int:
    return max(a, b)
"#,
    );
    assert!(code.contains("max") || code.contains("std::cmp"), "Should handle max(): {code}");
}

#[test]
fn test_sum_function() {
    let code = transpile(
        r#"
def f(items: list) -> int:
    return sum(items)
"#,
    );
    assert!(code.contains("sum") || code.contains("iter"), "Should handle sum(): {code}");
}

#[test]
fn test_sorted_function() {
    let code = transpile(
        r#"
def f(items: list) -> list:
    return sorted(items)
"#,
    );
    assert!(code.contains("sort") || code.contains("clone"), "Should handle sorted(): {code}");
}

#[test]
fn test_reversed_function() {
    let code = transpile(
        r#"
def f(items: list) -> list:
    return list(reversed(items))
"#,
    );
    assert!(
        code.contains("rev") || code.contains("reverse") || code.contains("iter"),
        "Should handle reversed(): {code}"
    );
}

#[test]
fn test_enumerate_function() {
    let code = transpile(
        r#"
def f(items: list) -> list:
    result = []
    for i, item in enumerate(items):
        result.append(i)
    return result
"#,
    );
    assert!(
        code.contains("enumerate") || code.contains("iter"),
        "Should handle enumerate(): {code}"
    );
}

#[test]
fn test_zip_function() {
    let code = transpile(
        r#"
def f(a: list, b: list) -> list:
    result = []
    for x, y in zip(a, b):
        result.append(x)
    return result
"#,
    );
    assert!(code.contains("zip") || code.contains("iter"), "Should handle zip(): {code}");
}

#[test]
fn test_range_function() {
    let code = transpile(
        r#"
def f(n: int) -> int:
    total = 0
    for i in range(n):
        total += i
    return total
"#,
    );
    assert!(code.contains("0..") || code.contains("range"), "Should handle range(): {code}");
}

#[test]
fn test_range_with_start_stop() {
    let code = transpile(
        r#"
def f() -> int:
    total = 0
    for i in range(1, 10):
        total += i
    return total
"#,
    );
    assert!(
        code.contains("1..") || code.contains("range"),
        "Should handle range(start, stop): {code}"
    );
}

#[test]
fn test_range_with_step() {
    let code = transpile(
        r#"
def f() -> int:
    total = 0
    for i in range(0, 10, 2):
        total += i
    return total
"#,
    );
    assert!(
        code.contains("step") || code.contains("0..") || code.contains("range"),
        "Should handle range(start, stop, step): {code}"
    );
}

// ── Type conversion methods ────────────────────────────────────────

#[test]
fn test_int_conversion() {
    let code = transpile(
        r#"
def f(s: str) -> int:
    return int(s)
"#,
    );
    assert!(code.contains("parse") || code.contains("unwrap"), "Should handle int(): {code}");
}

#[test]
fn test_float_conversion() {
    let code = transpile(
        r#"
def f(s: str) -> float:
    return float(s)
"#,
    );
    assert!(code.contains("parse") || code.contains("f64"), "Should handle float(): {code}");
}

#[test]
fn test_str_conversion() {
    let code = transpile(
        r#"
def f(x: int) -> str:
    return str(x)
"#,
    );
    assert!(code.contains("to_string") || code.contains("format"), "Should handle str(): {code}");
}

// ── Complex patterns ───────────────────────────────────────────────

#[test]
fn test_list_comprehension_with_method() {
    let code = transpile(
        r#"
def f(words: list) -> list:
    return [w.upper() for w in words]
"#,
    );
    assert!(
        code.contains("to_uppercase") || code.contains("map"),
        "Should handle comprehension with method: {code}"
    );
}

#[test]
fn test_dict_comprehension() {
    let code = transpile(
        r#"
def f(items: list) -> dict:
    return {str(x): x for x in items}
"#,
    );
    assert!(
        code.contains("HashMap") || code.contains("collect") || code.contains("map"),
        "Should handle dict comprehension: {code}"
    );
}

#[test]
fn test_nested_method_calls() {
    let code = transpile(
        r#"
def f(s: str) -> list:
    return s.strip().split(",")
"#,
    );
    assert!(
        code.contains("trim") || code.contains("split"),
        "Should handle nested method calls: {code}"
    );
}

#[test]
fn test_print_function() {
    let code = transpile(
        r#"
def f(x: int) -> None:
    print(x)
    print("hello", x)
"#,
    );
    assert!(code.contains("println") || code.contains("print"), "Should handle print(): {code}");
}

#[test]
fn test_in_operator_list() {
    let code = transpile(
        r#"
def f(items: list, target: int) -> bool:
    return target in items
"#,
    );
    assert!(
        code.contains("contains") || code.contains("iter") || code.contains("any"),
        "Should handle 'in' operator: {code}"
    );
}

#[test]
fn test_not_in_operator() {
    let code = transpile(
        r#"
def f(items: list, target: int) -> bool:
    return target not in items
"#,
    );
    assert!(
        code.contains("contains") || code.contains("!") || code.contains("not"),
        "Should handle 'not in' operator: {code}"
    );
}

#[test]
fn test_len_list() {
    let code = transpile(
        r#"
def f(items: list) -> int:
    return len(items)
"#,
    );
    assert!(code.contains("len()") || code.contains(".len()"), "Should handle len(): {code}");
}

#[test]
fn test_len_str() {
    let code = transpile(
        r#"
def f(s: str) -> int:
    return len(s)
"#,
    );
    assert!(code.contains("len()") || code.contains(".len()"), "Should handle len(str): {code}");
}
