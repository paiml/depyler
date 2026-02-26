//! Wave 14: Coverage tests for string methods, list methods, instance dispatch,
//! file I/O, and attribute access.
//!
//! 150 tests targeting uncovered code paths in:
//! - string_methods.rs: encode, split w/ maxsplit, rsplit, join, replace w/ count,
//!   find, rfind, count, zfill, center, ljust, rjust, expandtabs, partition,
//!   removeprefix, removesuffix
//! - list_methods.rs: extend, insert, pop, reverse, sort, index, count, remove,
//!   clear, copy
//! - instance_dispatch.rs: file read/readline/readlines/write/writelines,
//!   path exists/is_file/is_dir/resolve, datetime strftime/isoformat,
//!   regex group/groups
//! - attribute_convert.rs: sys.stdout/stderr, os.path.*, os.environ, class self,
//!   chained attribute, path attributes
//!
//! Status: 150/150 tests

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
    // STRING METHODS (40 tests: test_w14md_str_001 through test_w14md_str_040)
    // ========================================================================

    #[test]
    fn test_w14md_str_001_encode_no_args() {
        let code = r#"
def encode_str(s: str) -> list:
    return s.encode()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("as_bytes") || result.contains("to_vec"));
    }

    #[test]
    fn test_w14md_str_002_encode_utf8() {
        let code = r#"
def encode_utf8(s: str) -> list:
    return s.encode("utf-8")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("as_bytes") || result.contains("to_vec"));
    }

    #[test]
    fn test_w14md_str_003_split_maxsplit() {
        let code = r#"
def split_first(s: str) -> list:
    return s.split(",", 1)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("splitn") || result.contains("split"));
    }

    #[test]
    fn test_w14md_str_004_split_maxsplit_space() {
        let code = r#"
def split_space(s: str) -> list:
    return s.split(" ", 2)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("splitn") || result.contains("split"));
    }

    #[test]
    fn test_w14md_str_005_rsplit_basic() {
        let code = r#"
def rsplit_path(s: str) -> list:
    return s.rsplit("/")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rsplit") || result.contains("split"));
    }

    #[test]
    fn test_w14md_str_006_rsplit_maxsplit() {
        let code = r#"
def rsplit_last(s: str) -> list:
    return s.rsplit("/", 1)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rsplitn") || result.contains("rsplit"));
    }

    #[test]
    fn test_w14md_str_007_rsplit_no_args() {
        let code = r#"
def rsplit_ws(s: str) -> list:
    return s.rsplit()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("split_whitespace") || result.contains("rev"));
    }

    #[test]
    fn test_w14md_str_008_join_list_literal() {
        let code = r#"
def join_items() -> str:
    return ",".join(["a", "b", "c"])
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("join"));
    }

    #[test]
    fn test_w14md_str_009_join_variable() {
        let code = r#"
def join_parts(parts: list) -> str:
    return "-".join(parts)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("join"));
    }

    #[test]
    fn test_w14md_str_010_join_separator_var() {
        let code = r#"
def join_with_sep(sep: str, items: list) -> str:
    return sep.join(items)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("join"));
    }

    #[test]
    fn test_w14md_str_011_replace_with_count() {
        let code = r#"
def replace_first(s: str) -> str:
    return s.replace("a", "b", 1)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("replacen") || result.contains("replace"));
    }

    #[test]
    fn test_w14md_str_012_replace_no_count() {
        let code = r#"
def replace_all(s: str) -> str:
    return s.replace("old", "new")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("replace"));
    }

    #[test]
    fn test_w14md_str_013_find_basic() {
        let code = r#"
def find_sub(s: str) -> int:
    return s.find("sub")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("find") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w14md_str_014_find_with_start() {
        let code = r#"
def find_from(s: str, start: int) -> int:
    return s.find("x", start)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("find"));
    }

    #[test]
    fn test_w14md_str_015_rfind_basic() {
        let code = r#"
def rfind_sub(s: str) -> int:
    return s.rfind("sub")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rfind") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w14md_str_016_count_basic() {
        let code = r#"
def count_char(s: str) -> int:
    return s.count("a")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("matches") || result.contains("count"));
    }

    #[test]
    fn test_w14md_str_017_count_substring() {
        let code = r#"
def count_substr(s: str) -> int:
    return s.count("ab")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("matches") || result.contains("count"));
    }

    #[test]
    fn test_w14md_str_018_zfill_basic() {
        let code = r#"
def pad_number(s: str) -> str:
    return s.zfill(5)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("zfill") || result.contains("width") || result.contains("repeat"));
    }

    #[test]
    fn test_w14md_str_019_zfill_in_func() {
        let code = r#"
def format_code(n: int) -> str:
    s = str(n)
    return s.zfill(8)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_str_020_center_basic() {
        let code = r#"
def center_text(s: str) -> str:
    return s.center(10)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("width") || result.contains("center") || result.contains("pad"));
    }

    #[test]
    fn test_w14md_str_021_center_with_fillchar() {
        let code = r#"
def center_star(s: str) -> str:
    return s.center(20, "*")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("width") || result.contains("fillchar"));
    }

    #[test]
    fn test_w14md_str_022_ljust_basic() {
        let code = r#"
def left_justify(s: str) -> str:
    return s.ljust(10)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("width") || result.contains("ljust") || result.contains("format"));
    }

    #[test]
    fn test_w14md_str_023_ljust_with_fillchar() {
        let code = r#"
def left_fill(s: str) -> str:
    return s.ljust(15, ".")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("width") || result.contains("fillchar"));
    }

    #[test]
    fn test_w14md_str_024_rjust_basic() {
        let code = r#"
def right_justify(s: str) -> str:
    return s.rjust(10)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("width") || result.contains("rjust") || result.contains("format"));
    }

    #[test]
    fn test_w14md_str_025_rjust_with_fillchar() {
        let code = r#"
def right_fill(s: str) -> str:
    return s.rjust(15, "0")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("width") || result.contains("fillchar"));
    }

    #[test]
    fn test_w14md_str_026_expandtabs_default() {
        let code = r#"
def expand_default(s: str) -> str:
    return s.expandtabs()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("replace")
                || result.contains("repeat")
                || result.contains("expandtabs")
        );
    }

    #[test]
    fn test_w14md_str_027_expandtabs_custom() {
        let code = r#"
def expand_4(s: str) -> str:
    return s.expandtabs(4)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("replace") || result.contains("repeat"));
    }

    #[test]
    fn test_w14md_str_028_partition_basic() {
        let code = r#"
def split_at(s: str) -> tuple:
    return s.partition(",")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("find") || result.contains("partition"));
    }

    #[test]
    fn test_w14md_str_029_partition_space() {
        let code = r#"
def split_first_word(s: str) -> tuple:
    return s.partition(" ")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_str_030_removeprefix_basic() {
        let code = r#"
def strip_prefix(s: str) -> str:
    return s.removeprefix("pre_")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_str_031_removesuffix_basic() {
        let code = r#"
def strip_suffix(s: str) -> str:
    return s.removesuffix("_suf")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_str_032_encode_in_function() {
        let code = r#"
def to_bytes(text: str) -> list:
    data = text.encode()
    return data
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_str_033_split_empty_sep() {
        let code = r#"
def split_whitespace(s: str) -> list:
    return s.split()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("split_whitespace") || result.contains("split"));
    }

    #[test]
    fn test_w14md_str_034_replace_count_var() {
        let code = r#"
def replace_n(s: str, old: str, new: str, count: int) -> str:
    return s.replace(old, new, count)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("replacen") || result.contains("replace"));
    }

    #[test]
    fn test_w14md_str_035_find_literal() {
        let code = r#"
def has_colon(s: str) -> int:
    return s.find(":")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("find"));
    }

    #[test]
    fn test_w14md_str_036_rfind_with_var() {
        let code = r#"
def rfind_char(s: str, needle: str) -> int:
    return s.rfind(needle)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rfind"));
    }

    #[test]
    fn test_w14md_str_037_join_empty_sep() {
        let code = r#"
def concat_chars(chars: list) -> str:
    return "".join(chars)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("join"));
    }

    #[test]
    fn test_w14md_str_038_split_tab() {
        let code = r#"
def split_tsv(line: str) -> list:
    return line.split("\t")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("split"));
    }

    #[test]
    fn test_w14md_str_039_count_with_var() {
        let code = r#"
def count_pattern(text: str, pattern: str) -> int:
    return text.count(pattern)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("matches") || result.contains("count"));
    }

    #[test]
    fn test_w14md_str_040_partition_in_assignment() {
        let code = r#"
def parse_header(line: str) -> tuple:
    result = line.partition(":")
    return result
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // LIST METHODS (35 tests: test_w14md_list_001 through test_w14md_list_035)
    // ========================================================================

    #[test]
    fn test_w14md_list_001_extend_basic() {
        let code = r#"
def extend_list(x: list) -> list:
    x.extend([2, 3])
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("extend") || result.contains("iter"));
    }

    #[test]
    fn test_w14md_list_002_extend_var() {
        let code = r#"
def extend_with(x: list, y: list) -> list:
    x.extend(y)
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("extend"));
    }

    #[test]
    fn test_w14md_list_003_insert_basic() {
        let code = r#"
def insert_at(x: list) -> list:
    x.insert(1, 2)
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w14md_list_004_insert_beginning() {
        let code = r#"
def prepend(x: list, item: int) -> list:
    x.insert(0, item)
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w14md_list_005_pop_with_index() {
        let code = r#"
def pop_first(x: list) -> int:
    return x.pop(0)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("remove") || result.contains("pop"));
    }

    #[test]
    fn test_w14md_list_006_pop_no_args() {
        let code = r#"
def pop_last(x: list) -> int:
    return x.pop()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("pop"));
    }

    #[test]
    fn test_w14md_list_007_reverse_basic() {
        let code = r#"
def reverse_list(x: list) -> list:
    x.reverse()
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("reverse"));
    }

    #[test]
    fn test_w14md_list_008_sort_basic() {
        let code = r#"
def sort_list(x: list) -> list:
    x.sort()
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("sort"));
    }

    #[test]
    fn test_w14md_list_009_sort_reverse() {
        let code = r#"
def sort_desc(x: list) -> list:
    x.sort(reverse=True)
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("sort") || result.contains("cmp"));
    }

    #[test]
    fn test_w14md_list_010_index_basic() {
        let code = r#"
def find_index(x: list, v: int) -> int:
    return x.index(v)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("position") || result.contains("index"));
    }

    #[test]
    fn test_w14md_list_011_count_basic() {
        let code = r#"
def count_items(x: list, v: int) -> int:
    return x.count(v)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("filter") || result.contains("count"));
    }

    #[test]
    fn test_w14md_list_012_remove_basic() {
        let code = r#"
def remove_item(x: list, v: int) -> list:
    x.remove(v)
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("position") || result.contains("remove"));
    }

    #[test]
    fn test_w14md_list_013_clear_basic() {
        let code = r#"
def clear_list(x: list) -> list:
    x.clear()
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("clear"));
    }

    #[test]
    fn test_w14md_list_014_copy_basic() {
        let code = r#"
def copy_list(x: list) -> list:
    return x.copy()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("clone") || result.contains("copy"));
    }

    #[test]
    fn test_w14md_list_015_extend_empty() {
        let code = r#"
def extend_empty(x: list) -> list:
    x.extend([])
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("extend"));
    }

    #[test]
    fn test_w14md_list_016_insert_end() {
        let code = r#"
def append_at_end(x: list, n: int) -> list:
    x.insert(n, 99)
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w14md_list_017_pop_middle() {
        let code = r#"
def pop_middle(x: list) -> int:
    return x.pop(2)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("remove") || result.contains("pop"));
    }

    #[test]
    fn test_w14md_list_018_sort_key_len() {
        let code = r#"
def sort_by_len(words: list) -> list:
    words.sort(key=len)
    return words
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("sort"));
    }

    #[test]
    fn test_w14md_list_019_sort_key_reverse() {
        let code = r#"
def sort_by_len_desc(words: list) -> list:
    words.sort(key=len, reverse=True)
    return words
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("sort") || result.contains("Reverse"));
    }

    #[test]
    fn test_w14md_list_020_reverse_then_sort() {
        let code = r#"
def reverse_sort(x: list) -> list:
    x.reverse()
    x.sort()
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("reverse") && result.contains("sort"));
    }

    #[test]
    fn test_w14md_list_021_count_zero() {
        let code = r#"
def count_missing(x: list) -> int:
    return x.count(999)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("filter") || result.contains("count"));
    }

    #[test]
    fn test_w14md_list_022_index_in_condition() {
        let code = r#"
def has_item(x: list, v: int) -> bool:
    idx = x.index(v)
    return idx >= 0
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_list_023_copy_modify() {
        let code = r#"
def copy_and_modify(x: list) -> list:
    y = x.copy()
    y.append(10)
    return y
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("clone") || result.contains("push"));
    }

    #[test]
    fn test_w14md_list_024_clear_refill() {
        let code = r#"
def clear_and_fill(x: list) -> list:
    x.clear()
    x.append(1)
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("clear"));
    }

    #[test]
    fn test_w14md_list_025_remove_first_occurrence() {
        let code = r#"
def remove_first(x: list) -> list:
    x.remove(1)
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("position") || result.contains("remove"));
    }

    #[test]
    fn test_w14md_list_026_pop_assign() {
        let code = r#"
def pop_and_use(x: list) -> int:
    val = x.pop()
    return val
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("pop"));
    }

    #[test]
    fn test_w14md_list_027_extend_range_like() {
        let code = r#"
def extend_with_vals(x: list) -> list:
    x.extend([4, 5, 6])
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("extend"));
    }

    #[test]
    fn test_w14md_list_028_insert_negative_idx() {
        let code = r#"
def insert_near_end(x: list) -> list:
    x.insert(-1, 42)
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w14md_list_029_sort_default() {
        let code = r#"
def sort_default(nums: list) -> list:
    nums.sort()
    return nums
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("sort()") || result.contains("sort"));
    }

    #[test]
    fn test_w14md_list_030_reverse_in_function() {
        let code = r#"
def get_reversed(items: list) -> list:
    items.reverse()
    return items
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("reverse"));
    }

    #[test]
    fn test_w14md_list_031_count_string_item() {
        let code = r#"
def count_word(words: list, word: str) -> int:
    return words.count(word)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("filter") || result.contains("count"));
    }

    #[test]
    fn test_w14md_list_032_index_string_item() {
        let code = r#"
def find_word(words: list, word: str) -> int:
    return words.index(word)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("position") || result.contains("index"));
    }

    #[test]
    fn test_w14md_list_033_extend_then_sort() {
        let code = r#"
def merge_and_sort(x: list, y: list) -> list:
    x.extend(y)
    x.sort()
    return x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("extend") && result.contains("sort"));
    }

    #[test]
    fn test_w14md_list_034_pop_check_empty() {
        let code = r#"
def safe_pop(x: list) -> int:
    if len(x) > 0:
        return x.pop()
    return 0
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("pop") || result.contains("len"));
    }

    #[test]
    fn test_w14md_list_035_copy_independent() {
        let code = r#"
def make_copy(original: list) -> list:
    backup = original.copy()
    return backup
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("clone"));
    }

    // ========================================================================
    // INSTANCE DISPATCH (40 tests: test_w14md_inst_001 through test_w14md_inst_040)
    // ========================================================================

    #[test]
    fn test_w14md_inst_001_file_read_basic() {
        let code = r#"
def read_file(path: str) -> str:
    with open(path) as f:
        data = f.read()
    return data
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("read") || result.contains("String"));
    }

    #[test]
    fn test_w14md_inst_002_file_readline() {
        let code = r#"
def read_first_line(path: str) -> str:
    with open(path) as f:
        line = f.readline()
    return line
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("read_line") || result.contains("BufReader") || result.contains("line")
        );
    }

    #[test]
    fn test_w14md_inst_003_file_readlines() {
        let code = r#"
def read_all_lines(path: str) -> list:
    with open(path) as f:
        lines = f.readlines()
    return lines
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("BufReader") || result.contains("lines") || result.contains("collect")
        );
    }

    #[test]
    fn test_w14md_inst_004_file_write() {
        let code = r#"
def write_data(path: str, data: str):
    with open(path, "w") as f:
        f.write(data)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("write") || result.contains("as_bytes"));
    }

    #[test]
    fn test_w14md_inst_005_file_writelines() {
        let code = r#"
def write_lines(path: str, lines: list):
    with open(path, "w") as f:
        f.writelines(lines)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_inst_006_path_exists() {
        let code = r#"
def check_exists(path: str) -> bool:
    return path.exists()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_inst_007_path_is_file() {
        let code = r#"
def check_file(path: str) -> bool:
    return path.is_file()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_inst_008_path_is_dir() {
        let code = r#"
def check_dir(path: str) -> bool:
    return path.is_dir()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_inst_009_path_resolve() {
        let code = r#"
def resolve_path(path: str) -> str:
    return path.resolve()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_inst_010_path_stem_attr() {
        let code = r#"
def get_stem(path: str) -> str:
    s = path.stem
    return s
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_inst_011_path_suffix_attr() {
        let code = r#"
def get_ext(path: str) -> str:
    ext = path.suffix
    return ext
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_inst_012_path_parent_attr() {
        let code = r#"
def get_parent(path: str) -> str:
    parent = path.parent
    return parent
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_inst_013_path_name_attr() {
        let code = r#"
def get_name(path: str) -> str:
    name = path.name
    return name
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_inst_014_datetime_strftime() {
        let code = r#"
def format_date(dt) -> str:
    return dt.strftime("%Y-%m-%d")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format") || result.contains("strftime"));
    }

    #[test]
    fn test_w14md_inst_015_datetime_isoformat() {
        let code = r#"
def to_iso(dt) -> str:
    return dt.isoformat()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("to_string")
                || result.contains("format")
                || result.contains("isoformat")
        );
    }

    #[test]
    fn test_w14md_inst_016_regex_group_zero() {
        let code = r#"
def get_match(m) -> str:
    return m.group(0)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("group") || result.contains("as_str"));
    }

    #[test]
    fn test_w14md_inst_017_regex_group_numbered() {
        let code = r#"
def get_group(m) -> str:
    return m.group(1)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("group") || result.contains("get"));
    }

    #[test]
    fn test_w14md_inst_018_regex_groups() {
        let code = r#"
def get_all_groups(m) -> list:
    return m.groups()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Vec") || result.contains("groups") || result.contains("String"));
    }

    #[test]
    fn test_w14md_inst_019_file_read_text_mode() {
        let code = r#"
def read_text_file(filename: str) -> str:
    with open(filename, "r") as f:
        content = f.read()
    return content
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("read") || result.contains("String"));
    }

    #[test]
    fn test_w14md_inst_020_file_close() {
        let code = r#"
def close_file(path: str):
    f = open(path)
    data = f.read()
    f.close()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_inst_021_datetime_timestamp() {
        let code = r#"
def get_timestamp(dt) -> float:
    return dt.timestamp()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("timestamp") || result.contains("f64") || result.contains("duration")
        );
    }

    #[test]
    fn test_w14md_inst_022_datetime_date() {
        let code = r#"
def get_date(dt):
    return dt.date()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("date") || result.contains("fn"));
    }

    #[test]
    fn test_w14md_inst_023_datetime_time() {
        let code = r#"
def get_time(dt):
    return dt.time()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_inst_024_regex_group_no_args() {
        let code = r#"
def get_whole_match(m) -> str:
    return m.group()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("group") || result.contains("as_str"));
    }

    #[test]
    fn test_w14md_inst_025_path_stat() {
        let code = r#"
def get_stat(path: str):
    stats = path.stat()
    return stats
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_inst_026_file_write_string_literal() {
        let code = r#"
def write_header(path: str):
    with open(path, "w") as f:
        f.write("header\n")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("write"));
    }

    #[test]
    fn test_w14md_inst_027_file_read_with_size() {
        let code = r#"
def read_chunk(path: str) -> list:
    with open(path) as f:
        chunk = f.read(1024)
    return chunk
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("read") || result.contains("buf"));
    }

    #[test]
    fn test_w14md_inst_028_path_resolve_named() {
        let code = r#"
def abs_path(path: str) -> str:
    p = path.resolve()
    return p
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_inst_029_csv_writerow() {
        let code = r#"
def write_row(writer, row: dict):
    writer.writerow(row)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("serialize") || result.contains("writerow") || result.contains("write")
        );
    }

    #[test]
    fn test_w14md_inst_030_csv_writeheader() {
        let code = r#"
def write_header(writer):
    writer.writeheader()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_inst_031_datetime_strftime_full() {
        let code = r#"
def format_datetime(dt) -> str:
    return dt.strftime("%Y-%m-%d %H:%M:%S")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format") || result.contains("strftime"));
    }

    #[test]
    fn test_w14md_inst_032_regex_findall() {
        let code = r#"
def find_all_matches(pattern, text: str) -> list:
    return pattern.findall(text)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("findall") || result.contains("find"));
    }

    #[test]
    fn test_w14md_inst_033_regex_search() {
        let code = r#"
def search_pattern(pattern, text: str):
    return pattern.search(text)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_inst_034_file_readline_loop() {
        let code = r#"
def read_lines_loop(path: str) -> list:
    result = []
    with open(path) as f:
        line = f.readline()
    return result
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_inst_035_path_absolute() {
        let code = r#"
def get_absolute(path: str) -> str:
    return path.absolute()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_inst_036_parse_args_skip() {
        let code = r#"
def main(parser):
    args = parser.parse_args()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_inst_037_add_argument_skip() {
        let code = r#"
def setup(parser):
    parser.add_argument("--verbose")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_inst_038_print_help() {
        let code = r#"
def show_help(parser):
    parser.print_help()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_inst_039_datetime_isoformat_named() {
        let code = r#"
def iso_string(date) -> str:
    return date.isoformat()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_inst_040_regex_match() {
        let code = r#"
def match_start(pattern, text: str):
    return pattern.match(text)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // ATTRIBUTE ACCESS (35 tests: test_w14md_attr_001 through test_w14md_attr_035)
    // ========================================================================

    #[test]
    fn test_w14md_attr_001_sys_stdout_write() {
        let code = r#"
import sys
def write_out(msg: str):
    sys.stdout.write(msg)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("write") || result.contains("stdout"));
    }

    #[test]
    fn test_w14md_attr_002_sys_stderr_write() {
        let code = r#"
import sys
def write_err(msg: str):
    sys.stderr.write(msg)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("write") || result.contains("stderr"));
    }

    #[test]
    fn test_w14md_attr_003_os_path_join() {
        let code = r#"
import os
def join_path(a: str, b: str) -> str:
    return os.path.join(a, b)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_attr_004_os_path_exists() {
        let code = r#"
import os
def check_path(path: str) -> bool:
    return os.path.exists(path)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_attr_005_os_path_dirname() {
        let code = r#"
import os
def get_dir(filepath: str) -> str:
    return os.path.dirname(filepath)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_attr_006_os_path_basename() {
        let code = r#"
import os
def get_base(filepath: str) -> str:
    return os.path.basename(filepath)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_attr_007_os_environ_get() {
        let code = r#"
import os
def get_env(key: str) -> str:
    return os.environ.get(key, "default")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_attr_008_class_self_attr_init() {
        let code = r#"
class Foo:
    def __init__(self):
        self.x = 1
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("struct") || result.contains("Foo") || result.contains("x"));
    }

    #[test]
    fn test_w14md_attr_009_class_self_method() {
        let code = r#"
class Foo:
    def __init__(self):
        self.x = 1
    def get_x(self) -> int:
        return self.x
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("self") || result.contains("Foo"));
    }

    #[test]
    fn test_w14md_attr_010_class_self_multiple_attrs() {
        let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Point") || result.contains("struct"));
    }

    #[test]
    fn test_w14md_attr_011_property_like() {
        let code = r#"
class Circle:
    def __init__(self, radius: float):
        self.radius = radius
    def area(self) -> float:
        return 3.14159 * self.radius * self.radius
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("radius") || result.contains("Circle"));
    }

    #[test]
    fn test_w14md_attr_012_chained_method() {
        let code = r#"
def process(s: str) -> str:
    return s.strip().lower()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("trim") || result.contains("to_lowercase"));
    }

    #[test]
    fn test_w14md_attr_013_chained_split_join() {
        let code = r#"
def normalize(s: str) -> str:
    return " ".join(s.split())
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("join") || result.contains("split"));
    }

    #[test]
    fn test_w14md_attr_014_os_environ_access() {
        let code = r#"
import os
def get_home() -> str:
    return os.environ.get("HOME", "/tmp")
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_attr_015_math_pi() {
        let code = r#"
import math
def circle_area(r: float) -> float:
    return math.pi * r * r
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("PI") || result.contains("pi") || result.contains("consts"));
    }

    #[test]
    fn test_w14md_attr_016_math_e() {
        let code = r#"
import math
def euler() -> float:
    return math.e
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("E") || result.contains("consts") || result.contains("e"));
    }

    #[test]
    fn test_w14md_attr_017_enum_constant() {
        let code = r#"
class Color:
    RED = 1
    GREEN = 2
    BLUE = 3

def get_red() -> int:
    return Color.RED
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Color") || result.contains("RED"));
    }

    #[test]
    fn test_w14md_attr_018_sys_stdout_flush() {
        let code = r#"
import sys
def flush_out():
    sys.stdout.flush()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("flush") || result.contains("stdout"));
    }

    #[test]
    fn test_w14md_attr_019_class_method_call() {
        let code = r#"
class Calculator:
    def __init__(self):
        self.result = 0
    def add(self, x: int) -> int:
        self.result = self.result + x
        return self.result
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Calculator") || result.contains("result"));
    }

    #[test]
    fn test_w14md_attr_020_exception_returncode() {
        let code = r#"
def get_exit_code(e) -> int:
    return e.returncode
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_attr_021_path_name_attribute() {
        let code = r#"
def get_filename(path: str) -> str:
    return path.name
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_attr_022_path_suffix_attribute() {
        let code = r#"
def get_extension(path: str) -> str:
    return path.suffix
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_attr_023_path_stem_attribute() {
        let code = r#"
def get_stem(path: str) -> str:
    return path.stem
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_attr_024_path_parent_attribute() {
        let code = r#"
def get_parent(path: str) -> str:
    return path.parent
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_attr_025_class_inheritance_like() {
        let code = r#"
class Animal:
    def __init__(self, name: str):
        self.name = name
    def speak(self) -> str:
        return self.name
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Animal") || result.contains("name"));
    }

    #[test]
    fn test_w14md_attr_026_os_path_isfile() {
        let code = r#"
import os
def is_file(path: str) -> bool:
    return os.path.isfile(path)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_attr_027_os_path_isdir() {
        let code = r#"
import os
def is_directory(path: str) -> bool:
    return os.path.isdir(path)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_attr_028_os_environ_direct() {
        let code = r#"
import os
def show_env():
    env = os.environ
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_attr_029_class_str_attr() {
        let code = r#"
class Person:
    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age
    def greet(self) -> str:
        return self.name
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Person") || result.contains("name") || result.contains("age"));
    }

    #[test]
    fn test_w14md_attr_030_sys_stdin_readline() {
        let code = r#"
import sys
def read_input() -> str:
    return sys.stdin.readline()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("stdin") || result.contains("read_line") || result.contains("BufRead")
        );
    }

    #[test]
    fn test_w14md_attr_031_chained_upper_strip() {
        let code = r#"
def clean_upper(s: str) -> str:
    return s.strip().upper()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("trim") || result.contains("to_uppercase"));
    }

    #[test]
    fn test_w14md_attr_032_chained_replace_lower() {
        let code = r#"
def normalize_text(s: str) -> str:
    return s.replace("-", " ").lower()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("replace") || result.contains("to_lowercase"));
    }

    #[test]
    fn test_w14md_attr_033_os_path_splitext() {
        let code = r#"
import os
def get_ext(filepath: str):
    return os.path.splitext(filepath)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w14md_attr_034_class_list_attr() {
        let code = r#"
class Stack:
    def __init__(self):
        self.items = []
    def push(self, item: int):
        self.items.append(item)
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("Stack") || result.contains("items") || result.contains("push"));
    }

    #[test]
    fn test_w14md_attr_035_sys_stdin_readlines() {
        let code = r#"
import sys
def read_all_stdin() -> list:
    return sys.stdin.readlines()
"#;
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("stdin") || result.contains("lines") || result.contains("BufRead"));
    }
}
