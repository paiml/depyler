//! Session 11: Regex compiled methods and sys.io stream coverage tests
//!
//! Targets uncovered paths in:
//! - convert_regex_method: compiled regex findall, match, search, group, groups
//! - convert_sys_io_method: stdout/stderr write, stdin read/readline
//! - Complex regex patterns through direct rules
//! - sys module attribute access

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
// Compiled regex method patterns
// ============================================================================

#[test]
fn test_s11_regex_compiled_findall() {
    let code = r#"
import re

def find_words(text: str) -> list:
    pattern = re.compile(r"\w+")
    return pattern.findall(text)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn find_words"),
        "Should transpile compiled findall. Got: {}",
        result
    );
}

#[test]
fn test_s11_regex_compiled_match() {
    let code = r#"
import re

def check_start(text: str) -> bool:
    pattern = re.compile(r"\d+")
    return pattern.match(text) is not None
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_start"),
        "Should transpile compiled match. Got: {}",
        result
    );
}

#[test]
fn test_s11_regex_compiled_search() {
    let code = r#"
import re

def find_number(text: str) -> bool:
    pattern = re.compile(r"\d+")
    return pattern.search(text) is not None
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn find_number"),
        "Should transpile compiled search. Got: {}",
        result
    );
}

#[test]
fn test_s11_regex_compiled_sub() {
    let code = r#"
import re

def clean_whitespace(text: str) -> str:
    pattern = re.compile(r"\s+")
    return pattern.sub(" ", text)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn clean_whitespace"),
        "Should transpile compiled sub. Got: {}",
        result
    );
}

#[test]
fn test_s11_regex_compiled_split() {
    let code = r#"
import re

def split_on_commas(text: str) -> list:
    pattern = re.compile(r",\s*")
    return pattern.split(text)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn split_on_commas"),
        "Should transpile compiled split. Got: {}",
        result
    );
}

// ============================================================================
// Sys module patterns
// ============================================================================

#[test]
fn test_s11_sys_stdout_write() {
    let code = r#"
import sys

def write_out(msg: str):
    sys.stdout.write(msg)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn write_out"),
        "Should transpile sys.stdout.write. Got: {}",
        result
    );
}

#[test]
fn test_s11_sys_stderr_write() {
    let code = r#"
import sys

def write_err(msg: str):
    sys.stderr.write(msg)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn write_err"),
        "Should transpile sys.stderr.write. Got: {}",
        result
    );
}

#[test]
fn test_s11_sys_stdout_flush() {
    let code = r#"
import sys

def flush():
    sys.stdout.flush()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn flush"),
        "Should transpile sys.stdout.flush. Got: {}",
        result
    );
}

#[test]
fn test_s11_sys_stdin_readline() {
    let code = r#"
import sys

def read_line() -> str:
    return sys.stdin.readline()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn read_line"),
        "Should transpile sys.stdin.readline. Got: {}",
        result
    );
}

#[test]
fn test_s11_sys_stdin_readlines() {
    let code = r#"
import sys

def read_all_lines() -> list:
    return sys.stdin.readlines()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn read_all_lines"),
        "Should transpile sys.stdin.readlines. Got: {}",
        result
    );
}

#[test]
fn test_s11_sys_argv() {
    let code = r#"
import sys

def get_args() -> list:
    return sys.argv
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_args"),
        "Should transpile sys.argv. Got: {}",
        result
    );
}

#[test]
fn test_s11_sys_exit() {
    let code = r#"
import sys

def quit(code: int):
    sys.exit(code)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn quit"),
        "Should transpile sys.exit. Got: {}",
        result
    );
}

// ============================================================================
// Complex regex usage patterns (exercise direct_rules regex paths)
// ============================================================================

#[test]
fn test_s11_regex_email_validator() {
    let code = r#"
import re

def is_valid_email(email: str) -> bool:
    pattern = r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$"
    return re.match(pattern, email) is not None
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn is_valid_email"),
        "Should transpile email validator regex. Got: {}",
        result
    );
}

#[test]
fn test_s11_regex_replace_multiple() {
    let code = r#"
import re

def normalize_spaces(text: str) -> str:
    return re.sub(r"\s+", " ", text).strip()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn normalize_spaces"),
        "Should transpile regex replace chain. Got: {}",
        result
    );
}

#[test]
fn test_s11_regex_extract_numbers() {
    let code = r#"
import re

def extract_ints(text: str) -> list:
    return [int(n) for n in re.findall(r"\d+", text)]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn extract_ints"),
        "Should transpile regex extract with conversion. Got: {}",
        result
    );
}

// ============================================================================
// Complex sys patterns
// ============================================================================

#[test]
fn test_s11_sys_platform() {
    let code = r#"
import sys

def get_platform() -> str:
    return sys.platform
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_platform"),
        "Should transpile sys.platform. Got: {}",
        result
    );
}

#[test]
fn test_s11_sys_stderr_flush() {
    let code = r#"
import sys

def flush_err():
    sys.stderr.flush()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn flush_err"),
        "Should transpile sys.stderr.flush. Got: {}",
        result
    );
}

// ============================================================================
// String methods through convert_string_method paths
// ============================================================================

#[test]
fn test_s11_str_method_index() {
    let code = r#"
def find_must(s: str, sub: str) -> int:
    return s.index(sub)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn find_must"),
        "Should transpile str.index. Got: {}",
        result
    );
}

#[test]
fn test_s11_str_method_rindex() {
    let code = r#"
def find_last_must(s: str, sub: str) -> int:
    return s.rindex(sub)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn find_last_must"),
        "Should transpile str.rindex. Got: {}",
        result
    );
}

#[test]
fn test_s11_str_method_center_no_fill() {
    let code = r#"
def center_text(s: str) -> str:
    return s.center(40)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn center_text"),
        "Should transpile center without fillchar. Got: {}",
        result
    );
}

#[test]
fn test_s11_str_method_ljust_no_fill() {
    let code = r#"
def left_pad(s: str) -> str:
    return s.ljust(40)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn left_pad"),
        "Should transpile ljust without fillchar. Got: {}",
        result
    );
}

#[test]
fn test_s11_str_method_rjust_no_fill() {
    let code = r#"
def right_pad(s: str) -> str:
    return s.rjust(40)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn right_pad"),
        "Should transpile rjust without fillchar. Got: {}",
        result
    );
}

// ============================================================================
// List method edge cases
// ============================================================================

#[test]
fn test_s11_list_sort_no_args() {
    let code = r#"
def sort_in_place(items: list) -> list:
    items.sort()
    return items
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sort_in_place"),
        "Should transpile sort(). Got: {}",
        result
    );
}

#[test]
fn test_s11_list_pop_no_arg() {
    let code = r#"
def pop_last(items: list) -> int:
    return items.pop()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn pop_last"),
        "Should transpile pop(). Got: {}",
        result
    );
}

#[test]
fn test_s11_list_index_method() {
    let code = r#"
def find_index(items: list, val: int) -> int:
    return items.index(val)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn find_index"),
        "Should transpile list.index(). Got: {}",
        result
    );
}

#[test]
fn test_s11_list_count_method() {
    let code = r#"
def count_val(items: list, val: int) -> int:
    return items.count(val)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn count_val"),
        "Should transpile list.count(). Got: {}",
        result
    );
}

#[test]
fn test_s11_list_copy_method() {
    let code = r#"
def copy_list(items: list) -> list:
    return items.copy()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn copy_list"),
        "Should transpile list.copy(). Got: {}",
        result
    );
}

// ============================================================================
// Dict method edge cases
// ============================================================================

#[test]
fn test_s11_dict_clear() {
    let code = r#"
def clear_dict(d: dict) -> dict:
    d.clear()
    return d
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn clear_dict"),
        "Should transpile dict.clear(). Got: {}",
        result
    );
}

#[test]
fn test_s11_dict_copy() {
    let code = r#"
def copy_dict(d: dict) -> dict:
    return d.copy()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn copy_dict"),
        "Should transpile dict.copy(). Got: {}",
        result
    );
}

#[test]
fn test_s11_dict_get_no_default() {
    let code = r#"
def maybe_get(d: dict, key: str):
    return d.get(key)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn maybe_get"),
        "Should transpile dict.get without default. Got: {}",
        result
    );
}

#[test]
fn test_s11_dict_pop_no_default() {
    let code = r#"
def take(d: dict, key: str) -> int:
    return d.pop(key)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn take"),
        "Should transpile dict.pop without default. Got: {}",
        result
    );
}

// ============================================================================
// Set method edge cases
// ============================================================================

#[test]
fn test_s11_set_discard() {
    let code = r#"
def safe_remove(s: set, val: int) -> set:
    s.discard(val)
    return s
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn safe_remove"),
        "Should transpile set.discard(). Got: {}",
        result
    );
}

#[test]
fn test_s11_set_clear() {
    let code = r#"
def clear_set(s: set) -> set:
    s.clear()
    return s
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn clear_set"),
        "Should transpile set.clear(). Got: {}",
        result
    );
}

#[test]
fn test_s11_set_copy() {
    let code = r#"
def copy_set(s: set) -> set:
    return s.copy()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn copy_set"),
        "Should transpile set.copy(). Got: {}",
        result
    );
}

#[test]
fn test_s11_set_pop() {
    let code = r#"
def pop_any(s: set) -> int:
    return s.pop()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn pop_any"),
        "Should transpile set.pop(). Got: {}",
        result
    );
}

#[test]
fn test_s11_set_update() {
    let code = r#"
def merge_sets(a: set, b: set) -> set:
    a.update(b)
    return a
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn merge_sets"),
        "Should transpile set.update(). Got: {}",
        result
    );
}
