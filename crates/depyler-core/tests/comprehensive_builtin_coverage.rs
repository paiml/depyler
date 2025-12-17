//! Comprehensive Builtin Coverage Tests for expr_gen.rs
//!
//! These tests target the convert_*_builtin functions to improve coverage
//! in the expression generator module.

use depyler_core::DepylerPipeline;

fn transpile(python: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(python).expect("Transpilation should succeed")
}

// ============================================================================
// Numeric Builtins
// ============================================================================

#[test]
fn test_coverage_abs_builtin_int() {
    let code = transpile("def f(x: int) -> int:\n    return abs(x)");
    assert!(code.contains("abs") || code.contains(".abs()"));
}

#[test]
fn test_coverage_abs_builtin_float() {
    let code = transpile("def f(x: float) -> float:\n    return abs(x)");
    assert!(code.contains("abs") || code.contains(".abs()"));
}

#[test]
fn test_coverage_round_builtin_no_decimals() {
    let code = transpile("def f(x: float) -> int:\n    return round(x)");
    assert!(code.contains("round"));
}

#[test]
fn test_coverage_round_builtin_with_decimals() {
    let code = transpile("def f(x: float) -> float:\n    return round(x, 2)");
    assert!(code.contains("round") || code.contains("10_f64.powi"));
}

#[test]
fn test_coverage_pow_builtin_two_args() {
    let code = transpile("def f(x: int, y: int) -> int:\n    return pow(x, y)");
    assert!(code.contains("pow") || code.contains(".pow("));
}

#[test]
fn test_coverage_pow_builtin_three_args() {
    let code = transpile("def f(x: int, y: int, z: int) -> int:\n    return pow(x, y, z)");
    assert!(code.contains("pow") || code.contains("%"));
}

#[test]
fn test_coverage_divmod_builtin() {
    let code = transpile("def f(a: int, b: int) -> tuple:\n    return divmod(a, b)");
    assert!(code.contains("/") && code.contains("%"));
}

#[test]
fn test_coverage_min_builtin_two_args() {
    let code = transpile("def f(a: int, b: int) -> int:\n    return min(a, b)");
    assert!(code.contains("min") || code.contains(".min("));
}

#[test]
fn test_coverage_min_builtin_iterable() {
    let code = transpile("def f(nums: list[int]) -> int:\n    return min(nums)");
    assert!(code.contains("min") || code.contains(".min()"));
}

#[test]
fn test_coverage_max_builtin_two_args() {
    let code = transpile("def f(a: int, b: int) -> int:\n    return max(a, b)");
    assert!(code.contains("max") || code.contains(".max("));
}

#[test]
fn test_coverage_max_builtin_iterable() {
    let code = transpile("def f(nums: list[int]) -> int:\n    return max(nums)");
    assert!(code.contains("max") || code.contains(".max()"));
}

#[test]
fn test_coverage_sum_builtin() {
    let code = transpile("def f(nums: list[int]) -> int:\n    return sum(nums)");
    assert!(code.contains("sum") || code.contains(".sum()"));
}

#[test]
fn test_coverage_sum_builtin_with_start() {
    let code = transpile("def f(nums: list[int]) -> int:\n    return sum(nums, 10)");
    // sum with start value generates fold pattern
    assert!(code.contains("fold") || code.contains("sum") || code.contains(".sum()"));
}

// ============================================================================
// Type Conversion Builtins
// ============================================================================

#[test]
fn test_coverage_int_from_string() {
    let code = transpile("def f(s: str) -> int:\n    return int(s)");
    assert!(code.contains("parse"));
}

#[test]
fn test_coverage_int_from_float() {
    let code = transpile("def f(x: float) -> int:\n    return int(x)");
    assert!(code.contains("as i") || code.contains("i32") || code.contains("i64"));
}

#[test]
fn test_coverage_float_from_string() {
    let code = transpile("def f(s: str) -> float:\n    return float(s)");
    assert!(code.contains("parse"));
}

#[test]
fn test_coverage_float_from_int() {
    let code = transpile("def f(x: int) -> float:\n    return float(x)");
    assert!(code.contains("as f") || code.contains("f64"));
}

#[test]
fn test_coverage_str_from_int() {
    let code = transpile("def f(x: int) -> str:\n    return str(x)");
    assert!(code.contains("to_string") || code.contains("format!"));
}

#[test]
fn test_coverage_bool_from_int() {
    let code = transpile("def f(x: int) -> bool:\n    return bool(x)");
    assert!(code.contains("!= 0") || code.contains("bool"));
}

#[test]
fn test_coverage_bool_from_string() {
    let code = transpile("def f(s: str) -> bool:\n    return bool(s)");
    assert!(code.contains("is_empty") || code.contains("!"));
}

// ============================================================================
// Format Conversion Builtins
// ============================================================================

#[test]
fn test_coverage_hex_builtin() {
    let code = transpile("def f(x: int) -> str:\n    return hex(x)");
    assert!(code.contains("format!") || code.contains(":x"));
}

#[test]
fn test_coverage_bin_builtin() {
    let code = transpile("def f(x: int) -> str:\n    return bin(x)");
    assert!(code.contains("format!") || code.contains(":b"));
}

#[test]
fn test_coverage_oct_builtin() {
    let code = transpile("def f(x: int) -> str:\n    return oct(x)");
    assert!(code.contains("format!") || code.contains(":o"));
}

#[test]
fn test_coverage_chr_builtin() {
    let code = transpile("def f(x: int) -> str:\n    return chr(x)");
    assert!(code.contains("char") || code.contains("from_u32"));
}

#[test]
fn test_coverage_ord_builtin() {
    let code = transpile("def f(c: str) -> int:\n    return ord(c)");
    assert!(code.contains("chars") || code.contains("as u32") || code.contains("as i"));
}

#[test]
fn test_coverage_repr_builtin() {
    let code = transpile("def f(x: int) -> str:\n    return repr(x)");
    assert!(code.contains("format!") || code.contains(":?"));
}

#[test]
fn test_coverage_hash_builtin() {
    let code = transpile("def f(s: str) -> int:\n    return hash(s)");
    assert!(code.contains("hash") || code.contains("Hash"));
}

// ============================================================================
// Collection Builtins
// ============================================================================

#[test]
fn test_coverage_len_builtin_list() {
    let code = transpile("def f(items: list[int]) -> int:\n    return len(items)");
    assert!(code.contains(".len()"));
}

#[test]
fn test_coverage_len_builtin_string() {
    let code = transpile("def f(s: str) -> int:\n    return len(s)");
    assert!(code.contains(".len()") || code.contains(".chars().count()"));
}

#[test]
fn test_coverage_len_builtin_dict() {
    let code = transpile("def f(d: dict) -> int:\n    return len(d)");
    assert!(code.contains(".len()"));
}

#[test]
fn test_coverage_sorted_builtin() {
    let code = transpile("def f(items: list[int]) -> list[int]:\n    return sorted(items)");
    assert!(code.contains("sort") || code.contains("clone"));
}

#[test]
fn test_coverage_sorted_builtin_reverse() {
    let code = transpile("def f(items: list[int]) -> list[int]:\n    return sorted(items, reverse=True)");
    assert!(code.contains("sort") || code.contains("reverse"));
}

#[test]
fn test_coverage_reversed_builtin() {
    let code = transpile("def f(items: list[int]) -> list[int]:\n    return list(reversed(items))");
    assert!(code.contains("rev()") || code.contains("reverse"));
}

#[test]
fn test_coverage_enumerate_builtin() {
    let code = transpile(r#"
def f(items: list[str]) -> list[tuple]:
    result = []
    for i, item in enumerate(items):
        result.append((i, item))
    return result
"#);
    assert!(code.contains("enumerate()"));
}

#[test]
fn test_coverage_enumerate_builtin_with_start() {
    let code = transpile(r#"
def f(items: list[str]) -> list[tuple]:
    result = []
    for i, item in enumerate(items, 1):
        result.append((i, item))
    return result
"#);
    assert!(code.contains("enumerate"));
}

#[test]
fn test_coverage_zip_builtin() {
    let code = transpile(r#"
def f(a: list[int], b: list[str]) -> list[tuple]:
    result = []
    for x, y in zip(a, b):
        result.append((x, y))
    return result
"#);
    assert!(code.contains("zip"));
}

#[test]
fn test_coverage_any_builtin() {
    let code = transpile("def f(items: list[bool]) -> bool:\n    return any(items)");
    assert!(code.contains("any") || code.contains(".any("));
}

#[test]
fn test_coverage_all_builtin() {
    let code = transpile("def f(items: list[bool]) -> bool:\n    return all(items)");
    assert!(code.contains("all") || code.contains(".all("));
}

#[test]
fn test_coverage_filter_builtin() {
    let code = transpile(r#"
def is_even(x: int) -> bool:
    return x % 2 == 0

def f(nums: list[int]) -> list[int]:
    return list(filter(is_even, nums))
"#);
    assert!(code.contains("filter") || code.contains(".filter("));
}

#[test]
fn test_coverage_map_builtin() {
    let code = transpile(r#"
def double(x: int) -> int:
    return x * 2

def f(nums: list[int]) -> list[int]:
    return list(map(double, nums))
"#);
    assert!(code.contains("map") || code.contains(".map("));
}

// ============================================================================
// Range Builtin
// ============================================================================

#[test]
fn test_coverage_range_single_arg() {
    let code = transpile(r#"
def f() -> list[int]:
    result = []
    for i in range(5):
        result.append(i)
    return result
"#);
    assert!(code.contains("0..") || code.contains("range"));
}

#[test]
fn test_coverage_range_two_args() {
    let code = transpile(r#"
def f() -> list[int]:
    result = []
    for i in range(1, 10):
        result.append(i)
    return result
"#);
    assert!(code.contains("..") || code.contains("range"));
}

#[test]
fn test_coverage_range_three_args() {
    let code = transpile(r#"
def f() -> list[int]:
    result = []
    for i in range(0, 10, 2):
        result.append(i)
    return result
"#);
    assert!(code.contains("step_by") || code.contains(".."));
}

// ============================================================================
// Collection Constructors
// ============================================================================

#[test]
fn test_coverage_list_constructor_empty() {
    let code = transpile("def f() -> list[int]:\n    return list()");
    assert!(code.contains("Vec") || code.contains("vec!"));
}

#[test]
fn test_coverage_list_constructor_from_string() {
    let code = transpile("def f(s: str) -> list[str]:\n    return list(s)");
    assert!(code.contains("chars") || code.contains("collect"));
}

#[test]
fn test_coverage_dict_constructor_empty() {
    let code = transpile("def f() -> dict:\n    return dict()");
    assert!(code.contains("HashMap") || code.contains("new()"));
}

#[test]
fn test_coverage_set_constructor_empty() {
    let code = transpile("def f() -> set:\n    return set()");
    assert!(code.contains("HashSet") || code.contains("new()"));
}

#[test]
fn test_coverage_tuple_constructor() {
    let code = transpile("def f(items: list[int]) -> tuple:\n    return tuple(items)");
    assert!(code.contains("tuple") || code.contains("Vec") || code.contains("collect"));
}

#[test]
fn test_coverage_frozenset_constructor() {
    let code = transpile("def f(items: list[int]) -> frozenset:\n    return frozenset(items)");
    assert!(code.contains("HashSet") || code.contains("collect"));
}

// ============================================================================
// Bytes/Bytearray Builtins
// ============================================================================

#[test]
fn test_coverage_bytes_from_string() {
    let code = transpile(r#"def f(s: str) -> bytes:
    return bytes(s, "utf-8")"#);
    assert!(code.contains("as_bytes") || code.contains("Vec<u8>") || code.contains("bytes"));
}

#[test]
fn test_coverage_bytes_from_int() {
    let code = transpile("def f(n: int) -> bytes:\n    return bytes(n)");
    assert!(code.contains("vec!") || code.contains("Vec"));
}

#[test]
fn test_coverage_bytearray_from_string() {
    let code = transpile(r#"def f(s: str) -> bytearray:
    return bytearray(s, "utf-8")"#);
    assert!(code.contains("as_bytes") || code.contains("Vec<u8>") || code.contains("bytearray"));
}

// ============================================================================
// Reflection Builtins
// ============================================================================

#[test]
fn test_coverage_type_builtin() {
    let code = transpile("def f(x: int) -> str:\n    return str(type(x))");
    assert!(code.contains("type") || code.contains("type_name"));
}

#[test]
fn test_coverage_isinstance_builtin() {
    let code = transpile("def f(x: int) -> bool:\n    return isinstance(x, int)");
    assert!(code.contains("true") || code.contains("bool"));
}

#[test]
fn test_coverage_callable_builtin() {
    let code = transpile("def f(x: int) -> bool:\n    return callable(x)");
    assert!(code.contains("false") || code.contains("callable") || code.contains("bool"));
}

#[test]
fn test_coverage_iter_builtin() {
    let code = transpile(r#"
def f(items: list[int]) -> list[int]:
    it = iter(items)
    result = []
    for x in it:
        result.append(x)
    return result
"#);
    assert!(code.contains("iter") || code.contains("into_iter"));
}

#[test]
fn test_coverage_next_builtin() {
    let code = transpile(r#"
def f(items: list[int]) -> int:
    it = iter(items)
    return next(it)
"#);
    assert!(code.contains("next") || code.contains(".next()"));
}

#[test]
fn test_coverage_next_builtin_with_default() {
    let code = transpile(r#"
def f(items: list[int]) -> int:
    it = iter(items)
    return next(it, 0)
"#);
    assert!(code.contains("next") || code.contains("unwrap_or"));
}

// ============================================================================
// I/O Builtins
// ============================================================================

#[test]
fn test_coverage_print_builtin_simple() {
    let code = transpile(r#"def f():
    print("hello")"#);
    assert!(code.contains("println!") || code.contains("print"));
}

#[test]
fn test_coverage_print_builtin_multiple_args() {
    let code = transpile(r#"def f(x: int, y: int):
    print(x, y)"#);
    assert!(code.contains("println!") || code.contains("print"));
}

#[test]
fn test_coverage_print_builtin_with_sep() {
    let code = transpile(r#"def f(x: int, y: int):
    print(x, y, sep=", ")"#);
    assert!(code.contains("println!") || code.contains("print"));
}

#[test]
fn test_coverage_print_builtin_with_end() {
    let code = transpile(r#"def f(x: int):
    print(x, end="")"#);
    assert!(code.contains("print!") || code.contains("print"));
}

#[test]
fn test_coverage_input_builtin() {
    let code = transpile(r#"def f() -> str:
    return input("Enter: ")"#);
    assert!(code.contains("stdin") || code.contains("read_line") || code.contains("input"));
}

#[test]
fn test_coverage_open_builtin_read() {
    let code = transpile(r#"def f(path: str) -> str:
    with open(path, "r") as f:
        return f.read()"#);
    assert!(code.contains("File") || code.contains("open") || code.contains("read"));
}

#[test]
fn test_coverage_open_builtin_write() {
    let code = transpile(r#"def f(path: str, content: str):
    with open(path, "w") as f:
        f.write(content)"#);
    assert!(code.contains("File") || code.contains("open") || code.contains("write"));
}

// ============================================================================
// String Methods
// ============================================================================

#[test]
fn test_coverage_string_upper() {
    let code = transpile("def f(s: str) -> str:\n    return s.upper()");
    assert!(code.contains("to_uppercase"));
}

#[test]
fn test_coverage_string_lower() {
    let code = transpile("def f(s: str) -> str:\n    return s.lower()");
    assert!(code.contains("to_lowercase"));
}

#[test]
fn test_coverage_string_strip() {
    let code = transpile("def f(s: str) -> str:\n    return s.strip()");
    assert!(code.contains("trim"));
}

#[test]
fn test_coverage_string_lstrip() {
    let code = transpile("def f(s: str) -> str:\n    return s.lstrip()");
    assert!(code.contains("trim_start"));
}

#[test]
fn test_coverage_string_rstrip() {
    let code = transpile("def f(s: str) -> str:\n    return s.rstrip()");
    assert!(code.contains("trim_end"));
}

#[test]
fn test_coverage_string_split() {
    let code = transpile("def f(s: str) -> list[str]:\n    return s.split()");
    assert!(code.contains("split"));
}

#[test]
fn test_coverage_string_split_with_sep() {
    let code = transpile(r#"def f(s: str) -> list[str]:
    return s.split(",")"#);
    assert!(code.contains("split"));
}

#[test]
fn test_coverage_string_rsplit() {
    let code = transpile(r#"def f(s: str) -> list[str]:
    return s.rsplit(",")"#);
    assert!(code.contains("rsplit"));
}

#[test]
fn test_coverage_string_join() {
    let code = transpile(r#"def f(items: list[str]) -> str:
    return ", ".join(items)"#);
    assert!(code.contains("join"));
}

#[test]
fn test_coverage_string_replace() {
    let code = transpile(r#"def f(s: str) -> str:
    return s.replace("a", "b")"#);
    assert!(code.contains("replace"));
}

#[test]
fn test_coverage_string_find() {
    let code = transpile(r#"def f(s: str) -> int:
    return s.find("x")"#);
    assert!(code.contains("find") || code.contains("position"));
}

#[test]
fn test_coverage_string_rfind() {
    let code = transpile(r#"def f(s: str) -> int:
    return s.rfind("x")"#);
    assert!(code.contains("rfind") || code.contains("rposition"));
}

#[test]
fn test_coverage_string_count() {
    let code = transpile(r#"def f(s: str) -> int:
    return s.count("a")"#);
    assert!(code.contains("matches") || code.contains("count"));
}

#[test]
fn test_coverage_string_startswith() {
    let code = transpile(r#"def f(s: str) -> bool:
    return s.startswith("pre")"#);
    assert!(code.contains("starts_with"));
}

#[test]
fn test_coverage_string_endswith() {
    let code = transpile(r#"def f(s: str) -> bool:
    return s.endswith("suf")"#);
    assert!(code.contains("ends_with"));
}

#[test]
fn test_coverage_string_isdigit() {
    let code = transpile("def f(s: str) -> bool:\n    return s.isdigit()");
    assert!(code.contains("is_ascii_digit") || code.contains("chars().all"));
}

#[test]
fn test_coverage_string_isalpha() {
    let code = transpile("def f(s: str) -> bool:\n    return s.isalpha()");
    assert!(code.contains("is_alphabetic") || code.contains("chars().all"));
}

#[test]
fn test_coverage_string_isalnum() {
    let code = transpile("def f(s: str) -> bool:\n    return s.isalnum()");
    assert!(code.contains("is_alphanumeric") || code.contains("chars().all"));
}

#[test]
fn test_coverage_string_isspace() {
    let code = transpile("def f(s: str) -> bool:\n    return s.isspace()");
    assert!(code.contains("is_whitespace") || code.contains("chars().all"));
}

#[test]
fn test_coverage_string_capitalize() {
    let code = transpile("def f(s: str) -> str:\n    return s.capitalize()");
    assert!(code.contains("uppercase") || code.contains("capitalize"));
}

#[test]
fn test_coverage_string_title() {
    let code = transpile("def f(s: str) -> str:\n    return s.title()");
    assert!(code.contains("title") || code.contains("split"));
}

#[test]
fn test_coverage_string_zfill() {
    let code = transpile("def f(s: str) -> str:\n    return s.zfill(5)");
    assert!(code.contains("format!") || code.contains("zfill") || code.contains("0>"));
}

#[test]
fn test_coverage_string_center() {
    let code = transpile("def f(s: str) -> str:\n    return s.center(10)");
    assert!(code.contains("format!") || code.contains("center") || code.contains("^"));
}

#[test]
fn test_coverage_string_ljust() {
    let code = transpile("def f(s: str) -> str:\n    return s.ljust(10)");
    assert!(code.contains("format!") || code.contains("<"));
}

#[test]
fn test_coverage_string_rjust() {
    let code = transpile("def f(s: str) -> str:\n    return s.rjust(10)");
    assert!(code.contains("format!") || code.contains(">"));
}

// ============================================================================
// List Methods
// ============================================================================

#[test]
fn test_coverage_list_append() {
    let code = transpile(r#"def f(items: list[int]):
    items.append(1)"#);
    assert!(code.contains("push"));
}

#[test]
fn test_coverage_list_extend() {
    let code = transpile(r#"def f(items: list[int], more: list[int]):
    items.extend(more)"#);
    assert!(code.contains("extend"));
}

#[test]
fn test_coverage_list_insert() {
    let code = transpile(r#"def f(items: list[int]):
    items.insert(0, 1)"#);
    assert!(code.contains("insert"));
}

#[test]
fn test_coverage_list_remove() {
    let code = transpile(r#"def f(items: list[int]):
    items.remove(1)"#);
    assert!(code.contains("retain") || code.contains("remove"));
}

#[test]
fn test_coverage_list_pop() {
    let code = transpile("def f(items: list[int]) -> int:\n    return items.pop()");
    assert!(code.contains("pop"));
}

#[test]
fn test_coverage_list_pop_index() {
    let code = transpile("def f(items: list[int]) -> int:\n    return items.pop(0)");
    assert!(code.contains("remove") || code.contains("pop"));
}

#[test]
fn test_coverage_list_clear() {
    let code = transpile(r#"def f(items: list[int]):
    items.clear()"#);
    assert!(code.contains("clear"));
}

#[test]
fn test_coverage_list_index() {
    let code = transpile("def f(items: list[int]) -> int:\n    return items.index(5)");
    assert!(code.contains("position") || code.contains("index"));
}

#[test]
fn test_coverage_list_count() {
    let code = transpile("def f(items: list[int]) -> int:\n    return items.count(5)");
    assert!(code.contains("filter") || code.contains("count"));
}

#[test]
fn test_coverage_list_sort() {
    let code = transpile(r#"def f(items: list[int]):
    items.sort()"#);
    assert!(code.contains("sort"));
}

#[test]
fn test_coverage_list_sort_reverse() {
    let code = transpile(r#"def f(items: list[int]):
    items.sort(reverse=True)"#);
    assert!(code.contains("sort") || code.contains("reverse"));
}

#[test]
fn test_coverage_list_reverse() {
    let code = transpile(r#"def f(items: list[int]):
    items.reverse()"#);
    assert!(code.contains("reverse"));
}

#[test]
fn test_coverage_list_copy() {
    let code = transpile("def f(items: list[int]) -> list[int]:\n    return items.copy()");
    assert!(code.contains("clone"));
}

// ============================================================================
// Dict Methods
// ============================================================================

#[test]
fn test_coverage_dict_get() {
    let code = transpile(r#"def f(d: dict) -> int:
    return d.get("key", 0)"#);
    assert!(code.contains("get") || code.contains("unwrap_or"));
}

#[test]
fn test_coverage_dict_keys() {
    let code = transpile("def f(d: dict) -> list[str]:\n    return list(d.keys())");
    assert!(code.contains("keys"));
}

#[test]
fn test_coverage_dict_values() {
    let code = transpile("def f(d: dict) -> list[int]:\n    return list(d.values())");
    assert!(code.contains("values"));
}

#[test]
fn test_coverage_dict_items() {
    let code = transpile("def f(d: dict) -> list[tuple]:\n    return list(d.items())");
    assert!(code.contains("iter") || code.contains("items"));
}

#[test]
fn test_coverage_dict_pop() {
    let code = transpile(r#"def f(d: dict) -> int:
    return d.pop("key")"#);
    assert!(code.contains("remove"));
}

#[test]
fn test_coverage_dict_pop_default() {
    let code = transpile(r#"def f(d: dict) -> int:
    return d.pop("key", 0)"#);
    assert!(code.contains("remove") || code.contains("unwrap_or"));
}

#[test]
fn test_coverage_dict_update() {
    let code = transpile(r#"def f(d1: dict, d2: dict):
    d1.update(d2)"#);
    // dict.update() generates a for loop with insert
    assert!(code.contains("extend") || code.contains("update") || code.contains("insert"));
}

#[test]
fn test_coverage_dict_clear() {
    let code = transpile(r#"def f(d: dict):
    d.clear()"#);
    assert!(code.contains("clear"));
}

#[test]
fn test_coverage_dict_setdefault() {
    let code = transpile(r#"def f(d: dict) -> int:
    return d.setdefault("key", 0)"#);
    assert!(code.contains("entry") || code.contains("or_insert"));
}

// ============================================================================
// Set Methods
// ============================================================================

#[test]
fn test_coverage_set_add() {
    let code = transpile(r#"def f(s: set):
    s.add(1)"#);
    assert!(code.contains("insert"));
}

#[test]
fn test_coverage_set_remove() {
    let code = transpile(r#"def f(s: set):
    s.remove(1)"#);
    assert!(code.contains("remove"));
}

#[test]
fn test_coverage_set_discard() {
    let code = transpile(r#"def f(s: set):
    s.discard(1)"#);
    assert!(code.contains("remove") || code.contains("discard"));
}

#[test]
fn test_coverage_set_pop() {
    let code = transpile("def f(s: set) -> int:\n    return s.pop()");
    assert!(code.contains("iter") || code.contains("next") || code.contains("remove"));
}

#[test]
fn test_coverage_set_clear() {
    let code = transpile(r#"def f(s: set):
    s.clear()"#);
    assert!(code.contains("clear"));
}

#[test]
fn test_coverage_set_union() {
    let code = transpile("def f(s1: set, s2: set) -> set:\n    return s1.union(s2)");
    assert!(code.contains("union") || code.contains("extend"));
}

#[test]
fn test_coverage_set_intersection() {
    let code = transpile("def f(s1: set, s2: set) -> set:\n    return s1.intersection(s2)");
    assert!(code.contains("intersection") || code.contains("retain"));
}

#[test]
fn test_coverage_set_difference() {
    let code = transpile("def f(s1: set, s2: set) -> set:\n    return s1.difference(s2)");
    assert!(code.contains("difference") || code.contains("filter"));
}

#[test]
fn test_coverage_set_issubset() {
    let code = transpile("def f(s1: set, s2: set) -> bool:\n    return s1.issubset(s2)");
    assert!(code.contains("is_subset") || code.contains("all"));
}

#[test]
fn test_coverage_set_issuperset() {
    let code = transpile("def f(s1: set, s2: set) -> bool:\n    return s1.issuperset(s2)");
    assert!(code.contains("is_superset") || code.contains("all"));
}
