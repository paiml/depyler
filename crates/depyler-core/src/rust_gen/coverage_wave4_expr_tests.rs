//! Coverage wave 4: expr_gen submodule coverage boost tests
//!
//! Targets uncovered branches in:
//! - binary_ops.rs (49.8% cov, 407 missed)
//! - call_generic.rs (57.4% cov, 251 missed)
//! - convert_unary_and_call.rs (69.8% cov, 266 missed)
//! - stdlib_data.rs (61.9% cov, 125 missed)
//! - stdlib_misc.rs (52.5% cov, 181 missed)
//! - stdlib_datetime.rs (42.6% cov, 135 missed)
//! - stdlib_numpy.rs (38.1% cov, 166 missed)
//! - stdlib_pathlib.rs (55.4% cov, 37 missed)
//! - stdlib_os.rs (58.1% cov, 54 missed)
//! - stdlib_subprocess.rs (69.2% cov, 24 missed)

use crate::DepylerPipeline;

fn transpile(code: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline
        .transpile(code)
        .expect("transpilation should succeed")
}

fn transpile_ok(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_ok()
}

// =============================================================================
// Section 1: binary_ops.rs - Floor division
// =============================================================================

#[test]
fn test_floor_div_positive_integers() {
    let code = transpile("def floor_div(a: int, b: int) -> int:\n    return a // b");
    assert!(!code.is_empty(), "floor div positive: {}", code);
}

#[test]
fn test_floor_div_negative_dividend() {
    let code = transpile("def neg_floor(x: int) -> int:\n    return -7 // 2");
    assert!(!code.is_empty(), "floor div negative dividend: {}", code);
}

#[test]
fn test_floor_div_negative_divisor() {
    let code = transpile("def neg_div(x: int) -> int:\n    return 7 // -2");
    assert!(!code.is_empty(), "floor div negative divisor: {}", code);
}

#[test]
fn test_floor_div_both_negative() {
    let code = transpile("def both_neg() -> int:\n    return -7 // -3");
    assert!(!code.is_empty(), "floor div both neg: {}", code);
}

// =============================================================================
// Section 2: binary_ops.rs - Set operations
// =============================================================================

#[test]
fn test_set_intersection_operator() {
    let code = transpile("def intersect(a: set, b: set) -> set:\n    return a & b");
    assert!(!code.is_empty(), "set intersection: {}", code);
}

#[test]
fn test_set_union_operator() {
    let code = transpile("def union_op(a: set, b: set) -> set:\n    return a | b");
    assert!(!code.is_empty(), "set union: {}", code);
}

#[test]
fn test_set_difference_operator() {
    let code = transpile("def diff(a: set, b: set) -> set:\n    return a - b");
    assert!(!code.is_empty(), "set difference: {}", code);
}

#[test]
fn test_set_symmetric_diff_operator() {
    let code = transpile("def sym_diff(a: set, b: set) -> set:\n    return a ^ b");
    assert!(!code.is_empty(), "set symmetric diff: {}", code);
}

// =============================================================================
// Section 3: binary_ops.rs - Dict merge operator
// =============================================================================

#[test]
fn test_dict_merge_operator() {
    let code = transpile(
        "def merge(d1: dict, d2: dict) -> dict:\n    return d1 | d2",
    );
    assert!(!code.is_empty(), "dict merge: {}", code);
}

// =============================================================================
// Section 4: binary_ops.rs - Value-returning or/and operators
// =============================================================================

#[test]
fn test_or_with_string_default() {
    let code = transpile(
        "def default_str(name: str) -> str:\n    return name or \"unknown\"",
    );
    assert!(!code.is_empty(), "or string default: {}", code);
}

#[test]
fn test_and_boolean_exprs() {
    let code = transpile(
        "def both_true(a: int, b: int) -> bool:\n    return a > 0 and b > 0",
    );
    assert!(!code.is_empty(), "and boolean: {}", code);
}

#[test]
fn test_or_boolean_exprs() {
    let code = transpile(
        "def either_true(a: int, b: int) -> bool:\n    return a > 0 or b > 0",
    );
    assert!(!code.is_empty(), "or boolean: {}", code);
}

#[test]
fn test_or_numeric_default() {
    let code = transpile(
        "def num_default(x: int) -> int:\n    return x or 42",
    );
    assert!(!code.is_empty(), "or numeric default: {}", code);
}

#[test]
fn test_and_numeric_default() {
    let code = transpile(
        "def check_and(x: int) -> int:\n    return x and 42",
    );
    assert!(!code.is_empty(), "and numeric: {}", code);
}

// =============================================================================
// Section 5: binary_ops.rs - String comparison coercion
// =============================================================================

#[test]
fn test_string_ordering_compare() {
    let code = transpile(
        "def compare_chars(s: str) -> bool:\n    return s[0] >= \"a\"",
    );
    assert!(!code.is_empty(), "string ordering compare: {}", code);
}

#[test]
fn test_string_equality_compare() {
    let code = transpile(
        "def eq_check(s: str, t: str) -> bool:\n    return s == t",
    );
    assert!(!code.is_empty(), "string equality: {}", code);
}

// =============================================================================
// Section 6: binary_ops.rs - Power operator
// =============================================================================

#[test]
fn test_pow_int_positive() {
    let code = transpile("def power(base: int) -> int:\n    return 2 ** 10");
    assert!(!code.is_empty(), "pow int positive: {}", code);
}

#[test]
fn test_pow_int_negative_exp() {
    let code = transpile("def neg_pow() -> float:\n    return 2 ** -3");
    assert!(!code.is_empty(), "pow negative exp: {}", code);
}

#[test]
fn test_pow_float_base() {
    let code = transpile("def float_pow() -> float:\n    return 2.5 ** 3");
    assert!(!code.is_empty(), "pow float base: {}", code);
}

#[test]
fn test_pow_float_exp() {
    let code = transpile("def float_exp() -> float:\n    return 2 ** 0.5");
    assert!(!code.is_empty(), "pow float exp: {}", code);
}

#[test]
fn test_pow_variable_exponents() {
    let code = transpile(
        "def var_pow(base: int, exp: int) -> int:\n    return base ** exp",
    );
    assert!(!code.is_empty(), "pow variables: {}", code);
}

// =============================================================================
// Section 7: binary_ops.rs - Containment operators (in / not in)
// =============================================================================

#[test]
fn test_in_string_containment() {
    let code = transpile(
        "def contains_sub(s: str) -> bool:\n    return \"hello\" in s",
    );
    assert!(!code.is_empty(), "string in: {}", code);
}

#[test]
fn test_not_in_string() {
    let code = transpile(
        "def not_contains(s: str) -> bool:\n    return \"x\" not in s",
    );
    assert!(!code.is_empty(), "string not in: {}", code);
}

#[test]
fn test_in_list_containment() {
    let code = transpile(
        "def in_list(x: int) -> bool:\n    return x in [1, 2, 3]",
    );
    assert!(!code.is_empty(), "list in: {}", code);
}

#[test]
fn test_in_tuple_containment() {
    let code = transpile(
        "def in_tup(x: str) -> bool:\n    return x in (\"a\", \"b\", \"c\")",
    );
    assert!(!code.is_empty(), "tuple in: {}", code);
}

#[test]
fn test_not_in_tuple() {
    let code = transpile(
        "def not_in_tup(x: str) -> bool:\n    return x not in (\"a\", \"b\")",
    );
    assert!(!code.is_empty(), "not in tuple: {}", code);
}

#[test]
fn test_in_dict_containment() {
    let code = transpile(
        "def in_dict(k: str, d: dict) -> bool:\n    return k in d",
    );
    assert!(!code.is_empty(), "dict in: {}", code);
}

#[test]
fn test_not_in_dict() {
    let code = transpile(
        "def not_in_dict(k: str, d: dict) -> bool:\n    return k not in d",
    );
    assert!(!code.is_empty(), "dict not in: {}", code);
}

// =============================================================================
// Section 8: binary_ops.rs - String repetition and list array creation
// =============================================================================

#[test]
fn test_string_repetition() {
    let code = transpile("def repeat_str() -> str:\n    return \"abc\" * 3");
    assert!(!code.is_empty(), "string repeat: {}", code);
}

#[test]
fn test_list_array_creation() {
    let code = transpile("def zeros_list() -> list:\n    return [0] * 5");
    assert!(!code.is_empty(), "list array creation: {}", code);
}

#[test]
fn test_int_times_string() {
    let code = transpile("def int_str() -> str:\n    return 3 * \"ha\"");
    assert!(!code.is_empty(), "int * string: {}", code);
}

// =============================================================================
// Section 9: binary_ops.rs - List concatenation
// =============================================================================

#[test]
fn test_list_concatenation() {
    let code = transpile(
        "def concat(a: list, b: list) -> list:\n    return a + b",
    );
    assert!(!code.is_empty(), "list concat: {}", code);
}

#[test]
fn test_string_concat_literals() {
    let code = transpile(
        "def greet(name: str) -> str:\n    return \"Hello, \" + name",
    );
    assert!(!code.is_empty(), "string concat: {}", code);
}

// =============================================================================
// Section 10: binary_ops.rs - Path division
// =============================================================================

#[test]
fn test_path_division_join() {
    let code = transpile(
        "from pathlib import Path\ndef build_path() -> str:\n    p = Path(\"/dir\")\n    return str(p / \"file.txt\")",
    );
    assert!(!code.is_empty(), "path division: {}", code);
}

// =============================================================================
// Section 11: binary_ops.rs - Float division coercion
// =============================================================================

#[test]
fn test_float_division_returns_float() {
    let code = transpile(
        "def divide(a: int, b: int) -> float:\n    return a / b",
    );
    assert!(!code.is_empty(), "float division: {}", code);
    assert!(code.contains("f64"), "should cast to f64: {}", code);
}

#[test]
fn test_integer_division() {
    let code = transpile(
        "def int_div(a: int, b: int) -> int:\n    return a / b",
    );
    assert!(!code.is_empty(), "integer division: {}", code);
}

// =============================================================================
// Section 12: binary_ops.rs - Modulo operator
// =============================================================================

#[test]
fn test_modulo_operator() {
    let code = transpile("def modulo(a: int, b: int) -> int:\n    return a % b");
    assert!(!code.is_empty(), "modulo: {}", code);
}

// =============================================================================
// Section 13: call_generic.rs - Constructor calls
// =============================================================================

#[test]
fn test_generic_constructor_no_args() {
    let code = transpile(
        "class Counter:\n    def __init__(self):\n        self.count = 0\n\ndef make() -> Counter:\n    return Counter()",
    );
    assert!(!code.is_empty(), "constructor no args: {}", code);
}

#[test]
fn test_generic_constructor_with_args() {
    let code = transpile(
        "class Point:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y\n\ndef make() -> Point:\n    return Point(1, 2)",
    );
    assert!(!code.is_empty(), "constructor with args: {}", code);
}

// =============================================================================
// Section 14: call_generic.rs - isinstance fallback
// =============================================================================

#[test]
fn test_isinstance_returns_true() {
    let code = transpile(
        "def check(x: int) -> bool:\n    return isinstance(x, int)",
    );
    assert!(!code.is_empty(), "isinstance: {}", code);
}

// =============================================================================
// Section 15: call_generic.rs - isqrt fallback
// =============================================================================

#[test]
fn test_isqrt_fallback() {
    let code = transpile("def isq(n: int) -> int:\n    return isqrt(n)");
    assert!(!code.is_empty(), "isqrt fallback: {}", code);
}

// =============================================================================
// Section 16: convert_unary_and_call.rs - not operator on collections
// =============================================================================

#[test]
fn test_not_on_list_var() {
    let code = transpile(
        "def is_empty(items: list) -> bool:\n    return not items",
    );
    assert!(!code.is_empty(), "not on list: {}", code);
    assert!(code.contains("is_empty"), "should use is_empty: {}", code);
}

#[test]
fn test_not_on_string_var() {
    let code = transpile(
        "def blank(s: str) -> bool:\n    return not s",
    );
    assert!(!code.is_empty(), "not on string: {}", code);
    assert!(code.contains("is_empty"), "should use is_empty: {}", code);
}

#[test]
fn test_not_on_dict_var() {
    let code = transpile(
        "def empty_dict(d: dict) -> bool:\n    return not d",
    );
    assert!(!code.is_empty(), "not on dict: {}", code);
    assert!(code.contains("is_empty"), "should use is_empty: {}", code);
}

// =============================================================================
// Section 17: convert_unary_and_call.rs - Counter constructor
// =============================================================================

#[test]
fn test_counter_from_string() {
    let code = transpile(
        "from collections import Counter\ndef count_chars(s: str) -> dict:\n    return Counter(s)",
    );
    assert!(!code.is_empty(), "Counter(string): {}", code);
}

#[test]
fn test_counter_empty() {
    let code = transpile(
        "from collections import Counter\ndef empty_counter() -> dict:\n    return Counter()",
    );
    assert!(!code.is_empty(), "Counter(): {}", code);
}

// =============================================================================
// Section 18: convert_unary_and_call.rs - list() constructor
// =============================================================================

#[test]
fn test_list_from_string() {
    let code = transpile(
        "def chars(s: str) -> list:\n    return list(s)",
    );
    assert!(!code.is_empty(), "list(string): {}", code);
    assert!(code.contains("chars"), "should use .chars(): {}", code);
}

#[test]
fn test_list_empty() {
    let code = transpile("def empty() -> list:\n    return list()");
    assert!(!code.is_empty(), "list(): {}", code);
}

// =============================================================================
// Section 19: convert_unary_and_call.rs - bytes() and bytearray()
// =============================================================================

#[test]
fn test_bytes_empty() {
    let code = transpile("def empty_bytes() -> bytes:\n    return bytes()");
    assert!(!code.is_empty(), "bytes(): {}", code);
}

#[test]
fn test_bytes_from_int() {
    let code = transpile("def zero_bytes() -> bytes:\n    return bytes(10)");
    assert!(!code.is_empty(), "bytes(n): {}", code);
}

#[test]
fn test_bytes_from_list() {
    let code = transpile(
        "def make_bytes() -> bytes:\n    return bytes([72, 101, 108])",
    );
    assert!(!code.is_empty(), "bytes(list): {}", code);
}

#[test]
fn test_bytes_from_string() {
    let code = transpile(
        "def str_bytes(s: str) -> bytes:\n    return bytes(s, \"utf-8\")",
    );
    assert!(!code.is_empty(), "bytes(str, enc): {}", code);
}

#[test]
fn test_bytearray_empty() {
    let code = transpile("def empty_ba() -> bytes:\n    return bytearray()");
    assert!(!code.is_empty(), "bytearray(): {}", code);
}

#[test]
fn test_bytearray_from_int() {
    let code = transpile("def zero_ba() -> bytes:\n    return bytearray(10)");
    assert!(!code.is_empty(), "bytearray(n): {}", code);
}

#[test]
fn test_bytearray_from_list() {
    let code = transpile(
        "def make_ba() -> bytes:\n    return bytearray([1, 2, 3])",
    );
    assert!(!code.is_empty(), "bytearray(list): {}", code);
}

// =============================================================================
// Section 20: convert_unary_and_call.rs - map() with various patterns
// =============================================================================

#[test]
fn test_map_with_str() {
    let code = transpile(
        "def stringify(nums: list) -> list:\n    return list(map(str, nums))",
    );
    assert!(!code.is_empty(), "map(str, ...): {}", code);
}

#[test]
fn test_map_with_int() {
    let code = transpile(
        "def parse_ints(strs: list) -> list:\n    return list(map(int, strs))",
    );
    assert!(!code.is_empty(), "map(int, ...): {}", code);
}

#[test]
fn test_map_with_lambda() {
    let code = transpile(
        "def double(nums: list) -> list:\n    return list(map(lambda x: x * 2, nums))",
    );
    assert!(!code.is_empty(), "map(lambda, ...): {}", code);
}

// =============================================================================
// Section 21: convert_unary_and_call.rs - filter() with lambda
// =============================================================================

#[test]
fn test_filter_with_lambda() {
    let code = transpile(
        "def positives(nums: list) -> list:\n    return list(filter(lambda x: x > 0, nums))",
    );
    assert!(!code.is_empty(), "filter(lambda, ...): {}", code);
}

// =============================================================================
// Section 22: convert_unary_and_call.rs - bool() conversions
// =============================================================================

#[test]
fn test_bool_of_string_literal() {
    let code = transpile("def truthy() -> bool:\n    return bool(\"hello\")");
    assert!(!code.is_empty(), "bool(string): {}", code);
}

#[test]
fn test_bool_of_zero() {
    let code = transpile("def falsy() -> bool:\n    return bool(0)");
    assert!(!code.is_empty(), "bool(0): {}", code);
}

#[test]
fn test_bool_of_float_literal() {
    let code = transpile("def truthy_f() -> bool:\n    return bool(3.14)");
    assert!(!code.is_empty(), "bool(float): {}", code);
}

#[test]
fn test_bool_of_bool_literal() {
    let code = transpile("def identity() -> bool:\n    return bool(True)");
    assert!(!code.is_empty(), "bool(True): {}", code);
}

// =============================================================================
// Section 23: convert_unary_and_call.rs - tuple() builtin
// =============================================================================

#[test]
fn test_tuple_empty() {
    let code = transpile("def empty_tup():\n    return tuple()");
    assert!(!code.is_empty(), "tuple(): {}", code);
}

#[test]
fn test_tuple_from_string() {
    let code = transpile(
        "def chars(s: str) -> tuple:\n    return tuple(s)",
    );
    assert!(!code.is_empty(), "tuple(string): {}", code);
}

// =============================================================================
// Section 24: stdlib_data.rs - calendar module
// =============================================================================

#[test]
fn test_calendar_isleap() {
    let code = transpile(
        "import calendar\ndef check_leap(year: int) -> bool:\n    return calendar.isleap(year)",
    );
    assert!(!code.is_empty(), "calendar.isleap: {}", code);
}

#[test]
fn test_calendar_weekday() {
    let code = transpile(
        "import calendar\ndef get_day(y: int, m: int, d: int) -> int:\n    return calendar.weekday(y, m, d)",
    );
    assert!(!code.is_empty(), "calendar.weekday: {}", code);
}

#[test]
fn test_calendar_monthrange() {
    let code = transpile(
        "import calendar\ndef month_info(y: int, m: int):\n    return calendar.monthrange(y, m)",
    );
    assert!(!code.is_empty(), "calendar.monthrange: {}", code);
}

#[test]
fn test_calendar_leapdays() {
    let code = transpile(
        "import calendar\ndef count_leaps(y1: int, y2: int) -> int:\n    return calendar.leapdays(y1, y2)",
    );
    assert!(!code.is_empty(), "calendar.leapdays: {}", code);
}

// =============================================================================
// Section 25: stdlib_data.rs - binascii module
// =============================================================================

#[test]
fn test_binascii_hexlify() {
    let code = transpile(
        "import binascii\ndef to_hex(data: bytes) -> bytes:\n    return binascii.hexlify(data)",
    );
    assert!(!code.is_empty(), "binascii.hexlify: {}", code);
}

#[test]
fn test_binascii_unhexlify() {
    let code = transpile(
        "import binascii\ndef from_hex(data: bytes) -> bytes:\n    return binascii.unhexlify(data)",
    );
    assert!(!code.is_empty(), "binascii.unhexlify: {}", code);
}

// =============================================================================
// Section 26: stdlib_data.rs - urllib.parse module
// =============================================================================

#[test]
fn test_urllib_quote() {
    let code = transpile(
        "from urllib import parse\ndef encode_url(text: str) -> str:\n    return parse.quote(text)",
    );
    assert!(!code.is_empty(), "urllib.parse.quote: {}", code);
}

#[test]
fn test_urllib_unquote() {
    let code = transpile(
        "from urllib import parse\ndef decode_url(text: str) -> str:\n    return parse.unquote(text)",
    );
    assert!(!code.is_empty(), "urllib.parse.unquote: {}", code);
}

#[test]
fn test_urllib_parse_qs() {
    let code = transpile(
        "from urllib import parse\ndef parse_query(qs: str) -> dict:\n    return parse.parse_qs(qs)",
    );
    assert!(!code.is_empty(), "urllib.parse.parse_qs: {}", code);
}

// =============================================================================
// Section 27: stdlib_data.rs - fnmatch module
// =============================================================================

#[test]
fn test_fnmatch_fnmatch() {
    let code = transpile(
        "import fnmatch\ndef match_pattern(name: str, pattern: str) -> bool:\n    return fnmatch.fnmatch(name, pattern)",
    );
    assert!(!code.is_empty(), "fnmatch.fnmatch: {}", code);
}

#[test]
fn test_fnmatch_translate() {
    let code = transpile(
        "import fnmatch\ndef to_regex(pattern: str) -> str:\n    return fnmatch.translate(pattern)",
    );
    assert!(!code.is_empty(), "fnmatch.translate: {}", code);
}

// =============================================================================
// Section 28: stdlib_data.rs - shlex module
// =============================================================================

#[test]
fn test_shlex_quote() {
    let code = transpile(
        "import shlex\ndef safe_quote(s: str) -> str:\n    return shlex.quote(s)",
    );
    assert!(!code.is_empty(), "shlex.quote: {}", code);
}

#[test]
fn test_shlex_split() {
    let code = transpile(
        "import shlex\ndef split_cmd(s: str) -> list:\n    return shlex.split(s)",
    );
    assert!(!code.is_empty(), "shlex.split: {}", code);
}

#[test]
fn test_shlex_join() {
    let code = transpile(
        "import shlex\ndef join_args(args: list) -> str:\n    return shlex.join(args)",
    );
    assert!(!code.is_empty(), "shlex.join: {}", code);
}

// =============================================================================
// Section 29: stdlib_data.rs - textwrap module
// =============================================================================

#[test]
fn test_textwrap_wrap() {
    let code = transpile(
        "import textwrap\ndef wrap_text(text: str, width: int) -> list:\n    return textwrap.wrap(text, width)",
    );
    assert!(!code.is_empty(), "textwrap.wrap: {}", code);
}

#[test]
fn test_textwrap_fill() {
    let code = transpile(
        "import textwrap\ndef fill_text(text: str, width: int) -> str:\n    return textwrap.fill(text, width)",
    );
    assert!(!code.is_empty(), "textwrap.fill: {}", code);
}

#[test]
fn test_textwrap_dedent() {
    let code = transpile(
        "import textwrap\ndef clean(text: str) -> str:\n    return textwrap.dedent(text)",
    );
    assert!(!code.is_empty(), "textwrap.dedent: {}", code);
}

#[test]
fn test_textwrap_indent() {
    let code = transpile(
        "import textwrap\ndef add_prefix(text: str, prefix: str) -> str:\n    return textwrap.indent(text, prefix)",
    );
    assert!(!code.is_empty(), "textwrap.indent: {}", code);
}

#[test]
fn test_textwrap_shorten() {
    let code = transpile(
        "import textwrap\ndef short(text: str, width: int) -> str:\n    return textwrap.shorten(text, width)",
    );
    assert!(!code.is_empty(), "textwrap.shorten: {}", code);
}

// =============================================================================
// Section 30: stdlib_misc.rs - bisect module
// =============================================================================

#[test]
fn test_bisect_bisect_left() {
    let code = transpile(
        "import bisect\ndef find_pos(arr: list, val: int) -> int:\n    return bisect.bisect_left(arr, val)",
    );
    assert!(!code.is_empty(), "bisect.bisect_left: {}", code);
}

#[test]
fn test_bisect_bisect_right() {
    let code = transpile(
        "import bisect\ndef find_pos_right(arr: list, val: int) -> int:\n    return bisect.bisect_right(arr, val)",
    );
    assert!(!code.is_empty(), "bisect.bisect_right: {}", code);
}

#[test]
fn test_bisect_insort_left() {
    let code = transpile(
        "import bisect\ndef insert_sorted(arr: list, val: int):\n    bisect.insort_left(arr, val)",
    );
    assert!(!code.is_empty(), "bisect.insort_left: {}", code);
}

#[test]
fn test_bisect_insort_right() {
    let code = transpile(
        "import bisect\ndef insert_sorted_right(arr: list, val: int):\n    bisect.insort_right(arr, val)",
    );
    assert!(!code.is_empty(), "bisect.insort_right: {}", code);
}

// =============================================================================
// Section 31: stdlib_misc.rs - heapq module
// =============================================================================

#[test]
fn test_heapq_heappush() {
    let code = transpile(
        "import heapq\ndef push(heap: list, val: int):\n    heapq.heappush(heap, val)",
    );
    assert!(!code.is_empty(), "heapq.heappush: {}", code);
}

#[test]
fn test_heapq_heappop() {
    let code = transpile(
        "import heapq\ndef pop(heap: list) -> int:\n    return heapq.heappop(heap)",
    );
    assert!(!code.is_empty(), "heapq.heappop: {}", code);
}

#[test]
fn test_heapq_heapify() {
    let code = transpile(
        "import heapq\ndef make_heap(arr: list):\n    heapq.heapify(arr)",
    );
    assert!(!code.is_empty(), "heapq.heapify: {}", code);
}

#[test]
fn test_heapq_nlargest() {
    let code = transpile(
        "import heapq\ndef top_n(n: int, arr: list) -> list:\n    return heapq.nlargest(n, arr)",
    );
    assert!(!code.is_empty(), "heapq.nlargest: {}", code);
}

#[test]
fn test_heapq_nsmallest() {
    let code = transpile(
        "import heapq\ndef bot_n(n: int, arr: list) -> list:\n    return heapq.nsmallest(n, arr)",
    );
    assert!(!code.is_empty(), "heapq.nsmallest: {}", code);
}

// =============================================================================
// Section 32: stdlib_misc.rs - copy module
// =============================================================================

#[test]
fn test_copy_deepcopy() {
    let code = transpile(
        "import copy\ndef dup(data: list) -> list:\n    return copy.deepcopy(data)",
    );
    assert!(!code.is_empty(), "copy.deepcopy: {}", code);
    assert!(code.contains("clone"), "should use clone: {}", code);
}

#[test]
fn test_copy_copy() {
    let code = transpile(
        "import copy\ndef shallow(data: list) -> list:\n    return copy.copy(data)",
    );
    assert!(!code.is_empty(), "copy.copy: {}", code);
    assert!(code.contains("clone"), "should use clone: {}", code);
}

// =============================================================================
// Section 33: stdlib_misc.rs - statistics module
// =============================================================================

#[test]
fn test_statistics_mean() {
    let code = transpile(
        "import statistics\ndef avg(data: list) -> float:\n    return statistics.mean(data)",
    );
    assert!(!code.is_empty(), "statistics.mean: {}", code);
}

#[test]
fn test_statistics_median() {
    let code = transpile(
        "import statistics\ndef mid(data: list) -> float:\n    return statistics.median(data)",
    );
    assert!(!code.is_empty(), "statistics.median: {}", code);
}

#[test]
fn test_statistics_mode() {
    let code = transpile(
        "import statistics\ndef common(data: list) -> int:\n    return statistics.mode(data)",
    );
    assert!(!code.is_empty(), "statistics.mode: {}", code);
}

#[test]
fn test_statistics_variance() {
    let code = transpile(
        "import statistics\ndef var(data: list) -> float:\n    return statistics.variance(data)",
    );
    assert!(!code.is_empty(), "statistics.variance: {}", code);
}

#[test]
fn test_statistics_stdev() {
    let code = transpile(
        "import statistics\ndef sd(data: list) -> float:\n    return statistics.stdev(data)",
    );
    assert!(!code.is_empty(), "statistics.stdev: {}", code);
}

#[test]
fn test_statistics_pvariance() {
    let code = transpile(
        "import statistics\ndef pvar(data: list) -> float:\n    return statistics.pvariance(data)",
    );
    assert!(!code.is_empty(), "statistics.pvariance: {}", code);
}

#[test]
fn test_statistics_pstdev() {
    let code = transpile(
        "import statistics\ndef psd(data: list) -> float:\n    return statistics.pstdev(data)",
    );
    assert!(!code.is_empty(), "statistics.pstdev: {}", code);
}

#[test]
fn test_statistics_harmonic_mean() {
    let code = transpile(
        "import statistics\ndef hmean(data: list) -> float:\n    return statistics.harmonic_mean(data)",
    );
    assert!(!code.is_empty(), "statistics.harmonic_mean: {}", code);
}

#[test]
fn test_statistics_geometric_mean() {
    let code = transpile(
        "import statistics\ndef gmean(data: list) -> float:\n    return statistics.geometric_mean(data)",
    );
    assert!(!code.is_empty(), "statistics.geometric_mean: {}", code);
}

// =============================================================================
// Section 34: stdlib_misc.rs - fractions module
// =============================================================================

#[test]
fn test_fractions_constructor() {
    let code = transpile(
        "from fractions import Fraction\ndef make_frac() -> float:\n    f = Fraction(1, 3)\n    return float(f)",
    );
    assert!(!code.is_empty(), "Fraction(): {}", code);
}

// =============================================================================
// Section 35: stdlib_misc.rs - sys module
// =============================================================================

#[test]
fn test_sys_exit() {
    let code = transpile(
        "import sys\ndef quit_now():\n    sys.exit(1)",
    );
    assert!(!code.is_empty(), "sys.exit: {}", code);
}

// =============================================================================
// Section 36: stdlib_misc.rs - pickle module
// =============================================================================

#[test]
fn test_pickle_dumps() {
    let code = transpile(
        "import pickle\ndef serialize(data: str) -> bytes:\n    return pickle.dumps(data)",
    );
    assert!(!code.is_empty(), "pickle.dumps: {}", code);
}

#[test]
fn test_pickle_loads() {
    let code = transpile(
        "import pickle\ndef deserialize(data: bytes) -> str:\n    return pickle.loads(data)",
    );
    assert!(!code.is_empty(), "pickle.loads: {}", code);
}

// =============================================================================
// Section 37: stdlib_misc.rs - pprint module
// =============================================================================

#[test]
fn test_pprint_pprint() {
    let code = transpile(
        "import pprint\ndef show(data: dict):\n    pprint.pprint(data)",
    );
    assert!(!code.is_empty(), "pprint.pprint: {}", code);
}

// =============================================================================
// Section 38: stdlib_datetime.rs - datetime.now(), utcnow(), today()
// =============================================================================

#[test]
fn test_datetime_now() {
    let code = transpile(
        "from datetime import datetime\ndef current():\n    return datetime.now()",
    );
    assert!(!code.is_empty(), "datetime.now: {}", code);
}

#[test]
fn test_datetime_utcnow() {
    let code = transpile(
        "from datetime import datetime\ndef utc_current():\n    return datetime.utcnow()",
    );
    assert!(!code.is_empty(), "datetime.utcnow: {}", code);
}

#[test]
fn test_datetime_today() {
    let code = transpile(
        "from datetime import datetime\ndef today():\n    return datetime.today()",
    );
    assert!(!code.is_empty(), "datetime.today: {}", code);
}

// =============================================================================
// Section 39: stdlib_datetime.rs - strptime, fromtimestamp
// =============================================================================

#[test]
fn test_datetime_strptime() {
    let code = transpile(
        "from datetime import datetime\ndef parse_date(s: str):\n    return datetime.strptime(s, \"%Y-%m-%d\")",
    );
    assert!(!code.is_empty(), "datetime.strptime: {}", code);
}

#[test]
fn test_datetime_fromtimestamp() {
    let code = transpile(
        "from datetime import datetime\ndef from_ts(ts: float):\n    return datetime.fromtimestamp(ts)",
    );
    assert!(!code.is_empty(), "datetime.fromtimestamp: {}", code);
}

#[test]
fn test_datetime_fromisoformat() {
    let code = transpile(
        "from datetime import datetime\ndef from_iso(s: str):\n    return datetime.fromisoformat(s)",
    );
    assert!(!code.is_empty(), "datetime.fromisoformat: {}", code);
}

// =============================================================================
// Section 40: stdlib_datetime.rs - datetime instance methods
// =============================================================================

#[test]
fn test_datetime_strftime() {
    let code = transpile(
        "from datetime import datetime\ndef format_date():\n    dt = datetime.now()\n    return dt.strftime(\"%Y-%m-%d\")",
    );
    assert!(!code.is_empty(), "datetime.strftime: {}", code);
}

#[test]
fn test_datetime_isoformat() {
    let code = transpile(
        "from datetime import datetime\ndef iso():\n    dt = datetime.now()\n    return dt.isoformat()",
    );
    assert!(!code.is_empty(), "datetime.isoformat: {}", code);
}

#[test]
fn test_datetime_weekday() {
    let code = transpile(
        "from datetime import datetime\ndef dow():\n    dt = datetime.now()\n    return dt.weekday()",
    );
    assert!(!code.is_empty(), "datetime.weekday: {}", code);
}

// =============================================================================
// Section 41: stdlib_numpy.rs - np.array()
// =============================================================================

#[test]
fn test_np_array_literal() {
    let code = transpile(
        "import numpy as np\ndef make_arr():\n    return np.array([1.0, 2.0, 3.0])",
    );
    assert!(!code.is_empty(), "np.array: {}", code);
}

#[test]
fn test_np_array_empty() {
    let code = transpile(
        "import numpy as np\ndef empty_arr():\n    return np.array([])",
    );
    assert!(!code.is_empty(), "np.array([]): {}", code);
}

// =============================================================================
// Section 42: stdlib_numpy.rs - np.zeros(), np.ones()
// =============================================================================

#[test]
fn test_np_zeros() {
    let code = transpile(
        "import numpy as np\ndef zero_arr(n: int):\n    return np.zeros(n)",
    );
    assert!(!code.is_empty(), "np.zeros: {}", code);
}

#[test]
fn test_np_ones() {
    let code = transpile(
        "import numpy as np\ndef one_arr(n: int):\n    return np.ones(n)",
    );
    assert!(!code.is_empty(), "np.ones: {}", code);
}

// =============================================================================
// Section 43: stdlib_numpy.rs - np.dot()
// =============================================================================

#[test]
fn test_np_dot() {
    let code = transpile(
        "import numpy as np\ndef dot_product(a: list, b: list) -> float:\n    return np.dot(a, b)",
    );
    assert!(!code.is_empty(), "np.dot: {}", code);
}

// =============================================================================
// Section 44: stdlib_numpy.rs - np.sum(), np.mean()
// =============================================================================

#[test]
fn test_np_sum() {
    let code = transpile(
        "import numpy as np\ndef total(a: list) -> float:\n    return np.sum(a)",
    );
    assert!(!code.is_empty(), "np.sum: {}", code);
}

#[test]
fn test_np_mean() {
    let code = transpile(
        "import numpy as np\ndef average(a: list) -> float:\n    return np.mean(a)",
    );
    assert!(!code.is_empty(), "np.mean: {}", code);
}

// =============================================================================
// Section 45: stdlib_numpy.rs - np.sqrt(), np.abs()
// =============================================================================

#[test]
fn test_np_sqrt_scalar() {
    let code = transpile(
        "import numpy as np\ndef root(x: float) -> float:\n    return np.sqrt(x)",
    );
    assert!(!code.is_empty(), "np.sqrt scalar: {}", code);
}

#[test]
fn test_np_abs_scalar() {
    let code = transpile(
        "import numpy as np\ndef absolute(x: float) -> float:\n    return np.abs(x)",
    );
    assert!(!code.is_empty(), "np.abs scalar: {}", code);
}

// =============================================================================
// Section 46: stdlib_numpy.rs - np.exp(), np.log(), np.sin(), np.cos()
// =============================================================================

#[test]
fn test_np_exp_scalar() {
    let code = transpile(
        "import numpy as np\ndef e_x(x: float) -> float:\n    return np.exp(x)",
    );
    assert!(!code.is_empty(), "np.exp: {}", code);
}

#[test]
fn test_np_log_scalar() {
    let code = transpile(
        "import numpy as np\ndef ln_x(x: float) -> float:\n    return np.log(x)",
    );
    assert!(!code.is_empty(), "np.log: {}", code);
}

#[test]
fn test_np_sin_scalar() {
    let code = transpile(
        "import numpy as np\ndef sine(x: float) -> float:\n    return np.sin(x)",
    );
    assert!(!code.is_empty(), "np.sin: {}", code);
}

#[test]
fn test_np_cos_scalar() {
    let code = transpile(
        "import numpy as np\ndef cosine(x: float) -> float:\n    return np.cos(x)",
    );
    assert!(!code.is_empty(), "np.cos: {}", code);
}

// =============================================================================
// Section 47: stdlib_numpy.rs - np.min(), np.max(), np.std(), np.var()
// =============================================================================

#[test]
fn test_np_min() {
    let code = transpile(
        "import numpy as np\ndef smallest(a: list) -> float:\n    return np.min(a)",
    );
    assert!(!code.is_empty(), "np.min: {}", code);
}

#[test]
fn test_np_max() {
    let code = transpile(
        "import numpy as np\ndef largest(a: list) -> float:\n    return np.max(a)",
    );
    assert!(!code.is_empty(), "np.max: {}", code);
}

#[test]
fn test_np_std() {
    let code = transpile(
        "import numpy as np\ndef stddev(a: list) -> float:\n    return np.std(a)",
    );
    assert!(!code.is_empty(), "np.std: {}", code);
}

#[test]
fn test_np_var() {
    let code = transpile(
        "import numpy as np\ndef variance(a: list) -> float:\n    return np.var(a)",
    );
    assert!(!code.is_empty(), "np.var: {}", code);
}

// =============================================================================
// Section 48: stdlib_numpy.rs - np.argmax(), np.argmin()
// =============================================================================

#[test]
fn test_np_argmax() {
    let code = transpile(
        "import numpy as np\ndef max_idx(a: list) -> int:\n    return np.argmax(a)",
    );
    assert!(!code.is_empty(), "np.argmax: {}", code);
}

#[test]
fn test_np_argmin() {
    let code = transpile(
        "import numpy as np\ndef min_idx(a: list) -> int:\n    return np.argmin(a)",
    );
    assert!(!code.is_empty(), "np.argmin: {}", code);
}

// =============================================================================
// Section 49: stdlib_numpy.rs - np.norm()
// =============================================================================

#[test]
fn test_np_norm() {
    let code = transpile(
        "import numpy as np\ndef l2(a: list) -> float:\n    return np.norm(a)",
    );
    assert!(!code.is_empty(), "np.norm: {}", code);
}

// =============================================================================
// Section 50: stdlib_pathlib.rs - pathlib instance methods
// =============================================================================

#[test]
fn test_pathlib_exists() {
    let code = transpile(
        "from pathlib import Path\ndef check(p: str) -> bool:\n    return Path(p).exists()",
    );
    assert!(!code.is_empty(), "Path.exists: {}", code);
}

#[test]
fn test_pathlib_is_file() {
    let code = transpile(
        "from pathlib import Path\ndef check_file(p: str) -> bool:\n    return Path(p).is_file()",
    );
    assert!(!code.is_empty(), "Path.is_file: {}", code);
}

#[test]
fn test_pathlib_is_dir() {
    let code = transpile(
        "from pathlib import Path\ndef check_dir(p: str) -> bool:\n    return Path(p).is_dir()",
    );
    assert!(!code.is_empty(), "Path.is_dir: {}", code);
}

#[test]
fn test_pathlib_read_text() {
    let code = transpile(
        "from pathlib import Path\ndef read(p: str) -> str:\n    return Path(p).read_text()",
    );
    assert!(!code.is_empty(), "Path.read_text: {}", code);
}

#[test]
fn test_pathlib_mkdir() {
    let code = transpile(
        "from pathlib import Path\ndef make_dir(p: str):\n    Path(p).mkdir()",
    );
    assert!(!code.is_empty(), "Path.mkdir: {}", code);
}

// =============================================================================
// Section 51: stdlib_os.rs - os.path methods
// =============================================================================

#[test]
fn test_os_path_join() {
    let code = transpile(
        "import os\ndef join_paths(a: str, b: str) -> str:\n    return os.path.join(a, b)",
    );
    assert!(!code.is_empty(), "os.path.join: {}", code);
}

#[test]
fn test_os_path_basename() {
    let code = transpile(
        "import os\ndef base(p: str) -> str:\n    return os.path.basename(p)",
    );
    assert!(!code.is_empty(), "os.path.basename: {}", code);
}

#[test]
fn test_os_path_dirname() {
    let code = transpile(
        "import os\ndef dir_name(p: str) -> str:\n    return os.path.dirname(p)",
    );
    assert!(!code.is_empty(), "os.path.dirname: {}", code);
}

#[test]
fn test_os_path_exists() {
    let code = transpile(
        "import os\ndef file_exists(p: str) -> bool:\n    return os.path.exists(p)",
    );
    assert!(!code.is_empty(), "os.path.exists: {}", code);
}

#[test]
fn test_os_path_isfile() {
    let code = transpile(
        "import os\ndef is_file(p: str) -> bool:\n    return os.path.isfile(p)",
    );
    assert!(!code.is_empty(), "os.path.isfile: {}", code);
}

#[test]
fn test_os_path_isdir() {
    let code = transpile(
        "import os\ndef is_dir(p: str) -> bool:\n    return os.path.isdir(p)",
    );
    assert!(!code.is_empty(), "os.path.isdir: {}", code);
}

#[test]
fn test_os_path_splitext() {
    let code = transpile(
        "import os\ndef split_ext(p: str):\n    return os.path.splitext(p)",
    );
    assert!(!code.is_empty(), "os.path.splitext: {}", code);
}

#[test]
fn test_os_path_isabs() {
    let code = transpile(
        "import os\ndef check_abs(p: str) -> bool:\n    return os.path.isabs(p)",
    );
    assert!(!code.is_empty(), "os.path.isabs: {}", code);
}

#[test]
fn test_os_path_abspath() {
    let code = transpile(
        "import os\ndef abs_path(p: str) -> str:\n    return os.path.abspath(p)",
    );
    assert!(!code.is_empty(), "os.path.abspath: {}", code);
}

#[test]
fn test_os_path_split() {
    let code = transpile(
        "import os\ndef split_path(p: str):\n    return os.path.split(p)",
    );
    assert!(!code.is_empty(), "os.path.split: {}", code);
}

#[test]
fn test_os_path_getsize() {
    let code = transpile(
        "import os\ndef fsize(p: str) -> int:\n    return os.path.getsize(p)",
    );
    assert!(!code.is_empty(), "os.path.getsize: {}", code);
}

#[test]
fn test_os_path_expanduser() {
    let code = transpile(
        "import os\ndef expand(p: str) -> str:\n    return os.path.expanduser(p)",
    );
    assert!(!code.is_empty(), "os.path.expanduser: {}", code);
}

#[test]
fn test_os_path_normpath() {
    let code = transpile(
        "import os\ndef normalize(p: str) -> str:\n    return os.path.normpath(p)",
    );
    assert!(!code.is_empty(), "os.path.normpath: {}", code);
}

// =============================================================================
// Section 52: stdlib_subprocess.rs - subprocess.run
// =============================================================================

#[test]
fn test_subprocess_run_basic() {
    let code = transpile(
        "import subprocess\ndef run_cmd(cmd: list):\n    return subprocess.run(cmd)",
    );
    assert!(!code.is_empty(), "subprocess.run basic: {}", code);
}

#[test]
fn test_subprocess_run_capture() {
    let code = transpile(
        "import subprocess\ndef run_capture(cmd: list):\n    return subprocess.run(cmd, capture_output=True)",
    );
    assert!(!code.is_empty(), "subprocess.run capture: {}", code);
}

// =============================================================================
// Section 53: Additional binary_ops.rs coverage - Arithmetic coercion
// =============================================================================

#[test]
fn test_add_int_and_float() {
    let code = transpile(
        "def mixed(a: int, b: float) -> float:\n    return a + b",
    );
    assert!(!code.is_empty(), "int+float: {}", code);
}

#[test]
fn test_subtract_int_from_float() {
    let code = transpile(
        "def sub_mixed(a: float, b: int) -> float:\n    return a - b",
    );
    assert!(!code.is_empty(), "float-int: {}", code);
}

#[test]
fn test_multiply_int_and_float() {
    let code = transpile(
        "def mul_mixed(a: int, b: float) -> float:\n    return a * b",
    );
    assert!(!code.is_empty(), "int*float: {}", code);
}

// =============================================================================
// Section 54: Additional binary_ops.rs - Comparison operators
// =============================================================================

#[test]
fn test_less_than() {
    let code = transpile(
        "def is_less(a: int, b: int) -> bool:\n    return a < b",
    );
    assert!(!code.is_empty(), "less than: {}", code);
}

#[test]
fn test_greater_equal() {
    let code = transpile(
        "def is_gte(a: int, b: int) -> bool:\n    return a >= b",
    );
    assert!(!code.is_empty(), "greater equal: {}", code);
}

#[test]
fn test_not_equal() {
    let code = transpile(
        "def neq(a: int, b: int) -> bool:\n    return a != b",
    );
    assert!(!code.is_empty(), "not equal: {}", code);
}

#[test]
fn test_float_int_comparison() {
    let code = transpile(
        "def compare_fi(x: float, n: int) -> bool:\n    return x > n",
    );
    assert!(!code.is_empty(), "float>int compare: {}", code);
}

// =============================================================================
// Section 55: Additional convert_unary_and_call.rs - Unary operators
// =============================================================================

#[test]
fn test_unary_negation() {
    let code = transpile("def negate(x: int) -> int:\n    return -x");
    assert!(!code.is_empty(), "unary neg: {}", code);
}

#[test]
fn test_unary_positive() {
    let code = transpile("def pos(x: int) -> int:\n    return +x");
    assert!(!code.is_empty(), "unary pos: {}", code);
}

#[test]
fn test_unary_bitnot() {
    let code = transpile("def bitnot(x: int) -> int:\n    return ~x");
    assert!(!code.is_empty(), "unary bitnot: {}", code);
}

#[test]
fn test_not_bool() {
    let code = transpile("def flip(x: bool) -> bool:\n    return not x");
    assert!(!code.is_empty(), "not bool: {}", code);
}

// =============================================================================
// Section 56: Additional call patterns - format(), open(), divmod()
// =============================================================================

#[test]
fn test_format_binary() {
    let code = transpile(
        "def to_bin(n: int) -> str:\n    return format(n, \"b\")",
    );
    assert!(!code.is_empty(), "format binary: {}", code);
}

#[test]
fn test_format_hex() {
    let code = transpile(
        "def to_hex(n: int) -> str:\n    return format(n, \"x\")",
    );
    assert!(!code.is_empty(), "format hex: {}", code);
}

#[test]
fn test_format_octal() {
    let code = transpile(
        "def to_oct(n: int) -> str:\n    return format(n, \"o\")",
    );
    assert!(!code.is_empty(), "format octal: {}", code);
}

#[test]
fn test_format_decimal() {
    let code = transpile(
        "def to_dec(n: int) -> str:\n    return format(n, \"d\")",
    );
    assert!(!code.is_empty(), "format decimal: {}", code);
}

#[test]
fn test_format_upper_hex() {
    let code = transpile(
        "def to_hex_upper(n: int) -> str:\n    return format(n, \"X\")",
    );
    assert!(!code.is_empty(), "format upper hex: {}", code);
}

#[test]
fn test_divmod_call() {
    let code = transpile(
        "def div_mod(a: int, b: int):\n    return divmod(a, b)",
    );
    assert!(!code.is_empty(), "divmod: {}", code);
}

// =============================================================================
// Section 57: Additional call patterns - pow(), abs(), round()
// =============================================================================

#[test]
fn test_pow_builtin() {
    let code = transpile(
        "def power(base: int, exp: int) -> int:\n    return pow(base, exp)",
    );
    assert!(!code.is_empty(), "pow builtin: {}", code);
}

#[test]
fn test_abs_builtin() {
    let code = transpile("def absolute(x: int) -> int:\n    return abs(x)");
    assert!(!code.is_empty(), "abs builtin: {}", code);
}

#[test]
fn test_round_builtin() {
    let code = transpile(
        "def rounded(x: float) -> int:\n    return round(x)",
    );
    assert!(!code.is_empty(), "round builtin: {}", code);
}

// =============================================================================
// Section 58: Additional map/filter patterns
// =============================================================================

#[test]
fn test_map_with_float() {
    let code = transpile(
        "def to_floats(strs: list) -> list:\n    return list(map(float, strs))",
    );
    assert!(!code.is_empty(), "map(float, ...): {}", code);
}

#[test]
fn test_map_two_iterables() {
    let code = transpile(
        "def add_pairs(a: list, b: list) -> list:\n    return list(map(lambda x, y: x + y, a, b))",
    );
    assert!(!code.is_empty(), "map with two iterables: {}", code);
}

// =============================================================================
// Section 59: Additional collection constructors
// =============================================================================

#[test]
fn test_set_constructor_empty() {
    let code = transpile("def empty_set() -> set:\n    return set()");
    assert!(!code.is_empty(), "set(): {}", code);
}

#[test]
fn test_dict_constructor_empty() {
    let code = transpile("def empty_dict() -> dict:\n    return dict()");
    assert!(!code.is_empty(), "dict(): {}", code);
}

#[test]
fn test_frozenset_constructor() {
    let code = transpile("def frozen() -> set:\n    return frozenset([1, 2, 3])");
    assert!(!code.is_empty(), "frozenset: {}", code);
}

// =============================================================================
// Section 60: Additional stdlib datetime coverage
// =============================================================================

#[test]
fn test_datetime_combine() {
    let code = transpile(
        "from datetime import datetime, date, time\ndef combine_dt():\n    d = date(2024, 1, 1)\n    t = time(12, 0)\n    return datetime.combine(d, t)",
    );
    assert!(transpile_ok(
        "from datetime import datetime, date, time\ndef combine_dt():\n    d = date(2024, 1, 1)\n    t = time(12, 0)\n    return datetime.combine(d, t)",
    ), "datetime.combine should transpile");
}

#[test]
fn test_datetime_timestamp() {
    let code = transpile(
        "from datetime import datetime\ndef get_ts():\n    dt = datetime.now()\n    return dt.timestamp()",
    );
    assert!(!code.is_empty(), "datetime.timestamp: {}", code);
}

// =============================================================================
// Section 61: Additional stdlib numpy coverage - clip, norm
// =============================================================================

#[test]
fn test_np_clip() {
    let code = transpile(
        "import numpy as np\ndef clip_arr(a: list) -> list:\n    return np.clip(a, 0.0, 1.0)",
    );
    assert!(!code.is_empty(), "np.clip: {}", code);
}

// =============================================================================
// Section 62: os.environ containment
// =============================================================================

#[test]
fn test_in_os_environ() {
    let code = transpile(
        "import os\ndef check_env(key: str) -> bool:\n    return key in os.environ",
    );
    assert!(!code.is_empty(), "in os.environ: {}", code);
}

#[test]
fn test_not_in_os_environ() {
    let code = transpile(
        "import os\ndef missing_env(key: str) -> bool:\n    return key not in os.environ",
    );
    assert!(!code.is_empty(), "not in os.environ: {}", code);
}

// =============================================================================
// Section 63: Additional binary_ops - bitwise operators
// =============================================================================

#[test]
fn test_bitwise_and() {
    let code = transpile(
        "def bit_and(a: int, b: int) -> int:\n    return a & b",
    );
    assert!(!code.is_empty(), "bitwise and: {}", code);
}

#[test]
fn test_bitwise_or() {
    let code = transpile(
        "def bit_or(a: int, b: int) -> int:\n    return a | b",
    );
    assert!(!code.is_empty(), "bitwise or: {}", code);
}

#[test]
fn test_bitwise_xor() {
    let code = transpile(
        "def bit_xor(a: int, b: int) -> int:\n    return a ^ b",
    );
    assert!(!code.is_empty(), "bitwise xor: {}", code);
}

// =============================================================================
// Section 64: Additional call_generic.rs - enum/range patterns
// =============================================================================

#[test]
fn test_enumerate_call() {
    let code = transpile(
        "def indexed(items: list) -> list:\n    return list(enumerate(items))",
    );
    assert!(!code.is_empty(), "enumerate: {}", code);
}

#[test]
fn test_zip_call() {
    let code = transpile(
        "def pair(a: list, b: list) -> list:\n    return list(zip(a, b))",
    );
    assert!(!code.is_empty(), "zip: {}", code);
}

#[test]
fn test_reversed_call() {
    let code = transpile(
        "def rev(items: list) -> list:\n    return list(reversed(items))",
    );
    assert!(!code.is_empty(), "reversed: {}", code);
}

#[test]
fn test_sorted_call() {
    let code = transpile(
        "def sort(items: list) -> list:\n    return sorted(items)",
    );
    assert!(!code.is_empty(), "sorted: {}", code);
}

// =============================================================================
// Section 65: Additional convert_unary_and_call.rs - memoryview, chr, ord
// =============================================================================

#[test]
fn test_memoryview_noop() {
    let code = transpile(
        "def view(data: bytes) -> bytes:\n    return memoryview(data)",
    );
    assert!(!code.is_empty(), "memoryview: {}", code);
}

#[test]
fn test_chr_builtin() {
    let code = transpile("def to_char(n: int) -> str:\n    return chr(n)");
    assert!(!code.is_empty(), "chr: {}", code);
}

#[test]
fn test_ord_builtin() {
    let code = transpile(
        "def to_code(s: str) -> int:\n    return ord(s)",
    );
    assert!(!code.is_empty(), "ord: {}", code);
}

// =============================================================================
// Section 66: Additional convert_unary_and_call.rs - int/float/str casts
// =============================================================================

#[test]
fn test_int_cast_from_float() {
    let code = transpile(
        "def to_int(x: float) -> int:\n    return int(x)",
    );
    assert!(!code.is_empty(), "int(float): {}", code);
}

#[test]
fn test_float_cast_from_int() {
    let code = transpile(
        "def to_float(x: int) -> float:\n    return float(x)",
    );
    assert!(!code.is_empty(), "float(int): {}", code);
}

#[test]
fn test_str_cast_from_int() {
    let code = transpile(
        "def to_str(x: int) -> str:\n    return str(x)",
    );
    assert!(!code.is_empty(), "str(int): {}", code);
}

// =============================================================================
// Section 67: Additional range() patterns
// =============================================================================

#[test]
fn test_range_single_arg() {
    let code = transpile(
        "def count(n: int) -> list:\n    result = []\n    for i in range(n):\n        result.append(i)\n    return result",
    );
    assert!(!code.is_empty(), "range(n): {}", code);
}

#[test]
fn test_range_two_args() {
    let code = transpile(
        "def count_from(start: int, end: int) -> list:\n    result = []\n    for i in range(start, end):\n        result.append(i)\n    return result",
    );
    assert!(!code.is_empty(), "range(start, end): {}", code);
}

#[test]
fn test_range_three_args() {
    let code = transpile(
        "def count_step(start: int, end: int, step: int) -> list:\n    result = []\n    for i in range(start, end, step):\n        result.append(i)\n    return result",
    );
    assert!(!code.is_empty(), "range(start, end, step): {}", code);
}

// =============================================================================
// Section 68: calendar.month, calendar.monthcalendar
// =============================================================================

#[test]
fn test_calendar_month() {
    let code = transpile(
        "import calendar\ndef month_str(y: int, m: int) -> str:\n    return calendar.month(y, m)",
    );
    assert!(!code.is_empty(), "calendar.month: {}", code);
}

#[test]
fn test_calendar_monthcalendar() {
    let code = transpile(
        "import calendar\ndef month_cal(y: int, m: int) -> list:\n    return calendar.monthcalendar(y, m)",
    );
    assert!(!code.is_empty(), "calendar.monthcalendar: {}", code);
}

// =============================================================================
// Section 69: Additional statistics module - quantiles
// =============================================================================

#[test]
fn test_statistics_quantiles_default() {
    let code = transpile(
        "import statistics\ndef quartiles(data: list) -> list:\n    return statistics.quantiles(data)",
    );
    assert!(!code.is_empty(), "statistics.quantiles: {}", code);
}

#[test]
fn test_statistics_quantiles_with_n() {
    let code = transpile(
        "import statistics\ndef deciles(data: list) -> list:\n    return statistics.quantiles(data, 10)",
    );
    assert!(!code.is_empty(), "statistics.quantiles(n=10): {}", code);
}

// =============================================================================
// Section 70: Additional os.path methods
// =============================================================================

#[test]
fn test_os_path_realpath() {
    let code = transpile(
        "import os\ndef real(p: str) -> str:\n    return os.path.realpath(p)",
    );
    assert!(!code.is_empty(), "os.path.realpath: {}", code);
}

#[test]
fn test_os_path_getmtime() {
    let code = transpile(
        "import os\ndef mtime(p: str) -> float:\n    return os.path.getmtime(p)",
    );
    assert!(!code.is_empty(), "os.path.getmtime: {}", code);
}

// =============================================================================
// Section 71: Additional boolean/truthiness patterns
// =============================================================================

#[test]
fn test_and_with_string_truthiness() {
    let code = transpile(
        "def check(s: str) -> bool:\n    return len(s) > 0 and s[0] == \"a\"",
    );
    assert!(!code.is_empty(), "and with string: {}", code);
}

#[test]
fn test_or_with_empty_string_default() {
    let code = transpile(
        "def default_name(name: str) -> str:\n    return name or \"\"",
    );
    assert!(!code.is_empty(), "or empty string: {}", code);
}

// =============================================================================
// Section 72: len() on various types
// =============================================================================

#[test]
fn test_len_string() {
    let code = transpile(
        "def length(s: str) -> int:\n    return len(s)",
    );
    assert!(!code.is_empty(), "len(str): {}", code);
}

#[test]
fn test_len_list() {
    let code = transpile(
        "def count(items: list) -> int:\n    return len(items)",
    );
    assert!(!code.is_empty(), "len(list): {}", code);
}

#[test]
fn test_len_dict() {
    let code = transpile(
        "def num_keys(d: dict) -> int:\n    return len(d)",
    );
    assert!(!code.is_empty(), "len(dict): {}", code);
}

// =============================================================================
// Section 73: Additional in/not in patterns for string lists
// =============================================================================

#[test]
fn test_in_string_list() {
    let code = transpile(
        "def check(name: str) -> bool:\n    return name in [\"alice\", \"bob\"]",
    );
    assert!(!code.is_empty(), "in string list: {}", code);
}

#[test]
fn test_not_in_string_list() {
    let code = transpile(
        "def check(name: str) -> bool:\n    return name not in [\"alice\", \"bob\"]",
    );
    assert!(!code.is_empty(), "not in string list: {}", code);
}

// =============================================================================
// Section 74: Additional pathlib instance methods
// =============================================================================

#[test]
fn test_pathlib_write_text() {
    let code = transpile(
        "from pathlib import Path\ndef write(p: str, content: str):\n    Path(p).write_text(content)",
    );
    assert!(!code.is_empty(), "Path.write_text: {}", code);
}

#[test]
fn test_pathlib_read_bytes() {
    let code = transpile(
        "from pathlib import Path\ndef read_bin(p: str) -> bytes:\n    return Path(p).read_bytes()",
    );
    assert!(!code.is_empty(), "Path.read_bytes: {}", code);
}

// =============================================================================
// Section 75: Additional list*int patterns
// =============================================================================

#[test]
fn test_list_times_variable() {
    let code = transpile(
        "def zeros(n: int) -> list:\n    return [0] * n",
    );
    assert!(!code.is_empty(), "list*var: {}", code);
}

#[test]
fn test_list_times_expression() {
    let code = transpile(
        "def make(n: int) -> list:\n    return [True] * (n + 1)",
    );
    assert!(!code.is_empty(), "list*expr: {}", code);
}

// =============================================================================
// Section 76: Additional subprocess patterns
// =============================================================================

#[test]
fn test_subprocess_popen() {
    let ok = transpile_ok(
        "import subprocess\ndef spawn(cmd: list):\n    p = subprocess.Popen(cmd)\n    return p",
    );
    assert!(ok, "subprocess.Popen should transpile");
}

// =============================================================================
// Section 77: Additional number conversion edge cases
// =============================================================================

#[test]
fn test_int_from_string_literal() {
    let code = transpile(
        "def parse() -> int:\n    return int(\"42\")",
    );
    assert!(!code.is_empty(), "int(str): {}", code);
}

#[test]
fn test_float_from_string_literal() {
    let code = transpile(
        "def parse() -> float:\n    return float(\"3.14\")",
    );
    assert!(!code.is_empty(), "float(str): {}", code);
}

#[test]
fn test_bool_of_empty_string() {
    let code = transpile(
        "def falsy() -> bool:\n    return bool(\"\")",
    );
    assert!(!code.is_empty(), "bool(empty str): {}", code);
}
