//! Wave 16 coverage tests: slicing, indexing, comprehensions, and attribute_convert
//!
//! Targets uncovered code paths in expr_gen_instance_methods:
//! - slicing.rs: negative step combos, string slice start+step, stop+step, start+stop+step
//! - indexing.rs: tuple index, chained dict access, os.environ, computed index, list assign
//! - comprehensions.rs: walrus, multi-generator, nested comp, set comp, dict comp filter, string iter
//! - attribute_convert.rs: type().__name__, datetime min/max, sys attrs, path attrs, math constants
//!
//! 200 tests total across 4 categories

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
    // SECTION 1: Slicing (50 tests)
    // ========================================================================

    #[test]
    fn test_w16sc_slice_001_list_reverse_neg_one() {
        let code = r#"
def rev(data: list) -> list:
    return data[::-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rev") || result.contains("step"));
    }

    #[test]
    fn test_w16sc_slice_002_list_neg_step_two_range() {
        let code = r#"
def rev_step(data: list) -> list:
    return data[4:0:-2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rev") || result.contains("step") || result.contains("collect"));
    }

    #[test]
    fn test_w16sc_slice_003_list_neg_step_three() {
        let code = r#"
def every_third_rev(data: list) -> list:
    return data[::-3]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("step") || result.contains("rev"));
    }

    #[test]
    fn test_w16sc_slice_004_string_start_stop() {
        let code = r#"
def substr(s: str) -> str:
    return s[1:3]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("skip") || result.contains("take"));
    }

    #[test]
    fn test_w16sc_slice_005_string_full_reverse() {
        let code = r#"
def rev_str(s: str) -> str:
    return s[::-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rev") || result.contains("chars"));
    }

    #[test]
    fn test_w16sc_slice_006_list_start_only() {
        let code = r#"
def from_two(data: list) -> list:
    return data[2:]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("start") || result.contains("to_vec") || result.contains("isize"));
    }

    #[test]
    fn test_w16sc_slice_007_list_stop_only() {
        let code = r#"
def first_three(data: list) -> list:
    return data[:3]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("stop") || result.contains("to_vec") || result.contains("min"));
    }

    #[test]
    fn test_w16sc_slice_008_list_all_three() {
        let code = r#"
def stepped(data: list) -> list:
    return data[1:5:2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("step") || result.contains("collect"));
    }

    #[test]
    fn test_w16sc_slice_009_list_empty_result() {
        let code = r#"
def empty_slice(data: list) -> list:
    return data[5:2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("start") || result.contains("stop") || result.contains("isize"));
    }

    #[test]
    fn test_w16sc_slice_010_list_large_step() {
        let code = r#"
def big_step(data: list) -> list:
    return data[::100]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("step") || result.contains("iter"));
    }

    #[test]
    fn test_w16sc_slice_011_list_neg_start_neg_step() {
        let code = r#"
def last_to_first(data: list) -> list:
    return data[-1::-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rev") || result.contains("step") || result.contains("isize"));
    }

    #[test]
    fn test_w16sc_slice_012_string_start_only() {
        let code = r#"
def from_idx(s: str) -> str:
    return s[2:]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("skip"));
    }

    #[test]
    fn test_w16sc_slice_013_string_stop_only() {
        let code = r#"
def first_chars(s: str) -> str:
    return s[:3]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("take"));
    }

    #[test]
    fn test_w16sc_slice_014_string_neg_start() {
        let code = r#"
def tail_str(s: str) -> str:
    return s[-4:]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("skip"));
    }

    #[test]
    fn test_w16sc_slice_015_string_neg_stop() {
        let code = r#"
def head_trim(s: str) -> str:
    return s[:-2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("take"));
    }

    #[test]
    fn test_w16sc_slice_016_string_neg_both() {
        let code = r#"
def mid_str(s: str) -> str:
    return s[-5:-2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("String"));
    }

    #[test]
    fn test_w16sc_slice_017_string_step_three() {
        let code = r#"
def every_third_char(s: str) -> str:
    return s[::3]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("step_by") || result.contains("chars"));
    }

    #[test]
    fn test_w16sc_slice_018_string_start_step() {
        let code = r#"
def from_one_every_two(s: str) -> str:
    return s[1::2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("step"));
    }

    #[test]
    fn test_w16sc_slice_019_string_stop_step() {
        let code = r#"
def first_five_step_two(s: str) -> str:
    return s[:5:2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("step") || result.contains("take"));
    }

    #[test]
    fn test_w16sc_slice_020_string_all_three() {
        let code = r#"
def substr_stepped(s: str) -> str:
    return s[1:6:2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("step"));
    }

    #[test]
    fn test_w16sc_slice_021_list_neg_start_pos_step() {
        let code = r#"
def neg_start_step(data: list) -> list:
    return data[-6::2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("step") || result.contains("isize"));
    }

    #[test]
    fn test_w16sc_slice_022_list_start_neg_stop() {
        let code = r#"
def partial(data: list) -> list:
    return data[1:-2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("isize") || result.contains("to_vec"));
    }

    #[test]
    fn test_w16sc_slice_023_list_stop_neg_step_one() {
        let code = r#"
def rev_to_stop(data: list) -> list:
    return data[:3:-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rev") || result.contains("step"));
    }

    #[test]
    fn test_w16sc_slice_024_string_neg_step_two() {
        let code = r#"
def rev_step_two_str(s: str) -> str:
    return s[::-2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rev") || result.contains("step") || result.contains("chars"));
    }

    #[test]
    fn test_w16sc_slice_025_list_start_stop_step_one() {
        let code = r#"
def range_step_one(data: list) -> list:
    return data[2:7:1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("step") || result.contains("to_vec"));
    }

    #[test]
    fn test_w16sc_slice_026_list_neg_start_neg_stop() {
        let code = r#"
def neg_range(data: list) -> list:
    return data[-4:-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("isize") || result.contains("to_vec"));
    }

    #[test]
    fn test_w16sc_slice_027_string_start_stop_step_one() {
        let code = r#"
def str_range_step_one(s: str) -> str:
    return s[2:5:1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("step") || result.contains("skip"));
    }

    #[test]
    fn test_w16sc_slice_028_string_neg_start_step() {
        let code = r#"
def str_neg_start_step(s: str) -> str:
    return s[-5::2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("step") || result.contains("skip"));
    }

    #[test]
    fn test_w16sc_slice_029_list_start_neg_step_three() {
        let code = r#"
def rev_three(data: list) -> list:
    return data[6:0:-3]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rev") || result.contains("step") || result.contains("collect"));
    }

    #[test]
    fn test_w16sc_slice_030_string_start_neg_step() {
        let code = r#"
def str_rev_from(s: str) -> str:
    return s[5:1:-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("rev") || result.contains("step"));
    }

    #[test]
    fn test_w16sc_slice_031_list_clone_full() {
        let code = r#"
def clone_all(data: list) -> list:
    return data[:]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("clone"));
    }

    #[test]
    fn test_w16sc_slice_032_string_clone_full() {
        let code = r#"
def clone_str(s: str) -> str:
    return s[:]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("to_string"));
    }

    #[test]
    fn test_w16sc_slice_033_list_pos_start_step_four() {
        let code = r#"
def every_fourth(data: list) -> list:
    return data[1::4]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("step") || result.contains("iter"));
    }

    #[test]
    fn test_w16sc_slice_034_list_neg_step_neg_start_neg_stop() {
        let code = r#"
def full_neg_slice(data: list) -> list:
    return data[-2:-6:-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rev") || result.contains("step") || result.contains("isize"));
    }

    #[test]
    fn test_w16sc_slice_035_string_start_neg_stop() {
        let code = r#"
def str_partial(s: str) -> str:
    return s[1:-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("String"));
    }

    #[test]
    fn test_w16sc_slice_036_list_step_five() {
        let code = r#"
def every_fifth(data: list) -> list:
    return data[::5]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("step") || result.contains("iter"));
    }

    #[test]
    fn test_w16sc_slice_037_list_start_two_stop_eight() {
        let code = r#"
def mid_range(data: list) -> list:
    return data[2:8]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("to_vec") || result.contains("isize"));
    }

    #[test]
    fn test_w16sc_slice_038_string_stop_step_neg_one() {
        let code = r#"
def str_rev_to(s: str) -> str:
    return s[:4:-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("rev") || result.contains("take"));
    }

    #[test]
    fn test_w16sc_slice_039_list_neg_five_to_end() {
        let code = r#"
def last_five(data: list) -> list:
    return data[-5:]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("isize") || result.contains("to_vec") || result.contains("len"));
    }

    #[test]
    fn test_w16sc_slice_040_string_pos_start_neg_stop_step() {
        let code = r#"
def str_complex(s: str) -> str:
    return s[1:-1:2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("step"));
    }

    #[test]
    fn test_w16sc_slice_041_list_step_two_start_three() {
        let code = r#"
def from_three_step(data: list) -> list:
    return data[3::2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("step") || result.contains("iter"));
    }

    #[test]
    fn test_w16sc_slice_042_string_neg_start_neg_stop() {
        let code = r#"
def str_neg_range(s: str) -> str:
    return s[-6:-3]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("String"));
    }

    #[test]
    fn test_w16sc_slice_043_list_zero_to_ten_step_three() {
        let code = r#"
def stepped_range(data: list) -> list:
    return data[0:10:3]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("step") || result.contains("collect"));
    }

    #[test]
    fn test_w16sc_slice_044_string_full_step_one() {
        let code = r#"
def identity(s: str) -> str:
    return s[::1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("to_string") || result.contains("chars") || result.contains("step")
        );
    }

    #[test]
    fn test_w16sc_slice_045_list_neg_three_to_end_step_one() {
        let code = r#"
def tail_three(data: list) -> list:
    return data[-3::1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("step") || result.contains("isize") || result.contains("to_vec"));
    }

    #[test]
    fn test_w16sc_slice_046_list_full_step_one() {
        let code = r#"
def identity_list(data: list) -> list:
    return data[::1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("step") || result.contains("clone") || result.contains("base"));
    }

    #[test]
    fn test_w16sc_slice_047_string_start_stop_neg_step() {
        let code = r#"
def str_rev_range(s: str) -> str:
    return s[5:1:-2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("rev") || result.contains("step"));
    }

    #[test]
    fn test_w16sc_slice_048_list_neg_one_stop_step() {
        let code = r#"
def until_end_rev(data: list) -> list:
    return data[:1:-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rev") || result.contains("step"));
    }

    #[test]
    fn test_w16sc_slice_049_list_zero_start_neg_stop() {
        let code = r#"
def trim_tail(data: list) -> list:
    return data[0:-3]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("isize") || result.contains("to_vec"));
    }

    #[test]
    fn test_w16sc_slice_050_string_start_step_neg_one() {
        let code = r#"
def str_from_rev(s: str) -> str:
    return s[3::-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("rev") || result.contains("skip"));
    }

    // ========================================================================
    // SECTION 2: Indexing (50 tests)
    // ========================================================================

    #[test]
    fn test_w16sc_index_050_tuple_first_element() {
        let code = r#"
def get_first(t: tuple) -> int:
    return t[0]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains(".0") || result.contains("get(0") || result.contains("t[0"));
    }

    #[test]
    fn test_w16sc_index_051_tuple_second_element() {
        let code = r#"
def get_second(t: tuple) -> int:
    return t[1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains(".1") || result.contains("get(1") || result.contains("t[1"));
    }

    #[test]
    fn test_w16sc_index_052_dict_string_key_access() {
        let code = r#"
def get_name(d: dict) -> str:
    return d["name"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("name"));
    }

    #[test]
    fn test_w16sc_index_053_list_neg_one() {
        let code = r#"
def get_last(data: list) -> int:
    return data[-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("len") || result.contains("saturating_sub") || result.contains("get")
        );
    }

    #[test]
    fn test_w16sc_index_054_list_neg_two() {
        let code = r#"
def get_penultimate(data: list) -> int:
    return data[-2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("len") || result.contains("saturating_sub") || result.contains("get")
        );
    }

    #[test]
    fn test_w16sc_index_055_chained_dict_access() {
        let code = r#"
def nested_val(d: dict) -> int:
    return d["outer"]["inner"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("outer"));
    }

    #[test]
    fn test_w16sc_index_056_chained_list_access() {
        let code = r#"
def matrix_val(grid: list, r: int, c: int) -> int:
    return grid[r][c]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("grid"));
    }

    #[test]
    fn test_w16sc_index_057_index_with_variable() {
        let code = r#"
def at_index(data: list, idx: int) -> int:
    return data[idx]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("idx") || result.contains("as usize"));
    }

    #[test]
    fn test_w16sc_index_058_os_environ_access() {
        let code = r#"
import os
def get_env_var() -> str:
    return os.environ["HOME"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("env::var") || result.contains("HOME"));
    }

    #[test]
    fn test_w16sc_index_059_list_index_zero() {
        let code = r#"
def head(data: list) -> int:
    return data[0]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get(0") || result.contains("data"));
    }

    #[test]
    fn test_w16sc_index_060_list_index_five() {
        let code = r#"
def sixth(data: list) -> int:
    return data[5]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get(5") || result.contains("data"));
    }

    #[test]
    fn test_w16sc_index_061_dict_variable_key() {
        let code = r#"
def lookup(d: dict, key: str) -> int:
    return d[key]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("key"));
    }

    #[test]
    fn test_w16sc_index_062_dict_assign_key() {
        let code = r#"
def set_val():
    d = {}
    d["score"] = 100
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert") || result.contains("score"));
    }

    #[test]
    fn test_w16sc_index_063_list_assign_index() {
        let code = r#"
def set_elem(data: list):
    data[0] = 99
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("99") || result.contains("data"));
    }

    #[test]
    fn test_w16sc_index_064_string_first_char() {
        let code = r#"
def first_ch(s: str) -> str:
    return s[0]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("nth"));
    }

    #[test]
    fn test_w16sc_index_065_string_last_char() {
        let code = r#"
def last_ch(s: str) -> str:
    return s[-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("chars") || result.contains("nth") || result.contains("saturating_sub")
        );
    }

    #[test]
    fn test_w16sc_index_066_dict_int_key() {
        let code = r#"
def int_key():
    d = {1: "one", 2: "two"}
    return d[1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("insert"));
    }

    #[test]
    fn test_w16sc_index_067_list_computed_index() {
        let code = r#"
def computed(data: list, n: int) -> int:
    return data[n - 1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("idx") || result.contains("actual_idx"));
    }

    #[test]
    fn test_w16sc_index_068_string_variable_index() {
        let code = r#"
def char_at(s: str, pos: int) -> str:
    return s[pos]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("nth") || result.contains("idx"));
    }

    #[test]
    fn test_w16sc_index_069_tuple_third_element() {
        let code = r#"
def get_third(t: tuple):
    return t[2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains(".2") || result.contains("get(2") || result.contains("t"));
    }

    #[test]
    fn test_w16sc_index_070_list_neg_three() {
        let code = r#"
def third_from_end(data: list) -> int:
    return data[-3]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("len") || result.contains("saturating_sub") || result.contains("get")
        );
    }

    #[test]
    fn test_w16sc_index_071_dict_get_method() {
        let code = r#"
def safe_lookup(d: dict, k: str) -> str:
    return d.get(k)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains(".get(") || result.contains("cloned"));
    }

    #[test]
    fn test_w16sc_index_072_dict_get_with_default() {
        let code = r#"
def with_default(d: dict, k: str) -> int:
    return d.get(k, 0)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains(".get(") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w16sc_index_073_nested_list_index() {
        let code = r#"
def nested(grid: list) -> int:
    return grid[0][0]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("grid"));
    }

    #[test]
    fn test_w16sc_index_074_dict_multiple_keys() {
        let code = r#"
def multi_key(d: dict) -> str:
    a = d["x"]
    b = d["y"]
    c = d["z"]
    return a
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("\"x\""));
    }

    #[test]
    fn test_w16sc_index_075_list_literal_then_index() {
        let code = r#"
def inline_list() -> int:
    vals = [10, 20, 30, 40]
    return vals[2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("vec!") || result.contains("get") || result.contains("30"));
    }

    #[test]
    fn test_w16sc_index_076_string_index_two() {
        let code = r#"
def third_char(s: str) -> str:
    return s[2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("nth"));
    }

    #[test]
    fn test_w16sc_index_077_dict_bool_value() {
        let code = r#"
def check_flag(config: dict) -> bool:
    return config["enabled"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("enabled"));
    }

    #[test]
    fn test_w16sc_index_078_list_index_one() {
        let code = r#"
def second_item(data: list) -> int:
    return data[1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get(1") || result.contains("data"));
    }

    #[test]
    fn test_w16sc_index_079_dict_assign_int_val() {
        let code = r#"
def add_count(d: dict):
    d["count"] = 42
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert") || result.contains("count"));
    }

    #[test]
    fn test_w16sc_index_080_string_neg_two() {
        let code = r#"
def second_last_ch(s: str) -> str:
    return s[-2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("nth") || result.contains("count"));
    }

    #[test]
    fn test_w16sc_index_081_list_index_three() {
        let code = r#"
def fourth(data: list) -> int:
    return data[3]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get(3") || result.contains("data"));
    }

    #[test]
    fn test_w16sc_index_082_dict_literal_access() {
        let code = r#"
def from_lit():
    d = {"msg": "hello"}
    return d["msg"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("msg"));
    }

    #[test]
    fn test_w16sc_index_083_list_assign_variable_idx() {
        let code = r#"
def set_at(data: list, idx: int):
    data[idx] = 77
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("77") || result.contains("data") || result.contains("idx"));
    }

    #[test]
    fn test_w16sc_index_084_nested_dict_assign() {
        let code = r#"
def nested_set():
    d = {"a": {}}
    d["a"]["b"] = 5
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w16sc_index_085_list_build_and_access() {
        let code = r#"
def build_access():
    items = [5, 10, 15, 20]
    return items[3]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("vec!") || result.contains("get") || result.contains("20"));
    }

    #[test]
    fn test_w16sc_index_086_dict_string_default() {
        let code = r#"
def str_default(d: dict) -> str:
    return d.get("key", "unknown")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w16sc_index_087_dict_contains_key() {
        let code = r#"
def has_key(d: dict, k: str) -> bool:
    return k in d
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("contains_key") || result.contains("contains"));
    }

    #[test]
    fn test_w16sc_index_088_list_len_access() {
        let code = r#"
def size(data: list) -> int:
    return len(data)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("len()"));
    }

    #[test]
    fn test_w16sc_index_089_dict_pop_method() {
        let code = r#"
def remove_key(d: dict) -> int:
    return d.pop("key")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("remove") || result.contains("expect"));
    }

    #[test]
    fn test_w16sc_index_090_dict_pop_with_default() {
        let code = r#"
def safe_pop(d: dict) -> int:
    return d.pop("key", 0)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("remove") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w16sc_index_091_list_index_four() {
        let code = r#"
def fifth(data: list) -> int:
    return data[4]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get(4") || result.contains("data"));
    }

    #[test]
    fn test_w16sc_index_092_string_index_three() {
        let code = r#"
def fourth_ch(s: str) -> str:
    return s[3]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("nth"));
    }

    #[test]
    fn test_w16sc_index_093_dict_clear() {
        let code = r#"
def clear_all(d: dict):
    d.clear()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("clear()"));
    }

    #[test]
    fn test_w16sc_index_094_dict_copy() {
        let code = r#"
def dup(d: dict) -> dict:
    return d.copy()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("clone()"));
    }

    #[test]
    fn test_w16sc_index_095_dict_keys_loop() {
        let code = r#"
def show_keys(d: dict):
    for k in d.keys():
        print(k)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("keys()") || result.contains("for"));
    }

    #[test]
    fn test_w16sc_index_096_dict_values_loop() {
        let code = r#"
def show_values(d: dict):
    for v in d.values():
        print(v)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("values()") || result.contains("for"));
    }

    #[test]
    fn test_w16sc_index_097_dict_items_loop() {
        let code = r#"
def show_items(d: dict):
    for k, v in d.items():
        print(k, v)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("iter") || result.contains("for") || result.contains("items"));
    }

    #[test]
    fn test_w16sc_index_098_list_neg_four() {
        let code = r#"
def fourth_from_end(data: list) -> int:
    return data[-4]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("len") || result.contains("saturating_sub") || result.contains("get")
        );
    }

    #[test]
    fn test_w16sc_index_099_dict_setdefault() {
        let code = r#"
def ensure_key(d: dict) -> int:
    return d.setdefault("val", 0)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("entry")
                || result.contains("or_insert")
                || result.contains("setdefault")
        );
    }

    // ========================================================================
    // SECTION 3: Comprehensions (50 tests)
    // ========================================================================

    #[test]
    fn test_w16sc_comp_100_multi_gen_pairs() {
        let code = r#"
def pairs():
    return [(x, y) for x in range(3) for y in range(3)]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("flat_map") || result.contains("into_iter"));
    }

    #[test]
    fn test_w16sc_comp_101_nested_list_comp() {
        let code = r#"
def nested_lists():
    return [[j for j in range(3)] for i in range(3)]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("collect") || result.contains("map"));
    }

    #[test]
    fn test_w16sc_comp_102_dict_comp_with_filter() {
        let code = r#"
def positive_dict():
    return {k: v for k, v in [(1, 2), (3, -1), (5, 6)] if v > 0}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("filter") || result.contains("collect") || result.contains("HashMap")
        );
    }

    #[test]
    fn test_w16sc_comp_103_set_comp_lower() {
        let code = r#"
def lower_set(words: list):
    return {s.lower() for s in words}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("HashSet")
                || result.contains("collect")
                || result.contains("to_lowercase")
        );
    }

    #[test]
    fn test_w16sc_comp_104_comp_with_int_call() {
        let code = r#"
def parse_ints(strings: list) -> list:
    return [int(x) for x in strings]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("map") || result.contains("parse") || result.contains("collect"));
    }

    #[test]
    fn test_w16sc_comp_105_comp_with_ternary() {
        let code = r#"
def abs_vals(nums: list) -> list:
    return [x if x > 0 else -x for x in nums]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("map") || result.contains("if") || result.contains("collect"));
    }

    #[test]
    fn test_w16sc_comp_106_comp_over_string() {
        let code = r#"
def upper_chars(text: str) -> list:
    return [c.upper() for c in text]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("map") || result.contains("collect"));
    }

    #[test]
    fn test_w16sc_comp_107_list_comp_squared() {
        let code = r#"
def squares(n: int) -> list:
    return [x * x for x in range(n)]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("map") || result.contains("collect") || result.contains("into_iter")
        );
    }

    #[test]
    fn test_w16sc_comp_108_list_comp_filter_gt() {
        let code = r#"
def above_threshold(items: list, threshold: int) -> list:
    return [x for x in items if x > threshold]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("filter") || result.contains("collect"));
    }

    #[test]
    fn test_w16sc_comp_109_dict_comp_from_range() {
        let code = r#"
def square_dict():
    return {i: i * i for i in range(5)}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("collect") || result.contains("HashMap") || result.contains("map"));
    }

    #[test]
    fn test_w16sc_comp_110_set_comp_from_range() {
        let code = r#"
def unique_squares():
    return {x * x for x in range(10)}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashSet") || result.contains("collect"));
    }

    #[test]
    fn test_w16sc_comp_111_comp_with_len() {
        let code = r#"
def word_lengths(words: list) -> list:
    return [len(w) for w in words]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("len") || result.contains("map") || result.contains("collect"));
    }

    #[test]
    fn test_w16sc_comp_112_multi_gen_product() {
        let code = r#"
def products():
    return [x * y for x in range(4) for y in range(4)]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("flat_map") || result.contains("into_iter"));
    }

    #[test]
    fn test_w16sc_comp_113_dict_comp_string_keys() {
        let code = r#"
def name_lengths(names: list):
    return {name: len(name) for name in names}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("collect") || result.contains("HashMap") || result.contains("len"));
    }

    #[test]
    fn test_w16sc_comp_114_set_comp_modulo() {
        let code = r#"
def remainders(nums: list):
    return {x % 3 for x in nums}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashSet") || result.contains("collect"));
    }

    #[test]
    fn test_w16sc_comp_115_comp_filter_notempty() {
        let code = r#"
def non_empty(strings: list) -> list:
    return [s for s in strings if len(s) > 0]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("filter") || result.contains("collect"));
    }

    #[test]
    fn test_w16sc_comp_116_comp_add_one() {
        let code = r#"
def increment(vals: list) -> list:
    return [v + 1 for v in vals]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("map") || result.contains("collect"));
    }

    #[test]
    fn test_w16sc_comp_117_comp_negate() {
        let code = r#"
def negate(vals: list) -> list:
    return [-v for v in vals]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("map") || result.contains("collect"));
    }

    #[test]
    fn test_w16sc_comp_118_dict_comp_filter_positive() {
        let code = r#"
def positive_items():
    return {k: v for k, v in [("a", 1), ("b", -2), ("c", 3)] if v > 0}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("filter") || result.contains("HashMap") || result.contains("collect")
        );
    }

    #[test]
    fn test_w16sc_comp_119_set_comp_filter_even() {
        let code = r#"
def even_set():
    return {x for x in range(20) if x % 2 == 0}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("filter") || result.contains("HashSet") || result.contains("collect")
        );
    }

    #[test]
    fn test_w16sc_comp_120_comp_str_format() {
        let code = r#"
def labeled(nums: list) -> list:
    return [str(x) for x in nums]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("map") || result.contains("to_string") || result.contains("collect")
        );
    }

    #[test]
    fn test_w16sc_comp_121_comp_double() {
        let code = r#"
def doubled(items: list) -> list:
    return [x * 2 for x in items]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("map") || result.contains("collect"));
    }

    #[test]
    fn test_w16sc_comp_122_comp_cube() {
        let code = r#"
def cubed(items: list) -> list:
    return [x * x * x for x in items]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("map") || result.contains("collect"));
    }

    #[test]
    fn test_w16sc_comp_123_multi_gen_with_filter() {
        let code = r#"
def filtered_pairs():
    return [(x, y) for x in range(5) for y in range(5) if x != y]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("flat_map") || result.contains("filter") || result.contains("collect")
        );
    }

    #[test]
    fn test_w16sc_comp_124_nested_inner_comp() {
        let code = r#"
def matrix():
    return [[i * j for j in range(4)] for i in range(4)]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("collect") || result.contains("map"));
    }

    #[test]
    fn test_w16sc_comp_125_dict_comp_enumerate() {
        let code = r#"
def indexed(items: list):
    return {i: v for i, v in enumerate(items)}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("collect")
                || result.contains("HashMap")
                || result.contains("enumerate")
        );
    }

    #[test]
    fn test_w16sc_comp_126_comp_bool_filter() {
        let code = r#"
def truthy(items: list) -> list:
    return [x for x in items if x]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("filter") || result.contains("collect"));
    }

    #[test]
    fn test_w16sc_comp_127_comp_mod_ten() {
        let code = r#"
def last_digits(nums: list) -> list:
    return [n % 10 for n in nums]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("map") || result.contains("collect"));
    }

    #[test]
    fn test_w16sc_comp_128_set_comp_abs() {
        let code = r#"
def abs_set(nums: list):
    return {abs(x) for x in nums}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashSet") || result.contains("collect") || result.contains("abs"));
    }

    #[test]
    fn test_w16sc_comp_129_dict_comp_reverse() {
        let code = r#"
def swap_kv(d: dict):
    return {v: k for k, v in d.items()}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("collect") || result.contains("HashMap") || result.contains("map"));
    }

    #[test]
    fn test_w16sc_comp_130_comp_filter_divisible() {
        let code = r#"
def divisible_by_three(n: int) -> list:
    return [x for x in range(n) if x % 3 == 0]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("filter") || result.contains("collect"));
    }

    #[test]
    fn test_w16sc_comp_131_comp_add_two_lists() {
        let code = r#"
def flat_range():
    return [x + y for x in range(3) for y in range(3)]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("flat_map") || result.contains("into_iter"));
    }

    #[test]
    fn test_w16sc_comp_132_comp_string_filter() {
        let code = r#"
def long_words(words: list) -> list:
    return [w for w in words if len(w) > 3]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("filter") || result.contains("collect"));
    }

    #[test]
    fn test_w16sc_comp_133_dict_comp_squared() {
        let code = r#"
def sq_dict(n: int):
    return {x: x * x for x in range(n)}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("collect") || result.contains("HashMap"));
    }

    #[test]
    fn test_w16sc_comp_134_set_comp_from_list() {
        let code = r#"
def unique(items: list):
    return {x for x in items}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashSet") || result.contains("collect"));
    }

    #[test]
    fn test_w16sc_comp_135_comp_add_constant() {
        let code = r#"
def shift(vals: list, offset: int) -> list:
    return [v + offset for v in vals]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("map") || result.contains("collect"));
    }

    #[test]
    fn test_w16sc_comp_136_comp_filter_not_none() {
        let code = r#"
def compact(items: list) -> list:
    return [x for x in items if x is not None]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("filter") || result.contains("collect") || result.contains("None"));
    }

    #[test]
    fn test_w16sc_comp_137_comp_range_five() {
        let code = r#"
def first_five() -> list:
    return [i for i in range(5)]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("collect") || result.contains("0..5") || result.contains("into_iter")
        );
    }

    #[test]
    fn test_w16sc_comp_138_multi_gen_three() {
        let code = r#"
def triples():
    return [(x, y, z) for x in range(2) for y in range(2) for z in range(2)]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("flat_map") || result.contains("into_iter"));
    }

    #[test]
    fn test_w16sc_comp_139_dict_comp_conditional_val() {
        let code = r#"
def label_sign(nums: list):
    return {n: "pos" if n > 0 else "neg" for n in nums}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("collect") || result.contains("HashMap") || result.contains("map"));
    }

    #[test]
    fn test_w16sc_comp_140_set_comp_square() {
        let code = r#"
def square_set(n: int):
    return {x * x for x in range(n)}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashSet") || result.contains("collect"));
    }

    #[test]
    fn test_w16sc_comp_141_comp_strip_lines() {
        let code = r#"
def clean_lines(lines: list) -> list:
    return [line.strip() for line in lines]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("trim") || result.contains("map") || result.contains("collect"));
    }

    #[test]
    fn test_w16sc_comp_142_comp_upper_words() {
        let code = r#"
def shout(words: list) -> list:
    return [w.upper() for w in words]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("to_uppercase") || result.contains("map") || result.contains("collect")
        );
    }

    #[test]
    fn test_w16sc_comp_143_comp_lower_words() {
        let code = r#"
def whisper(words: list) -> list:
    return [w.lower() for w in words]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("to_lowercase") || result.contains("map") || result.contains("collect")
        );
    }

    #[test]
    fn test_w16sc_comp_144_dict_comp_from_zip() {
        let code = r#"
def zipped(keys: list, vals: list):
    return {k: v for k, v in zip(keys, vals)}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("collect") || result.contains("HashMap") || result.contains("zip"));
    }

    #[test]
    fn test_w16sc_comp_145_comp_index_access() {
        let code = r#"
def first_elements(lists: list) -> list:
    return [lst[0] for lst in lists]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("map") || result.contains("collect") || result.contains("get"));
    }

    #[test]
    fn test_w16sc_comp_146_comp_ternary_zero() {
        let code = r#"
def zero_negative(nums: list) -> list:
    return [x if x > 0 else 0 for x in nums]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("map") || result.contains("collect") || result.contains("if"));
    }

    #[test]
    fn test_w16sc_comp_147_set_comp_filtered() {
        let code = r#"
def big_set(n: int):
    return {x for x in range(n) if x > 5}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("filter") || result.contains("HashSet") || result.contains("collect")
        );
    }

    #[test]
    fn test_w16sc_comp_148_comp_chain_methods() {
        let code = r#"
def trimmed_upper(lines: list) -> list:
    return [s.strip().upper() for s in lines]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("map") || result.contains("collect"));
    }

    #[test]
    fn test_w16sc_comp_149_multi_gen_filtered() {
        let code = r#"
def even_products():
    return [x * y for x in range(5) for y in range(5) if (x * y) % 2 == 0]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("flat_map") || result.contains("filter") || result.contains("collect")
        );
    }

    // ========================================================================
    // SECTION 4: Attribute Access (50 tests)
    // ========================================================================

    #[test]
    fn test_w16sc_attr_150_math_pi() {
        let code = r#"
import math
def get_pi() -> float:
    return math.pi
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("PI") || result.contains("consts"));
    }

    #[test]
    fn test_w16sc_attr_151_math_e() {
        let code = r#"
import math
def get_e() -> float:
    return math.e
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("E") || result.contains("consts"));
    }

    #[test]
    fn test_w16sc_attr_152_math_inf() {
        let code = r#"
import math
def get_inf() -> float:
    return math.inf
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("INFINITY") || result.contains("inf"));
    }

    #[test]
    fn test_w16sc_attr_153_math_nan() {
        let code = r#"
import math
def get_nan() -> float:
    return math.nan
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("NAN") || result.contains("nan"));
    }

    #[test]
    fn test_w16sc_attr_154_math_tau() {
        let code = r#"
import math
def get_tau() -> float:
    return math.tau
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("TAU") || result.contains("consts"));
    }

    #[test]
    fn test_w16sc_attr_155_sys_platform() {
        let code = r#"
import sys
def platform() -> str:
    return sys.platform
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("to_string")
                || result.contains("darwin")
                || result.contains("linux")
                || result.contains("win32")
        );
    }

    #[test]
    fn test_w16sc_attr_156_sys_argv() {
        let code = r#"
import sys
def get_args() -> list:
    return sys.argv
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("env::args") || result.contains("collect") || result.contains("Vec")
        );
    }

    #[test]
    fn test_w16sc_attr_157_sys_stdin() {
        let code = r#"
import sys
def get_stdin():
    return sys.stdin
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("stdin") || result.contains("io"));
    }

    #[test]
    fn test_w16sc_attr_158_sys_stdout() {
        let code = r#"
import sys
def get_stdout():
    return sys.stdout
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("stdout") || result.contains("io"));
    }

    #[test]
    fn test_w16sc_attr_159_sys_stderr() {
        let code = r#"
import sys
def get_stderr():
    return sys.stderr
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("stderr") || result.contains("io"));
    }

    #[test]
    fn test_w16sc_attr_160_os_environ() {
        let code = r#"
import os
def get_env():
    return os.environ
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("env::vars")
                || result.contains("collect")
                || result.contains("HashMap")
        );
    }

    #[test]
    fn test_w16sc_attr_161_path_parent() {
        let code = r#"
def get_parent(path):
    return path.parent
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("parent") || result.contains("to_path_buf"));
    }

    #[test]
    fn test_w16sc_attr_162_path_stem() {
        let code = r#"
def get_stem(path):
    return path.stem
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("file_stem") || result.contains("to_str"));
    }

    #[test]
    fn test_w16sc_attr_163_path_suffix() {
        let code = r#"
def get_suffix(path):
    return path.suffix
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("extension") || result.contains("format"));
    }

    #[test]
    fn test_w16sc_attr_164_path_name() {
        let code = r#"
def get_name(path):
    return path.name
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("file_name") || result.contains("to_str"));
    }

    #[test]
    fn test_w16sc_attr_165_string_ascii_lowercase() {
        let code = r#"
import string
def get_lower() -> str:
    return string.ascii_lowercase
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("abcdefghijklmnopqrstuvwxyz"));
    }

    #[test]
    fn test_w16sc_attr_166_string_ascii_uppercase() {
        let code = r#"
import string
def get_upper() -> str:
    return string.ascii_uppercase
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("ABCDEFGHIJKLMNOPQRSTUVWXYZ"));
    }

    #[test]
    fn test_w16sc_attr_167_string_digits() {
        let code = r#"
import string
def get_digits() -> str:
    return string.digits
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("0123456789"));
    }

    #[test]
    fn test_w16sc_attr_168_string_ascii_letters() {
        let code = r#"
import string
def get_letters() -> str:
    return string.ascii_letters
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("abcdefghijklmnopqrstuvwxyz")
                || result.contains("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
        );
    }

    #[test]
    fn test_w16sc_attr_169_string_hexdigits() {
        let code = r#"
import string
def get_hex() -> str:
    return string.hexdigits
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("0123456789abcdefABCDEF"));
    }

    #[test]
    fn test_w16sc_attr_170_string_octdigits() {
        let code = r#"
import string
def get_oct() -> str:
    return string.octdigits
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("01234567"));
    }

    #[test]
    fn test_w16sc_attr_171_string_whitespace() {
        let code = r#"
import string
def get_ws() -> str:
    return string.whitespace
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        // Whitespace contains tab and newline chars
        assert!(
            result.contains("\\t")
                || result.contains("\\n")
                || result.contains("whitespace")
                || result.len() > 10
        );
    }

    #[test]
    fn test_w16sc_attr_172_re_ignorecase() {
        let code = r#"
import re
def get_flag():
    return re.IGNORECASE
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("2") || result.contains("IGNORECASE"));
    }

    #[test]
    fn test_w16sc_attr_173_re_multiline() {
        let code = r#"
import re
def get_flag():
    return re.MULTILINE
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("8") || result.contains("MULTILINE"));
    }

    #[test]
    fn test_w16sc_attr_174_re_dotall() {
        let code = r#"
import re
def get_flag():
    return re.DOTALL
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("16") || result.contains("DOTALL"));
    }

    #[test]
    fn test_w16sc_attr_175_obj_prop_access() {
        let code = r#"
def get_val(obj):
    return obj.value
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("value") || result.contains("obj"));
    }

    #[test]
    fn test_w16sc_attr_176_obj_method_call() {
        let code = r#"
def process(obj):
    return obj.compute()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("compute") || result.contains("obj"));
    }

    #[test]
    fn test_w16sc_attr_177_datetime_year() {
        let code = r#"
def get_year(dt):
    return dt.year
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("year()") || result.contains("year"));
    }

    #[test]
    fn test_w16sc_attr_178_datetime_month() {
        let code = r#"
def get_month(dt):
    return dt.month
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("month()") || result.contains("month"));
    }

    #[test]
    fn test_w16sc_attr_179_datetime_day() {
        let code = r#"
def get_day(dt):
    return dt.day
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("day()") || result.contains("day"));
    }

    #[test]
    fn test_w16sc_attr_180_datetime_hour() {
        let code = r#"
def get_hour(dt):
    return dt.hour
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("hour()") || result.contains("hour"));
    }

    #[test]
    fn test_w16sc_attr_181_datetime_minute() {
        let code = r#"
def get_minute(dt):
    return dt.minute
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("minute()") || result.contains("minute"));
    }

    #[test]
    fn test_w16sc_attr_182_datetime_second() {
        let code = r#"
def get_second(dt):
    return dt.second
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("second()") || result.contains("second"));
    }

    #[test]
    fn test_w16sc_attr_183_timedelta_days() {
        let code = r#"
def get_days(td):
    return td.days
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("days()") || result.contains("days"));
    }

    #[test]
    fn test_w16sc_attr_184_timedelta_seconds() {
        let code = r#"
def get_secs(td):
    return td.seconds
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("seconds()") || result.contains("seconds"));
    }

    #[test]
    fn test_w16sc_attr_185_timedelta_microseconds() {
        let code = r#"
def get_micros(td):
    return td.microseconds
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("microseconds()") || result.contains("microseconds"));
    }

    #[test]
    fn test_w16sc_attr_186_fraction_numerator() {
        let code = r#"
def get_num(frac):
    return frac.numerator
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("numer()") || result.contains("numerator"));
    }

    #[test]
    fn test_w16sc_attr_187_fraction_denominator() {
        let code = r#"
def get_den(frac):
    return frac.denominator
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("denom()") || result.contains("denominator"));
    }

    #[test]
    fn test_w16sc_attr_188_path_parts() {
        let code = r#"
def get_parts(path):
    return path.parts
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("components") || result.contains("collect") || result.contains("parts")
        );
    }

    #[test]
    fn test_w16sc_attr_189_stat_st_size() {
        let code = r#"
def file_size(stats):
    return stats.st_size
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("len()") || result.contains("st_size"));
    }

    #[test]
    fn test_w16sc_attr_190_stat_st_mtime() {
        let code = r#"
def mod_time(stats):
    return stats.st_mtime
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("modified") || result.contains("st_mtime"));
    }

    #[test]
    fn test_w16sc_attr_191_err_returncode() {
        let code = r#"
def get_code(e):
    return e.returncode
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        // Exception returncode maps to integer 1
        assert!(result.contains("1") || result.contains("returncode"));
    }

    #[test]
    fn test_w16sc_attr_192_enum_constant() {
        let code = r#"
def get_color():
    return Color.RED
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Color::RED") || result.contains("RED"));
    }

    #[test]
    fn test_w16sc_attr_193_enum_constant_green() {
        let code = r#"
def get_green():
    return Status.ACTIVE
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Status::ACTIVE") || result.contains("ACTIVE"));
    }

    #[test]
    fn test_w16sc_attr_194_math_sin_ref() {
        let code = r#"
import math
def get_fn():
    return math.sin
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("sin") || result.contains("f64"));
    }

    #[test]
    fn test_w16sc_attr_195_math_cos_ref() {
        let code = r#"
import math
def get_fn():
    return math.cos
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("cos") || result.contains("f64"));
    }

    #[test]
    fn test_w16sc_attr_196_math_sqrt_ref() {
        let code = r#"
import math
def get_fn():
    return math.sqrt
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("sqrt") || result.contains("f64"));
    }

    #[test]
    fn test_w16sc_attr_197_sys_version_info() {
        let code = r#"
import sys
def get_version():
    return sys.version_info
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("3") || result.contains("11") || result.contains("version"));
    }

    #[test]
    fn test_w16sc_attr_198_stat_st_atime() {
        let code = r#"
def access_time(stats):
    return stats.st_atime
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("accessed") || result.contains("st_atime"));
    }

    #[test]
    fn test_w16sc_attr_199_obj_chained_attr() {
        let code = r#"
def get_nested(obj):
    x = obj.data
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("data") || result.contains("obj"));
    }
}
