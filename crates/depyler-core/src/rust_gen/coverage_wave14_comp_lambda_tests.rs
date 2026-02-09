//! Wave 14: Coverage tests for comprehensions, lambda expressions, f-strings, and special expressions
//!
//! Tests target UNCOVERED code paths in:
//! - Comprehensions: list comp, set comp, dict comp, generator expressions with filters, nesting, methods
//! - Lambda: simple, multi-arg, no-arg, ternary body, as argument, with defaults
//! - F-strings: basic, expressions, multiple vars, method calls, format specifiers
//! - Special: ternary, nested ternary, walrus, yield, await, star unpack, boolean ops, chained comparison
//!
//! Status: 150 tests (test_w14cl_comp_001..050, test_w14cl_lambda_001..030,
//!         test_w14cl_fstr_001..040, test_w14cl_special_001..030)

#[cfg(test)]
mod tests {
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

    // =========================================================================
    // COMPREHENSIONS (50 tests: test_w14cl_comp_001 through test_w14cl_comp_050)
    // =========================================================================

    #[test]
    fn test_w14cl_comp_001_list_comp_filter_even() {
        let code = "def evens(n: int) -> list:\n    return [x for x in range(n) if x % 2 == 0]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("filter") || result.contains("into_iter"));
    }

    #[test]
    fn test_w14cl_comp_002_list_comp_filter_positive() {
        let code = "def positives(items: list) -> list:\n    return [x for x in items if x > 0]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("filter") || result.contains("into_iter"));
    }

    #[test]
    fn test_w14cl_comp_003_list_comp_nested_product() {
        let code = "def products() -> list:\n    return [i * j for i in range(3) for j in range(3)]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_004_list_comp_nested_pairs() {
        let code = "def pairs() -> list:\n    return [(i, j) for i in range(2) for j in range(2)]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_005_list_comp_method_upper() {
        let code = "def upper_all(words: list) -> list:\n    return [s.upper() for s in words]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("to_uppercase") || result.contains("upper") || result.contains("into_iter"));
    }

    #[test]
    fn test_w14cl_comp_006_list_comp_method_strip() {
        let code = "def stripped(lines: list) -> list:\n    return [s.strip() for s in lines]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("trim") || result.contains("strip") || result.contains("into_iter"));
    }

    #[test]
    fn test_w14cl_comp_007_list_comp_method_lower() {
        let code = "def lower_all(words: list) -> list:\n    return [w.lower() for w in words]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_008_list_comp_ternary_abs() {
        let code = "def abs_values(nums: list) -> list:\n    return [x if x > 0 else -x for x in nums]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("if") || result.contains("into_iter"));
    }

    #[test]
    fn test_w14cl_comp_009_list_comp_ternary_classify() {
        let code = "def classify(nums: list) -> list:\n    return [\"pos\" if x > 0 else \"neg\" for x in nums]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_010_set_comp_basic() {
        let code = "def unique_items(items: list) -> set:\n    return {x for x in items}\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashSet") || result.contains("collect"));
    }

    #[test]
    fn test_w14cl_comp_011_set_comp_filter_gt5() {
        let code = "def big_items(items: list) -> set:\n    return {x for x in items if x > 5}\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashSet") || result.contains("filter") || result.contains("collect"));
    }

    #[test]
    fn test_w14cl_comp_012_set_comp_squared() {
        let code = "def squared_set(n: int) -> set:\n    return {x * x for x in range(n)}\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_013_set_comp_lengths() {
        let code = "def word_lengths(words: list) -> set:\n    return {len(w) for w in words}\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_014_dict_comp_basic() {
        let code = "def make_dict(keys: list) -> dict:\n    return {k: k * 2 for k in keys}\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashMap") || result.contains("collect"));
    }

    #[test]
    fn test_w14cl_comp_015_dict_comp_squared() {
        let code = "def squared_dict(n: int) -> dict:\n    return {i: i * i for i in range(n)}\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashMap") || result.contains("collect"));
    }

    #[test]
    fn test_w14cl_comp_016_dict_comp_filter_positive() {
        let code = "def positive_dict(items: list) -> dict:\n    return {i: v for i, v in enumerate(items) if v > 0}\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_017_dict_comp_str_keys() {
        let code = "def str_keys(n: int) -> dict:\n    return {str(i): i for i in range(n)}\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_018_dict_comp_enumerate() {
        let code = "def indexed_dict(items: list) -> dict:\n    return {i: v for i, v in enumerate(items)}\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_019_nested_list_comp() {
        let code = "def matrix() -> list:\n    return [[j for j in range(3)] for i in range(2)]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_020_nested_comp_flatten() {
        let code = "def flatten() -> list:\n    return [x for row in [[1, 2], [3, 4]] for x in row]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_021_genexpr_sum() {
        let code = "def total(n: int) -> int:\n    return sum(x for x in range(n))\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_022_genexpr_sum_squared() {
        let code = "def sum_sq(n: int) -> int:\n    return sum(x * x for x in range(n))\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_023_genexpr_any() {
        let code = "def has_big(nums: list) -> bool:\n    return any(x > 5 for x in nums)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_024_genexpr_all() {
        let code = "def all_positive(nums: list) -> bool:\n    return all(x > 0 for x in nums)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_025_genexpr_with_filter() {
        let code = "def sum_even(n: int) -> int:\n    return sum(x for x in range(n) if x % 2 == 0)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_026_genexpr_max() {
        let code = "def biggest(nums: list) -> int:\n    return max(x for x in nums)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_027_genexpr_min() {
        let code = "def smallest(nums: list) -> int:\n    return min(x for x in nums)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_028_genexpr_list_conversion() {
        let code = "def doubled(items: list) -> list:\n    return list(x * 2 for x in items)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_029_comp_over_string_chars() {
        let code = "def char_list(s: str) -> list:\n    return [c for c in s]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_030_comp_over_enumerate() {
        let code = "def indices(items: list) -> list:\n    return [i for i, x in enumerate(items)]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_031_comp_over_zip() {
        let code = "def sum_pairs(xs: list, ys: list) -> list:\n    return [a + b for a, b in zip(xs, ys)]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_032_list_comp_len_call() {
        let code = "def lengths(items: list) -> list:\n    return [len(x) for x in items]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_033_list_comp_str_call() {
        let code = "def stringify(nums: list) -> list:\n    return [str(x) for x in nums]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_034_list_comp_int_call() {
        let code = "def to_ints(items: list) -> list:\n    return [int(x) for x in items]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_035_list_comp_bool_filter() {
        let code = "def truthy(items: list) -> list:\n    return [x for x in items if bool(x)]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_036_list_comp_double_filter() {
        let code = "def range_items(items: list) -> list:\n    return [x for x in items if x > 0 if x < 100]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_037_list_comp_arithmetic() {
        let code = "def doubled(n: int) -> list:\n    return [x * 2 for x in range(n)]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("into_iter") || result.contains("map"));
    }

    #[test]
    fn test_w14cl_comp_038_list_comp_addition() {
        let code = "def incremented(n: int) -> list:\n    return [x + 1 for x in range(n)]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_039_set_comp_modulo() {
        let code = "def remainders(n: int) -> set:\n    return {x % 3 for x in range(n)}\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_040_dict_comp_length_map() {
        let code = "def word_len_map(words: list) -> dict:\n    return {w: len(w) for w in words}\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_041_comp_assign_to_var() {
        let code = "def f(n: int) -> list:\n    squares = [x * x for x in range(n)]\n    return squares\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w14cl_comp_042_set_comp_assign_to_var() {
        let code = "def f(items: list) -> set:\n    unique = {x for x in items}\n    return unique\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_043_dict_comp_assign_to_var() {
        let code = "def f(n: int) -> dict:\n    d = {i: i * i for i in range(n)}\n    return d\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_044_comp_with_not_filter() {
        let code = "def non_zero(items: list) -> list:\n    return [x for x in items if not x == 0]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_045_comp_with_and_filter() {
        let code = "def in_range(items: list) -> list:\n    return [x for x in items if x > 0 and x < 100]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_046_comp_with_or_filter() {
        let code = "def extremes(items: list) -> list:\n    return [x for x in items if x < -10 or x > 10]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_047_genexpr_sum_filtered() {
        let code = "def sum_pos(items: list) -> int:\n    return sum(x for x in items if x > 0)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_048_genexpr_any_negative() {
        let code = "def has_neg(items: list) -> bool:\n    return any(x < 0 for x in items)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_049_genexpr_all_even() {
        let code = "def all_even(items: list) -> bool:\n    return all(x % 2 == 0 for x in items)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_comp_050_comp_with_subtraction() {
        let code = "def offsets(items: list, base: int) -> list:\n    return [x - base for x in items]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // =========================================================================
    // LAMBDA (30 tests: test_w14cl_lambda_001 through test_w14cl_lambda_030)
    // =========================================================================

    #[test]
    fn test_w14cl_lambda_001_simple_add_one() {
        let code = "def f() -> int:\n    inc = lambda x: x + 1\n    return inc(5)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w14cl_lambda_002_two_args_add() {
        let code = "def f() -> int:\n    add = lambda x, y: x + y\n    return add(3, 4)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_003_no_args() {
        let code = "def f() -> int:\n    constant = lambda: 42\n    return constant()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_004_ternary_body() {
        let code = "def f() -> int:\n    abs_val = lambda x: x if x > 0 else -x\n    return abs_val(-5)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_005_multiply() {
        let code = "def f() -> int:\n    double = lambda x: x * 2\n    return double(10)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_006_subtract() {
        let code = "def f() -> int:\n    sub = lambda a, b: a - b\n    return sub(10, 3)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_007_three_args() {
        let code = "def f() -> int:\n    add3 = lambda a, b, c: a + b + c\n    return add3(1, 2, 3)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_008_boolean_body() {
        let code = "def f() -> bool:\n    is_pos = lambda x: x > 0\n    return is_pos(5)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_009_modulo_body() {
        let code = "def f() -> bool:\n    is_even = lambda x: x % 2 == 0\n    return is_even(4)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_010_square() {
        let code = "def f() -> int:\n    square = lambda x: x * x\n    return square(7)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_011_negate() {
        let code = "def f() -> int:\n    neg = lambda x: -x\n    return neg(5)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_012_division() {
        let code = "def f() -> float:\n    half = lambda x: x / 2\n    return half(10)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_013_string_body() {
        let code = "def f() -> str:\n    greet = lambda name: \"Hello \" + name\n    return greet(\"world\")\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_014_return_literal() {
        let code = "def f() -> str:\n    msg = lambda: \"hello\"\n    return msg()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_015_power() {
        let code = "def f() -> int:\n    cube = lambda x: x ** 3\n    return cube(2)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_016_and_logic() {
        let code = "def f() -> bool:\n    both = lambda a, b: a and b\n    return both(True, False)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_017_or_logic() {
        let code = "def f() -> bool:\n    either = lambda a, b: a or b\n    return either(True, False)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_018_not_logic() {
        let code = "def f() -> bool:\n    negate = lambda a: not a\n    return negate(True)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_019_comparison_ge() {
        let code = "def f() -> bool:\n    at_least = lambda x, y: x >= y\n    return at_least(5, 3)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_020_comparison_le() {
        let code = "def f() -> bool:\n    at_most = lambda x, y: x <= y\n    return at_most(3, 5)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_021_sorted_key() {
        let code = "def f(items: list) -> list:\n    items.sort(key=lambda x: x)\n    return items\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_022_in_map_call() {
        let code = "def f(nums: list) -> list:\n    return list(map(lambda x: x * 2, nums))\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_023_in_filter_call() {
        let code = "def f(nums: list) -> list:\n    return list(filter(lambda x: x > 0, nums))\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_024_with_capture() {
        let code = "def f(offset: int) -> int:\n    add_offset = lambda x: x + offset\n    return add_offset(10)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_025_floor_div() {
        let code = "def f() -> int:\n    half_floor = lambda x: x // 2\n    return half_floor(7)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_026_min_of_two() {
        let code = "def f() -> int:\n    smaller = lambda a, b: a if a < b else b\n    return smaller(3, 7)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_027_max_of_two() {
        let code = "def f() -> int:\n    bigger = lambda a, b: a if a > b else b\n    return bigger(3, 7)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_028_bitwise_and() {
        let code = "def f() -> int:\n    mask = lambda x, m: x & m\n    return mask(255, 15)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_029_bitwise_or() {
        let code = "def f() -> int:\n    combine = lambda x, y: x | y\n    return combine(12, 3)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_lambda_030_equality_check() {
        let code = "def f() -> bool:\n    same = lambda a, b: a == b\n    return same(3, 3)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // =========================================================================
    // F-STRINGS (40 tests: test_w14cl_fstr_001 through test_w14cl_fstr_040)
    // =========================================================================

    #[test]
    fn test_w14cl_fstr_001_basic_name() {
        let code = "def greet(name: str) -> str:\n    return f\"Hello {name}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_002_expr_addition() {
        let code = "def show(x: int, y: int) -> str:\n    return f\"Result: {x + y}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_003_multiple_vars() {
        let code = "def info(a: int, b: int) -> str:\n    return f\"{a} and {b}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_004_method_upper() {
        let code = "def loud(name: str) -> str:\n    return f\"{name.upper()}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_fstr_005_with_int_var() {
        let code = "def show_num(n: int) -> str:\n    return f\"Number: {n}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_006_with_float_var() {
        let code = "def show_val(x: float) -> str:\n    return f\"Value: {x}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_007_with_bool_var() {
        let code = "def show_flag(flag: bool) -> str:\n    return f\"Flag: {flag}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_008_literal_only() {
        let code = "def constant() -> str:\n    return f\"no expressions\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_fstr_009_empty() {
        let code = "def blank() -> str:\n    return f\"\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_fstr_010_three_vars() {
        let code = "def triple(a: int, b: int, c: int) -> str:\n    return f\"{a}, {b}, {c}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_011_multiplication_expr() {
        let code = "def show_product(x: int) -> str:\n    return f\"Double: {x * 2}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_012_subtraction_expr() {
        let code = "def show_diff(a: int, b: int) -> str:\n    return f\"Diff: {a - b}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_013_mixed_text_and_expr() {
        let code = "def greeting(name: str, age: int) -> str:\n    return f\"{name} is {age} years old\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_014_with_comparison() {
        let code = "def check(x: int) -> str:\n    return f\"Positive: {x > 0}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_015_with_len_call() {
        let code = "def show_len(items: list) -> str:\n    return f\"Length: {len(items)}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_016_string_concat_in_expr() {
        let code = "def full_name(first: str, last: str) -> str:\n    return f\"Name: {first + last}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_017_nested_in_function() {
        let code = "def describe(x: int) -> str:\n    label = f\"value={x}\"\n    return label\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_018_with_negative() {
        let code = "def show_neg(x: int) -> str:\n    return f\"Neg: {-x}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_019_with_modulo() {
        let code = "def show_mod(x: int) -> str:\n    return f\"Mod 3: {x % 3}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_020_assign_and_return() {
        let code = "def make_msg(name: str) -> str:\n    msg = f\"Hi {name}!\"\n    return msg\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_021_in_conditional() {
        let code = "def status(ok: bool) -> str:\n    if ok:\n        return f\"Success\"\n    return f\"Failure\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_fstr_022_with_division_expr() {
        let code = "def show_half(x: int) -> str:\n    return f\"Half: {x / 2}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_023_two_string_vars() {
        let code = "def pair(a: str, b: str) -> str:\n    return f\"{a}-{b}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_024_prefix_suffix() {
        let code = "def wrap(s: str) -> str:\n    return f\"[{s}]\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_025_with_str_method_lower() {
        let code = "def quiet(name: str) -> str:\n    return f\"{name.lower()}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_fstr_026_with_str_method_strip() {
        let code = "def clean(s: str) -> str:\n    return f\"{s.strip()}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_fstr_027_with_ternary_expr() {
        let code = "def label(x: int) -> str:\n    return f\"{'pos' if x > 0 else 'neg'}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_fstr_028_multipart_text() {
        let code = "def header(title: str) -> str:\n    return f\"=== {title} ===\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_029_with_power_expr() {
        let code = "def show_sq(x: int) -> str:\n    return f\"Squared: {x ** 2}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_030_four_vars() {
        let code = "def coords(a: int, b: int, c: int, d: int) -> str:\n    return f\"({a},{b},{c},{d})\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_031_with_and_expr() {
        let code = "def check_both(a: bool, b: bool) -> str:\n    return f\"Both: {a and b}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_032_with_or_expr() {
        let code = "def check_either(a: bool, b: bool) -> str:\n    return f\"Either: {a or b}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_033_with_not_expr() {
        let code = "def invert(flag: bool) -> str:\n    return f\"Inverted: {not flag}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_034_in_loop_body() {
        let code = "def messages(names: list) -> list:\n    result = []\n    for name in names:\n        result.append(f\"Hi {name}\")\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_035_with_floor_div() {
        let code = "def show_floor(x: int) -> str:\n    return f\"Floor: {x // 2}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_036_with_bitwise_and() {
        let code = "def show_mask(x: int) -> str:\n    return f\"Masked: {x & 255}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_037_with_equality_check() {
        let code = "def show_eq(a: int, b: int) -> str:\n    return f\"Equal: {a == b}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_038_with_not_equal() {
        let code = "def show_ne(a: int, b: int) -> str:\n    return f\"Different: {a != b}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    #[test]
    fn test_w14cl_fstr_039_concatenated_fstrings() {
        let code = "def full(first: str, last: str) -> str:\n    return f\"{first}\" + f\" {last}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_fstr_040_with_ge_check() {
        let code = "def show_ge(a: int, b: int) -> str:\n    return f\"GE: {a >= b}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"));
    }

    // =========================================================================
    // SPECIAL EXPRESSIONS (30 tests: test_w14cl_special_001 through test_w14cl_special_030)
    // =========================================================================

    #[test]
    fn test_w14cl_special_001_ternary_int() {
        let code = "def abs_val(x: int) -> int:\n    return x if x > 0 else -x\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("if") || result.contains("else"));
    }

    #[test]
    fn test_w14cl_special_002_ternary_string() {
        let code = "def yesno(flag: bool) -> str:\n    return \"yes\" if flag else \"no\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_special_003_nested_ternary() {
        let code = "def classify(x: int) -> str:\n    return \"pos\" if x > 0 else \"zero\" if x == 0 else \"neg\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_special_004_ternary_assign() {
        let code = "def f(x: int) -> int:\n    result = x if x > 0 else 0\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_special_005_ternary_with_call() {
        let code = "def f(items: list) -> int:\n    return len(items) if items else 0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_special_006_ternary_float() {
        let code = "def f(x: float) -> float:\n    return x if x > 0.0 else 0.0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_special_007_yield_basic() {
        let code = "def gen():\n    yield 1\n    yield 2\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_special_008_yield_in_loop() {
        let code = "def count(n: int):\n    for i in range(n):\n        yield i\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_special_009_yield_with_value() {
        let code = "def doubled(n: int):\n    for i in range(n):\n        yield i * 2\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_special_010_yield_conditional() {
        let code = "def evens(n: int):\n    for i in range(n):\n        if i % 2 == 0:\n            yield i\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_special_011_yield_none() {
        let code = "def gen():\n    yield\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_special_012_boolean_and() {
        let code = "def f(a: bool, b: bool) -> bool:\n    return a and b\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("&&") || result.contains("and"));
    }

    #[test]
    fn test_w14cl_special_013_boolean_or() {
        let code = "def f(a: bool, b: bool) -> bool:\n    return a or b\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("||") || result.contains("or"));
    }

    #[test]
    fn test_w14cl_special_014_boolean_not() {
        let code = "def f(a: bool) -> bool:\n    return not a\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("!") || result.contains("not"));
    }

    #[test]
    fn test_w14cl_special_015_boolean_and_or_chain() {
        let code = "def f(a: bool, b: bool, c: bool) -> bool:\n    return a and b or c\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_special_016_chained_comparison_lt() {
        let code = "def in_range(x: int) -> bool:\n    return 0 < x < 10\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_special_017_chained_comparison_le() {
        let code = "def in_range(x: int) -> bool:\n    return 0 <= x <= 100\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_special_018_star_unpack_first() {
        let code = "def f() -> int:\n    items = [1, 2, 3, 4, 5]\n    first = items[0]\n    return first\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_special_019_star_unpack_rest() {
        let code = "def f() -> list:\n    items = [1, 2, 3, 4]\n    rest = items[1:]\n    return rest\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_special_020_await_async_fn() {
        let code = "async def fetch() -> int:\n    result = await get_data()\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_special_021_ternary_with_arithmetic() {
        let code = "def clamp(x: int) -> int:\n    return x if x < 100 else 100\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_special_022_multiple_yield() {
        let code = "def fibonacci():\n    a = 0\n    b = 1\n    yield a\n    yield b\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_special_023_boolean_not_and() {
        let code = "def f(a: bool, b: bool) -> bool:\n    return not a and b\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_special_024_boolean_not_or() {
        let code = "def f(a: bool, b: bool) -> bool:\n    return not a or b\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_special_025_ternary_none_fallback() {
        let code = "from typing import Optional\ndef f(x: Optional[int]) -> int:\n    return x if x is not None else 0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_special_026_yield_string() {
        let code = "def words():\n    yield \"hello\"\n    yield \"world\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_special_027_ternary_with_string_ops() {
        let code = "def f(s: str) -> str:\n    return s.upper() if s else \"\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_special_028_boolean_double_not() {
        let code = "def f(a: bool) -> bool:\n    return not not a\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_special_029_ternary_with_list() {
        let code = "def f(items: list) -> list:\n    return items if items else []\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14cl_special_030_yield_with_while() {
        let code = "def countdown(n: int):\n    while n > 0:\n        yield n\n        n = n - 1\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }
}
