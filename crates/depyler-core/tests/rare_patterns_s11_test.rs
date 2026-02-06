//! Session 11: Rare pattern coverage tests
//!
//! Targets extremely specific code paths that are rarely exercised:
//! - Complex class hierarchies with multiple dunder methods
//! - Subprocess patterns
//! - Complex list/dict/set comprehension combinations
//! - Multiple return type patterns
//! - Complex decorator patterns
//! - Advanced string formatting
//! - Collection algebra operations
//! - Exception handling with chained exceptions
//! - File I/O patterns with various modes
//! - Itertools patterns

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
// Complex class patterns with dunder methods
// ============================================================================

#[test]
fn test_s11_rare_class_add_operator() {
    let code = r#"
class Vector:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y

    def __add__(self, other):
        return Vector(self.x + other.x, self.y + other.y)

    def __sub__(self, other):
        return Vector(self.x - other.x, self.y - other.y)

    def __mul__(self, scalar: float):
        return Vector(self.x * scalar, self.y * scalar)
"#;
    let result = transpile(code);
    assert!(
        result.contains("Vector"),
        "Should transpile class with operator overloading. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_class_contains() {
    let code = r#"
class Bag:
    def __init__(self):
        self.items: list = []

    def __contains__(self, item: int) -> bool:
        return item in self.items

    def add(self, item: int):
        self.items.append(item)
"#;
    let result = transpile(code);
    assert!(
        result.contains("Bag"),
        "Should transpile class with __contains__. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_class_iter() {
    let code = r#"
class Range:
    def __init__(self, start: int, end: int):
        self.start = start
        self.end = end
        self.current = start

    def __iter__(self):
        return self

    def __next__(self) -> int:
        if self.current >= self.end:
            raise StopIteration()
        val = self.current
        self.current += 1
        return val
"#;
    let result = transpile(code);
    assert!(
        result.contains("Range"),
        "Should transpile iterator class. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_class_getitem() {
    let code = r#"
class Matrix:
    def __init__(self, rows: int, cols: int):
        self.rows = rows
        self.cols = cols
        self.data: list = []

    def __getitem__(self, idx: int) -> list:
        return self.data[idx]

    def __setitem__(self, idx: int, value: list):
        self.data[idx] = value
"#;
    let result = transpile(code);
    assert!(
        result.contains("Matrix"),
        "Should transpile class with __getitem__. Got: {}",
        result
    );
}

// ============================================================================
// Complex comprehension patterns
// ============================================================================

#[test]
fn test_s11_rare_comprehension_nested_with_filter() {
    let code = r#"
def flat_evens(matrix: list) -> list:
    return [x for row in matrix for x in row if x % 2 == 0]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn flat_evens"),
        "Should transpile nested comp with filter. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_dict_comprehension_filter() {
    let code = r#"
def positive_only(d: dict) -> dict:
    return {k: v for k, v in d.items() if v > 0}
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn positive_only"),
        "Should transpile dict comp with filter. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_set_comprehension_complex() {
    let code = r#"
def unique_lengths(words: list) -> set:
    return {len(w) for w in words if len(w) > 2}
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn unique_lengths"),
        "Should transpile set comp with filter. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_list_comp_with_method() {
    let code = r#"
def upper_words(text: str) -> list:
    return [w.upper() for w in text.split()]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn upper_words"),
        "Should transpile list comp with method call. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_list_comp_with_ternary() {
    let code = r#"
def abs_list(items: list) -> list:
    return [x if x >= 0 else -x for x in items]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn abs_list"),
        "Should transpile list comp with ternary. Got: {}",
        result
    );
}

// ============================================================================
// Complex exception patterns
// ============================================================================

#[test]
fn test_s11_rare_try_except_return_in_finally() {
    let code = r#"
def safe_read(path: str) -> str:
    result = ""
    try:
        with open(path) as f:
            result = f.read()
    except FileNotFoundError:
        result = "not found"
    except PermissionError:
        result = "no access"
    finally:
        print("attempted read")
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn safe_read"),
        "Should transpile complex try/except/finally. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_nested_try() {
    let code = r#"
def nested_parse(a: str, b: str) -> int:
    try:
        x = int(a)
        try:
            y = int(b)
            return x + y
        except ValueError:
            return x
    except ValueError:
        return 0
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn nested_parse"),
        "Should transpile nested try. Got: {}",
        result
    );
}

// ============================================================================
// Subprocess patterns
// ============================================================================

#[test]
fn test_s11_rare_subprocess_run() {
    let code = r#"
import subprocess

def run_cmd(cmd: str) -> str:
    result = subprocess.run(cmd, capture_output=True, text=True)
    return result.stdout
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn run_cmd"),
        "Should transpile subprocess.run. Got: {}",
        result
    );
}

// ============================================================================
// Complex string operations
// ============================================================================

#[test]
fn test_s11_rare_multiline_string() {
    let code = r#"
def template(name: str, items: list) -> str:
    header = f"Report for {name}"
    body = "\n".join(items)
    return header + "\n" + body
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn template"),
        "Should transpile multiline string building. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_string_methods_chain() {
    let code = r#"
def clean(s: str) -> str:
    return s.strip().lower().replace(" ", "_")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn clean"),
        "Should transpile chained string methods. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_string_translate() {
    let code = r#"
def remove_punctuation(s: str) -> str:
    table = str.maketrans("", "", ".,!?;:")
    return s.translate(table)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn remove_punctuation"),
        "Should transpile string translate. Got: {}",
        result
    );
}

// ============================================================================
// Advanced math operations
// ============================================================================

#[test]
fn test_s11_rare_math_comb() {
    let code = r#"
import math

def combinations(n: int, k: int) -> int:
    return math.comb(n, k)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn combinations"),
        "Should transpile math.comb. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_math_perm() {
    let code = r#"
import math

def permutations(n: int, k: int) -> int:
    return math.perm(n, k)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn permutations"),
        "Should transpile math.perm. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_math_isfinite() {
    let code = r#"
import math

def check_finite(x: float) -> bool:
    return math.isfinite(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_finite"),
        "Should transpile math.isfinite. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_math_isinf() {
    let code = r#"
import math

def check_inf(x: float) -> bool:
    return math.isinf(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_inf"),
        "Should transpile math.isinf. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_math_isnan() {
    let code = r#"
import math

def check_nan(x: float) -> bool:
    return math.isnan(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_nan"),
        "Should transpile math.isnan. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_math_copysign() {
    let code = r#"
import math

def copy_sign(x: float, y: float) -> float:
    return math.copysign(x, y)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn copy_sign"),
        "Should transpile math.copysign. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_math_fmod() {
    let code = r#"
import math

def float_mod(x: float, y: float) -> float:
    return math.fmod(x, y)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn float_mod"),
        "Should transpile math.fmod. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_math_trunc() {
    let code = r#"
import math

def truncate(x: float) -> int:
    return math.trunc(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn truncate"),
        "Should transpile math.trunc. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_math_modf() {
    let code = r#"
import math

def split_float(x: float) -> tuple:
    return math.modf(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn split_float"),
        "Should transpile math.modf. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_math_ldexp() {
    let code = r#"
import math

def scale(x: float, n: int) -> float:
    return math.ldexp(x, n)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn scale"),
        "Should transpile math.ldexp. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_math_frexp() {
    let code = r#"
import math

def decompose(x: float) -> tuple:
    return math.frexp(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn decompose"),
        "Should transpile math.frexp. Got: {}",
        result
    );
}

// ============================================================================
// Advanced trig/hyperbolic math
// ============================================================================

#[test]
fn test_s11_rare_math_asin() {
    let code = r#"
import math

def arc_sine(x: float) -> float:
    return math.asin(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn arc_sine"),
        "Should transpile math.asin. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_math_acos() {
    let code = r#"
import math

def arc_cosine(x: float) -> float:
    return math.acos(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn arc_cosine"),
        "Should transpile math.acos. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_math_atan() {
    let code = r#"
import math

def arc_tangent(x: float) -> float:
    return math.atan(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn arc_tangent"),
        "Should transpile math.atan. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_math_atan2() {
    let code = r#"
import math

def angle(y: float, x: float) -> float:
    return math.atan2(y, x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn angle"),
        "Should transpile math.atan2. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_math_sinh() {
    let code = r#"
import math

def hyp_sine(x: float) -> float:
    return math.sinh(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn hyp_sine"),
        "Should transpile math.sinh. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_math_cosh() {
    let code = r#"
import math

def hyp_cosine(x: float) -> float:
    return math.cosh(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn hyp_cosine"),
        "Should transpile math.cosh. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_math_tanh() {
    let code = r#"
import math

def hyp_tangent(x: float) -> float:
    return math.tanh(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn hyp_tangent"),
        "Should transpile math.tanh. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_math_degrees() {
    let code = r#"
import math

def to_degrees(rad: float) -> float:
    return math.degrees(rad)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn to_degrees"),
        "Should transpile math.degrees. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_math_radians() {
    let code = r#"
import math

def to_radians(deg: float) -> float:
    return math.radians(deg)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn to_radians"),
        "Should transpile math.radians. Got: {}",
        result
    );
}

// ============================================================================
// Random module
// ============================================================================

#[test]
fn test_s11_rare_random_randint() {
    let code = r#"
import random

def dice() -> int:
    return random.randint(1, 6)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn dice"),
        "Should transpile random.randint. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_random_choice() {
    let code = r#"
import random

def pick(items: list) -> int:
    return random.choice(items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn pick"),
        "Should transpile random.choice. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_random_shuffle() {
    let code = r#"
import random

def shuffle(items: list) -> list:
    random.shuffle(items)
    return items
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn shuffle"),
        "Should transpile random.shuffle. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_random_random() {
    let code = r#"
import random

def coin_flip() -> bool:
    return random.random() < 0.5
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn coin_flip"),
        "Should transpile random.random. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_random_uniform() {
    let code = r#"
import random

def rand_float(lo: float, hi: float) -> float:
    return random.uniform(lo, hi)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn rand_float"),
        "Should transpile random.uniform. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_random_sample() {
    let code = r#"
import random

def pick_n(items: list, n: int) -> list:
    return random.sample(items, n)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn pick_n"),
        "Should transpile random.sample. Got: {}",
        result
    );
}

// ============================================================================
// Itertools patterns
// ============================================================================

#[test]
fn test_s11_rare_itertools_chain() {
    let code = r#"
import itertools

def chain_lists(a: list, b: list) -> list:
    return list(itertools.chain(a, b))
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn chain_lists"),
        "Should transpile itertools.chain. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_itertools_product() {
    let code = r#"
import itertools

def cross_product(a: list, b: list) -> list:
    return list(itertools.product(a, b))
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn cross_product"),
        "Should transpile itertools.product. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_itertools_permutations() {
    let code = r#"
import itertools

def all_orders(items: list) -> list:
    return list(itertools.permutations(items))
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn all_orders"),
        "Should transpile itertools.permutations. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_itertools_combinations() {
    let code = r#"
import itertools

def choose_pairs(items: list) -> list:
    return list(itertools.combinations(items, 2))
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn choose_pairs"),
        "Should transpile itertools.combinations. Got: {}",
        result
    );
}

// ============================================================================
// Complex algorithms exercising multiple code paths
// ============================================================================

#[test]
fn test_s11_rare_algo_merge_sort() {
    let code = r#"
def merge_sort(items: list) -> list:
    if len(items) <= 1:
        return items
    mid = len(items) // 2
    left = merge_sort(items[:mid])
    right = merge_sort(items[mid:])
    result: list = []
    i = 0
    j = 0
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            result.append(left[i])
            i += 1
        else:
            result.append(right[j])
            j += 1
    while i < len(left):
        result.append(left[i])
        i += 1
    while j < len(right):
        result.append(right[j])
        j += 1
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn merge_sort"),
        "Should transpile merge sort. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_algo_trie() {
    let code = r#"
class TrieNode:
    def __init__(self):
        self.children: dict = {}
        self.is_end = False

class Trie:
    def __init__(self):
        self.root = TrieNode()

    def insert(self, word: str):
        node = self.root
        for ch in word:
            if ch not in node.children:
                node.children[ch] = TrieNode()
            node = node.children[ch]
        node.is_end = True

    def search(self, word: str) -> bool:
        node = self.root
        for ch in word:
            if ch not in node.children:
                return False
            node = node.children[ch]
        return node.is_end
"#;
    let result = transpile(code);
    assert!(
        result.contains("TrieNode") || result.contains("Trie"),
        "Should transpile Trie. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_algo_lru_cache() {
    let code = r#"
class LRUCache:
    def __init__(self, capacity: int):
        self.capacity = capacity
        self.cache: dict = {}
        self.order: list = []

    def get(self, key: str) -> int:
        if key in self.cache:
            self.order.remove(key)
            self.order.append(key)
            return self.cache[key]
        return -1

    def put(self, key: str, value: int):
        if key in self.cache:
            self.order.remove(key)
        elif len(self.cache) >= self.capacity:
            oldest = self.order.pop(0)
            del self.cache[oldest]
        self.cache[key] = value
        self.order.append(key)
"#;
    let result = transpile(code);
    assert!(
        result.contains("LRUCache"),
        "Should transpile LRU cache. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_algo_graph_bfs() {
    let code = r#"
from collections import deque

def bfs(graph: dict, start: str) -> list:
    visited: list = []
    queue = deque([start])
    while len(queue) > 0:
        node = queue.popleft()
        if node not in visited:
            visited.append(node)
            for neighbor in graph.get(node, []):
                queue.append(neighbor)
    return visited
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn bfs"),
        "Should transpile BFS algorithm. Got: {}",
        result
    );
}

// ============================================================================
// Type system edge cases
// ============================================================================

#[test]
fn test_s11_rare_type_union_return() {
    let code = r#"
from typing import Union

def parse_value(s: str) -> Union[int, float]:
    try:
        return int(s)
    except ValueError:
        return float(s)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn parse_value"),
        "Should transpile Union return. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_type_callable() {
    let code = r#"
from typing import Callable

def apply(f: Callable, x: int) -> int:
    return f(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn apply"),
        "Should transpile Callable param. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_type_any() {
    let code = r#"
from typing import Any

def identity(x: Any) -> Any:
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn identity"),
        "Should transpile Any type. Got: {}",
        result
    );
}

// ============================================================================
// Complex expression patterns
// ============================================================================

#[test]
fn test_s11_rare_expr_multiple_assignment() {
    let code = r#"
def init_vars() -> tuple:
    a = b = c = 0
    a += 1
    b += 2
    c += 3
    return (a, b, c)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn init_vars"),
        "Should transpile multiple assignment. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_expr_conditional_import() {
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
fn test_s11_rare_expr_nested_function_call() {
    let code = r#"
def nested_calls(items: list) -> int:
    return len(list(set(items)))
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn nested_calls"),
        "Should transpile nested function calls. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_expr_complex_fstring() {
    let code = r#"
def complex_format(items: list) -> str:
    return f"Found {len(items)} items: {', '.join(str(x) for x in items)}"
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn complex_format"),
        "Should transpile complex f-string. Got: {}",
        result
    );
}

// ============================================================================
// Builtin functions edge cases
// ============================================================================

#[test]
fn test_s11_rare_builtin_chr() {
    let code = r#"
def to_char(n: int) -> str:
    return chr(n)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn to_char"),
        "Should transpile chr(). Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_builtin_ord() {
    let code = r#"
def to_code(c: str) -> int:
    return ord(c)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn to_code"),
        "Should transpile ord(). Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_builtin_hex() {
    let code = r#"
def to_hex(n: int) -> str:
    return hex(n)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn to_hex"),
        "Should transpile hex(). Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_builtin_bin() {
    let code = r#"
def to_bin(n: int) -> str:
    return bin(n)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn to_bin"),
        "Should transpile bin(). Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_builtin_oct() {
    let code = r#"
def to_oct(n: int) -> str:
    return oct(n)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn to_oct"),
        "Should transpile oct(). Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_builtin_reversed() {
    let code = r#"
def reverse(items: list) -> list:
    return list(reversed(items))
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn reverse"),
        "Should transpile reversed(). Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_builtin_isinstance_multi() {
    let code = r#"
def is_number(x) -> bool:
    return isinstance(x, (int, float))
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn is_number"),
        "Should transpile isinstance with tuple. Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_builtin_hasattr() {
    let code = r#"
def has_name(obj) -> bool:
    return hasattr(obj, "name")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn has_name"),
        "Should transpile hasattr(). Got: {}",
        result
    );
}

#[test]
fn test_s11_rare_builtin_divmod() {
    let code = r#"
def div_and_mod(a: int, b: int) -> tuple:
    return divmod(a, b)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn div_and_mod"),
        "Should transpile divmod(). Got: {}",
        result
    );
}
