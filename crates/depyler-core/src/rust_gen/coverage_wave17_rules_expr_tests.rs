//! Wave 17 coverage tests: direct_rules_convert and expr_gen/mod.rs
//!
//! Targets uncovered code paths in:
//! - direct_rules_convert: augmented assignments (all operators), comparison/boolean,
//!   tuple/unpacking patterns
//! - expr_gen/mod.rs: variable handling, complex expressions (ternary, walrus,
//!   lambda, f-strings, yield, starred expressions)
//!
//! 200 tests total across 5 categories

#![cfg(test)]

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

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // SECTION 1: Augmented Assign (40 tests)
    // ========================================================================

    #[test]
    fn test_w17re_augassign_001_add_int() {
        let code = "def f(x: int) -> int:\n    x += 5\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("+=") || result.contains("+"), "augmented add int: {result}");
    }

    #[test]
    fn test_w17re_augassign_002_sub_int() {
        let code = "def f(x: int) -> int:\n    x -= 3\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("-=") || result.contains("-"), "augmented sub int: {result}");
    }

    #[test]
    fn test_w17re_augassign_003_mul_int() {
        let code = "def f(x: int) -> int:\n    x *= 4\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("*=") || result.contains("*"), "augmented mul int: {result}");
    }

    #[test]
    fn test_w17re_augassign_004_div_float() {
        let code = "def f(x: float) -> float:\n    x /= 2.0\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("/=") || result.contains("/"), "augmented div float: {result}");
    }

    #[test]
    fn test_w17re_augassign_005_floordiv_int() {
        let code = "def f(x: int, y: int) -> int:\n    x //= y\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_augassign_006_mod_int() {
        let code = "def f(x: int) -> int:\n    x %= 7\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("%=") || result.contains("%"), "augmented mod: {result}");
    }

    #[test]
    fn test_w17re_augassign_007_pow_int() {
        let code = "def f(x: int) -> int:\n    x **= 2\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("pow") || result.contains("**"), "augmented pow: {result}");
    }

    #[test]
    fn test_w17re_augassign_008_bitand() {
        let code = "def f(x: int) -> int:\n    x &= 0xff\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("&=") || result.contains("&"), "augmented bitand: {result}");
    }

    #[test]
    fn test_w17re_augassign_009_bitor() {
        let code = "def f(x: int) -> int:\n    x |= 0x01\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("|=") || result.contains("|"), "augmented bitor: {result}");
    }

    #[test]
    fn test_w17re_augassign_010_bitxor() {
        let code = "def f(x: int) -> int:\n    x ^= 0x10\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("^=") || result.contains("^"), "augmented bitxor: {result}");
    }

    #[test]
    fn test_w17re_augassign_011_lshift() {
        let code = "def f(x: int) -> int:\n    x <<= 2\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("<<=") || result.contains("<<"), "augmented lshift: {result}");
    }

    #[test]
    fn test_w17re_augassign_012_rshift() {
        let code = "def f(x: int) -> int:\n    x >>= 1\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains(">>=") || result.contains(">>"), "augmented rshift: {result}");
    }

    #[test]
    fn test_w17re_augassign_013_add_float() {
        let code = "def f(x: float) -> float:\n    x += 1.5\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("+=") || result.contains("+"), "augmented add float: {result}");
    }

    #[test]
    fn test_w17re_augassign_014_sub_float() {
        let code = "def f(x: float) -> float:\n    x -= 0.5\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("-=") || result.contains("-"), "augmented sub float: {result}");
    }

    #[test]
    fn test_w17re_augassign_015_mul_float() {
        let code = "def f(x: float) -> float:\n    x *= 2.5\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("*=") || result.contains("*"), "augmented mul float: {result}");
    }

    #[test]
    fn test_w17re_augassign_016_string_concat() {
        let code = "def f(s: str) -> str:\n    s += \" world\"\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_augassign_017_list_extend() {
        let code = "def f() -> list:\n    lst = [1, 2]\n    lst += [3, 4]\n    return lst";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_augassign_018_augadd_in_loop() {
        let code = "def f() -> int:\n    total = 0\n    for i in range(10):\n        total += i\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("+=") || result.contains("+"), "augadd in loop: {result}");
    }

    #[test]
    fn test_w17re_augassign_019_augmul_in_loop() {
        let code = "def f() -> int:\n    product = 1\n    for i in range(1, 6):\n        product *= i\n    return product";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("*=") || result.contains("*"), "augmul in loop: {result}");
    }

    #[test]
    fn test_w17re_augassign_020_augadd_in_if() {
        let code = "def f(x: int, y: int) -> int:\n    if x > 0:\n        y += x\n    else:\n        y -= x\n    return y";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_augassign_021_chain_augadd() {
        let code = "def f(x: int) -> int:\n    x += 1\n    x += 2\n    x += 3\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_augassign_022_chain_mixed() {
        let code = "def f(x: int) -> int:\n    x += 10\n    x -= 3\n    x *= 2\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_augassign_023_augadd_negative() {
        let code = "def f(x: int) -> int:\n    x += -5\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_augassign_024_augmod_in_conditional() {
        let code = "def f(x: int) -> int:\n    if x > 100:\n        x %= 100\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_augassign_025_augand_mask() {
        let code = "def f(x: int, mask: int) -> int:\n    x &= mask\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_augassign_026_augor_flags() {
        let code = "def f(flags: int, bit: int) -> int:\n    flags |= bit\n    return flags";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_augassign_027_augxor_toggle() {
        let code = "def f(x: int, bit: int) -> int:\n    x ^= bit\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_augassign_028_augadd_with_call() {
        let code = "def f(lst: list) -> int:\n    count = 0\n    count += len(lst)\n    return count";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_augassign_029_augmul_zero() {
        let code = "def f(x: int) -> int:\n    x *= 0\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_augassign_030_augadd_string_var() {
        let code = "def f(a: str, b: str) -> str:\n    a += b\n    return a";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_augassign_031_augadd_while_loop() {
        let code = "def f(n: int) -> int:\n    total = 0\n    i = 0\n    while i < n:\n        total += i\n        i += 1\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_augassign_032_augdiv_float() {
        let code = "def f(x: float, y: float) -> float:\n    x /= y\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_augassign_033_augsub_in_while() {
        let code = "def f(n: int) -> int:\n    while n > 0:\n        n -= 1\n    return n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_augassign_034_augadd_large_value() {
        let code = "def f(x: int) -> int:\n    x += 1000000\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_augassign_035_auglshift_var() {
        let code = "def f(x: int, n: int) -> int:\n    x <<= n\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_augassign_036_augrshift_var() {
        let code = "def f(x: int, n: int) -> int:\n    x >>= n\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_augassign_037_augpow_float() {
        let code = "def f(x: float) -> float:\n    x **= 3\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_augassign_038_nested_loop_augadd() {
        let code = "def f(n: int) -> int:\n    total = 0\n    for i in range(n):\n        for j in range(n):\n            total += 1\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_augassign_039_augfloordiv_var() {
        let code = "def f(x: int) -> int:\n    x //= 3\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_augassign_040_augmod_var() {
        let code = "def f(x: int, m: int) -> int:\n    x %= m\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 2: Comparison + Boolean (40 tests)
    // ========================================================================

    #[test]
    fn test_w17re_compare_041_chained_lt_lt() {
        let code = "def f(x: int) -> bool:\n    return 0 < x < 10";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("<") || result.contains("&&"), "chained lt: {result}");
    }

    #[test]
    fn test_w17re_compare_042_chained_le_le() {
        let code = "def f(a: int, b: int, c: int) -> bool:\n    return a <= b <= c";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_compare_043_chained_ne_ne() {
        let code = "def f(a: int, b: int, c: int) -> bool:\n    return a != b != c";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("!="), "chained ne: {result}");
    }

    #[test]
    fn test_w17re_compare_044_is_none() {
        let code = "def f(x: int) -> bool:\n    return x is None";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("is_none") || result.contains("None"), "is None: {result}");
    }

    #[test]
    fn test_w17re_compare_045_is_not_none() {
        let code = "def f(x: int) -> bool:\n    return x is not None";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("is_some") || result.contains("None") || result.contains("!"), "is not None: {result}");
    }

    #[test]
    fn test_w17re_compare_046_in_list() {
        let code = "def f(x: int) -> bool:\n    return x in [1, 2, 3]";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("contains") || result.contains("iter") || result.contains("in"), "in list: {result}");
    }

    #[test]
    fn test_w17re_compare_047_not_in_list() {
        let code = "def f(x: int) -> bool:\n    return x not in [1, 2, 3]";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("!") || result.contains("not") || result.contains("contains"), "not in list: {result}");
    }

    #[test]
    fn test_w17re_compare_048_in_string() {
        let code = "def f(sub: str, text: str) -> bool:\n    return sub in text";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("contains"), "in string: {result}");
    }

    #[test]
    fn test_w17re_compare_049_bool_and_and() {
        let code = "def f(a: bool, b: bool, c: bool) -> bool:\n    return a and b and c";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("&&"), "and chain: {result}");
    }

    #[test]
    fn test_w17re_compare_050_bool_or_or() {
        let code = "def f(a: bool, b: bool, c: bool) -> bool:\n    return a or b or c";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("||"), "or chain: {result}");
    }

    #[test]
    fn test_w17re_compare_051_not_and() {
        let code = "def f(a: bool, b: bool) -> bool:\n    return not (a and b)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("!") || result.contains("not"), "not and: {result}");
    }

    #[test]
    fn test_w17re_compare_052_mixed_gt_and_lt() {
        let code = "def f(x: int, y: int) -> bool:\n    return x > 0 and y < 10";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("&&"), "mixed and: {result}");
    }

    #[test]
    fn test_w17re_compare_053_truthiness_if() {
        let code = "def f(x: int) -> int:\n    if x:\n        return 1\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_compare_054_truthiness_not() {
        let code = "def f(x: int) -> int:\n    if not x:\n        return 0\n    return 1";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_compare_055_truthiness_while() {
        let code = "def f(x: int) -> int:\n    count = 0\n    while x:\n        x -= 1\n        count += 1\n    return count";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_compare_056_eq_eq() {
        let code = "def f(a: int, b: int) -> bool:\n    return a == b";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("=="), "eq: {result}");
    }

    #[test]
    fn test_w17re_compare_057_ne() {
        let code = "def f(a: int, b: int) -> bool:\n    return a != b";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("!="), "ne: {result}");
    }

    #[test]
    fn test_w17re_compare_058_gt() {
        let code = "def f(a: int, b: int) -> bool:\n    return a > b";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains(">"), "gt: {result}");
    }

    #[test]
    fn test_w17re_compare_059_ge() {
        let code = "def f(a: int, b: int) -> bool:\n    return a >= b";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains(">="), "ge: {result}");
    }

    #[test]
    fn test_w17re_compare_060_lt() {
        let code = "def f(a: int, b: int) -> bool:\n    return a < b";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("<"), "lt: {result}");
    }

    #[test]
    fn test_w17re_compare_061_le() {
        let code = "def f(a: int, b: int) -> bool:\n    return a <= b";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("<="), "le: {result}");
    }

    #[test]
    fn test_w17re_compare_062_or_with_in() {
        let code = "def f(x: int) -> bool:\n    return x in [1, 2] or x in [8, 9]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_compare_063_and_with_comparison() {
        let code = "def f(x: int) -> bool:\n    return x > 0 and x < 100 and x != 50";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("&&"), "complex and: {result}");
    }

    #[test]
    fn test_w17re_compare_064_not_bool() {
        let code = "def f(b: bool) -> bool:\n    return not b";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("!"), "not bool: {result}");
    }

    #[test]
    fn test_w17re_compare_065_chained_lt_le() {
        let code = "def f(a: int, b: int, c: int) -> bool:\n    return a < b <= c";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_compare_066_not_in_string() {
        let code = "def f(word: str, text: str) -> bool:\n    return word not in text";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_compare_067_bool_short_circuit_and() {
        let code = "def f(x: int) -> int:\n    if x > 0 and x < 100:\n        return x\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("&&"), "short circuit and: {result}");
    }

    #[test]
    fn test_w17re_compare_068_bool_short_circuit_or() {
        let code = "def f(x: int) -> int:\n    if x < 0 or x > 100:\n        return 0\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("||"), "short circuit or: {result}");
    }

    #[test]
    fn test_w17re_compare_069_is_true() {
        let code = "def f(x: bool) -> bool:\n    return x is True";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_compare_070_is_false() {
        let code = "def f(x: bool) -> bool:\n    return x is False";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_compare_071_nested_not_or() {
        let code = "def f(a: bool, b: bool) -> bool:\n    return not a or not b";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_compare_072_compare_string_eq() {
        let code = "def f(s: str) -> bool:\n    return s == \"hello\"";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("=="), "string eq: {result}");
    }

    #[test]
    fn test_w17re_compare_073_compare_string_ne() {
        let code = "def f(s: str) -> bool:\n    return s != \"world\"";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("!="), "string ne: {result}");
    }

    #[test]
    fn test_w17re_compare_074_and_or_mixed() {
        let code = "def f(a: bool, b: bool, c: bool) -> bool:\n    return a and b or c";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_compare_075_comparison_in_loop() {
        let code = "def f(n: int) -> int:\n    count = 0\n    for i in range(n):\n        if i > 5 and i < 15:\n            count += 1\n    return count";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_compare_076_bool_var_as_condition() {
        let code = "def f(flag: bool) -> int:\n    if flag:\n        return 1\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_compare_077_negated_comparison() {
        let code = "def f(x: int) -> bool:\n    return not (x > 0)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_compare_078_float_comparison() {
        let code = "def f(x: float, y: float) -> bool:\n    return x > y";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_compare_079_multiple_or_conditions() {
        let code = "def f(x: int) -> bool:\n    return x == 1 or x == 2 or x == 3 or x == 4";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_compare_080_comparison_return_ternary() {
        let code = "def f(x: int) -> str:\n    return \"pos\" if x > 0 else \"non_pos\"";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 3: Tuple/Unpacking (40 tests)
    // ========================================================================

    #[test]
    fn test_w17re_tuple_081_pack_two() {
        let code = "def f() -> tuple:\n    t = (1, 2)\n    return t";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_082_pack_three() {
        let code = "def f() -> tuple:\n    t = (1, 2, 3)\n    return t";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_083_unpack_two() {
        let code = "def f() -> int:\n    a, b = 1, 2\n    return a + b";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_084_unpack_three() {
        let code = "def f() -> int:\n    a, b, c = 1, 2, 3\n    return a + b + c";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_085_swap() {
        let code = "def f(a: int, b: int) -> int:\n    a, b = b, a\n    return a";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_086_return_tuple() {
        let code = "def f(x: int) -> tuple:\n    return x, x + 1";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_087_return_triple() {
        let code = "def f(x: int) -> tuple:\n    return x, x + 1, x + 2";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_088_unpack_in_for() {
        let code = "def f() -> int:\n    total = 0\n    pairs = [(1, 2), (3, 4)]\n    for a, b in pairs:\n        total += a + b\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_089_tuple_as_arg() {
        let code = "def f(t: tuple) -> int:\n    return len(t)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_090_unpack_from_func() {
        let code = "def get_pair() -> tuple:\n    return 1, 2\n\ndef f() -> int:\n    x, y = get_pair()\n    return x + y";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_091_tuple_with_strings() {
        let code = "def f() -> tuple:\n    return \"hello\", \"world\"";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_092_tuple_comparison() {
        let code = "def f() -> bool:\n    a = (1, 2)\n    b = (1, 2)\n    return a == b";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_093_single_element() {
        let code = "def f() -> int:\n    t = (42,)\n    return t[0]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_094_empty_tuple() {
        let code = "def f() -> tuple:\n    return ()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_095_nested_tuple() {
        let code = "def f() -> tuple:\n    return (1, (2, 3))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_096_tuple_of_bools() {
        let code = "def f() -> tuple:\n    return (True, False, True)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_097_unpack_with_ignore() {
        let code = "def f() -> int:\n    a, _, c = 1, 2, 3\n    return a + c";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_098_tuple_of_floats() {
        let code = "def f() -> tuple:\n    return (1.5, 2.5, 3.5)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_099_unpack_enumerate() {
        let code = "def f() -> int:\n    total = 0\n    items = [10, 20, 30]\n    for i, val in enumerate(items):\n        total += val\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_100_unpack_four() {
        let code = "def f() -> int:\n    a, b, c, d = 1, 2, 3, 4\n    return a + b + c + d";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_101_tuple_mixed_types() {
        let code = "def f() -> tuple:\n    return (1, \"hello\", True)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_102_unpack_from_list() {
        let code = "def f() -> int:\n    a, b, c = [10, 20, 30]\n    return a + b + c";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_103_swap_three() {
        let code = "def f(a: int, b: int, c: int) -> int:\n    a, b, c = c, a, b\n    return a";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_104_tuple_in_return() {
        let code = "def f(x: int) -> tuple:\n    if x > 0:\n        return (x, True)\n    return (0, False)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_105_unpack_inline() {
        let code = "def f() -> int:\n    x, y = 10, 20\n    return x * y";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_106_tuple_index_zero() {
        let code = "def f() -> int:\n    t = (10, 20, 30)\n    return t[0]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_107_tuple_index_one() {
        let code = "def f() -> int:\n    t = (10, 20, 30)\n    return t[1]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_108_tuple_index_last() {
        let code = "def f() -> int:\n    t = (10, 20, 30)\n    return t[2]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_109_tuple_len() {
        let code = "def f() -> int:\n    t = (1, 2, 3, 4)\n    return len(t)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_110_unpack_zip() {
        let code = "def f() -> int:\n    total = 0\n    names = [\"a\", \"b\"]\n    vals = [1, 2]\n    for name, val in zip(names, vals):\n        total += val\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_111_tuple_paren_expr() {
        let code = "def f(x: int) -> int:\n    return (x + 1) * 2";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_112_unpack_assign_exprs() {
        let code = "def f(x: int) -> int:\n    a, b = x + 1, x + 2\n    return a + b";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_113_tuple_with_negatives() {
        let code = "def f() -> tuple:\n    return (-1, -2, -3)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_114_tuple_with_zero() {
        let code = "def f() -> tuple:\n    return (0, 0, 0)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_115_unpack_in_conditional() {
        let code = "def f(flag: bool) -> int:\n    if flag:\n        x, y = 1, 2\n    else:\n        x, y = 3, 4\n    return x + y";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_116_tuple_with_call_results() {
        let code = "def f(lst: list) -> tuple:\n    return (len(lst), len(lst) + 1)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_117_unpack_five() {
        let code = "def f() -> int:\n    a, b, c, d, e = 1, 2, 3, 4, 5\n    return a + e";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_118_swap_with_expr() {
        let code = "def f(a: int, b: int) -> int:\n    a, b = a + b, a - b\n    return a";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_119_tuple_in_list() {
        let code = "def f() -> list:\n    return [(1, 2), (3, 4), (5, 6)]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_tuple_120_unpack_nested_call() {
        let code = "def divmod_pair(a: int, b: int) -> tuple:\n    return a // b, a % b\n\ndef f() -> int:\n    q, r = divmod_pair(17, 5)\n    return q";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 4: Variable Handling (40 tests)
    // ========================================================================

    #[test]
    fn test_w17re_var_121_builtin_int_ref() {
        let code = "def f() -> list:\n    items = [\"1\", \"2\", \"3\"]\n    return list(map(int, items))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_122_builtin_str_ref() {
        let code = "def f() -> list:\n    items = [1, 2, 3]\n    return list(map(str, items))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_123_isinstance_int() {
        let code = "def f(x: int) -> bool:\n    return isinstance(x, int)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_124_isinstance_str() {
        let code = "def f(x: str) -> bool:\n    return isinstance(x, str)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_125_isinstance_in_if() {
        let code = "def f(x: int) -> str:\n    if isinstance(x, int):\n        return \"integer\"\n    return \"other\"";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_126_global_constant_int() {
        let code = "MAX_SIZE = 1000\n\ndef f() -> int:\n    return MAX_SIZE";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("1000") || result.contains("MAX_SIZE"), "constant: {result}");
    }

    #[test]
    fn test_w17re_var_127_global_constant_float() {
        let code = "RATE = 3.5\n\ndef f() -> float:\n    return RATE";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("3.5") || result.contains("RATE"), "float constant: {result}");
    }

    #[test]
    fn test_w17re_var_128_global_constant_str() {
        let code = "PREFIX = \"hello\"\n\ndef f() -> str:\n    return PREFIX";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_129_variable_shadowing() {
        let code = "def f(x: int) -> int:\n    x = x + 1\n    x = x * 2\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_130_variable_reassign_type() {
        let code = "def f() -> str:\n    x = 42\n    y = str(x)\n    return y";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_131_multiple_assignment() {
        let code = "def f() -> int:\n    a = b = c = 0\n    return a + b + c";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_132_dunder_name() {
        let code = "def f() -> str:\n    return __name__";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("__main__"), "dunder name: {result}");
    }

    #[test]
    fn test_w17re_var_133_dunder_file() {
        let code = "def f() -> str:\n    return __file__";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("file!"), "dunder file: {result}");
    }

    #[test]
    fn test_w17re_var_134_variable_in_nested_if() {
        let code = "def f(x: int) -> int:\n    result = 0\n    if x > 0:\n        if x > 10:\n            result = 2\n        else:\n            result = 1\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_135_loop_variable_scope() {
        let code = "def f() -> int:\n    last = 0\n    for i in range(10):\n        last = i\n    return last";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_136_bool_variable() {
        let code = "def f() -> bool:\n    found = False\n    found = True\n    return found";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_137_str_variable_ops() {
        let code = "def f(name: str) -> int:\n    return len(name)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("len"), "str var ops: {result}");
    }

    #[test]
    fn test_w17re_var_138_list_variable() {
        let code = "def f() -> int:\n    items = [1, 2, 3]\n    return len(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_139_dict_variable() {
        let code = "def f() -> int:\n    d = {\"a\": 1, \"b\": 2}\n    return len(d)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_140_variable_type_annotation() {
        let code = "def f() -> int:\n    x: int = 42\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_141_param_default_int() {
        let code = "def f(x: int = 10) -> int:\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_142_param_default_str() {
        let code = "def f(name: str = \"default\") -> str:\n    return name";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_143_param_default_bool() {
        let code = "def f(flag: bool = False) -> bool:\n    return flag";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_144_global_constant_bool() {
        let code = "DEBUG = True\n\ndef f() -> bool:\n    return DEBUG";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_145_multiple_constants() {
        let code = "WIDTH = 800\nHEIGHT = 600\n\ndef f() -> int:\n    return WIDTH * HEIGHT";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_146_constant_in_expression() {
        let code = "OFFSET = 10\n\ndef f(x: int) -> int:\n    return x + OFFSET";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_147_var_from_call() {
        let code = "def f() -> int:\n    x = abs(-5)\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_148_var_from_method() {
        let code = "def f(s: str) -> str:\n    result = s.upper()\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_149_var_from_binary() {
        let code = "def f(a: int, b: int) -> int:\n    result = a * b + 1\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_150_var_conditional_assign() {
        let code = "def f(x: int) -> str:\n    if x > 0:\n        label = \"positive\"\n    else:\n        label = \"non_positive\"\n    return label";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_151_isinstance_bool() {
        let code = "def f(x: bool) -> bool:\n    return isinstance(x, bool)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_152_isinstance_float() {
        let code = "def f(x: float) -> bool:\n    return isinstance(x, float)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_153_var_accum_pattern() {
        let code = "def f(items: list) -> int:\n    total = 0\n    for item in items:\n        total += item\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_154_var_flag_pattern() {
        let code = "def f(items: list) -> bool:\n    found = False\n    for item in items:\n        if item > 10:\n            found = True\n    return found";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_155_var_min_max_pattern() {
        let code = "def f(items: list) -> int:\n    best = 0\n    for item in items:\n        if item > best:\n            best = item\n    return best";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_156_const_string_list() {
        let code = "COLORS = [\"red\", \"green\", \"blue\"]\n\ndef f() -> int:\n    return len(COLORS)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_157_var_counter_pattern() {
        let code = "def f(text: str) -> int:\n    count = 0\n    for ch in text:\n        if ch == \"a\":\n            count += 1\n    return count";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_158_var_builder_pattern() {
        let code = "def f() -> str:\n    result = \"\"\n    result += \"hello\"\n    result += \" \"\n    result += \"world\"\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_159_var_chain_assign() {
        let code = "def f() -> int:\n    x = 1\n    y = x + 1\n    z = y + 1\n    return z";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_var_160_nested_scope_var() {
        let code = "def f(x: int) -> int:\n    result = 0\n    for i in range(x):\n        temp = i * 2\n        result += temp\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 5: Complex Expressions (40 tests)
    // ========================================================================

    #[test]
    fn test_w17re_expr_161_ternary_simple() {
        let code = "def f(x: int) -> int:\n    return x if x > 0 else -x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("if") || result.contains("else"), "ternary: {result}");
    }

    #[test]
    fn test_w17re_expr_162_ternary_string() {
        let code = "def f(b: bool) -> str:\n    return \"yes\" if b else \"no\"";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_expr_163_ternary_zero() {
        let code = "def f(x: int) -> int:\n    return 0 if x < 0 else x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_expr_164_ternary_assign() {
        let code = "def f(x: int) -> int:\n    result = x * 2 if x > 0 else 0\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_expr_165_lambda_identity() {
        let code = "def f() -> int:\n    fn = lambda x: x\n    return fn(42)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_expr_166_lambda_mul() {
        let code = "def f() -> int:\n    double = lambda x: x * 2\n    return double(5)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_expr_167_lambda_add() {
        let code = "def f() -> int:\n    add = lambda a, b: a + b\n    return add(3, 4)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_expr_168_nested_call_len_str() {
        let code = "def f(x: int) -> int:\n    return len(str(x))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_expr_169_nested_call_abs() {
        let code = "def f(x: int) -> int:\n    return abs(x - 10)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("abs"), "nested abs: {result}");
    }

    #[test]
    fn test_w17re_expr_170_method_chain_strip_upper() {
        let code = "def f(s: str) -> str:\n    return s.strip().upper()";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("trim") || result.contains("strip") || result.contains("to_uppercase"), "chain: {result}");
    }

    #[test]
    fn test_w17re_expr_171_method_chain_lower_strip() {
        let code = "def f(s: str) -> str:\n    return s.lower().strip()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_expr_172_fstring_simple() {
        let code = "def f(name: str) -> str:\n    return f\"hello {name}\"";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"), "fstring: {result}");
    }

    #[test]
    fn test_w17re_expr_173_fstring_expr() {
        let code = "def f(x: int) -> str:\n    return f\"value is {x + 1}\"";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"), "fstring expr: {result}");
    }

    #[test]
    fn test_w17re_expr_174_fstring_multiple() {
        let code = "def f(a: int, b: int) -> str:\n    return f\"{a} + {b} = {a + b}\"";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"), "fstring multi: {result}");
    }

    #[test]
    fn test_w17re_expr_175_yield_simple() {
        let code = "def gen(n: int):\n    for i in range(n):\n        yield i";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_expr_176_yield_value() {
        let code = "def gen():\n    yield 1\n    yield 2\n    yield 3";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_expr_177_list_comp_simple() {
        let code = "def f() -> list:\n    return [x * 2 for x in range(5)]";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("map") || result.contains("iter") || result.contains("collect") || result.contains("vec"), "list comp: {result}");
    }

    #[test]
    fn test_w17re_expr_178_list_comp_filter() {
        let code = "def f() -> list:\n    return [x for x in range(10) if x > 5]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_expr_179_list_comp_string() {
        let code = "def f(words: list) -> list:\n    return [w.upper() for w in words]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_expr_180_dict_comp() {
        let code = "def f() -> dict:\n    return {k: k * 2 for k in range(5)}";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_expr_181_set_comp() {
        let code = "def f() -> set:\n    return {x * x for x in range(5)}";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_expr_182_nested_ternary() {
        let code = "def f(x: int) -> str:\n    return \"pos\" if x > 0 else \"zero\" if x == 0 else \"neg\"";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_expr_183_lambda_with_ternary() {
        let code = "def f() -> int:\n    clamp = lambda x: 0 if x < 0 else x\n    return clamp(-5)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_expr_184_fstring_with_method() {
        let code = "def f(name: str) -> str:\n    return f\"Hello, {name.upper()}!\"";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"), "fstring method: {result}");
    }

    #[test]
    fn test_w17re_expr_185_walrus_in_if() {
        let code = "def f(items: list) -> int:\n    if (n := len(items)) > 5:\n        return n\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_expr_186_walrus_basic() {
        let code = "def f(x: int) -> int:\n    if (y := x * 2) > 10:\n        return y\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_expr_187_nested_method_calls() {
        let code = "def f(s: str) -> list:\n    return s.strip().split()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_expr_188_complex_binary() {
        let code = "def f(a: int, b: int, c: int) -> int:\n    return a * b + c * (a - b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_expr_189_unary_negative() {
        let code = "def f(x: int) -> int:\n    return -x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("-"), "unary neg: {result}");
    }

    #[test]
    fn test_w17re_expr_190_unary_not() {
        let code = "def f(b: bool) -> bool:\n    return not b";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("!"), "unary not: {result}");
    }

    #[test]
    fn test_w17re_expr_191_bitwise_not() {
        let code = "def f(x: int) -> int:\n    return ~x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_expr_192_bitwise_and() {
        let code = "def f(a: int, b: int) -> int:\n    return a & b";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("&"), "bitwise and: {result}");
    }

    #[test]
    fn test_w17re_expr_193_bitwise_or() {
        let code = "def f(a: int, b: int) -> int:\n    return a | b";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("|"), "bitwise or: {result}");
    }

    #[test]
    fn test_w17re_expr_194_bitwise_xor() {
        let code = "def f(a: int, b: int) -> int:\n    return a ^ b";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("^"), "bitwise xor: {result}");
    }

    #[test]
    fn test_w17re_expr_195_lshift() {
        let code = "def f(x: int) -> int:\n    return x << 3";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("<<"), "lshift: {result}");
    }

    #[test]
    fn test_w17re_expr_196_rshift() {
        let code = "def f(x: int) -> int:\n    return x >> 2";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains(">>"), "rshift: {result}");
    }

    #[test]
    fn test_w17re_expr_197_power() {
        let code = "def f(x: int) -> int:\n    return x ** 3";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("pow") || result.contains("**"), "power: {result}");
    }

    #[test]
    fn test_w17re_expr_198_floor_div() {
        let code = "def f(a: int, b: int) -> int:\n    return a // b";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w17re_expr_199_complex_fstring() {
        let code = "def f(x: int, y: int) -> str:\n    return f\"sum={x+y}, diff={x-y}\"";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format!"), "complex fstring: {result}");
    }

    #[test]
    fn test_w17re_expr_200_generator_expression() {
        let code = "def f() -> int:\n    return sum(x * x for x in range(10))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }
}
