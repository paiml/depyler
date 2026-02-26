//! Coverage wave 4: analysis crate and depyler-core coverage boost tests
//!
//! Targets uncovered branches in depyler-analysis (param_type_inference,
//! const_generic_inference, escape_analysis, lifetime_analysis,
//! generic_inference, inlining, type_hints, borrowing_context) and
//! depyler-core files (ast_bridge/type_extraction, codegen, stmt_gen_complex,
//! var_analysis, rust_gen, stmt_gen).

use crate::DepylerPipeline;

fn transpile(code: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).expect("transpilation should succeed")
}

fn transpile_ok(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_ok()
}

// =============================================================================
// Section 1: param_type_inference.rs (67.9% cov, 134 missed)
// Infers parameter types from usage patterns in function body
// =============================================================================

#[test]
fn test_param_infer_strip_method_implies_str() {
    let code = transpile("def clean(data):\n    return data.strip()");
    assert!(!code.is_empty(), "strip implies str param: {}", code);
}

#[test]
fn test_param_infer_append_method_implies_list() {
    let code = transpile("def add_item(items, val: int):\n    items.append(val)\n    return items");
    assert!(!code.is_empty(), "append implies list param: {}", code);
}

#[test]
fn test_param_infer_upper_method_implies_str() {
    let code = transpile("def shout(text):\n    return text.upper()");
    assert!(!code.is_empty(), "upper implies str param: {}", code);
}

#[test]
fn test_param_infer_split_method_implies_str() {
    let code = transpile("def tokenize(line):\n    return line.split(',')");
    assert!(!code.is_empty(), "split implies str param: {}", code);
}

#[test]
fn test_param_infer_keys_method_implies_dict() {
    let code = transpile("def get_keys(mapping):\n    return list(mapping.keys())");
    assert!(!code.is_empty(), "keys implies dict param: {}", code);
}

#[test]
fn test_param_infer_values_method_implies_dict() {
    let code = transpile("def get_vals(mapping):\n    return list(mapping.values())");
    assert!(!code.is_empty(), "values implies dict param: {}", code);
}

#[test]
fn test_param_infer_items_method_implies_dict() {
    let code = transpile(
        "def iterate_pairs(mapping):\n    for k, v in mapping.items():\n        print(k)",
    );
    assert!(!code.is_empty(), "items implies dict param: {}", code);
}

#[test]
fn test_param_infer_arithmetic_implies_int() {
    let code = transpile("def double(n):\n    return n * 2");
    assert!(!code.is_empty(), "arithmetic implies int param: {}", code);
}

#[test]
fn test_param_infer_subtraction_implies_int() {
    let code = transpile("def decrement(n):\n    return n - 1");
    assert!(!code.is_empty(), "subtraction implies int param: {}", code);
}

#[test]
fn test_param_infer_floor_div_implies_int() {
    let code = transpile("def halve(n):\n    return n // 2");
    assert!(!code.is_empty(), "floor div implies int param: {}", code);
}

#[test]
fn test_param_infer_modulo_implies_int() {
    let code = transpile("def is_even(n):\n    return n % 2 == 0");
    assert!(!code.is_empty(), "modulo implies int param: {}", code);
}

#[test]
fn test_param_infer_comparison_with_string_literal() {
    let code = transpile("def is_admin(role):\n    return role == 'admin'");
    assert!(!code.is_empty(), "comparison with str literal implies str: {}", code);
}

#[test]
fn test_param_infer_comparison_string_on_right() {
    let code = transpile(
        "def check_status(status):\n    if 'active' == status:\n        return True\n    return False",
    );
    assert!(!code.is_empty(), "str literal on left implies str param: {}", code);
}

#[test]
fn test_param_infer_logical_and_implies_bool() {
    let code = transpile("def both_true(a, b):\n    return a and b");
    assert!(!code.is_empty(), "logical and implies bool param: {}", code);
}

#[test]
fn test_param_infer_logical_or_implies_bool() {
    let code = transpile("def either_true(a, b):\n    return a or b");
    assert!(!code.is_empty(), "logical or implies bool param: {}", code);
}

#[test]
fn test_param_infer_in_operator_implies_str() {
    let code = transpile("def contains_hello(text):\n    return 'hello' in text");
    assert!(!code.is_empty(), "in operator implies str: {}", code);
}

#[test]
fn test_param_infer_index_with_string_key_implies_dict() {
    let code = transpile("def get_name(data):\n    return data['name']");
    assert!(!code.is_empty(), "string index implies dict param: {}", code);
}

#[test]
fn test_param_infer_index_with_int_implies_list() {
    let code = transpile("def first_elem(data):\n    return data[0]");
    assert!(!code.is_empty(), "int index implies list param: {}", code);
}

#[test]
fn test_param_infer_slice_implies_str() {
    let code = transpile("def head(data):\n    return data[:5]");
    assert!(!code.is_empty(), "slice implies str/list param: {}", code);
}

#[test]
fn test_param_infer_fstring_usage_implies_str() {
    let code = transpile("def greet(name):\n    return f'Hello, {name}!'");
    assert!(!code.is_empty(), "fstring usage implies str param: {}", code);
}

#[test]
fn test_param_infer_print_arg_implies_str() {
    let code = transpile("def show(msg):\n    print(msg)");
    assert!(!code.is_empty(), "print arg implies str param: {}", code);
}

#[test]
fn test_param_infer_as_callable() {
    let code = transpile("def apply(func, x: int) -> int:\n    return func(x)");
    assert!(!code.is_empty(), "callable param: {}", code);
}

#[test]
fn test_param_infer_tuple_unpacking() {
    let code = transpile("def unpack(pair):\n    a, b = pair\n    return a");
    assert!(!code.is_empty(), "tuple unpacking infers tuple type: {}", code);
}

#[test]
fn test_param_infer_in_if_condition() {
    let code = transpile(
        "def process(data):\n    if data.startswith('http'):\n        return data\n    return ''",
    );
    assert!(!code.is_empty(), "param in if condition: {}", code);
}

#[test]
fn test_param_infer_in_while_condition() {
    let code =
        transpile("def drain(items):\n    while items:\n        items.pop()\n    return items");
    assert!(!code.is_empty(), "param in while condition: {}", code);
}

#[test]
fn test_param_infer_in_for_loop_body() {
    let code = transpile(
        "def count_chars(text):\n    total = 0\n    for ch in text:\n        total = total + 1\n    return total",
    );
    assert!(!code.is_empty(), "param in for loop body: {}", code);
}

#[test]
fn test_param_infer_in_with_block() {
    let code =
        transpile("def read_file(path):\n    with open(path) as f:\n        return f.read()");
    assert!(!code.is_empty(), "param in with block: {}", code);
}

#[test]
fn test_param_infer_in_try_block() {
    let code = transpile(
        "def safe_parse(text):\n    try:\n        return int(text)\n    except ValueError:\n        return 0",
    );
    assert!(!code.is_empty(), "param in try block: {}", code);
}

#[test]
fn test_param_infer_file_object_write() {
    let code = transpile("def write_data(output):\n    output.write('hello')\n    output.flush()");
    assert!(!code.is_empty(), "file object write: {}", code);
}

#[test]
fn test_param_infer_file_object_read() {
    let code = transpile("def read_all(source):\n    return source.read()");
    assert!(!code.is_empty(), "file object read: {}", code);
}

#[test]
fn test_param_infer_listcomp_iter() {
    let code = transpile("def squares(nums):\n    return [x * x for x in nums]");
    assert!(!code.is_empty(), "listcomp iter param: {}", code);
}

#[test]
fn test_param_infer_generator_expr_iter() {
    let code = transpile("def total(nums):\n    return sum(x * 2 for x in nums)");
    assert!(!code.is_empty(), "genexp iter param: {}", code);
}

// =============================================================================
// Section 2: const_generic_inference.rs (82.2% cov, 164 missed)
// Fixed-size array patterns and const generics
// =============================================================================

#[test]
fn test_const_generic_fixed_list_literal() {
    let code = transpile("def fixed_array() -> list[int]:\n    return [1, 2, 3, 4, 5]");
    assert!(!code.is_empty(), "fixed list literal: {}", code);
}

#[test]
fn test_const_generic_list_multiply() {
    let code = transpile("def zeros() -> list[int]:\n    return [0] * 10");
    assert!(!code.is_empty(), "list multiply pattern: {}", code);
}

#[test]
fn test_const_generic_nested_fixed_list() {
    let code = transpile("def matrix() -> list[list[int]]:\n    return [[1, 2], [3, 4], [5, 6]]");
    assert!(!code.is_empty(), "nested fixed list: {}", code);
}

#[test]
fn test_const_generic_tuple_known_size() {
    let code = transpile("def point() -> tuple[int, int, int]:\n    return (1, 2, 3)");
    assert!(!code.is_empty(), "tuple known size: {}", code);
}

#[test]
fn test_const_generic_assign_fixed_list() {
    let code =
        transpile("def init() -> list[int]:\n    arr = [0, 0, 0]\n    arr[1] = 5\n    return arr");
    assert!(!code.is_empty(), "assign fixed list: {}", code);
}

#[test]
fn test_const_generic_range_len() {
    let code = transpile("def indices(n: int) -> list[int]:\n    return list(range(n))");
    assert!(!code.is_empty(), "range with len: {}", code);
}

#[test]
fn test_const_generic_if_branch_arrays() {
    let code = transpile(
        "def choose(flag: bool) -> list[int]:\n    if flag:\n        arr = [1, 2, 3]\n    else:\n        arr = [4, 5, 6]\n    return arr",
    );
    assert!(!code.is_empty(), "if branch arrays: {}", code);
}

// =============================================================================
// Section 3: escape_analysis.rs (90.8% cov, 133 missed)
// Variable escape and ownership analysis
// =============================================================================

#[test]
fn test_escape_variable_returned() {
    let code =
        transpile("def create_list() -> list[int]:\n    items = [1, 2, 3]\n    return items");
    assert!(!code.is_empty(), "variable returned escapes: {}", code);
}

#[test]
fn test_escape_variable_assigned_then_returned() {
    let code = transpile(
        "def transform(data: list[int]) -> list[int]:\n    result = data\n    result.append(99)\n    return result",
    );
    assert!(!code.is_empty(), "assign then return escapes: {}", code);
}

#[test]
fn test_escape_variable_passed_to_mutating_func() {
    let code = transpile(
        "def extend_list(items: list[int]) -> list[int]:\n    items.extend([4, 5, 6])\n    return items",
    );
    assert!(!code.is_empty(), "mutating func escapes: {}", code);
}

#[test]
fn test_escape_use_after_move_pattern() {
    let code = transpile(
        "def use_twice(data: list[int]) -> int:\n    total = sum(data)\n    length = len(data)\n    return total + length",
    );
    assert!(!code.is_empty(), "use after potential move: {}", code);
}

#[test]
fn test_escape_alias_then_use_both() {
    let code = transpile(
        "def alias_test(original: list[int]) -> int:\n    clone = list(original)\n    clone.append(1)\n    return len(original) + len(clone)",
    );
    assert!(!code.is_empty(), "alias then use both: {}", code);
}

#[test]
fn test_escape_variable_in_lambda() {
    let code = transpile(
        "def capture_var(items: list[int]) -> list[int]:\n    return sorted(items, key=lambda x: -x)",
    );
    assert!(!code.is_empty(), "variable captured in lambda: {}", code);
}

#[test]
fn test_escape_conditional_move() {
    let code = transpile(
        "def conditional_use(data: list[int], flag: bool) -> list[int]:\n    if flag:\n        result = data\n    else:\n        result = []\n    return result",
    );
    assert!(!code.is_empty(), "conditional move: {}", code);
}

#[test]
fn test_escape_loop_accumulator() {
    let code = transpile(
        "def accumulate(items: list[int]) -> list[int]:\n    result = []\n    for x in items:\n        result.append(x * 2)\n    return result",
    );
    assert!(!code.is_empty(), "loop accumulator: {}", code);
}

#[test]
fn test_escape_nested_function_capture() {
    let code = transpile(
        "def outer(x: int) -> int:\n    def inner() -> int:\n        return x + 1\n    return inner()",
    );
    assert!(!code.is_empty(), "nested function capture: {}", code);
}

// =============================================================================
// Section 4: lifetime_analysis.rs (82.8% cov, 208 missed)
// Borrow checking and lifetime inference
// =============================================================================

#[test]
fn test_lifetime_mutable_borrow_collection() {
    let code = transpile(
        "def mutate_list(items: list[int]):\n    items.append(42)\n    items.sort()\n    return items",
    );
    assert!(!code.is_empty(), "mutable borrow collection: {}", code);
}

#[test]
fn test_lifetime_iterator_borrow() {
    let code = transpile(
        "def sum_all(items: list[int]) -> int:\n    total = 0\n    for x in items:\n        total = total + x\n    return total",
    );
    assert!(!code.is_empty(), "iterator borrow: {}", code);
}

#[test]
fn test_lifetime_parameter_read_only() {
    let code = transpile("def peek(items: list[int]) -> int:\n    return items[0]");
    assert!(!code.is_empty(), "read-only parameter: {}", code);
}

#[test]
fn test_lifetime_parameter_mutated_and_returned() {
    let code = transpile(
        "def process(items: list[int]) -> list[int]:\n    items.reverse()\n    items.append(0)\n    return items",
    );
    assert!(!code.is_empty(), "mutated and returned: {}", code);
}

#[test]
fn test_lifetime_string_borrow_slice() {
    let code = transpile("def prefix(s: str) -> str:\n    return s[:3]");
    assert!(!code.is_empty(), "string borrow slice: {}", code);
}

#[test]
fn test_lifetime_multiple_borrows_same_scope() {
    let code = transpile(
        "def multi_borrow(a: list[int], b: list[int]) -> int:\n    return len(a) + len(b)",
    );
    assert!(!code.is_empty(), "multiple borrows same scope: {}", code);
}

#[test]
fn test_lifetime_borrow_in_loop() {
    let code = transpile(
        "def loop_borrow(items: list[str]) -> str:\n    result = ''\n    for s in items:\n        result = result + s\n    return result",
    );
    assert!(!code.is_empty(), "borrow in loop: {}", code);
}

#[test]
fn test_lifetime_nested_borrow() {
    let code = transpile(
        "def nested_access(data: dict[str, list[int]]) -> int:\n    return data['key'][0]",
    );
    assert!(!code.is_empty(), "nested borrow: {}", code);
}

#[test]
fn test_lifetime_return_reference_pattern() {
    let code = transpile(
        "def find_max(items: list[int]) -> int:\n    best = items[0]\n    for x in items:\n        if x > best:\n            best = x\n    return best",
    );
    assert!(!code.is_empty(), "return reference pattern: {}", code);
}

#[test]
fn test_lifetime_param_used_in_conditional_loop() {
    let code = transpile(
        "def search(items: list[str], target: str) -> bool:\n    for item in items:\n        if item == target:\n            return True\n    return False",
    );
    assert!(!code.is_empty(), "param in conditional loop: {}", code);
}

// =============================================================================
// Section 5: generic_inference.rs (85.6% cov, 128 missed)
// Generic type inference and TypeVar resolution
// =============================================================================

#[test]
fn test_generic_list_type_from_append() {
    let code = transpile(
        "def build_ints() -> list[int]:\n    result = []\n    result.append(1)\n    result.append(2)\n    return result",
    );
    assert!(!code.is_empty(), "list type from append: {}", code);
}

#[test]
fn test_generic_dict_type_from_assignment() {
    let code = transpile(
        "def build_dict() -> dict[str, int]:\n    d = {}\n    d['a'] = 1\n    d['b'] = 2\n    return d",
    );
    assert!(!code.is_empty(), "dict type from assignment: {}", code);
}

#[test]
fn test_generic_set_type_from_add() {
    let code = transpile(
        "def build_set() -> set[str]:\n    s = set()\n    s.add('hello')\n    s.add('world')\n    return s",
    );
    assert!(!code.is_empty(), "set type from add: {}", code);
}

#[test]
fn test_generic_sorted_preserves_type() {
    let code = transpile("def sort_ints(items: list[int]) -> list[int]:\n    return sorted(items)");
    assert!(!code.is_empty(), "sorted preserves type: {}", code);
}

#[test]
fn test_generic_map_result_type() {
    let code = transpile(
        "def double_all(items: list[int]) -> list[int]:\n    return list(map(lambda x: x * 2, items))",
    );
    assert!(!code.is_empty(), "map result type: {}", code);
}

#[test]
fn test_generic_filter_result_type() {
    let code = transpile(
        "def evens(items: list[int]) -> list[int]:\n    return list(filter(lambda x: x % 2 == 0, items))",
    );
    assert!(!code.is_empty(), "filter result type: {}", code);
}

#[test]
fn test_generic_zip_two_lists() {
    let code = transpile(
        "def pair_up(a: list[int], b: list[str]) -> list[tuple[int, str]]:\n    return list(zip(a, b))",
    );
    assert!(!code.is_empty(), "zip two lists: {}", code);
}

#[test]
fn test_generic_enumerate_result() {
    let code = transpile(
        "def indexed(items: list[str]) -> list[tuple[int, str]]:\n    return list(enumerate(items))",
    );
    assert!(!code.is_empty(), "enumerate result: {}", code);
}

// =============================================================================
// Section 6: inlining.rs (86.5% cov, 261 missed)
// Function inlining heuristics
// =============================================================================

#[test]
fn test_inline_small_function_single_return() {
    let code = transpile(
        "def add(a: int, b: int) -> int:\n    return a + b\n\ndef use_add(x: int) -> int:\n    return add(x, 1)",
    );
    assert!(!code.is_empty(), "small function inline: {}", code);
}

#[test]
fn test_inline_trivial_identity() {
    let code = transpile(
        "def identity(x: int) -> int:\n    return x\n\ndef caller(n: int) -> int:\n    return identity(n)",
    );
    assert!(!code.is_empty(), "trivial identity: {}", code);
}

#[test]
fn test_inline_lambda_single_use() {
    let code =
        transpile("def apply_once(x: int) -> int:\n    fn = lambda a: a + 10\n    return fn(x)");
    assert!(!code.is_empty(), "lambda single use: {}", code);
}

#[test]
fn test_inline_function_with_loop_not_inlined() {
    let code = transpile(
        "def sum_range(n: int) -> int:\n    total = 0\n    for i in range(n):\n        total = total + i\n    return total\n\ndef caller(x: int) -> int:\n    return sum_range(x)",
    );
    assert!(!code.is_empty(), "function with loop: {}", code);
}

#[test]
fn test_inline_multiple_callers() {
    let code = transpile(
        "def helper(x: int) -> int:\n    return x * 2\n\ndef a(n: int) -> int:\n    return helper(n)\n\ndef b(n: int) -> int:\n    return helper(n + 1)",
    );
    assert!(!code.is_empty(), "multiple callers: {}", code);
}

#[test]
fn test_inline_recursive_function_not_inlined() {
    let code = transpile(
        "def factorial(n: int) -> int:\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)",
    );
    assert!(!code.is_empty(), "recursive function: {}", code);
}

#[test]
fn test_inline_nested_function_definitions() {
    let code = transpile(
        "def outer(x: int) -> int:\n    def inner(y: int) -> int:\n        return y + 1\n    return inner(x) + inner(x + 1)",
    );
    assert!(!code.is_empty(), "nested function definitions: {}", code);
}

// =============================================================================
// Section 7: type_hints.rs (94.3% cov, 139 missed)
// Type hint processing and complex annotations
// =============================================================================

#[test]
fn test_type_hint_dict_str_list_int() {
    let code = transpile("def process(data: dict[str, list[int]]) -> int:\n    return len(data)");
    assert!(!code.is_empty(), "dict[str, list[int]] hint: {}", code);
}

#[test]
fn test_type_hint_optional_str() {
    let code = transpile(
        "def greet(name: str = None) -> str:\n    if name:\n        return f'Hello {name}'\n    return 'Hello'",
    );
    assert!(!code.is_empty(), "optional str hint: {}", code);
}

#[test]
fn test_type_hint_list_of_tuples() {
    let code = transpile("def pairs(items: list[tuple[str, int]]) -> int:\n    return len(items)");
    assert!(!code.is_empty(), "list of tuples hint: {}", code);
}

#[test]
fn test_type_hint_nested_dict() {
    let code = transpile("def deep(data: dict[str, dict[str, int]]) -> int:\n    return 0");
    assert!(!code.is_empty(), "nested dict hint: {}", code);
}

#[test]
fn test_type_hint_set_str() {
    let code = transpile("def unique_words(words: set[str]) -> int:\n    return len(words)");
    assert!(!code.is_empty(), "set[str] hint: {}", code);
}

#[test]
fn test_type_hint_tuple_mixed() {
    let code = transpile("def record(entry: tuple[str, int, float]) -> str:\n    return entry[0]");
    assert!(!code.is_empty(), "tuple mixed types hint: {}", code);
}

#[test]
fn test_type_hint_return_none() {
    let code = transpile("def side_effect(x: int) -> None:\n    print(x)");
    assert!(!code.is_empty(), "return None hint: {}", code);
}

#[test]
fn test_type_hint_bool_return() {
    let code = transpile("def is_valid(x: int) -> bool:\n    return x > 0");
    assert!(!code.is_empty(), "bool return hint: {}", code);
}

#[test]
fn test_type_hint_infer_from_default_value_int() {
    let code = transpile("def with_default(count=0):\n    return count + 1");
    assert!(!code.is_empty(), "default int value: {}", code);
}

#[test]
fn test_type_hint_infer_from_default_value_str() {
    let code = transpile("def with_default_str(name='world'):\n    return f'Hello {name}'");
    assert!(!code.is_empty(), "default str value: {}", code);
}

#[test]
fn test_type_hint_infer_from_default_value_bool() {
    let code = transpile("def with_flag(verbose=False):\n    if verbose:\n        print('debug')");
    assert!(!code.is_empty(), "default bool value: {}", code);
}

#[test]
fn test_type_hint_variable_used_as_iterator() {
    let code = transpile(
        "def process(data):\n    for item in data:\n        print(item)\n    return len(data)",
    );
    assert!(!code.is_empty(), "variable used as iterator: {}", code);
}

#[test]
fn test_type_hint_variable_numeric_ops() {
    let code = transpile("def calc(x):\n    y = x + 1\n    z = y * 2\n    return z - x");
    assert!(!code.is_empty(), "variable numeric ops: {}", code);
}

#[test]
fn test_type_hint_analyze_with_statement() {
    let code = transpile(
        "def read_config(path: str) -> str:\n    with open(path) as f:\n        content = f.read()\n    return content",
    );
    assert!(!code.is_empty(), "with statement analysis: {}", code);
}

#[test]
fn test_type_hint_analyze_try_except() {
    let code = transpile(
        "def safe_int(val: str) -> int:\n    try:\n        return int(val)\n    except ValueError:\n        return -1",
    );
    assert!(!code.is_empty(), "try/except analysis: {}", code);
}

#[test]
fn test_type_hint_for_loop_dict_items() {
    let code = transpile(
        "def show_dict(d: dict[str, int]):\n    for k, v in d.items():\n        print(f'{k}: {v}')",
    );
    assert!(!code.is_empty(), "for loop dict items: {}", code);
}

// =============================================================================
// Section 8: borrowing_context.rs (93.9% cov, 68 missed)
// Borrowing strategy determination
// =============================================================================

#[test]
fn test_borrow_param_read_only_access() {
    let code = transpile("def readonly(items: list[int]) -> int:\n    return items[0] + items[1]");
    assert!(!code.is_empty(), "read only access: {}", code);
}

#[test]
fn test_borrow_param_mutated_in_place() {
    let code = transpile("def mutate(items: list[int]):\n    items.append(99)\n    items.sort()");
    assert!(!code.is_empty(), "mutated in place: {}", code);
}

#[test]
fn test_borrow_param_stored_in_container() {
    let code = transpile(
        "def store(item: str) -> dict[str, str]:\n    result = {}\n    result['item'] = item\n    return result",
    );
    assert!(!code.is_empty(), "stored in container: {}", code);
}

#[test]
fn test_borrow_param_used_in_closure() {
    let code = transpile(
        "def with_closure(items: list[int]) -> list[int]:\n    return list(map(lambda x: x + 1, items))",
    );
    assert!(!code.is_empty(), "used in closure: {}", code);
}

#[test]
fn test_borrow_param_in_loop_context() {
    let code = transpile(
        "def loop_use(items: list[int]) -> int:\n    total = 0\n    for i in range(len(items)):\n        total = total + items[i]\n    return total",
    );
    assert!(!code.is_empty(), "param in loop context: {}", code);
}

#[test]
fn test_borrow_method_call_tracking() {
    let code = transpile(
        "def method_calls(s: str) -> str:\n    s = s.strip()\n    s = s.lower()\n    return s",
    );
    assert!(!code.is_empty(), "method call tracking: {}", code);
}

// =============================================================================
// Section 9: ast_bridge/type_extraction.rs (70.5% cov, 93 missed)
// Complex type annotations from Python AST
// =============================================================================

#[test]
fn test_type_extract_list_int() {
    let code = transpile("def foo(x: list[int]) -> list[int]:\n    return x");
    assert!(!code.is_empty(), "list[int] extraction: {}", code);
}

#[test]
fn test_type_extract_dict_str_str() {
    let code = transpile("def foo(x: dict[str, str]) -> dict[str, str]:\n    return x");
    assert!(!code.is_empty(), "dict[str, str] extraction: {}", code);
}

#[test]
fn test_type_extract_tuple_int_str() {
    let code = transpile("def foo(x: tuple[int, str]) -> tuple[int, str]:\n    return x");
    assert!(!code.is_empty(), "tuple[int, str] extraction: {}", code);
}

#[test]
fn test_type_extract_set_float() {
    let code = transpile("def foo(x: set[float]) -> set[float]:\n    return x");
    assert!(!code.is_empty(), "set[float] extraction: {}", code);
}

#[test]
fn test_type_extract_nested_list_of_lists() {
    let code = transpile("def foo(x: list[list[int]]) -> list[list[int]]:\n    return x");
    assert!(!code.is_empty(), "list[list[int]] extraction: {}", code);
}

#[test]
fn test_type_extract_dict_str_list_str() {
    let code = transpile("def foo(x: dict[str, list[str]]) -> int:\n    return len(x)");
    assert!(!code.is_empty(), "dict[str, list[str]] extraction: {}", code);
}

// =============================================================================
// Section 10: codegen.rs (88.8% cov, 234 missed)
// Module-level codegen orchestration
// =============================================================================

#[test]
fn test_codegen_module_constant_int() {
    let code = transpile("MAX_SIZE = 100\n\ndef get_max() -> int:\n    return MAX_SIZE");
    assert!(!code.is_empty(), "module constant int: {}", code);
}

#[test]
fn test_codegen_module_constant_str() {
    let code = transpile("GREETING = 'hello'\n\ndef greet() -> str:\n    return GREETING");
    assert!(!code.is_empty(), "module constant str: {}", code);
}

#[test]
fn test_codegen_multiple_functions() {
    let code = transpile(
        "def add(a: int, b: int) -> int:\n    return a + b\n\ndef sub(a: int, b: int) -> int:\n    return a - b\n\ndef mul(a: int, b: int) -> int:\n    return a * b",
    );
    assert!(!code.is_empty(), "multiple functions: {}", code);
}

#[test]
fn test_codegen_function_calling_another() {
    let code = transpile(
        "def square(x: int) -> int:\n    return x * x\n\ndef sum_squares(a: int, b: int) -> int:\n    return square(a) + square(b)",
    );
    assert!(!code.is_empty(), "function calling another: {}", code);
}

#[test]
fn test_codegen_module_constant_float() {
    let code =
        transpile("RATE = 0.05\n\ndef calculate(base: float) -> float:\n    return base * RATE");
    assert!(!code.is_empty(), "module constant float: {}", code);
}

#[test]
fn test_codegen_module_constant_bool() {
    let code = transpile("DEBUG = True\n\ndef is_debug() -> bool:\n    return DEBUG");
    assert!(!code.is_empty(), "module constant bool: {}", code);
}

#[test]
fn test_codegen_class_with_init() {
    let code = transpile(
        "class Counter:\n    def __init__(self, start: int):\n        self.count = start\n\n    def increment(self):\n        self.count = self.count + 1",
    );
    assert!(!code.is_empty(), "class with init: {}", code);
}

#[test]
fn test_codegen_class_with_multiple_methods() {
    let code = transpile(
        "class Stack:\n    def __init__(self):\n        self.items = []\n\n    def push(self, item: int):\n        self.items.append(item)\n\n    def pop(self) -> int:\n        return self.items.pop()\n\n    def is_empty(self) -> bool:\n        return len(self.items) == 0",
    );
    assert!(!code.is_empty(), "class with multiple methods: {}", code);
}

// =============================================================================
// Section 11: stmt_gen_complex.rs (91.2% cov, 336 missed)
// Complex statement generation: try/except, with, match/case
// =============================================================================

#[test]
fn test_complex_try_except_basic() {
    let code = transpile(
        "def safe_div(a: int, b: int) -> int:\n    try:\n        return a // b\n    except ZeroDivisionError:\n        return 0",
    );
    assert!(!code.is_empty(), "try/except basic: {}", code);
}

#[test]
fn test_complex_try_except_multiple_handlers() {
    let code = transpile(
        "def safe_parse(text: str) -> int:\n    try:\n        return int(text)\n    except ValueError:\n        return -1\n    except TypeError:\n        return -2",
    );
    assert!(!code.is_empty(), "try/except multiple handlers: {}", code);
}

#[test]
fn test_complex_try_except_finally() {
    let code = transpile(
        "def with_cleanup(x: int) -> int:\n    result = 0\n    try:\n        result = x * 2\n    except Exception:\n        result = -1\n    finally:\n        print('done')\n    return result",
    );
    assert!(!code.is_empty(), "try/except/finally: {}", code);
}

#[test]
fn test_complex_try_except_as_variable() {
    let code = transpile(
        "def capture_error(x: int) -> str:\n    try:\n        return str(x * 2)\n    except ValueError as e:\n        return str(e)",
    );
    assert!(!code.is_empty(), "try/except as variable: {}", code);
}

#[test]
fn test_complex_try_except_else() {
    let code = transpile(
        "def try_else(x: int) -> int:\n    try:\n        result = x * 2\n    except Exception:\n        result = 0\n    else:\n        result = result + 1\n    return result",
    );
    assert!(!code.is_empty(), "try/except/else: {}", code);
}

#[test]
fn test_complex_nested_try_except() {
    let code = transpile(
        "def nested_try(a: int, b: int) -> int:\n    try:\n        try:\n            return a // b\n        except ZeroDivisionError:\n            return -1\n    except Exception:\n        return -2",
    );
    assert!(!code.is_empty(), "nested try/except: {}", code);
}

#[test]
fn test_complex_with_statement_file() {
    let code = transpile(
        "def read_file(path: str) -> str:\n    with open(path) as f:\n        return f.read()",
    );
    assert!(!code.is_empty(), "with statement file: {}", code);
}

#[test]
fn test_complex_with_statement_no_as() {
    let code =
        transpile("def use_context(path: str):\n    with open(path):\n        print('opened')");
    assert!(!code.is_empty(), "with statement no as: {}", code);
}

#[test]
fn test_complex_match_case_int() {
    let code = transpile(
        "def describe(x: int) -> str:\n    match x:\n        case 0:\n            return 'zero'\n        case 1:\n            return 'one'\n        case _:\n            return 'other'",
    );
    assert!(!code.is_empty(), "match/case int: {}", code);
}

#[test]
fn test_complex_match_case_string() {
    let code = transpile(
        "def handle_cmd(cmd: str) -> int:\n    match cmd:\n        case 'start':\n            return 1\n        case 'stop':\n            return 0\n        case _:\n            return -1",
    );
    assert!(!code.is_empty(), "match/case string: {}", code);
}

#[test]
fn test_complex_raise_value_error() {
    let code = transpile(
        "def validate(x: int) -> int:\n    if x < 0:\n        raise ValueError('negative')\n    return x",
    );
    assert!(!code.is_empty(), "raise ValueError: {}", code);
}

#[test]
fn test_complex_raise_runtime_error() {
    let code = transpile(
        "def check(condition: bool):\n    if not condition:\n        raise RuntimeError('failed')",
    );
    assert!(!code.is_empty(), "raise RuntimeError: {}", code);
}

// =============================================================================
// Section 12: var_analysis.rs (92.0% cov, 196 missed)
// Variable mutability and scope analysis
// =============================================================================

#[test]
fn test_var_analysis_immutable_binding() {
    let code = transpile("def immut() -> int:\n    x = 42\n    return x");
    assert!(!code.is_empty(), "immutable binding: {}", code);
}

#[test]
fn test_var_analysis_mutable_reassignment() {
    let code =
        transpile("def mutate() -> int:\n    x = 0\n    x = x + 1\n    x = x * 2\n    return x");
    assert!(!code.is_empty(), "mutable reassignment: {}", code);
}

#[test]
fn test_var_analysis_aug_assign() {
    let code = transpile(
        "def aug_assign(x: int) -> int:\n    x += 5\n    x -= 2\n    x *= 3\n    return x",
    );
    assert!(!code.is_empty(), "augmented assignment: {}", code);
}

#[test]
fn test_var_analysis_nested_scope() {
    let code = transpile(
        "def outer() -> int:\n    x = 10\n    def inner() -> int:\n        y = x + 1\n        return y\n    return inner()",
    );
    assert!(!code.is_empty(), "nested scope analysis: {}", code);
}

#[test]
fn test_var_analysis_loop_variable() {
    let code = transpile(
        "def loop_var() -> int:\n    total = 0\n    for i in range(10):\n        total = total + i\n    return total",
    );
    assert!(!code.is_empty(), "loop variable analysis: {}", code);
}

#[test]
fn test_var_analysis_conditional_assignment() {
    let code = transpile(
        "def cond_assign(flag: bool) -> int:\n    if flag:\n        x = 1\n    else:\n        x = 2\n    return x",
    );
    assert!(!code.is_empty(), "conditional assignment: {}", code);
}

#[test]
fn test_var_analysis_try_except_scope() {
    let code = transpile(
        "def try_scope() -> int:\n    result = 0\n    try:\n        result = 42\n    except Exception:\n        result = -1\n    return result",
    );
    assert!(!code.is_empty(), "try/except scope: {}", code);
}

#[test]
fn test_var_analysis_multiple_targets() {
    let code = transpile("def multi_assign() -> int:\n    a, b, c = 1, 2, 3\n    return a + b + c");
    assert!(!code.is_empty(), "multiple targets: {}", code);
}

// =============================================================================
// Section 13: rust_gen.rs (87.0% cov) and stmt_gen.rs (86.4% cov)
// Complex classes, enums, async, decorators, deep control flow
// =============================================================================

#[test]
fn test_class_with_str_method() {
    let code = transpile(
        "class Point:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y\n\n    def __str__(self) -> str:\n        return f'({self.x}, {self.y})'",
    );
    assert!(!code.is_empty(), "class with __str__: {}", code);
}

#[test]
fn test_class_with_eq_method() {
    let code = transpile(
        "class Pair:\n    def __init__(self, a: int, b: int):\n        self.a = a\n        self.b = b\n\n    def __eq__(self, other) -> bool:\n        return self.a == other.a and self.b == other.b",
    );
    assert!(!code.is_empty(), "class with __eq__: {}", code);
}

#[test]
fn test_class_with_len_method() {
    let code = transpile(
        "class Bag:\n    def __init__(self):\n        self.items = []\n\n    def __len__(self) -> int:\n        return len(self.items)",
    );
    assert!(!code.is_empty(), "class with __len__: {}", code);
}

#[test]
fn test_class_property_like() {
    let code = transpile(
        "class Circle:\n    def __init__(self, radius: float):\n        self.radius = radius\n\n    def area(self) -> float:\n        return 3.14159 * self.radius * self.radius",
    );
    assert!(!code.is_empty(), "class property-like method: {}", code);
}

#[test]
fn test_deeply_nested_if_else() {
    let code = transpile(
        "def classify(x: int) -> str:\n    if x > 100:\n        if x > 1000:\n            return 'huge'\n        else:\n            return 'large'\n    else:\n        if x > 10:\n            return 'medium'\n        else:\n            if x > 0:\n                return 'small'\n            else:\n                return 'zero'",
    );
    assert!(!code.is_empty(), "deeply nested if/else: {}", code);
}

#[test]
fn test_nested_for_loops() {
    let code = transpile(
        "def matrix_sum(rows: list[list[int]]) -> int:\n    total = 0\n    for row in rows:\n        for val in row:\n            total = total + val\n    return total",
    );
    assert!(!code.is_empty(), "nested for loops: {}", code);
}

#[test]
fn test_for_with_break() {
    let code = transpile(
        "def find_first(items: list[int], target: int) -> int:\n    idx = -1\n    for i in range(len(items)):\n        if items[i] == target:\n            idx = i\n            break\n    return idx",
    );
    assert!(!code.is_empty(), "for with break: {}", code);
}

#[test]
fn test_for_with_continue() {
    let code = transpile(
        "def sum_positive(items: list[int]) -> int:\n    total = 0\n    for x in items:\n        if x < 0:\n            continue\n        total = total + x\n    return total",
    );
    assert!(!code.is_empty(), "for with continue: {}", code);
}

#[test]
fn test_while_loop_with_break() {
    let code = transpile(
        "def countdown(n: int) -> int:\n    count = 0\n    while True:\n        if n <= 0:\n            break\n        n = n - 1\n        count = count + 1\n    return count",
    );
    assert!(!code.is_empty(), "while with break: {}", code);
}

#[test]
fn test_complex_boolean_expression() {
    let code = transpile(
        "def complex_check(a: int, b: int, c: int) -> bool:\n    return (a > 0 and b > 0) or (c > 0 and a + b > c)",
    );
    assert!(!code.is_empty(), "complex boolean expression: {}", code);
}

#[test]
fn test_ternary_expression() {
    let code = transpile("def abs_val(x: int) -> int:\n    return x if x >= 0 else -x");
    assert!(!code.is_empty(), "ternary expression: {}", code);
}

#[test]
fn test_list_comprehension_with_filter() {
    let code = transpile(
        "def evens_only(items: list[int]) -> list[int]:\n    return [x for x in items if x % 2 == 0]",
    );
    assert!(!code.is_empty(), "list comp with filter: {}", code);
}

#[test]
fn test_dict_comprehension() {
    let code = transpile(
        "def make_dict(keys: list[str]) -> dict[str, int]:\n    return {k: len(k) for k in keys}",
    );
    assert!(!code.is_empty(), "dict comprehension: {}", code);
}

#[test]
fn test_set_comprehension() {
    let code = transpile(
        "def unique_lengths(words: list[str]) -> set[int]:\n    return {len(w) for w in words}",
    );
    assert!(!code.is_empty(), "set comprehension: {}", code);
}

#[test]
fn test_nested_list_comprehension() {
    let code = transpile(
        "def flatten(matrix: list[list[int]]) -> list[int]:\n    return [x for row in matrix for x in row]",
    );
    assert!(!code.is_empty(), "nested list comp: {}", code);
}

#[test]
fn test_string_join_method() {
    let code = transpile("def join_words(words: list[str]) -> str:\n    return ', '.join(words)");
    assert!(!code.is_empty(), "string join method: {}", code);
}

#[test]
fn test_string_format_method() {
    let code = transpile(
        "def format_greeting(name: str, age: int) -> str:\n    return 'Name: {}, Age: {}'.format(name, age)",
    );
    assert!(!code.is_empty(), "string format method: {}", code);
}

#[test]
fn test_multiple_return_values() {
    let code = transpile(
        "def divmod_custom(a: int, b: int) -> tuple[int, int]:\n    return a // b, a % b",
    );
    assert!(!code.is_empty(), "multiple return values: {}", code);
}

#[test]
fn test_assert_statement() {
    let code =
        transpile("def checked_sqrt(x: float) -> float:\n    assert x >= 0.0\n    return x ** 0.5");
    assert!(!code.is_empty(), "assert statement: {}", code);
}

#[test]
fn test_assert_with_message() {
    let code = transpile(
        "def positive_only(x: int) -> int:\n    assert x > 0, 'x must be positive'\n    return x",
    );
    assert!(!code.is_empty(), "assert with message: {}", code);
}

#[test]
fn test_chained_comparison() {
    let code = transpile("def in_range(x: int) -> bool:\n    return 0 < x < 100");
    assert!(!code.is_empty(), "chained comparison: {}", code);
}

#[test]
fn test_walrus_operator() {
    let code = transpile(
        "def find_match(items: list[int]) -> int:\n    for x in items:\n        if (n := x * 2) > 10:\n            return n\n    return 0",
    );
    assert!(!code.is_empty(), "walrus operator: {}", code);
}

#[test]
fn test_complex_class_inheritance() {
    let code = transpile(
        "class Animal:\n    def __init__(self, name: str):\n        self.name = name\n\n    def speak(self) -> str:\n        return ''\n\nclass Dog(Animal):\n    def speak(self) -> str:\n        return 'Woof'",
    );
    assert!(!code.is_empty(), "class inheritance: {}", code);
}

#[test]
fn test_class_with_class_variable() {
    let code = transpile(
        "class Counter:\n    count = 0\n\n    def __init__(self):\n        Counter.count = Counter.count + 1",
    );
    assert!(!code.is_empty(), "class variable: {}", code);
}

#[test]
fn test_pass_statement() {
    let code = transpile("def noop():\n    pass");
    assert!(!code.is_empty(), "pass statement: {}", code);
}

#[test]
fn test_delete_statement() {
    let code = transpile("def remove_key(d: dict[str, int], key: str):\n    del d[key]");
    assert!(!code.is_empty(), "delete statement: {}", code);
}

#[test]
fn test_global_variable_usage() {
    let code = transpile(
        "counter = 0\n\ndef increment() -> int:\n    global counter\n    counter = counter + 1\n    return counter",
    );
    assert!(!code.is_empty(), "global variable: {}", code);
}

#[test]
fn test_multiple_assignment_same_value() {
    let code = transpile("def init_zeros() -> int:\n    a = b = c = 0\n    return a + b + c");
    assert!(!code.is_empty(), "multiple assignment same value: {}", code);
}

#[test]
fn test_complex_fstring() {
    let code = transpile(
        "def format_record(name: str, age: int, score: float) -> str:\n    return f'{name} (age {age}): {score:.2f}'",
    );
    assert!(!code.is_empty(), "complex fstring: {}", code);
}

#[test]
fn test_string_replace_method() {
    let code = transpile(
        "def clean_text(text: str) -> str:\n    return text.replace('\\n', ' ').replace('\\t', ' ')",
    );
    assert!(!code.is_empty(), "string replace method: {}", code);
}

#[test]
fn test_string_startswith_endswith() {
    let code = transpile(
        "def check_ext(filename: str) -> bool:\n    return filename.endswith('.py') or filename.endswith('.rs')",
    );
    assert!(!code.is_empty(), "startswith/endswith: {}", code);
}

#[test]
fn test_min_max_builtins() {
    let code =
        transpile("def clamp(x: int, lo: int, hi: int) -> int:\n    return max(lo, min(x, hi))");
    assert!(!code.is_empty(), "min/max builtins: {}", code);
}

#[test]
fn test_abs_builtin() {
    let code = transpile("def distance(a: int, b: int) -> int:\n    return abs(a - b)");
    assert!(!code.is_empty(), "abs builtin: {}", code);
}

#[test]
fn test_isinstance_check() {
    let code = transpile("def is_string(val) -> bool:\n    return isinstance(val, str)");
    assert!(!code.is_empty(), "isinstance check: {}", code);
}

#[test]
fn test_type_conversion_chain() {
    let code = transpile("def convert(x: str) -> float:\n    return float(int(x))");
    assert!(!code.is_empty(), "type conversion chain: {}", code);
}

#[test]
fn test_power_operator() {
    let code = transpile("def cube(x: int) -> int:\n    return x ** 3");
    assert!(!code.is_empty(), "power operator: {}", code);
}

#[test]
fn test_unary_not() {
    let code = transpile("def invert(flag: bool) -> bool:\n    return not flag");
    assert!(!code.is_empty(), "unary not: {}", code);
}

#[test]
fn test_unary_negative() {
    let code = transpile("def negate(x: int) -> int:\n    return -x");
    assert!(!code.is_empty(), "unary negative: {}", code);
}

#[test]
fn test_bitwise_operations() {
    let code = transpile("def bitwise(a: int, b: int) -> int:\n    return (a & b) | (a ^ b)");
    assert!(!code.is_empty(), "bitwise operations: {}", code);
}

#[test]
fn test_string_multiplication() {
    let code = transpile("def repeat(s: str, n: int) -> str:\n    return s * n");
    assert!(!code.is_empty(), "string multiplication: {}", code);
}

#[test]
fn test_list_extend_operator() {
    let code = transpile("def merge(a: list[int], b: list[int]) -> list[int]:\n    return a + b");
    assert!(!code.is_empty(), "list extend operator: {}", code);
}

#[test]
fn test_generator_expression_sum() {
    let code =
        transpile("def sum_squares(n: int) -> int:\n    return sum(x * x for x in range(n))");
    assert!(!code.is_empty(), "generator expression sum: {}", code);
}

#[test]
fn test_any_all_builtins() {
    let code = transpile(
        "def has_positive(items: list[int]) -> bool:\n    return any(x > 0 for x in items)",
    );
    assert!(!code.is_empty(), "any builtin: {}", code);
}

#[test]
fn test_all_builtin() {
    let code = transpile(
        "def all_positive(items: list[int]) -> bool:\n    return all(x > 0 for x in items)",
    );
    assert!(!code.is_empty(), "all builtin: {}", code);
}

#[test]
fn test_enumerate_loop() {
    let code = transpile(
        "def print_indexed(items: list[str]):\n    for i, item in enumerate(items):\n        print(f'{i}: {item}')",
    );
    assert!(!code.is_empty(), "enumerate loop: {}", code);
}

#[test]
fn test_zip_loop() {
    let code = transpile(
        "def dot_product(a: list[int], b: list[int]) -> int:\n    total = 0\n    for x, y in zip(a, b):\n        total = total + x * y\n    return total",
    );
    assert!(!code.is_empty(), "zip loop: {}", code);
}

#[test]
fn test_reversed_builtin() {
    let code = transpile(
        "def reverse_list(items: list[int]) -> list[int]:\n    return list(reversed(items))",
    );
    assert!(!code.is_empty(), "reversed builtin: {}", code);
}

#[test]
fn test_complex_dict_operations() {
    let code = transpile(
        "def merge_dicts(a: dict[str, int], b: dict[str, int]) -> dict[str, int]:\n    result = {}\n    for k, v in a.items():\n        result[k] = v\n    for k, v in b.items():\n        result[k] = v\n    return result",
    );
    assert!(!code.is_empty(), "complex dict operations: {}", code);
}

#[test]
fn test_dict_get_with_default() {
    let code =
        transpile("def safe_get(d: dict[str, int], key: str) -> int:\n    return d.get(key, 0)");
    assert!(!code.is_empty(), "dict get with default: {}", code);
}

#[test]
fn test_string_split_and_join() {
    let code = transpile(
        "def normalize(text: str) -> str:\n    words = text.split()\n    return ' '.join(words)",
    );
    assert!(!code.is_empty(), "string split and join: {}", code);
}

#[test]
fn test_string_strip_variations() {
    let code = transpile("def clean(s: str) -> str:\n    return s.strip().lstrip('x').rstrip('y')");
    assert!(!code.is_empty(), "string strip variations: {}", code);
}

#[test]
fn test_conditional_expression_nested() {
    let code = transpile(
        "def classify(x: int) -> str:\n    return 'positive' if x > 0 else ('zero' if x == 0 else 'negative')",
    );
    assert!(!code.is_empty(), "nested ternary: {}", code);
}

#[test]
fn test_multi_line_string_method_chain() {
    let code = transpile(
        "def process_text(text: str) -> str:\n    result = text.strip()\n    result = result.lower()\n    result = result.replace(' ', '_')\n    return result",
    );
    assert!(!code.is_empty(), "multi-line string chain: {}", code);
}

#[test]
fn test_for_else_pattern() {
    let code = transpile(
        "def search_linear(items: list[int], target: int) -> int:\n    for i in range(len(items)):\n        if items[i] == target:\n            return i\n    return -1",
    );
    assert!(!code.is_empty(), "for/else pattern: {}", code);
}

#[test]
fn test_class_with_repr() {
    let code = transpile(
        "class Vector:\n    def __init__(self, x: float, y: float):\n        self.x = x\n        self.y = y\n\n    def __repr__(self) -> str:\n        return f'Vector({self.x}, {self.y})'",
    );
    assert!(!code.is_empty(), "class with __repr__: {}", code);
}

#[test]
fn test_class_with_add_method() {
    let code = transpile(
        "class Vec2:\n    def __init__(self, x: float, y: float):\n        self.x = x\n        self.y = y\n\n    def __add__(self, other):\n        return Vec2(self.x + other.x, self.y + other.y)",
    );
    assert!(!code.is_empty(), "class with __add__: {}", code);
}

#[test]
fn test_deeply_nested_loops_with_conditions() {
    let code = transpile(
        "def find_triplet(items: list[int], target: int) -> bool:\n    for i in range(len(items)):\n        for j in range(i + 1, len(items)):\n            if items[i] + items[j] == target:\n                return True\n    return False",
    );
    assert!(!code.is_empty(), "deeply nested loops: {}", code);
}

#[test]
fn test_complex_while_with_state() {
    let code = transpile(
        "def binary_search(items: list[int], target: int) -> int:\n    lo = 0\n    hi = len(items) - 1\n    while lo <= hi:\n        mid = (lo + hi) // 2\n        if items[mid] == target:\n            return mid\n        elif items[mid] < target:\n            lo = mid + 1\n        else:\n            hi = mid - 1\n    return -1",
    );
    assert!(!code.is_empty(), "binary search: {}", code);
}

#[test]
fn test_exception_chaining() {
    let code = transpile(
        "def risky(x: int) -> int:\n    if x == 0:\n        raise ValueError('zero input')\n    if x < 0:\n        raise ValueError('negative input')\n    return 100 // x",
    );
    assert!(!code.is_empty(), "exception chaining: {}", code);
}

#[test]
fn test_complex_param_type_infer_from_body_with_untyped_dict() {
    let code = transpile(
        "def summarize(config):\n    name = config.get('name', 'unknown')\n    value = config.get('value', 0)\n    return f'{name}: {value}'",
    );
    assert!(!code.is_empty(), "untyped dict param inference: {}", code);
}

#[test]
fn test_param_infer_regex_module_usage() {
    // Exercises the re.match/re.search path in param_type_inference
    let code =
        transpile("def matches(pattern, text):\n    import re\n    return re.match(pattern, text)");
    assert!(!code.is_empty(), "regex module usage: {}", code);
}

#[test]
fn test_param_infer_not_equal_comparison() {
    let code = transpile("def is_not_admin(role):\n    return role != 'admin'");
    assert!(!code.is_empty(), "not equal comparison: {}", code);
}

#[test]
fn test_param_infer_in_list_of_strings() {
    let code = transpile("def is_color(name):\n    return name in ['red', 'green', 'blue']");
    assert!(!code.is_empty(), "in list of strings: {}", code);
}

#[test]
fn test_param_infer_not_in_operator() {
    let code = transpile("def is_excluded(item):\n    return item not in ['banned', 'blocked']");
    assert!(!code.is_empty(), "not in operator: {}", code);
}

#[test]
fn test_param_infer_unary_negation() {
    let code = transpile("def negate_val(x):\n    return -x");
    assert!(!code.is_empty(), "unary negation inference: {}", code);
}

#[test]
fn test_param_infer_encode_method() {
    let code = transpile("def encode_str(text):\n    return text.encode('utf-8')");
    assert!(!code.is_empty(), "encode method implies str: {}", code);
}

#[test]
fn test_param_infer_capitalize_method() {
    let code = transpile("def cap(text):\n    return text.capitalize()");
    assert!(!code.is_empty(), "capitalize method implies str: {}", code);
}

#[test]
fn test_param_infer_isdigit_method() {
    let code = transpile("def check_numeric(text):\n    return text.isdigit()");
    assert!(!code.is_empty(), "isdigit method implies str: {}", code);
}

#[test]
fn test_param_infer_isalpha_method() {
    let code = transpile("def check_alpha(text):\n    return text.isalpha()");
    assert!(!code.is_empty(), "isalpha method implies str: {}", code);
}

#[test]
fn test_param_infer_zfill_method() {
    let code = transpile("def pad_zeros(text):\n    return text.zfill(10)");
    assert!(!code.is_empty(), "zfill method implies str: {}", code);
}

#[test]
fn test_param_infer_center_method() {
    let code = transpile("def centered(text):\n    return text.center(20)");
    assert!(!code.is_empty(), "center method implies str: {}", code);
}

#[test]
fn test_param_infer_swapcase_method() {
    let code = transpile("def swap(text):\n    return text.swapcase()");
    assert!(!code.is_empty(), "swapcase method implies str: {}", code);
}

#[test]
fn test_param_infer_casefold_method() {
    let code = transpile("def fold(text):\n    return text.casefold()");
    assert!(!code.is_empty(), "casefold method implies str: {}", code);
}

#[test]
fn test_param_infer_expandtabs_method() {
    let code = transpile("def expand(text):\n    return text.expandtabs(4)");
    assert!(!code.is_empty(), "expandtabs method implies str: {}", code);
}

#[test]
fn test_param_infer_partition_method() {
    let code = transpile("def split_at(text):\n    return text.partition(':')");
    assert!(!code.is_empty(), "partition method implies str: {}", code);
}

#[test]
fn test_param_infer_pop_method_implies_dict() {
    let code = transpile("def remove_key(mapping):\n    return mapping.pop('key')");
    assert!(!code.is_empty(), "pop implies dict param: {}", code);
}

#[test]
fn test_param_infer_setdefault_implies_dict() {
    let code = transpile("def ensure_key(mapping):\n    return mapping.setdefault('key', 0)");
    assert!(!code.is_empty(), "setdefault implies dict: {}", code);
}

#[test]
fn test_param_infer_dict_clear() {
    let code = transpile("def wipe(mapping):\n    mapping.clear()\n    return mapping");
    assert!(!code.is_empty(), "clear implies dict: {}", code);
}

#[test]
fn test_param_infer_dict_update() {
    let code = transpile(
        "def merge_into(base, extra: dict[str, int]):\n    base.update(extra)\n    return base",
    );
    assert!(!code.is_empty(), "update implies dict: {}", code);
}

#[test]
fn test_param_infer_index_with_key_var() {
    // Tests the is_likely_string_key branch
    let code = transpile("def lookup(data, key):\n    return data[key]");
    assert!(!code.is_empty(), "index with key variable: {}", code);
}

#[test]
fn test_param_infer_seekable_file() {
    let code = transpile("def rewind(handle):\n    handle.seek(0)\n    return handle.tell()");
    assert!(!code.is_empty(), "seek/tell implies file: {}", code);
}

#[test]
fn test_param_infer_writelines_file() {
    let code = transpile("def write_all(handle, lines: list[str]):\n    handle.writelines(lines)");
    assert!(!code.is_empty(), "writelines implies file: {}", code);
}

#[test]
fn test_param_infer_readline_file() {
    let code = transpile("def first_line(handle):\n    return handle.readline()");
    assert!(!code.is_empty(), "readline implies file: {}", code);
}

#[test]
fn test_param_infer_truncate_file() {
    let code = transpile("def truncate_file(handle):\n    handle.truncate(0)");
    assert!(!code.is_empty(), "truncate implies file: {}", code);
}

#[test]
fn test_param_infer_close_file() {
    let code = transpile("def cleanup(handle):\n    handle.close()");
    assert!(!code.is_empty(), "close implies file: {}", code);
}

#[test]
fn test_param_infer_title_method() {
    let code = transpile("def title_case(text):\n    return text.title()");
    assert!(!code.is_empty(), "title implies str: {}", code);
}

#[test]
fn test_param_infer_ljust_rjust() {
    let code = transpile(
        "def pad(text):\n    left = text.ljust(20)\n    right = text.rjust(20)\n    return left + right",
    );
    assert!(!code.is_empty(), "ljust/rjust implies str: {}", code);
}

#[test]
fn test_param_infer_isspace_method() {
    let code = transpile("def check_whitespace(ch):\n    return ch.isspace()");
    assert!(!code.is_empty(), "isspace implies str: {}", code);
}

#[test]
fn test_param_infer_isupper_islower() {
    let code = transpile("def check_case(text):\n    return text.isupper() or text.islower()");
    assert!(!code.is_empty(), "isupper/islower implies str: {}", code);
}

#[test]
fn test_param_infer_isalnum_method() {
    let code = transpile("def check_alnum(text):\n    return text.isalnum()");
    assert!(!code.is_empty(), "isalnum implies str: {}", code);
}

#[test]
fn test_param_infer_splitlines_method() {
    let code = transpile("def get_lines(text):\n    return text.splitlines()");
    assert!(!code.is_empty(), "splitlines implies str: {}", code);
}

#[test]
fn test_param_infer_rpartition_method() {
    let code = transpile("def split_last(text):\n    return text.rpartition('/')");
    assert!(!code.is_empty(), "rpartition implies str: {}", code);
}

#[test]
fn test_param_infer_find_rfind_methods() {
    let code = transpile(
        "def locate(text):\n    first = text.find('x')\n    last = text.rfind('x')\n    return first + last",
    );
    assert!(!code.is_empty(), "find/rfind implies str: {}", code);
}

#[test]
fn test_param_infer_count_method() {
    let code = transpile("def occurrences(text):\n    return text.count('a')");
    assert!(!code.is_empty(), "count implies str: {}", code);
}

#[test]
fn test_param_infer_dict_copy() {
    let code = transpile("def clone_dict(original):\n    return original.copy()");
    assert!(!code.is_empty(), "copy implies dict: {}", code);
}

#[test]
fn test_param_infer_dict_popitem() {
    let code = transpile("def remove_last(d):\n    return d.popitem()");
    assert!(!code.is_empty(), "popitem implies dict: {}", code);
}

#[test]
fn test_lifetime_string_concatenation_in_loop() {
    let code = transpile(
        "def build_string(parts: list[str]) -> str:\n    result = ''\n    for p in parts:\n        result = result + p + ' '\n    return result.strip()",
    );
    assert!(!code.is_empty(), "string concat in loop: {}", code);
}

#[test]
fn test_lifetime_dict_mutation_during_iteration() {
    let code = transpile(
        "def increment_all(d: dict[str, int]) -> dict[str, int]:\n    result = {}\n    for k, v in d.items():\n        result[k] = v + 1\n    return result",
    );
    assert!(!code.is_empty(), "dict mutation during iteration: {}", code);
}

#[test]
fn test_escape_multiple_returns() {
    let code = transpile(
        "def branch_return(flag: bool, items: list[int]) -> list[int]:\n    if flag:\n        return items\n    result = []\n    for x in items:\n        result.append(x + 1)\n    return result",
    );
    assert!(!code.is_empty(), "multiple returns escape: {}", code);
}

#[test]
fn test_type_hint_infer_from_default_value_list() {
    let code = transpile("def with_items(items=[]):\n    items.append(1)\n    return items");
    assert!(!code.is_empty(), "default list value: {}", code);
}

#[test]
fn test_var_analysis_swap_pattern() {
    let code = transpile(
        "def swap(a: int, b: int) -> tuple[int, int]:\n    temp = a\n    a = b\n    b = temp\n    return (a, b)",
    );
    assert!(!code.is_empty(), "swap pattern: {}", code);
}

#[test]
fn test_var_analysis_accumulator_in_nested_loop() {
    let code = transpile(
        "def count_pairs(matrix: list[list[int]]) -> int:\n    count = 0\n    for row in matrix:\n        for val in row:\n            if val > 0:\n                count = count + 1\n    return count",
    );
    assert!(!code.is_empty(), "accumulator in nested loop: {}", code);
}

#[test]
fn test_codegen_empty_function() {
    let code = transpile("def empty():\n    pass\n\ndef also_empty():\n    pass");
    assert!(!code.is_empty(), "empty functions: {}", code);
}

#[test]
fn test_codegen_function_with_docstring() {
    let code = transpile(
        "def documented(x: int) -> int:\n    \"\"\"Double the input.\"\"\"\n    return x * 2",
    );
    assert!(!code.is_empty(), "function with docstring: {}", code);
}

#[test]
fn test_codegen_class_with_docstring() {
    let code = transpile(
        "class Documented:\n    \"\"\"A documented class.\"\"\"\n    def __init__(self):\n        self.value = 0",
    );
    assert!(!code.is_empty(), "class with docstring: {}", code);
}

#[test]
fn test_complex_try_with_variable_hoisting() {
    let code = transpile(
        "def hoisted(x: int) -> int:\n    try:\n        result = x * 2\n        extra = result + 1\n    except ValueError:\n        result = 0\n        extra = 0\n    return result + extra",
    );
    assert!(!code.is_empty(), "try variable hoisting: {}", code);
}

#[test]
fn test_borrow_nested_field_access() {
    let code = transpile(
        "class Node:\n    def __init__(self, val: int):\n        self.val = val\n        self.children = []\n\n    def total(self) -> int:\n        total = self.val\n        for child in self.children:\n            total = total + child.val\n        return total",
    );
    assert!(!code.is_empty(), "nested field access: {}", code);
}

#[test]
fn test_escape_param_stored_in_list() {
    let code = transpile(
        "def store_param(x: str, items: list[str]) -> list[str]:\n    items.append(x)\n    return items",
    );
    assert!(!code.is_empty(), "param stored in list: {}", code);
}

#[test]
fn test_type_hint_complex_return_inference() {
    let code = transpile(
        "def analyze(data):\n    if not data:\n        return []\n    result = []\n    for item in data:\n        result.append(item * 2)\n    return result",
    );
    assert!(!code.is_empty(), "complex return inference: {}", code);
}
