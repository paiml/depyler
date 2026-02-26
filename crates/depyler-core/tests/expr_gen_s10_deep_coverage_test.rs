// Session 10: Deep coverage tests for expr_gen.rs
// Targets uncovered paths in expression code generation

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

// ============ Chained comparisons ============

#[test]
fn test_s10_chained_comparison() {
    let code = transpile("def in_range(x: int) -> bool:\n    return 0 < x < 100\n");
    assert!(code.contains("fn in_range"), "Should handle chained cmp: {code}");
}

#[test]
fn test_s10_chained_comparison_eq() {
    let code = transpile("def all_same(a: int, b: int, c: int) -> bool:\n    return a == b == c\n");
    assert!(code.contains("fn all_same"), "Should handle == chain: {code}");
}

// ============ Complex boolean expressions ============

#[test]
fn test_s10_complex_boolean_and() {
    let code =
        transpile("def check(a: bool, b: bool, c: bool) -> bool:\n    return a and b and c\n");
    assert!(code.contains("&&"), "Should have && operator: {code}");
}

#[test]
fn test_s10_complex_boolean_or() {
    let code = transpile("def check(a: bool, b: bool, c: bool) -> bool:\n    return a or b or c\n");
    assert!(code.contains("||"), "Should have || operator: {code}");
}

#[test]
fn test_s10_boolean_mixed() {
    let code = transpile(
        "def check(a: bool, b: bool, c: bool) -> bool:\n    return (a and b) or (not c)\n",
    );
    assert!(code.contains("fn check"), "Should handle mixed bool: {code}");
}

// ============ String multiplication ============

#[test]
fn test_s10_string_multiply() {
    let code = transpile("def repeat(s: str, n: int) -> str:\n    return s * n\n");
    assert!(code.contains("fn repeat"), "Should handle str * n: {code}");
    assert!(code.contains("repeat(") || code.contains("*"), "Should have repeat: {code}");
}

// ============ List operations ============

#[test]
fn test_s10_list_multiply() {
    let code = transpile("def zeros(n: int) -> list:\n    return [0] * n\n");
    assert!(code.contains("fn zeros"), "Should handle [0] * n: {code}");
}

#[test]
fn test_s10_list_in_operator() {
    let code =
        transpile("def contains(items: list, target: int) -> bool:\n    return target in items\n");
    assert!(code.contains("fn contains"), "Should handle 'in': {code}");
    assert!(code.contains("contains(") || code.contains("any"), "Should check containment: {code}");
}

#[test]
fn test_s10_list_not_in() {
    let code = transpile(
        "def missing(items: list, target: int) -> bool:\n    return target not in items\n",
    );
    assert!(code.contains("fn missing"), "Should handle 'not in': {code}");
}

#[test]
fn test_s10_list_slice_basic() {
    let code = transpile("def first_three(items: list) -> list:\n    return items[:3]\n");
    assert!(code.contains("fn first_three"), "Should handle slice: {code}");
}

#[test]
fn test_s10_list_slice_negative() {
    let code = transpile("def last_two(items: list) -> list:\n    return items[-2:]\n");
    assert!(code.contains("fn last_two"), "Should handle negative slice: {code}");
}

#[test]
fn test_s10_list_negative_index() {
    let code = transpile("def last(items: list) -> int:\n    return items[-1]\n");
    assert!(code.contains("fn last"), "Should handle negative index: {code}");
}

// ============ Dict operations ============

#[test]
fn test_s10_dict_get_default() {
    let code = transpile("def safe_get(d: dict, key: str) -> int:\n    return d.get(key, 0)\n");
    assert!(code.contains("fn safe_get"), "Should handle dict.get: {code}");
}

#[test]
fn test_s10_dict_setdefault() {
    let code =
        transpile("def init_key(d: dict, key: str) -> int:\n    return d.setdefault(key, 0)\n");
    assert!(code.contains("fn init_key"), "Should handle setdefault: {code}");
}

#[test]
fn test_s10_dict_update() {
    let code = transpile("def merge(a: dict, b: dict) -> None:\n    a.update(b)\n");
    assert!(code.contains("fn merge"), "Should handle dict update: {code}");
}

#[test]
fn test_s10_dict_pop() {
    let code = transpile("def remove_key(d: dict, key: str) -> int:\n    return d.pop(key)\n");
    assert!(code.contains("fn remove_key"), "Should handle dict pop: {code}");
}

// ============ Set operations ============

#[test]
fn test_s10_set_union() {
    let code = transpile("def combine(a: set, b: set) -> set:\n    return a.union(b)\n");
    assert!(code.contains("fn combine"), "Should handle set union: {code}");
}

#[test]
fn test_s10_set_intersection() {
    let code = transpile("def common(a: set, b: set) -> set:\n    return a.intersection(b)\n");
    assert!(code.contains("fn common"), "Should handle set intersection: {code}");
}

#[test]
fn test_s10_set_difference() {
    let code = transpile("def unique_to_a(a: set, b: set) -> set:\n    return a.difference(b)\n");
    assert!(code.contains("fn unique_to_a"), "Should handle set difference: {code}");
}

// ============ String methods ============

#[test]
fn test_s10_str_isdigit() {
    let code = transpile("def is_number(s: str) -> bool:\n    return s.isdigit()\n");
    assert!(code.contains("fn is_number"), "Should handle isdigit: {code}");
}

#[test]
fn test_s10_str_isalpha() {
    let code = transpile("def is_letters(s: str) -> bool:\n    return s.isalpha()\n");
    assert!(code.contains("fn is_letters"), "Should handle isalpha: {code}");
}

#[test]
fn test_s10_str_center() {
    let code = transpile("def pad(s: str) -> str:\n    return s.center(20)\n");
    assert!(code.contains("fn pad"), "Should handle center: {code}");
}

#[test]
fn test_s10_str_zfill() {
    let code = transpile("def pad_num(s: str) -> str:\n    return s.zfill(5)\n");
    assert!(code.contains("fn pad_num"), "Should handle zfill: {code}");
}

#[test]
fn test_s10_str_title() {
    let code = transpile("def title_case(s: str) -> str:\n    return s.title()\n");
    assert!(code.contains("fn title_case"), "Should handle title: {code}");
}

#[test]
fn test_s10_str_capitalize() {
    let code = transpile("def cap(s: str) -> str:\n    return s.capitalize()\n");
    assert!(code.contains("fn cap"), "Should handle capitalize: {code}");
}

#[test]
fn test_s10_str_count() {
    let code = transpile("def count_a(s: str) -> int:\n    return s.count('a')\n");
    assert!(code.contains("fn count_a"), "Should handle count: {code}");
}

#[test]
fn test_s10_str_encode() {
    let code = transpile("def to_bytes(s: str) -> bytes:\n    return s.encode('utf-8')\n");
    assert!(code.contains("fn to_bytes"), "Should handle encode: {code}");
}

#[test]
fn test_s10_str_removeprefix() {
    let code = transpile("def rm_prefix(s: str) -> str:\n    return s.removeprefix('test_')\n");
    assert!(code.contains("fn rm_prefix"), "Should handle removeprefix: {code}");
}

#[test]
fn test_s10_str_removesuffix() {
    let code = transpile("def rm_suffix(s: str) -> str:\n    return s.removesuffix('.py')\n");
    assert!(code.contains("fn rm_suffix"), "Should handle removesuffix: {code}");
}

// ============ Builtin functions ============

#[test]
fn test_s10_builtin_any() {
    let code =
        transpile("def has_positive(items: list) -> bool:\n    return any(x > 0 for x in items)\n");
    assert!(code.contains("fn has_positive"), "Should handle any(): {code}");
}

#[test]
fn test_s10_builtin_all() {
    let code =
        transpile("def all_positive(items: list) -> bool:\n    return all(x > 0 for x in items)\n");
    assert!(code.contains("fn all_positive"), "Should handle all(): {code}");
}

#[test]
fn test_s10_builtin_enumerate() {
    let code = transpile(
        "def with_index(items: list) -> None:\n    for i, item in enumerate(items):\n        print(i, item)\n",
    );
    assert!(code.contains("enumerate"), "Should handle enumerate: {code}");
}

#[test]
fn test_s10_builtin_zip() {
    let code = transpile(
        "def pair_up(a: list, b: list) -> list:\n    result = []\n    for x, y in zip(a, b):\n        result.append((x, y))\n    return result\n",
    );
    assert!(code.contains("zip"), "Should handle zip: {code}");
}

#[test]
fn test_s10_builtin_reversed() {
    let code =
        transpile("def reverse_list(items: list) -> list:\n    return list(reversed(items))\n");
    assert!(code.contains("fn reverse_list"), "Should handle reversed: {code}");
}

#[test]
fn test_s10_builtin_abs() {
    let code = transpile("def absolute(x: int) -> int:\n    return abs(x)\n");
    assert!(code.contains("abs"), "Should handle abs: {code}");
}

#[test]
fn test_s10_builtin_min_max() {
    let code =
        transpile("def clamp(x: int, lo: int, hi: int) -> int:\n    return max(lo, min(x, hi))\n");
    assert!(code.contains("fn clamp"), "Should handle min/max: {code}");
}

#[test]
fn test_s10_builtin_sum() {
    let code = transpile("def total(items: list) -> int:\n    return sum(items)\n");
    assert!(code.contains("fn total"), "Should handle sum: {code}");
}

#[test]
fn test_s10_builtin_len() {
    let code = transpile("def size(items: list) -> int:\n    return len(items)\n");
    assert!(code.contains("fn size"), "Should handle len: {code}");
    assert!(code.contains("len()"), "Should use .len(): {code}");
}

// ============ Type conversions ============

#[test]
fn test_s10_int_to_str() {
    let code = transpile("def num_to_str(x: int) -> str:\n    return str(x)\n");
    assert!(code.contains("fn num_to_str"), "Should handle int to str: {code}");
    assert!(code.contains("to_string()"), "Should use to_string(): {code}");
}

#[test]
fn test_s10_str_to_int() {
    let code = transpile("def parse_int(s: str) -> int:\n    return int(s)\n");
    assert!(code.contains("fn parse_int"), "Should handle str to int: {code}");
    assert!(code.contains("parse"), "Should use parse: {code}");
}

#[test]
fn test_s10_float_to_int() {
    let code = transpile("def truncate(x: float) -> int:\n    return int(x)\n");
    assert!(code.contains("fn truncate"), "Should handle float to int: {code}");
}

#[test]
fn test_s10_bool_to_int() {
    let code = transpile("def flag_to_num(b: bool) -> int:\n    return int(b)\n");
    assert!(code.contains("fn flag_to_num"), "Should handle bool to int: {code}");
}

// ============ Complex nested expressions ============

#[test]
fn test_s10_nested_calls() {
    let code = transpile("def process(x: int) -> int:\n    return abs(min(x, max(0, x - 10)))\n");
    assert!(code.contains("fn process"), "Should handle nested calls: {code}");
}

#[test]
fn test_s10_nested_ternary() {
    let code = transpile(
        "def sign(x: int) -> str:\n    return 'positive' if x > 0 else ('negative' if x < 0 else 'zero')\n",
    );
    assert!(code.contains("fn sign"), "Should handle nested ternary: {code}");
}

#[test]
fn test_s10_method_chain() {
    let code = transpile(
        "def process_text(s: str) -> str:\n    return s.strip().lower().replace(' ', '_')\n",
    );
    assert!(code.contains("fn process_text"), "Should handle method chain: {code}");
}

// ============ List/dict method return values ============

#[test]
fn test_s10_list_pop_with_index() {
    let code = transpile("def remove_first(items: list) -> int:\n    return items.pop(0)\n");
    assert!(code.contains("fn remove_first"), "Should handle pop(0): {code}");
}

#[test]
fn test_s10_list_index() {
    let code = transpile(
        "def find_pos(items: list, target: int) -> int:\n    return items.index(target)\n",
    );
    assert!(code.contains("fn find_pos"), "Should handle index(): {code}");
}

#[test]
fn test_s10_list_count() {
    let code = transpile(
        "def count_item(items: list, target: int) -> int:\n    return items.count(target)\n",
    );
    assert!(code.contains("fn count_item"), "Should handle list count: {code}");
}

#[test]
fn test_s10_list_extend() {
    let code = transpile("def combine_lists(a: list, b: list) -> None:\n    a.extend(b)\n");
    assert!(code.contains("fn combine_lists"), "Should handle extend: {code}");
}

#[test]
fn test_s10_list_insert() {
    let code = transpile("def prepend(items: list, val: int) -> None:\n    items.insert(0, val)\n");
    assert!(code.contains("fn prepend"), "Should handle insert: {code}");
}

#[test]
fn test_s10_list_remove() {
    let code = transpile("def rm(items: list, val: int) -> None:\n    items.remove(val)\n");
    assert!(code.contains("fn rm"), "Should handle remove: {code}");
}

#[test]
fn test_s10_list_clear() {
    let code = transpile("def empty(items: list) -> None:\n    items.clear()\n");
    assert!(code.contains("fn empty"), "Should handle clear: {code}");
}

#[test]
fn test_s10_list_copy() {
    let code = transpile("def clone_list(items: list) -> list:\n    return items.copy()\n");
    assert!(code.contains("fn clone_list"), "Should handle copy: {code}");
}

// ============ Frozenset and set add/discard ============

#[test]
fn test_s10_set_add() {
    let code = transpile("def add_to(s: set, val: int) -> None:\n    s.add(val)\n");
    assert!(code.contains("fn add_to"), "Should handle set add: {code}");
}

#[test]
fn test_s10_set_discard() {
    let code = transpile("def rm_from(s: set, val: int) -> None:\n    s.discard(val)\n");
    assert!(code.contains("fn rm_from"), "Should handle set discard: {code}");
}

// ============ Floor division and modulo ============

#[test]
fn test_s10_floor_div() {
    let code = transpile("def half(x: int) -> int:\n    return x // 2\n");
    assert!(code.contains("fn half"), "Should handle //: {code}");
}

#[test]
fn test_s10_modulo() {
    let code = transpile("def is_even(x: int) -> bool:\n    return x % 2 == 0\n");
    assert!(code.contains("fn is_even"), "Should handle %: {code}");
}

// ============ Unary operations ============

#[test]
fn test_s10_unary_neg() {
    let code = transpile("def negate(x: int) -> int:\n    return -x\n");
    assert!(code.contains("fn negate"), "Should handle unary neg: {code}");
}

#[test]
fn test_s10_unary_not() {
    let code = transpile("def flip(x: bool) -> bool:\n    return not x\n");
    assert!(code.contains("fn flip"), "Should handle unary not: {code}");
}

#[test]
fn test_s10_unary_bitnot() {
    let code = transpile("def invert(x: int) -> int:\n    return ~x\n");
    assert!(code.contains("fn invert"), "Should handle bitnot: {code}");
}

// ============ Global references ============

#[test]
fn test_s10_global_in_function() {
    let code = transpile("PI = 3.14159\ndef circle(r: float) -> float:\n    return PI * r * r\n");
    assert!(code.contains("fn circle"), "Should handle global ref: {code}");
    assert!(code.contains("PI") || code.contains("pi"), "Should reference constant: {code}");
}

// ============ Empty collections ============

#[test]
fn test_s10_empty_list() {
    let code = transpile("def make_list() -> list:\n    return []\n");
    assert!(code.contains("fn make_list"), "Should handle empty list: {code}");
}

#[test]
fn test_s10_empty_dict() {
    let code = transpile("def make_dict() -> dict:\n    return {}\n");
    assert!(code.contains("fn make_dict"), "Should handle empty dict: {code}");
}

#[test]
fn test_s10_empty_set() {
    let code = transpile("def make_set() -> set:\n    return set()\n");
    assert!(code.contains("fn make_set"), "Should handle empty set: {code}");
}

// ============ Complex subscript expressions ============

#[test]
fn test_s10_nested_subscript() {
    let code = transpile(
        "def get_cell(matrix: list, row: int, col: int) -> int:\n    return matrix[row][col]\n",
    );
    assert!(code.contains("fn get_cell"), "Should handle nested subscript: {code}");
}

#[test]
fn test_s10_dict_subscript() {
    let code = transpile("def get_value(d: dict, key: str) -> int:\n    return d[key]\n");
    assert!(code.contains("fn get_value"), "Should handle dict subscript: {code}");
}
