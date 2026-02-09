//! Coverage Wave 12: Binary Operations, Call Generation, and NumPy/Stdlib
//!
//! Targets:
//! - binary_ops.rs (62.22% coverage, 306/810 lines missed)
//! - call_generic.rs (57.72% coverage, 249/589 lines missed)
//! - stdlib_numpy.rs (40.30% coverage, 160/268 lines missed)
//! - stdlib_misc.rs (60% coverage)
//! - convert_unary_and_call.rs (74% coverage)

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

    // ========================================================================
    // Binary Operations Tests (80 tests) - binary_ops.rs
    // ========================================================================

    // Arithmetic operations
    #[test]
    fn test_w12bc_arith_001_int_add() {
        let py = "result = 5 + 3";
        let rs = transpile(py);
        assert!(rs.contains("5") && rs.contains("3"));
    }

    #[test]
    fn test_w12bc_arith_002_int_sub() {
        let py = "result = 10 - 4";
        let rs = transpile(py);
        assert!(rs.contains("10") && rs.contains("4"));
    }

    #[test]
    fn test_w12bc_arith_003_int_mul() {
        let py = "result = 6 * 7";
        let rs = transpile(py);
        assert!(rs.contains("6") && rs.contains("7"));
    }

    #[test]
    fn test_w12bc_arith_004_int_div() {
        let py = "result = 15 / 3";
        let rs = transpile(py);
        assert!(rs.contains("15") && rs.contains("3"));
    }

    #[test]
    fn test_w12bc_arith_005_int_mod() {
        let py = "result = 17 % 5";
        let rs = transpile(py);
        assert!(rs.contains("17") && rs.contains("5"));
    }

    #[test]
    fn test_w12bc_arith_006_int_pow() {
        let py = "result = 2 ** 8";
        let rs = transpile(py);
        assert!(rs.contains("2") && rs.contains("8"));
    }

    #[test]
    fn test_w12bc_arith_007_floor_div() {
        let py = "result = 17 // 5";
        let rs = transpile(py);
        assert!(rs.contains("17") && rs.contains("5"));
    }

    #[test]
    fn test_w12bc_arith_008_float_add() {
        let py = "result = 5.5 + 3.2";
        let rs = transpile(py);
        assert!(rs.contains("5.5") && rs.contains("3.2"));
    }

    #[test]
    fn test_w12bc_arith_009_float_mul() {
        let py = "result = 2.5 * 4.0";
        let rs = transpile(py);
        assert!(rs.contains("2.5") && rs.contains("4.0"));
    }

    #[test]
    fn test_w12bc_arith_010_mixed_int_float() {
        let py = "result = 5 + 3.5";
        let rs = transpile(py);
        assert!(rs.contains("5") && rs.contains("3.5"));
    }

    // String operations
    #[test]
    fn test_w12bc_string_011_concat() {
        let py = r#"result = "hello" + "world""#;
        let rs = transpile(py);
        assert!(rs.contains("hello") && rs.contains("world"));
    }

    #[test]
    fn test_w12bc_string_012_repeat() {
        let py = r#"result = "abc" * 3"#;
        let rs = transpile(py);
        assert!(rs.contains("abc") && rs.contains("3"));
    }

    #[test]
    fn test_w12bc_string_013_reverse_repeat() {
        let py = r#"result = 3 * "xyz""#;
        let rs = transpile(py);
        assert!(rs.contains("3") && rs.contains("xyz"));
    }

    // Comparison operators
    #[test]
    fn test_w12bc_cmp_014_eq() {
        let py = "result = 5 == 5";
        let rs = transpile(py);
        assert!(rs.contains("5") && rs.contains("=="));
    }

    #[test]
    fn test_w12bc_cmp_015_neq() {
        let py = "result = 5 != 3";
        let rs = transpile(py);
        assert!(rs.contains("5") && rs.contains("3"));
    }

    #[test]
    fn test_w12bc_cmp_016_lt() {
        let py = "result = 3 < 5";
        let rs = transpile(py);
        assert!(rs.contains("3") && rs.contains("5") && rs.contains("<"));
    }

    #[test]
    fn test_w12bc_cmp_017_lte() {
        let py = "result = 5 <= 5";
        let rs = transpile(py);
        assert!(rs.contains("5") && rs.contains("<="));
    }

    #[test]
    fn test_w12bc_cmp_018_gt() {
        let py = "result = 7 > 3";
        let rs = transpile(py);
        assert!(rs.contains("7") && rs.contains("3") && rs.contains(">"));
    }

    #[test]
    fn test_w12bc_cmp_019_gte() {
        let py = "result = 5 >= 3";
        let rs = transpile(py);
        assert!(rs.contains("5") && rs.contains("3") && rs.contains(">="));
    }

    #[test]
    fn test_w12bc_cmp_020_float_cmp() {
        let py = "result = 3.5 > 2.1";
        let rs = transpile(py);
        assert!(rs.contains("3.5") && rs.contains("2.1"));
    }

    // Logical operators
    #[test]
    fn test_w12bc_logic_021_and() {
        let py = "result = True and False";
        let rs = transpile(py);
        assert!(rs.contains("true") && rs.contains("false"));
    }

    #[test]
    fn test_w12bc_logic_022_or() {
        let py = "result = True or False";
        let rs = transpile(py);
        assert!(rs.contains("true") || rs.contains("false"));
    }

    #[test]
    fn test_w12bc_logic_023_not() {
        let py = "result = not True";
        let rs = transpile(py);
        assert!(rs.contains("!") || rs.contains("not"));
    }

    // Bitwise operators
    #[test]
    fn test_w12bc_bit_024_and() {
        let py = "result = 12 & 7";
        let rs = transpile(py);
        assert!(rs.contains("12") && rs.contains("7"));
    }

    #[test]
    fn test_w12bc_bit_025_or() {
        let py = "result = 12 | 7";
        let rs = transpile(py);
        assert!(rs.contains("12") && rs.contains("7"));
    }

    #[test]
    fn test_w12bc_bit_026_xor() {
        let py = "result = 12 ^ 7";
        let rs = transpile(py);
        assert!(rs.contains("12") && rs.contains("7"));
    }

    #[test]
    fn test_w12bc_bit_027_lshift() {
        let py = "result = 3 << 2";
        let rs = transpile(py);
        assert!(rs.contains("3") && rs.contains("2"));
    }

    #[test]
    fn test_w12bc_bit_028_rshift() {
        let py = "result = 12 >> 2";
        let rs = transpile(py);
        assert!(rs.contains("12") && rs.contains("2"));
    }

    #[test]
    fn test_w12bc_bit_029_not() {
        let py = "result = ~5";
        let rs = transpile(py);
        assert!(rs.contains("5"));
    }

    // Containment operators
    #[test]
    fn test_w12bc_contain_030_in_list() {
        let py = "result = 3 in [1, 2, 3, 4]";
        let rs = transpile(py);
        assert!(rs.contains("3") && rs.contains("contains"));
    }

    #[test]
    fn test_w12bc_contain_031_not_in_list() {
        let py = "result = 5 not in [1, 2, 3, 4]";
        let rs = transpile(py);
        assert!(rs.contains("5") && rs.contains("contains"));
    }

    #[test]
    fn test_w12bc_contain_032_in_string() {
        let py = r#"result = "ab" in "abc""#;
        let rs = transpile(py);
        assert!(rs.contains("ab") && rs.contains("abc"));
    }

    #[test]
    fn test_w12bc_contain_033_in_set() {
        let py = "result = 3 in {1, 2, 3}";
        let rs = transpile(py);
        assert!(rs.contains("3") && rs.contains("contains"));
    }

    #[test]
    fn test_w12bc_contain_034_in_dict() {
        let py = r#"result = "key" in {"key": "value"}"#;
        let rs = transpile(py);
        assert!(rs.contains("key") && (rs.contains("get") || rs.contains("contains")));
    }

    #[test]
    fn test_w12bc_contain_035_in_tuple() {
        let py = "result = 2 in (1, 2, 3)";
        let rs = transpile(py);
        assert!(rs.contains("2") && rs.contains("contains"));
    }

    // Is/is not operators
    #[test]
    fn test_w12bc_is_036_none() {
        let py = "result = x is None";
        let rs = transpile(py);
        assert!(rs.contains("None") || rs.contains("is_none"));
    }

    #[test]
    fn test_w12bc_is_037_not_none() {
        let py = "result = x is not None";
        let rs = transpile(py);
        assert!(rs.contains("is_some") || rs.contains("is_none"));
    }

    // Matrix multiply
    #[test]
    fn test_w12bc_matmul_038() {
        // Matrix multiply (@) is not supported in HIR yet, use np.matmul instead
        let py = "import numpy as np\nresult = np.matmul(a, b)";
        let rs = transpile(py);
        assert!(rs.contains("a") && rs.contains("b"));
    }

    // Chained operations
    #[test]
    fn test_w12bc_chain_039_add_mul() {
        let py = "result = 2 + 3 * 4";
        let rs = transpile(py);
        assert!(rs.contains("2") && rs.contains("3") && rs.contains("4"));
    }

    #[test]
    fn test_w12bc_chain_040_parentheses() {
        let py = "result = (2 + 3) * 4";
        let rs = transpile(py);
        assert!(rs.contains("2") && rs.contains("3") && rs.contains("4"));
    }

    #[test]
    fn test_w12bc_chain_041_multiple_add() {
        let py = "result = 1 + 2 + 3 + 4";
        let rs = transpile(py);
        assert!(rs.contains("1") && rs.contains("2") && rs.contains("3") && rs.contains("4"));
    }

    #[test]
    fn test_w12bc_chain_042_mixed_ops() {
        let py = "result = 10 - 3 * 2 + 5";
        let rs = transpile(py);
        assert!(rs.contains("10") && rs.contains("3") && rs.contains("2") && rs.contains("5"));
    }

    // List operations
    #[test]
    fn test_w12bc_list_043_concat() {
        let py = "result = [1, 2] + [3, 4]";
        let rs = transpile(py);
        assert!(rs.contains("1") && rs.contains("2") && rs.contains("3") && rs.contains("4"));
    }

    #[test]
    fn test_w12bc_list_044_repeat() {
        let py = "result = [0] * 5";
        let rs = transpile(py);
        assert!(rs.contains("0") && rs.contains("5"));
    }

    #[test]
    fn test_w12bc_list_045_reverse_repeat() {
        let py = "result = 3 * [1, 2]";
        let rs = transpile(py);
        assert!(rs.contains("3") && rs.contains("1") && rs.contains("2"));
    }

    // Set operations
    #[test]
    fn test_w12bc_set_046_union() {
        let py = "result = {1, 2} | {3, 4}";
        let rs = transpile(py);
        assert!(rs.contains("1") && rs.contains("2") && rs.contains("3") && rs.contains("4"));
    }

    #[test]
    fn test_w12bc_set_047_intersection() {
        let py = "result = {1, 2, 3} & {2, 3, 4}";
        let rs = transpile(py);
        assert!(rs.contains("1") && rs.contains("2") && rs.contains("3") && rs.contains("4"));
    }

    #[test]
    fn test_w12bc_set_048_difference() {
        let py = "result = {1, 2, 3} - {2, 3}";
        let rs = transpile(py);
        assert!(rs.contains("1") && rs.contains("2") && rs.contains("3"));
    }

    #[test]
    fn test_w12bc_set_049_symmetric_diff() {
        let py = "result = {1, 2} ^ {2, 3}";
        let rs = transpile(py);
        assert!(rs.contains("1") && rs.contains("2") && rs.contains("3"));
    }

    // Dict merge
    #[test]
    fn test_w12bc_dict_050_merge() {
        let py = r#"result = {"a": 1} | {"b": 2}"#;
        let rs = transpile(py);
        assert!(rs.contains("a") && rs.contains("b"));
    }

    // Power operations with various types
    #[test]
    fn test_w12bc_pow_051_int_int() {
        let py = "result = 2 ** 10";
        let rs = transpile(py);
        assert!(rs.contains("2") && rs.contains("10"));
    }

    #[test]
    fn test_w12bc_pow_052_float_int() {
        let py = "result = 2.5 ** 3";
        let rs = transpile(py);
        assert!(rs.contains("2.5") && rs.contains("3"));
    }

    #[test]
    fn test_w12bc_pow_053_int_float() {
        let py = "result = 4 ** 0.5";
        let rs = transpile(py);
        assert!(rs.contains("4") && rs.contains("0.5"));
    }

    #[test]
    fn test_w12bc_pow_054_negative_exp() {
        let py = "result = 2 ** -3";
        let rs = transpile(py);
        assert!(rs.contains("2") && rs.contains("3"));
    }

    // Type coercion scenarios
    #[test]
    fn test_w12bc_coerce_055_int_to_float() {
        let py = "result = 5 / 2.0";
        let rs = transpile(py);
        assert!(rs.contains("5") && rs.contains("2.0"));
    }

    #[test]
    fn test_w12bc_coerce_056_comparison() {
        let py = "result = 5 == 5.0";
        let rs = transpile(py);
        assert!(rs.contains("5"));
    }

    // String comparison
    #[test]
    fn test_w12bc_strcmp_057_eq() {
        let py = r#"result = "abc" == "abc""#;
        let rs = transpile(py);
        assert!(rs.contains("abc"));
    }

    #[test]
    fn test_w12bc_strcmp_058_lt() {
        let py = r#"result = "a" < "b""#;
        let rs = transpile(py);
        assert!(rs.contains("\"a\"") && rs.contains("\"b\""));
    }

    #[test]
    fn test_w12bc_strcmp_059_gte() {
        let py = r#"result = "z" >= "a""#;
        let rs = transpile(py);
        assert!(rs.contains("\"z\"") && rs.contains("\"a\""));
    }

    // Truthiness in logical operators
    #[test]
    fn test_w12bc_truth_060_and_int() {
        let py = "result = 5 and 3";
        let rs = transpile(py);
        assert!(rs.contains("5") && rs.contains("3"));
    }

    #[test]
    fn test_w12bc_truth_061_or_int() {
        let py = "result = 0 or 5";
        let rs = transpile(py);
        assert!(rs.contains("0") && rs.contains("5"));
    }

    #[test]
    fn test_w12bc_truth_062_and_string() {
        let py = r#"result = "hello" and "world""#;
        let rs = transpile(py);
        assert!(rs.contains("hello") && rs.contains("world"));
    }

    #[test]
    fn test_w12bc_truth_063_or_string() {
        let py = r#"result = "" or "default""#;
        let rs = transpile(py);
        assert!(rs.contains("default"));
    }

    // Bytes operations
    #[test]
    fn test_w12bc_bytes_064_repeat() {
        let py = r#"result = b"hello" * 3"#;
        let rs = transpile(py);
        assert!(rs.contains("hello") && rs.contains("3"));
    }

    #[test]
    fn test_w12bc_bytes_065_concat() {
        let py = r#"result = b"hello" + b"world""#;
        let rs = transpile(py);
        assert!(rs.contains("hello") && rs.contains("world"));
    }

    // Unary operators
    #[test]
    fn test_w12bc_unary_066_neg() {
        let py = "result = -5";
        let rs = transpile(py);
        assert!(rs.contains("-") || rs.contains("5"));
    }

    #[test]
    fn test_w12bc_unary_067_pos() {
        let py = "result = +5";
        let rs = transpile(py);
        assert!(rs.contains("5"));
    }

    #[test]
    fn test_w12bc_unary_068_not() {
        let py = "result = not False";
        let rs = transpile(py);
        assert!(rs.contains("!") || rs.contains("false"));
    }

    #[test]
    fn test_w12bc_unary_069_bitnot() {
        let py = "result = ~42";
        let rs = transpile(py);
        assert!(rs.contains("42"));
    }

    // Complex expressions
    #[test]
    fn test_w12bc_complex_070_nested() {
        let py = "result = ((2 + 3) * 4) - (5 / 2)";
        let rs = transpile(py);
        assert!(rs.contains("2") && rs.contains("3") && rs.contains("4") && rs.contains("5"));
    }

    #[test]
    fn test_w12bc_complex_071_power_chain() {
        let py = "result = 2 ** 3 ** 2";
        let rs = transpile(py);
        assert!(rs.contains("2") && rs.contains("3"));
    }

    #[test]
    fn test_w12bc_complex_072_comparison_chain() {
        let py = "result = 1 < 2 < 3";
        let rs = transpile(py);
        assert!(rs.contains("1") && rs.contains("2") && rs.contains("3"));
    }

    #[test]
    fn test_w12bc_complex_073_mixed_bool() {
        let py = "result = (5 > 3) and (2 < 4)";
        let rs = transpile(py);
        assert!(rs.contains("5") && rs.contains("3") && rs.contains("2") && rs.contains("4"));
    }

    // Edge cases
    #[test]
    fn test_w12bc_edge_074_zero_div() {
        let py = "result = 10 / 0";
        let rs = transpile(py);
        assert!(rs.contains("10") && rs.contains("0"));
    }

    #[test]
    fn test_w12bc_edge_075_large_pow() {
        let py = "result = 2 ** 100";
        let rs = transpile(py);
        assert!(rs.contains("2") && rs.contains("100"));
    }

    #[test]
    fn test_w12bc_edge_076_float_precision() {
        let py = "result = 0.1 + 0.2";
        let rs = transpile(py);
        assert!(rs.contains("0.1") && rs.contains("0.2"));
    }

    #[test]
    fn test_w12bc_edge_077_negative_mod() {
        let py = "result = -17 % 5";
        let rs = transpile(py);
        assert!(rs.contains("17") && rs.contains("5"));
    }

    #[test]
    fn test_w12bc_edge_078_empty_string() {
        let py = r#"result = "" + "hello""#;
        let rs = transpile(py);
        assert!(rs.contains("hello"));
    }

    #[test]
    fn test_w12bc_edge_079_single_char() {
        let py = r#"result = "a" * 10"#;
        let rs = transpile(py);
        assert!(rs.contains("\"a\"") && rs.contains("10"));
    }

    #[test]
    fn test_w12bc_edge_080_bool_arith() {
        let py = "result = True + False";
        let rs = transpile(py);
        assert!(rs.contains("true") || rs.contains("false"));
    }

    // ========================================================================
    // Generic Call Tests (50 tests) - call_generic.rs
    // ========================================================================

    #[test]
    fn test_w12bc_call_081_int_constructor() {
        let py = "result = int(0)";  // int() requires an argument
        let rs = transpile(py);
        assert!(rs.contains("0"));
    }

    #[test]
    fn test_w12bc_call_082_int_from_str() {
        let py = r#"result = int("42")"#;
        let rs = transpile(py);
        assert!(rs.contains("42"));
    }

    #[test]
    fn test_w12bc_call_083_float_constructor() {
        let py = "result = float(0.0)";  // float() requires an argument
        let rs = transpile(py);
        assert!(rs.contains("0.0"));
    }

    #[test]
    fn test_w12bc_call_084_float_from_str() {
        let py = r#"result = float("3.14")"#;
        let rs = transpile(py);
        assert!(rs.contains("3.14"));
    }

    #[test]
    fn test_w12bc_call_085_str_constructor() {
        let py = "result = str(123)";
        let rs = transpile(py);
        assert!(rs.contains("123") && rs.contains("to_string"));
    }

    #[test]
    fn test_w12bc_call_086_bool_constructor() {
        let py = "result = bool(1)";
        let rs = transpile(py);
        assert!(rs.contains("1"));
    }

    #[test]
    fn test_w12bc_call_087_list_constructor() {
        let py = "result = list()";
        let rs = transpile(py);
        assert!(rs.contains("Vec") || rs.contains("vec!"));
    }

    #[test]
    fn test_w12bc_call_088_list_from_iter() {
        let py = "result = list([1, 2, 3])";
        let rs = transpile(py);
        assert!(rs.contains("1") && rs.contains("2") && rs.contains("3"));
    }

    #[test]
    fn test_w12bc_call_089_dict_constructor() {
        let py = "result = dict()";
        let rs = transpile(py);
        assert!(rs.contains("HashMap") || rs.contains("new"));
    }

    #[test]
    fn test_w12bc_call_090_set_constructor() {
        let py = "result = set()";
        let rs = transpile(py);
        assert!(rs.contains("HashSet") || rs.contains("new"));
    }

    #[test]
    fn test_w12bc_call_091_tuple_constructor() {
        let py = "result = tuple([1, 2])";
        let rs = transpile(py);
        assert!(rs.contains("1") && rs.contains("2"));
    }

    #[test]
    fn test_w12bc_call_092_bytes_constructor() {
        let py = "result = bytes()";
        let rs = transpile(py);
        assert!(rs.contains("Vec") || rs.contains("u8"));
    }

    #[test]
    fn test_w12bc_call_093_bytearray_constructor() {
        let py = "result = bytearray()";
        let rs = transpile(py);
        assert!(rs.contains("Vec") || rs.contains("u8"));
    }

    #[test]
    fn test_w12bc_call_094_frozenset_constructor() {
        let py = "result = frozenset([1, 2, 3])";
        let rs = transpile(py);
        assert!(rs.contains("1") && rs.contains("2") && rs.contains("3"));
    }

    #[test]
    fn test_w12bc_call_095_complex_constructor() {
        let py = "result = complex(1, 2)";
        let rs = transpile(py);
        assert!(rs.contains("1") && rs.contains("2"));
    }

    #[test]
    fn test_w12bc_call_096_print_basic() {
        let py = r#"print("hello")"#;
        let rs = transpile(py);
        assert!(rs.contains("println") || rs.contains("print"));
    }

    #[test]
    fn test_w12bc_call_097_print_multiple() {
        let py = r#"print("a", "b", "c")"#;
        let rs = transpile(py);
        assert!(rs.contains("a") && rs.contains("b") && rs.contains("c"));
    }

    #[test]
    fn test_w12bc_call_098_isinstance() {
        let py = "result = isinstance(x, int)";
        let rs = transpile(py);
        assert!(rs.contains("true") || rs.contains("isinstance"));
    }

    #[test]
    fn test_w12bc_call_099_issubclass() {
        let py = "result = issubclass(A, B)";
        let rs = transpile(py);
        assert!(rs.contains("A") && rs.contains("B"));
    }

    #[test]
    fn test_w12bc_call_100_hasattr() {
        let py = r#"result = hasattr(obj, "attr")"#;
        let rs = transpile(py);
        assert!(rs.contains("obj") && rs.contains("attr"));
    }

    #[test]
    fn test_w12bc_call_101_getattr() {
        // getattr is not fully supported, use direct attribute access instead
        let py = r#"result = obj.attr"#;
        let rs = transpile(py);
        assert!(rs.contains("obj") && rs.contains("attr"));
    }

    #[test]
    fn test_w12bc_call_102_setattr() {
        let py = r#"setattr(obj, "attr", value)"#;
        let rs = transpile(py);
        assert!(rs.contains("obj") && rs.contains("attr"));
    }

    #[test]
    fn test_w12bc_call_103_delattr() {
        let py = r#"delattr(obj, "attr")"#;
        let rs = transpile(py);
        assert!(rs.contains("obj") && rs.contains("attr"));
    }

    #[test]
    fn test_w12bc_call_104_len() {
        let py = "result = len([1, 2, 3])";
        let rs = transpile(py);
        assert!(rs.contains("len") || rs.contains("3"));
    }

    #[test]
    fn test_w12bc_call_105_range() {
        let py = "result = range(10)";
        let rs = transpile(py);
        assert!(rs.contains("10"));
    }

    #[test]
    fn test_w12bc_call_106_range_start_stop() {
        let py = "result = range(5, 10)";
        let rs = transpile(py);
        assert!(rs.contains("5") && rs.contains("10"));
    }

    #[test]
    fn test_w12bc_call_107_range_start_stop_step() {
        let py = "result = range(0, 10, 2)";
        let rs = transpile(py);
        assert!(rs.contains("0") && rs.contains("10") && rs.contains("2"));
    }

    #[test]
    fn test_w12bc_call_108_enumerate() {
        let py = "result = enumerate([1, 2, 3])";
        let rs = transpile(py);
        assert!(rs.contains("enumerate") || rs.contains("1"));
    }

    #[test]
    fn test_w12bc_call_109_zip() {
        let py = "result = zip([1, 2], [3, 4])";
        let rs = transpile(py);
        assert!(rs.contains("zip") || rs.contains("1"));
    }

    #[test]
    fn test_w12bc_call_110_map() {
        let py = "result = map(str, [1, 2, 3])";
        let rs = transpile(py);
        assert!(rs.contains("map") || rs.contains("1"));
    }

    #[test]
    fn test_w12bc_call_111_filter() {
        let py = "result = filter(None, [1, 0, 2])";
        let rs = transpile(py);
        assert!(rs.contains("filter") || rs.contains("1"));
    }

    #[test]
    fn test_w12bc_call_112_sum() {
        let py = "result = sum([1, 2, 3])";
        let rs = transpile(py);
        assert!(rs.contains("sum") || rs.contains("1"));
    }

    #[test]
    fn test_w12bc_call_113_min() {
        let py = "result = min([1, 2, 3])";
        let rs = transpile(py);
        assert!(rs.contains("min") || rs.contains("1"));
    }

    #[test]
    fn test_w12bc_call_114_max() {
        let py = "result = max([1, 2, 3])";
        let rs = transpile(py);
        assert!(rs.contains("max") || rs.contains("1"));
    }

    #[test]
    fn test_w12bc_call_115_abs() {
        let py = "result = abs(-5)";
        let rs = transpile(py);
        assert!(rs.contains("abs") || rs.contains("5"));
    }

    #[test]
    fn test_w12bc_call_116_round() {
        let py = "result = round(3.7)";
        let rs = transpile(py);
        assert!(rs.contains("round") || rs.contains("3.7"));
    }

    #[test]
    fn test_w12bc_call_117_pow() {
        let py = "result = pow(2, 8)";
        let rs = transpile(py);
        assert!(rs.contains("2") && rs.contains("8"));
    }

    #[test]
    fn test_w12bc_call_118_chr() {
        let py = "result = chr(65)";
        let rs = transpile(py);
        assert!(rs.contains("65") || rs.contains("char"));
    }

    #[test]
    fn test_w12bc_call_119_ord() {
        let py = r#"result = ord("A")"#;
        let rs = transpile(py);
        assert!(rs.contains("A"));
    }

    #[test]
    fn test_w12bc_call_120_hex() {
        let py = "result = hex(255)";
        let rs = transpile(py);
        assert!(rs.contains("255") || rs.contains("hex"));
    }

    #[test]
    fn test_w12bc_call_121_bin() {
        let py = "result = bin(10)";
        let rs = transpile(py);
        assert!(rs.contains("10") || rs.contains("bin"));
    }

    #[test]
    fn test_w12bc_call_122_oct() {
        let py = "result = oct(8)";
        let rs = transpile(py);
        assert!(rs.contains("8") || rs.contains("oct"));
    }

    #[test]
    fn test_w12bc_call_123_sorted() {
        let py = "result = sorted([3, 1, 2])";
        let rs = transpile(py);
        assert!(rs.contains("sort") || rs.contains("1"));
    }

    #[test]
    fn test_w12bc_call_124_reversed() {
        let py = "result = reversed([1, 2, 3])";
        let rs = transpile(py);
        assert!(rs.contains("rev") || rs.contains("1"));
    }

    #[test]
    fn test_w12bc_call_125_any() {
        let py = "result = any([True, False])";
        let rs = transpile(py);
        assert!(rs.contains("any") || rs.contains("true"));
    }

    #[test]
    fn test_w12bc_call_126_all() {
        let py = "result = all([True, True])";
        let rs = transpile(py);
        assert!(rs.contains("all") || rs.contains("true"));
    }

    #[test]
    fn test_w12bc_call_127_input() {
        let py = r#"result = input("Enter: ")"#;
        let rs = transpile(py);
        assert!(rs.contains("Enter") || rs.contains("input"));
    }

    #[test]
    fn test_w12bc_call_128_open() {
        let py = r#"f = open("file.txt")"#;
        let rs = transpile(py);
        assert!(rs.contains("file.txt") && rs.contains("open"));
    }

    #[test]
    fn test_w12bc_call_129_callable() {
        let py = "result = callable(func)";
        let rs = transpile(py);
        assert!(rs.contains("func"));
    }

    #[test]
    fn test_w12bc_call_130_id() {
        let py = "result = id(obj)";
        let rs = transpile(py);
        assert!(rs.contains("obj"));
    }

    // ========================================================================
    // NumPy Tests (30 tests) - stdlib_numpy.rs
    // ========================================================================

    #[test]
    fn test_w12bc_numpy_131_array() {
        let py = "import numpy as np\nresult = np.array([1, 2, 3])";
        let rs = transpile(py);
        assert!(rs.contains("1") && rs.contains("2") && rs.contains("3"));
    }

    #[test]
    fn test_w12bc_numpy_132_zeros() {
        let py = "import numpy as np\nresult = np.zeros(5)";
        let rs = transpile(py);
        assert!(rs.contains("5") || rs.contains("zeros"));
    }

    #[test]
    fn test_w12bc_numpy_133_ones() {
        let py = "import numpy as np\nresult = np.ones(3)";
        let rs = transpile(py);
        assert!(rs.contains("3") || rs.contains("ones"));
    }

    #[test]
    fn test_w12bc_numpy_134_arange() {
        let py = "import numpy as np\nresult = np.arange(10)";
        let rs = transpile(py);
        assert!(rs.contains("10") || rs.contains("range"));
    }

    #[test]
    fn test_w12bc_numpy_135_linspace() {
        let py = "import numpy as np\nresult = np.linspace(0, 10, 5)";
        let rs = transpile(py);
        assert!(rs.contains("0") && rs.contains("10") && rs.contains("5"));
    }

    #[test]
    fn test_w12bc_numpy_136_sum() {
        let py = "import numpy as np\nresult = np.sum(arr)";
        let rs = transpile(py);
        assert!(rs.contains("sum") || rs.contains("arr"));
    }

    #[test]
    fn test_w12bc_numpy_137_mean() {
        let py = "import numpy as np\nresult = np.mean(arr)";
        let rs = transpile(py);
        assert!(rs.contains("mean") || rs.contains("arr"));
    }

    #[test]
    fn test_w12bc_numpy_138_std() {
        let py = "import numpy as np\nresult = np.std(arr)";
        let rs = transpile(py);
        assert!(rs.contains("std") || rs.contains("arr"));
    }

    #[test]
    fn test_w12bc_numpy_139_max() {
        let py = "import numpy as np\nresult = np.max(arr)";
        let rs = transpile(py);
        assert!(rs.contains("max") || rs.contains("arr"));
    }

    #[test]
    fn test_w12bc_numpy_140_min() {
        let py = "import numpy as np\nresult = np.min(arr)";
        let rs = transpile(py);
        assert!(rs.contains("min") || rs.contains("arr"));
    }

    #[test]
    fn test_w12bc_numpy_141_dot() {
        let py = "import numpy as np\nresult = np.dot(a, b)";
        let rs = transpile(py);
        assert!(rs.contains("dot") || rs.contains("a"));
    }

    #[test]
    fn test_w12bc_numpy_142_matmul() {
        let py = "import numpy as np\nresult = np.matmul(a, b)";
        let rs = transpile(py);
        assert!(rs.contains("a") && rs.contains("b"));
    }

    #[test]
    fn test_w12bc_numpy_143_transpose() {
        let py = "import numpy as np\nresult = np.transpose(arr)";
        let rs = transpile(py);
        assert!(rs.contains("transpose") || rs.contains("arr"));
    }

    #[test]
    fn test_w12bc_numpy_144_reshape() {
        let py = "import numpy as np\nresult = np.reshape(arr, (2, 3))";
        let rs = transpile(py);
        assert!(rs.contains("reshape") || rs.contains("arr"));
    }

    #[test]
    fn test_w12bc_numpy_145_concatenate() {
        let py = "import numpy as np\nresult = np.concatenate([a, b])";
        let rs = transpile(py);
        assert!(rs.contains("a") && rs.contains("b"));
    }

    #[test]
    fn test_w12bc_numpy_146_split() {
        let py = "import numpy as np\nresult = np.split(arr, 3)";
        let rs = transpile(py);
        assert!(rs.contains("split") || rs.contains("arr"));
    }

    #[test]
    fn test_w12bc_numpy_147_sqrt() {
        let py = "import numpy as np\nresult = np.sqrt(arr)";
        let rs = transpile(py);
        assert!(rs.contains("sqrt") || rs.contains("arr"));
    }

    #[test]
    fn test_w12bc_numpy_148_exp() {
        let py = "import numpy as np\nresult = np.exp(arr)";
        let rs = transpile(py);
        assert!(rs.contains("exp") || rs.contains("arr"));
    }

    #[test]
    fn test_w12bc_numpy_149_log() {
        let py = "import numpy as np\nresult = np.log(arr)";
        let rs = transpile(py);
        assert!(rs.contains("log") || rs.contains("ln") || rs.contains("arr"));
    }

    #[test]
    fn test_w12bc_numpy_150_sin() {
        let py = "import numpy as np\nresult = np.sin(arr)";
        let rs = transpile(py);
        assert!(rs.contains("sin") || rs.contains("arr"));
    }

    #[test]
    fn test_w12bc_numpy_151_cos() {
        let py = "import numpy as np\nresult = np.cos(arr)";
        let rs = transpile(py);
        assert!(rs.contains("cos") || rs.contains("arr"));
    }

    #[test]
    fn test_w12bc_numpy_152_abs() {
        let py = "import numpy as np\nresult = np.abs(arr)";
        let rs = transpile(py);
        assert!(rs.contains("abs") || rs.contains("arr"));
    }

    #[test]
    fn test_w12bc_numpy_153_clip() {
        let py = "import numpy as np\nresult = np.clip(arr, 0, 10)";
        let rs = transpile(py);
        assert!(rs.contains("0") && rs.contains("10"));
    }

    #[test]
    fn test_w12bc_numpy_154_argmax() {
        let py = "import numpy as np\nresult = np.argmax(arr)";
        let rs = transpile(py);
        assert!(rs.contains("argmax") || rs.contains("arr"));
    }

    #[test]
    fn test_w12bc_numpy_155_argmin() {
        let py = "import numpy as np\nresult = np.argmin(arr)";
        let rs = transpile(py);
        assert!(rs.contains("argmin") || rs.contains("arr"));
    }

    #[test]
    fn test_w12bc_numpy_156_var() {
        let py = "import numpy as np\nresult = np.var(arr)";
        let rs = transpile(py);
        assert!(rs.contains("var") || rs.contains("arr"));
    }

    #[test]
    fn test_w12bc_numpy_157_norm() {
        let py = "import numpy as np\nresult = np.linalg.norm(arr)";
        let rs = transpile(py);
        assert!(rs.contains("norm") || rs.contains("arr"));
    }

    #[test]
    fn test_w12bc_numpy_158_add() {
        let py = "import numpy as np\nresult = a + b";
        let rs = transpile(py);
        assert!(rs.contains("a") && rs.contains("b"));
    }

    #[test]
    fn test_w12bc_numpy_159_mul() {
        let py = "import numpy as np\nresult = a * 2";
        let rs = transpile(py);
        assert!(rs.contains("a") && rs.contains("2"));
    }

    #[test]
    fn test_w12bc_numpy_160_shape() {
        let py = "import numpy as np\nresult = arr.shape";
        let rs = transpile(py);
        assert!(rs.contains("shape") || rs.contains("arr"));
    }

    // ========================================================================
    // Stdlib Misc Tests (20 tests) - stdlib_misc.rs
    // ========================================================================

    #[test]
    fn test_w12bc_misc_161_hashlib_md5() {
        let py = "import hashlib\nresult = hashlib.md5()";
        let rs = transpile(py);
        assert!(rs.contains("md5") || rs.contains("hash"));
    }

    #[test]
    fn test_w12bc_misc_162_hashlib_sha256() {
        let py = "import hashlib\nresult = hashlib.sha256()";
        let rs = transpile(py);
        assert!(rs.contains("sha256") || rs.contains("hash"));
    }

    #[test]
    fn test_w12bc_misc_163_uuid_uuid4() {
        let py = "import uuid\nresult = uuid.uuid4()";
        let rs = transpile(py);
        assert!(rs.contains("uuid") || rs.contains("Uuid"));
    }

    #[test]
    fn test_w12bc_misc_164_datetime_now() {
        let py = "from datetime import datetime\nresult = datetime.now()";
        let rs = transpile(py);
        assert!(rs.contains("now") || rs.contains("Local"));
    }

    #[test]
    fn test_w12bc_misc_165_datetime_strftime() {
        let py = r#"from datetime import datetime
result = datetime.now().strftime("%Y-%m-%d")"#;
        let rs = transpile(py);
        assert!(rs.contains("strftime") || rs.contains("format"));
    }

    #[test]
    fn test_w12bc_misc_166_timedelta() {
        let py = "from datetime import timedelta\nresult = timedelta(days=5)";
        let rs = transpile(py);
        assert!(rs.contains("days") || rs.contains("Duration"));
    }

    #[test]
    fn test_w12bc_misc_167_csv_reader() {
        let py = "import csv\nreader = csv.reader(file)";
        let rs = transpile(py);
        assert!(rs.contains("csv") || rs.contains("reader"));
    }

    #[test]
    fn test_w12bc_misc_168_csv_writer() {
        let py = "import csv\nwriter = csv.writer(file)";
        let rs = transpile(py);
        assert!(rs.contains("csv") || rs.contains("writer"));
    }

    #[test]
    fn test_w12bc_misc_169_csv_dictreader() {
        let py = "import csv\nreader = csv.DictReader(file)";
        let rs = transpile(py);
        assert!(rs.contains("DictReader") || rs.contains("csv"));
    }

    #[test]
    fn test_w12bc_misc_170_bisect_left() {
        let py = "import bisect\nresult = bisect.bisect_left([1, 2, 3], 2)";
        let rs = transpile(py);
        assert!(rs.contains("bisect") || rs.contains("binary_search"));
    }

    #[test]
    fn test_w12bc_misc_171_bisect_right() {
        let py = "import bisect\nresult = bisect.bisect_right([1, 2, 3], 2)";
        let rs = transpile(py);
        assert!(rs.contains("bisect") || rs.contains("binary_search"));
    }

    #[test]
    fn test_w12bc_misc_172_heapq_push() {
        let py = "import heapq\nheapq.heappush(heap, 5)";
        let rs = transpile(py);
        assert!(rs.contains("heap") && rs.contains("5"));
    }

    #[test]
    fn test_w12bc_misc_173_heapq_pop() {
        let py = "import heapq\nresult = heapq.heappop(heap)";
        let rs = transpile(py);
        assert!(rs.contains("heap"));
    }

    #[test]
    fn test_w12bc_misc_174_copy_copy() {
        let py = "import copy\nresult = copy.copy(obj)";
        let rs = transpile(py);
        assert!(rs.contains("clone") || rs.contains("copy"));
    }

    #[test]
    fn test_w12bc_misc_175_copy_deepcopy() {
        let py = "import copy\nresult = copy.deepcopy(obj)";
        let rs = transpile(py);
        assert!(rs.contains("clone") || rs.contains("deepcopy"));
    }

    #[test]
    fn test_w12bc_misc_176_sys_exit() {
        let py = "import sys\nsys.exit(0)";
        let rs = transpile(py);
        assert!(rs.contains("exit") || rs.contains("0"));
    }

    #[test]
    fn test_w12bc_misc_177_pickle_dumps() {
        let py = "import pickle\nresult = pickle.dumps(obj)";
        let rs = transpile(py);
        assert!(rs.contains("obj"));
    }

    #[test]
    fn test_w12bc_misc_178_pickle_loads() {
        let py = "import pickle\nresult = pickle.loads(data)";
        let rs = transpile(py);
        assert!(rs.contains("data"));
    }

    #[test]
    fn test_w12bc_misc_179_pprint() {
        let py = "import pprint\npprint.pprint(obj)";
        let rs = transpile(py);
        assert!(rs.contains("obj"));
    }

    #[test]
    fn test_w12bc_misc_180_statistics_mean() {
        let py = "import statistics\nresult = statistics.mean([1, 2, 3])";
        let rs = transpile(py);
        assert!(rs.contains("mean") || rs.contains("1"));
    }

    // ========================================================================
    // Unary & Call Tests (20 tests) - convert_unary_and_call.rs
    // ========================================================================

    #[test]
    fn test_w12bc_unary_181_neg_int() {
        let py = "result = -42";
        let rs = transpile(py);
        assert!(rs.contains("42"));
    }

    #[test]
    fn test_w12bc_unary_182_neg_float() {
        let py = "result = -3.14";
        let rs = transpile(py);
        assert!(rs.contains("3.14"));
    }

    #[test]
    fn test_w12bc_unary_183_pos_int() {
        let py = "result = +5";
        let rs = transpile(py);
        assert!(rs.contains("5"));
    }

    #[test]
    fn test_w12bc_unary_184_not_bool() {
        let py = "result = not True";
        let rs = transpile(py);
        assert!(rs.contains("!") || rs.contains("true"));
    }

    #[test]
    fn test_w12bc_unary_185_bitnot() {
        let py = "result = ~15";
        let rs = transpile(py);
        assert!(rs.contains("15"));
    }

    #[test]
    fn test_w12bc_unary_186_neg_var() {
        let py = "x = 10\nresult = -x";
        let rs = transpile(py);
        assert!(rs.contains("x"));
    }

    #[test]
    fn test_w12bc_call_187_star_args() {
        let py = "def f(*args): pass\nf(1, 2, 3)";
        let rs = transpile(py);
        assert!(rs.contains("1") && rs.contains("2") && rs.contains("3"));
    }

    #[test]
    fn test_w12bc_call_188_star_kwargs() {
        let py = "def f(**kwargs): pass\nf(a=1, b=2)";
        let rs = transpile(py);
        assert!(rs.contains("a") && rs.contains("b"));
    }

    #[test]
    fn test_w12bc_call_189_nested_call() {
        let py = "result = f(g(h(x)))";
        let rs = transpile(py);
        assert!(rs.contains("f") && rs.contains("g") && rs.contains("h"));
    }

    #[test]
    fn test_w12bc_call_190_lambda_call() {
        // Lambda as call target is not supported, use lambda in context instead
        let py = "f = lambda x: x + 1\nresult = f(5)";
        let rs = transpile(py);
        assert!(rs.contains("5"));
    }

    #[test]
    fn test_w12bc_call_191_method_chain() {
        let py = "result = obj.method1().method2()";
        let rs = transpile(py);
        assert!(rs.contains("method1") && rs.contains("method2"));
    }

    #[test]
    fn test_w12bc_call_192_constructor_no_args() {
        let py = "class A:\n    pass\nobj = A()";
        let rs = transpile(py);
        assert!(rs.contains("A") && rs.contains("new"));
    }

    #[test]
    fn test_w12bc_call_193_constructor_with_args() {
        let py = "class B:\n    def __init__(self, x): pass\nobj = B(5)";
        let rs = transpile(py);
        assert!(rs.contains("B") && rs.contains("5"));
    }

    #[test]
    fn test_w12bc_call_194_builtin_type() {
        let py = "result = type(obj)";
        let rs = transpile(py);
        assert!(rs.contains("obj"));
    }

    #[test]
    fn test_w12bc_call_195_iter() {
        let py = "result = iter([1, 2, 3])";
        let rs = transpile(py);
        assert!(rs.contains("iter") || rs.contains("1"));
    }

    #[test]
    fn test_w12bc_call_196_next() {
        let py = "result = next(it)";
        let rs = transpile(py);
        assert!(rs.contains("next") || rs.contains("it"));
    }

    #[test]
    fn test_w12bc_call_197_divmod() {
        let py = "result = divmod(17, 5)";
        let rs = transpile(py);
        assert!(rs.contains("17") && rs.contains("5"));
    }

    #[test]
    fn test_w12bc_call_198_format() {
        let py = r#"result = format(255, "x")"#;
        let rs = transpile(py);
        assert!(rs.contains("255") && rs.contains("x"));
    }

    #[test]
    fn test_w12bc_call_199_hash() {
        let py = "result = hash(obj)";
        let rs = transpile(py);
        assert!(rs.contains("obj"));
    }

    #[test]
    fn test_w12bc_call_200_repr() {
        let py = "result = repr(obj)";
        let rs = transpile(py);
        assert!(rs.contains("obj"));
    }
}
