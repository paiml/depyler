//! Session 11: Deep coverage tests for expr_gen_instance_methods.rs
//!
//! Targets the #1 coverage bottleneck (65% covered, 4421 missed regions):
//! - String methods with fillchar arguments
//! - Strip/lstrip/rstrip with charset
//! - Find with start position
//! - Split with maxsplit
//! - Set algebra operations (intersection_update, difference_update)
//! - Dict edge cases (popitem, setdefault, fromkeys)
//! - Sort with key and reverse combinations
//! - Partition/rpartition
//! - Decode method
//! - String is*() methods via char iteration

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
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

// ============================================================================
// String methods with fillchar/charset arguments
// ============================================================================

#[test]
fn test_s11_deep_center_with_fillchar() {
    let code = r#"
def pad_center(s: str) -> str:
    return s.center(20, "*")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn pad_center"),
        "Should transpile center with fillchar. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_ljust_with_fillchar() {
    let code = r#"
def pad_left(s: str) -> str:
    return s.ljust(20, "-")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn pad_left"),
        "Should transpile ljust with fillchar. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_rjust_with_fillchar() {
    let code = r#"
def pad_right(s: str) -> str:
    return s.rjust(20, "+")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn pad_right"),
        "Should transpile rjust with fillchar. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_zfill_negative() {
    let code = r#"
def zero_fill(s: str) -> str:
    return s.zfill(10)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn zero_fill"),
        "Should transpile zfill. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_strip_with_chars() {
    let code = r#"
def strip_chars(s: str) -> str:
    return s.strip("xyz")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn strip_chars"),
        "Should transpile strip with charset. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_lstrip_with_chars() {
    let code = r#"
def lstrip_chars(s: str) -> str:
    return s.lstrip("0")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn lstrip_chars"),
        "Should transpile lstrip with charset. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_rstrip_with_chars() {
    let code = r#"
def rstrip_chars(s: str) -> str:
    return s.rstrip("\n")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn rstrip_chars"),
        "Should transpile rstrip with charset. Got: {}",
        result
    );
}

// ============================================================================
// String find/split with extra arguments
// ============================================================================

#[test]
fn test_s11_deep_find_with_start() {
    let code = r#"
def find_from(s: str, sub: str, start: int) -> int:
    return s.find(sub, start)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn find_from"),
        "Should transpile find with start. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_rfind_basic() {
    let code = r#"
def find_last(s: str, sub: str) -> int:
    return s.rfind(sub)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn find_last"),
        "Should transpile rfind. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_split_with_maxsplit() {
    let code = r#"
def split_limited(s: str) -> list:
    return s.split(",", 2)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn split_limited"),
        "Should transpile split with maxsplit. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_rsplit_basic() {
    let code = r#"
def split_right(s: str) -> list:
    return s.rsplit(",")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn split_right"),
        "Should transpile rsplit. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_splitlines() {
    let code = r#"
def get_lines(s: str) -> list:
    return s.splitlines()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_lines"),
        "Should transpile splitlines. Got: {}",
        result
    );
}

// ============================================================================
// String partition methods
// ============================================================================

#[test]
fn test_s11_deep_partition() {
    let code = r#"
def split_at(s: str) -> tuple:
    return s.partition("=")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn split_at"),
        "Should transpile partition. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_rpartition() {
    let code = r#"
def split_at_last(s: str) -> tuple:
    return s.rpartition(".")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn split_at_last"),
        "Should transpile rpartition. Got: {}",
        result
    );
}

// ============================================================================
// String format method
// ============================================================================

#[test]
fn test_s11_deep_format_multiple_args() {
    let code = r#"
def fmt(name: str, age: int) -> str:
    return "{}: {}".format(name, age)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn fmt"),
        "Should transpile format with multiple args. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_format_no_args() {
    let code = r#"
def empty_fmt() -> str:
    return "hello".format()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn empty_fmt"),
        "Should transpile format with no args. Got: {}",
        result
    );
}

// ============================================================================
// String is*() methods
// ============================================================================

#[test]
fn test_s11_deep_isnumeric() {
    let code = r#"
def check_numeric(s: str) -> bool:
    return s.isnumeric()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_numeric"),
        "Should transpile isnumeric. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_isascii() {
    let code = r#"
def check_ascii(s: str) -> bool:
    return s.isascii()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_ascii"),
        "Should transpile isascii. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_isdecimal() {
    let code = r#"
def check_decimal(s: str) -> bool:
    return s.isdecimal()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_decimal"),
        "Should transpile isdecimal. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_isidentifier() {
    let code = r#"
def check_ident(s: str) -> bool:
    return s.isidentifier()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_ident"),
        "Should transpile isidentifier. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_isprintable() {
    let code = r#"
def check_printable(s: str) -> bool:
    return s.isprintable()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_printable"),
        "Should transpile isprintable. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_istitle() {
    let code = r#"
def check_title(s: str) -> bool:
    return s.istitle()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_title"),
        "Should transpile istitle. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_casefold() {
    let code = r#"
def lower_case(s: str) -> str:
    return s.casefold()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn lower_case"),
        "Should transpile casefold. Got: {}",
        result
    );
}

// ============================================================================
// String char iteration methods
// ============================================================================

#[test]
fn test_s11_deep_char_isdigit_in_loop() {
    let code = r#"
def count_digits(s: str) -> int:
    count = 0
    for c in s:
        if c.isdigit():
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn count_digits"),
        "Should transpile char isdigit in loop. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_char_isalpha_in_loop() {
    let code = r#"
def count_alpha(s: str) -> int:
    count = 0
    for c in s:
        if c.isalpha():
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn count_alpha"),
        "Should transpile char isalpha in loop. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_char_isspace_in_loop() {
    let code = r#"
def count_spaces(s: str) -> int:
    count = 0
    for c in s:
        if c.isspace():
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn count_spaces"),
        "Should transpile char isspace in loop. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_char_isupper_in_loop() {
    let code = r#"
def count_upper(s: str) -> int:
    count = 0
    for c in s:
        if c.isupper():
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn count_upper"),
        "Should transpile char isupper in loop. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_char_islower_in_loop() {
    let code = r#"
def count_lower(s: str) -> int:
    count = 0
    for c in s:
        if c.islower():
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn count_lower"),
        "Should transpile char islower in loop. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_char_isalnum_in_loop() {
    let code = r#"
def count_alnum(s: str) -> int:
    count = 0
    for c in s:
        if c.isalnum():
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn count_alnum"),
        "Should transpile char isalnum in loop. Got: {}",
        result
    );
}

// ============================================================================
// Set algebra operations
// ============================================================================

#[test]
fn test_s11_deep_set_intersection_update() {
    let code = r#"
def intersect_sets(a: set, b: set) -> set:
    a.intersection_update(b)
    return a
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn intersect_sets"),
        "Should transpile intersection_update. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_set_difference_update() {
    let code = r#"
def diff_sets(a: set, b: set) -> set:
    a.difference_update(b)
    return a
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn diff_sets"),
        "Should transpile difference_update. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_set_symmetric_difference() {
    let code = r#"
def sym_diff(a: set, b: set) -> set:
    return a.symmetric_difference(b)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sym_diff"),
        "Should transpile symmetric_difference. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_set_issubset() {
    let code = r#"
def is_sub(a: set, b: set) -> bool:
    return a.issubset(b)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn is_sub"),
        "Should transpile issubset. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_set_issuperset() {
    let code = r#"
def is_super(a: set, b: set) -> bool:
    return a.issuperset(b)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn is_super"),
        "Should transpile issuperset. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_set_isdisjoint() {
    let code = r#"
def no_common(a: set, b: set) -> bool:
    return a.isdisjoint(b)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn no_common"),
        "Should transpile isdisjoint. Got: {}",
        result
    );
}

// ============================================================================
// Dict edge cases
// ============================================================================

#[test]
fn test_s11_deep_dict_popitem() {
    let code = r#"
def pop_last(d: dict) -> tuple:
    return d.popitem()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn pop_last"),
        "Should transpile popitem. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_dict_setdefault() {
    let code = r#"
def get_or_set(d: dict, key: str) -> int:
    return d.setdefault(key, 0)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_or_set"),
        "Should transpile setdefault. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_dict_fromkeys() {
    let code = r#"
def make_dict(keys: list) -> dict:
    return dict.fromkeys(keys, 0)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn make_dict"),
        "Should transpile fromkeys. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_dict_update() {
    let code = r#"
def merge_dicts(d1: dict, d2: dict) -> dict:
    d1.update(d2)
    return d1
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn merge_dicts"),
        "Should transpile dict update. Got: {}",
        result
    );
}

// ============================================================================
// Sort with key and reverse combinations
// ============================================================================

#[test]
fn test_s11_deep_sort_with_key() {
    let code = r#"
def sort_by_len(items: list) -> list:
    items.sort(key=len)
    return items
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sort_by_len"),
        "Should transpile sort with key. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_sort_reverse() {
    let code = r#"
def sort_desc(items: list) -> list:
    items.sort(reverse=True)
    return items
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sort_desc"),
        "Should transpile sort with reverse. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_sort_key_and_reverse() {
    let code = r#"
def sort_by_len_desc(items: list) -> list:
    items.sort(key=len, reverse=True)
    return items
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sort_by_len_desc"),
        "Should transpile sort with key and reverse. Got: {}",
        result
    );
}

// ============================================================================
// List methods - edge cases
// ============================================================================

#[test]
fn test_s11_deep_list_extend() {
    let code = r#"
def extend_list(a: list, b: list) -> list:
    a.extend(b)
    return a
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn extend_list"),
        "Should transpile list extend. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_list_insert() {
    let code = r#"
def insert_at(items: list, pos: int, val: int) -> list:
    items.insert(pos, val)
    return items
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn insert_at"),
        "Should transpile list insert. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_list_remove() {
    let code = r#"
def remove_val(items: list, val: int) -> list:
    items.remove(val)
    return items
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn remove_val"),
        "Should transpile list remove. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_list_reverse() {
    let code = r#"
def reverse_list(items: list) -> list:
    items.reverse()
    return items
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn reverse_list"),
        "Should transpile list reverse. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_list_pop_with_index() {
    let code = r#"
def pop_at(items: list, idx: int) -> int:
    return items.pop(idx)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn pop_at"),
        "Should transpile list pop with index. Got: {}",
        result
    );
}

// ============================================================================
// String encode/decode
// ============================================================================

#[test]
fn test_s11_deep_str_encode() {
    let code = r#"
def to_bytes(s: str) -> bytes:
    return s.encode("utf-8")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn to_bytes"),
        "Should transpile str encode. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_str_encode_no_arg() {
    let code = r#"
def to_bytes_default(s: str) -> bytes:
    return s.encode()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn to_bytes_default"),
        "Should transpile str encode no arg. Got: {}",
        result
    );
}

// ============================================================================
// String additional methods
// ============================================================================

#[test]
fn test_s11_deep_str_swapcase() {
    let code = r#"
def swap(s: str) -> str:
    return s.swapcase()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn swap"),
        "Should transpile swapcase. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_str_expandtabs() {
    let code = r#"
def expand(s: str) -> str:
    return s.expandtabs(4)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn expand"),
        "Should transpile expandtabs. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_str_maketrans() {
    let code = r#"
def make_table() -> dict:
    return str.maketrans("abc", "xyz")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn make_table"),
        "Should transpile maketrans. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_str_removeprefix() {
    let code = r#"
def strip_prefix(s: str) -> str:
    return s.removeprefix("test_")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn strip_prefix"),
        "Should transpile removeprefix. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_str_removesuffix() {
    let code = r#"
def strip_suffix(s: str) -> str:
    return s.removesuffix(".py")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn strip_suffix"),
        "Should transpile removesuffix. Got: {}",
        result
    );
}

// ============================================================================
// Dict iteration patterns
// ============================================================================

#[test]
fn test_s11_deep_dict_keys_iterate() {
    let code = r#"
def get_keys(d: dict) -> list:
    result: list = []
    for k in d.keys():
        result.append(k)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_keys"),
        "Should transpile dict keys iteration. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_dict_values_iterate() {
    let code = r#"
def sum_values(d: dict) -> int:
    total = 0
    for v in d.values():
        total += v
    return total
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sum_values"),
        "Should transpile dict values iteration. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_dict_items_iterate() {
    let code = r#"
def format_items(d: dict) -> list:
    result: list = []
    for k, v in d.items():
        result.append(f"{k}={v}")
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn format_items"),
        "Should transpile dict items iteration. Got: {}",
        result
    );
}

// ============================================================================
// String join with various iterables
// ============================================================================

#[test]
fn test_s11_deep_join_list() {
    let code = r#"
def join_words(words: list) -> str:
    return " ".join(words)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn join_words"),
        "Should transpile join with list. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_join_with_newline() {
    let code = r#"
def join_lines(lines: list) -> str:
    return "\n".join(lines)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn join_lines"),
        "Should transpile join with newline. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_join_empty_sep() {
    let code = r#"
def concat_all(parts: list) -> str:
    return "".join(parts)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn concat_all"),
        "Should transpile join with empty separator. Got: {}",
        result
    );
}
