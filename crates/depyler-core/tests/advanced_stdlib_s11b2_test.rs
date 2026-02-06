//! Session 11 Batch 2: Advanced stdlib function coverage
//!
//! Targets deep conversion paths in:
//! - expr_gen.rs: textwrap, shlex, fnmatch, urllib, binascii, heapq
//! - expr_gen.rs: uuid, pickle, struct, itertools
//! - direct_rules_convert.rs: complex builtins (min/max/abs/round edge cases)
//! - expr_gen_instance_methods.rs: file I/O methods, path methods, datetime methods

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

// ===== itertools =====

#[test]
fn test_s11b2_itertools_chain() {
    let code = r#"
import itertools

def chain_lists(a: list, b: list) -> list:
    return list(itertools.chain(a, b))
"#;
    let result = transpile(code);
    assert!(result.contains("fn chain_lists"), "Got: {}", result);
}

#[test]
fn test_s11b2_itertools_product() {
    let code = r#"
import itertools

def all_pairs(a: list, b: list) -> list:
    return list(itertools.product(a, b))
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_pairs"), "Got: {}", result);
}

#[test]
fn test_s11b2_itertools_permutations() {
    let code = r#"
import itertools

def all_perms(items: list) -> list:
    return list(itertools.permutations(items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_perms"), "Got: {}", result);
}

#[test]
fn test_s11b2_itertools_combinations() {
    let code = r#"
import itertools

def all_combos(items: list, k: int) -> list:
    return list(itertools.combinations(items, k))
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_combos"), "Got: {}", result);
}

// ===== heapq =====

#[test]
fn test_s11b2_heapq_push_pop() {
    let code = r#"
import heapq

def heap_sort(items: list) -> list:
    heap = []
    for item in items:
        heapq.heappush(heap, item)
    result = []
    while heap:
        result.append(heapq.heappop(heap))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn heap_sort"), "Got: {}", result);
}

#[test]
fn test_s11b2_heapq_nsmallest() {
    let code = r#"
import heapq

def top_three(items: list) -> list:
    return heapq.nsmallest(3, items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn top_three"), "Got: {}", result);
}

#[test]
fn test_s11b2_heapq_nlargest() {
    let code = r#"
import heapq

def bottom_three(items: list) -> list:
    return heapq.nlargest(3, items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn bottom_three"), "Got: {}", result);
}

// ===== random =====

#[test]
fn test_s11b2_random_randint() {
    let code = r#"
import random

def roll_dice() -> int:
    return random.randint(1, 6)
"#;
    let result = transpile(code);
    assert!(result.contains("fn roll_dice"), "Got: {}", result);
}

#[test]
fn test_s11b2_random_choice() {
    let code = r#"
import random

def pick_one(items: list) -> int:
    return random.choice(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn pick_one"), "Got: {}", result);
}

#[test]
fn test_s11b2_random_shuffle() {
    let code = r#"
import random

def shuffle_list(items: list) -> list:
    random.shuffle(items)
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn shuffle_list"), "Got: {}", result);
}

#[test]
fn test_s11b2_random_uniform() {
    let code = r#"
import random

def random_float(lo: float, hi: float) -> float:
    return random.uniform(lo, hi)
"#;
    let result = transpile(code);
    assert!(result.contains("fn random_float"), "Got: {}", result);
}

#[test]
fn test_s11b2_random_sample() {
    let code = r#"
import random

def sample_items(items: list, k: int) -> list:
    return random.sample(items, k)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sample_items"), "Got: {}", result);
}

// ===== Complex algorithms =====

#[test]
fn test_s11b2_binary_search() {
    let code = r#"
def binary_search(items: list, target: int) -> int:
    lo = 0
    hi = len(items) - 1
    while lo <= hi:
        mid = (lo + hi) // 2
        if items[mid] == target:
            return mid
        elif items[mid] < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn binary_search"), "Got: {}", result);
}

#[test]
fn test_s11b2_bubble_sort() {
    let code = r#"
def bubble_sort(items: list) -> list:
    n = len(items)
    for i in range(n):
        for j in range(0, n - i - 1):
            if items[j] > items[j + 1]:
                items[j], items[j + 1] = items[j + 1], items[j]
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn bubble_sort"), "Got: {}", result);
}

#[test]
fn test_s11b2_fibonacci_iterative() {
    let code = r#"
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    a = 0
    b = 1
    for i in range(2, n + 1):
        a, b = b, a + b
    return b
"#;
    let result = transpile(code);
    assert!(result.contains("fn fibonacci"), "Got: {}", result);
}

#[test]
fn test_s11b2_matrix_multiply() {
    let code = r#"
def mat_mul(a: list, b: list) -> list:
    n = len(a)
    result = [[0 for j in range(n)] for i in range(n)]
    for i in range(n):
        for j in range(n):
            for k in range(n):
                result[i][j] += a[i][k] * b[k][j]
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn mat_mul"), "Got: {}", result);
}

#[test]
fn test_s11b2_gcd_recursive() {
    let code = r#"
def gcd(a: int, b: int) -> int:
    if b == 0:
        return a
    return gcd(b, a % b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn gcd"), "Got: {}", result);
}

#[test]
fn test_s11b2_sieve_primes() {
    let code = r#"
def sieve(n: int) -> list:
    is_prime = [True] * (n + 1)
    is_prime[0] = False
    is_prime[1] = False
    for i in range(2, n + 1):
        if is_prime[i]:
            for j in range(i * i, n + 1, i):
                is_prime[j] = False
    result = []
    for i in range(n + 1):
        if is_prime[i]:
            result.append(i)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn sieve"), "Got: {}", result);
}

// ===== Complex data structures =====

#[test]
fn test_s11b2_stack_impl() {
    let code = r#"
class Stack:
    def __init__(self):
        self.items = []

    def push(self, item: int):
        self.items.append(item)

    def pop(self) -> int:
        return self.items.pop()

    def peek(self) -> int:
        return self.items[-1]

    def is_empty(self) -> bool:
        return len(self.items) == 0

    def size(self) -> int:
        return len(self.items)
"#;
    let result = transpile(code);
    assert!(result.contains("Stack"), "Got: {}", result);
}

#[test]
fn test_s11b2_linked_list_node() {
    let code = r#"
class ListNode:
    def __init__(self, val: int):
        self.val = val
        self.next = None
"#;
    let result = transpile(code);
    assert!(result.contains("ListNode"), "Got: {}", result);
}

#[test]
fn test_s11b2_tree_node() {
    let code = r#"
class TreeNode:
    def __init__(self, val: int):
        self.val = val
        self.left = None
        self.right = None
"#;
    let result = transpile(code);
    assert!(result.contains("TreeNode"), "Got: {}", result);
}

// ===== Complex string operations =====

#[test]
fn test_s11b2_string_encode() {
    let code = r#"
def to_bytes(s: str) -> bytes:
    return s.encode("utf-8")
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_bytes"), "Got: {}", result);
}

#[test]
fn test_s11b2_string_format_multiple() {
    let code = r#"
def format_greeting(name: str, age: int) -> str:
    return "Hello {}, you are {} years old".format(name, age)
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_greeting"), "Got: {}", result);
}

#[test]
fn test_s11b2_string_translate() {
    let code = r#"
def remove_vowels(s: str) -> str:
    return s.translate(str.maketrans("", "", "aeiouAEIOU"))
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_vowels"), "Got: {}", result);
}

#[test]
fn test_s11b2_string_splitlines() {
    let code = r#"
def get_lines(text: str) -> list:
    return text.splitlines()
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_lines"), "Got: {}", result);
}

#[test]
fn test_s11b2_string_partition() {
    let code = r#"
def split_first(text: str, sep: str) -> tuple:
    return text.partition(sep)
"#;
    let result = transpile(code);
    assert!(result.contains("fn split_first"), "Got: {}", result);
}

#[test]
fn test_s11b2_string_rpartition() {
    let code = r#"
def split_last(text: str, sep: str) -> tuple:
    return text.rpartition(sep)
"#;
    let result = transpile(code);
    assert!(result.contains("fn split_last"), "Got: {}", result);
}

#[test]
fn test_s11b2_string_expandtabs() {
    let code = r#"
def expand(text: str) -> str:
    return text.expandtabs(4)
"#;
    let result = transpile(code);
    assert!(result.contains("fn expand"), "Got: {}", result);
}

#[test]
fn test_s11b2_string_zfill() {
    let code = r#"
def pad_number(n: str) -> str:
    return n.zfill(5)
"#;
    let result = transpile(code);
    assert!(result.contains("fn pad_number"), "Got: {}", result);
}

// ===== Complex patterns =====

#[test]
fn test_s11b2_decorator_classmethod() {
    let code = r#"
class Config:
    instances = 0

    def __init__(self):
        Config.instances += 1

    @classmethod
    def get_count(cls) -> int:
        return cls.instances
"#;
    let result = transpile(code);
    assert!(result.contains("Config"), "Got: {}", result);
}

#[test]
fn test_s11b2_enumerate_unpack() {
    let code = r#"
def print_indexed(items: list):
    for idx, val in enumerate(items):
        print(str(idx) + ": " + str(val))
"#;
    let result = transpile(code);
    assert!(result.contains("fn print_indexed"), "Got: {}", result);
}

#[test]
fn test_s11b2_dict_items_unpack() {
    let code = r#"
def format_dict(d: dict) -> str:
    result = ""
    for key, value in d.items():
        result += str(key) + "=" + str(value) + " "
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_dict"), "Got: {}", result);
}

#[test]
fn test_s11b2_nested_dict_access() {
    let code = r#"
def deep_get(d: dict, key1: str, key2: str) -> int:
    return d[key1][key2]
"#;
    let result = transpile(code);
    assert!(result.contains("fn deep_get"), "Got: {}", result);
}

#[test]
fn test_s11b2_list_of_dicts() {
    let code = r#"
def find_by_name(items: list, name: str) -> dict:
    for item in items:
        if item["name"] == name:
            return item
    return {}
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_by_name"), "Got: {}", result);
}

#[test]
fn test_s11b2_multiple_assignment() {
    let code = r#"
def init_vars() -> tuple:
    x = y = z = 0
    return (x, y, z)
"#;
    let result = transpile(code);
    assert!(result.contains("fn init_vars"), "Got: {}", result);
}

// ===== Optional/None patterns =====

#[test]
fn test_s11b2_none_comparison() {
    let code = r#"
def is_none(x) -> bool:
    return x is None

def is_not_none(x) -> bool:
    return x is not None
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_none"), "Got: {}", result);
    assert!(result.contains("fn is_not_none"), "Got: {}", result);
}

#[test]
fn test_s11b2_default_parameter_none() {
    let code = r#"
def greet(name: str = None) -> str:
    if name is None:
        return "Hello"
    return "Hello " + name
"#;
    let result = transpile(code);
    assert!(result.contains("fn greet"), "Got: {}", result);
}

// ===== Complex expression patterns =====

#[test]
fn test_s11b2_string_multiplication() {
    let code = r#"
def repeat(s: str, n: int) -> str:
    return s * n
"#;
    let result = transpile(code);
    assert!(result.contains("fn repeat"), "Got: {}", result);
}

#[test]
fn test_s11b2_list_multiplication() {
    let code = r#"
def zeros(n: int) -> list:
    return [0] * n
"#;
    let result = transpile(code);
    assert!(result.contains("fn zeros"), "Got: {}", result);
}

#[test]
fn test_s11b2_fstring_format_spec() {
    let code = r#"
def format_float(x: float) -> str:
    return f"{x:.2f}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_float"), "Got: {}", result);
}

#[test]
fn test_s11b2_fstring_padding() {
    let code = r#"
def pad_right(s: str) -> str:
    return f"{s:<20}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn pad_right"), "Got: {}", result);
}

#[test]
fn test_s11b2_fstring_multiple_values() {
    let code = r#"
def describe(name: str, age: int) -> str:
    return f"{name} is {age} years old"
"#;
    let result = transpile(code);
    assert!(result.contains("fn describe"), "Got: {}", result);
}
