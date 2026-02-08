//! Coverage boost tests for statement codegen functions
//!
//! Targets uncovered branches in:
//! - stmt_gen.rs: codegen_assign_stmt, codegen_expr_stmt, codegen_return_stmt,
//!   codegen_assign_index, codegen_assign_tuple
//! - stmt_gen_complex.rs: try_generate_subcommand_match
//! - direct_rules_convert/stmt_convert.rs: convert_stmt_with_context
//! - direct_rules_convert/method_stmt_convert.rs: convert_method_stmt

use crate::DepylerPipeline;

fn transpile(code: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline
        .transpile(code)
        .expect("transpilation should succeed")
}

fn transpile_ok(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_ok()
}

// =============================================================================
// Section 1: codegen_assign_stmt (62.3% -> target 85%)
// =============================================================================

#[test]
fn test_assign_type_annotation_only() {
    let code = transpile("def foo():\n    count: int = 0\n    return count");
    assert!(
        code.contains("i64") || code.contains("i32") || code.contains("0"),
        "typed assign: {}",
        code
    );
}

#[test]
fn test_assign_dict_augmented() {
    let code = transpile(
        "def count_words(words: list) -> dict:\n    counts: dict = {}\n    for w in words:\n        counts[w] = counts.get(w, 0) + 1\n    return counts",
    );
    assert!(!code.is_empty(), "dict augmented: {}", code);
}

#[test]
fn test_assign_multiple_targets() {
    let code = transpile("def swap() -> int:\n    a = 1\n    b = 2\n    a, b = b, a\n    return a");
    assert!(!code.is_empty(), "swap: {}", code);
}

#[test]
fn test_assign_from_call() {
    let code =
        transpile("def foo(items: list) -> int:\n    n = len(items)\n    return n");
    assert!(code.contains("len()"), "assign from call: {}", code);
}

#[test]
fn test_assign_bool() {
    let code = transpile("def foo() -> bool:\n    flag = True\n    return flag");
    assert!(code.contains("true"), "bool assign: {}", code);
}

#[test]
fn test_assign_string() {
    let code = transpile("def foo() -> str:\n    name = \"hello\"\n    return name");
    assert!(code.contains("hello"), "string assign: {}", code);
}

#[test]
fn test_assign_nested_dict() {
    let code = transpile(
        "def foo(data: dict):\n    value = data[\"key\"]\n    return value",
    );
    assert!(!code.is_empty(), "nested dict: {}", code);
}

#[test]
fn test_assign_list_comprehension() {
    let code = transpile(
        "def squares(n: int) -> list:\n    result = [x * x for x in range(n)]\n    return result",
    );
    assert!(
        code.contains("map") || code.contains("collect") || code.contains("iter"),
        "listcomp assign: {}",
        code
    );
}

#[test]
fn test_assign_ternary() {
    let code = transpile(
        "def pick(x: int) -> str:\n    label = \"big\" if x > 10 else \"small\"\n    return label",
    );
    assert!(
        code.contains("if") || code.contains("else"),
        "ternary: {}",
        code
    );
}

#[test]
fn test_assign_string_method() {
    let code = transpile(
        "def clean(s: str) -> str:\n    result = s.strip()\n    return result",
    );
    assert!(code.contains("trim"), "string method assign: {}", code);
}

// =============================================================================
// Section 2: codegen_expr_stmt (46.5% -> target 75%)
// =============================================================================

#[test]
fn test_expr_stmt_method_call() {
    let code = transpile("def foo(items: list):\n    items.append(42)");
    assert!(code.contains("push"), "append expr stmt: {}", code);
}

#[test]
fn test_expr_stmt_print() {
    let code = transpile("def foo():\n    print(\"hello\")");
    assert!(
        code.contains("println!") || code.contains("print"),
        "print expr: {}",
        code
    );
}

#[test]
fn test_expr_stmt_sort() {
    let code = transpile("def foo(items: list):\n    items.sort()");
    assert!(code.contains("sort"), "sort expr stmt: {}", code);
}

#[test]
fn test_expr_stmt_list_extend() {
    let code = transpile("def foo(a: list, b: list):\n    a.extend(b)");
    assert!(code.contains("extend"), "extend expr stmt: {}", code);
}

#[test]
fn test_expr_stmt_dict_update() {
    let code = transpile("def foo(d: dict):\n    d.update({\"a\": 1})");
    assert!(!code.is_empty(), "dict update expr: {}", code);
}

#[test]
fn test_expr_stmt_list_reverse() {
    let code = transpile("def foo(items: list):\n    items.reverse()");
    assert!(code.contains("reverse"), "reverse expr stmt: {}", code);
}

#[test]
fn test_expr_stmt_list_clear() {
    let code = transpile("def foo(items: list):\n    items.clear()");
    assert!(code.contains("clear"), "clear expr stmt: {}", code);
}

#[test]
fn test_expr_stmt_list_pop() {
    let code =
        transpile("def foo(items: list) -> int:\n    return items.pop()");
    assert!(code.contains("pop"), "pop expr stmt: {}", code);
}

// =============================================================================
// Section 3: codegen_return_stmt (67.1% -> target 85%)
// =============================================================================

#[test]
fn test_return_none_explicit() {
    let code = transpile("def foo():\n    return None");
    assert!(!code.is_empty(), "return None: {}", code);
}

#[test]
fn test_return_tuple_two() {
    let code =
        transpile("def foo(x: int) -> tuple:\n    return (x, x + 1)");
    assert!(
        code.contains("(") || code.contains(","),
        "return tuple: {}",
        code
    );
}

#[test]
fn test_return_tuple_three() {
    let code = transpile(
        "def foo(x: int) -> tuple:\n    return (x, x + 1, x + 2)",
    );
    assert!(!code.is_empty(), "return 3-tuple: {}", code);
}

#[test]
fn test_return_list_literal() {
    let code = transpile("def foo() -> list:\n    return [1, 2, 3]");
    assert!(
        code.contains("vec!") || code.contains("Vec"),
        "return list: {}",
        code
    );
}

#[test]
fn test_return_dict_literal() {
    let code =
        transpile("def foo() -> dict:\n    return {\"a\": 1, \"b\": 2}");
    assert!(!code.is_empty(), "return dict: {}", code);
}

#[test]
fn test_return_string_format() {
    let code = transpile(
        "def greet(name: str) -> str:\n    return f\"Hello, {name}\"",
    );
    assert!(code.contains("format!"), "return fstring: {}", code);
}

#[test]
fn test_return_boolean_expr() {
    let code = transpile(
        "def check(x: int) -> bool:\n    return x > 0 and x < 100",
    );
    assert!(
        code.contains("&&") || code.contains(">"),
        "return bool expr: {}",
        code
    );
}

#[test]
fn test_return_empty() {
    let code = transpile("def foo():\n    return");
    assert!(!code.is_empty(), "return empty: {}", code);
}

#[test]
fn test_return_nested_call() {
    let code = transpile(
        "def foo(items: list) -> int:\n    return len(sorted(items))",
    );
    assert!(!code.is_empty(), "return nested call: {}", code);
}

// =============================================================================
// Section 4: codegen_assign_index (52.8% -> target 75%)
// =============================================================================

#[test]
fn test_assign_index_dict_set() {
    let code = transpile(
        "def set_val(d: dict, key: str, val: int):\n    d[key] = val",
    );
    assert!(code.contains("insert"), "dict index set: {}", code);
}

#[test]
fn test_assign_index_list_set() {
    let code = transpile(
        "def set_val(items: list, idx: int, val: int):\n    items[idx] = val",
    );
    assert!(code.contains("["), "list index set: {}", code);
}

#[test]
fn test_assign_index_negative() {
    let code = transpile(
        "def set_last(items: list, val: int):\n    items[-1] = val",
    );
    assert!(!code.is_empty(), "negative index: {}", code);
}

#[test]
fn test_assign_index_string_key() {
    let code = transpile(
        "def set_config(config: dict):\n    config[\"host\"] = \"localhost\"\n    config[\"port\"] = 8080",
    );
    assert!(
        code.contains("insert") || code.contains("localhost"),
        "string key: {}",
        code
    );
}

#[test]
fn test_assign_index_counter_pattern() {
    let code = transpile(
        "def count(words: list) -> dict:\n    result: dict = {}\n    for w in words:\n        result[w] = result.get(w, 0) + 1\n    return result",
    );
    assert!(!code.is_empty(), "counter pattern: {}", code);
}

// =============================================================================
// Section 5: codegen_assign_tuple (48.0% -> target 75%)
// =============================================================================

#[test]
fn test_assign_tuple_unpack_two() {
    let code = transpile(
        "def split_pair(pair: tuple) -> int:\n    a, b = pair\n    return a + b",
    );
    assert!(!code.is_empty(), "tuple unpack 2: {}", code);
}

#[test]
fn test_assign_tuple_unpack_three() {
    let code = transpile(
        "def unpack(t: tuple) -> int:\n    a, b, c = t\n    return a + b + c",
    );
    assert!(!code.is_empty(), "tuple unpack 3: {}", code);
}

#[test]
fn test_assign_tuple_from_enumerate() {
    let code = transpile(
        "def indexed(items: list):\n    for i, val in enumerate(items):\n        print(i, val)",
    );
    assert!(
        code.contains("enumerate") || code.contains("iter"),
        "enumerate unpack: {}",
        code
    );
}

#[test]
fn test_assign_tuple_swap() {
    let code = transpile(
        "def swap(a: int, b: int) -> tuple:\n    a, b = b, a\n    return (a, b)",
    );
    assert!(!code.is_empty(), "swap pattern: {}", code);
}

#[test]
fn test_assign_tuple_from_split() {
    let code = transpile(
        "def parse(line: str) -> str:\n    key, value = line.split(\"=\", 1)\n    return key",
    );
    assert!(
        code.contains("split") || code.contains("splitn"),
        "split unpack: {}",
        code
    );
}

#[test]
fn test_assign_tuple_reassign() {
    let code = transpile(
        "def fib() -> int:\n    a = 0\n    b = 1\n    a, b = b, a + b\n    return b",
    );
    assert!(!code.is_empty(), "fib swap: {}", code);
}

#[test]
fn test_assign_tuple_from_items() {
    let code = transpile(
        "def process(d: dict):\n    for key, value in d.items():\n        print(key, value)",
    );
    assert!(!code.is_empty(), "dict items unpack: {}", code);
}

// =============================================================================
// Section 6: convert_stmt_with_context (50.6% -> target 75%)
// =============================================================================

#[test]
fn test_stmt_if_else() {
    let code = transpile(
        "def check(x: int) -> str:\n    if x > 0:\n        return \"positive\"\n    else:\n        return \"non-positive\"",
    );
    assert!(
        code.contains("if") && code.contains("else"),
        "if-else: {}",
        code
    );
}

#[test]
fn test_stmt_elif_chain() {
    let code = transpile(
        "def grade(score: int) -> str:\n    if score >= 90:\n        return \"A\"\n    elif score >= 80:\n        return \"B\"\n    elif score >= 70:\n        return \"C\"\n    else:\n        return \"F\"",
    );
    assert!(!code.is_empty(), "elif chain: {}", code);
}

#[test]
fn test_stmt_while_loop() {
    let code = transpile(
        "def count_down(n: int) -> int:\n    total = 0\n    while n > 0:\n        total += n\n        n -= 1\n    return total",
    );
    assert!(code.contains("while"), "while loop: {}", code);
}

#[test]
fn test_stmt_for_range() {
    let code = transpile(
        "def total(n: int) -> int:\n    s = 0\n    for i in range(n):\n        s += i\n    return s",
    );
    assert!(code.contains("for"), "for range: {}", code);
}

#[test]
fn test_stmt_with_open() {
    let code = transpile(
        "def read_file(path: str) -> str:\n    with open(path) as f:\n        return f.read()",
    );
    assert!(!code.is_empty(), "with open: {}", code);
}

#[test]
fn test_stmt_try_except() {
    let code = transpile(
        "def safe_div(a: int, b: int) -> int:\n    try:\n        return a // b\n    except ZeroDivisionError:\n        return 0",
    );
    assert!(!code.is_empty(), "try-except: {}", code);
}

#[test]
fn test_stmt_try_except_as() {
    let code = transpile(
        "def safe(x: int) -> str:\n    try:\n        return str(x)\n    except Exception as e:\n        return str(e)",
    );
    assert!(!code.is_empty(), "try-except-as: {}", code);
}

#[test]
fn test_stmt_raise() {
    let code = transpile(
        "def validate(x: int):\n    if x < 0:\n        raise ValueError(\"negative\")",
    );
    assert!(!code.is_empty(), "raise: {}", code);
}

#[test]
fn test_stmt_assert() {
    let code =
        transpile("def check(x: int):\n    assert x > 0, \"must be positive\"");
    assert!(code.contains("assert"), "assert: {}", code);
}

#[test]
fn test_stmt_break_continue() {
    let code = transpile(
        "def find_first(items: list, target: int) -> int:\n    for i in range(len(items)):\n        if items[i] == target:\n            return i\n        if items[i] < 0:\n            continue\n    return -1",
    );
    assert!(
        code.contains("continue") || code.contains("return"),
        "break/continue: {}",
        code
    );
}

#[test]
fn test_stmt_nested_loops() {
    let code = transpile(
        "def matrix_sum(matrix: list) -> int:\n    total = 0\n    for row in matrix:\n        for val in row:\n            total += val\n    return total",
    );
    assert!(code.contains("for"), "nested loops: {}", code);
}

#[test]
fn test_stmt_while_break() {
    let code = transpile(
        "def search(items: list, target: int) -> bool:\n    i = 0\n    while i < len(items):\n        if items[i] == target:\n            break\n        i += 1\n    return i < len(items)",
    );
    assert!(
        code.contains("break") || code.contains("while"),
        "while break: {}",
        code
    );
}

// =============================================================================
// Section 7: convert_method_stmt (42.5% -> target 65%)
// =============================================================================

#[test]
fn test_method_stmt_self_assign() {
    let code = transpile(
        "class Counter:\n    def __init__(self):\n        self.count = 0\n    def increment(self):\n        self.count += 1",
    );
    assert!(
        code.contains("count") || code.contains("self"),
        "method self assign: {}",
        code
    );
}

#[test]
fn test_method_stmt_return() {
    let code = transpile(
        "class Box:\n    def __init__(self, value: int):\n        self.value = value\n    def get(self) -> int:\n        return self.value",
    );
    assert!(code.contains("value"), "method return: {}", code);
}

#[test]
fn test_method_stmt_conditional() {
    let code = transpile(
        "class Toggle:\n    def __init__(self):\n        self.on = False\n    def toggle(self) -> bool:\n        if self.on:\n            self.on = False\n        else:\n            self.on = True\n        return self.on",
    );
    assert!(!code.is_empty(), "method conditional: {}", code);
}

#[test]
fn test_method_stmt_loop() {
    let code = transpile(
        "class Accumulator:\n    def __init__(self):\n        self.total = 0\n    def add_all(self, items: list):\n        for item in items:\n            self.total += item",
    );
    assert!(code.contains("for"), "method loop: {}", code);
}

#[test]
fn test_method_stmt_list_append() {
    let code = transpile(
        "class Stack:\n    def __init__(self):\n        self.items: list = []\n    def push(self, item: int):\n        self.items.append(item)",
    );
    assert!(code.contains("push"), "method list append: {}", code);
}

// =============================================================================
// Supplemental: transpile_ok smoke tests for edge cases
// =============================================================================

#[test]
fn test_transpile_ok_empty_function() {
    assert!(transpile_ok("def noop():\n    pass"));
}

#[test]
fn test_transpile_ok_class_with_methods() {
    assert!(transpile_ok(
        "class Pair:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y\n    def sum(self) -> int:\n        return self.x + self.y"
    ));
}
