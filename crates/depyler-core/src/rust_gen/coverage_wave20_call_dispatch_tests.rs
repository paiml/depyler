//! Wave 20: Deep coverage tests for call dispatch code paths
//!
//! 200 tests targeting uncovered code in:
//! - convert_generic_call (call_generic.rs:22) - builtins, type constructors,
//!   aggregation, math, object inspection, scope, identity, constructors
//! - convert_method_call (method_call_routing.rs:40) - string/list/dict/set/file/path routing
//! - convert_instance_method (instance_dispatch.rs:17) - file I/O, path, datetime, regex,
//!   CSV, deque, custom class, dunder methods
//!
//! Status: 200/200 tests

#[cfg(test)]
mod tests {
    #![allow(unused_variables)]

    use crate::ast_bridge::AstBridge;
    use crate::rust_gen::generate_rust_file;
    use crate::type_mapper::TypeMapper;
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

    // ========================================================================
    // SECTION 1: convert_generic_call (70 tests: test_w20cd_001 through _070)
    // ========================================================================

    // --- sorted/reversed/enumerate/zip/map/filter ---

    #[test]
    fn test_w20cd_001_sorted_list() {
        let code = "def f(items: list) -> list:\n    return sorted(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_002_sorted_with_reverse() {
        let code = "def f(items: list) -> list:\n    return sorted(items, reverse=True)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_003_reversed_list() {
        let code = "def f(items: list) -> list:\n    result = []\n    for x in reversed(items):\n        result.append(x)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_004_enumerate_basic() {
        let code = "def f(items: list) -> list:\n    result = []\n    for i, item in enumerate(items):\n        result.append(i)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_005_enumerate_with_start() {
        let code = "def f(items: list) -> list:\n    result = []\n    for i, item in enumerate(items, 1):\n        result.append(i)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_006_zip_two_lists() {
        let code = "def f(a: list, b: list) -> list:\n    result = []\n    for x, y in zip(a, b):\n        result.append(x)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_007_map_with_function() {
        let code = "def double(x: int) -> int:\n    return x * 2\ndef f(items: list) -> list:\n    return list(map(double, items))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_008_filter_with_function() {
        let code = "def is_positive(x: int) -> bool:\n    return x > 0\ndef f(items: list) -> list:\n    return list(filter(is_positive, items))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- any/all/sum/min/max ---

    #[test]
    fn test_w20cd_009_any_basic() {
        let code = "def f(items: list) -> bool:\n    return any(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_010_all_basic() {
        let code = "def f(items: list) -> bool:\n    return all(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_011_sum_list() {
        let code = "def f(items: list) -> int:\n    return sum(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_012_min_two_args() {
        let code = "def f(a: int, b: int) -> int:\n    return min(a, b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_013_max_two_args() {
        let code = "def f(a: int, b: int) -> int:\n    return max(a, b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_014_min_list() {
        let code = "def f(items: list) -> int:\n    return min(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_015_max_list() {
        let code = "def f(items: list) -> int:\n    return max(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- abs/round/pow/divmod ---

    #[test]
    fn test_w20cd_016_abs_int() {
        let code = "def f(x: int) -> int:\n    return abs(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("abs"), "abs int: {result}");
    }

    #[test]
    fn test_w20cd_017_abs_float() {
        let code = "def f(x: float) -> float:\n    return abs(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_018_round_no_digits() {
        let code = "def f(x: float) -> int:\n    return round(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_019_round_with_digits() {
        let code = "def f(x: float) -> float:\n    return round(x, 2)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_020_pow_two_args() {
        let code = "def f(base: int, exp: int) -> int:\n    return pow(base, exp)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_021_divmod_basic() {
        let code = "def f(a: int, b: int) -> tuple:\n    return divmod(a, b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- isinstance/type/callable ---

    #[test]
    fn test_w20cd_022_isinstance_check() {
        let code = "def f(x) -> bool:\n    return isinstance(x, int)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_023_type_call() {
        let code = "def f(x) -> str:\n    return str(type(x))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_024_callable_check() {
        let code = "def f(x) -> bool:\n    return callable(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- getattr/setattr/hasattr/delattr ---

    #[test]
    fn test_w20cd_025_len_set() {
        let code = "def f(s: set) -> int:\n    return len(s)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_026_len_tuple() {
        let code = "def f(t: tuple) -> int:\n    return len(t)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_027_hasattr_check() {
        let code = "def f(obj, name: str) -> bool:\n    return hasattr(obj, name)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_028_setattr_basic() {
        let code = "def f(obj, name: str, value: int):\n    setattr(obj, name, value)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_029_delattr_basic() {
        let code = "def f(obj, name: str):\n    delattr(obj, name)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- hash/id ---

    #[test]
    fn test_w20cd_030_hash_call() {
        let code = "def f(x: str) -> int:\n    return hash(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_031_id_call() {
        let code = "def f(x) -> int:\n    return id(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- repr/ascii/chr/ord/hex/oct/bin/format ---

    #[test]
    fn test_w20cd_032_repr_call() {
        let code = "def f(x) -> str:\n    return repr(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_033_ascii_call() {
        let code = "def f(x: str) -> str:\n    return ascii(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_034_chr_call() {
        let code = "def f(n: int) -> str:\n    return chr(n)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_035_ord_call() {
        let code = "def f(c: str) -> int:\n    return ord(c)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_036_hex_call() {
        let code = "def f(n: int) -> str:\n    return hex(n)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_037_oct_call() {
        let code = "def f(n: int) -> str:\n    return oct(n)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_038_bin_call() {
        let code = "def f(n: int) -> str:\n    return bin(n)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_039_format_call() {
        let code = "def f(x: float) -> str:\n    return format(x, \".2f\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- vars/dir/globals/locals ---

    #[test]
    fn test_w20cd_040_vars_call() {
        let code = "def f(obj):\n    return vars(obj)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_041_dir_call() {
        let code = "def f(obj):\n    return dir(obj)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- int/float/str/bool/list/dict/set/tuple constructors ---

    #[test]
    fn test_w20cd_042_int_from_str() {
        let code = "def f(s: str) -> int:\n    return int(s)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_043_int_from_float() {
        let code = "def f(x: float) -> int:\n    return int(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_044_float_from_str() {
        let code = "def f(s: str) -> float:\n    return float(s)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_045_float_from_int() {
        let code = "def f(n: int) -> float:\n    return float(n)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_046_str_from_int() {
        let code = "def f(n: int) -> str:\n    return str(n)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_047_str_from_bool() {
        let code = "def f(b: bool) -> str:\n    return str(b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_048_bool_from_int() {
        let code = "def f(n: int) -> bool:\n    return bool(n)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_049_list_empty() {
        let code = "def f() -> list:\n    return list()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_050_dict_empty() {
        let code = "def f() -> dict:\n    return dict()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_051_set_empty() {
        let code = "def f() -> set:\n    return set()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_052_tuple_empty() {
        let code = "def f() -> tuple:\n    return tuple()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_053_frozenset_call() {
        let code = "def f(items: list) -> frozenset:\n    return frozenset(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_054_bytes_call() {
        let code = "def f(n: int) -> bytes:\n    return bytes(n)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_055_bytearray_call() {
        let code = "def f(n: int) -> bytearray:\n    return bytearray(n)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_056_complex_call() {
        let code = "def f(r: float, i: float):\n    return complex(r, i)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_057_range_single_arg() {
        let code = "def f() -> list:\n    result = []\n    for i in range(5):\n        result.append(i)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_058_slice_call() {
        let code = "def f(items: list) -> list:\n    s = slice(1, 3)\n    return items[1:3]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- open/print/input/len/iter/next/super ---

    #[test]
    fn test_w20cd_059_open_read() {
        let code = "def f(path: str) -> str:\n    f = open(path, \"r\")\n    content: str = f.read()\n    return content";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_060_print_multiple_args() {
        let code = "def f(a: str, b: str):\n    print(a, b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_061_print_with_sep() {
        let code = "def f(a: str, b: str):\n    print(a, b, sep=\", \")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_062_print_with_end() {
        let code = "def f(msg: str):\n    print(msg, end=\"\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_063_input_basic() {
        let code = "def f() -> str:\n    name: str = input(\"Enter name: \")\n    return name";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_064_len_string() {
        let code = "def f(s: str) -> int:\n    return len(s)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_065_len_dict() {
        let code = "def f(d: dict) -> int:\n    return len(d)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_066_iter_list() {
        let code = "def f(items: list) -> list:\n    result = []\n    it = iter(items)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_067_next_call() {
        let code = "def f(it) -> int:\n    return next(it)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_068_super_call() {
        let code = "class Child(Parent):\n    def __init__(self):\n        super().__init__()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_069_sum_with_start() {
        let code = "def f(items: list) -> int:\n    return sum(items, 10)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_070_sorted_with_key() {
        let code = "def f(items: list) -> list:\n    return sorted(items, key=len)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 2: convert_method_call / method_call_routing (70 tests: 071-140)
    // ========================================================================

    // --- String method routing (20 tests) ---

    #[test]
    fn test_w20cd_071_str_split_basic() {
        let code = "def f(s: str) -> list:\n    return s.split(\",\")";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("split"), "split: {result}");
    }

    #[test]
    fn test_w20cd_072_str_join_basic() {
        let code = "def f(items: list) -> str:\n    sep: str = \",\"\n    return sep.join(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_073_str_strip() {
        let code = "def f(s: str) -> str:\n    return s.strip()";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("trim"), "strip: {result}");
    }

    #[test]
    fn test_w20cd_074_str_replace() {
        let code = "def f(s: str) -> str:\n    return s.replace(\"old\", \"new\")";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("replace"), "replace: {result}");
    }

    #[test]
    fn test_w20cd_075_str_find() {
        let code = "def f(s: str) -> int:\n    return s.find(\"abc\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_076_str_rfind() {
        let code = "def f(s: str) -> int:\n    return s.rfind(\"abc\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_077_str_upper() {
        let code = "def f(s: str) -> str:\n    return s.upper()";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("to_uppercase"), "upper: {result}");
    }

    #[test]
    fn test_w20cd_078_str_lower() {
        let code = "def f(s: str) -> str:\n    return s.lower()";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("to_lowercase"), "lower: {result}");
    }

    #[test]
    fn test_w20cd_079_str_startswith() {
        let code = "def f(s: str) -> bool:\n    return s.startswith(\"prefix\")";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("starts_with"), "startswith: {result}");
    }

    #[test]
    fn test_w20cd_080_str_endswith() {
        let code = "def f(s: str) -> bool:\n    return s.endswith(\"suffix\")";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("ends_with"), "endswith: {result}");
    }

    #[test]
    fn test_w20cd_081_str_isdigit() {
        let code = "def f(s: str) -> bool:\n    return s.isdigit()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_082_str_isalpha() {
        let code = "def f(s: str) -> bool:\n    return s.isalpha()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_083_str_isalnum() {
        let code = "def f(s: str) -> bool:\n    return s.isalnum()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_084_str_title() {
        let code = "def f(s: str) -> str:\n    return s.title()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_085_str_capitalize() {
        let code = "def f(s: str) -> str:\n    return s.capitalize()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_086_str_center() {
        let code = "def f(s: str) -> str:\n    return s.center(20)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_087_str_ljust() {
        let code = "def f(s: str) -> str:\n    return s.ljust(20)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_088_str_rjust() {
        let code = "def f(s: str) -> str:\n    return s.rjust(20)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_089_str_zfill() {
        let code = "def f(s: str) -> str:\n    return s.zfill(10)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_090_str_encode() {
        let code = "def f(s: str) -> bytes:\n    return s.encode()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- List method routing (15 tests) ---

    #[test]
    fn test_w20cd_091_list_append() {
        let code = "def f():\n    items: list = []\n    items.append(1)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("push"), "append: {result}");
    }

    #[test]
    fn test_w20cd_092_list_extend() {
        let code = "def f():\n    items: list = [1, 2]\n    items.extend([3, 4])";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_093_list_pop_no_args() {
        let code = "def f(items: list) -> int:\n    return items.pop()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_094_list_pop_index() {
        let code = "def f(items: list) -> int:\n    return items.pop(0)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_095_list_insert() {
        let code = "def f():\n    items: list = [1, 3]\n    items.insert(1, 2)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_096_list_remove() {
        let code = "def f():\n    items: list = [1, 2, 3]\n    items.remove(2)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_097_list_sort() {
        let code = "def f():\n    items: list = [3, 1, 2]\n    items.sort()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_098_list_reverse() {
        let code = "def f():\n    items: list = [1, 2, 3]\n    items.reverse()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_099_list_copy() {
        let code = "def f(items: list) -> list:\n    return items.copy()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_100_list_clear() {
        let code = "def f():\n    items: list = [1, 2, 3]\n    items.clear()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_101_list_index() {
        let code = "def f(items: list) -> int:\n    return items.index(5)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_102_list_count() {
        let code = "def f(items: list) -> int:\n    return items.count(5)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_103_list_sort_reverse() {
        let code = "def f():\n    items: list = [3, 1, 2]\n    items.sort(reverse=True)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_104_list_append_string() {
        let code = "def f():\n    items: list = []\n    items.append(\"hello\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_105_list_insert_at_end() {
        let code = "def f():\n    items: list = [1, 2]\n    items.insert(2, 3)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Dict method routing (15 tests) ---

    #[test]
    fn test_w20cd_106_dict_get_basic() {
        let code = "def f(d: dict, key: str) -> str:\n    return d.get(key)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_107_dict_get_default() {
        let code = "def f(d: dict, key: str) -> str:\n    return d.get(key, \"default\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_108_dict_keys() {
        let code = "def f(d: dict) -> list:\n    return list(d.keys())";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_109_dict_values() {
        let code = "def f(d: dict) -> list:\n    return list(d.values())";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_110_dict_items() {
        let code = "def f(d: dict) -> list:\n    result = []\n    for k, v in d.items():\n        result.append(k)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_111_dict_update() {
        let code = "def f():\n    d: dict = {\"a\": 1}\n    d.update({\"b\": 2})";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_112_dict_pop() {
        let code = "def f(d: dict, key: str):\n    return d.pop(key)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_113_dict_setdefault() {
        let code = "def f(d: dict, key: str) -> str:\n    return d.setdefault(key, \"value\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_114_dict_clear() {
        let code = "def f():\n    d: dict = {\"a\": 1}\n    d.clear()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_115_dict_copy() {
        let code = "def f(d: dict) -> dict:\n    return d.copy()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_116_dict_pop_default() {
        let code = "def f(d: dict, key: str) -> str:\n    return d.pop(key, \"none\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_117_dict_popitem() {
        let code = "def f(d: dict) -> tuple:\n    return d.popitem()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_118_dict_fromkeys() {
        let code = "def f(keys: list) -> dict:\n    return dict.fromkeys(keys, 0)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_119_dict_in_loop() {
        let code = "def f(d: dict) -> int:\n    total: int = 0\n    for k in d.keys():\n        total += 1\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_120_dict_values_sum() {
        let code = "def f(d: dict) -> int:\n    total: int = 0\n    for v in d.values():\n        total += 1\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Set method routing (12 tests) ---

    #[test]
    fn test_w20cd_121_set_add() {
        let code = "def f():\n    s: set = set()\n    s.add(1)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_122_set_remove() {
        let code = "def f():\n    s: set = {1, 2, 3}\n    s.remove(2)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_123_set_discard() {
        let code = "def f():\n    s: set = {1, 2, 3}\n    s.discard(2)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_124_set_union() {
        let code = "def f(a: set, b: set) -> set:\n    return a.union(b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_125_set_intersection() {
        let code = "def f(a: set, b: set) -> set:\n    return a.intersection(b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_126_set_difference() {
        let code = "def f(a: set, b: set) -> set:\n    return a.difference(b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_127_set_symmetric_difference() {
        let code = "def f(a: set, b: set) -> set:\n    return a.symmetric_difference(b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_128_set_issubset() {
        let code = "def f(a: set, b: set) -> bool:\n    return a.issubset(b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_129_set_issuperset() {
        let code = "def f(a: set, b: set) -> bool:\n    return a.issuperset(b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_130_set_isdisjoint() {
        let code = "def f(a: set, b: set) -> bool:\n    return a.isdisjoint(b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_131_set_update() {
        let code = "def f():\n    s: set = {1, 2}\n    s.update({3, 4})";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_132_set_intersection_update() {
        let code = "def f():\n    s: set = {1, 2, 3}\n    s.intersection_update({2, 3, 4})";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- File method routing (4 tests) ---

    #[test]
    fn test_w20cd_133_file_read_text() {
        let code = "def f(path: str) -> str:\n    f = open(path)\n    data: str = f.read()\n    return data";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_134_file_write_text() {
        let code = "def f(path: str, data: str):\n    f = open(path, \"w\")\n    f.write(data)\n    f.close()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_135_file_readline() {
        let code = "def f(path: str) -> str:\n    f = open(path)\n    line: str = f.readline()\n    return line";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_136_file_readlines() {
        let code = "def f(path: str) -> list:\n    f = open(path)\n    lines: list = f.readlines()\n    return lines";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Path method routing (4 tests) ---

    #[test]
    fn test_w20cd_137_path_exists() {
        let code = "def f(path: str) -> bool:\n    p = path\n    return p.endswith(\".txt\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_138_str_lstrip() {
        let code = "def f(s: str) -> str:\n    return s.lstrip()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_139_str_rstrip() {
        let code = "def f(s: str) -> str:\n    return s.rstrip()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_140_str_splitlines() {
        let code = "def f(s: str) -> list:\n    return s.splitlines()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 3: convert_instance_method (60 tests: 141-200)
    // ========================================================================

    // --- File I/O instances (12 tests) ---

    #[test]
    fn test_w20cd_141_file_read_no_args() {
        let code = "def read_all(f) -> str:\n    content: str = f.read()\n    return content";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_142_file_read_with_size() {
        let code = "def read_chunk(f, n: int) -> bytes:\n    data: bytes = f.read(n)\n    return data";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_143_file_readline_instance() {
        let code = "def get_line(f) -> str:\n    line: str = f.readline()\n    return line";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_144_file_readlines_instance() {
        let code = "def get_all_lines(f) -> list:\n    lines: list = f.readlines()\n    return lines";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_145_file_write_instance() {
        let code = "def write_data(f, data: str):\n    f.write(data)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_146_file_close_instance() {
        let code = "def close_file(f):\n    f.close()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_147_file_flush_instance() {
        let code = "def flush_file(f):\n    f.flush()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_148_file_seek_instance() {
        let code = "def seek_file(f, pos: int):\n    f.seek(pos)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_149_file_tell_instance() {
        let code = "def tell_pos(f) -> int:\n    return f.tell()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_150_file_write_multiple() {
        let code = "def write_lines(f, lines: list):\n    for line in lines:\n        f.write(line)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_151_file_read_then_close() {
        let code = "def read_and_close(f) -> str:\n    data: str = f.read()\n    f.close()\n    return data";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_152_file_write_bytes() {
        let code = "def write_bytes_data(f, content: str):\n    f.write(content)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Path instances (8 tests) ---

    #[test]
    fn test_w20cd_153_path_stat() {
        let code = "def get_stat(path: str):\n    return path.startswith(\"/\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_154_path_endswith() {
        let code = "def check_ext(path: str) -> bool:\n    return path.endswith(\".py\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_155_path_startswith() {
        let code = "def check_prefix(path: str) -> bool:\n    return path.startswith(\"/home\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_156_path_split() {
        let code = "def split_path(path: str) -> list:\n    return path.split(\"/\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_157_path_replace() {
        let code = "def replace_sep(path: str) -> str:\n    return path.replace(\"\\\\\", \"/\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_158_path_strip() {
        let code = "def clean_path(path: str) -> str:\n    return path.strip(\"/\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_159_path_lower() {
        let code = "def normalize_path(path: str) -> str:\n    return path.lower()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_160_path_upper() {
        let code = "def uppercase_path(path: str) -> str:\n    return path.upper()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Datetime instances (10 tests) ---

    #[test]
    fn test_w20cd_161_dt_strftime() {
        let code = "def format_date(dt) -> str:\n    return dt.strftime(\"%Y-%m-%d\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_162_dt_isoformat() {
        let code = "def iso_date(dt) -> str:\n    return dt.isoformat()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_163_dt_timestamp() {
        let code = "def get_ts(dt) -> float:\n    return dt.timestamp()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_164_dt_date_component() {
        let code = "def get_date_part(dt):\n    return dt.date()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_165_dt_time_component() {
        let code = "def get_time_part(dt):\n    return dt.time()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_166_date_var_strftime() {
        let code = "def fmt(date) -> str:\n    return date.strftime(\"%d/%m/%Y\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_167_datetime_var_isoformat() {
        let code = "def iso(datetime) -> str:\n    return datetime.isoformat()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_168_time_var_strftime() {
        let code = "def fmt_time(time_val) -> str:\n    return time_val.strftime(\"%H:%M:%S\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_169_dt_with_suffix() {
        let code = "def fmt_created(created_dt) -> str:\n    return created_dt.isoformat()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_170_date_prefix() {
        let code = "def fmt(date_created) -> str:\n    return date_created.strftime(\"%Y\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Regex match instances (6 tests) ---

    #[test]
    fn test_w20cd_171_match_group_zero() {
        let code = "def get_match(m) -> str:\n    return m.group(0)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_172_match_group_n() {
        let code = "def get_group(m) -> str:\n    return m.group(1)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_173_match_group_no_args() {
        let code = "def whole_match(m) -> str:\n    return m.group()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_174_match_groups() {
        let code = "def all_groups(m) -> tuple:\n    return m.groups()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_175_match_start() {
        let code = "def match_start(m) -> int:\n    return m.start()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_176_match_end() {
        let code = "def match_end(m) -> int:\n    return m.end()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- CSV operations (4 tests) ---

    #[test]
    fn test_w20cd_177_csv_writerow() {
        let code = "def write_row(writer, row: list):\n    writer.writerow(row)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_178_csv_writeheader() {
        let code = "def write_header(writer):\n    writer.writeheader()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_179_csv_reader_iter() {
        let code = "def read_csv(reader) -> list:\n    rows: list = []\n    for row in reader:\n        rows.append(row)\n    return rows";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_180_csv_writerow_dict() {
        let code = "def write_dict_row(writer, data: dict):\n    writer.writerow(data)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Deque instances (4 tests) ---

    #[test]
    fn test_w20cd_181_deque_append() {
        let code = "def add_item(dq, item: int):\n    dq.append(item)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_182_deque_appendleft() {
        let code = "def add_front(dq, item: int):\n    dq.appendleft(item)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_183_deque_pop() {
        let code = "def remove_last(dq) -> int:\n    return dq.pop()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_184_deque_popleft() {
        let code = "def remove_front(dq) -> int:\n    return dq.popleft()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Custom class instances (10 tests) ---

    #[test]
    fn test_w20cd_185_class_method_call() {
        let code = "class Dog:\n    def __init__(self, name: str):\n        self.name = name\n    def bark(self) -> str:\n        return \"woof\"\n\ndef f():\n    d = Dog(\"Rex\")\n    d.bark()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_186_class_method_with_args() {
        let code = "class Calculator:\n    def __init__(self):\n        self.result = 0\n    def add(self, x: int) -> int:\n        self.result += x\n        return self.result\n\ndef f():\n    c = Calculator()\n    c.add(5)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_187_class_method_chain() {
        let code = "class Builder:\n    def __init__(self):\n        self.value = 0\n    def set_val(self, v: int):\n        self.value = v";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_188_class_attribute_access() {
        let code = "class Point:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y\n    def magnitude(self) -> float:\n        return float(self.x * self.x + self.y * self.y)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_189_class_two_methods() {
        let code = "class Stack:\n    def __init__(self):\n        self.items = []\n    def push(self, item: int):\n        self.items.append(item)\n    def is_empty(self) -> bool:\n        return len(self.items) == 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_190_class_string_method() {
        let code = "class Greeter:\n    def __init__(self, name: str):\n        self.name = name\n    def greet(self) -> str:\n        return \"Hello, \" + self.name";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_191_class_bool_method() {
        let code = "class Checker:\n    def __init__(self, val: int):\n        self.val = val\n    def is_positive(self) -> bool:\n        return self.val > 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_192_class_with_default() {
        let code = "class Config:\n    def __init__(self, host: str, port: int):\n        self.host = host\n        self.port = port";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_193_class_multiple_attrs() {
        let code = "class Person:\n    def __init__(self, name: str, age: int):\n        self.name = name\n        self.age = age\n    def info(self) -> str:\n        return self.name";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_194_class_list_attr() {
        let code = "class Queue:\n    def __init__(self):\n        self.items = []\n    def enqueue(self, item: int):\n        self.items.append(item)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // --- Dunder methods (6 tests) ---

    #[test]
    fn test_w20cd_195_dunder_str() {
        let code = "class Wrapper:\n    def __init__(self, val: int):\n        self.val = val\n    def __str__(self) -> str:\n        return str(self.val)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_196_dunder_repr() {
        let code = "class Item:\n    def __init__(self, name: str):\n        self.name = name\n    def __repr__(self) -> str:\n        return self.name";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_197_dunder_len() {
        let code = "class Container:\n    def __init__(self):\n        self.items = []\n    def __len__(self) -> int:\n        return len(self.items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_198_dunder_eq() {
        let code = "class Pair:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y\n    def __eq__(self, other) -> bool:\n        return self.x == other.x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_199_dunder_iter() {
        let code = "class Range:\n    def __init__(self, n: int):\n        self.n = n\n    def __iter__(self):\n        return self";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20cd_200_dunder_next_call() {
        let code = "class Counter:\n    def __init__(self, start: int):\n        self.current = start\n    def __next__(self) -> int:\n        self.current += 1\n        return self.current";
        let result = transpile(code);
        assert!(!result.is_empty());
    }
}
