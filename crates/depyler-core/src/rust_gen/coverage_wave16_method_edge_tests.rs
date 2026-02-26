//! Wave 16: Coverage tests for string, dict, list, and set method edge cases.
//!
//! 200 tests targeting uncovered code paths in:
//! - string_methods.rs: lstrip/rstrip with chars, split with maxsplit, replace with count,
//!   find with start, partition, encode, casefold, center/ljust/rjust, zfill, expandtabs,
//!   isdigit/isnumeric/isdecimal, title/swapcase/capitalize, format, hex, splitlines,
//!   index, rindex, isprintable, isidentifier, isascii, isupper, islower, istitle
//! - dict_methods.rs: get with default, popitem, setdefault, keys, values, items,
//!   update, pop, clear, copy
//! - list_methods.rs: pop with index, sort with key/reverse, insert, extend, count,
//!   index, copy, clear, reverse, remove
//! - set_methods.rs: add, intersection_update, difference_update, union, intersection,
//!   difference, symmetric_difference, issubset, issuperset, isdisjoint, discard, remove
//!
//! Status: 200/200 tests

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
    // STRING METHODS (50 tests: test_w16me_string_001 through test_w16me_string_050)
    // ========================================================================

    #[test]
    fn test_w16me_string_001_lstrip_with_chars() {
        let code = r#"
def trim_left(s: str) -> str:
    return s.lstrip("he")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("trim_start_matches") || result.contains("trim_start"));
    }

    #[test]
    fn test_w16me_string_002_rstrip_with_chars() {
        let code = r#"
def trim_right(s: str) -> str:
    return s.rstrip("ld")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("trim_end_matches") || result.contains("trim_end"));
    }

    #[test]
    fn test_w16me_string_003_strip_with_chars() {
        let code = r#"
def trim_both(s: str) -> str:
    return s.strip("xy")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("trim_matches") || result.contains("trim"));
    }

    #[test]
    fn test_w16me_string_004_split_maxsplit_two() {
        let code = r#"
def split_twice(s: str) -> list:
    return s.split(":", 2)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("splitn") || result.contains("split"));
    }

    #[test]
    fn test_w16me_string_005_split_maxsplit_one() {
        let code = r#"
def split_once(s: str) -> list:
    return s.split("-", 1)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("splitn") || result.contains("split"));
    }

    #[test]
    fn test_w16me_string_006_replace_with_count() {
        let code = r#"
def replace_first_only(s: str) -> str:
    return s.replace("lo", "LO", 1)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("replacen") || result.contains("replace"));
    }

    #[test]
    fn test_w16me_string_007_replace_with_count_two() {
        let code = r#"
def replace_two(s: str) -> str:
    return s.replace("a", "b", 2)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("replacen") || result.contains("replace"));
    }

    #[test]
    fn test_w16me_string_008_find_with_start() {
        let code = r#"
def find_from_pos(s: str) -> int:
    return s.find("lo", 3)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("find"));
    }

    #[test]
    fn test_w16me_string_009_find_with_start_var() {
        let code = r#"
def find_from_var(s: str, pos: int) -> int:
    return s.find("x", pos)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("find"));
    }

    #[test]
    fn test_w16me_string_010_partition_dash() {
        let code = r#"
def split_on_dash(s: str) -> tuple:
    return s.partition("-")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("find") || result.contains("partition"));
    }

    #[test]
    fn test_w16me_string_011_partition_colon() {
        let code = r#"
def split_on_colon(s: str) -> tuple:
    return s.partition(":")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16me_string_012_encode_utf8() {
        let code = r#"
def to_bytes(s: str) -> list:
    return s.encode("utf-8")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("as_bytes") || result.contains("to_vec"));
    }

    #[test]
    fn test_w16me_string_013_encode_no_arg() {
        let code = r#"
def to_default_bytes(s: str) -> list:
    return s.encode()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("as_bytes") || result.contains("to_vec"));
    }

    #[test]
    fn test_w16me_string_014_casefold_basic() {
        let code = r#"
def normalize_case(s: str) -> str:
    return s.casefold()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("to_lowercase"));
    }

    #[test]
    fn test_w16me_string_015_center_basic() {
        let code = r#"
def center_it(s: str) -> str:
    return s.center(20)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("width") || result.contains("pad") || result.contains("format"));
    }

    #[test]
    fn test_w16me_string_016_center_with_fill() {
        let code = r#"
def center_stars(s: str) -> str:
    return s.center(30, "*")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("width") || result.contains("fillchar") || result.contains("repeat")
        );
    }

    #[test]
    fn test_w16me_string_017_ljust_basic() {
        let code = r#"
def pad_right(s: str) -> str:
    return s.ljust(20)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("width") || result.contains("format"));
    }

    #[test]
    fn test_w16me_string_018_ljust_fill_dot() {
        let code = r#"
def pad_right_dot(s: str) -> str:
    return s.ljust(20, ".")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("width") || result.contains("fillchar"));
    }

    #[test]
    fn test_w16me_string_019_rjust_basic() {
        let code = r#"
def pad_left(s: str) -> str:
    return s.rjust(20)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("width") || result.contains("format"));
    }

    #[test]
    fn test_w16me_string_020_rjust_fill_zero() {
        let code = r#"
def pad_left_zero(s: str) -> str:
    return s.rjust(20, "0")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("width") || result.contains("fillchar"));
    }

    #[test]
    fn test_w16me_string_021_zfill_basic() {
        let code = r#"
def zero_pad(s: str) -> str:
    return s.zfill(5)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("width") || result.contains("zfill") || result.contains("repeat"));
    }

    #[test]
    fn test_w16me_string_022_zfill_large() {
        let code = r#"
def zero_pad_large(s: str) -> str:
    return s.zfill(10)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16me_string_023_expandtabs_default() {
        let code = r#"
def expand_tabs(s: str) -> str:
    return s.expandtabs()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("replace") || result.contains("repeat"));
    }

    #[test]
    fn test_w16me_string_024_expandtabs_four() {
        let code = r#"
def expand_tabs_four(s: str) -> str:
    return s.expandtabs(4)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("replace") || result.contains("repeat"));
    }

    #[test]
    fn test_w16me_string_025_isdigit_basic() {
        let code = r#"
def check_digit(s: str) -> bool:
    return s.isdigit()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("is_numeric") || result.contains("isdigit"));
    }

    #[test]
    fn test_w16me_string_026_isnumeric_basic() {
        let code = r#"
def check_numeric(s: str) -> bool:
    return s.isnumeric()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("is_numeric") || result.contains("isnumeric"));
    }

    #[test]
    fn test_w16me_string_027_isdecimal_basic() {
        let code = r#"
def check_decimal(s: str) -> bool:
    return s.isdecimal()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("is_ascii_digit") || result.contains("isdecimal"));
    }

    #[test]
    fn test_w16me_string_028_title_basic() {
        let code = r#"
def make_title(s: str) -> str:
    return s.title()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("to_uppercase") || result.contains("split_whitespace"));
    }

    #[test]
    fn test_w16me_string_029_swapcase_basic() {
        let code = r#"
def swap_case(s: str) -> str:
    return s.swapcase()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("is_uppercase")
                || result.contains("to_lowercase")
                || result.contains("to_uppercase")
        );
    }

    #[test]
    fn test_w16me_string_030_capitalize_basic() {
        let code = r#"
def cap_first(s: str) -> str:
    return s.capitalize()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("to_uppercase") || result.contains("chars"));
    }

    #[test]
    fn test_w16me_string_031_format_single_arg() {
        let code = r#"
def greet(name: str) -> str:
    return "Hello, {}!".format(name)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("replacen") || result.contains("format"));
    }

    #[test]
    fn test_w16me_string_032_format_two_args() {
        let code = r#"
def greet_full(first: str, last: str) -> str:
    return "{} {}".format(first, last)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("replacen") || result.contains("format"));
    }

    #[test]
    fn test_w16me_string_033_format_no_args() {
        let code = r#"
def no_format() -> str:
    return "plain text".format()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16me_string_034_hex_method() {
        let code = r#"
def to_hex(s: str) -> str:
    return s.hex()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("bytes") || result.contains("hex") || result.contains("format"));
    }

    #[test]
    fn test_w16me_string_035_splitlines_basic() {
        let code = r#"
def get_lines(s: str) -> list:
    return s.splitlines()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("lines") || result.contains("splitlines"));
    }

    #[test]
    fn test_w16me_string_036_index_basic() {
        let code = r#"
def find_index(s: str) -> int:
    return s.index("lo")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("find") || result.contains("expect"));
    }

    #[test]
    fn test_w16me_string_037_rindex_basic() {
        let code = r#"
def find_rindex(s: str) -> int:
    return s.rindex("lo")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rfind") || result.contains("expect"));
    }

    #[test]
    fn test_w16me_string_038_rfind_basic() {
        let code = r#"
def find_last(s: str) -> int:
    return s.rfind("lo")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rfind") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w16me_string_039_isprintable_basic() {
        let code = r#"
def check_printable(s: str) -> bool:
    return s.isprintable()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("is_control") || result.contains("isprintable"));
    }

    #[test]
    fn test_w16me_string_040_isidentifier_basic() {
        let code = r#"
def check_ident(s: str) -> bool:
    return s.isidentifier()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("is_alphabetic") || result.contains("is_alphanumeric"));
    }

    #[test]
    fn test_w16me_string_041_isascii_basic() {
        let code = r#"
def check_ascii(s: str) -> bool:
    return s.isascii()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("is_ascii"));
    }

    #[test]
    fn test_w16me_string_042_isupper_basic() {
        let code = r#"
def check_upper(s: str) -> bool:
    return s.isupper()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("is_uppercase") || result.contains("is_alphabetic"));
    }

    #[test]
    fn test_w16me_string_043_islower_basic() {
        let code = r#"
def check_lower(s: str) -> bool:
    return s.islower()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("is_lowercase") || result.contains("is_alphabetic"));
    }

    #[test]
    fn test_w16me_string_044_istitle_basic() {
        let code = r#"
def check_title(s: str) -> bool:
    return s.istitle()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("is_uppercase")
                || result.contains("is_lowercase")
                || result.contains("prev_is_cased")
        );
    }

    #[test]
    fn test_w16me_string_045_isspace_basic() {
        let code = r#"
def check_space(s: str) -> bool:
    return s.isspace()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("is_whitespace"));
    }

    #[test]
    fn test_w16me_string_046_isalpha_basic() {
        let code = r#"
def check_alpha(s: str) -> bool:
    return s.isalpha()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("is_alphabetic"));
    }

    #[test]
    fn test_w16me_string_047_isalnum_basic() {
        let code = r#"
def check_alnum(s: str) -> bool:
    return s.isalnum()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("is_alphanumeric"));
    }

    #[test]
    fn test_w16me_string_048_count_str_basic() {
        let code = r#"
def count_occ(s: str) -> int:
    return s.count("ab")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("matches") || result.contains("count"));
    }

    #[test]
    fn test_w16me_string_049_rsplit_maxsplit() {
        let code = r#"
def rsplit_once(s: str) -> list:
    return s.rsplit("/", 1)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rsplitn") || result.contains("rsplit"));
    }

    #[test]
    fn test_w16me_string_050_rsplit_no_args() {
        let code = r#"
def rsplit_whitespace(s: str) -> list:
    return s.rsplit()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("split_whitespace") || result.contains("rev"));
    }

    // ========================================================================
    // DICT METHODS (50 tests: test_w16me_dict_051 through test_w16me_dict_100)
    // ========================================================================

    #[test]
    fn test_w16me_dict_051_get_with_default_str() {
        let code = r#"
def get_name(d: dict) -> str:
    return d.get("name", "unknown")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w16me_dict_052_get_with_default_int() {
        let code = r#"
def get_count(d: dict) -> int:
    return d.get("count", 0)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w16me_dict_053_get_single_arg() {
        let code = r#"
def get_val(d: dict, key: str):
    return d.get(key)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("cloned"));
    }

    #[test]
    fn test_w16me_dict_054_popitem_basic() {
        let code = r#"
def pop_any(d: dict) -> tuple:
    return d.popitem()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("keys") || result.contains("remove") || result.contains("popitem"));
    }

    #[test]
    fn test_w16me_dict_055_setdefault_basic() {
        let code = r#"
def ensure_key(d: dict) -> str:
    return d.setdefault("key", "default")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("entry")
                || result.contains("or_insert")
                || result.contains("setdefault")
        );
    }

    #[test]
    fn test_w16me_dict_056_keys_basic() {
        let code = r#"
def get_keys(d: dict) -> list:
    return d.keys()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("keys") || result.contains("collect"));
    }

    #[test]
    fn test_w16me_dict_057_values_basic() {
        let code = r#"
def get_values(d: dict) -> list:
    return d.values()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("values") || result.contains("collect"));
    }

    #[test]
    fn test_w16me_dict_058_items_basic() {
        let code = r#"
def get_items(d: dict) -> list:
    return d.items()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("iter") || result.contains("items") || result.contains("clone"));
    }

    #[test]
    fn test_w16me_dict_059_update_basic() {
        let code = r#"
def merge_dicts(d1: dict, d2: dict):
    d1.update(d2)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert") || result.contains("iter") || result.contains("update"));
    }

    #[test]
    fn test_w16me_dict_060_pop_with_key() {
        let code = r#"
def remove_key(d: dict, key: str):
    return d.pop(key)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("remove") || result.contains("pop"));
    }

    #[test]
    fn test_w16me_dict_061_pop_with_default() {
        let code = r#"
def remove_or_default(d: dict, key: str) -> int:
    return d.pop(key, 0)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("remove") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w16me_dict_062_clear_basic() {
        let code = r#"
def empty_dict(d: dict):
    d.clear()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("clear"));
    }

    #[test]
    fn test_w16me_dict_063_copy_basic() {
        let code = r#"
def clone_dict(d: dict) -> dict:
    return d.copy()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("clone") || result.contains("copy"));
    }

    #[test]
    fn test_w16me_dict_064_dict_literal_access() {
        let code = r#"
def get_from_literal() -> int:
    d = {"a": 1, "b": 2}
    return d.get("a", 0)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("HashMap"));
    }

    #[test]
    fn test_w16me_dict_065_dict_comprehension_basic() {
        let code = r#"
def square_map(nums: list) -> dict:
    return {x: x * x for x in nums}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("collect") || result.contains("map") || result.contains("HashMap"));
    }

    #[test]
    fn test_w16me_dict_066_dict_keys_iteration() {
        let code = r#"
def iterate_keys(d: dict) -> list:
    result = []
    for k in d.keys():
        result.append(k)
    return result
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("keys"));
    }

    #[test]
    fn test_w16me_dict_067_dict_values_iteration() {
        let code = r#"
def iterate_values(d: dict) -> list:
    result = []
    for v in d.values():
        result.append(v)
    return result
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("values"));
    }

    #[test]
    fn test_w16me_dict_068_dict_items_iteration() {
        let code = r#"
def iterate_items(d: dict):
    for k, v in d.items():
        pass
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16me_dict_069_dict_with_int_values() {
        let code = r#"
def counter() -> dict:
    d = {"a": 1, "b": 2, "c": 3}
    return d
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashMap") || result.contains("insert"));
    }

    #[test]
    fn test_w16me_dict_070_dict_get_string_default() {
        let code = r#"
def safe_get(d: dict, key: str) -> str:
    return d.get(key, "none")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w16me_dict_071_dict_pop_string_key() {
        let code = r#"
def remove_str_key(d: dict) -> str:
    return d.pop("key")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("remove") || result.contains("pop"));
    }

    #[test]
    fn test_w16me_dict_072_dict_update_in_func() {
        let code = r#"
def add_entry(d: dict, key: str, val: int):
    other = {key: val}
    d.update(other)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16me_dict_073_nested_dict_literal() {
        let code = r#"
def nested() -> dict:
    return {"outer": {"inner": 42}}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashMap") || result.contains("insert"));
    }

    #[test]
    fn test_w16me_dict_074_dict_empty() {
        let code = r#"
def empty() -> dict:
    return {}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashMap") || result.contains("new"));
    }

    #[test]
    fn test_w16me_dict_075_dict_len() {
        let code = r#"
def size(d: dict) -> int:
    return len(d)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("len"));
    }

    #[test]
    fn test_w16me_dict_076_dict_in_check() {
        let code = r#"
def has_key(d: dict, key: str) -> bool:
    return key in d
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("contains"));
    }

    #[test]
    fn test_w16me_dict_077_dict_not_in_check() {
        let code = r#"
def missing_key(d: dict, key: str) -> bool:
    return key not in d
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("contains"));
    }

    #[test]
    fn test_w16me_dict_078_dict_literal_get_int() {
        let code = r#"
def get_int() -> int:
    d = {"x": 10, "y": 20}
    return d.get("x", 0)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16me_dict_079_dict_setdefault_int() {
        let code = r#"
def set_or_get(d: dict) -> int:
    return d.setdefault("count", 0)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("entry") || result.contains("or_insert"));
    }

    #[test]
    fn test_w16me_dict_080_dict_keys_list() {
        let code = r#"
def key_list(d: dict) -> list:
    return list(d.keys())
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("keys"));
    }

    #[test]
    fn test_w16me_dict_081_dict_values_list() {
        let code = r#"
def val_list(d: dict) -> list:
    return list(d.values())
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("values"));
    }

    #[test]
    fn test_w16me_dict_082_dict_fromkeys_basic() {
        let code = r#"
def make_dict(keys: list) -> dict:
    return dict.fromkeys(keys, 0)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("iter") || result.contains("map") || result.contains("collect"));
    }

    #[test]
    fn test_w16me_dict_083_dict_fromkeys_no_default() {
        let code = r#"
def make_dict_none(keys: list) -> dict:
    return dict.fromkeys(keys)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16me_dict_084_dict_pop_with_str_default() {
        let code = r#"
def safe_pop(d: dict, key: str) -> str:
    return d.pop(key, "missing")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("remove") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w16me_dict_085_dict_get_with_var_key() {
        let code = r#"
def lookup(d: dict, key: str) -> int:
    return d.get(key, 0)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get"));
    }

    #[test]
    fn test_w16me_dict_086_dict_comprehension_filter() {
        let code = r#"
def filter_dict(d: dict) -> dict:
    return {k: v for k, v in d.items() if v > 0}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("filter") || result.contains("collect") || result.contains("HashMap")
        );
    }

    #[test]
    fn test_w16me_dict_087_dict_setdefault_list() {
        let code = r#"
def set_default_list(d: dict, key: str) -> list:
    return d.setdefault(key, [])
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("entry") || result.contains("or_insert"));
    }

    #[test]
    fn test_w16me_dict_088_dict_multi_ops() {
        let code = r#"
def dict_ops(d: dict):
    d.clear()
    return len(d)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("clear"));
    }

    #[test]
    fn test_w16me_dict_089_dict_copy_and_modify() {
        let code = r#"
def copy_and_add(d: dict) -> dict:
    result = d.copy()
    return result
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("clone") || result.contains("copy"));
    }

    #[test]
    fn test_w16me_dict_090_dict_popitem_in_func() {
        let code = r#"
def drain_one(d: dict):
    item = d.popitem()
    return item
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16me_dict_091_dict_get_literal_key() {
        let code = r#"
def get_literal(d: dict) -> int:
    return d.get("x", 0)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get"));
    }

    #[test]
    fn test_w16me_dict_092_dict_update_and_read() {
        let code = r#"
def update_read(d: dict, extra: dict):
    d.update(extra)
    return d.keys()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16me_dict_093_dict_pop_int_key() {
        let code = r#"
def pop_int_key(d: dict) -> str:
    return d.pop(1)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("remove"));
    }

    #[test]
    fn test_w16me_dict_094_dict_multiple_gets() {
        let code = r#"
def multi_get(d: dict) -> list:
    a = d.get("a", 0)
    b = d.get("b", 0)
    return [a, b]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get"));
    }

    #[test]
    fn test_w16me_dict_095_dict_values_sum() {
        let code = r#"
def total(d: dict) -> int:
    return sum(d.values())
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("values") || result.contains("sum"));
    }

    #[test]
    fn test_w16me_dict_096_dict_items_to_list() {
        let code = r#"
def items_list(d: dict) -> list:
    return list(d.items())
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16me_dict_097_dict_keys_sorted() {
        let code = r#"
def sorted_keys(d: dict) -> list:
    return sorted(d.keys())
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("keys") || result.contains("sort"));
    }

    #[test]
    fn test_w16me_dict_098_dict_len_check() {
        let code = r#"
def is_empty(d: dict) -> bool:
    return len(d) == 0
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("len"));
    }

    #[test]
    fn test_w16me_dict_099_dict_get_no_default_str() {
        let code = r#"
def get_optional(d: dict, key: str):
    return d.get(key)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("cloned"));
    }

    #[test]
    fn test_w16me_dict_100_dict_setdefault_str() {
        let code = r#"
def set_str_default(d: dict) -> str:
    return d.setdefault("name", "anon")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("entry") || result.contains("or_insert"));
    }

    // ========================================================================
    // LIST METHODS (50 tests: test_w16me_list_101 through test_w16me_list_150)
    // ========================================================================

    #[test]
    fn test_w16me_list_101_pop_no_args() {
        let code = r#"
def pop_last(lst: list) -> int:
    return lst.pop()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("pop") || result.contains("unwrap_or_default"));
    }

    #[test]
    fn test_w16me_list_102_pop_with_index_zero() {
        let code = r#"
def pop_first(lst: list) -> int:
    return lst.pop(0)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("remove") || result.contains("pop"));
    }

    #[test]
    fn test_w16me_list_103_pop_with_index_two() {
        let code = r#"
def pop_at(lst: list) -> int:
    return lst.pop(2)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("remove") || result.contains("pop"));
    }

    #[test]
    fn test_w16me_list_104_sort_no_args() {
        let code = r#"
def sort_list(lst: list):
    lst.sort()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("sort"));
    }

    #[test]
    fn test_w16me_list_105_sort_reverse_true() {
        let code = r#"
def sort_desc(lst: list):
    lst.sort(reverse=True)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("sort_by") || result.contains("cmp") || result.contains("sort"));
    }

    #[test]
    fn test_w16me_list_106_sort_key_len() {
        let code = r#"
def sort_by_len(lst: list):
    lst.sort(key=len)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("sort_by_key") || result.contains("sort"));
    }

    #[test]
    fn test_w16me_list_107_insert_at_zero() {
        let code = r#"
def prepend(lst: list, val: int):
    lst.insert(0, val)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w16me_list_108_insert_at_middle() {
        let code = r#"
def insert_mid(lst: list, val: int):
    lst.insert(2, val)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w16me_list_109_extend_basic() {
        let code = r#"
def combine(lst: list, other: list):
    lst.extend(other)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("extend") || result.contains("iter"));
    }

    #[test]
    fn test_w16me_list_110_count_basic() {
        let code = r#"
def count_val(lst: list, val: int) -> int:
    return lst.count(val)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("filter") || result.contains("count"));
    }

    #[test]
    fn test_w16me_list_111_index_basic() {
        let code = r#"
def find_pos(lst: list, val: int) -> int:
    return lst.index(val)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("position") || result.contains("index"));
    }

    #[test]
    fn test_w16me_list_112_copy_basic() {
        let code = r#"
def clone_list(lst: list) -> list:
    return lst.copy()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("clone") || result.contains("copy"));
    }

    #[test]
    fn test_w16me_list_113_clear_basic() {
        let code = r#"
def empty_list(lst: list):
    lst.clear()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("clear"));
    }

    #[test]
    fn test_w16me_list_114_reverse_basic() {
        let code = r#"
def flip_list(lst: list):
    lst.reverse()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("reverse"));
    }

    #[test]
    fn test_w16me_list_115_remove_basic() {
        let code = r#"
def remove_val(lst: list, val: int):
    lst.remove(val)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("position") || result.contains("remove"));
    }

    #[test]
    fn test_w16me_list_116_append_int() {
        let code = r#"
def add_item(lst: list, val: int):
    lst.append(val)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("push") || result.contains("append"));
    }

    #[test]
    fn test_w16me_list_117_append_str() {
        let code = r#"
def add_name(lst: list, name: str):
    lst.append(name)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("push"));
    }

    #[test]
    fn test_w16me_list_118_list_literal_ops() {
        let code = r#"
def list_ops() -> int:
    lst = [1, 2, 3, 4, 5]
    return len(lst)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("len") || result.contains("vec!"));
    }

    #[test]
    fn test_w16me_list_119_nested_list() {
        let code = r#"
def nested() -> list:
    return [[1, 2], [3, 4]]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("vec!") || result.contains("Vec"));
    }

    #[test]
    fn test_w16me_list_120_list_of_tuples() {
        let code = r#"
def pairs() -> list:
    return [(1, 2), (3, 4)]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("vec!") || result.contains("Vec"));
    }

    #[test]
    fn test_w16me_list_121_list_comprehension_basic() {
        let code = r#"
def squares(n: int) -> list:
    return [x * x for x in range(n)]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("map") || result.contains("collect"));
    }

    #[test]
    fn test_w16me_list_122_list_comprehension_filter() {
        let code = r#"
def evens(n: int) -> list:
    return [x for x in range(n) if x % 2 == 0]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("filter") || result.contains("collect"));
    }

    #[test]
    fn test_w16me_list_123_sort_reverse_false() {
        let code = r#"
def sort_asc(lst: list):
    lst.sort(reverse=False)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("sort"));
    }

    #[test]
    fn test_w16me_list_124_pop_and_use() {
        let code = r#"
def pop_and_store(lst: list) -> int:
    val = lst.pop()
    return val
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("pop"));
    }

    #[test]
    fn test_w16me_list_125_extend_and_sort() {
        let code = r#"
def merge_sorted(lst: list, other: list):
    lst.extend(other)
    lst.sort()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("extend"));
        assert!(result.contains("sort"));
    }

    #[test]
    fn test_w16me_list_126_insert_and_remove() {
        let code = r#"
def insert_remove(lst: list, val: int):
    lst.insert(0, val)
    lst.remove(val)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w16me_list_127_count_and_check() {
        let code = r#"
def has_item(lst: list, val: int) -> bool:
    return lst.count(val) > 0
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("filter") || result.contains("count"));
    }

    #[test]
    fn test_w16me_list_128_reverse_and_return() {
        let code = r#"
def reversed_copy(lst: list) -> list:
    result = lst.copy()
    result.reverse()
    return result
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("clone") || result.contains("reverse"));
    }

    #[test]
    fn test_w16me_list_129_clear_and_rebuild() {
        let code = r#"
def reset_list(lst: list):
    lst.clear()
    lst.append(0)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("clear"));
        assert!(result.contains("push"));
    }

    #[test]
    fn test_w16me_list_130_index_and_pop() {
        let code = r#"
def find_and_pop(lst: list, val: int) -> int:
    idx = lst.index(val)
    return lst.pop(idx)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("position") || result.contains("index"));
    }

    #[test]
    fn test_w16me_list_131_append_string_literal() {
        let code = r#"
def add_str(lst: list):
    lst.append("hello")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("push"));
    }

    #[test]
    fn test_w16me_list_132_extend_empty() {
        let code = r#"
def extend_empty(lst: list):
    lst.extend([])
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("extend"));
    }

    #[test]
    fn test_w16me_list_133_list_len() {
        let code = r#"
def size(lst: list) -> int:
    return len(lst)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("len"));
    }

    #[test]
    fn test_w16me_list_134_list_in_check() {
        let code = r#"
def has_val(lst: list, val: int) -> bool:
    return val in lst
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("contains"));
    }

    #[test]
    fn test_w16me_list_135_list_not_in_check() {
        let code = r#"
def missing_val(lst: list, val: int) -> bool:
    return val not in lst
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("contains"));
    }

    #[test]
    fn test_w16me_list_136_list_concat() {
        let code = r#"
def concat(a: list, b: list) -> list:
    return a + b
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16me_list_137_list_empty_check() {
        let code = r#"
def is_empty(lst: list) -> bool:
    return len(lst) == 0
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("len"));
    }

    #[test]
    fn test_w16me_list_138_list_min() {
        let code = r#"
def minimum(lst: list) -> int:
    return min(lst)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("min") || result.contains("iter"));
    }

    #[test]
    fn test_w16me_list_139_list_max() {
        let code = r#"
def maximum(lst: list) -> int:
    return max(lst)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("max") || result.contains("iter"));
    }

    #[test]
    fn test_w16me_list_140_list_sum() {
        let code = r#"
def total(lst: list) -> int:
    return sum(lst)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("sum") || result.contains("iter"));
    }

    #[test]
    fn test_w16me_list_141_list_sorted() {
        let code = r#"
def sorted_copy(lst: list) -> list:
    return sorted(lst)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("sort") || result.contains("sorted"));
    }

    #[test]
    fn test_w16me_list_142_list_reversed() {
        let code = r#"
def reversed_iter(lst: list) -> list:
    return list(reversed(lst))
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16me_list_143_list_enumerate() {
        let code = r#"
def with_index(lst: list) -> list:
    return list(enumerate(lst))
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("enumerate") || result.contains("iter"));
    }

    #[test]
    fn test_w16me_list_144_list_zip() {
        let code = r#"
def combine(a: list, b: list) -> list:
    return list(zip(a, b))
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("zip") || result.contains("iter"));
    }

    #[test]
    fn test_w16me_list_145_list_bool_check() {
        let code = r#"
def check(lst: list) -> bool:
    if lst:
        return True
    return False
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16me_list_146_sort_key_reverse_true() {
        let code = r#"
def sort_key_desc(lst: list):
    lst.sort(key=len, reverse=True)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("sort") || result.contains("Reverse") || result.contains("sort_by_key")
        );
    }

    #[test]
    fn test_w16me_list_147_list_slice() {
        let code = r#"
def first_three(lst: list) -> list:
    return lst[0:3]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16me_list_148_list_negative_index() {
        let code = r#"
def last_item(lst: list) -> int:
    return lst[-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16me_list_149_pop_index_one() {
        let code = r#"
def pop_second(lst: list) -> int:
    return lst.pop(1)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("remove") || result.contains("pop"));
    }

    #[test]
    fn test_w16me_list_150_append_and_len() {
        let code = r#"
def add_and_count(lst: list, val: int) -> int:
    lst.append(val)
    return len(lst)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("push"));
        assert!(result.contains("len"));
    }

    // ========================================================================
    // SET METHODS (50 tests: test_w16me_set_151 through test_w16me_set_200)
    // ========================================================================

    #[test]
    fn test_w16me_set_151_add_int() {
        let code = r#"
def add_to_set(s: set, val: int):
    s.add(val)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w16me_set_152_add_str() {
        let code = r#"
def add_name(s: set):
    s.add("hello")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w16me_set_153_discard_int() {
        let code = r#"
def discard_val(s: set, val: int):
    s.discard(val)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("remove"));
    }

    #[test]
    fn test_w16me_set_154_discard_str() {
        let code = r#"
def discard_name(s: set):
    s.discard("hello")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("remove"));
    }

    #[test]
    fn test_w16me_set_155_remove_int() {
        let code = r#"
def remove_val(s: set, val: int):
    s.remove(val)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("remove"));
    }

    #[test]
    fn test_w16me_set_156_remove_str() {
        let code = r#"
def remove_name(s: set):
    s.remove("hello")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("remove"));
    }

    #[test]
    fn test_w16me_set_157_clear_basic() {
        let code = r#"
def empty_set(s: set):
    s.clear()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("clear"));
    }

    #[test]
    fn test_w16me_set_158_union_basic() {
        let code = r#"
def merge_sets(a: set, b: set) -> set:
    return a.union(b)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("union") || result.contains("collect"));
    }

    #[test]
    fn test_w16me_set_159_intersection_basic() {
        let code = r#"
def common_elements(a: set, b: set) -> set:
    return a.intersection(b)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("intersection") || result.contains("collect"));
    }

    #[test]
    fn test_w16me_set_160_difference_basic() {
        let code = r#"
def only_in_first(a: set, b: set) -> set:
    return a.difference(b)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("difference") || result.contains("collect"));
    }

    #[test]
    fn test_w16me_set_161_symmetric_difference_basic() {
        let code = r#"
def unique_to_each(a: set, b: set) -> set:
    return a.symmetric_difference(b)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("symmetric_difference") || result.contains("collect"));
    }

    #[test]
    fn test_w16me_set_162_issubset_basic() {
        let code = r#"
def check_subset(a: set, b: set) -> bool:
    return a.issubset(b)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("is_subset") || result.contains("issubset"));
    }

    #[test]
    fn test_w16me_set_163_issuperset_basic() {
        let code = r#"
def check_superset(a: set, b: set) -> bool:
    return a.issuperset(b)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("is_superset") || result.contains("issuperset"));
    }

    #[test]
    fn test_w16me_set_164_isdisjoint_basic() {
        let code = r#"
def check_disjoint(a: set, b: set) -> bool:
    return a.isdisjoint(b)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("is_disjoint") || result.contains("isdisjoint"));
    }

    #[test]
    fn test_w16me_set_165_update_basic() {
        let code = r#"
def extend_set(a: set, b: set):
    a.update(b)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert") || result.contains("update"));
    }

    #[test]
    fn test_w16me_set_166_intersection_update_basic() {
        let code = r#"
def keep_common(a: set, b: set):
    a.intersection_update(b)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("intersection")
                || result.contains("clear")
                || result.contains("extend")
        );
    }

    #[test]
    fn test_w16me_set_167_difference_update_basic() {
        let code = r#"
def remove_common(a: set, b: set):
    a.difference_update(b)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("difference") || result.contains("clear") || result.contains("extend")
        );
    }

    #[test]
    fn test_w16me_set_168_set_comprehension_basic() {
        let code = r#"
def unique_squares(n: int) -> set:
    return {x * x for x in range(n)}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("collect") || result.contains("HashSet") || result.contains("map"));
    }

    #[test]
    fn test_w16me_set_169_set_from_list() {
        let code = r#"
def deduplicate(lst: list) -> set:
    return set(lst)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("collect")
                || result.contains("HashSet")
                || result.contains("into_iter")
        );
    }

    #[test]
    fn test_w16me_set_170_set_len() {
        let code = r#"
def set_size(s: set) -> int:
    return len(s)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("len"));
    }

    #[test]
    fn test_w16me_set_171_set_in_check() {
        let code = r#"
def has_element(s: set, val: int) -> bool:
    return val in s
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("contains"));
    }

    #[test]
    fn test_w16me_set_172_set_not_in_check() {
        let code = r#"
def missing_element(s: set, val: int) -> bool:
    return val not in s
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("contains"));
    }

    #[test]
    fn test_w16me_set_173_set_literal() {
        let code = r#"
def make_set() -> set:
    return {1, 2, 3}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashSet") || result.contains("insert") || result.contains("from"));
    }

    #[test]
    fn test_w16me_set_174_set_add_and_check() {
        let code = r#"
def add_and_check(s: set, val: int) -> bool:
    s.add(val)
    return val in s
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w16me_set_175_set_discard_and_len() {
        let code = r#"
def discard_and_count(s: set, val: int) -> int:
    s.discard(val)
    return len(s)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("remove"));
    }

    #[test]
    fn test_w16me_set_176_set_union_multiple_ops() {
        let code = r#"
def union_ops(a: set, b: set) -> int:
    result = a.union(b)
    return len(result)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("union"));
    }

    #[test]
    fn test_w16me_set_177_set_intersection_and_len() {
        let code = r#"
def common_count(a: set, b: set) -> int:
    result = a.intersection(b)
    return len(result)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("intersection"));
    }

    #[test]
    fn test_w16me_set_178_set_difference_and_len() {
        let code = r#"
def diff_count(a: set, b: set) -> int:
    result = a.difference(b)
    return len(result)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("difference"));
    }

    #[test]
    fn test_w16me_set_179_set_symmetric_diff_and_len() {
        let code = r#"
def sym_diff_count(a: set, b: set) -> int:
    result = a.symmetric_difference(b)
    return len(result)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("symmetric_difference"));
    }

    #[test]
    fn test_w16me_set_180_set_clear_and_rebuild() {
        let code = r#"
def reset_set(s: set):
    s.clear()
    s.add(0)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("clear"));
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w16me_set_181_set_iter_basic() {
        let code = r#"
def iterate_set(s: set) -> list:
    result = []
    for item in s:
        result.append(item)
    return result
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("for") || result.contains("push"));
    }

    #[test]
    fn test_w16me_set_182_set_comprehension_filter() {
        let code = r#"
def even_set(n: int) -> set:
    return {x for x in range(n) if x % 2 == 0}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("filter") || result.contains("collect") || result.contains("HashSet")
        );
    }

    #[test]
    fn test_w16me_set_183_set_add_multiple() {
        let code = r#"
def add_many(s: set):
    s.add(1)
    s.add(2)
    s.add(3)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w16me_set_184_set_discard_missing() {
        let code = r#"
def safe_discard(s: set, val: int):
    s.discard(val)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("remove"));
    }

    #[test]
    fn test_w16me_set_185_set_issubset_check() {
        let code = r#"
def is_contained(small: set, big: set) -> bool:
    return small.issubset(big)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("is_subset"));
    }

    #[test]
    fn test_w16me_set_186_set_issuperset_check() {
        let code = r#"
def contains_all(big: set, small: set) -> bool:
    return big.issuperset(small)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("is_superset"));
    }

    #[test]
    fn test_w16me_set_187_set_isdisjoint_check() {
        let code = r#"
def no_overlap(a: set, b: set) -> bool:
    return a.isdisjoint(b)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("is_disjoint"));
    }

    #[test]
    fn test_w16me_set_188_set_update_and_check() {
        let code = r#"
def merge_and_check(a: set, b: set) -> int:
    a.update(b)
    return len(a)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16me_set_189_set_empty_literal() {
        let code = r#"
def make_empty() -> set:
    return set()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashSet") || result.contains("new"));
    }

    #[test]
    fn test_w16me_set_190_set_from_range() {
        let code = r#"
def range_set(n: int) -> set:
    return set(range(n))
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("collect") || result.contains("HashSet"));
    }

    #[test]
    fn test_w16me_set_191_set_union_return() {
        let code = r#"
def merged(a: set, b: set) -> set:
    return a.union(b)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("union") || result.contains("collect"));
    }

    #[test]
    fn test_w16me_set_192_set_intersection_return() {
        let code = r#"
def shared(a: set, b: set) -> set:
    return a.intersection(b)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("intersection"));
    }

    #[test]
    fn test_w16me_set_193_set_difference_return() {
        let code = r#"
def exclusive(a: set, b: set) -> set:
    return a.difference(b)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("difference"));
    }

    #[test]
    fn test_w16me_set_194_set_symmetric_diff_return() {
        let code = r#"
def xor_set(a: set, b: set) -> set:
    return a.symmetric_difference(b)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("symmetric_difference"));
    }

    #[test]
    fn test_w16me_set_195_set_add_and_discard() {
        let code = r#"
def add_then_discard(s: set, val: int):
    s.add(val)
    s.discard(val)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert"));
        assert!(result.contains("remove"));
    }

    #[test]
    fn test_w16me_set_196_set_update_and_diff() {
        let code = r#"
def update_then_diff(a: set, b: set, c: set):
    a.update(b)
    result = a.difference(c)
    return result
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16me_set_197_set_bool_check() {
        let code = r#"
def set_truthy(s: set) -> bool:
    if s:
        return True
    return False
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16me_set_198_set_add_string_var() {
        let code = r#"
def add_str_var(s: set, name: str):
    s.add(name)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w16me_set_199_set_intersection_update_ops() {
        let code = r#"
def keep_common_only(a: set, b: set) -> int:
    a.intersection_update(b)
    return len(a)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16me_set_200_set_difference_update_ops() {
        let code = r#"
def remove_overlap(a: set, b: set) -> int:
    a.difference_update(b)
    return len(a)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }
}
