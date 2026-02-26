//! Wave 14 coverage tests: slicing, indexing, dict methods, and constructors
//!
//! Targets uncovered code paths in expr_gen_instance_methods:
//! - slicing.rs: negative step, step-only, string slicing with step/negative indices
//! - indexing.rs: tuple indexing, dict variable key, nested access, string indexing
//! - dict_methods.rs: setdefault, pop, update, keys, values, items, popitem, clear, copy
//! - constructors.rs: set literals, frozenset, mixed lists, None lists, float coercion
//!
//! 150 tests total across 4 categories

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
    // SECTION 1: Slicing (40 tests)
    // ========================================================================

    #[test]
    fn test_w14si_slice_001_list_reverse_full() {
        let code = r#"
def reverse_list(data: list) -> list:
    return data[::-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rev") || result.contains("step"));
    }

    #[test]
    fn test_w14si_slice_002_list_step_by_two() {
        let code = r#"
def every_other(data: list) -> list:
    return data[::2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("step_by") || result.contains("step"));
    }

    #[test]
    fn test_w14si_slice_003_list_start_step_two() {
        let code = r#"
def from_one_step_two(data: list) -> list:
    return data[1::2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("step") || result.contains("iter"));
    }

    #[test]
    fn test_w14si_slice_004_list_stop_step_two() {
        let code = r#"
def to_three_step_two(data: list) -> list:
    return data[:3:2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("step") || result.contains("min"));
    }

    #[test]
    fn test_w14si_slice_005_list_negative_start() {
        let code = r#"
def last_three(data: list) -> list:
    return data[-3:]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("isize") || result.contains("start") || result.contains("len"));
    }

    #[test]
    fn test_w14si_slice_006_list_negative_stop() {
        let code = r#"
def except_last_two(data: list) -> list:
    return data[:-2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("isize") || result.contains("stop") || result.contains("len"));
    }

    #[test]
    fn test_w14si_slice_007_list_negative_both() {
        let code = r#"
def middle_neg(data: list) -> list:
    return data[1:-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("isize") || result.contains("to_vec"));
    }

    #[test]
    fn test_w14si_slice_008_string_step_two() {
        let code = r#"
def every_other_char(s: str) -> str:
    return s[::2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("step_by"));
    }

    #[test]
    fn test_w14si_slice_009_string_reverse() {
        let code = r#"
def reverse_string(s: str) -> str:
    return s[::-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rev") || result.contains("chars"));
    }

    #[test]
    fn test_w14si_slice_010_string_neg_start() {
        let code = r#"
def last_three_chars(s: str) -> str:
    return s[-3:]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("skip"));
    }

    #[test]
    fn test_w14si_slice_011_string_neg_to_neg() {
        let code = r#"
def middle_chars(s: str) -> str:
    return s[1:-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("String"));
    }

    #[test]
    fn test_w14si_slice_012_list_start_neg_step() {
        let code = r#"
def from_two_reverse(data: list) -> list:
    return data[2::-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rev") || result.contains("step") || result.contains("iter"));
    }

    #[test]
    fn test_w14si_slice_013_list_stop_neg_step() {
        let code = r#"
def until_two_reverse(data: list) -> list:
    return data[:2:-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rev") || result.contains("step") || result.contains("iter"));
    }

    #[test]
    fn test_w14si_slice_014_list_full_range_neg_step() {
        let code = r#"
def full_reverse_step(data: list) -> list:
    return data[4:1:-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rev") || result.contains("step") || result.contains("collect"));
    }

    #[test]
    fn test_w14si_slice_015_list_positive_range_step() {
        let code = r#"
def range_with_step(data: list) -> list:
    return data[0:5:2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("step_by") || result.contains("step"));
    }

    #[test]
    fn test_w14si_slice_016_string_start_stop_step() {
        let code = r#"
def substr_step(s: str) -> str:
    return s[0:5:2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("step"));
    }

    #[test]
    fn test_w14si_slice_017_string_neg_step_two() {
        let code = r#"
def reverse_step_two(s: str) -> str:
    return s[::-2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rev") || result.contains("step") || result.contains("chars"));
    }

    #[test]
    fn test_w14si_slice_018_list_neg_start_step() {
        let code = r#"
def neg_start_step(data: list) -> list:
    return data[-4::2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("step") || result.contains("isize") || result.contains("iter"));
    }

    #[test]
    fn test_w14si_slice_019_list_full_clone() {
        let code = r#"
def clone_list(data: list) -> list:
    return data[:]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("clone"));
    }

    #[test]
    fn test_w14si_slice_020_string_full_clone() {
        let code = r#"
def clone_str(s: str) -> str:
    return s[:]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("to_string"));
    }

    #[test]
    fn test_w14si_slice_021_list_step_three() {
        let code = r#"
def every_third(data: list) -> list:
    return data[::3]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("step") || result.contains("iter"));
    }

    #[test]
    fn test_w14si_slice_022_list_start_stop_positive() {
        let code = r#"
def sublist(data: list) -> list:
    return data[2:7]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("to_vec") || result.contains("start") || result.contains("stop"));
    }

    #[test]
    fn test_w14si_slice_023_string_start_only() {
        let code = r#"
def from_second(s: str) -> str:
    return s[2:]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("skip"));
    }

    #[test]
    fn test_w14si_slice_024_string_stop_only() {
        let code = r#"
def first_four(s: str) -> str:
    return s[:4]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("take"));
    }

    #[test]
    fn test_w14si_slice_025_list_neg_step_neg_start() {
        let code = r#"
def neg_both(data: list) -> list:
    return data[-1::-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rev") || result.contains("step") || result.contains("iter"));
    }

    #[test]
    fn test_w14si_slice_026_string_start_step() {
        let code = r#"
def from_one_step(s: str) -> str:
    return s[1::2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("step"));
    }

    #[test]
    fn test_w14si_slice_027_string_stop_step() {
        let code = r#"
def to_four_step(s: str) -> str:
    return s[:4:2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("step") || result.contains("take"));
    }

    #[test]
    fn test_w14si_slice_028_list_neg_stop_only() {
        let code = r#"
def trim_end(data: list) -> list:
    return data[:-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("isize") || result.contains("to_vec") || result.contains("len"));
    }

    #[test]
    fn test_w14si_slice_029_string_neg_stop_only() {
        let code = r#"
def trim_last(s: str) -> str:
    return s[:-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("take"));
    }

    #[test]
    fn test_w14si_slice_030_list_start_to_neg_stop() {
        let code = r#"
def mid_section(data: list) -> list:
    return data[2:-2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("isize") || result.contains("to_vec"));
    }

    #[test]
    fn test_w14si_slice_031_string_neg_start_neg_stop() {
        let code = r#"
def neg_range_str(s: str) -> str:
    return s[-4:-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("String"));
    }

    #[test]
    fn test_w14si_slice_032_list_step_one() {
        let code = r#"
def step_one(data: list) -> list:
    return data[::1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("step") || result.contains("clone") || result.contains("base"));
    }

    #[test]
    fn test_w14si_slice_033_string_step_one() {
        let code = r#"
def str_step_one(s: str) -> str:
    return s[::1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("to_string") || result.contains("chars") || result.contains("step")
        );
    }

    #[test]
    fn test_w14si_slice_034_list_start_stop_step_one() {
        let code = r#"
def sublist_step_one(data: list) -> list:
    return data[1:4:1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("to_vec") || result.contains("step") || result.contains("base"));
    }

    #[test]
    fn test_w14si_slice_035_string_start_stop_step_one() {
        let code = r#"
def substr_step_one(s: str) -> str:
    return s[1:4:1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("String") || result.contains("step"));
    }

    #[test]
    fn test_w14si_slice_036_list_neg_start_pos_stop() {
        let code = r#"
def neg_to_pos(data: list) -> list:
    return data[-5:3]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("isize") || result.contains("to_vec") || result.contains("start"));
    }

    #[test]
    fn test_w14si_slice_037_string_neg_start_pos_stop() {
        let code = r#"
def str_neg_to_pos(s: str) -> str:
    return s[-5:3]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("String"));
    }

    #[test]
    fn test_w14si_slice_038_list_start_neg_step_two() {
        let code = r#"
def rev_step_two(data: list) -> list:
    return data[4:0:-2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rev") || result.contains("step") || result.contains("collect"));
    }

    #[test]
    fn test_w14si_slice_039_string_start_neg_step() {
        let code = r#"
def str_rev_from(s: str) -> str:
    return s[3:0:-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("rev") || result.contains("step"));
    }

    #[test]
    fn test_w14si_slice_040_list_neg_start_neg_stop_neg_step() {
        let code = r#"
def full_neg(data: list) -> list:
    return data[-1:-4:-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("rev")
                || result.contains("step")
                || result.contains("isize")
                || result.contains("collect")
        );
    }

    // ========================================================================
    // SECTION 2: Indexing (30 tests)
    // ========================================================================

    #[test]
    fn test_w14si_index_001_tuple_first() {
        let code = r#"
def first(t: tuple) -> int:
    return t[0]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains(".0") || result.contains("get(0") || result.contains("t[0"));
    }

    #[test]
    fn test_w14si_index_002_tuple_second() {
        let code = r#"
def second(t: tuple) -> int:
    return t[1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains(".1") || result.contains("get(1") || result.contains("t[1"));
    }

    #[test]
    fn test_w14si_index_003_dict_string_key() {
        let code = r#"
def get_name(d: dict) -> str:
    return d["name"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("name"));
    }

    #[test]
    fn test_w14si_index_004_dict_variable_key() {
        let code = r#"
def get_val(d: dict, k: str):
    return d[k]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("k"));
    }

    #[test]
    fn test_w14si_index_005_nested_dict() {
        let code = r#"
def nested(d: dict) -> int:
    return d["a"]["b"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("\"a\"") || result.contains("\"b\""));
    }

    #[test]
    fn test_w14si_index_006_list_neg_one() {
        let code = r#"
def last_item(data: list) -> int:
    return data[-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("len") || result.contains("saturating_sub") || result.contains("get")
        );
    }

    #[test]
    fn test_w14si_index_007_string_first_char() {
        let code = r#"
def first_char(s: str) -> str:
    return s[0]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("nth"));
    }

    #[test]
    fn test_w14si_index_008_string_neg_one() {
        let code = r#"
def last_char(s: str) -> str:
    return s[-1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("chars") || result.contains("nth") || result.contains("saturating_sub")
        );
    }

    #[test]
    fn test_w14si_index_009_subscript_assign_dict() {
        let code = r#"
def set_key():
    d = {}
    d["key"] = 42
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert") || result.contains("key"));
    }

    #[test]
    fn test_w14si_index_010_nested_list_index() {
        let code = r#"
def get_cell(matrix: list, i: int, j: int) -> int:
    return matrix[i][j]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("matrix"));
    }

    #[test]
    fn test_w14si_index_011_list_positive_literal() {
        let code = r#"
def third(data: list) -> int:
    return data[2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get(2") || result.contains("data"));
    }

    #[test]
    fn test_w14si_index_012_list_variable_index() {
        let code = r#"
def by_index(data: list, idx: int) -> int:
    return data[idx]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("idx") || result.contains("as usize"));
    }

    #[test]
    fn test_w14si_index_013_dict_get_method() {
        let code = r#"
def safe_get(d: dict, key: str) -> str:
    return d.get(key)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains(".get(") || result.contains("cloned"));
    }

    #[test]
    fn test_w14si_index_014_dict_get_with_default() {
        let code = r#"
def safe_get_default(d: dict, key: str) -> int:
    return d.get(key, 0)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains(".get(") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w14si_index_015_list_neg_two() {
        let code = r#"
def second_to_last(data: list) -> int:
    return data[-2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("len") || result.contains("saturating_sub") || result.contains("get")
        );
    }

    #[test]
    fn test_w14si_index_016_string_index_two() {
        let code = r#"
def third_char(s: str) -> str:
    return s[2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("nth"));
    }

    #[test]
    fn test_w14si_index_017_dict_assign_new_key() {
        let code = r#"
def add_entry(d: dict):
    d["new_key"] = "value"
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert") || result.contains("new_key"));
    }

    #[test]
    fn test_w14si_index_018_list_computed_index() {
        let code = r#"
def computed(data: list, n: int) -> int:
    return data[n - 1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("idx") || result.contains("actual_idx"));
    }

    #[test]
    fn test_w14si_index_019_dict_int_key() {
        let code = r#"
def int_key():
    d = {1: "a", 2: "b"}
    return d[1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("insert"));
    }

    #[test]
    fn test_w14si_index_020_string_neg_two() {
        let code = r#"
def second_last_char(s: str) -> str:
    return s[-2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("nth") || result.contains("count"));
    }

    #[test]
    fn test_w14si_index_021_list_zero() {
        let code = r#"
def first_elem(data: list) -> int:
    return data[0]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get(0") || result.contains("data"));
    }

    #[test]
    fn test_w14si_index_022_nested_dict_assign() {
        let code = r#"
def nested_set(d: dict):
    d["a"] = {}
    d["a"]["b"] = 1
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w14si_index_023_list_assign_index() {
        let code = r#"
def set_elem(data: list, i: int):
    data[i] = 42
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("42") || result.contains("data"));
    }

    #[test]
    fn test_w14si_index_024_dict_multiple_access() {
        let code = r#"
def multi_access(d: dict) -> str:
    a = d["x"]
    b = d["y"]
    return a
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("\"x\"") || result.contains("\"y\""));
    }

    #[test]
    fn test_w14si_index_025_string_index_variable() {
        let code = r#"
def char_at(s: str, i: int) -> str:
    return s[i]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("chars") || result.contains("nth") || result.contains("idx"));
    }

    #[test]
    fn test_w14si_index_026_dict_literal_access() {
        let code = r#"
def from_literal():
    d = {"key": "value"}
    return d["key"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("key"));
    }

    #[test]
    fn test_w14si_index_027_list_last_positive() {
        let code = r#"
def element_four(data: list) -> int:
    return data[4]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get(4") || result.contains("data"));
    }

    #[test]
    fn test_w14si_index_028_tuple_third() {
        let code = r#"
def third_elem(t: tuple):
    return t[2]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains(".2") || result.contains("get(2") || result.contains("t"));
    }

    #[test]
    fn test_w14si_index_029_list_append_then_index() {
        let code = r#"
def build_and_get():
    data = [10, 20, 30]
    return data[1]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("vec!") || result.contains("get") || result.contains("10"));
    }

    #[test]
    fn test_w14si_index_030_dict_bool_val_access() {
        let code = r#"
def flag_check(config: dict) -> bool:
    return config["enabled"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("enabled"));
    }

    // ========================================================================
    // SECTION 3: Dict Methods (40 tests)
    // ========================================================================

    #[test]
    fn test_w14si_dict_001_setdefault_with_int() {
        let code = r#"
def set_default_int(d: dict):
    x = d.setdefault("count", 0)
    return x
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
    fn test_w14si_dict_002_setdefault_with_list() {
        let code = r#"
def set_default_list(d: dict):
    x = d.setdefault("items", [])
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("entry") || result.contains("or_insert") || result.contains("vec!")
        );
    }

    #[test]
    fn test_w14si_dict_003_pop_with_key() {
        let code = r#"
def pop_key(d: dict) -> int:
    return d.pop("a")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("remove") || result.contains("expect"));
    }

    #[test]
    fn test_w14si_dict_004_pop_with_default() {
        let code = r#"
def pop_default(d: dict) -> int:
    return d.pop("b", 0)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("remove") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w14si_dict_005_update_dict() {
        let code = r#"
def update_dict(d: dict):
    d.update({"x": 1})
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert") || result.contains("iter") || result.contains("update"));
    }

    #[test]
    fn test_w14si_dict_006_keys_basic() {
        let code = r#"
def get_keys(d: dict):
    return d.keys()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("keys()") || result.contains("collect"));
    }

    #[test]
    fn test_w14si_dict_007_values_basic() {
        let code = r#"
def get_values(d: dict):
    return d.values()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("values()") || result.contains("collect"));
    }

    #[test]
    fn test_w14si_dict_008_items_basic() {
        let code = r#"
def get_items(d: dict):
    return d.items()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("iter()") || result.contains("collect") || result.contains("items")
        );
    }

    #[test]
    fn test_w14si_dict_009_dict_comprehension() {
        let code = r#"
def square_map():
    return {i: i * 2 for i in range(5)}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("collect")
                || result.contains("HashMap")
                || result.contains("map")
                || result.contains("insert")
        );
    }

    #[test]
    fn test_w14si_dict_010_get_string_default() {
        let code = r#"
def get_with_str_default(d: dict) -> str:
    return d.get("key", "unknown")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w14si_dict_011_clear() {
        let code = r#"
def clear_dict(d: dict):
    d.clear()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("clear()"));
    }

    #[test]
    fn test_w14si_dict_012_copy() {
        let code = r#"
def copy_dict(d: dict) -> dict:
    return d.copy()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("clone()"));
    }

    #[test]
    fn test_w14si_dict_013_popitem() {
        let code = r#"
def pop_item(d: dict):
    return d.popitem()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("keys") || result.contains("remove") || result.contains("popitem"));
    }

    #[test]
    fn test_w14si_dict_014_get_literal_key() {
        let code = r#"
def lookup(d: dict) -> int:
    return d.get("name", 0)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("name"));
    }

    #[test]
    fn test_w14si_dict_015_update_then_get() {
        let code = r#"
def update_and_read(d: dict):
    d.update({"a": 1})
    return d.get("a")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert") || result.contains("get"));
    }

    #[test]
    fn test_w14si_dict_016_keys_in_loop() {
        let code = r#"
def print_keys(d: dict):
    for k in d.keys():
        print(k)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("keys()") || result.contains("for") || result.contains("println"));
    }

    #[test]
    fn test_w14si_dict_017_values_in_loop() {
        let code = r#"
def print_values(d: dict):
    for v in d.values():
        print(v)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("values()") || result.contains("for") || result.contains("println")
        );
    }

    #[test]
    fn test_w14si_dict_018_items_in_loop() {
        let code = r#"
def print_items(d: dict):
    for k, v in d.items():
        print(k, v)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("iter") || result.contains("for") || result.contains("items"));
    }

    #[test]
    fn test_w14si_dict_019_dict_comp_with_filter() {
        let code = r#"
def even_squares():
    return {i: i * i for i in range(10) if i % 2 == 0}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("filter")
                || result.contains("collect")
                || result.contains("HashMap")
                || result.contains("map")
        );
    }

    #[test]
    fn test_w14si_dict_020_dict_literal_multi() {
        let code = r#"
def multi_dict():
    return {"a": 1, "b": 2, "c": 3}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("HashMap") || result.contains("insert") || result.contains("\"a\"")
        );
    }

    #[test]
    fn test_w14si_dict_021_get_one_arg() {
        let code = r#"
def maybe_get(d: dict, key: str):
    return d.get(key)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("cloned"));
    }

    #[test]
    fn test_w14si_dict_022_pop_only_key() {
        let code = r#"
def remove_key(d: dict):
    val = d.pop("remove_me")
    return val
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("remove") || result.contains("expect"));
    }

    #[test]
    fn test_w14si_dict_023_setdefault_string() {
        let code = r#"
def default_name(d: dict):
    return d.setdefault("name", "anon")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("entry") || result.contains("or_insert") || result.contains("name")
        );
    }

    #[test]
    fn test_w14si_dict_024_update_multiple() {
        let code = r#"
def batch_update(d: dict):
    d.update({"x": 10, "y": 20, "z": 30})
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert") || result.contains("iter") || result.contains("clone"));
    }

    #[test]
    fn test_w14si_dict_025_dict_comp_string_keys() {
        let code = r#"
def word_lengths(words: list):
    return {w: len(w) for w in words}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("collect") || result.contains("map") || result.contains("len"));
    }

    #[test]
    fn test_w14si_dict_026_keys_to_list() {
        let code = r#"
def keys_list(d: dict) -> list:
    return list(d.keys())
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("keys") || result.contains("collect") || result.contains("Vec"));
    }

    #[test]
    fn test_w14si_dict_027_values_to_list() {
        let code = r#"
def values_list(d: dict) -> list:
    return list(d.values())
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("values") || result.contains("collect") || result.contains("Vec"));
    }

    #[test]
    fn test_w14si_dict_028_get_nested_key() {
        let code = r#"
def deep_get(config: dict) -> str:
    section = config.get("database")
    return section
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("database"));
    }

    #[test]
    fn test_w14si_dict_029_pop_with_str_default() {
        let code = r#"
def pop_str(d: dict) -> str:
    return d.pop("key", "default")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("remove") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w14si_dict_030_clear_then_update() {
        let code = r#"
def reset_dict(d: dict):
    d.clear()
    d.update({"fresh": True})
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("clear") || result.contains("insert"));
    }

    #[test]
    fn test_w14si_dict_031_copy_and_modify() {
        let code = r#"
def copy_modify(d: dict) -> dict:
    new_d = d.copy()
    return new_d
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("clone") || result.contains("new_d"));
    }

    #[test]
    fn test_w14si_dict_032_dict_empty_literal() {
        let code = r#"
def empty_dict() -> dict:
    return {}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashMap") || result.contains("new()"));
    }

    #[test]
    fn test_w14si_dict_033_dict_get_int_default() {
        let code = r#"
def count_get(counts: dict, key: str) -> int:
    return counts.get(key, 0)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w14si_dict_034_items_unpack() {
        let code = r#"
def process_items(d: dict):
    result = []
    for k, v in d.items():
        result.append(k)
    return result
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("iter") || result.contains("push") || result.contains("for"));
    }

    #[test]
    fn test_w14si_dict_035_dict_in_check() {
        let code = r#"
def has_key(d: dict, key: str) -> bool:
    return key in d
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("contains_key") || result.contains("contains"));
    }

    #[test]
    fn test_w14si_dict_036_dict_len() {
        let code = r#"
def dict_size(d: dict) -> int:
    return len(d)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("len()"));
    }

    #[test]
    fn test_w14si_dict_037_dict_comp_conditional() {
        let code = r#"
def positive_only(d: dict):
    return {k: v for k, v in d.items() if v > 0}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("filter") || result.contains("collect") || result.contains("iter"));
    }

    #[test]
    fn test_w14si_dict_038_update_empty() {
        let code = r#"
def fill_empty():
    d = {}
    d.update({"a": 1, "b": 2})
    return d
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert") || result.contains("HashMap"));
    }

    #[test]
    fn test_w14si_dict_039_get_with_none_check() {
        let code = r#"
def check_exists(d: dict, key: str) -> bool:
    val = d.get(key)
    return val is not None
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("get")
                || result.contains("is_some")
                || result.contains("is_none")
                || result.contains("None")
        );
    }

    #[test]
    fn test_w14si_dict_040_dict_with_bool_values() {
        let code = r#"
def feature_flags():
    return {"debug": True, "verbose": False}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("true") || result.contains("false") || result.contains("insert"));
    }

    // ========================================================================
    // SECTION 4: Constructors (40 tests)
    // ========================================================================

    #[test]
    fn test_w14si_ctor_001_list_with_none() {
        let code = r#"
def with_none():
    return [1, None, 3]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Some") || result.contains("None") || result.contains("vec!"));
    }

    #[test]
    fn test_w14si_ctor_002_list_all_none() {
        let code = r#"
def all_none():
    return [None, None]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("None") || result.contains("vec!"));
    }

    #[test]
    fn test_w14si_ctor_003_set_literal_ints() {
        let code = r#"
def make_set():
    return {1, 2, 3}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashSet") || result.contains("insert"));
    }

    #[test]
    fn test_w14si_ctor_004_set_literal_strings() {
        let code = r#"
def make_str_set():
    return {"a", "b", "c"}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("HashSet") || result.contains("insert") || result.contains("to_string")
        );
    }

    #[test]
    fn test_w14si_ctor_005_frozenset_ints() {
        let code = r#"
def make_frozen():
    return frozenset([1, 2, 3])
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashSet") || result.contains("Arc") || result.contains("insert"));
    }

    #[test]
    fn test_w14si_ctor_006_tuple_hetero() {
        let code = r#"
def mixed_tuple():
    return (1, "hello", 3.14)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("to_string") || result.contains("hello") || result.contains("3.14")
        );
    }

    #[test]
    fn test_w14si_ctor_007_empty_list() {
        let code = r#"
def empty():
    x = []
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("vec!") || result.contains("Vec::new"));
    }

    #[test]
    fn test_w14si_ctor_008_empty_dict() {
        let code = r#"
def empty_d():
    x = {}
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashMap") || result.contains("new"));
    }

    #[test]
    fn test_w14si_ctor_009_list_of_strings() {
        let code = r#"
def string_list():
    return ["hello", "world"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("to_string") || result.contains("vec!"));
    }

    #[test]
    fn test_w14si_ctor_010_list_of_floats() {
        let code = r#"
def float_list():
    return [1.5, 2.5, 3.5]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("vec!") || result.contains("1.5"));
    }

    #[test]
    fn test_w14si_ctor_011_list_of_bools() {
        let code = r#"
def bool_list():
    return [True, False, True]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("vec!") || result.contains("true") || result.contains("false"));
    }

    #[test]
    fn test_w14si_ctor_012_nested_list() {
        let code = r#"
def nested():
    return [[1, 2], [3, 4]]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("vec!"));
    }

    #[test]
    fn test_w14si_ctor_013_tuple_two_ints() {
        let code = r#"
def pair():
    return (1, 2)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("1") && result.contains("2"));
    }

    #[test]
    fn test_w14si_ctor_014_tuple_single() {
        let code = r#"
def single():
    return (42,)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("42"));
    }

    #[test]
    fn test_w14si_ctor_015_set_single_element() {
        let code = r#"
def single_set():
    return {42}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashSet") || result.contains("insert") || result.contains("42"));
    }

    #[test]
    fn test_w14si_ctor_016_dict_single_entry() {
        let code = r#"
def single_dict():
    return {"key": "value"}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert") || result.contains("HashMap") || result.contains("key"));
    }

    #[test]
    fn test_w14si_ctor_017_list_with_string_and_none() {
        let code = r#"
def str_none():
    return ["hello", None, "world"]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Some") || result.contains("None") || result.contains("vec!"));
    }

    #[test]
    fn test_w14si_ctor_018_list_ints_large() {
        let code = r#"
def large_list():
    return [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("vec!") || result.contains("10"));
    }

    #[test]
    fn test_w14si_ctor_019_tuple_three_strings() {
        let code = r#"
def triple_str():
    return ("a", "b", "c")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("to_string") || result.contains("\"a\""));
    }

    #[test]
    fn test_w14si_ctor_020_set_from_range() {
        let code = r#"
def range_set():
    s = set()
    for i in range(5):
        s.add(i)
    return s
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashSet") || result.contains("insert"));
    }

    #[test]
    fn test_w14si_ctor_021_dict_int_values() {
        let code = r#"
def int_vals():
    return {"x": 10, "y": 20}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert") || result.contains("HashMap") || result.contains("10"));
    }

    #[test]
    fn test_w14si_ctor_022_list_single_int() {
        let code = r#"
def single_list():
    return [42]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("vec!") || result.contains("42"));
    }

    #[test]
    fn test_w14si_ctor_023_frozenset_strings() {
        let code = r#"
def frozen_strs():
    return frozenset(["a", "b"])
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashSet") || result.contains("Arc") || result.contains("insert"));
    }

    #[test]
    fn test_w14si_ctor_024_tuple_with_bool() {
        let code = r#"
def bool_tuple():
    return (True, False)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("true") || result.contains("false"));
    }

    #[test]
    fn test_w14si_ctor_025_list_mixed_int_float() {
        let code = r#"
def mixed_nums():
    return [1, 2.5, 3]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("vec!")
                || result.contains("json")
                || result.contains("format")
                || result.contains("2.5")
        );
    }

    #[test]
    fn test_w14si_ctor_026_dict_bool_values() {
        let code = r#"
def flag_dict():
    return {"enabled": True, "debug": False}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("true") || result.contains("false") || result.contains("insert"));
    }

    #[test]
    fn test_w14si_ctor_027_empty_tuple() {
        let code = r#"
def empty_tup():
    return ()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14si_ctor_028_list_comprehension_range() {
        let code = r#"
def comp_list():
    return [i for i in range(5)]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("collect") || result.contains("0..5") || result.contains("map"));
    }

    #[test]
    fn test_w14si_ctor_029_set_comprehension() {
        let code = r#"
def comp_set():
    return {i for i in range(5)}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("collect") || result.contains("HashSet") || result.contains("insert")
        );
    }

    #[test]
    fn test_w14si_ctor_030_dict_from_zip_like() {
        let code = r#"
def pair_dict():
    keys = ["a", "b"]
    vals = [1, 2]
    return {keys[0]: vals[0], keys[1]: vals[1]}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert") || result.contains("HashMap") || result.contains("keys"));
    }

    #[test]
    fn test_w14si_ctor_031_list_mixed_int_str() {
        let code = r#"
def mixed_types():
    return [1, "hello", 3]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("vec!") || result.contains("json") || result.contains("format"));
    }

    #[test]
    fn test_w14si_ctor_032_list_mixed_int_bool() {
        let code = r#"
def int_bool_mix():
    return [1, True, 0, False]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("vec!") || result.contains("json") || result.contains("format"));
    }

    #[test]
    fn test_w14si_ctor_033_nested_dict() {
        let code = r#"
def nested_dict():
    return {"outer": {"inner": 42}}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert") || result.contains("HashMap") || result.contains("42"));
    }

    #[test]
    fn test_w14si_ctor_034_set_with_duplicates() {
        let code = r#"
def dedup():
    return {1, 2, 2, 3, 3, 3}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashSet") || result.contains("insert"));
    }

    #[test]
    fn test_w14si_ctor_035_tuple_five_ints() {
        let code = r#"
def five_tuple():
    return (1, 2, 3, 4, 5)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("1") && result.contains("5"));
    }

    #[test]
    fn test_w14si_ctor_036_list_string_none_mix() {
        let code = r#"
def opt_strings():
    return [None, "hello", None]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("None") || result.contains("Some") || result.contains("vec!"));
    }

    #[test]
    fn test_w14si_ctor_037_dict_many_entries() {
        let code = r#"
def big_dict():
    return {"a": 1, "b": 2, "c": 3, "d": 4, "e": 5}
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert") || result.contains("HashMap"));
    }

    #[test]
    fn test_w14si_ctor_038_frozenset_empty() {
        let code = r#"
def empty_frozen():
    return frozenset([])
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("HashSet") || result.contains("Arc"));
    }

    #[test]
    fn test_w14si_ctor_039_list_of_tuples() {
        let code = r#"
def tuple_list():
    return [(1, 2), (3, 4)]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("vec!") || result.contains("(1") || result.contains("(3"));
    }

    #[test]
    fn test_w14si_ctor_040_list_of_dicts() {
        let code = r#"
def dict_list():
    return [{"a": 1}, {"b": 2}]
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("vec!") || result.contains("insert") || result.contains("HashMap"));
    }
}
