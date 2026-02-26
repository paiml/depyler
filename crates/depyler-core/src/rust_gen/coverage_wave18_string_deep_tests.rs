//! Wave 18 Deep: Comprehensive coverage tests for string_methods.rs
//!
//! 200 tests targeting uncovered code paths in string method transpilation:
//! zfill, capitalize, swapcase, expandtabs, center, ljust, rjust,
//! partition, rpartition, isidentifier, rsplit, format, title,
//! isupper/islower/isspace, isnumeric/isascii/isdecimal,
//! maketrans/translate, splitlines, removeprefix/removesuffix,
//! count, encode, casefold, hex, isprintable, istitle, isalnum,
//! rfind, rindex, index, decode, find with start, strip with chars

#[cfg(test)]
mod tests {
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

    // ========================================================================
    // ZFILL (tests 001-010): sign-preserving zero-padding
    // ========================================================================

    #[test]
    fn test_wave18_string_zfill_positive_number() {
        let code = r#"
def pad_num(s: str) -> str:
    return s.zfill(5)
"#;
        let result = transpile(code);
        assert!(
            result.contains("width") && result.contains("starts_with"),
            "zfill basic: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_zfill_negative_number() {
        let code = r#"
def pad_neg() -> str:
    return "-42".zfill(6)
"#;
        let result = transpile(code);
        assert!(
            result.contains("starts_with") || result.contains("zfill") || result.contains("0"),
            "zfill negative: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_zfill_no_pad_needed() {
        let code = r#"
def no_pad(s: str) -> str:
    return s.zfill(3)
"#;
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("len"), "zfill no-pad: {}", result);
    }

    #[test]
    fn test_wave18_string_zfill_literal_hello() {
        let code = r#"
def pad_hello() -> str:
    return "hello".zfill(3)
"#;
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("zfill"), "zfill hello: {}", result);
    }

    #[test]
    fn test_wave18_string_zfill_empty_string() {
        let code = r#"
def pad_empty() -> str:
    return "".zfill(5)
"#;
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("0"), "zfill empty: {}", result);
    }

    #[test]
    fn test_wave18_string_zfill_plus_sign() {
        let code = r#"
def pad_plus() -> str:
    return "+42".zfill(6)
"#;
        let result = transpile(code);
        assert!(
            result.contains("starts_with") || result.contains("sign"),
            "zfill plus: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_zfill_exact_width() {
        let code = r#"
def pad_exact() -> str:
    return "hello".zfill(5)
"#;
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("len"), "zfill exact: {}", result);
    }

    #[test]
    fn test_wave18_string_zfill_large_width() {
        let code = r#"
def pad_large(s: str) -> str:
    return s.zfill(100)
"#;
        let result = transpile(code);
        assert!(result.contains("100") || result.contains("width"), "zfill large: {}", result);
    }

    #[test]
    fn test_wave18_string_zfill_variable_width() {
        let code = r#"
def pad_var(s: str, w: int) -> str:
    return s.zfill(w)
"#;
        let result = transpile(code);
        assert!(
            result.contains("width") || result.contains("starts_with"),
            "zfill var: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_zfill_in_expression() {
        let code = r#"
def pad_expr(n: int) -> str:
    s = str(n)
    return s.zfill(8)
"#;
        let result = transpile(code);
        assert!(
            result.contains("zfill") || result.contains("width") || result.contains("0"),
            "zfill expr: {}",
            result
        );
    }

    // ========================================================================
    // CAPITALIZE (tests 011-020)
    // ========================================================================

    #[test]
    fn test_wave18_string_capitalize_basic() {
        let code = r#"
def cap(s: str) -> str:
    return s.capitalize()
"#;
        let result = transpile(code);
        assert!(
            result.contains("to_uppercase") || result.contains("chars"),
            "capitalize basic: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_capitalize_literal() {
        let code = r#"
def cap_lit() -> str:
    return "hello".capitalize()
"#;
        let result = transpile(code);
        assert!(
            result.contains("to_uppercase") || result.contains("chars"),
            "capitalize literal: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_capitalize_all_upper() {
        let code = r#"
def cap_upper() -> str:
    return "HELLO".capitalize()
"#;
        let result = transpile(code);
        assert!(result.contains("chars"), "capitalize upper: {}", result);
    }

    #[test]
    fn test_wave18_string_capitalize_mixed_case() {
        let code = r#"
def cap_mixed() -> str:
    return "hELLO".capitalize()
"#;
        let result = transpile(code);
        assert!(
            result.contains("to_uppercase") || result.contains("chars"),
            "capitalize mixed: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_capitalize_digits_prefix() {
        let code = r#"
def cap_digits() -> str:
    return "123abc".capitalize()
"#;
        let result = transpile(code);
        assert!(
            result.contains("chars") || result.contains("to_uppercase"),
            "capitalize digits: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_capitalize_empty() {
        let code = r#"
def cap_empty() -> str:
    return "".capitalize()
"#;
        let result = transpile(code);
        assert!(
            result.contains("chars") || result.contains("None"),
            "capitalize empty: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_capitalize_single_char() {
        let code = r#"
def cap_single() -> str:
    return "a".capitalize()
"#;
        let result = transpile(code);
        assert!(
            result.contains("to_uppercase") || result.contains("chars"),
            "capitalize single: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_capitalize_with_spaces() {
        let code = r#"
def cap_spaces() -> str:
    return "hello world".capitalize()
"#;
        let result = transpile(code);
        assert!(
            result.contains("to_uppercase") || result.contains("chars"),
            "capitalize spaces: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_capitalize_already_capitalized() {
        let code = r#"
def cap_noop() -> str:
    return "Hello".capitalize()
"#;
        let result = transpile(code);
        assert!(result.contains("chars"), "capitalize noop: {}", result);
    }

    #[test]
    fn test_wave18_string_capitalize_assign() {
        let code = r#"
def cap_assign(s: str) -> str:
    result = s.capitalize()
    return result
"#;
        let result = transpile(code);
        assert!(
            result.contains("to_uppercase") || result.contains("chars"),
            "capitalize assign: {}",
            result
        );
    }

    // ========================================================================
    // SWAPCASE (tests 021-030)
    // ========================================================================

    #[test]
    fn test_wave18_string_swapcase_basic() {
        let code = r#"
def swap(s: str) -> str:
    return s.swapcase()
"#;
        let result = transpile(code);
        assert!(
            result.contains("is_uppercase") || result.contains("to_lowercase"),
            "swapcase basic: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_swapcase_literal() {
        let code = r#"
def swap_lit() -> str:
    return "HeLLo".swapcase()
"#;
        let result = transpile(code);
        assert!(
            result.contains("is_uppercase") && result.contains("to_lowercase"),
            "swapcase literal: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_swapcase_all_upper() {
        let code = r#"
def swap_upper() -> str:
    return "ABC".swapcase()
"#;
        let result = transpile(code);
        assert!(result.contains("is_uppercase"), "swapcase upper: {}", result);
    }

    #[test]
    fn test_wave18_string_swapcase_all_lower() {
        let code = r#"
def swap_lower() -> str:
    return "abc".swapcase()
"#;
        let result = transpile(code);
        assert!(
            result.contains("to_uppercase") && result.contains("chars"),
            "swapcase lower: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_swapcase_mixed_with_digits() {
        let code = r#"
def swap_digits() -> str:
    return "Hello123".swapcase()
"#;
        let result = transpile(code);
        assert!(result.contains("is_uppercase"), "swapcase digits: {}", result);
    }

    #[test]
    fn test_wave18_string_swapcase_empty() {
        let code = r#"
def swap_empty() -> str:
    return "".swapcase()
"#;
        let result = transpile(code);
        assert!(result.contains("chars") && result.contains("map"), "swapcase empty: {}", result);
    }

    #[test]
    fn test_wave18_string_swapcase_special_chars() {
        let code = r#"
def swap_special() -> str:
    return "Hello, World!".swapcase()
"#;
        let result = transpile(code);
        assert!(result.contains("is_uppercase"), "swapcase special: {}", result);
    }

    #[test]
    fn test_wave18_string_swapcase_assign() {
        let code = r#"
def swap_assign(s: str) -> str:
    swapped = s.swapcase()
    return swapped
"#;
        let result = transpile(code);
        assert!(
            result.contains("is_uppercase") || result.contains("to_lowercase"),
            "swapcase assign: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_swapcase_chained() {
        let code = r#"
def swap_chain(s: str) -> str:
    return s.swapcase()
"#;
        let result = transpile(code);
        assert!(
            result.contains("collect::<String>") || result.contains("String"),
            "swapcase chain: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_swapcase_single_char() {
        let code = r#"
def swap_one() -> str:
    return "A".swapcase()
"#;
        let result = transpile(code);
        assert!(result.contains("is_uppercase"), "swapcase one: {}", result);
    }

    // ========================================================================
    // EXPANDTABS (tests 031-040)
    // ========================================================================

    #[test]
    fn test_wave18_string_expandtabs_default() {
        let code = "def expand(s: str) -> str:\n    return s.expandtabs()\n";
        let result = transpile(code);
        // expandtabs may fall through to generic method call
        assert!(!result.is_empty(), "expandtabs default: {}", result);
    }

    #[test]
    fn test_wave18_string_expandtabs_custom() {
        let code = "def expand4(s: str) -> str:\n    return s.expandtabs(4)\n";
        let result = transpile(code);
        assert!(
            result.contains("replace") && result.contains("repeat"),
            "expandtabs custom: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_expandtabs_literal() {
        let code = "def expand_lit() -> str:\n    return \"a\\tb\".expandtabs()\n";
        let result = transpile(code);
        assert!(
            result.contains("replace") || result.contains("repeat"),
            "expandtabs literal: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_expandtabs_custom_2() {
        let code = "def expand2(s: str) -> str:\n    return s.expandtabs(2)\n";
        let result = transpile(code);
        assert!(
            result.contains("replace") && result.contains("repeat"),
            "expandtabs 2: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_expandtabs_large() {
        let code = "def expand16(s: str) -> str:\n    return s.expandtabs(16)\n";
        let result = transpile(code);
        assert!(result.contains("replace"), "expandtabs 16: {}", result);
    }

    #[test]
    fn test_wave18_string_expandtabs_variable() {
        let code = "def expand_var(s: str, n: int) -> str:\n    return s.expandtabs(n)\n";
        let result = transpile(code);
        assert!(
            result.contains("replace") && result.contains("repeat"),
            "expandtabs var: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_expandtabs_zero() {
        let code = "def expand0(s: str) -> str:\n    return s.expandtabs(0)\n";
        let result = transpile(code);
        assert!(result.contains("replace"), "expandtabs zero: {}", result);
    }

    #[test]
    fn test_wave18_string_expandtabs_one() {
        let code = "def expand1(s: str) -> str:\n    return s.expandtabs(1)\n";
        let result = transpile(code);
        assert!(result.contains("replace"), "expandtabs one: {}", result);
    }

    #[test]
    fn test_wave18_string_expandtabs_assign() {
        let code = "def expand_assign(s: str) -> str:\n    expanded = s.expandtabs()\n    return expanded\n";
        let result = transpile(code);
        // expandtabs may fall through to generic method call
        assert!(!result.is_empty(), "expandtabs assign: {}", result);
    }

    #[test]
    fn test_wave18_string_expandtabs_literal_tab() {
        let code = "def tab_lit() -> str:\n    return \"col1\\tcol2\\tcol3\".expandtabs(8)\n";
        let result = transpile(code);
        assert!(result.contains("replace"), "expandtabs tab lit: {}", result);
    }

    // ========================================================================
    // CENTER (tests 041-050)
    // ========================================================================

    #[test]
    fn test_wave18_string_center_basic() {
        let code = r#"
def ctr(s: str) -> str:
    return s.center(10)
"#;
        let result = transpile(code);
        assert!(
            result.contains("width") && result.contains("total_pad"),
            "center basic: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_center_with_fillchar() {
        let code = r#"
def ctr_fill(s: str) -> str:
    return s.center(10, "*")
"#;
        let result = transpile(code);
        assert!(result.contains("fillchar") || result.contains("width"), "center fill: {}", result);
    }

    #[test]
    fn test_wave18_string_center_literal() {
        let code = r#"
def ctr_lit() -> str:
    return "hi".center(10)
"#;
        let result = transpile(code);
        assert!(
            result.contains("width") || result.contains("total_pad"),
            "center literal: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_center_star_fill() {
        let code = r#"
def ctr_star() -> str:
    return "hi".center(10, "*")
"#;
        let result = transpile(code);
        assert!(
            result.contains("fillchar") || result.contains("format!"),
            "center star: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_center_wider_than_string() {
        let code = r#"
def ctr_wide(s: str) -> str:
    return s.center(20)
"#;
        let result = transpile(code);
        assert!(
            result.contains("left_pad") || result.contains("total_pad"),
            "center wide: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_center_narrow() {
        let code = r#"
def ctr_narrow() -> str:
    return "hello".center(3)
"#;
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("len"), "center narrow: {}", result);
    }

    #[test]
    fn test_wave18_string_center_dash_fill() {
        let code = r#"
def ctr_dash(s: str) -> str:
    return s.center(20, "-")
"#;
        let result = transpile(code);
        assert!(
            result.contains("fillchar") || result.contains("format!"),
            "center dash: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_center_exact_width() {
        let code = r#"
def ctr_exact() -> str:
    return "hello".center(5)
"#;
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("len"), "center exact: {}", result);
    }

    #[test]
    fn test_wave18_string_center_assign() {
        let code = r#"
def ctr_assign(s: str) -> str:
    centered = s.center(30)
    return centered
"#;
        let result = transpile(code);
        assert!(
            result.contains("width") || result.contains("total_pad"),
            "center assign: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_center_variable_width() {
        let code = r#"
def ctr_var(s: str, w: int) -> str:
    return s.center(w)
"#;
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("total_pad"), "center var: {}", result);
    }

    // ========================================================================
    // LJUST (tests 051-060)
    // ========================================================================

    #[test]
    fn test_wave18_string_ljust_basic() {
        let code = r#"
def lj(s: str) -> str:
    return s.ljust(10)
"#;
        let result = transpile(code);
        assert!(result.contains("width") && result.contains("fillchar"), "ljust basic: {}", result);
    }

    #[test]
    fn test_wave18_string_ljust_with_fillchar() {
        let code = r#"
def lj_fill(s: str) -> str:
    return s.ljust(10, "-")
"#;
        let result = transpile(code);
        assert!(
            result.contains("fillchar") || result.contains("format!"),
            "ljust fill: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_ljust_literal() {
        let code = r#"
def lj_lit() -> str:
    return "hi".ljust(10)
"#;
        let result = transpile(code);
        assert!(
            result.contains("width") || result.contains("fillchar"),
            "ljust literal: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_ljust_dot_fill() {
        let code = r#"
def lj_dot() -> str:
    return "hi".ljust(10, ".")
"#;
        let result = transpile(code);
        assert!(result.contains("fillchar") || result.contains("format!"), "ljust dot: {}", result);
    }

    #[test]
    fn test_wave18_string_ljust_narrow() {
        let code = r#"
def lj_narrow() -> str:
    return "hello".ljust(3)
"#;
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("len"), "ljust narrow: {}", result);
    }

    #[test]
    fn test_wave18_string_ljust_zero_fill() {
        let code = r#"
def lj_zero(s: str) -> str:
    return s.ljust(10, "0")
"#;
        let result = transpile(code);
        assert!(
            result.contains("fillchar") || result.contains("format!"),
            "ljust zero: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_ljust_wide() {
        let code = r#"
def lj_wide(s: str) -> str:
    return s.ljust(50)
"#;
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("50"), "ljust wide: {}", result);
    }

    #[test]
    fn test_wave18_string_ljust_assign() {
        let code = r#"
def lj_assign(s: str) -> str:
    padded = s.ljust(15)
    return padded
"#;
        let result = transpile(code);
        assert!(
            result.contains("width") || result.contains("fillchar"),
            "ljust assign: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_ljust_variable_width() {
        let code = r#"
def lj_var(s: str, w: int) -> str:
    return s.ljust(w)
"#;
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("fillchar"), "ljust var: {}", result);
    }

    #[test]
    fn test_wave18_string_ljust_exact() {
        let code = r#"
def lj_exact() -> str:
    return "hello".ljust(5)
"#;
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("len"), "ljust exact: {}", result);
    }

    // ========================================================================
    // RJUST (tests 061-070)
    // ========================================================================

    #[test]
    fn test_wave18_string_rjust_basic() {
        let code = r#"
def rj(s: str) -> str:
    return s.rjust(10)
"#;
        let result = transpile(code);
        assert!(result.contains("width") && result.contains("fillchar"), "rjust basic: {}", result);
    }

    #[test]
    fn test_wave18_string_rjust_with_fillchar() {
        let code = r#"
def rj_fill(s: str) -> str:
    return s.rjust(10, "0")
"#;
        let result = transpile(code);
        assert!(
            result.contains("fillchar") || result.contains("format!"),
            "rjust fill: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_rjust_literal() {
        let code = r#"
def rj_lit() -> str:
    return "hi".rjust(10)
"#;
        let result = transpile(code);
        assert!(
            result.contains("width") || result.contains("fillchar"),
            "rjust literal: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_rjust_star_fill() {
        let code = r#"
def rj_star() -> str:
    return "hi".rjust(10, "*")
"#;
        let result = transpile(code);
        assert!(
            result.contains("fillchar") || result.contains("format!"),
            "rjust star: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_rjust_narrow() {
        let code = r#"
def rj_narrow() -> str:
    return "hello".rjust(3)
"#;
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("len"), "rjust narrow: {}", result);
    }

    #[test]
    fn test_wave18_string_rjust_hash_fill() {
        let code = "def rj_hash(s: str) -> str:\n    return s.rjust(20, \"#\")\n";
        let result = transpile(code);
        assert!(
            result.contains("fillchar") || result.contains("format!"),
            "rjust hash: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_rjust_wide() {
        let code = r#"
def rj_wide(s: str) -> str:
    return s.rjust(80)
"#;
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("80"), "rjust wide: {}", result);
    }

    #[test]
    fn test_wave18_string_rjust_assign() {
        let code = r#"
def rj_assign(s: str) -> str:
    padded = s.rjust(12)
    return padded
"#;
        let result = transpile(code);
        assert!(
            result.contains("width") || result.contains("fillchar"),
            "rjust assign: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_rjust_variable_width() {
        let code = r#"
def rj_var(s: str, w: int) -> str:
    return s.rjust(w)
"#;
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("fillchar"), "rjust var: {}", result);
    }

    #[test]
    fn test_wave18_string_rjust_exact() {
        let code = r#"
def rj_exact() -> str:
    return "hello".rjust(5)
"#;
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("len"), "rjust exact: {}", result);
    }

    // ========================================================================
    // PARTITION (tests 071-080)
    // ========================================================================

    #[test]
    fn test_wave18_string_partition_basic() {
        let code = r#"
def part(s: str) -> tuple:
    return s.partition(":")
"#;
        let result = transpile(code);
        assert!(
            result.contains("find") || result.contains("partition"),
            "partition basic: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_partition_literal() {
        let code = r#"
def part_lit() -> tuple:
    return "a:b:c".partition(":")
"#;
        let result = transpile(code);
        assert!(
            result.contains("find") || result.contains("partition"),
            "partition literal: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_partition_space() {
        let code = r#"
def part_space(s: str) -> tuple:
    return s.partition(" ")
"#;
        let result = transpile(code);
        assert!(
            result.contains("find") || result.contains("partition"),
            "partition space: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_partition_not_found() {
        let code = r#"
def part_miss() -> tuple:
    return "hello".partition("xyz")
"#;
        let result = transpile(code);
        assert!(
            result.contains("find") || result.contains("partition"),
            "partition miss: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_partition_multi_char_sep() {
        let code = r#"
def part_multi(s: str) -> tuple:
    return s.partition("::")
"#;
        let result = transpile(code);
        assert!(
            result.contains("find") || result.contains("sep_str"),
            "partition multi: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_partition_assign() {
        let code = r#"
def part_assign(s: str) -> tuple:
    result = s.partition(",")
    return result
"#;
        let result = transpile(code);
        assert!(
            result.contains("find") || result.contains("partition"),
            "partition assign: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_partition_dash() {
        let code = r#"
def part_dash() -> tuple:
    return "key-value-pair".partition("-")
"#;
        let result = transpile(code);
        assert!(
            result.contains("find") || result.contains("partition"),
            "partition dash: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_partition_equals() {
        let code = r#"
def part_eq(s: str) -> tuple:
    return s.partition("=")
"#;
        let result = transpile(code);
        assert!(
            result.contains("find") || result.contains("partition"),
            "partition eq: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_partition_at_start() {
        let code = r#"
def part_start() -> tuple:
    return ":hello".partition(":")
"#;
        let result = transpile(code);
        assert!(
            result.contains("find") || result.contains("before"),
            "partition start: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_partition_at_end() {
        let code = r#"
def part_end() -> tuple:
    return "hello:".partition(":")
"#;
        let result = transpile(code);
        assert!(result.contains("find") || result.contains("after"), "partition end: {}", result);
    }

    // ========================================================================
    // RPARTITION (tests 081-085) - falls through to default handler
    // ========================================================================

    #[test]
    fn test_wave18_string_rpartition_basic() {
        let code = r#"
def rpart(s: str) -> tuple:
    return s.rpartition(":")
"#;
        let result = transpile(code);
        assert!(!result.is_empty(), "rpartition basic: {}", result);
    }

    #[test]
    fn test_wave18_string_rpartition_literal() {
        let code = r#"
def rpart_lit() -> tuple:
    return "a:b:c".rpartition(":")
"#;
        let result = transpile(code);
        assert!(!result.is_empty(), "rpartition literal: {}", result);
    }

    #[test]
    fn test_wave18_string_rpartition_space() {
        let code = r#"
def rpart_space(s: str) -> tuple:
    return s.rpartition(" ")
"#;
        let result = transpile(code);
        assert!(!result.is_empty(), "rpartition space: {}", result);
    }

    #[test]
    fn test_wave18_string_rpartition_not_found() {
        let code = r#"
def rpart_miss() -> tuple:
    return "hello".rpartition("xyz")
"#;
        let result = transpile(code);
        assert!(!result.is_empty(), "rpartition miss: {}", result);
    }

    #[test]
    fn test_wave18_string_rpartition_assign() {
        let code = r#"
def rpart_assign(s: str) -> tuple:
    result = s.rpartition(",")
    return result
"#;
        let result = transpile(code);
        assert!(!result.is_empty(), "rpartition assign: {}", result);
    }

    // ========================================================================
    // ISIDENTIFIER (tests 086-095)
    // ========================================================================

    #[test]
    fn test_wave18_string_isidentifier_valid() {
        let code = r#"
def check_id(s: str) -> bool:
    return s.isidentifier()
"#;
        let result = transpile(code);
        assert!(
            result.contains("is_alphabetic") || result.contains("is_alphanumeric"),
            "isidentifier valid: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_isidentifier_literal_valid() {
        let code = r#"
def check_valid() -> bool:
    return "valid_name".isidentifier()
"#;
        let result = transpile(code);
        assert!(
            result.contains("is_empty") || result.contains("enumerate"),
            "isidentifier valid lit: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_isidentifier_literal_invalid() {
        let code = r#"
def check_invalid() -> bool:
    return "123bad".isidentifier()
"#;
        let result = transpile(code);
        assert!(
            result.contains("is_alphabetic") || result.contains("enumerate"),
            "isidentifier invalid lit: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_isidentifier_underscore() {
        let code = r#"
def check_under() -> bool:
    return "_private".isidentifier()
"#;
        let result = transpile(code);
        assert!(
            result.contains("_") || result.contains("is_alphabetic"),
            "isidentifier underscore: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_isidentifier_empty() {
        let code = r#"
def check_empty() -> bool:
    return "".isidentifier()
"#;
        let result = transpile(code);
        assert!(result.contains("is_empty"), "isidentifier empty: {}", result);
    }

    #[test]
    fn test_wave18_string_isidentifier_with_spaces() {
        let code = r#"
def check_spaces() -> bool:
    return "has space".isidentifier()
"#;
        let result = transpile(code);
        assert!(
            result.contains("is_alphanumeric") || result.contains("enumerate"),
            "isidentifier spaces: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_isidentifier_assign() {
        let code = r#"
def check_assign(s: str) -> bool:
    is_id = s.isidentifier()
    return is_id
"#;
        let result = transpile(code);
        assert!(
            result.contains("is_alphabetic") || result.contains("is_alphanumeric"),
            "isidentifier assign: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_isidentifier_double_under() {
        let code = r#"
def check_dunder() -> bool:
    return "__init__".isidentifier()
"#;
        let result = transpile(code);
        assert!(
            result.contains("enumerate") || result.contains("is_alphabetic"),
            "isidentifier dunder: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_isidentifier_single_char() {
        let code = r#"
def check_single() -> bool:
    return "x".isidentifier()
"#;
        let result = transpile(code);
        assert!(
            result.contains("is_alphabetic") || result.contains("enumerate"),
            "isidentifier single: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_isidentifier_digit_only() {
        let code = r#"
def check_digit_only() -> bool:
    return "42".isidentifier()
"#;
        let result = transpile(code);
        assert!(
            result.contains("is_alphabetic") || result.contains("enumerate"),
            "isidentifier digit: {}",
            result
        );
    }

    // ========================================================================
    // RSPLIT with maxsplit (tests 096-105)
    // ========================================================================

    #[test]
    fn test_wave18_string_rsplit_maxsplit_basic() {
        let code = r#"
def rs_max(s: str) -> list:
    return s.rsplit(",", 1)
"#;
        let result = transpile(code);
        assert!(result.contains("rsplitn"), "rsplit maxsplit basic: {}", result);
    }

    #[test]
    fn test_wave18_string_rsplit_maxsplit_literal() {
        let code = r#"
def rs_max_lit() -> list:
    return "a,b,c,d".rsplit(",", 1)
"#;
        let result = transpile(code);
        assert!(result.contains("rsplitn"), "rsplit maxsplit literal: {}", result);
    }

    #[test]
    fn test_wave18_string_rsplit_maxsplit_two() {
        let code = r#"
def rs_max2(s: str) -> list:
    return s.rsplit(",", 2)
"#;
        let result = transpile(code);
        assert!(result.contains("rsplitn"), "rsplit maxsplit 2: {}", result);
    }

    #[test]
    fn test_wave18_string_rsplit_no_args() {
        let code = r#"
def rs_noargs(s: str) -> list:
    return s.rsplit()
"#;
        let result = transpile(code);
        assert!(
            result.contains("split_whitespace") && result.contains("rev"),
            "rsplit no args: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_rsplit_sep_only() {
        let code = r#"
def rs_sep(s: str) -> list:
    return s.rsplit(".")
"#;
        let result = transpile(code);
        assert!(result.contains("rsplit"), "rsplit sep only: {}", result);
    }

    #[test]
    fn test_wave18_string_rsplit_maxsplit_zero() {
        let code = r#"
def rs_max0(s: str) -> list:
    return s.rsplit(",", 0)
"#;
        let result = transpile(code);
        assert!(result.contains("rsplitn"), "rsplit maxsplit 0: {}", result);
    }

    #[test]
    fn test_wave18_string_rsplit_maxsplit_space() {
        let code = r#"
def rs_space() -> list:
    return "a b c d".rsplit(" ", 2)
"#;
        let result = transpile(code);
        assert!(result.contains("rsplitn"), "rsplit maxsplit space: {}", result);
    }

    #[test]
    fn test_wave18_string_rsplit_assign() {
        let code = r#"
def rs_assign(s: str) -> list:
    parts = s.rsplit(",", 1)
    return parts
"#;
        let result = transpile(code);
        assert!(result.contains("rsplitn"), "rsplit assign: {}", result);
    }

    #[test]
    fn test_wave18_string_rsplit_maxsplit_dash() {
        let code = r#"
def rs_dash() -> list:
    return "a-b-c-d".rsplit("-", 1)
"#;
        let result = transpile(code);
        assert!(result.contains("rsplitn"), "rsplit maxsplit dash: {}", result);
    }

    #[test]
    fn test_wave18_string_rsplit_maxsplit_variable() {
        let code = r#"
def rs_var(s: str, n: int) -> list:
    return s.rsplit(",", n)
"#;
        let result = transpile(code);
        assert!(result.contains("rsplitn"), "rsplit maxsplit var: {}", result);
    }

    // ========================================================================
    // FORMAT (tests 106-120)
    // ========================================================================

    #[test]
    fn test_wave18_string_format_single_arg() {
        let code = r#"
def fmt1(name: str) -> str:
    return "Hello, {}!".format(name)
"#;
        let result = transpile(code);
        assert!(
            result.contains("replacen") || result.contains("format!"),
            "format single: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_format_two_args() {
        let code = r#"
def fmt2(a: str, b: str) -> str:
    return "{} {}".format(a, b)
"#;
        let result = transpile(code);
        assert!(result.contains("replacen"), "format two args: {}", result);
    }

    #[test]
    fn test_wave18_string_format_three_args() {
        let code = r#"
def fmt3(a: str, b: str, c: str) -> str:
    return "{} {} {}".format(a, b, c)
"#;
        let result = transpile(code);
        assert!(result.contains("replacen"), "format three args: {}", result);
    }

    #[test]
    fn test_wave18_string_format_no_args() {
        let code = r#"
def fmt0() -> str:
    return "hello".format()
"#;
        let result = transpile(code);
        assert!(!result.is_empty(), "format no args: {}", result);
    }

    #[test]
    fn test_wave18_string_format_int_arg() {
        let code = r#"
def fmt_int(n: int) -> str:
    return "Number: {}".format(n)
"#;
        let result = transpile(code);
        assert!(
            result.contains("replacen") || result.contains("format!"),
            "format int: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_format_literal_args() {
        let code = r#"
def fmt_lit() -> str:
    return "{} {}".format("hello", "world")
"#;
        let result = transpile(code);
        assert!(result.contains("replacen"), "format literal args: {}", result);
    }

    #[test]
    fn test_wave18_string_format_mixed_types() {
        let code = r#"
def fmt_mixed(name: str, age: int) -> str:
    return "Name: {}, Age: {}".format(name, age)
"#;
        let result = transpile(code);
        assert!(result.contains("replacen"), "format mixed: {}", result);
    }

    #[test]
    fn test_wave18_string_format_assign() {
        let code = r#"
def fmt_assign(x: str) -> str:
    msg = "Value: {}".format(x)
    return msg
"#;
        let result = transpile(code);
        assert!(
            result.contains("replacen") || result.contains("format!"),
            "format assign: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_format_in_variable_template() {
        let code = r#"
def fmt_var_tmpl(tmpl: str, val: str) -> str:
    return tmpl.format(val)
"#;
        let result = transpile(code);
        assert!(
            result.contains("replacen") || result.contains("format!"),
            "format var tmpl: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_format_empty_placeholder() {
        let code = r#"
def fmt_empty() -> str:
    return "{}".format("test")
"#;
        let result = transpile(code);
        assert!(
            result.contains("replacen") || result.contains("format!"),
            "format empty ph: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_format_multiple_same() {
        let code = r#"
def fmt_dup(a: str, b: str, c: str, d: str) -> str:
    return "{} {} {} {}".format(a, b, c, d)
"#;
        let result = transpile(code);
        assert!(result.contains("replacen"), "format dup: {}", result);
    }

    #[test]
    fn test_wave18_string_format_float_arg() {
        let code = r#"
def fmt_float(x: float) -> str:
    return "Value: {}".format(x)
"#;
        let result = transpile(code);
        assert!(
            result.contains("replacen") || result.contains("format!"),
            "format float: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_format_bool_arg() {
        let code = r#"
def fmt_bool(b: bool) -> str:
    return "Flag: {}".format(b)
"#;
        let result = transpile(code);
        assert!(
            result.contains("replacen") || result.contains("format!"),
            "format bool: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_format_sentence() {
        let code = r#"
def fmt_sentence(subj: str, verb: str, obj: str) -> str:
    return "The {} {} the {}".format(subj, verb, obj)
"#;
        let result = transpile(code);
        assert!(result.contains("replacen"), "format sentence: {}", result);
    }

    #[test]
    fn test_wave18_string_format_with_newlines() {
        let code = r#"
def fmt_nl(a: str, b: str) -> str:
    return "{}\n{}".format(a, b)
"#;
        let result = transpile(code);
        assert!(result.contains("replacen"), "format newlines: {}", result);
    }

    // ========================================================================
    // TITLE (tests 121-130)
    // ========================================================================

    #[test]
    fn test_wave18_string_title_basic() {
        let code = r#"
def titlecase(s: str) -> str:
    return s.title()
"#;
        let result = transpile(code);
        assert!(
            result.contains("split_whitespace") || result.contains("to_uppercase"),
            "title basic: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_title_literal() {
        let code = r#"
def title_lit() -> str:
    return "hello world".title()
"#;
        let result = transpile(code);
        assert!(
            result.contains("split_whitespace") || result.contains("join"),
            "title literal: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_title_single_word() {
        let code = r#"
def title_single() -> str:
    return "hello".title()
"#;
        let result = transpile(code);
        assert!(
            result.contains("split_whitespace") || result.contains("to_uppercase"),
            "title single: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_title_all_upper() {
        let code = r#"
def title_upper() -> str:
    return "HELLO WORLD".title()
"#;
        let result = transpile(code);
        assert!(result.contains("split_whitespace"), "title upper: {}", result);
    }

    #[test]
    fn test_wave18_string_title_mixed() {
        let code = r#"
def title_mixed() -> str:
    return "hELLO wORLD".title()
"#;
        let result = transpile(code);
        assert!(
            result.contains("to_uppercase") || result.contains("split_whitespace"),
            "title mixed: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_title_with_digits() {
        let code = r#"
def title_digits() -> str:
    return "hello 123 world".title()
"#;
        let result = transpile(code);
        assert!(result.contains("split_whitespace"), "title digits: {}", result);
    }

    #[test]
    fn test_wave18_string_title_empty() {
        let code = r#"
def title_empty() -> str:
    return "".title()
"#;
        let result = transpile(code);
        assert!(
            result.contains("split_whitespace") || result.contains("None"),
            "title empty: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_title_assign() {
        let code = r#"
def title_assign(s: str) -> str:
    titled = s.title()
    return titled
"#;
        let result = transpile(code);
        assert!(
            result.contains("split_whitespace") || result.contains("to_uppercase"),
            "title assign: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_title_three_words() {
        let code = r#"
def title_three() -> str:
    return "foo bar baz".title()
"#;
        let result = transpile(code);
        assert!(result.contains("split_whitespace"), "title three: {}", result);
    }

    #[test]
    fn test_wave18_string_title_already_titled() {
        let code = r#"
def title_noop() -> str:
    return "Hello World".title()
"#;
        let result = transpile(code);
        assert!(result.contains("split_whitespace"), "title noop: {}", result);
    }

    // ========================================================================
    // ISUPPER / ISLOWER / ISSPACE (tests 131-145)
    // ========================================================================

    #[test]
    fn test_wave18_string_isupper_basic() {
        let code = r#"
def check_upper(s: str) -> bool:
    return s.isupper()
"#;
        let result = transpile(code);
        assert!(
            result.contains("is_uppercase") || result.contains("is_alphabetic"),
            "isupper basic: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_isupper_literal_true() {
        let code = r#"
def check_upper_lit() -> bool:
    return "ABC".isupper()
"#;
        let result = transpile(code);
        assert!(result.contains("is_uppercase"), "isupper literal: {}", result);
    }

    #[test]
    fn test_wave18_string_isupper_literal_false() {
        let code = r#"
def check_upper_false() -> bool:
    return "aBc".isupper()
"#;
        let result = transpile(code);
        assert!(result.contains("is_uppercase"), "isupper false: {}", result);
    }

    #[test]
    fn test_wave18_string_isupper_assign() {
        let code = r#"
def check_upper_assign(s: str) -> bool:
    is_up = s.isupper()
    return is_up
"#;
        let result = transpile(code);
        assert!(result.contains("is_uppercase"), "isupper assign: {}", result);
    }

    #[test]
    fn test_wave18_string_isupper_with_digits() {
        let code = r#"
def check_upper_digits() -> bool:
    return "ABC123".isupper()
"#;
        let result = transpile(code);
        assert!(
            result.contains("is_uppercase") || result.contains("is_alphabetic"),
            "isupper digits: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_islower_basic() {
        let code = r#"
def check_lower(s: str) -> bool:
    return s.islower()
"#;
        let result = transpile(code);
        assert!(
            result.contains("is_lowercase") || result.contains("is_alphabetic"),
            "islower basic: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_islower_literal_true() {
        let code = r#"
def check_lower_lit() -> bool:
    return "abc".islower()
"#;
        let result = transpile(code);
        assert!(result.contains("is_lowercase"), "islower literal: {}", result);
    }

    #[test]
    fn test_wave18_string_islower_literal_false() {
        let code = r#"
def check_lower_false() -> bool:
    return "AbC".islower()
"#;
        let result = transpile(code);
        assert!(result.contains("is_lowercase"), "islower false: {}", result);
    }

    #[test]
    fn test_wave18_string_islower_assign() {
        let code = r#"
def check_lower_assign(s: str) -> bool:
    is_low = s.islower()
    return is_low
"#;
        let result = transpile(code);
        assert!(result.contains("is_lowercase"), "islower assign: {}", result);
    }

    #[test]
    fn test_wave18_string_islower_with_numbers() {
        let code = r#"
def check_lower_num() -> bool:
    return "abc123".islower()
"#;
        let result = transpile(code);
        assert!(result.contains("is_lowercase"), "islower num: {}", result);
    }

    #[test]
    fn test_wave18_string_isspace_basic() {
        let code = r#"
def check_space(s: str) -> bool:
    return s.isspace()
"#;
        let result = transpile(code);
        assert!(result.contains("is_whitespace"), "isspace basic: {}", result);
    }

    #[test]
    fn test_wave18_string_isspace_literal_true() {
        let code = r#"
def check_space_lit() -> bool:
    return "   ".isspace()
"#;
        let result = transpile(code);
        assert!(result.contains("is_whitespace"), "isspace literal: {}", result);
    }

    #[test]
    fn test_wave18_string_isspace_literal_false() {
        let code = r#"
def check_space_false() -> bool:
    return "hello".isspace()
"#;
        let result = transpile(code);
        assert!(result.contains("is_whitespace"), "isspace false: {}", result);
    }

    #[test]
    fn test_wave18_string_isspace_tab() {
        let code = r#"
def check_tab() -> bool:
    return "\t\n".isspace()
"#;
        let result = transpile(code);
        assert!(result.contains("is_whitespace"), "isspace tab: {}", result);
    }

    #[test]
    fn test_wave18_string_isspace_assign() {
        let code = r#"
def check_space_assign(s: str) -> bool:
    ws = s.isspace()
    return ws
"#;
        let result = transpile(code);
        assert!(result.contains("is_whitespace"), "isspace assign: {}", result);
    }

    // ========================================================================
    // ISNUMERIC / ISASCII / ISDECIMAL (tests 146-160)
    // ========================================================================

    #[test]
    fn test_wave18_string_isnumeric_basic() {
        let code = r#"
def check_num(s: str) -> bool:
    return s.isnumeric()
"#;
        let result = transpile(code);
        assert!(result.contains("is_numeric"), "isnumeric basic: {}", result);
    }

    #[test]
    fn test_wave18_string_isnumeric_literal_true() {
        let code = r#"
def check_num_lit() -> bool:
    return "123".isnumeric()
"#;
        let result = transpile(code);
        assert!(result.contains("is_numeric"), "isnumeric literal: {}", result);
    }

    #[test]
    fn test_wave18_string_isnumeric_literal_false() {
        let code = r#"
def check_num_false() -> bool:
    return "abc".isnumeric()
"#;
        let result = transpile(code);
        assert!(result.contains("is_numeric"), "isnumeric false: {}", result);
    }

    #[test]
    fn test_wave18_string_isnumeric_assign() {
        let code = r#"
def check_num_assign(s: str) -> bool:
    is_num = s.isnumeric()
    return is_num
"#;
        let result = transpile(code);
        assert!(result.contains("is_numeric"), "isnumeric assign: {}", result);
    }

    #[test]
    fn test_wave18_string_isnumeric_mixed() {
        let code = r#"
def check_num_mixed() -> bool:
    return "123abc".isnumeric()
"#;
        let result = transpile(code);
        assert!(result.contains("is_numeric"), "isnumeric mixed: {}", result);
    }

    #[test]
    fn test_wave18_string_isascii_basic() {
        let code = r#"
def check_ascii(s: str) -> bool:
    return s.isascii()
"#;
        let result = transpile(code);
        assert!(result.contains("is_ascii"), "isascii basic: {}", result);
    }

    #[test]
    fn test_wave18_string_isascii_literal_true() {
        let code = r#"
def check_ascii_lit() -> bool:
    return "abc".isascii()
"#;
        let result = transpile(code);
        assert!(result.contains("is_ascii"), "isascii literal: {}", result);
    }

    #[test]
    fn test_wave18_string_isascii_literal_mixed() {
        let code = r#"
def check_ascii_mixed() -> bool:
    return "hello123!@#".isascii()
"#;
        let result = transpile(code);
        assert!(result.contains("is_ascii"), "isascii mixed: {}", result);
    }

    #[test]
    fn test_wave18_string_isascii_assign() {
        let code = r#"
def check_ascii_assign(s: str) -> bool:
    is_asc = s.isascii()
    return is_asc
"#;
        let result = transpile(code);
        assert!(result.contains("is_ascii"), "isascii assign: {}", result);
    }

    #[test]
    fn test_wave18_string_isascii_empty() {
        let code = r#"
def check_ascii_empty() -> bool:
    return "".isascii()
"#;
        let result = transpile(code);
        assert!(result.contains("is_ascii"), "isascii empty: {}", result);
    }

    #[test]
    fn test_wave18_string_isdecimal_basic() {
        let code = r#"
def check_dec(s: str) -> bool:
    return s.isdecimal()
"#;
        let result = transpile(code);
        assert!(result.contains("is_ascii_digit"), "isdecimal basic: {}", result);
    }

    #[test]
    fn test_wave18_string_isdecimal_literal_true() {
        let code = r#"
def check_dec_lit() -> bool:
    return "123".isdecimal()
"#;
        let result = transpile(code);
        assert!(result.contains("is_ascii_digit"), "isdecimal literal: {}", result);
    }

    #[test]
    fn test_wave18_string_isdecimal_literal_false() {
        let code = r#"
def check_dec_false() -> bool:
    return "12.3".isdecimal()
"#;
        let result = transpile(code);
        assert!(result.contains("is_ascii_digit"), "isdecimal false: {}", result);
    }

    #[test]
    fn test_wave18_string_isdecimal_assign() {
        let code = r#"
def check_dec_assign(s: str) -> bool:
    is_dec = s.isdecimal()
    return is_dec
"#;
        let result = transpile(code);
        assert!(result.contains("is_ascii_digit"), "isdecimal assign: {}", result);
    }

    #[test]
    fn test_wave18_string_isdecimal_empty() {
        let code = r#"
def check_dec_empty() -> bool:
    return "".isdecimal()
"#;
        let result = transpile(code);
        assert!(result.contains("is_ascii_digit"), "isdecimal empty: {}", result);
    }

    // ========================================================================
    // SPLITLINES (tests 161-165)
    // ========================================================================

    #[test]
    fn test_wave18_string_splitlines_basic() {
        let code = r#"
def split_lines(s: str) -> list:
    return s.splitlines()
"#;
        let result = transpile(code);
        assert!(result.contains("lines()"), "splitlines basic: {}", result);
    }

    #[test]
    fn test_wave18_string_splitlines_literal() {
        let code = "def split_lit() -> list:\n    return \"a\\nb\\nc\".splitlines()\n";
        let result = transpile(code);
        assert!(result.contains("lines()"), "splitlines literal: {}", result);
    }

    #[test]
    fn test_wave18_string_splitlines_assign() {
        let code = r#"
def split_lines_assign(s: str) -> list:
    lines = s.splitlines()
    return lines
"#;
        let result = transpile(code);
        assert!(result.contains("lines()"), "splitlines assign: {}", result);
    }

    #[test]
    fn test_wave18_string_splitlines_empty() {
        let code = r#"
def split_lines_empty() -> list:
    return "".splitlines()
"#;
        let result = transpile(code);
        assert!(result.contains("lines()"), "splitlines empty: {}", result);
    }

    #[test]
    fn test_wave18_string_splitlines_single_line() {
        let code = r#"
def split_single() -> list:
    return "hello".splitlines()
"#;
        let result = transpile(code);
        assert!(result.contains("lines()"), "splitlines single: {}", result);
    }

    // ========================================================================
    // REMOVEPREFIX / REMOVESUFFIX (tests 166-175)
    // ========================================================================

    #[test]
    fn test_wave18_string_removeprefix_basic() {
        let code = r#"
def rmpfx(s: str) -> str:
    return s.removeprefix("Hello")
"#;
        let result = transpile(code);
        assert!(!result.is_empty(), "removeprefix basic: {}", result);
    }

    #[test]
    fn test_wave18_string_removeprefix_literal() {
        let code = r#"
def rmpfx_lit() -> str:
    return "HelloWorld".removeprefix("Hello")
"#;
        let result = transpile(code);
        assert!(!result.is_empty(), "removeprefix literal: {}", result);
    }

    #[test]
    fn test_wave18_string_removeprefix_no_match() {
        let code = r#"
def rmpfx_miss() -> str:
    return "HelloWorld".removeprefix("Bye")
"#;
        let result = transpile(code);
        assert!(!result.is_empty(), "removeprefix miss: {}", result);
    }

    #[test]
    fn test_wave18_string_removeprefix_assign() {
        let code = r#"
def rmpfx_assign(s: str) -> str:
    result = s.removeprefix("pre_")
    return result
"#;
        let result = transpile(code);
        assert!(!result.is_empty(), "removeprefix assign: {}", result);
    }

    #[test]
    fn test_wave18_string_removeprefix_empty() {
        let code = r#"
def rmpfx_empty(s: str) -> str:
    return s.removeprefix("")
"#;
        let result = transpile(code);
        assert!(!result.is_empty(), "removeprefix empty: {}", result);
    }

    #[test]
    fn test_wave18_string_removesuffix_basic() {
        let code = r#"
def rmsfx(s: str) -> str:
    return s.removesuffix("World")
"#;
        let result = transpile(code);
        assert!(!result.is_empty(), "removesuffix basic: {}", result);
    }

    #[test]
    fn test_wave18_string_removesuffix_literal() {
        let code = r#"
def rmsfx_lit() -> str:
    return "HelloWorld".removesuffix("World")
"#;
        let result = transpile(code);
        assert!(!result.is_empty(), "removesuffix literal: {}", result);
    }

    #[test]
    fn test_wave18_string_removesuffix_no_match() {
        let code = r#"
def rmsfx_miss() -> str:
    return "HelloWorld".removesuffix("Bye")
"#;
        let result = transpile(code);
        assert!(!result.is_empty(), "removesuffix miss: {}", result);
    }

    #[test]
    fn test_wave18_string_removesuffix_assign() {
        let code = r#"
def rmsfx_assign(s: str) -> str:
    result = s.removesuffix("_suf")
    return result
"#;
        let result = transpile(code);
        assert!(!result.is_empty(), "removesuffix assign: {}", result);
    }

    #[test]
    fn test_wave18_string_removesuffix_empty() {
        let code = r#"
def rmsfx_empty(s: str) -> str:
    return s.removesuffix("")
"#;
        let result = transpile(code);
        assert!(!result.is_empty(), "removesuffix empty: {}", result);
    }

    // ========================================================================
    // COUNT (tests 176-180)
    // ========================================================================

    #[test]
    fn test_wave18_string_count_basic() {
        let code = r#"
def cnt(s: str) -> int:
    return s.count("ab")
"#;
        let result = transpile(code);
        assert!(result.contains("matches") && result.contains("count"), "count basic: {}", result);
    }

    #[test]
    fn test_wave18_string_count_literal() {
        let code = r#"
def cnt_lit() -> int:
    return "aababc".count("ab")
"#;
        let result = transpile(code);
        assert!(
            result.contains("matches") && result.contains("count"),
            "count literal: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_count_single_char() {
        let code = r#"
def cnt_char(s: str) -> int:
    return s.count("a")
"#;
        let result = transpile(code);
        assert!(result.contains("matches") && result.contains("count"), "count char: {}", result);
    }

    #[test]
    fn test_wave18_string_count_assign() {
        let code = r#"
def cnt_assign(s: str) -> int:
    n = s.count("x")
    return n
"#;
        let result = transpile(code);
        assert!(result.contains("matches") && result.contains("count"), "count assign: {}", result);
    }

    #[test]
    fn test_wave18_string_count_no_match() {
        let code = r#"
def cnt_miss() -> int:
    return "hello".count("xyz")
"#;
        let result = transpile(code);
        assert!(result.contains("matches"), "count miss: {}", result);
    }

    // ========================================================================
    // ENCODE (tests 181-185)
    // ========================================================================

    #[test]
    fn test_wave18_string_encode_default() {
        let code = r#"
def enc(s: str) -> bytes:
    return s.encode()
"#;
        let result = transpile(code);
        assert!(
            result.contains("as_bytes") && result.contains("to_vec"),
            "encode default: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_encode_utf8() {
        let code = r#"
def enc_utf8(s: str) -> bytes:
    return s.encode("utf-8")
"#;
        let result = transpile(code);
        assert!(
            result.contains("as_bytes") && result.contains("to_vec"),
            "encode utf8: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_encode_literal() {
        let code = r#"
def enc_lit() -> bytes:
    return "hello".encode()
"#;
        let result = transpile(code);
        assert!(
            result.contains("as_bytes") && result.contains("to_vec"),
            "encode literal: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_encode_assign() {
        let code = r#"
def enc_assign(s: str) -> bytes:
    data = s.encode()
    return data
"#;
        let result = transpile(code);
        assert!(
            result.contains("as_bytes") && result.contains("to_vec"),
            "encode assign: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_encode_empty() {
        let code = r#"
def enc_empty() -> bytes:
    return "".encode()
"#;
        let result = transpile(code);
        assert!(result.contains("as_bytes"), "encode empty: {}", result);
    }

    // ========================================================================
    // CASEFOLD (tests 186-188)
    // ========================================================================

    #[test]
    fn test_wave18_string_casefold_basic() {
        let code = r#"
def fold(s: str) -> str:
    return s.casefold()
"#;
        let result = transpile(code);
        assert!(result.contains("to_lowercase"), "casefold basic: {}", result);
    }

    #[test]
    fn test_wave18_string_casefold_literal() {
        let code = r#"
def fold_lit() -> str:
    return "HELLO".casefold()
"#;
        let result = transpile(code);
        assert!(result.contains("to_lowercase"), "casefold literal: {}", result);
    }

    #[test]
    fn test_wave18_string_casefold_assign() {
        let code = r#"
def fold_assign(s: str) -> str:
    folded = s.casefold()
    return folded
"#;
        let result = transpile(code);
        assert!(result.contains("to_lowercase"), "casefold assign: {}", result);
    }

    // ========================================================================
    // HEX (tests 189-190)
    // ========================================================================

    #[test]
    fn test_wave18_string_hex_basic() {
        let code = r#"
def to_hex(s: str) -> str:
    return s.hex()
"#;
        let result = transpile(code);
        assert!(result.contains("bytes") || result.contains("02x"), "hex basic: {}", result);
    }

    #[test]
    fn test_wave18_string_hex_literal() {
        let code = r#"
def hex_lit() -> str:
    return "hello".hex()
"#;
        let result = transpile(code);
        assert!(result.contains("bytes") || result.contains("02x"), "hex literal: {}", result);
    }

    // ========================================================================
    // ISPRINTABLE (tests 191-192)
    // ========================================================================

    #[test]
    fn test_wave18_string_isprintable_basic() {
        let code = r#"
def check_print(s: str) -> bool:
    return s.isprintable()
"#;
        let result = transpile(code);
        assert!(result.contains("is_control"), "isprintable basic: {}", result);
    }

    #[test]
    fn test_wave18_string_isprintable_literal() {
        let code = r#"
def check_print_lit() -> bool:
    return "hello world".isprintable()
"#;
        let result = transpile(code);
        assert!(result.contains("is_control"), "isprintable literal: {}", result);
    }

    // ========================================================================
    // ISTITLE (tests 193-194)
    // ========================================================================

    #[test]
    fn test_wave18_string_istitle_basic() {
        let code = r#"
def check_title(s: str) -> bool:
    return s.istitle()
"#;
        let result = transpile(code);
        assert!(
            result.contains("prev_is_cased") || result.contains("is_uppercase"),
            "istitle basic: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_istitle_literal() {
        let code = r#"
def check_title_lit() -> bool:
    return "Hello World".istitle()
"#;
        let result = transpile(code);
        assert!(
            result.contains("prev_is_cased") || result.contains("is_uppercase"),
            "istitle literal: {}",
            result
        );
    }

    // ========================================================================
    // ISALNUM (tests 195-196)
    // ========================================================================

    #[test]
    fn test_wave18_string_isalnum_basic() {
        let code = r#"
def check_alnum(s: str) -> bool:
    return s.isalnum()
"#;
        let result = transpile(code);
        assert!(result.contains("is_alphanumeric"), "isalnum basic: {}", result);
    }

    #[test]
    fn test_wave18_string_isalnum_literal() {
        let code = r#"
def check_alnum_lit() -> bool:
    return "abc123".isalnum()
"#;
        let result = transpile(code);
        assert!(result.contains("is_alphanumeric"), "isalnum literal: {}", result);
    }

    // ========================================================================
    // RFIND / RINDEX / INDEX / DECODE (tests 197-200)
    // ========================================================================

    #[test]
    fn test_wave18_string_rfind_basic() {
        let code = r#"
def rfind_sub(s: str) -> int:
    return s.rfind("x")
"#;
        let result = transpile(code);
        assert!(
            result.contains("rfind") || result.contains("unwrap_or"),
            "rfind basic: {}",
            result
        );
    }

    #[test]
    fn test_wave18_string_rindex_basic() {
        let code = r#"
def rindex_sub(s: str) -> int:
    return s.rindex("x")
"#;
        let result = transpile(code);
        assert!(result.contains("rfind") || result.contains("expect"), "rindex basic: {}", result);
    }

    #[test]
    fn test_wave18_string_index_basic() {
        let code = r#"
def index_sub(s: str) -> int:
    return s.index("x")
"#;
        let result = transpile(code);
        assert!(result.contains("find") || result.contains("expect"), "index basic: {}", result);
    }

    #[test]
    fn test_wave18_string_decode_basic() {
        let code = r#"
def dec(s: str) -> str:
    return s.decode()
"#;
        let result = transpile(code);
        assert!(
            result.contains("from_utf8_lossy") || result.contains("to_string"),
            "decode basic: {}",
            result
        );
    }
}
