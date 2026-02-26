//! Wave 15 coverage tests: direct_rules_convert and type_mapper/type_helpers
//!
//! Targets uncovered code paths in:
//! - direct_rules_convert: augmented assignments, unpacking, delete, assert,
//!   bitwise ops, floor division, power, unary ops, chained comparisons
//! - type_mapper/type_helpers: type annotations, Optional, Union, generics,
//!   complex return types, nested generics
//! - Complex expression patterns: nested calls, method chaining, conditionals,
//!   string formatting, list/string operations, enumerate, zip, range
//!
//! 200 tests total across 3 categories

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
    // SECTION 1: Direct Rules Convert (80 tests)
    // ========================================================================

    #[test]
    fn test_w15rt_rules_001_augmented_add_int() {
        let code = "def f(x: int) -> int:\n    x += 1\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("+=") || result.contains("+"), "augmented add: {}", result);
    }

    #[test]
    fn test_w15rt_rules_002_augmented_sub_int() {
        let code = "def f(x: int) -> int:\n    x -= 1\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("-=") || result.contains("-"), "augmented sub: {}", result);
    }

    #[test]
    fn test_w15rt_rules_003_augmented_mul_int() {
        let code = "def f(x: int) -> int:\n    x *= 2\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("*=") || result.contains("*"), "augmented mul: {}", result);
    }

    #[test]
    fn test_w15rt_rules_004_augmented_div_float() {
        let code = "def f(x: float) -> float:\n    x /= 2.0\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("/=") || result.contains("/"), "augmented div: {}", result);
    }

    #[test]
    fn test_w15rt_rules_005_augmented_floordiv() {
        let code = "def f(x: int, y: int) -> int:\n    x //= y\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_rules_006_augmented_mod() {
        let code = "def f(x: int) -> int:\n    x %= 2\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("%=") || result.contains("%"), "augmented mod: {}", result);
    }

    #[test]
    fn test_w15rt_rules_007_augmented_pow() {
        let code = "def f(x: int) -> int:\n    x **= 2\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_rules_008_augmented_bitand() {
        let code = "def f(x: int, mask: int) -> int:\n    x &= mask\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("&=") || result.contains("&"), "bitand assign: {}", result);
    }

    #[test]
    fn test_w15rt_rules_009_augmented_bitor() {
        let code = "def f(x: int, flag: int) -> int:\n    x |= flag\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("|=") || result.contains("|"), "bitor assign: {}", result);
    }

    #[test]
    fn test_w15rt_rules_010_augmented_bitxor() {
        let code = "def f(x: int, bits: int) -> int:\n    x ^= bits\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("^=") || result.contains("^"), "bitxor assign: {}", result);
    }

    #[test]
    fn test_w15rt_rules_011_augmented_lshift() {
        let code = "def f(x: int) -> int:\n    x <<= 1\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("<<=") || result.contains("<<"), "lshift assign: {}", result);
    }

    #[test]
    fn test_w15rt_rules_012_augmented_rshift() {
        let code = "def f(x: int) -> int:\n    x >>= 1\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains(">>=") || result.contains(">>"), "rshift assign: {}", result);
    }

    #[test]
    fn test_w15rt_rules_013_tuple_unpack_two() {
        let code = "def f() -> int:\n    x, y = 1, 2\n    return x + y";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_rules_014_tuple_unpack_three() {
        let code = "def f() -> int:\n    a, b, c = 1, 2, 3\n    return a + b + c";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_rules_015_tuple_unpack_paren() {
        let code = "def f() -> int:\n    (x, y) = (10, 20)\n    return x + y";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_rules_016_assert_simple() {
        let code = "def f(x: int) -> int:\n    assert x > 0\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("assert") || result.contains("panic"), "assert: {}", result);
    }

    #[test]
    fn test_w15rt_rules_017_assert_with_msg() {
        let code = "def f(x: int) -> int:\n    assert x > 0, \"must be positive\"\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_rules_018_pass_stmt() {
        let code = "def f() -> None:\n    pass";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("fn"), "should have function: {}", result);
    }

    #[test]
    fn test_w15rt_rules_019_ellipsis_in_func() {
        let code = "def f() -> None:\n    ...";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_rules_020_in_list() {
        let code = "def f(x: int, items: list) -> bool:\n    return x in items";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("contains"), "in list: {}", result);
    }

    #[test]
    fn test_w15rt_rules_021_not_in_list() {
        let code = "def f(x: int, items: list) -> bool:\n    return x not in items";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("contains") || result.contains("!"), "not in: {}", result);
    }

    #[test]
    fn test_w15rt_rules_022_is_none() {
        let code = "def f(x: int) -> bool:\n    return x is None";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_rules_023_is_not_none() {
        let code = "def f(x: int) -> bool:\n    return x is not None";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_rules_024_bitwise_and() {
        let code = "def f(x: int, y: int) -> int:\n    return x & y";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("&"), "bitwise and: {}", result);
    }

    #[test]
    fn test_w15rt_rules_025_bitwise_or() {
        let code = "def f(x: int, y: int) -> int:\n    return x | y";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("|"), "bitwise or: {}", result);
    }

    #[test]
    fn test_w15rt_rules_026_bitwise_xor() {
        let code = "def f(x: int, y: int) -> int:\n    return x ^ y";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("^"), "bitwise xor: {}", result);
    }

    #[test]
    fn test_w15rt_rules_027_bitwise_not() {
        let code = "def f(x: int) -> int:\n    return ~x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("!") || result.contains("~"), "bitwise not: {}", result);
    }

    #[test]
    fn test_w15rt_rules_028_lshift() {
        let code = "def f(x: int) -> int:\n    return x << 1";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("<<"), "lshift: {}", result);
    }

    #[test]
    fn test_w15rt_rules_029_rshift() {
        let code = "def f(x: int) -> int:\n    return x >> 1";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains(">>"), "rshift: {}", result);
    }

    #[test]
    fn test_w15rt_rules_030_floor_div() {
        let code = "def f(x: int, y: int) -> int:\n    return x // y";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_rules_031_power() {
        let code = "def f(x: int) -> int:\n    return x ** 2";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("pow"), "power: {}", result);
    }

    #[test]
    fn test_w15rt_rules_032_unary_neg() {
        let code = "def f(x: int) -> int:\n    return -x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("-"), "unary neg: {}", result);
    }

    #[test]
    fn test_w15rt_rules_033_unary_pos() {
        let code = "def f(x: int) -> int:\n    return +x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_rules_034_not_expr() {
        let code = "def f(x: bool) -> bool:\n    return not x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("!"), "not expr: {}", result);
    }

    #[test]
    fn test_w15rt_rules_035_comparison_lt() {
        let code = "def f(x: int) -> bool:\n    return x < 10";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("<"), "lt comparison: {}", result);
    }

    #[test]
    fn test_w15rt_rules_036_comparison_le() {
        let code = "def f(x: int) -> bool:\n    return x <= 10";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("<="), "le comparison: {}", result);
    }

    #[test]
    fn test_w15rt_rules_037_comparison_gt() {
        let code = "def f(x: int) -> bool:\n    return x > 0";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains(">"), "gt comparison: {}", result);
    }

    #[test]
    fn test_w15rt_rules_038_comparison_ge() {
        let code = "def f(x: int) -> bool:\n    return x >= 0";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains(">="), "ge comparison: {}", result);
    }

    #[test]
    fn test_w15rt_rules_039_comparison_eq() {
        let code = "def f(x: int) -> bool:\n    return x == 0";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("=="), "eq comparison: {}", result);
    }

    #[test]
    fn test_w15rt_rules_040_comparison_ne() {
        let code = "def f(x: int) -> bool:\n    return x != 0";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("!="), "ne comparison: {}", result);
    }

    #[test]
    fn test_w15rt_rules_041_and_expr() {
        let code = "def f(a: bool, b: bool) -> bool:\n    return a and b";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("&&"), "and expr: {}", result);
    }

    #[test]
    fn test_w15rt_rules_042_or_expr() {
        let code = "def f(a: bool, b: bool) -> bool:\n    return a or b";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("||"), "or expr: {}", result);
    }

    #[test]
    fn test_w15rt_rules_043_if_else_basic() {
        let code =
            "def f(x: int) -> int:\n    if x > 0:\n        return x\n    else:\n        return -x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("if"), "if else: {}", result);
    }

    #[test]
    fn test_w15rt_rules_044_while_loop() {
        let code = "def f(n: int) -> int:\n    s = 0\n    while n > 0:\n        s += n\n        n -= 1\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("while"), "while loop: {}", result);
    }

    #[test]
    fn test_w15rt_rules_045_for_range() {
        let code = "def f(n: int) -> int:\n    s = 0\n    for i in range(n):\n        s += i\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("for"), "for range: {}", result);
    }

    #[test]
    fn test_w15rt_rules_046_for_list() {
        let code = "def f(items: list) -> int:\n    s = 0\n    for x in items:\n        s += x\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("for"), "for list: {}", result);
    }

    #[test]
    fn test_w15rt_rules_047_break_in_loop() {
        let code = "def f(items: list) -> int:\n    for x in items:\n        if x < 0:\n            break\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("break"), "break: {}", result);
    }

    #[test]
    fn test_w15rt_rules_048_continue_in_loop() {
        let code = "def f(n: int) -> int:\n    s = 0\n    for i in range(n):\n        if i % 2 == 0:\n            continue\n        s += i\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("continue"), "continue: {}", result);
    }

    #[test]
    fn test_w15rt_rules_049_raise_exception() {
        let code = "def f(x: int) -> int:\n    if x < 0:\n        raise ValueError(\"negative\")\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("panic"), "raise: {}", result);
    }

    #[test]
    fn test_w15rt_rules_050_raise_bare() {
        let code = "def f() -> None:\n    raise Exception()";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("panic"), "raise bare: {}", result);
    }

    #[test]
    fn test_w15rt_rules_051_try_except() {
        let code =
            "def f() -> int:\n    try:\n        return 1\n    except Exception:\n        return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_rules_052_with_stmt() {
        let code = "def f() -> None:\n    with open(\"test.txt\") as fh:\n        data = fh.read()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_rules_053_return_none() {
        let code = "def f() -> None:\n    return None";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("return"), "return none: {}", result);
    }

    #[test]
    fn test_w15rt_rules_054_return_empty() {
        let code = "def f() -> None:\n    return";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_rules_055_multiple_assign() {
        let code = "def f() -> int:\n    a = 1\n    b = 2\n    c = 3\n    return a + b + c";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_rules_056_reassignment() {
        let code = "def f() -> int:\n    x = 1\n    x = 2\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("mut"), "mutable reassign: {}", result);
    }

    #[test]
    fn test_w15rt_rules_057_for_dict_items() {
        let code = "def f(d: dict) -> None:\n    for k, v in d.items():\n        print(k, v)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("iter"), "dict items: {}", result);
    }

    #[test]
    fn test_w15rt_rules_058_for_dict_keys() {
        let code = "def f(d: dict) -> None:\n    for k in d.keys():\n        print(k)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("keys"), "dict keys: {}", result);
    }

    #[test]
    fn test_w15rt_rules_059_for_dict_values() {
        let code = "def f(d: dict) -> None:\n    for v in d.values():\n        print(v)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("values"), "dict values: {}", result);
    }

    #[test]
    fn test_w15rt_rules_060_list_comp_simple() {
        let code = "def f(n: int) -> list:\n    return [x * 2 for x in range(n)]";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("map") || result.contains("collect"), "list comp: {}", result);
    }

    #[test]
    fn test_w15rt_rules_061_list_comp_filter() {
        let code = "def f(n: int) -> list:\n    return [x for x in range(n) if x > 5]";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("filter"), "filtered comp: {}", result);
    }

    #[test]
    fn test_w15rt_rules_062_dict_comp() {
        let code = "def f(n: int) -> dict:\n    return {x: x * x for x in range(n)}";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("collect") || result.contains("map"), "dict comp: {}", result);
    }

    #[test]
    fn test_w15rt_rules_063_set_comp() {
        let code = "def f(items: list) -> set:\n    return {x for x in items}";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_rules_064_conditional_expr() {
        let code = "def f(x: int) -> int:\n    return x if x > 0 else -x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("if"), "conditional expr: {}", result);
    }

    #[test]
    fn test_w15rt_rules_065_lambda_simple() {
        let code = "def f() -> None:\n    sq = lambda x: x * x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_rules_066_fstring_basic() {
        let code = "def f(name: str) -> str:\n    return f\"hello {name}\"";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"), "fstring: {}", result);
    }

    #[test]
    fn test_w15rt_rules_067_fstring_expr() {
        let code = "def f(x: int) -> str:\n    return f\"value is {x + 1}\"";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"), "fstring expr: {}", result);
    }

    #[test]
    fn test_w15rt_rules_068_in_string() {
        let code = "def f(s: str) -> bool:\n    return \"hello\" in s";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("contains"), "in string: {}", result);
    }

    #[test]
    fn test_w15rt_rules_069_not_in_string() {
        let code = "def f(s: str) -> bool:\n    return \"hello\" not in s";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("contains"), "not in string: {}", result);
    }

    #[test]
    fn test_w15rt_rules_070_in_tuple() {
        let code = "def f(x: int) -> bool:\n    return x in (1, 2, 3)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("contains"), "in tuple: {}", result);
    }

    #[test]
    fn test_w15rt_rules_071_not_in_tuple() {
        let code = "def f(x: int) -> bool:\n    return x not in (1, 2, 3)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("contains"), "not in tuple: {}", result);
    }

    #[test]
    fn test_w15rt_rules_072_in_dict() {
        let code = "def f(key: str, d: dict) -> bool:\n    return key in d";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("contains_key"), "in dict: {}", result);
    }

    #[test]
    fn test_w15rt_rules_073_not_in_dict() {
        let code = "def f(key: str, d: dict) -> bool:\n    return key not in d";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("contains_key") || result.contains("!"), "not in dict: {}", result);
    }

    #[test]
    fn test_w15rt_rules_074_floor_div_negative() {
        let code = "def f(a: int, b: int) -> int:\n    return a // b";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_rules_075_power_variable() {
        let code = "def f(base: int, exp: int) -> int:\n    return base ** exp";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("pow"), "power variable: {}", result);
    }

    #[test]
    fn test_w15rt_rules_076_list_repeat() {
        let code = "def f() -> list:\n    return [0] * 5";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_rules_077_mixed_float_int_mul() {
        let code = "def f(x: float, n: int) -> float:\n    return x * n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_rules_078_mixed_float_int_sub() {
        let code = "def f(x: float, n: int) -> float:\n    return x - n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_rules_079_len_sub_saturating() {
        let code = "def f(items: list) -> int:\n    return len(items) - 1";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("saturating_sub") || result.contains("len"), "len sub: {}", result);
    }

    #[test]
    fn test_w15rt_rules_080_augmented_add_float() {
        let code = "def f(x: float) -> float:\n    x += 1.5\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 2: Type Inference Patterns (60 tests)
    // ========================================================================

    #[test]
    fn test_w15rt_type_001_int_annotation() {
        let code = "def f() -> int:\n    x: int = 1\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("i32") || result.contains("i64"), "int type: {}", result);
    }

    #[test]
    fn test_w15rt_type_002_str_annotation() {
        let code = "def f() -> str:\n    y: str = \"hello\"\n    return y";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("String") || result.contains("str"), "str type: {}", result);
    }

    #[test]
    fn test_w15rt_type_003_float_annotation() {
        let code = "def f() -> float:\n    z: float = 1.5\n    return z";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("f64"), "float type: {}", result);
    }

    #[test]
    fn test_w15rt_type_004_bool_annotation() {
        let code = "def f() -> bool:\n    b: bool = True\n    return b";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("bool"), "bool type: {}", result);
    }

    #[test]
    fn test_w15rt_type_005_list_int_annotation() {
        let code = "def f() -> list:\n    x: list = [1, 2, 3]\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("vec!") || result.contains("Vec"), "list: {}", result);
    }

    #[test]
    fn test_w15rt_type_006_dict_annotation() {
        let code = "def f() -> dict:\n    x: dict = {}\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashMap") || result.contains("new"), "dict: {}", result);
    }

    #[test]
    fn test_w15rt_type_007_return_int() {
        let code = "def f() -> int:\n    return 42";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("-> i32") || result.contains("-> i64"), "return int: {}", result);
    }

    #[test]
    fn test_w15rt_type_008_return_str() {
        let code = "def f() -> str:\n    return \"hi\"";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("-> String"), "return str: {}", result);
    }

    #[test]
    fn test_w15rt_type_009_return_float() {
        let code = "def f() -> float:\n    return 3.14";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("-> f64"), "return float: {}", result);
    }

    #[test]
    fn test_w15rt_type_010_return_bool() {
        let code = "def f() -> bool:\n    return True";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("-> bool"), "return bool: {}", result);
    }

    #[test]
    fn test_w15rt_type_011_return_list() {
        let code = "def f() -> list:\n    return [1, 2, 3]";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Vec"), "return list: {}", result);
    }

    #[test]
    fn test_w15rt_type_012_param_int() {
        let code = "def f(x: int) -> int:\n    return x + 1";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("i32") || result.contains("i64"), "param int: {}", result);
    }

    #[test]
    fn test_w15rt_type_013_param_str() {
        let code = "def f(s: str) -> str:\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_014_param_float() {
        let code = "def f(x: float) -> float:\n    return x * 2.0";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("f64"), "param float: {}", result);
    }

    #[test]
    fn test_w15rt_type_015_param_bool() {
        let code = "def f(flag: bool) -> bool:\n    return not flag";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("bool"), "param bool: {}", result);
    }

    #[test]
    fn test_w15rt_type_016_param_list() {
        let code = "def f(items: list) -> int:\n    return len(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_017_param_dict() {
        let code = "def f(d: dict) -> int:\n    return len(d)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_018_optional_int() {
        let code = "from typing import Optional\ndef f(x: Optional[int]) -> int:\n    if x is None:\n        return 0\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Option"), "optional int: {}", result);
    }

    #[test]
    fn test_w15rt_type_019_optional_str() {
        let code = "from typing import Optional\ndef f(s: Optional[str]) -> str:\n    if s is None:\n        return \"\"\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Option"), "optional str: {}", result);
    }

    #[test]
    fn test_w15rt_type_020_list_typed() {
        let code =
            "from typing import List\ndef f(items: List[int]) -> int:\n    return len(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Vec"), "list typed: {}", result);
    }

    #[test]
    fn test_w15rt_type_021_dict_typed() {
        let code = "from typing import Dict\ndef f(d: Dict[str, int]) -> int:\n    return len(d)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashMap"), "dict typed: {}", result);
    }

    #[test]
    fn test_w15rt_type_022_tuple_typed() {
        let code =
            "from typing import Tuple\ndef f() -> Tuple[int, str]:\n    return (1, \"hello\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_023_set_typed() {
        let code = "def f() -> set:\n    return {1, 2, 3}";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashSet") || result.contains("set"), "set type: {}", result);
    }

    #[test]
    fn test_w15rt_type_024_return_none_type() {
        let code = "def f() -> None:\n    print(\"hello\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_025_multiple_params() {
        let code = "def f(a: int, b: float, c: str) -> str:\n    return c";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("i32") || result.contains("i64"), "multi params: {}", result);
        assert!(result.contains("f64"), "multi params f64: {}", result);
    }

    #[test]
    fn test_w15rt_type_026_nested_list() {
        let code =
            "from typing import List\ndef f(items: List[List[int]]) -> int:\n    return len(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Vec"), "nested list: {}", result);
    }

    #[test]
    fn test_w15rt_type_027_dict_list_value() {
        let code = "from typing import Dict, List\ndef f(d: Dict[str, List[int]]) -> int:\n    return len(d)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashMap"), "dict list value: {}", result);
    }

    #[test]
    fn test_w15rt_type_028_optional_list() {
        let code = "from typing import Optional, List\ndef f(items: Optional[List[int]]) -> int:\n    if items is None:\n        return 0\n    return len(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Option"), "optional list: {}", result);
    }

    #[test]
    fn test_w15rt_type_029_int_literal_type() {
        let code = "def f() -> int:\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_030_float_literal_type() {
        let code = "def f() -> float:\n    return 0.0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_031_str_literal_type() {
        let code = "def f() -> str:\n    return \"\"";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_032_bool_literal_true() {
        let code = "def f() -> bool:\n    return True";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("true"), "bool true: {}", result);
    }

    #[test]
    fn test_w15rt_type_033_bool_literal_false() {
        let code = "def f() -> bool:\n    return False";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("false"), "bool false: {}", result);
    }

    #[test]
    fn test_w15rt_type_034_none_literal() {
        let code = "from typing import Optional\ndef f() -> Optional[int]:\n    return None";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("None"), "none literal: {}", result);
    }

    #[test]
    fn test_w15rt_type_035_bytes_type() {
        let code = "def f() -> bytes:\n    return b\"hello\"";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_036_list_empty_typed() {
        let code = "from typing import List\ndef f() -> List[int]:\n    return []";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_037_dict_empty_typed() {
        let code = "from typing import Dict\ndef f() -> Dict[str, int]:\n    return {}";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_038_int_default_param() {
        let code = "def f(x: int = 0) -> int:\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_039_str_default_param() {
        let code = "def f(s: str = \"default\") -> str:\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_040_float_default_param() {
        let code = "def f(x: float = 0.0) -> float:\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_041_bool_default_param() {
        let code = "def f(flag: bool = False) -> bool:\n    return flag";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_042_complex_return_list_int() {
        let code =
            "from typing import List\ndef f(n: int) -> List[int]:\n    return list(range(n))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_043_complex_return_dict() {
        let code = "from typing import Dict\ndef f() -> Dict[str, int]:\n    return {\"a\": 1}";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_044_return_optional() {
        let code = "from typing import Optional\ndef f(x: int) -> Optional[int]:\n    if x > 0:\n        return x\n    return None";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_045_param_no_annotation() {
        let code = "def f(x):\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_046_no_return_annotation() {
        let code = "def f(x: int):\n    return x + 1";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_047_mixed_annotated_unannotated() {
        let code = "def f(x: int, y) -> int:\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_048_constant_int() {
        let code = "MAX_SIZE: int = 100\ndef f() -> int:\n    return MAX_SIZE";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_049_constant_str() {
        let code = "NAME: str = \"depyler\"\ndef f() -> str:\n    return NAME";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_050_constant_float() {
        let code = "RATE: float = 0.05\ndef f() -> float:\n    return RATE";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_051_set_int_literal() {
        let code = "def f() -> set:\n    return {1, 2, 3, 4, 5}";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_052_nested_dict() {
        let code = "def f() -> dict:\n    return {\"a\": {\"b\": 1}}";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_053_tuple_literal() {
        let code = "def f() -> tuple:\n    return (1, \"a\", True)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_054_list_str_literal() {
        let code = "def f() -> list:\n    return [\"a\", \"b\", \"c\"]";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("vec!"), "list str: {}", result);
    }

    #[test]
    fn test_w15rt_type_055_dict_str_str() {
        let code = "def f() -> dict:\n    return {\"key\": \"value\"}";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_056_list_float_literal() {
        let code = "def f() -> list:\n    return [1.0, 2.0, 3.0]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_057_empty_list() {
        let code = "def f() -> list:\n    return []";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_058_empty_dict() {
        let code = "def f() -> dict:\n    return {}";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_059_empty_set() {
        let code = "def f() -> set:\n    return set()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_type_060_empty_tuple() {
        let code = "def f() -> tuple:\n    return ()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 3: Complex Expression Patterns (60 tests)
    // ========================================================================

    #[test]
    fn test_w15rt_expr_001_nested_call_max_min() {
        let code = "def f(x: int) -> int:\n    return max(min(x, 10), 0)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("max") || result.contains("min"), "nested call: {}", result);
    }

    #[test]
    fn test_w15rt_expr_002_nested_call_len_sorted() {
        let code = "def f(items: list) -> int:\n    return len(sorted(items))";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("len"), "len sorted: {}", result);
    }

    #[test]
    fn test_w15rt_expr_003_method_chain_strip_split() {
        let code = "def f(s: str) -> list:\n    return s.strip().split(\",\")";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("trim") || result.contains("split"), "chain: {}", result);
    }

    #[test]
    fn test_w15rt_expr_004_method_upper() {
        let code = "def f(s: str) -> str:\n    return s.upper()";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("to_uppercase") || result.contains("to_ascii_uppercase"),
            "upper: {}",
            result
        );
    }

    #[test]
    fn test_w15rt_expr_005_method_lower() {
        let code = "def f(s: str) -> str:\n    return s.lower()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_expr_006_method_replace() {
        let code = "def f(s: str) -> str:\n    return s.replace(\"a\", \"b\")";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("replace"), "replace: {}", result);
    }

    #[test]
    fn test_w15rt_expr_007_method_startswith() {
        let code = "def f(s: str) -> bool:\n    return s.startswith(\"hello\")";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("starts_with"), "startswith: {}", result);
    }

    #[test]
    fn test_w15rt_expr_008_method_endswith() {
        let code = "def f(s: str) -> bool:\n    return s.endswith(\"world\")";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("ends_with"), "endswith: {}", result);
    }

    #[test]
    fn test_w15rt_expr_009_method_join() {
        let code = "def f(items: list) -> str:\n    return \",\".join(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("join"), "join: {}", result);
    }

    #[test]
    fn test_w15rt_expr_010_method_split() {
        let code = "def f(s: str) -> list:\n    return s.split(\",\")";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("split"), "split: {}", result);
    }

    #[test]
    fn test_w15rt_expr_011_multiple_return_tuple() {
        let code = "def f() -> tuple:\n    return 1, 2";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_expr_012_multiple_return_three() {
        let code = "def f(x: int, y: int, z: int) -> tuple:\n    return x, y, z";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_expr_013_global_constant_int() {
        let code = "MAX_SIZE = 100\ndef f() -> int:\n    return MAX_SIZE";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_expr_014_global_constant_float() {
        let code = "RATE = 0.05\ndef f() -> float:\n    return RATE";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_expr_015_global_constant_str() {
        let code = "GREETING = \"hello\"\ndef f() -> str:\n    return GREETING";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_expr_016_string_concat() {
        let code = "def f(a: str, b: str) -> str:\n    return a + b";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_expr_017_list_append() {
        let code = "def f() -> list:\n    items = [1, 2]\n    items.append(3)\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("push"), "append: {}", result);
    }

    #[test]
    fn test_w15rt_expr_018_list_extend() {
        let code =
            "def f() -> list:\n    items = [1, 2]\n    items.extend([3, 4])\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("extend"), "extend: {}", result);
    }

    #[test]
    fn test_w15rt_expr_019_list_pop() {
        let code = "def f(items: list) -> int:\n    return items.pop()";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("pop"), "pop: {}", result);
    }

    #[test]
    fn test_w15rt_expr_020_list_len() {
        let code = "def f(items: list) -> int:\n    return len(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("len"), "len: {}", result);
    }

    #[test]
    fn test_w15rt_expr_021_enumerate_basic() {
        let code =
            "def f(items: list) -> None:\n    for i, x in enumerate(items):\n        print(i, x)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("enumerate"), "enumerate: {}", result);
    }

    #[test]
    fn test_w15rt_expr_022_zip_basic() {
        let code =
            "def f(xs: list, ys: list) -> None:\n    for a, b in zip(xs, ys):\n        print(a, b)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("zip"), "zip: {}", result);
    }

    #[test]
    fn test_w15rt_expr_023_range_single() {
        let code = "def f() -> list:\n    return list(range(10))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_expr_024_range_start_stop() {
        let code = "def f() -> None:\n    for i in range(1, 10):\n        print(i)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_expr_025_range_step() {
        let code = "def f() -> None:\n    for i in range(0, 10, 2):\n        print(i)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_expr_026_range_reverse() {
        let code = "def f() -> None:\n    for i in range(10, 0, -1):\n        print(i)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_expr_027_sorted_call() {
        let code = "def f(items: list) -> list:\n    return sorted(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("sort") || result.contains("sorted"), "sorted: {}", result);
    }

    #[test]
    fn test_w15rt_expr_028_reversed_call() {
        let code = "def f(items: list) -> list:\n    return list(reversed(items))";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rev"), "reversed: {}", result);
    }

    #[test]
    fn test_w15rt_expr_029_abs_call() {
        let code = "def f(x: int) -> int:\n    return abs(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("abs"), "abs: {}", result);
    }

    #[test]
    fn test_w15rt_expr_030_max_call() {
        let code = "def f(a: int, b: int) -> int:\n    return max(a, b)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("max"), "max: {}", result);
    }

    #[test]
    fn test_w15rt_expr_031_min_call() {
        let code = "def f(a: int, b: int) -> int:\n    return min(a, b)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("min"), "min: {}", result);
    }

    #[test]
    fn test_w15rt_expr_032_sum_call() {
        let code = "def f(items: list) -> int:\n    return sum(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("sum") || result.contains("iter"), "sum: {}", result);
    }

    #[test]
    fn test_w15rt_expr_033_print_call() {
        let code = "def f() -> None:\n    print(\"hello world\")";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("println!") || result.contains("print"), "print: {}", result);
    }

    #[test]
    fn test_w15rt_expr_034_int_conversion() {
        let code = "def f(s: str) -> int:\n    return int(s)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("parse"), "int conv: {}", result);
    }

    #[test]
    fn test_w15rt_expr_035_str_conversion() {
        let code = "def f(x: int) -> str:\n    return str(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("to_string"), "str conv: {}", result);
    }

    #[test]
    fn test_w15rt_expr_036_float_conversion() {
        let code = "def f(x: int) -> float:\n    return float(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("f64") || result.contains("as"), "float conv: {}", result);
    }

    #[test]
    fn test_w15rt_expr_037_isinstance_call() {
        let code = "def f(x: int) -> bool:\n    return isinstance(x, int)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("true"), "isinstance: {}", result);
    }

    #[test]
    fn test_w15rt_expr_038_dict_get() {
        let code = "def f(d: dict, key: str) -> int:\n    return d.get(key, 0)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_expr_039_dict_keys_call() {
        let code = "def f(d: dict) -> list:\n    return list(d.keys())";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("keys"), "dict keys call: {}", result);
    }

    #[test]
    fn test_w15rt_expr_040_dict_values_call() {
        let code = "def f(d: dict) -> list:\n    return list(d.values())";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("values"), "dict values call: {}", result);
    }

    #[test]
    fn test_w15rt_expr_041_dict_items_call() {
        let code = "def f(d: dict) -> list:\n    return list(d.items())";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_expr_042_str_strip() {
        let code = "def f(s: str) -> str:\n    return s.strip()";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("trim"), "strip: {}", result);
    }

    #[test]
    fn test_w15rt_expr_043_str_lstrip() {
        let code = "def f(s: str) -> str:\n    return s.lstrip()";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("trim_start"), "lstrip: {}", result);
    }

    #[test]
    fn test_w15rt_expr_044_str_rstrip() {
        let code = "def f(s: str) -> str:\n    return s.rstrip()";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("trim_end"), "rstrip: {}", result);
    }

    #[test]
    fn test_w15rt_expr_045_str_find() {
        let code = "def f(s: str, sub: str) -> int:\n    return s.find(sub)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("find"), "find: {}", result);
    }

    #[test]
    fn test_w15rt_expr_046_str_count() {
        let code = "def f(s: str) -> int:\n    return s.count(\"a\")";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("match") || result.contains("count"), "count: {}", result);
    }

    #[test]
    fn test_w15rt_expr_047_str_isdigit() {
        let code = "def f(s: str) -> bool:\n    return s.isdigit()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_expr_048_str_isalpha() {
        let code = "def f(s: str) -> bool:\n    return s.isalpha()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_expr_049_ord_call() {
        let code = "def f(c: str) -> int:\n    return ord(c)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_expr_050_chr_call() {
        let code = "def f(n: int) -> str:\n    return chr(n)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_expr_051_complex_arithmetic() {
        let code = "def f(a: int, b: int, c: int) -> int:\n    return (a + b) * c - a";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_expr_052_nested_if_else() {
        let code = "def f(x: int) -> str:\n    if x > 0:\n        return \"positive\"\n    elif x < 0:\n        return \"negative\"\n    else:\n        return \"zero\"";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("if"), "nested if: {}", result);
    }

    #[test]
    fn test_w15rt_expr_053_modulo_operation() {
        let code = "def f(x: int, y: int) -> int:\n    return x % y";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("%"), "modulo: {}", result);
    }

    #[test]
    fn test_w15rt_expr_054_division() {
        let code = "def f(x: float, y: float) -> float:\n    return x / y";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("/"), "division: {}", result);
    }

    #[test]
    fn test_w15rt_expr_055_string_format_method() {
        let code = "def f(name: str) -> str:\n    return \"hello {}\".format(name)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("format!") || result.contains("format"),
            "format method: {}",
            result
        );
    }

    #[test]
    fn test_w15rt_expr_056_ternary_in_assign() {
        let code = "def f(x: int) -> int:\n    y = x if x > 0 else 0\n    return y";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_expr_057_list_index() {
        let code = "def f(items: list) -> int:\n    return items[0]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_expr_058_list_negative_index() {
        let code = "def f(items: list) -> int:\n    return items[-1]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_expr_059_list_slice() {
        let code = "def f(items: list) -> list:\n    return items[1:3]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15rt_expr_060_dict_access() {
        let code = "def f(d: dict, key: str) -> int:\n    return d[key]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }
}
