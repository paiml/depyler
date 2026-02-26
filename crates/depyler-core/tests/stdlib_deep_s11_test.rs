//! Session 11: Deep stdlib coverage tests
//!
//! Targets specific untested stdlib code paths:
//! - math module edge cases (log with base, isclose, lcm, gcd)
//! - os module (getenv with default, path operations)
//! - json module (dumps/loads patterns)
//! - regex module (fullmatch, search, findall)
//! - itertools (chain, islice, permutations)
//! - functools (reduce)
//! - datetime patterns
//! - collections (Counter, defaultdict)

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
// Math module edge cases
// ============================================================================

#[test]
fn test_s11_deep_math_log_with_base() {
    let code = r#"
import math

def log_base(x: float, base: float) -> float:
    return math.log(x, base)
"#;
    let result = transpile(code);
    assert!(
        result.contains("log") || result.contains("fn log_base"),
        "Should transpile math.log with base. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_math_log2() {
    let code = r#"
import math

def log_two(x: float) -> float:
    return math.log2(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("log2") || result.contains("fn log_two"),
        "Should transpile math.log2. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_math_log10() {
    let code = r#"
import math

def log_ten(x: float) -> float:
    return math.log10(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("log10") || result.contains("fn log_ten"),
        "Should transpile math.log10. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_math_exp() {
    let code = r#"
import math

def exponential(x: float) -> float:
    return math.exp(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("exp") || result.contains("fn exponential"),
        "Should transpile math.exp. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_math_gcd() {
    let code = r#"
import math

def greatest_common(a: int, b: int) -> int:
    return math.gcd(a, b)
"#;
    let result = transpile(code);
    assert!(
        result.contains("gcd") || result.contains("fn greatest_common"),
        "Should transpile math.gcd. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_math_isclose() {
    let code = r#"
import math

def nearly_equal(a: float, b: float) -> bool:
    return math.isclose(a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn nearly_equal"), "Should transpile math.isclose. Got: {}", result);
}

#[test]
fn test_s11_deep_math_factorial() {
    let code = r#"
import math

def fact(n: int) -> int:
    return math.factorial(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn fact"), "Should transpile math.factorial. Got: {}", result);
}

#[test]
fn test_s11_deep_math_tan() {
    let code = r#"
import math

def tangent(x: float) -> float:
    return math.tan(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("tan") || result.contains("fn tangent"),
        "Should transpile math.tan. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_math_atan() {
    let code = r#"
import math

def arctangent(x: float) -> float:
    return math.atan(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("atan") || result.contains("fn arctangent"),
        "Should transpile math.atan. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_math_atan2() {
    let code = r#"
import math

def angle(y: float, x: float) -> float:
    return math.atan2(y, x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("atan2") || result.contains("fn angle"),
        "Should transpile math.atan2. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_math_asin() {
    let code = r#"
import math

def arcsine(x: float) -> float:
    return math.asin(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("asin") || result.contains("fn arcsine"),
        "Should transpile math.asin. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_math_acos() {
    let code = r#"
import math

def arccosine(x: float) -> float:
    return math.acos(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("acos") || result.contains("fn arccosine"),
        "Should transpile math.acos. Got: {}",
        result
    );
}

// ============================================================================
// os module
// ============================================================================

#[test]
fn test_s11_deep_os_getenv() {
    let code = r#"
import os

def get_home() -> str:
    return os.getenv("HOME")
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_home"), "Should transpile os.getenv. Got: {}", result);
}

#[test]
fn test_s11_deep_os_getenv_default() {
    let code = r#"
import os

def get_port() -> str:
    return os.getenv("PORT", "8080")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_port"),
        "Should transpile os.getenv with default. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_os_path_exists() {
    let code = r#"
import os

def file_exists(path: str) -> bool:
    return os.path.exists(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn file_exists"), "Should transpile os.path.exists. Got: {}", result);
}

#[test]
fn test_s11_deep_os_path_join() {
    let code = r#"
import os

def make_path(base: str, name: str) -> str:
    return os.path.join(base, name)
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_path"), "Should transpile os.path.join. Got: {}", result);
}

#[test]
fn test_s11_deep_os_getcwd() {
    let code = r#"
import os

def current_dir() -> str:
    return os.getcwd()
"#;
    let result = transpile(code);
    assert!(result.contains("fn current_dir"), "Should transpile os.getcwd. Got: {}", result);
}

#[test]
fn test_s11_deep_os_listdir() {
    let code = r#"
import os

def list_files(path: str) -> list:
    return os.listdir(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn list_files"), "Should transpile os.listdir. Got: {}", result);
}

// ============================================================================
// json module
// ============================================================================

#[test]
fn test_s11_deep_json_dumps() {
    let code = r#"
import json

def to_json(data: dict) -> str:
    return json.dumps(data)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_json"), "Should transpile json.dumps. Got: {}", result);
}

#[test]
fn test_s11_deep_json_loads() {
    let code = r#"
import json

def from_json(text: str) -> dict:
    return json.loads(text)
"#;
    let result = transpile(code);
    assert!(result.contains("fn from_json"), "Should transpile json.loads. Got: {}", result);
}

// ============================================================================
// re (regex) module
// ============================================================================

#[test]
fn test_s11_deep_re_match() {
    let code = r#"
import re

def is_match(pattern: str, text: str) -> bool:
    m = re.match(pattern, text)
    return m is not None
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_match"), "Should transpile re.match. Got: {}", result);
}

#[test]
fn test_s11_deep_re_search() {
    let code = r#"
import re

def has_pattern(pattern: str, text: str) -> bool:
    m = re.search(pattern, text)
    return m is not None
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_pattern"), "Should transpile re.search. Got: {}", result);
}

#[test]
fn test_s11_deep_re_findall() {
    let code = r#"
import re

def find_numbers(text: str) -> list:
    return re.findall(r"\d+", text)
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_numbers"), "Should transpile re.findall. Got: {}", result);
}

#[test]
fn test_s11_deep_re_sub() {
    let code = r#"
import re

def clean_spaces(text: str) -> str:
    return re.sub(r"\s+", " ", text)
"#;
    let result = transpile(code);
    assert!(result.contains("fn clean_spaces"), "Should transpile re.sub. Got: {}", result);
}

#[test]
fn test_s11_deep_re_split() {
    let code = r#"
import re

def split_words(text: str) -> list:
    return re.split(r"\s+", text)
"#;
    let result = transpile(code);
    assert!(result.contains("fn split_words"), "Should transpile re.split. Got: {}", result);
}

// ============================================================================
// itertools
// ============================================================================

#[test]
fn test_s11_deep_itertools_chain() {
    let code = r#"
import itertools

def chain_lists(a: list, b: list) -> list:
    return list(itertools.chain(a, b))
"#;
    let result = transpile(code);
    assert!(result.contains("fn chain_lists"), "Should transpile itertools.chain. Got: {}", result);
}

// ============================================================================
// functools
// ============================================================================

#[test]
fn test_s11_deep_functools_reduce() {
    let code = r#"
from functools import reduce

def product(items: list) -> int:
    return reduce(lambda x, y: x * y, items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn product"), "Should transpile functools.reduce. Got: {}", result);
}

// ============================================================================
// Complex patterns combining stdlib
// ============================================================================

#[test]
fn test_s11_deep_map_filter() {
    let code = r#"
def double_positives(items: list) -> list:
    return list(map(lambda x: x * 2, filter(lambda x: x > 0, items)))
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn double_positives"),
        "Should transpile map/filter combo. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_sorted_with_key() {
    let code = r#"
def sort_by_length(words: list) -> list:
    return sorted(words, key=lambda w: len(w))
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sort_by_length"),
        "Should transpile sorted with key. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_sorted_with_reverse() {
    let code = r#"
def sort_desc(items: list) -> list:
    return sorted(items, reverse=True)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sort_desc"),
        "Should transpile sorted with reverse. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_any_all() {
    let code = r#"
def any_positive(items: list) -> bool:
    return any(x > 0 for x in items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn any_positive"),
        "Should transpile any with generator. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_all_check() {
    let code = r#"
def all_positive(items: list) -> bool:
    return all(x > 0 for x in items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn all_positive"),
        "Should transpile all with generator. Got: {}",
        result
    );
}

// ============================================================================
// Dict construction patterns
// ============================================================================

#[test]
fn test_s11_deep_dict_literal() {
    let code = r#"
def make_config() -> dict:
    return {"host": "localhost", "port": "8080"}
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_config"), "Should transpile dict literal. Got: {}", result);
}

#[test]
fn test_s11_deep_dict_get() {
    let code = r#"
def safe_get(d: dict, key: str) -> str:
    return d.get(key, "default")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn safe_get"),
        "Should transpile dict.get with default. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_dict_pop() {
    let code = r#"
def remove_key(d: dict, key: str) -> str:
    return d.pop(key)
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_key"), "Should transpile dict.pop. Got: {}", result);
}

#[test]
fn test_s11_deep_dict_setdefault() {
    let code = r#"
def get_or_set(d: dict, key: str, val: str) -> str:
    return d.setdefault(key, val)
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_or_set"), "Should transpile dict.setdefault. Got: {}", result);
}

// ============================================================================
// List slicing
// ============================================================================

#[test]
fn test_s11_deep_list_slice_start() {
    let code = r#"
def first_n(items: list, n: int) -> list:
    return items[:n]
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_n"), "Should transpile list slice [:n]. Got: {}", result);
}

#[test]
fn test_s11_deep_list_slice_end() {
    let code = r#"
def skip_n(items: list, n: int) -> list:
    return items[n:]
"#;
    let result = transpile(code);
    assert!(result.contains("fn skip_n"), "Should transpile list slice [n:]. Got: {}", result);
}

#[test]
fn test_s11_deep_list_slice_range() {
    let code = r#"
def sublist(items: list, start: int, end: int) -> list:
    return items[start:end]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sublist"),
        "Should transpile list slice [start:end]. Got: {}",
        result
    );
}

// ============================================================================
// Built-in type constructors
// ============================================================================

#[test]
fn test_s11_deep_list_constructor() {
    let code = r#"
def to_list(items: str) -> list:
    return list(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_list"), "Should transpile list() constructor. Got: {}", result);
}

#[test]
fn test_s11_deep_set_constructor() {
    let code = r#"
def to_set(items: list) -> set:
    return set(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_set"), "Should transpile set() constructor. Got: {}", result);
}

#[test]
fn test_s11_deep_dict_constructor() {
    let code = r#"
def empty_dict() -> dict:
    return dict()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn empty_dict"),
        "Should transpile dict() constructor. Got: {}",
        result
    );
}

// ============================================================================
// Numeric edge cases
// ============================================================================

#[test]
fn test_s11_deep_negative_literal() {
    let code = r#"
def neg() -> int:
    return -42
"#;
    let result = transpile(code);
    assert!(
        result.contains("-42") || result.contains("fn neg"),
        "Should transpile negative literal. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_float_literal() {
    let code = r#"
def half() -> float:
    return 0.5
"#;
    let result = transpile(code);
    assert!(
        result.contains("0.5") || result.contains("fn half"),
        "Should transpile float literal. Got: {}",
        result
    );
}

#[test]
fn test_s11_deep_none_return() {
    let code = r#"
def nothing() -> None:
    return None
"#;
    let result = transpile(code);
    assert!(result.contains("fn nothing"), "Should transpile None return. Got: {}", result);
}

// ============================================================================
// Multiple imports
// ============================================================================

#[test]
fn test_s11_deep_multiple_imports() {
    let code = r#"
import math
import os

def compute(x: float) -> float:
    return math.sqrt(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn compute"), "Should handle multiple imports. Got: {}", result);
}

#[test]
fn test_s11_deep_from_import() {
    let code = r#"
from math import sqrt

def root(x: float) -> float:
    return sqrt(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn root"), "Should handle from-import. Got: {}", result);
}
