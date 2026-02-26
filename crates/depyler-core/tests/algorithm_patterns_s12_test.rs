//! Session 12 Batch 31: Algorithm pattern transpilation tests
//!
//! Tests real-world algorithmic patterns that exercise multiple codegen paths
//! simultaneously, targeting combined coverage of stmt_gen, expr_gen, and
//! direct_rules_convert.

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

// ===== Sorting algorithms =====

#[test]
fn test_s12_bubble_sort() {
    let code = r#"
def bubble_sort(arr: list) -> list:
    n = len(arr)
    for i in range(n):
        for j in range(0, n - i - 1):
            if arr[j] > arr[j + 1]:
                arr[j], arr[j + 1] = arr[j + 1], arr[j]
    return arr
"#;
    let result = transpile(code);
    assert!(result.contains("fn bubble_sort"), "Got: {}", result);
}

#[test]
fn test_s12_insertion_sort() {
    let code = r#"
def insertion_sort(arr: list) -> list:
    for i in range(1, len(arr)):
        key = arr[i]
        j = i - 1
        while j >= 0 and arr[j] > key:
            arr[j + 1] = arr[j]
            j -= 1
        arr[j + 1] = key
    return arr
"#;
    let result = transpile(code);
    assert!(result.contains("fn insertion_sort"), "Got: {}", result);
}

#[test]
fn test_s12_selection_sort() {
    let code = r#"
def selection_sort(arr: list) -> list:
    n = len(arr)
    for i in range(n):
        min_idx = i
        for j in range(i + 1, n):
            if arr[j] < arr[min_idx]:
                min_idx = j
        arr[i], arr[min_idx] = arr[min_idx], arr[i]
    return arr
"#;
    let result = transpile(code);
    assert!(result.contains("fn selection_sort"), "Got: {}", result);
}

// ===== Search algorithms =====

#[test]
fn test_s12_linear_search() {
    let code = r#"
def linear_search(arr: list, target: int) -> int:
    for i in range(len(arr)):
        if arr[i] == target:
            return i
    return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn linear_search"), "Got: {}", result);
}

#[test]
fn test_s12_binary_search_recursive() {
    let code = r#"
def binary_search(arr: list, target: int, lo: int, hi: int) -> int:
    if lo > hi:
        return -1
    mid = (lo + hi) // 2
    if arr[mid] == target:
        return mid
    elif arr[mid] < target:
        return binary_search(arr, target, mid + 1, hi)
    else:
        return binary_search(arr, target, lo, mid - 1)
"#;
    let result = transpile(code);
    assert!(result.contains("fn binary_search"), "Got: {}", result);
}

// ===== String algorithms =====

#[test]
fn test_s12_is_palindrome() {
    let code = r#"
def is_palindrome(s: str) -> bool:
    cleaned = ""
    for c in s:
        if c.isalnum():
            cleaned += c.lower()
    return cleaned == cleaned[::-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_palindrome"), "Got: {}", result);
}

#[test]
fn test_s12_count_vowels() {
    let code = r#"
def count_vowels(text: str) -> int:
    count = 0
    for c in text:
        if c.lower() in "aeiou":
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_vowels"), "Got: {}", result);
}

#[test]
fn test_s12_reverse_words() {
    let code = r#"
def reverse_words(text: str) -> str:
    words = text.split()
    words.reverse()
    return " ".join(words)
"#;
    let result = transpile(code);
    assert!(result.contains("fn reverse_words"), "Got: {}", result);
}

#[test]
fn test_s12_caesar_cipher() {
    let code = r#"
def caesar_encrypt(text: str, shift: int) -> str:
    result = ""
    for c in text:
        if c.isalpha():
            base = ord("A") if c.isupper() else ord("a")
            shifted = (ord(c) - base + shift) % 26 + base
            result += chr(shifted)
        else:
            result += c
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn caesar_encrypt"), "Got: {}", result);
}

// ===== Math algorithms =====

#[test]
fn test_s12_gcd() {
    let code = r#"
def gcd(a: int, b: int) -> int:
    while b != 0:
        a, b = b, a % b
    return a
"#;
    let result = transpile(code);
    assert!(result.contains("fn gcd"), "Got: {}", result);
}

#[test]
fn test_s12_fibonacci_iterative() {
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
fn test_s12_is_prime() {
    let code = r#"
def is_prime(n: int) -> bool:
    if n < 2:
        return False
    if n < 4:
        return True
    if n % 2 == 0 or n % 3 == 0:
        return False
    i = 5
    while i * i <= n:
        if n % i == 0 or n % (i + 2) == 0:
            return False
        i += 6
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_prime"), "Got: {}", result);
}

#[test]
fn test_s12_sieve_of_eratosthenes() {
    let code = r#"
def sieve(n: int) -> list:
    is_prime = [True] * (n + 1)
    is_prime[0] = False
    is_prime[1] = False
    p = 2
    while p * p <= n:
        if is_prime[p]:
            for i in range(p * p, n + 1, p):
                is_prime[i] = False
        p += 1
    primes = []
    for i in range(2, n + 1):
        if is_prime[i]:
            primes.append(i)
    return primes
"#;
    let result = transpile(code);
    assert!(result.contains("fn sieve"), "Got: {}", result);
}

// ===== Data structure patterns =====

#[test]
fn test_s12_two_sum() {
    let code = r#"
def two_sum(nums: list, target: int) -> list:
    seen = {}
    for i in range(len(nums)):
        complement = target - nums[i]
        if complement in seen:
            return [seen[complement], i]
        seen[nums[i]] = i
    return []
"#;
    let result = transpile(code);
    assert!(result.contains("fn two_sum"), "Got: {}", result);
}

#[test]
fn test_s12_group_anagrams() {
    let code = r#"
def group_anagrams(words: list) -> dict:
    groups = {}
    for word in words:
        key = "".join(sorted(word))
        if key not in groups:
            groups[key] = []
        groups[key].append(word)
    return groups
"#;
    let result = transpile(code);
    assert!(result.contains("fn group_anagrams"), "Got: {}", result);
}

#[test]
fn test_s12_merge_intervals() {
    let code = r#"
def merge_intervals(intervals: list) -> list:
    if not intervals:
        return []
    intervals.sort()
    merged = [intervals[0]]
    for i in range(1, len(intervals)):
        if intervals[i][0] <= merged[-1][1]:
            merged[-1] = (merged[-1][0], max(merged[-1][1], intervals[i][1]))
        else:
            merged.append(intervals[i])
    return merged
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge_intervals"), "Got: {}", result);
}

// ===== Matrix operations =====

#[test]
fn test_s12_matrix_transpose() {
    let code = r#"
def transpose(matrix: list) -> list:
    rows = len(matrix)
    cols = len(matrix[0])
    result = []
    for j in range(cols):
        row = []
        for i in range(rows):
            row.append(matrix[i][j])
        result.append(row)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn transpose"), "Got: {}", result);
}

// ===== Graph-like algorithms =====

#[test]
fn test_s12_topological_sort_simple() {
    let code = r#"
def count_dependencies(graph: dict) -> dict:
    in_degree = {}
    for node in graph:
        if node not in in_degree:
            in_degree[node] = 0
        for neighbor in graph[node]:
            if neighbor not in in_degree:
                in_degree[neighbor] = 0
            in_degree[neighbor] += 1
    return in_degree
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_dependencies"), "Got: {}", result);
}

// ===== Functional patterns =====

#[test]
fn test_s12_map_filter_reduce_pattern() {
    let code = r#"
def sum_of_squares_of_evens(numbers: list) -> int:
    total = 0
    for n in numbers:
        if n % 2 == 0:
            total += n * n
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_of_squares_of_evens"), "Got: {}", result);
}

#[test]
fn test_s12_list_comp_pattern() {
    let code = r#"
def squares(n: int) -> list:
    return [i * i for i in range(n)]
"#;
    let result = transpile(code);
    assert!(result.contains("fn squares"), "Got: {}", result);
}

#[test]
fn test_s12_dict_comp_from_lists() {
    let code = r#"
def zip_to_dict(keys: list, values: list) -> dict:
    return {k: v for k, v in zip(keys, values)}
"#;
    let result = transpile(code);
    assert!(result.contains("fn zip_to_dict"), "Got: {}", result);
}

// ===== Recursive patterns =====

#[test]
fn test_s12_recursive_factorial() {
    let code = r#"
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"#;
    let result = transpile(code);
    assert!(result.contains("fn factorial"), "Got: {}", result);
}

#[test]
fn test_s12_recursive_sum() {
    let code = r#"
def recursive_sum(items: list, idx: int) -> int:
    if idx >= len(items):
        return 0
    return items[idx] + recursive_sum(items, idx + 1)
"#;
    let result = transpile(code);
    assert!(result.contains("fn recursive_sum"), "Got: {}", result);
}

// ===== Error handling patterns =====

#[test]
fn test_s12_try_parse_with_fallback() {
    let code = r#"
def safe_int(text: str, default: int) -> int:
    try:
        return int(text)
    except ValueError:
        return default
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_int"), "Got: {}", result);
}

#[test]
fn test_s12_validate_and_raise() {
    let code = r#"
def validate_age(age: int) -> int:
    if age < 0:
        raise ValueError("age cannot be negative")
    if age > 150:
        raise ValueError("age too large")
    return age
"#;
    let result = transpile(code);
    assert!(result.contains("fn validate_age"), "Got: {}", result);
}
