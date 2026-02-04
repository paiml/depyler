//! Coverage tests for direct_rules_convert.rs
//!
//! DEPYLER-99MODE-001: Targets direct_rules_convert.rs (8,169 lines)
//! Covers: body conversion, mutability analysis, assignment conversion,
//! ExprConverter, builtin converters, method calls, comprehensions,
//! type coercion, statement pipelines.

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
// Body conversion - basic statements
// ============================================================================

#[test]
fn test_drc_body_single_return() {
    let code = "def f() -> int:\n    return 42\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_body_multiple_stmts() {
    let code = r#"
def f(x: int) -> int:
    y = x + 1
    z = y * 2
    return z
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_body_pass() {
    let code = "def f():\n    pass\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Mutability analysis
// ============================================================================

#[test]
fn test_drc_mutable_reassignment() {
    let code = r#"
def f() -> int:
    x = 0
    x = 10
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_mutable_augmented_assign() {
    let code = r#"
def f() -> int:
    x = 0
    x += 5
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_mutable_list_append() {
    let code = r#"
def f() -> list:
    items = []
    items.append(1)
    items.append(2)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_mutable_list_extend() {
    let code = r#"
def f() -> list:
    items = [1]
    items.extend([2, 3])
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_mutable_list_insert() {
    let code = r#"
def f() -> list:
    items = [1, 3]
    items.insert(1, 2)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_mutable_list_pop() {
    let code = r#"
def f() -> int:
    items = [1, 2, 3]
    return items.pop()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_mutable_list_remove() {
    let code = r#"
def f():
    items = [1, 2, 3]
    items.remove(2)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_mutable_list_clear() {
    let code = r#"
def f():
    items = [1, 2, 3]
    items.clear()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_mutable_list_reverse() {
    let code = r#"
def f():
    items = [3, 1, 2]
    items.reverse()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_mutable_list_sort() {
    let code = r#"
def f():
    items = [3, 1, 2]
    items.sort()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_mutable_set_add() {
    let code = r#"
def f():
    s = {1, 2}
    s.add(3)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_mutable_set_discard() {
    let code = r#"
def f():
    s = {1, 2, 3}
    s.discard(2)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_mutable_set_update() {
    let code = r#"
def f():
    s = {1, 2}
    s.update({3, 4})
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_mutable_dict_setdefault() {
    let code = r#"
def f(d: dict):
    d.setdefault("key", 0)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Assignment conversion
// ============================================================================

#[test]
fn test_drc_assign_simple() {
    let code = "def f() -> int:\n    x = 42\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_assign_from_expr() {
    let code = "def f(a: int, b: int) -> int:\n    c = a + b\n    return c\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_assign_index() {
    let code = r#"
def f() -> list:
    items = [1, 2, 3]
    items[0] = 10
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_assign_attribute() {
    let code = r#"
class Foo:
    def __init__(self):
        self.x = 0

    def set_x(self, val: int):
        self.x = val
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_assign_tuple_unpack() {
    let code = r#"
def f() -> int:
    a, b = 1, 2
    return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_assign_augmented_add() {
    let code = "def f(x: int) -> int:\n    x += 10\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_assign_augmented_sub() {
    let code = "def f(x: int) -> int:\n    x -= 5\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_assign_augmented_mul() {
    let code = "def f(x: int) -> int:\n    x *= 3\n    return x\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Control flow statements
// ============================================================================

#[test]
fn test_drc_if_simple() {
    let code = r#"
def f(x: int) -> str:
    if x > 0:
        return "positive"
    return "non-positive"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_if_else() {
    let code = r#"
def f(x: int) -> str:
    if x > 0:
        return "positive"
    else:
        return "non-positive"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_if_elif_else() {
    let code = r#"
def f(x: int) -> str:
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    else:
        return "zero"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_while_loop() {
    let code = r#"
def f(n: int) -> int:
    total = 0
    i = 0
    while i < n:
        total += i
        i += 1
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_for_range() {
    let code = r#"
def f(n: int) -> int:
    total = 0
    for i in range(n):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_for_list() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for item in items:
        total += item
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_break_stmt() {
    let code = r#"
def f(items: list) -> int:
    for item in items:
        if item > 100:
            break
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_continue_stmt() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for item in items:
        if item < 0:
            continue
        total += item
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Exception handling
// ============================================================================

#[test]
fn test_drc_try_except() {
    let code = r#"
def f(x: int) -> int:
    try:
        return 100 // x
    except:
        return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_try_except_typed() {
    let code = r#"
def f(x: int) -> int:
    try:
        return 100 // x
    except ValueError:
        return -1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_try_finally() {
    let code = r#"
def f():
    x = 0
    try:
        x = 1
    finally:
        x = 2
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_raise() {
    let code = r#"
def f(x: int) -> int:
    if x < 0:
        raise ValueError("negative")
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_assert() {
    let code = r#"
def f(x: int) -> int:
    assert x > 0
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_assert_message() {
    let code = r#"
def f(x: int) -> int:
    assert x > 0, "must be positive"
    return x
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Context managers
// ============================================================================

#[test]
fn test_drc_with_statement() {
    let code = r#"
def f(path: str) -> str:
    with open(path) as file:
        return file.read()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Builtin function converters
// ============================================================================

#[test]
fn test_drc_builtin_len() {
    let code = "def f(s: str) -> int:\n    return len(s)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_builtin_abs() {
    let code = "def f(x: int) -> int:\n    return abs(x)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_builtin_min_max() {
    let code = r#"
def f(a: int, b: int) -> int:
    return min(a, max(b, 0))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_builtin_sum() {
    let code = "def f(items: list) -> int:\n    return sum(items)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_builtin_sorted() {
    let code = "def f(items: list) -> list:\n    return sorted(items)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_builtin_reversed() {
    let code = r#"
def f(items: list) -> list:
    result = []
    for item in reversed(items):
        result.append(item)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_builtin_enumerate() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for i, val in enumerate(items):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_builtin_zip() {
    let code = r#"
def f(a: list, b: list) -> list:
    result = []
    for x, y in zip(a, b):
        result.append(x + y)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_builtin_all() {
    let code = "def f(items: list) -> bool:\n    return all(items)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_builtin_any() {
    let code = "def f(items: list) -> bool:\n    return any(items)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_builtin_hex() {
    let code = "def f(n: int) -> str:\n    return hex(n)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_builtin_bin() {
    let code = "def f(n: int) -> str:\n    return bin(n)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_builtin_oct() {
    let code = "def f(n: int) -> str:\n    return oct(n)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_builtin_chr() {
    let code = "def f(n: int) -> str:\n    return chr(n)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_builtin_ord() {
    let code = "def f(c: str) -> int:\n    return ord(c)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_builtin_pow() {
    let code = "def f(b: int, e: int) -> int:\n    return pow(b, e)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_builtin_round() {
    let code = "def f(x: float) -> int:\n    return round(x)\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Type conversions
// ============================================================================

#[test]
fn test_drc_type_int_from_str() {
    let code = "def f(s: str) -> int:\n    return int(s)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_type_str_from_int() {
    let code = "def f(n: int) -> str:\n    return str(n)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_type_float_from_str() {
    let code = "def f(s: str) -> float:\n    return float(s)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_type_bool_from_int() {
    let code = "def f(n: int) -> bool:\n    return bool(n)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_type_list_from_range() {
    let code = "def f() -> list:\n    return list(range(10))\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Comprehension expressions
// ============================================================================

#[test]
fn test_drc_list_comp() {
    let code = r#"
def f() -> list:
    return [x * 2 for x in range(10)]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_list_comp_filter() {
    let code = r#"
def f() -> list:
    return [x for x in range(20) if x % 2 == 0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_set_comp() {
    let code = r#"
def f(items: list) -> set:
    return {x * 2 for x in items}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_dict_comp() {
    let code = r#"
def f() -> dict:
    return {str(i): i * i for i in range(5)}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Lambda and functional patterns
// ============================================================================

#[test]
fn test_drc_lambda_in_sorted() {
    let code = r#"
def f(items: list) -> list:
    return sorted(items, key=lambda x: -x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_map_lambda() {
    let code = r#"
def f(items: list) -> list:
    return list(map(lambda x: x * 2, items))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_filter_lambda() {
    let code = r#"
def f(items: list) -> list:
    return list(filter(lambda x: x > 0, items))
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Class method context
// ============================================================================

#[test]
fn test_drc_classmethod_context() {
    let code = r#"
class Counter:
    def __init__(self):
        self.count = 0

    def increment(self):
        self.count += 1

    def get_count(self) -> int:
        return self.count
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_classmethod_multiple() {
    let code = r#"
class Stack:
    def __init__(self):
        self.items = []

    def push(self, item: int):
        self.items.append(item)

    def pop(self) -> int:
        return self.items.pop()

    def is_empty(self) -> bool:
        return len(self.items) == 0
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Nested function support
// ============================================================================

#[test]
fn test_drc_nested_function() {
    let code = r#"
def outer(n: int) -> int:
    def inner(x: int) -> int:
        return x * 2
    return inner(n)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Multiple functions
// ============================================================================

#[test]
fn test_drc_multi_function() {
    let code = r#"
def helper(x: int) -> int:
    return x * 2

def process(items: list) -> int:
    total = 0
    for item in items:
        total += helper(item)
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex integration patterns
// ============================================================================

#[test]
fn test_drc_binary_search() {
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
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_bubble_sort() {
    let code = r#"
def bubble_sort(items: list) -> list:
    n = len(items)
    for i in range(n):
        for j in range(0, n - i - 1):
            if items[j] > items[j + 1]:
                items[j], items[j + 1] = items[j + 1], items[j]
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_recursive_factorial() {
    let code = r#"
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_dict_counter() {
    let code = r#"
def count_chars(s: str) -> dict:
    counts = {}
    for c in s:
        counts[c] = counts.get(c, 0) + 1
    return counts
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_string_processing() {
    let code = r#"
def process_text(text: str) -> list:
    words = text.split()
    result = []
    for word in words:
        cleaned = word.strip().lower()
        if len(cleaned) > 0:
            result.append(cleaned)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_drc_two_sum() {
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
    assert!(transpile_ok(code));
}
