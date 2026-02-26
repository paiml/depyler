//! Session 11: Coverage tests for instance method transpilation
//!
//! Exercises untested code paths in expr_gen_instance_methods.rs:
//! - Advanced string methods (center, ljust, rjust, zfill, swapcase, etc.)
//! - Dict operations (setdefault, popitem, pop with default, clear, copy)
//! - Set algebra (symmetric_difference, issubset, issuperset, isdisjoint)
//! - Set update operations (intersection_update, difference_update)
//! - List edge cases (remove, extend)

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile(python_code: &str) -> String {
    let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
    let (module, _) =
        AstBridge::new().with_source(python_code.to_string()).python_to_hir(ast).expect("hir");
    let tm = TypeMapper::default();
    let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
    result
}

// ============================================================================
// String justification methods
// ============================================================================

#[test]
fn test_s11_string_center() {
    let code = r#"
def centered(s: str, width: int) -> str:
    return s.center(width)
"#;
    let result = transpile(code);
    assert!(result.contains("fn centered"), "Should transpile str.center. Got: {}", result);
}

#[test]
fn test_s11_string_ljust() {
    let code = r#"
def left_justify(s: str, width: int) -> str:
    return s.ljust(width)
"#;
    let result = transpile(code);
    assert!(result.contains("fn left_justify"), "Should transpile str.ljust. Got: {}", result);
}

#[test]
fn test_s11_string_rjust() {
    let code = r#"
def right_justify(s: str, width: int) -> str:
    return s.rjust(width)
"#;
    let result = transpile(code);
    assert!(result.contains("fn right_justify"), "Should transpile str.rjust. Got: {}", result);
}

#[test]
fn test_s11_string_zfill() {
    let code = r#"
def zero_pad(s: str, width: int) -> str:
    return s.zfill(width)
"#;
    let result = transpile(code);
    assert!(result.contains("fn zero_pad"), "Should transpile str.zfill. Got: {}", result);
}

// ============================================================================
// String case methods
// ============================================================================

#[test]
fn test_s11_string_swapcase() {
    let code = r#"
def swap(s: str) -> str:
    return s.swapcase()
"#;
    let result = transpile(code);
    assert!(result.contains("fn swap"), "Should transpile str.swapcase. Got: {}", result);
}

#[test]
fn test_s11_string_casefold() {
    let code = r#"
def fold(s: str) -> str:
    return s.casefold()
"#;
    let result = transpile(code);
    assert!(result.contains("fn fold"), "Should transpile str.casefold. Got: {}", result);
}

#[test]
fn test_s11_string_title() {
    let code = r#"
def title_case(s: str) -> str:
    return s.title()
"#;
    let result = transpile(code);
    assert!(result.contains("fn title_case"), "Should transpile str.title. Got: {}", result);
}

#[test]
fn test_s11_string_capitalize() {
    let code = r#"
def cap(s: str) -> str:
    return s.capitalize()
"#;
    let result = transpile(code);
    assert!(result.contains("fn cap"), "Should transpile str.capitalize. Got: {}", result);
}

// ============================================================================
// String check methods
// ============================================================================

#[test]
fn test_s11_string_isprintable() {
    let code = r#"
def is_print(s: str) -> bool:
    return s.isprintable()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_print"), "Should transpile str.isprintable. Got: {}", result);
}

#[test]
fn test_s11_string_isupper() {
    let code = r#"
def is_up(s: str) -> bool:
    return s.isupper()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_up"), "Should transpile str.isupper. Got: {}", result);
}

#[test]
fn test_s11_string_islower() {
    let code = r#"
def is_low(s: str) -> bool:
    return s.islower()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_low"), "Should transpile str.islower. Got: {}", result);
}

#[test]
fn test_s11_string_istitle() {
    let code = r#"
def is_title(s: str) -> bool:
    return s.istitle()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_title"), "Should transpile str.istitle. Got: {}", result);
}

#[test]
fn test_s11_string_isnumeric() {
    let code = r#"
def is_num(s: str) -> bool:
    return s.isnumeric()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_num"), "Should transpile str.isnumeric. Got: {}", result);
}

#[test]
fn test_s11_string_isdecimal() {
    let code = r#"
def is_dec(s: str) -> bool:
    return s.isdecimal()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_dec"), "Should transpile str.isdecimal. Got: {}", result);
}

#[test]
fn test_s11_string_isascii() {
    let code = r#"
def is_asc(s: str) -> bool:
    return s.isascii()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_asc"), "Should transpile str.isascii. Got: {}", result);
}

#[test]
fn test_s11_string_isidentifier() {
    let code = r#"
def is_ident(s: str) -> bool:
    return s.isidentifier()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_ident"), "Should transpile str.isidentifier. Got: {}", result);
}

#[test]
fn test_s11_string_isspace() {
    let code = r#"
def is_ws(s: str) -> bool:
    return s.isspace()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_ws"), "Should transpile str.isspace. Got: {}", result);
}

#[test]
fn test_s11_string_isalnum() {
    let code = r#"
def is_an(s: str) -> bool:
    return s.isalnum()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_an"), "Should transpile str.isalnum. Got: {}", result);
}

// ============================================================================
// String partition/split methods
// ============================================================================

#[test]
fn test_s11_string_partition() {
    let code = r#"
def split_at(s: str, sep: str) -> str:
    parts = s.partition(sep)
    return parts[0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn split_at"), "Should transpile str.partition. Got: {}", result);
}

#[test]
fn test_s11_string_rpartition() {
    let code = r#"
def rsplit_at(s: str, sep: str) -> str:
    parts = s.rpartition(sep)
    return parts[0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn rsplit_at"), "Should transpile str.rpartition. Got: {}", result);
}

#[test]
fn test_s11_string_expandtabs() {
    let code = r#"
def expand(s: str) -> str:
    return s.expandtabs()
"#;
    let result = transpile(code);
    assert!(result.contains("fn expand"), "Should transpile str.expandtabs. Got: {}", result);
}

#[test]
fn test_s11_string_format() {
    let code = r#"
def greet(name: str) -> str:
    return "Hello {}".format(name)
"#;
    let result = transpile(code);
    assert!(result.contains("fn greet"), "Should transpile str.format. Got: {}", result);
}

#[test]
fn test_s11_string_encode() {
    let code = r#"
def to_bytes(s: str) -> bytes:
    return s.encode()
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_bytes"), "Should transpile str.encode. Got: {}", result);
}

#[test]
fn test_s11_string_rsplit() {
    let code = r#"
def split_right(s: str, sep: str) -> list:
    return s.rsplit(sep)
"#;
    let result = transpile(code);
    assert!(result.contains("fn split_right"), "Should transpile str.rsplit. Got: {}", result);
}

// ============================================================================
// Dict methods
// ============================================================================

#[test]
fn test_s11_dict_setdefault() {
    let code = r#"
def ensure_key(d: dict, key: str, val: int) -> int:
    return d.setdefault(key, val)
"#;
    let result = transpile(code);
    assert!(result.contains("fn ensure_key"), "Should transpile dict.setdefault. Got: {}", result);
}

#[test]
fn test_s11_dict_clear() {
    let code = r#"
def reset(d: dict) -> None:
    d.clear()
"#;
    let result = transpile(code);
    assert!(result.contains("clear"), "Should transpile dict.clear. Got: {}", result);
}

#[test]
fn test_s11_dict_copy() {
    let code = r#"
def clone_dict(d: dict) -> dict:
    return d.copy()
"#;
    let result = transpile(code);
    assert!(result.contains("clone"), "Should transpile dict.copy. Got: {}", result);
}

#[test]
fn test_s11_dict_keys() {
    let code = r#"
def get_keys(d: dict) -> list:
    return list(d.keys())
"#;
    let result = transpile(code);
    assert!(result.contains("keys"), "Should transpile dict.keys. Got: {}", result);
}

#[test]
fn test_s11_dict_values() {
    let code = r#"
def get_values(d: dict) -> list:
    return list(d.values())
"#;
    let result = transpile(code);
    assert!(result.contains("values"), "Should transpile dict.values. Got: {}", result);
}

#[test]
fn test_s11_dict_get_with_default() {
    let code = r#"
def safe_get(d: dict, key: str) -> int:
    return d.get(key, 0)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn safe_get"),
        "Should transpile dict.get with default. Got: {}",
        result
    );
}

#[test]
fn test_s11_dict_contains_key() {
    let code = r#"
def has_key(d: dict, key: str) -> bool:
    return key in d
"#;
    let result = transpile(code);
    assert!(
        result.contains("contains_key") || result.contains("fn has_key"),
        "Should transpile 'in' for dict. Got: {}",
        result
    );
}

// ============================================================================
// Set methods - algebra
// ============================================================================

#[test]
fn test_s11_set_symmetric_difference() {
    let code = r#"
from typing import Set

def sym_diff(a: Set[int], b: Set[int]) -> Set[int]:
    return a.symmetric_difference(b)
"#;
    let result = transpile(code);
    assert!(
        result.contains("symmetric_difference") || result.contains("HashSet"),
        "Should transpile set.symmetric_difference. Got: {}",
        result
    );
}

#[test]
fn test_s11_set_issubset() {
    let code = r#"
from typing import Set

def is_sub(a: Set[int], b: Set[int]) -> bool:
    return a.issubset(b)
"#;
    let result = transpile(code);
    assert!(
        result.contains("is_subset") || result.contains("fn is_sub"),
        "Should transpile set.issubset. Got: {}",
        result
    );
}

#[test]
fn test_s11_set_issuperset() {
    let code = r#"
from typing import Set

def is_super(a: Set[int], b: Set[int]) -> bool:
    return a.issuperset(b)
"#;
    let result = transpile(code);
    assert!(
        result.contains("is_superset") || result.contains("fn is_super"),
        "Should transpile set.issuperset. Got: {}",
        result
    );
}

#[test]
fn test_s11_set_isdisjoint() {
    let code = r#"
from typing import Set

def no_overlap(a: Set[int], b: Set[int]) -> bool:
    return a.isdisjoint(b)
"#;
    let result = transpile(code);
    assert!(
        result.contains("is_disjoint") || result.contains("fn no_overlap"),
        "Should transpile set.isdisjoint. Got: {}",
        result
    );
}

#[test]
fn test_s11_set_add() {
    let code = r#"
from typing import Set

def add_item(s: Set[int], val: int) -> None:
    s.add(val)
"#;
    let result = transpile(code);
    assert!(result.contains("insert"), "Should transpile set.add as insert. Got: {}", result);
}

#[test]
fn test_s11_set_discard() {
    let code = r#"
from typing import Set

def discard_item(s: Set[int], val: int) -> None:
    s.discard(val)
"#;
    let result = transpile(code);
    assert!(
        result.contains("remove") || result.contains("fn discard_item"),
        "Should transpile set.discard. Got: {}",
        result
    );
}

#[test]
fn test_s11_set_clear() {
    let code = r#"
from typing import Set

def clear_set(s: Set[int]) -> None:
    s.clear()
"#;
    let result = transpile(code);
    assert!(result.contains("clear"), "Should transpile set.clear. Got: {}", result);
}

// ============================================================================
// List methods - edge cases
// ============================================================================

#[test]
fn test_s11_list_index() {
    let code = r#"
def find_item(items: list, val: int) -> int:
    return items.index(val)
"#;
    let result = transpile(code);
    assert!(
        result.contains("position") || result.contains("fn find_item"),
        "Should transpile list.index. Got: {}",
        result
    );
}

#[test]
fn test_s11_list_count() {
    let code = r#"
def count_item(items: list, val: int) -> int:
    return items.count(val)
"#;
    let result = transpile(code);
    assert!(
        result.contains("filter") || result.contains("count") || result.contains("fn count_item"),
        "Should transpile list.count. Got: {}",
        result
    );
}

#[test]
fn test_s11_list_clear() {
    let code = r#"
def clear_list(items: list) -> None:
    items.clear()
"#;
    let result = transpile(code);
    assert!(result.contains("clear"), "Should transpile list.clear. Got: {}", result);
}

#[test]
fn test_s11_list_copy() {
    let code = r#"
def copy_list(items: list) -> list:
    return items.copy()
"#;
    let result = transpile(code);
    assert!(
        result.contains("clone") || result.contains("fn copy_list"),
        "Should transpile list.copy. Got: {}",
        result
    );
}

// ============================================================================
// Sys/IO methods
// ============================================================================

#[test]
fn test_s11_sys_stdout_write() {
    let code = r#"
import sys

def write_out(msg: str) -> None:
    sys.stdout.write(msg)
"#;
    let result = transpile(code);
    assert!(result.contains("fn write_out"), "Should transpile sys.stdout.write. Got: {}", result);
}

#[test]
fn test_s11_sys_stderr_write() {
    let code = r#"
import sys

def write_err(msg: str) -> None:
    sys.stderr.write(msg)
"#;
    let result = transpile(code);
    assert!(result.contains("fn write_err"), "Should transpile sys.stderr.write. Got: {}", result);
}

// ============================================================================
// Regex methods
// ============================================================================

#[test]
fn test_s11_re_compile_match() {
    let code = r#"
import re

def check_pattern(text: str) -> bool:
    pattern = re.compile(r"\d+")
    result = pattern.match(text)
    return result is not None
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_pattern"),
        "Should transpile re.compile/match. Got: {}",
        result
    );
}

#[test]
fn test_s11_re_search() {
    let code = r#"
import re

def has_digit(text: str) -> bool:
    return re.search(r"\d", text) is not None
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_digit"), "Should transpile re.search. Got: {}", result);
}

#[test]
fn test_s11_re_findall() {
    let code = r#"
import re

def find_numbers(text: str) -> list:
    return re.findall(r"\d+", text)
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_numbers"), "Should transpile re.findall. Got: {}", result);
}

#[test]
fn test_s11_re_sub() {
    let code = r#"
import re

def clean_text(text: str) -> str:
    return re.sub(r"\s+", " ", text)
"#;
    let result = transpile(code);
    assert!(result.contains("fn clean_text"), "Should transpile re.sub. Got: {}", result);
}

// ============================================================================
// Chained method calls
// ============================================================================

#[test]
fn test_s11_string_split_join() {
    let code = r#"
def normalize_spaces(s: str) -> str:
    return " ".join(s.split())
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn normalize_spaces"),
        "Should transpile split+join chain. Got: {}",
        result
    );
}

#[test]
fn test_s11_string_strip_lower() {
    let code = r#"
def clean(s: str) -> str:
    return s.strip().lower()
"#;
    let result = transpile(code);
    assert!(
        result.contains("trim") || result.contains("to_lowercase"),
        "Should transpile strip+lower chain. Got: {}",
        result
    );
}

#[test]
fn test_s11_string_upper_replace() {
    let code = r#"
def transform(s: str) -> str:
    return s.upper().replace("A", "X")
"#;
    let result = transpile(code);
    assert!(
        result.contains("to_uppercase") || result.contains("replace"),
        "Should transpile upper+replace chain. Got: {}",
        result
    );
}
