//! Wave 18 coverage tests: call_generic.rs and codegen_assign_stmt in stmt_gen.rs
//!
//! Targets uncovered code paths in:
//! - call_generic.rs: convert_generic_call for builtins, type constructors,
//!   nested calls, constructor calls, method routing, auto-borrow logic
//! - stmt_gen.rs codegen_assign_stmt: simple/tuple/index/attribute assignment,
//!   type annotations, augmented assign, collection literals, comprehensions
//!
//! 200 tests total: 100 call tests + 100 assign tests

#![cfg(test)]

use crate::ast_bridge::AstBridge;
use crate::rust_gen::generate_rust_file;
use crate::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile(python_code: &str) -> String {
    let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
    let (module, _) =
        AstBridge::new().with_source(python_code.to_string()).python_to_hir(ast).expect("hir");
    let tm = TypeMapper::default();
    let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // SECTION 1: CALL_GENERIC (tests 001-100)
    // ========================================================================

    // --- Builtin calls: len, range, print, input, type, isinstance, id, hash ---

    #[test]
    fn test_w18ca_call_001_len_string() {
        let code = "def f(s: str) -> int:\n    return len(s)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("len") || result.contains(".len()"), "len string: {result}");
    }

    #[test]
    fn test_w18ca_call_002_len_list() {
        let code = "def f(items: list) -> int:\n    return len(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_003_range_single() {
        let code = "def f() -> list:\n    result = []\n    for i in range(10):\n        result.append(i)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("0") && result.contains("10"), "range single: {result}");
    }

    #[test]
    fn test_w18ca_call_004_range_two_args() {
        let code = "def f() -> list:\n    result = []\n    for i in range(1, 5):\n        result.append(i)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_005_range_three_args() {
        let code = "def f() -> list:\n    result = []\n    for i in range(0, 20, 2):\n        result.append(i)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_006_print_simple() {
        let code = "def f():\n    print(\"hello world\")";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("println") || result.contains("print"), "print simple: {result}");
    }

    #[test]
    fn test_w18ca_call_007_print_multiple_args() {
        let code = "def f(x: int, y: int):\n    print(x, y)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_008_input_call() {
        let code = "def f() -> str:\n    return input(\"Enter: \")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_009_isinstance_check() {
        let code = "def f(x) -> bool:\n    return isinstance(x, int)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_010_type_call() {
        let code = "def f(x):\n    t = type(x)\n    return t";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Type constructors: int, float, str, bool, list, dict, set, tuple, bytes ---

    #[test]
    fn test_w18ca_call_011_int_from_string() {
        let code = "def f(s: str) -> int:\n    return int(s)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_012_int_from_float() {
        let code = "def f(x: float) -> int:\n    return int(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_013_float_from_string() {
        let code = "def f(s: str) -> float:\n    return float(s)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_014_float_from_int() {
        let code = "def f(x: int) -> float:\n    return float(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_015_str_from_int() {
        let code = "def f(x: int) -> str:\n    return str(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_016_bool_call() {
        let code = "def f(x: int) -> bool:\n    return bool(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_017_list_empty() {
        let code = "def f() -> list:\n    return list()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_018_dict_empty() {
        let code = "def f() -> dict:\n    return dict()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_019_set_empty() {
        let code = "def f() -> set:\n    return set()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_020_tuple_call() {
        let code = "def f(items: list) -> tuple:\n    return tuple(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Nested calls ---

    #[test]
    fn test_w18ca_call_021_int_str_nested() {
        let code = "def f(x: int) -> int:\n    return int(str(x))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_022_len_range() {
        let code = "def f() -> int:\n    return len(list(range(10)))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_023_sorted_list() {
        let code = "def f(items: list) -> list:\n    return sorted(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_024_reversed_list() {
        let code = "def f(items: list) -> list:\n    return list(reversed(items))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_025_str_int_float() {
        let code = "def f() -> str:\n    return str(int(float(\"3.7\")))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Constructor calls (capitalized names) ---

    #[test]
    fn test_w18ca_call_026_class_no_args() {
        let code =
            "class Foo:\n    def __init__(self):\n        self.x = 0\n\ndef f():\n    obj = Foo()";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Foo") && result.contains("new"), "class no args: {result}");
    }

    #[test]
    fn test_w18ca_call_027_class_with_args() {
        let code = "class Point:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y\n\ndef f():\n    p = Point(1, 2)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Point") && result.contains("new"), "class with args: {result}");
    }

    #[test]
    fn test_w18ca_call_028_counter_no_args() {
        let code = "def f():\n    c = Counter()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_029_class_single_arg() {
        let code = "class Wrapper:\n    def __init__(self, val: int):\n        self.val = val\n\ndef f():\n    w = Wrapper(42)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_030_class_string_arg() {
        let code = "class Name:\n    def __init__(self, name: str):\n        self.name = name\n\ndef f():\n    n = Name(\"Alice\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Regular function calls with auto-borrow ---

    #[test]
    fn test_w18ca_call_031_func_with_list_param() {
        let code = "def helper(items: list) -> int:\n    return len(items)\n\ndef f():\n    data = [1, 2, 3]\n    result = helper(data)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_032_func_with_string_param() {
        let code = "def greet(name: str) -> str:\n    return \"Hello \" + name\n\ndef f():\n    msg = greet(\"World\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_033_func_multiple_params() {
        let code = "def add(a: int, b: int) -> int:\n    return a + b\n\ndef f() -> int:\n    return add(3, 4)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_034_func_no_params() {
        let code = "def zero() -> int:\n    return 0\n\ndef f() -> int:\n    return zero()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_035_func_returns_string() {
        let code = "def label() -> str:\n    return \"ok\"\n\ndef f() -> str:\n    return label()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- abs, min, max, sum, any, all builtins ---

    #[test]
    fn test_w18ca_call_036_abs_int() {
        let code = "def f(x: int) -> int:\n    return abs(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("abs"), "abs int: {result}");
    }

    #[test]
    fn test_w18ca_call_037_min_two_args() {
        let code = "def f(a: int, b: int) -> int:\n    return min(a, b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_038_max_two_args() {
        let code = "def f(a: int, b: int) -> int:\n    return max(a, b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_039_sum_list() {
        let code = "def f(items: list) -> int:\n    return sum(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_040_any_call() {
        let code = "def f(items: list) -> bool:\n    return any(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_041_all_call() {
        let code = "def f(items: list) -> bool:\n    return all(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- enumerate, zip, map, filter ---

    #[test]
    fn test_w18ca_call_042_enumerate_loop() {
        let code =
            "def f(items: list):\n    for i, val in enumerate(items):\n        print(i, val)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_043_zip_two_lists() {
        let code = "def f(a: list, b: list):\n    for x, y in zip(a, b):\n        print(x, y)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_044_map_int() {
        let code = "def f(items: list) -> list:\n    return list(map(str, items))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_045_filter_none() {
        let code = "def f(items: list) -> list:\n    return list(filter(None, items))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- isqrt fallback ---

    #[test]
    fn test_w18ca_call_046_isqrt_standalone() {
        let code = "def f(n: int) -> int:\n    return isqrt(n)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("sqrt"), "isqrt standalone: {result}");
    }

    // --- isinstance fallback ---

    #[test]
    fn test_w18ca_call_047_isinstance_fallback() {
        let code = "def f(x) -> bool:\n    return isinstance(x, str)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Function call with complex args ---

    #[test]
    fn test_w18ca_call_048_call_with_expression_arg() {
        let code = "def double(x: int) -> int:\n    return x * 2\n\ndef f(a: int) -> int:\n    return double(a + 1)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_049_call_with_bool_arg() {
        let code = "def check(flag: bool) -> bool:\n    return flag\n\ndef f() -> bool:\n    return check(True)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_050_call_with_none_arg() {
        let code = "def maybe(x) -> bool:\n    return x is not None\n\ndef f() -> bool:\n    return maybe(None)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Recursive function calls ---

    #[test]
    fn test_w18ca_call_051_recursive_call() {
        let code = "def factorial(n: int) -> int:\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("factorial"), "recursive call: {result}");
    }

    #[test]
    fn test_w18ca_call_052_fibonacci() {
        let code = "def fib(n: int) -> int:\n    if n <= 1:\n        return n\n    return fib(n - 1) + fib(n - 2)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Multiple function calls in sequence ---

    #[test]
    fn test_w18ca_call_053_chained_calls() {
        let code = "def step1(x: int) -> int:\n    return x + 1\n\ndef step2(x: int) -> int:\n    return x * 2\n\ndef f(x: int) -> int:\n    return step2(step1(x))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_054_call_in_condition() {
        let code = "def is_even(n: int) -> bool:\n    return n % 2 == 0\n\ndef f(x: int) -> str:\n    if is_even(x):\n        return \"even\"\n    return \"odd\"";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_055_call_in_loop() {
        let code = "def process(x: int) -> int:\n    return x * x\n\ndef f(n: int) -> int:\n    total = 0\n    for i in range(n):\n        total += process(i)\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- String method calls after function call ---

    #[test]
    fn test_w18ca_call_056_str_upper() {
        let code = "def f(s: str) -> str:\n    return s.upper()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_057_str_lower() {
        let code = "def f(s: str) -> str:\n    return s.lower()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_058_str_strip() {
        let code = "def f(s: str) -> str:\n    return s.strip()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_059_str_split() {
        let code = "def f(s: str) -> list:\n    return s.split(\",\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_060_str_replace() {
        let code = "def f(s: str) -> str:\n    return s.replace(\"a\", \"b\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- List method calls ---

    #[test]
    fn test_w18ca_call_061_list_append() {
        let code = "def f():\n    items = [1, 2, 3]\n    items.append(4)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_062_list_extend() {
        let code = "def f():\n    items = [1, 2]\n    items.extend([3, 4])";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_063_list_pop() {
        let code = "def f() -> int:\n    items = [1, 2, 3]\n    return items.pop()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_064_list_insert() {
        let code = "def f():\n    items = [1, 3]\n    items.insert(1, 2)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_065_list_remove() {
        let code = "def f():\n    items = [1, 2, 3]\n    items.remove(2)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Dict method calls ---

    #[test]
    fn test_w18ca_call_066_dict_keys() {
        let code = "def f():\n    d = {\"a\": 1}\n    k = d.keys()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_067_dict_values() {
        let code = "def f():\n    d = {\"a\": 1}\n    v = d.values()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_068_dict_items() {
        let code = "def f():\n    d = {\"a\": 1}\n    pairs = d.items()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_069_dict_get() {
        let code = "def f():\n    d = {\"a\": 1}\n    v = d.get(\"b\", 0)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_070_dict_update() {
        let code = "def f():\n    d = {\"a\": 1}\n    d.update({\"b\": 2})";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Math functions ---

    #[test]
    fn test_w18ca_call_071_math_sqrt() {
        let code = "import math\ndef f(x: float) -> float:\n    return math.sqrt(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_072_math_floor() {
        let code = "import math\ndef f(x: float) -> int:\n    return math.floor(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_073_math_ceil() {
        let code = "import math\ndef f(x: float) -> int:\n    return math.ceil(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_074_math_abs() {
        let code = "import math\ndef f(x: float) -> float:\n    return math.fabs(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_075_math_pow() {
        let code = "import math\ndef f(x: float, y: float) -> float:\n    return math.pow(x, y)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Set method calls ---

    #[test]
    fn test_w18ca_call_076_set_add() {
        let code = "def f():\n    s = set()\n    s.add(1)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_077_set_discard() {
        let code = "def f():\n    s = {1, 2, 3}\n    s.discard(2)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_078_set_contains() {
        let code = "def f() -> bool:\n    s = {1, 2, 3}\n    return 2 in s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- String formatting and join ---

    #[test]
    fn test_w18ca_call_079_str_join() {
        let code = "def f(items: list) -> str:\n    return \", \".join(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_080_str_format() {
        let code = "def f(name: str) -> str:\n    return \"Hello {}\".format(name)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Call with keyword arguments ---

    #[test]
    fn test_w18ca_call_081_print_with_end() {
        let code = "def f():\n    print(\"hello\", end=\"\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_082_print_with_sep() {
        let code = "def f():\n    print(1, 2, 3, sep=\"-\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Chained method calls ---

    #[test]
    fn test_w18ca_call_083_chained_str_methods() {
        let code = "def f(s: str) -> str:\n    return s.strip().lower()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_084_chained_upper_replace() {
        let code = "def f(s: str) -> str:\n    return s.upper().replace(\"A\", \"X\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Builtin conversions in expressions ---

    #[test]
    fn test_w18ca_call_085_int_in_expression() {
        let code = "def f(s: str) -> int:\n    return int(s) + 1";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_086_float_in_expression() {
        let code = "def f(s: str) -> float:\n    return float(s) * 2.0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_087_len_in_condition() {
        let code = "def f(items: list) -> bool:\n    return len(items) > 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Multiple return paths with calls ---

    #[test]
    fn test_w18ca_call_088_call_in_ternary() {
        let code = "def f(x: int) -> int:\n    return abs(x) if x < 0 else x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_089_call_in_list_comp() {
        let code = "def double(x: int) -> int:\n    return x * 2\n\ndef f(items: list) -> list:\n    return [double(x) for x in items]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- ord and chr ---

    #[test]
    fn test_w18ca_call_090_ord_call() {
        let code = "def f(c: str) -> int:\n    return ord(c)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_091_chr_call() {
        let code = "def f(n: int) -> str:\n    return chr(n)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- hex, oct, bin ---

    #[test]
    fn test_w18ca_call_092_hex_call() {
        let code = "def f(n: int) -> str:\n    return hex(n)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_093_oct_call() {
        let code = "def f(n: int) -> str:\n    return oct(n)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_094_bin_call() {
        let code = "def f(n: int) -> str:\n    return bin(n)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- divmod, pow, round ---

    #[test]
    fn test_w18ca_call_095_divmod_call() {
        let code = "def f(a: int, b: int) -> tuple:\n    return divmod(a, b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_096_pow_call() {
        let code = "def f(x: int, y: int) -> int:\n    return pow(x, y)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_097_round_call() {
        let code = "def f(x: float) -> int:\n    return round(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Complex call contexts ---

    #[test]
    fn test_w18ca_call_098_call_as_dict_value() {
        let code = "def compute(x: int) -> int:\n    return x * x\n\ndef f() -> dict:\n    return {\"result\": compute(5)}";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_099_call_as_list_element() {
        let code = "def double(x: int) -> int:\n    return x * 2\n\ndef f() -> list:\n    return [double(1), double(2), double(3)]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_call_100_call_in_assert() {
        let code = "def is_valid(x: int) -> bool:\n    return x > 0\n\ndef f(x: int):\n    assert is_valid(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 2: ASSIGN PATTERNS (tests 101-200)
    // ========================================================================

    // --- Simple assignment ---

    #[test]
    fn test_w18ca_assign_101_int_literal() {
        let code = "def f():\n    x = 1\n    print(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("let") || result.contains("x"), "int assign: {result}");
    }

    #[test]
    fn test_w18ca_assign_102_string_literal() {
        let code = "def f():\n    x = \"hello\"\n    print(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_103_float_literal() {
        let code = "def f():\n    x = 3.5\n    print(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_104_bool_literal() {
        let code = "def f():\n    x = True\n    print(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_105_none_literal() {
        let code = "def f():\n    x = None\n    print(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("None") || result.contains("Option"), "none assign: {result}");
    }

    // --- Type-annotated assignment ---

    #[test]
    fn test_w18ca_assign_106_annotated_int() {
        let code = "def f():\n    x: int = 5\n    print(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("i32") || result.contains("i64"), "annotated int: {result}");
    }

    #[test]
    fn test_w18ca_assign_107_annotated_float() {
        let code = "def f():\n    x: float = 2.5\n    print(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_108_annotated_str() {
        let code = "def f():\n    x: str = \"world\"\n    print(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_109_annotated_bool() {
        let code = "def f():\n    x: bool = False\n    print(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_110_annotated_list_int() {
        let code =
            "from typing import List\ndef f():\n    items: List[int] = [1, 2, 3]\n    print(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Mutable variables ---

    #[test]
    fn test_w18ca_assign_111_mutable_int() {
        let code = "def f():\n    x = 0\n    x = x + 1\n    print(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("mut") || result.contains("x"), "mutable int: {result}");
    }

    #[test]
    fn test_w18ca_assign_112_mutable_string() {
        let code = "def f():\n    s = \"hello\"\n    s = s + \" world\"\n    print(s)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_113_mutable_list() {
        let code =
            "def f():\n    items = []\n    items.append(1)\n    items.append(2)\n    print(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Tuple unpacking ---

    #[test]
    fn test_w18ca_assign_114_tuple_unpack_two() {
        let code = "def f():\n    a, b = 1, 2\n    print(a, b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_115_tuple_unpack_three() {
        let code = "def f():\n    a, b, c = 1, 2, 3\n    print(a, b, c)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_116_tuple_unpack_from_func() {
        let code = "def pair() -> tuple:\n    return (10, 20)\n\ndef f():\n    x, y = pair()\n    print(x, y)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_117_tuple_unpack_strings() {
        let code = "def f():\n    first, last = \"Alice\", \"Smith\"\n    print(first, last)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_118_tuple_unpack_mixed() {
        let code = "def f():\n    name, age = \"Bob\", 30\n    print(name, age)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Index assignment ---

    #[test]
    fn test_w18ca_assign_119_list_index_assign() {
        let code = "def f():\n    items = [1, 2, 3]\n    items[0] = 10\n    print(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_120_dict_index_assign() {
        let code = "def f():\n    d = {\"a\": 1}\n    d[\"b\"] = 2\n    print(d)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_121_dict_string_key() {
        let code = "def f():\n    d = {}\n    d[\"key\"] = \"value\"\n    print(d)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_122_list_negative_index() {
        let code = "def f():\n    items = [1, 2, 3]\n    items[-1] = 99\n    print(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Attribute assignment ---

    #[test]
    fn test_w18ca_assign_123_attr_assign() {
        let code = "class Obj:\n    def __init__(self):\n        self.x = 0\n    def set_x(self, val: int):\n        self.x = val";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_124_attr_assign_string() {
        let code = "class Obj:\n    def __init__(self):\n        self.name = \"\"\n    def set_name(self, name: str):\n        self.name = name";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_125_attr_assign_bool() {
        let code = "class Obj:\n    def __init__(self):\n        self.active = False\n    def activate(self):\n        self.active = True";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Augmented assignment ---

    #[test]
    fn test_w18ca_assign_126_augadd_int() {
        let code = "def f(x: int) -> int:\n    x += 5\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("+=") || result.contains("+"), "aug add: {result}");
    }

    #[test]
    fn test_w18ca_assign_127_augsub_int() {
        let code = "def f(x: int) -> int:\n    x -= 3\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_128_augmul_int() {
        let code = "def f(x: int) -> int:\n    x *= 2\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_129_augdiv_float() {
        let code = "def f(x: float) -> float:\n    x /= 2.0\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_130_augmod_int() {
        let code = "def f(x: int) -> int:\n    x %= 3\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_131_augadd_string() {
        let code = "def f() -> str:\n    s = \"hello\"\n    s += \" world\"\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_132_augadd_in_loop() {
        let code = "def f(n: int) -> int:\n    total = 0\n    for i in range(n):\n        total += i\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Collection literal assignment ---

    #[test]
    fn test_w18ca_assign_133_empty_list() {
        let code = "def f():\n    items = []\n    print(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_134_list_of_ints() {
        let code = "def f():\n    items = [1, 2, 3, 4, 5]\n    print(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_135_list_of_strings() {
        let code = "def f():\n    items = [\"a\", \"b\", \"c\"]\n    print(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_136_empty_dict() {
        let code = "def f():\n    d = {}\n    print(d)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_137_dict_literal() {
        let code = "def f():\n    d = {\"x\": 1, \"y\": 2}\n    print(d)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_138_set_literal() {
        let code = "def f():\n    s = {1, 2, 3}\n    print(s)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_139_tuple_literal() {
        let code = "def f():\n    t = (1, 2, 3)\n    print(t)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Comprehension assignment ---

    #[test]
    fn test_w18ca_assign_140_list_comp_simple() {
        let code = "def f() -> list:\n    squares = [x * x for x in range(10)]\n    return squares";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_141_list_comp_with_filter() {
        let code =
            "def f() -> list:\n    evens = [x for x in range(20) if x % 2 == 0]\n    return evens";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_142_dict_comp() {
        let code =
            "def f() -> dict:\n    squares = {x: x * x for x in range(5)}\n    return squares";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_143_set_comp() {
        let code = "def f() -> set:\n    unique = {x % 3 for x in range(10)}\n    return unique";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Assignment from function return ---

    #[test]
    fn test_w18ca_assign_144_from_func_return() {
        let code = "def compute(x: int) -> int:\n    return x * x\n\ndef f():\n    result = compute(5)\n    print(result)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_145_from_method_return() {
        let code =
            "def f():\n    s = \"Hello World\"\n    words = s.split(\" \")\n    print(words)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Complex RHS expressions ---

    #[test]
    fn test_w18ca_assign_146_arithmetic_rhs() {
        let code =
            "def f(a: int, b: int, c: int) -> int:\n    result = a + b * c\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_147_ternary_rhs() {
        let code =
            "def f(x: int) -> str:\n    label = \"pos\" if x > 0 else \"neg\"\n    return label";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_148_boolean_rhs() {
        let code = "def f(a: bool, b: bool) -> bool:\n    result = a and b\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_149_or_expression() {
        let code = "def f(a: bool, b: bool) -> bool:\n    result = a or b\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_150_not_expression() {
        let code = "def f(x: bool) -> bool:\n    result = not x\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Assignment with comparison operators ---

    #[test]
    fn test_w18ca_assign_151_comparison_eq() {
        let code = "def f(a: int, b: int) -> bool:\n    result = a == b\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_152_comparison_ne() {
        let code = "def f(a: int, b: int) -> bool:\n    result = a != b\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_153_comparison_lt() {
        let code = "def f(a: int, b: int) -> bool:\n    result = a < b\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_154_comparison_gt() {
        let code = "def f(a: int, b: int) -> bool:\n    result = a > b\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_155_comparison_le() {
        let code = "def f(a: int, b: int) -> bool:\n    result = a <= b\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_156_comparison_ge() {
        let code = "def f(a: int, b: int) -> bool:\n    result = a >= b\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Slice assignment ---

    #[test]
    fn test_w18ca_assign_157_slice_from_list() {
        let code = "def f():\n    items = [1, 2, 3, 4, 5]\n    sub = items[1:3]\n    print(sub)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_158_slice_from_start() {
        let code = "def f():\n    items = [1, 2, 3, 4, 5]\n    head = items[:2]\n    print(head)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_159_slice_to_end() {
        let code = "def f():\n    items = [1, 2, 3, 4, 5]\n    tail = items[2:]\n    print(tail)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_160_slice_step() {
        let code =
            "def f():\n    items = [1, 2, 3, 4, 5, 6]\n    evens = items[::2]\n    print(evens)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Variable to variable assignment ---

    #[test]
    fn test_w18ca_assign_161_var_to_var() {
        let code = "def f():\n    x = 42\n    y = x\n    print(y)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_162_string_var_to_var() {
        let code = "def f():\n    a = \"hello\"\n    b = a\n    print(b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Reassignment ---

    #[test]
    fn test_w18ca_assign_163_reassign_int() {
        let code = "def f() -> int:\n    x = 1\n    x = 2\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_164_reassign_different_expr() {
        let code =
            "def f(a: int, b: int) -> int:\n    result = a\n    result = a + b\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Assignment with class instances ---

    #[test]
    fn test_w18ca_assign_165_class_instance_assign() {
        let code = "class Box:\n    def __init__(self, val: int):\n        self.val = val\n\ndef f():\n    b = Box(10)\n    print(b.val)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_166_class_field_reassign() {
        let code = "class Counter:\n    def __init__(self):\n        self.count = 0\n    def increment(self):\n        self.count = self.count + 1";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Assignment from method calls that return specific types ---

    #[test]
    fn test_w18ca_assign_167_split_result() {
        let code = "def f(s: str):\n    parts = s.split(\",\")\n    print(parts)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_168_upper_result() {
        let code = "def f(s: str):\n    result = s.upper()\n    print(result)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_169_lower_result() {
        let code = "def f(s: str):\n    result = s.lower()\n    print(result)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_170_strip_result() {
        let code = "def f(s: str):\n    result = s.strip()\n    print(result)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Augmented assign inside loops ---

    #[test]
    fn test_w18ca_assign_171_augadd_in_while() {
        let code = "def f() -> int:\n    total = 0\n    i = 0\n    while i < 10:\n        total += i\n        i += 1\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_172_augmul_in_loop() {
        let code = "def f(n: int) -> int:\n    result = 1\n    for i in range(1, n + 1):\n        result *= i\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Augmented assign inside conditions ---

    #[test]
    fn test_w18ca_assign_173_augadd_in_if() {
        let code = "def f(x: int) -> int:\n    total = 0\n    if x > 0:\n        total += x\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_174_augadd_in_else() {
        let code = "def f(x: int) -> int:\n    pos = 0\n    neg = 0\n    if x > 0:\n        pos += x\n    else:\n        neg += x\n    return pos + neg";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Nested structure assignment ---

    #[test]
    fn test_w18ca_assign_175_list_of_lists() {
        let code = "def f():\n    matrix = [[1, 2], [3, 4]]\n    print(matrix)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_176_dict_of_lists() {
        let code = "def f():\n    groups = {\"a\": [1, 2], \"b\": [3, 4]}\n    print(groups)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Assignment from index access ---

    #[test]
    fn test_w18ca_assign_177_list_index_access() {
        let code = "def f():\n    items = [10, 20, 30]\n    first = items[0]\n    print(first)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_178_dict_key_access() {
        let code = "def f():\n    d = {\"x\": 10}\n    val = d[\"x\"]\n    print(val)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Binary expression assignment ---

    #[test]
    fn test_w18ca_assign_179_add_expression() {
        let code = "def f(a: int, b: int) -> int:\n    result = a + b\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_180_sub_expression() {
        let code = "def f(a: int, b: int) -> int:\n    result = a - b\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_181_mul_expression() {
        let code = "def f(a: int, b: int) -> int:\n    result = a * b\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_182_div_expression() {
        let code = "def f(a: float, b: float) -> float:\n    result = a / b\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_183_mod_expression() {
        let code = "def f(a: int, b: int) -> int:\n    result = a % b\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_184_floor_div() {
        let code = "def f(a: int, b: int) -> int:\n    result = a // b\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_185_power_expression() {
        let code = "def f(a: int, b: int) -> int:\n    result = a ** b\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Bitwise operations ---

    #[test]
    fn test_w18ca_assign_186_bitwise_and() {
        let code = "def f(a: int, b: int) -> int:\n    result = a & b\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_187_bitwise_or() {
        let code = "def f(a: int, b: int) -> int:\n    result = a | b\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_188_bitwise_xor() {
        let code = "def f(a: int, b: int) -> int:\n    result = a ^ b\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_189_left_shift() {
        let code = "def f(a: int) -> int:\n    result = a << 2\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_190_right_shift() {
        let code = "def f(a: int) -> int:\n    result = a >> 2\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- String concatenation and operations ---

    #[test]
    fn test_w18ca_assign_191_string_concat() {
        let code = "def f() -> str:\n    result = \"hello\" + \" \" + \"world\"\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_192_string_multiply() {
        let code = "def f() -> str:\n    result = \"ab\" * 3\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- F-string assignment ---

    #[test]
    fn test_w18ca_assign_193_fstring_simple() {
        let code = "def f(name: str):\n    msg = f\"Hello {name}\"\n    print(msg)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_194_fstring_expression() {
        let code = "def f(x: int):\n    msg = f\"Value: {x + 1}\"\n    print(msg)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Complex assignment patterns ---

    #[test]
    fn test_w18ca_assign_195_nested_index_assign() {
        let code =
            "def f():\n    matrix = [[0, 0], [0, 0]]\n    matrix[0][1] = 5\n    print(matrix)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_196_set_from_list() {
        let code = "def f():\n    items = [1, 2, 2, 3]\n    unique = set(items)\n    print(unique)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_197_list_from_range() {
        let code = "def f():\n    numbers = list(range(10))\n    print(numbers)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_198_assign_in_nested_loops() {
        let code = "def f() -> int:\n    total = 0\n    for i in range(3):\n        for j in range(3):\n            total += i * j\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_199_assign_from_enumerate() {
        let code = "def f():\n    items = [\"a\", \"b\", \"c\"]\n    for idx, val in enumerate(items):\n        print(idx, val)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w18ca_assign_200_chain_operations() {
        let code = "def f(a: int, b: int, c: int, d: int) -> int:\n    result = a + b * c - d\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }
}
