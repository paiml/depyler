//! Coverage tests for lifetime_analysis.rs and borrowing_context.rs
//!
//! DEPYLER-99MODE-001: Targets lifetime_analysis.rs (1,757 lines)
//! and borrowing_context.rs (1,733 lines)
//! Covers: parameter borrow strategies, mutation detection, escape analysis,
//! loop/conditional contexts, lifetime elision, ownership transfer.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[allow(dead_code)]
fn transpile(code: &str) -> String {
    DepylerPipeline::new()
        .transpile(code)
        .unwrap_or_else(|e| panic!("Transpilation failed: {e}"))
}

// ============================================================================
// Immutable borrow (read-only parameter)
// ============================================================================

#[test]
fn test_borrow_read_only_str() {
    let code = r#"
def get_len(s: str) -> int:
    return len(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_read_only_list() {
    let code = r#"
def first(items: list) -> int:
    return items[0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_read_only_dict() {
    let code = r#"
def lookup(d: dict, key: str) -> int:
    return d[key]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Mutable borrow (parameter mutation)
// ============================================================================

#[test]
fn test_borrow_mutable_list_append() {
    let code = r#"
def add_item(items: list, val: int):
    items.append(val)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_mutable_dict_assign() {
    let code = r#"
def set_key(d: dict, key: str, val: int):
    d[key] = val
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_mutable_list_extend() {
    let code = r#"
def extend_list(items: list, more: list):
    items.extend(more)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Ownership transfer (parameter escapes via return)
// ============================================================================

#[test]
fn test_ownership_identity() {
    let code = r#"
def identity(x: str) -> str:
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ownership_conditional_return() {
    let code = r#"
def longer(a: str, b: str) -> str:
    if len(a) > len(b):
        return a
    return b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ownership_modified_return() {
    let code = r#"
def upper(s: str) -> str:
    return s.upper()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Copy type parameters (primitives)
// ============================================================================

#[test]
fn test_copy_type_int() {
    let code = r#"
def double(x: int) -> int:
    return x * 2
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_copy_type_float() {
    let code = r#"
def halve(x: float) -> float:
    return x / 2.0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_copy_type_bool() {
    let code = r#"
def negate(flag: bool) -> bool:
    return not flag
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Loop context borrowing
// ============================================================================

#[test]
fn test_borrow_in_for_loop() {
    let code = r#"
def count_items(items: list, target: int) -> int:
    count = 0
    for item in items:
        if item == target:
            count += 1
    return count
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_in_while_loop() {
    let code = r#"
def find_index(items: list, target: int) -> int:
    i = 0
    while i < len(items):
        if items[i] == target:
            return i
        i += 1
    return -1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_nested_loop() {
    let code = r#"
def has_pair(items: list) -> bool:
    for i in range(len(items)):
        for j in range(i + 1, len(items)):
            if items[i] == items[j]:
                return True
    return False
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Conditional context
// ============================================================================

#[test]
fn test_borrow_in_if() {
    let code = r#"
def check(items: list) -> str:
    if len(items) > 0:
        return "non-empty"
    return "empty"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_in_if_else() {
    let code = r#"
def process(text: str, flag: bool) -> str:
    if flag:
        return text.upper()
    else:
        return text.lower()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Index/attribute mutation
// ============================================================================

#[test]
fn test_mutation_index_assign() {
    let code = r#"
def set_first(items: list, val: int):
    items[0] = val
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_mutation_augmented_assign() {
    let code = r#"
def increment(items: list, idx: int):
    items[idx] += 1
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Reassignment patterns
// ============================================================================

#[test]
fn test_reassignment_string() {
    let code = r#"
def process(s: str) -> str:
    s = s + "!"
    return s
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_reassignment_in_loop() {
    let code = r#"
def build_string(items: list) -> str:
    result = ""
    for item in items:
        result = result + str(item)
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Try/except borrowing
// ============================================================================

#[test]
fn test_borrow_in_try() {
    let code = r#"
def safe_access(items: list, idx: int) -> int:
    try:
        return items[idx]
    except IndexError:
        return -1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_try_finally() {
    let code = r#"
def process(data: list) -> int:
    result = 0
    try:
        result = data[0]
    finally:
        pass
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// With statement context manager
// ============================================================================

#[test]
fn test_borrow_with_statement() {
    let code = r#"
def read_file(path: str) -> str:
    with open(path) as f:
        return f.read()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Nested function / closure capture
// ============================================================================

#[test]
fn test_closure_capture_int() {
    let code = r#"
def make_adder(n: int):
    def add(x: int) -> int:
        return x + n
    return add
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_closure_capture_string() {
    let code = r#"
def make_greeter(greeting: str):
    def greet(name: str) -> str:
        return greeting + " " + name
    return greet
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Lambda parameter patterns
// ============================================================================

#[test]
fn test_lambda_capture() {
    let code = r#"
def f(multiplier: int) -> list:
    items = [1, 2, 3, 4, 5]
    return list(map(lambda x: x * multiplier, items))
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Comprehension borrowing
// ============================================================================

#[test]
fn test_comprehension_borrow() {
    let code = r#"
def square_items(items: list) -> list:
    return [x * x for x in items]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_comprehension_filter_borrow() {
    let code = r#"
def positive_items(items: list) -> list:
    return [x for x in items if x > 0]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Multiple parameter strategies
// ============================================================================

#[test]
fn test_multi_param_mixed() {
    let code = r#"
def process(text: str, count: int) -> str:
    result = ""
    for i in range(count):
        result += text
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_multi_param_all_borrowed() {
    let code = r#"
def compare(a: str, b: str) -> bool:
    return len(a) > len(b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_multi_param_one_mutated() {
    let code = r#"
def populate(items: list, source: list):
    for s in source:
        items.append(s)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex patterns combining multiple analysis features
// ============================================================================

#[test]
fn test_complex_binary_search() {
    let code = r#"
def binary_search(items: list, target: int) -> int:
    low = 0
    high = len(items) - 1
    while low <= high:
        mid = (low + high) // 2
        if items[mid] == target:
            return mid
        elif items[mid] < target:
            low = mid + 1
        else:
            high = mid - 1
    return -1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_complex_merge_lists() {
    let code = r#"
def merge(a: list, b: list) -> list:
    result = []
    i = 0
    j = 0
    while i < len(a) and j < len(b):
        if a[i] <= b[j]:
            result.append(a[i])
            i += 1
        else:
            result.append(b[j])
            j += 1
    while i < len(a):
        result.append(a[i])
        i += 1
    while j < len(b):
        result.append(b[j])
        j += 1
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_complex_string_transform() {
    let code = r#"
def transform(text: str) -> str:
    words = text.split()
    result = []
    for word in words:
        if len(word) > 3:
            result.append(word.upper())
        else:
            result.append(word.lower())
    return " ".join(result)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_complex_dict_operations() {
    let code = r#"
def count_words(text: str) -> dict:
    counts = {}
    for word in text.lower().split():
        if word in counts:
            counts[word] += 1
        else:
            counts[word] = 1
    return counts
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_complex_nested_data() {
    let code = r#"
def flatten(matrix: list) -> list:
    result = []
    for row in matrix:
        for item in row:
            result.append(item)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_complex_accumulator() {
    let code = r#"
def running_total(items: list) -> list:
    result = []
    total = 0
    for item in items:
        total += item
        result.append(total)
    return result
"#;
    assert!(transpile_ok(code));
}
