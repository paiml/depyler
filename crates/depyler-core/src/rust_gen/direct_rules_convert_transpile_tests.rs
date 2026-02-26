//! Comprehensive transpile-based tests for direct_rules_convert submodules
//!
//! DEPYLER-COVERAGE-95: Targets zero-coverage submodules in direct_rules_convert/
//! through end-to-end transpilation pipeline tests.
//!
//! Covered submodules:
//! - expr_methods.rs: String, list, dict, module method calls
//! - expr_advanced.rs: Comprehensions, lambda, f-string, attribute access
//! - expr_operators.rs: Binary, unary, truthiness, operator precedence
//! - stmt_convert.rs: If/else, while, for, with, try/except, raise
//! - body_convert.rs: Function body, mutability analysis, assignments
//! - expr_builtins.rs: len, range, enumerate, zip, sorted, sum, etc.
//! - expr_collections.rs: List, dict, set, tuple, frozenset literals
//! - method_stmt_convert.rs: Method body statement conversion
//! - expr_index_slice.rs: Indexing and slicing
//! - stdlib_calls.rs: os.path, open, date, generic calls

use crate::DepylerPipeline;

fn transpile(code: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).expect("transpilation should succeed")
}

fn transpile_ok(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_ok()
}

// ============================================================================
// SECTION 1: expr_methods.rs - String method calls
// ============================================================================

#[test]
fn test_string_upper() {
    let code = transpile("def foo(s: str) -> str:\n    return s.upper()");
    assert!(
        code.contains("to_uppercase") || code.contains("to_ascii_uppercase"),
        "upper() should map to to_uppercase: {}",
        code
    );
}

#[test]
fn test_string_lower() {
    let code = transpile("def foo(s: str) -> str:\n    return s.lower()");
    assert!(
        code.contains("to_lowercase") || code.contains("to_ascii_lowercase"),
        "lower() should map to to_lowercase: {}",
        code
    );
}

#[test]
fn test_string_strip() {
    let code = transpile("def foo(s: str) -> str:\n    return s.strip()");
    assert!(code.contains("trim"), "strip() should map to trim(): {}", code);
}

#[test]
fn test_string_lstrip() {
    let code = transpile("def foo(s: str) -> str:\n    return s.lstrip()");
    assert!(code.contains("trim_start"), "lstrip() should map to trim_start(): {}", code);
}

#[test]
fn test_string_rstrip() {
    let code = transpile("def foo(s: str) -> str:\n    return s.rstrip()");
    assert!(code.contains("trim_end"), "rstrip() should map to trim_end(): {}", code);
}

#[test]
fn test_string_startswith() {
    let code = transpile("def foo(s: str) -> bool:\n    return s.startswith('hello')");
    assert!(code.contains("starts_with"), "startswith() should map to starts_with(): {}", code);
}

#[test]
fn test_string_endswith() {
    let code = transpile("def foo(s: str) -> bool:\n    return s.endswith('.py')");
    assert!(code.contains("ends_with"), "endswith() should map to ends_with(): {}", code);
}

#[test]
fn test_string_split_no_args() {
    let code = transpile("def foo(s: str) -> list:\n    return s.split()");
    assert!(code.contains("split"), "split() should use Rust split: {}", code);
}

#[test]
fn test_string_split_with_delimiter() {
    let code = transpile("def foo(s: str) -> list:\n    return s.split(',')");
    assert!(code.contains("split"), "split(',') should use Rust split: {}", code);
}

#[test]
fn test_string_join() {
    let code = transpile("def foo(items: list) -> str:\n    return ','.join(items)");
    assert!(code.contains("join"), "join() should map to Rust join: {}", code);
}

#[test]
fn test_string_replace() {
    let code = transpile("def foo(s: str) -> str:\n    return s.replace('a', 'b')");
    assert!(
        code.contains("replace") || code.contains("replacen"),
        "replace() should map to Rust replace: {}",
        code
    );
}

#[test]
fn test_string_find() {
    let code = transpile("def foo(s: str) -> int:\n    return s.find('x')");
    assert!(code.contains("find"), "find() should produce find-related code: {}", code);
}

#[test]
fn test_string_format_method() {
    assert!(transpile_ok("def foo() -> str:\n    return 'hello {}'.format('world')"));
}

#[test]
fn test_string_encode() {
    assert!(transpile_ok("def foo(s: str) -> bytes:\n    return s.encode('utf-8')"));
}

#[test]
fn test_string_isdigit() {
    assert!(transpile_ok("def foo(s: str) -> bool:\n    return s.isdigit()"));
}

#[test]
fn test_string_isalpha() {
    assert!(transpile_ok("def foo(s: str) -> bool:\n    return s.isalpha()"));
}

// ============================================================================
// SECTION 2: expr_methods.rs - List method calls
// ============================================================================

#[test]
fn test_list_append() {
    let code = transpile("def foo():\n    items = []\n    items.append(1)\n    return items");
    assert!(code.contains("push"), "append() should map to push(): {}", code);
}

#[test]
fn test_list_extend() {
    let code = transpile("def foo():\n    items = [1]\n    items.extend([2, 3])\n    return items");
    assert!(code.contains("extend"), "extend() should map to extend(): {}", code);
}

#[test]
fn test_list_pop() {
    assert!(transpile_ok("def foo():\n    items = [1, 2, 3]\n    items.pop()\n    return items"));
}

#[test]
fn test_list_remove() {
    assert!(transpile_ok(
        "def foo():\n    items = [1, 2, 3]\n    items.remove(2)\n    return items"
    ));
}

#[test]
fn test_list_clear() {
    assert!(transpile_ok("def foo():\n    items = [1, 2, 3]\n    items.clear()\n    return items"));
}

#[test]
fn test_list_sort() {
    assert!(transpile_ok("def foo():\n    items = [3, 1, 2]\n    items.sort()\n    return items"));
}

#[test]
fn test_list_reverse() {
    assert!(transpile_ok(
        "def foo():\n    items = [1, 2, 3]\n    items.reverse()\n    return items"
    ));
}

#[test]
fn test_list_copy() {
    assert!(transpile_ok("def foo():\n    items = [1, 2, 3]\n    return items.copy()"));
}

// ============================================================================
// SECTION 3: expr_methods.rs - Dict method calls
// ============================================================================

#[test]
fn test_dict_get() {
    assert!(transpile_ok("def foo():\n    data = {'a': 1}\n    return data.get('a')"));
}

#[test]
fn test_dict_keys() {
    assert!(transpile_ok("def foo():\n    data = {'a': 1}\n    return data.keys()"));
}

#[test]
fn test_dict_values() {
    assert!(transpile_ok("def foo():\n    data = {'a': 1}\n    return data.values()"));
}

#[test]
fn test_dict_items() {
    assert!(transpile_ok(
        "def foo():\n    data = {'a': 1}\n    for k, v in data.items():\n        pass"
    ));
}

#[test]
fn test_dict_update() {
    assert!(transpile_ok("def foo():\n    data = {'a': 1}\n    data.update({'b': 2})"));
}

// ============================================================================
// SECTION 4: expr_methods.rs - Module method calls (time, sys, re)
// ============================================================================

#[test]
fn test_time_time_method() {
    let code = transpile("def foo():\n    import time\n    return time.time()");
    assert!(
        code.contains("SystemTime") || code.contains("UNIX_EPOCH") || code.contains("time"),
        "time.time() should use SystemTime: {}",
        code
    );
}

#[test]
fn test_time_sleep_method() {
    assert!(transpile_ok("def foo():\n    import time\n    time.sleep(1)"));
}

#[test]
fn test_sys_exit() {
    let code = transpile("def foo():\n    import sys\n    sys.exit(0)");
    assert!(
        code.contains("exit") || code.contains("process"),
        "sys.exit should map to process::exit: {}",
        code
    );
}

// ============================================================================
// SECTION 5: expr_advanced.rs - List comprehensions
// ============================================================================

#[test]
fn test_list_comprehension_simple() {
    let code = transpile("def foo() -> list:\n    return [x * 2 for x in range(10)]");
    assert!(
        code.contains("map") || code.contains("collect"),
        "List comp should use iterator chain: {}",
        code
    );
}

#[test]
fn test_list_comprehension_with_condition() {
    let code = transpile("def foo() -> list:\n    return [x for x in range(10) if x > 5]");
    assert!(code.contains("filter"), "Conditional list comp should use filter: {}", code);
}

#[test]
fn test_set_comprehension() {
    let code = transpile("def foo() -> set:\n    return {x * 2 for x in range(10)}");
    assert!(
        code.contains("HashSet") || code.contains("collect"),
        "Set comp should collect into HashSet: {}",
        code
    );
}

#[test]
fn test_dict_comprehension() {
    let code = transpile("def foo() -> dict:\n    return {x: x * 2 for x in range(5)}");
    assert!(
        code.contains("HashMap") || code.contains("collect"),
        "Dict comp should collect into HashMap: {}",
        code
    );
}

#[test]
fn test_dict_comprehension_with_filter() {
    let code = transpile("def foo() -> dict:\n    return {x: x * 2 for x in range(10) if x > 3}");
    assert!(code.contains("filter"), "Dict comp with condition should use filter: {}", code);
}

// ============================================================================
// SECTION 6: expr_advanced.rs - Lambda expressions
// ============================================================================

#[test]
fn test_lambda_single_param() {
    let code = transpile("def foo():\n    f = lambda x: x * 2\n    return f(5)");
    assert!(code.contains("|"), "Lambda should generate closure: {}", code);
}

#[test]
fn test_lambda_multiple_params() {
    let code = transpile("def foo():\n    f = lambda x, y: x + y\n    return f(1, 2)");
    assert!(code.contains("|"), "Lambda should generate closure: {}", code);
}

#[test]
fn test_lambda_no_params() {
    let code = transpile("def foo():\n    f = lambda: 42\n    return f()");
    assert!(code.contains("||"), "No-param lambda should generate || closure: {}", code);
}

// ============================================================================
// SECTION 7: expr_advanced.rs - F-string support
// ============================================================================

#[test]
fn test_fstring_simple() {
    let code = transpile("def foo(name: str) -> str:\n    return f'hello {name}'");
    assert!(code.contains("format!"), "F-string should use format!: {}", code);
}

#[test]
fn test_fstring_multiple_expressions() {
    let code = transpile("def foo(x: int, y: int) -> str:\n    return f'{x} + {y} = {x + y}'");
    assert!(code.contains("format!"), "F-string with expressions should use format!: {}", code);
}

#[test]
fn test_fstring_empty() {
    assert!(transpile_ok("def foo() -> str:\n    return f''"));
}

// ============================================================================
// SECTION 8: expr_advanced.rs - Attribute access
// ============================================================================

#[test]
fn test_attribute_access_self() {
    let code = transpile(
        "class Point:\n    def __init__(self, x: int):\n        self.x = x\n    def get_x(self) -> int:\n        return self.x",
    );
    assert!(code.contains("self.x"), "self.x should remain self.x: {}", code);
}

#[test]
fn test_attribute_access_module() {
    assert!(transpile_ok("def foo():\n    import math\n    return math.pi"));
}

// ============================================================================
// SECTION 9: expr_advanced.rs - Module constructors
// ============================================================================

#[test]
fn test_collections_deque_constructor() {
    assert!(transpile_ok(
        "def foo():\n    import collections\n    d = collections.deque()\n    return d"
    ));
}

#[test]
fn test_collections_counter_constructor() {
    assert!(transpile_ok(
        "def foo():\n    import collections\n    c = collections.Counter([1, 2, 2, 3])\n    return c"
    ));
}

// ============================================================================
// SECTION 10: expr_operators.rs - Binary operators
// ============================================================================

#[test]
fn test_operator_add() {
    let code = transpile("def foo(a: int, b: int) -> int:\n    return a + b");
    assert!(code.contains("+"), "Should contain +: {}", code);
}

#[test]
fn test_operator_sub() {
    let code = transpile("def foo(a: int, b: int) -> int:\n    return a - b");
    assert!(code.contains("-"), "Should contain -: {}", code);
}

#[test]
fn test_operator_mul() {
    let code = transpile("def foo(a: int, b: int) -> int:\n    return a * b");
    assert!(code.contains("*"), "Should contain *: {}", code);
}

#[test]
fn test_operator_div() {
    let code = transpile("def foo(a: int, b: int) -> int:\n    return a / b");
    assert!(code.contains("/"), "Should contain /: {}", code);
}

#[test]
fn test_operator_modulo() {
    let code = transpile("def foo(a: int, b: int) -> int:\n    return a % b");
    assert!(code.contains("%"), "Should contain %: {}", code);
}

#[test]
fn test_operator_floor_div() {
    let code = transpile("def foo(a: int, b: int) -> int:\n    return a // b");
    assert!(
        code.contains("/") || code.contains("floor"),
        "Floor div should produce division: {}",
        code
    );
}

#[test]
fn test_operator_power() {
    let code = transpile("def foo(a: int, b: int) -> int:\n    return a ** b");
    assert!(code.contains("pow") || code.contains("checked_pow"), "Power should use pow: {}", code);
}

#[test]
fn test_operator_power_float() {
    let code = transpile("def foo() -> float:\n    return 2.0 ** 3.0");
    assert!(code.contains("powf"), "Float power should use powf: {}", code);
}

#[test]
fn test_operator_bitwise_and() {
    let code = transpile("def foo(a: int, b: int) -> int:\n    return a & b");
    assert!(code.contains("&"), "Should contain &: {}", code);
}

#[test]
fn test_operator_bitwise_or() {
    let code = transpile("def foo(a: int, b: int) -> int:\n    return a | b");
    assert!(code.contains("|"), "Should contain |: {}", code);
}

#[test]
fn test_operator_bitwise_xor() {
    let code = transpile("def foo(a: int, b: int) -> int:\n    return a ^ b");
    assert!(code.contains("^"), "Should contain ^: {}", code);
}

#[test]
fn test_operator_left_shift() {
    let code = transpile("def foo(a: int, b: int) -> int:\n    return a << b");
    assert!(code.contains("<<"), "Should contain <<: {}", code);
}

#[test]
fn test_operator_right_shift() {
    let code = transpile("def foo(a: int, b: int) -> int:\n    return a >> b");
    assert!(code.contains(">>"), "Should contain >>: {}", code);
}

// ============================================================================
// SECTION 11: expr_operators.rs - Comparison operators
// ============================================================================

#[test]
fn test_comparison_eq() {
    let code = transpile("def foo(a: int, b: int) -> bool:\n    return a == b");
    assert!(code.contains("=="), "Should contain ==: {}", code);
}

#[test]
fn test_comparison_neq() {
    let code = transpile("def foo(a: int, b: int) -> bool:\n    return a != b");
    assert!(code.contains("!="), "Should contain !=: {}", code);
}

#[test]
fn test_comparison_lt() {
    let code = transpile("def foo(a: int, b: int) -> bool:\n    return a < b");
    assert!(code.contains("<"), "Should contain <: {}", code);
}

#[test]
fn test_comparison_gt() {
    let code = transpile("def foo(a: int, b: int) -> bool:\n    return a > b");
    assert!(code.contains(">"), "Should contain >: {}", code);
}

#[test]
fn test_comparison_lte() {
    let code = transpile("def foo(a: int, b: int) -> bool:\n    return a <= b");
    assert!(code.contains("<="), "Should contain <=: {}", code);
}

#[test]
fn test_comparison_gte() {
    let code = transpile("def foo(a: int, b: int) -> bool:\n    return a >= b");
    assert!(code.contains(">="), "Should contain >=: {}", code);
}

// ============================================================================
// SECTION 12: expr_operators.rs - Logical operators
// ============================================================================

#[test]
fn test_logical_and() {
    let code = transpile("def foo(a: bool, b: bool) -> bool:\n    return a and b");
    assert!(code.contains("&&"), "Should contain &&: {}", code);
}

#[test]
fn test_logical_or() {
    let code = transpile("def foo(a: bool, b: bool) -> bool:\n    return a or b");
    assert!(code.contains("||"), "Should contain ||: {}", code);
}

#[test]
fn test_logical_not() {
    let code = transpile("def foo(a: bool) -> bool:\n    return not a");
    assert!(code.contains("!"), "Should contain !: {}", code);
}

// ============================================================================
// SECTION 13: expr_operators.rs - In/Not In operators
// ============================================================================

#[test]
fn test_in_operator_string() {
    let code = transpile("def foo(text: str) -> bool:\n    return 'hello' in text");
    assert!(code.contains("contains"), "String 'in' should use contains: {}", code);
}

#[test]
fn test_not_in_operator_string() {
    let code = transpile("def foo(text: str) -> bool:\n    return 'hello' not in text");
    assert!(code.contains("contains"), "String 'not in' should use contains: {}", code);
}

#[test]
fn test_in_operator_list() {
    let code = transpile("def foo(x: int) -> bool:\n    return x in [1, 2, 3]");
    assert!(code.contains("contains"), "List 'in' should use contains: {}", code);
}

#[test]
fn test_in_operator_tuple() {
    let code = transpile("def foo(x: int) -> bool:\n    return x in (1, 2, 3)");
    assert!(code.contains("contains"), "Tuple 'in' should use contains: {}", code);
}

// ============================================================================
// SECTION 14: expr_operators.rs - Unary operators
// ============================================================================

#[test]
fn test_unary_negate() {
    let code = transpile("def foo(x: int) -> int:\n    return -x");
    assert!(code.contains("-"), "Should negate: {}", code);
}

#[test]
fn test_unary_not_bool() {
    let code = transpile("def foo(x: bool) -> bool:\n    return not x");
    assert!(code.contains("!"), "Should negate bool: {}", code);
}

// ============================================================================
// SECTION 15: expr_operators.rs - String concatenation with +
// ============================================================================

#[test]
fn test_string_concatenation() {
    let code = transpile("def foo(a: str, b: str) -> str:\n    return a + b");
    assert!(code.contains("format!"), "String + should use format!: {}", code);
}

// ============================================================================
// SECTION 16: expr_operators.rs - Operator precedence
// ============================================================================

#[test]
fn test_mul_with_add_precedence() {
    let code = transpile("def foo(a: int, b: int) -> int:\n    return 2 * (a + b)");
    assert!(
        code.contains("(") && code.contains("+"),
        "Should preserve parentheses for precedence: {}",
        code
    );
}

#[test]
fn test_list_repeat_operator() {
    let code = transpile("def foo():\n    return [0] * 5");
    assert!(
        code.contains("py_mul") || code.contains("[0 ; 5]") || code.contains("[0; 5]"),
        "List repeat should produce array init or py_mul: {}",
        code
    );
}

// ============================================================================
// SECTION 17: expr_operators.rs - len() saturating_sub
// ============================================================================

#[test]
fn test_len_minus_one() {
    let code = transpile("def foo(items: list) -> int:\n    return len(items) - 1");
    assert!(
        code.contains("saturating_sub") || code.contains("len()") || code.contains("- 1"),
        "len() - N should produce subtraction: {}",
        code
    );
}

// ============================================================================
// SECTION 18: stmt_convert.rs - If/Else statements
// ============================================================================

#[test]
fn test_if_simple() {
    let code = transpile("def foo(x: int) -> int:\n    if x > 0:\n        return 1\n    return 0");
    assert!(code.contains("if"), "Should contain if statement: {}", code);
}

#[test]
fn test_if_else() {
    let code = transpile(
        "def foo(x: int) -> int:\n    if x > 0:\n        return 1\n    else:\n        return -1",
    );
    assert!(code.contains("if") && code.contains("else"), "Should contain if/else: {}", code);
}

#[test]
fn test_if_elif_else() {
    let code = transpile(
        "def foo(x: int) -> str:\n    if x > 0:\n        return 'pos'\n    elif x < 0:\n        return 'neg'\n    else:\n        return 'zero'",
    );
    assert!(code.contains("if") && code.contains("else"), "Should contain if/elif/else: {}", code);
}

// ============================================================================
// SECTION 19: stmt_convert.rs - While loops
// ============================================================================

#[test]
fn test_while_simple() {
    let code = transpile(
        "def foo() -> int:\n    i = 0\n    while i < 10:\n        i = i + 1\n    return i",
    );
    assert!(code.contains("while"), "Should contain while loop: {}", code);
}

#[test]
fn test_while_with_break() {
    let code = transpile(
        "def foo() -> int:\n    i = 0\n    while True:\n        if i > 5:\n            break\n        i = i + 1\n    return i",
    );
    assert!(code.contains("break"), "Should contain break: {}", code);
}

#[test]
fn test_while_with_continue() {
    let code = transpile(
        "def foo() -> int:\n    i = 0\n    total = 0\n    while i < 10:\n        i = i + 1\n        if i % 2 == 0:\n            continue\n        total = total + i\n    return total",
    );
    assert!(code.contains("continue"), "Should contain continue: {}", code);
}

// ============================================================================
// SECTION 20: stmt_convert.rs - For loops
// ============================================================================

#[test]
fn test_for_range() {
    let code = transpile(
        "def foo() -> int:\n    total = 0\n    for i in range(10):\n        total = total + i\n    return total",
    );
    assert!(code.contains("for"), "Should contain for loop: {}", code);
}

#[test]
fn test_for_range_with_start_stop() {
    let code = transpile(
        "def foo() -> int:\n    total = 0\n    for i in range(1, 10):\n        total = total + i\n    return total",
    );
    assert!(code.contains("for"), "Should contain for loop: {}", code);
}

#[test]
fn test_for_iterate_list() {
    let code = transpile(
        "def foo(items: list) -> int:\n    total = 0\n    for x in items:\n        total = total + x\n    return total",
    );
    assert!(code.contains("for"), "Should contain for loop: {}", code);
}

#[test]
fn test_for_dict_items() {
    let code =
        transpile("def foo():\n    data = {'a': 1}\n    for k, v in data.items():\n        pass");
    assert!(
        code.contains("for") && (code.contains("iter") || code.contains("items")),
        "Should iterate dict items: {}",
        code
    );
}

#[test]
fn test_for_dict_keys() {
    let code =
        transpile("def foo():\n    data = {'a': 1}\n    for k in data.keys():\n        pass");
    assert!(code.contains("keys"), "Should use .keys(): {}", code);
}

#[test]
fn test_for_dict_values() {
    let code =
        transpile("def foo():\n    data = {'a': 1}\n    for v in data.values():\n        pass");
    assert!(code.contains("values"), "Should use .values(): {}", code);
}

// ============================================================================
// SECTION 21: stmt_convert.rs - Raise statement
// ============================================================================

#[test]
fn test_raise_exception() {
    let code = transpile("def foo():\n    raise ValueError('bad value')");
    assert!(code.contains("panic!"), "raise should map to panic!: {}", code);
}

#[test]
fn test_raise_bare() {
    let code = transpile("def foo():\n    raise");
    assert!(code.contains("panic!"), "bare raise should map to panic!: {}", code);
}

// ============================================================================
// SECTION 22: stmt_convert.rs - With statement
// ============================================================================

#[test]
fn test_with_statement() {
    assert!(transpile_ok("def foo():\n    with open('test.txt') as f:\n        data = f.read()"));
}

// ============================================================================
// SECTION 23: stmt_convert.rs - Try/Except
// ============================================================================

#[test]
fn test_try_except_simple() {
    assert!(transpile_ok("def foo():\n    try:\n        x = 1\n    except:\n        x = 0"));
}

#[test]
fn test_try_except_finally() {
    assert!(transpile_ok(
        "def foo():\n    try:\n        x = 1\n    except:\n        x = 0\n    finally:\n        pass"
    ));
}

// ============================================================================
// SECTION 24: stmt_convert.rs - Pass statement
// ============================================================================

#[test]
fn test_pass_in_function() {
    assert!(transpile_ok("def foo():\n    pass"));
}

#[test]
fn test_pass_in_if() {
    assert!(transpile_ok("def foo(x: int):\n    if x > 0:\n        pass"));
}

// ============================================================================
// SECTION 25: body_convert.rs - Assignments
// ============================================================================

#[test]
fn test_simple_assignment() {
    let code = transpile("def foo() -> int:\n    x = 42\n    return x");
    assert!(code.contains("let") && code.contains("42"), "Should have let binding: {}", code);
}

#[test]
fn test_mutable_assignment() {
    let code = transpile("def foo() -> int:\n    x = 0\n    x = 1\n    return x");
    assert!(code.contains("mut"), "Reassigned variable should be mut: {}", code);
}

#[test]
fn test_tuple_unpacking() {
    let code = transpile("def foo():\n    a, b = 1, 2\n    return a + b");
    assert!(code.contains("a") && code.contains("b"), "Should unpack tuple: {}", code);
}

#[test]
fn test_attribute_assignment() {
    let code = transpile("class Foo:\n    def __init__(self):\n        self.x = 42");
    assert!(
        code.contains("self.x")
            || code.contains("x :")
            || code.contains("x:")
            || code.contains("struct Foo"),
        "Should produce class struct with x field: {}",
        code
    );
}

#[test]
fn test_index_assignment() {
    assert!(transpile_ok("def foo():\n    data = {'a': 1}\n    data['b'] = 2"));
}

// ============================================================================
// SECTION 26: body_convert.rs - Mutability detection
// ============================================================================

#[test]
fn test_mutability_from_append() {
    let code = transpile(
        "def foo() -> list:\n    items = []\n    items.append(1)\n    items.append(2)\n    return items",
    );
    assert!(code.contains("mut"), "List with append should be mut: {}", code);
}

#[test]
fn test_mutability_from_reassignment_in_loop() {
    let code = transpile(
        "def foo() -> int:\n    total = 0\n    for i in range(10):\n        total = total + i\n    return total",
    );
    assert!(code.contains("mut"), "Reassigned variable should be mut: {}", code);
}

#[test]
fn test_immutable_single_assignment() {
    let code = transpile("def foo() -> int:\n    x = 42\n    return x");
    // x is never reassigned, should not be mut
    // Note: counting "mut " to avoid matching "mut" in other identifiers
    let mut_count = code.matches("let mut ").count();
    // x should be immutable - either 0 mut bindings or only other variables
    assert!(
        mut_count == 0 || !code.contains("let mut x"),
        "Single-assignment variable should not be mut: {}",
        code
    );
}

// ============================================================================
// SECTION 27: expr_builtins.rs - len()
// ============================================================================

#[test]
fn test_builtin_len() {
    let code = transpile("def foo(items: list) -> int:\n    return len(items)");
    assert!(code.contains("len()"), "len() should map to .len(): {}", code);
}

// ============================================================================
// SECTION 28: expr_builtins.rs - range()
// ============================================================================

#[test]
fn test_builtin_range_single_arg() {
    let code = transpile("def foo():\n    for i in range(10):\n        pass");
    assert!(
        code.contains("0..") || code.contains("0 .."),
        "range(10) should map to 0..10: {}",
        code
    );
}

#[test]
fn test_builtin_range_two_args() {
    let code = transpile("def foo():\n    for i in range(1, 10):\n        pass");
    assert!(code.contains(".."), "range(1,10) should map to 1..10: {}", code);
}

// ============================================================================
// SECTION 29: expr_builtins.rs - enumerate, zip, reversed, sorted
// ============================================================================

#[test]
fn test_builtin_enumerate() {
    let code = transpile("def foo(items: list):\n    for i, x in enumerate(items):\n        pass");
    assert!(code.contains("enumerate"), "enumerate() should be in output: {}", code);
}

#[test]
fn test_builtin_zip() {
    let code = transpile("def foo(a: list, b: list):\n    for x, y in zip(a, b):\n        pass");
    assert!(code.contains("zip"), "zip() should be in output: {}", code);
}

#[test]
fn test_builtin_reversed() {
    let code = transpile("def foo(items: list):\n    for x in reversed(items):\n        pass");
    assert!(code.contains("rev"), "reversed() should map to .rev(): {}", code);
}

#[test]
fn test_builtin_sorted() {
    let code = transpile("def foo(items: list) -> list:\n    return sorted(items)");
    assert!(code.contains("sort"), "sorted() should produce sort: {}", code);
}

// ============================================================================
// SECTION 30: expr_builtins.rs - sum, all, any
// ============================================================================

#[test]
fn test_builtin_sum() {
    let code = transpile("def foo(items: list) -> int:\n    return sum(items)");
    assert!(code.contains("sum"), "sum() should produce .sum(): {}", code);
}

#[test]
fn test_builtin_all() {
    let code = transpile("def foo(items: list) -> bool:\n    return all(items)");
    assert!(code.contains("all"), "all() should produce .all(): {}", code);
}

#[test]
fn test_builtin_any() {
    let code = transpile("def foo(items: list) -> bool:\n    return any(items)");
    assert!(code.contains("any"), "any() should produce .any(): {}", code);
}

// ============================================================================
// SECTION 31: expr_builtins.rs - Type conversion functions
// ============================================================================

#[test]
fn test_builtin_int_cast() {
    let code = transpile("def foo(x: float) -> int:\n    return int(x)");
    assert!(
        code.contains("parse") || code.contains("as i32"),
        "int() should cast to int: {}",
        code
    );
}

#[test]
fn test_builtin_float_cast() {
    let code = transpile("def foo(x: str) -> float:\n    return float(x)");
    assert!(code.contains("parse") || code.contains("f64"), "float() should cast to f64: {}", code);
}

#[test]
fn test_builtin_str_cast() {
    let code = transpile("def foo(x: int) -> str:\n    return str(x)");
    assert!(code.contains("to_string"), "str() should use to_string(): {}", code);
}

#[test]
fn test_builtin_abs() {
    let code = transpile("def foo(x: int) -> int:\n    return abs(x)");
    assert!(code.contains("abs"), "abs() should map to .abs(): {}", code);
}

#[test]
fn test_builtin_min_two_args() {
    let code = transpile("def foo(a: int, b: int) -> int:\n    return min(a, b)");
    assert!(code.contains("min"), "min() should produce min: {}", code);
}

#[test]
fn test_builtin_max_two_args() {
    let code = transpile("def foo(a: int, b: int) -> int:\n    return max(a, b)");
    assert!(code.contains("max"), "max() should produce max: {}", code);
}

// ============================================================================
// SECTION 32: expr_builtins.rs - print()
// ============================================================================

#[test]
fn test_builtin_print_no_args() {
    let code = transpile("def foo():\n    print()");
    assert!(code.contains("println!"), "print() should map to println!: {}", code);
}

#[test]
fn test_builtin_print_one_arg() {
    let code = transpile("def foo():\n    print('hello')");
    assert!(code.contains("println!"), "print() should map to println!: {}", code);
}

#[test]
fn test_builtin_print_multiple_args() {
    let code = transpile("def foo():\n    print('a', 'b', 'c')");
    assert!(code.contains("println!"), "print() should map to println!: {}", code);
}

// ============================================================================
// SECTION 33: expr_builtins.rs - ord, chr
// ============================================================================

#[test]
fn test_builtin_ord() {
    let code = transpile("def foo() -> int:\n    return ord('a')");
    assert!(
        code.contains("chars") || code.contains("as i32"),
        "ord() should get char code: {}",
        code
    );
}

#[test]
fn test_builtin_chr() {
    let code = transpile("def foo() -> str:\n    return chr(97)");
    assert!(
        code.contains("from_u32") || code.contains("char"),
        "chr() should convert from code point: {}",
        code
    );
}

// ============================================================================
// SECTION 34: expr_builtins.rs - list(), dict(), bytes(), tuple()
// ============================================================================

#[test]
fn test_builtin_list_empty() {
    let code = transpile("def foo() -> list:\n    return list()");
    assert!(
        code.contains("Vec::new") || code.contains("vec!"),
        "list() should create Vec: {}",
        code
    );
}

#[test]
fn test_builtin_dict_empty() {
    let code = transpile("def foo() -> dict:\n    return dict()");
    assert!(
        code.contains("HashMap::new") || code.contains("HashMap"),
        "dict() should create HashMap: {}",
        code
    );
}

#[test]
fn test_builtin_isinstance() {
    let code = transpile("def foo(x: int) -> bool:\n    return isinstance(x, int)");
    assert!(code.contains("true"), "isinstance should return true: {}", code);
}

// ============================================================================
// SECTION 35: expr_builtins.rs - set, frozenset
// ============================================================================

#[test]
fn test_builtin_set_constructor() {
    assert!(transpile_ok("def foo() -> set:\n    return set()"));
}

// ============================================================================
// SECTION 36: expr_builtins.rs - open()
// ============================================================================

#[test]
fn test_builtin_open_read() {
    assert!(transpile_ok("def foo():\n    f = open('test.txt')\n    return f"));
}

#[test]
fn test_builtin_open_write() {
    assert!(transpile_ok("def foo():\n    f = open('test.txt', 'w')\n    return f"));
}

// ============================================================================
// SECTION 37: expr_collections.rs - Collection literals
// ============================================================================

#[test]
fn test_list_literal_empty() {
    let code = transpile("def foo() -> list:\n    return []");
    assert!(code.contains("vec!"), "Empty list should use vec!: {}", code);
}

#[test]
fn test_list_literal_with_elements() {
    let code = transpile("def foo() -> list:\n    return [1, 2, 3]");
    assert!(
        code.contains("vec!") && code.contains("1") && code.contains("2") && code.contains("3"),
        "List literal should use vec!: {}",
        code
    );
}

#[test]
fn test_dict_literal_empty() {
    let code = transpile("def foo() -> dict:\n    return {}");
    assert!(code.contains("HashMap"), "Empty dict should use HashMap: {}", code);
}

#[test]
fn test_dict_literal_with_entries() {
    let code = transpile("def foo() -> dict:\n    return {'a': 1, 'b': 2}");
    assert!(
        code.contains("HashMap") || code.contains("insert") || code.contains("DepylerValue"),
        "Dict literal should produce HashMap: {}",
        code
    );
}

#[test]
fn test_tuple_literal() {
    let code = transpile("def foo():\n    return (1, 'hello')");
    assert!(
        code.contains("1") && code.contains("hello"),
        "Tuple literal should contain elements: {}",
        code
    );
}

#[test]
fn test_set_literal() {
    let code = transpile("def foo() -> set:\n    return {1, 2, 3}");
    assert!(
        code.contains("HashSet") || code.contains("hash_set"),
        "Set literal should use HashSet: {}",
        code
    );
}

// ============================================================================
// SECTION 38: method_stmt_convert.rs - Class method body conversion
// ============================================================================

#[test]
fn test_class_method_with_self_assign() {
    let code = transpile(
        "class Foo:\n    def __init__(self, x: int):\n        self.x = x\n    def set_x(self, val: int):\n        self.x = val",
    );
    assert!(code.contains("self.x"), "Method body should access self.x: {}", code);
}

#[test]
fn test_class_method_return() {
    let code = transpile(
        "class Foo:\n    def __init__(self, x: int):\n        self.x = x\n    def get_x(self) -> int:\n        return self.x",
    );
    assert!(code.contains("return"), "Method body should have return: {}", code);
}

#[test]
fn test_class_method_with_if() {
    assert!(transpile_ok(
        "class Foo:\n    def __init__(self, x: int):\n        self.x = x\n    def check(self) -> bool:\n        if self.x > 0:\n            return True\n        return False"
    ));
}

#[test]
fn test_class_method_with_loop() {
    assert!(transpile_ok(
        "class Foo:\n    def __init__(self):\n        self.items = []\n    def add_items(self, n: int):\n        for i in range(n):\n            self.items.append(i)"
    ));
}

#[test]
fn test_class_method_optional_return() {
    assert!(transpile_ok(
        "class Foo:\n    def __init__(self, x: int):\n        self.x = x\n    def maybe_get(self) -> int:\n        if self.x > 0:\n            return self.x\n        return None"
    ));
}

// ============================================================================
// SECTION 39: expr_index_slice.rs - Indexing
// ============================================================================

#[test]
fn test_list_index_positive() {
    assert!(transpile_ok("def foo(items: list) -> int:\n    return items[0]"));
}

#[test]
fn test_list_index_negative() {
    let code = transpile("def foo(items: list) -> int:\n    return items[-1]");
    assert!(
        code.contains("len") || code.contains("wrapping"),
        "Negative index should handle wrapping: {}",
        code
    );
}

#[test]
fn test_dict_index_string_key() {
    let code = transpile("def foo():\n    data = {'key': 42}\n    return data['key']");
    assert!(
        code.contains("get") || code.contains("key"),
        "Dict string index should use get: {}",
        code
    );
}

// ============================================================================
// SECTION 40: expr_index_slice.rs - Slicing
// ============================================================================

#[test]
fn test_slice_basic() {
    assert!(transpile_ok("def foo(items: list) -> list:\n    return items[1:3]"));
}

#[test]
fn test_slice_from_start() {
    assert!(transpile_ok("def foo(items: list) -> list:\n    return items[:3]"));
}

#[test]
fn test_slice_to_end() {
    assert!(transpile_ok("def foo(items: list) -> list:\n    return items[1:]"));
}

#[test]
fn test_slice_negative() {
    assert!(transpile_ok("def foo(s: str) -> str:\n    return s[-3:]"));
}

#[test]
fn test_slice_with_step() {
    assert!(transpile_ok("def foo(items: list) -> list:\n    return items[::2]"));
}

// ============================================================================
// SECTION 41: stdlib_calls.rs - os.path functions
// ============================================================================

#[test]
fn test_os_path_exists() {
    assert!(transpile_ok(
        "def foo(p: str) -> bool:\n    from os.path import exists\n    return exists(p)"
    ));
}

#[test]
fn test_os_path_isfile() {
    assert!(transpile_ok(
        "def foo(p: str) -> bool:\n    from os.path import isfile\n    return isfile(p)"
    ));
}

#[test]
fn test_os_path_isdir() {
    assert!(transpile_ok(
        "def foo(p: str) -> bool:\n    from os.path import isdir\n    return isdir(p)"
    ));
}

#[test]
fn test_os_path_basename() {
    assert!(transpile_ok(
        "def foo(p: str) -> str:\n    from os.path import basename\n    return basename(p)"
    ));
}

#[test]
fn test_os_path_dirname() {
    assert!(transpile_ok(
        "def foo(p: str) -> str:\n    from os.path import dirname\n    return dirname(p)"
    ));
}

// ============================================================================
// SECTION 42: stdlib_calls.rs - Generic function calls
// ============================================================================

#[test]
fn test_constructor_call_capitalized() {
    assert!(transpile_ok("def foo():\n    p = Point(1, 2)\n    return p"));
}

#[test]
fn test_regular_function_call() {
    assert!(transpile_ok("def bar() -> int:\n    return 42\ndef foo() -> int:\n    return bar()"));
}

// ============================================================================
// SECTION 43: expr_operators.rs - Ternary (if expression)
// ============================================================================

#[test]
fn test_ternary_expression() {
    let code = transpile("def foo(x: int) -> str:\n    return 'pos' if x > 0 else 'neg'");
    assert!(
        code.contains("if") && code.contains("else"),
        "Ternary should produce if/else: {}",
        code
    );
}

// ============================================================================
// SECTION 44: expr_methods.rs - String method chaining
// ============================================================================

#[test]
fn test_string_method_chain() {
    assert!(transpile_ok("def foo(s: str) -> str:\n    return s.strip().upper()"));
}

#[test]
fn test_string_method_chain_split_join() {
    assert!(transpile_ok("def foo(s: str) -> str:\n    return '-'.join(s.split(','))"));
}

// ============================================================================
// SECTION 45: More complex transpilation patterns
// ============================================================================

#[test]
fn test_nested_list_comprehension() {
    assert!(transpile_ok("def foo() -> list:\n    return [x * y for x in range(3) if x > 0]"));
}

#[test]
fn test_function_with_default_param() {
    assert!(transpile_ok("def foo(x: int = 0) -> int:\n    return x + 1"));
}

#[test]
fn test_multiple_return_types() {
    assert!(transpile_ok(
        "def foo(x: int):\n    if x > 0:\n        return 'positive'\n    return 'non-positive'"
    ));
}

#[test]
fn test_nested_if_in_loop() {
    assert!(transpile_ok(
        "def foo() -> int:\n    count = 0\n    for i in range(10):\n        if i % 2 == 0:\n            count = count + 1\n    return count"
    ));
}

#[test]
fn test_list_comprehension_over_string() {
    assert!(transpile_ok("def foo(s: str) -> list:\n    return [c for c in s]"));
}

#[test]
fn test_dict_comprehension_from_list() {
    assert!(transpile_ok("def foo() -> dict:\n    return {str(i): i for i in range(5)}"));
}

// ============================================================================
// SECTION 46: Class with various method types
// ============================================================================

#[test]
fn test_class_with_staticmethod() {
    assert!(transpile_ok(
        "class Math:\n    @staticmethod\n    def add(a: int, b: int) -> int:\n        return a + b"
    ));
}

#[test]
fn test_class_with_classmethod() {
    assert!(transpile_ok(
        "class Factory:\n    @classmethod\n    def create(cls) -> 'Factory':\n        return cls()"
    ));
}

#[test]
fn test_class_multiple_methods() {
    assert!(transpile_ok(
        "class Stack:\n    def __init__(self):\n        self.items = []\n    def push(self, item: int):\n        self.items.append(item)\n    def pop(self) -> int:\n        return self.items.pop()\n    def is_empty(self) -> bool:\n        return len(self.items) == 0"
    ));
}

// ============================================================================
// SECTION 47: expr_builtins.rs - bytes, bytearray, tuple constructors
// ============================================================================

#[test]
fn test_builtin_bytes_empty() {
    assert!(transpile_ok("def foo() -> bytes:\n    return bytes()"));
}

#[test]
fn test_builtin_bytes_with_size() {
    assert!(transpile_ok("def foo() -> bytes:\n    return bytes(10)"));
}

#[test]
fn test_builtin_bytearray_empty() {
    assert!(transpile_ok("def foo():\n    return bytearray()"));
}

#[test]
fn test_builtin_tuple_empty() {
    assert!(transpile_ok("def foo():\n    return tuple()"));
}

// ============================================================================
// SECTION 48: Complex operator interactions
// ============================================================================

#[test]
fn test_mixed_arithmetic_and_comparison() {
    assert!(transpile_ok("def foo(a: int, b: int) -> bool:\n    return a + b > 10"));
}

#[test]
fn test_chained_comparison_style() {
    assert!(transpile_ok("def foo(x: int) -> bool:\n    return x > 0 and x < 100"));
}

#[test]
fn test_nested_function_calls_in_expression() {
    assert!(transpile_ok("def foo(items: list) -> int:\n    return len(items) + 1"));
}

#[test]
fn test_power_with_negative_exponent() {
    assert!(transpile_ok("def foo() -> float:\n    return 2 ** -1"));
}

// ============================================================================
// SECTION 49: Return types and coercion
// ============================================================================

#[test]
fn test_return_none() {
    assert!(transpile_ok("def foo() -> None:\n    return None"));
}

#[test]
fn test_return_implicit() {
    assert!(transpile_ok("def foo():\n    x = 42"));
}

#[test]
fn test_return_string_literal() {
    let code = transpile("def foo() -> str:\n    return 'hello'");
    assert!(code.contains("hello"), "Should return string: {}", code);
}

#[test]
fn test_return_bool_literal() {
    let code = transpile("def foo() -> bool:\n    return True");
    assert!(code.contains("true"), "Should return true: {}", code);
}

// ============================================================================
// SECTION 50: body_convert.rs - Expression purity
// ============================================================================

#[test]
fn test_pure_expression_as_statement() {
    // Pure expressions used as statements should get let _ = treatment
    let code = transpile("def foo(x: int):\n    x + 1\n    return x");
    assert!(
        code.contains("let _") || code.contains("_"),
        "Pure expression should use let _: {}",
        code
    );
}

#[test]
fn test_method_call_as_statement() {
    // Side-effectful method calls should NOT get let _ = treatment
    let code = transpile("def foo():\n    items = []\n    items.append(1)");
    assert!(code.contains("push"), "append should be a statement: {}", code);
}

// ============================================================================
// SECTION 51: Edge cases and integration
// ============================================================================

#[test]
fn test_empty_function() {
    assert!(transpile_ok("def foo():\n    pass"));
}

#[test]
fn test_function_with_only_return() {
    assert!(transpile_ok("def foo() -> int:\n    return 0"));
}

#[test]
fn test_deeply_nested_expressions() {
    assert!(transpile_ok(
        "def foo(a: int, b: int, c: int) -> int:\n    return (a + b) * (c - a) + (b * c)"
    ));
}

#[test]
fn test_multiple_functions() {
    assert!(transpile_ok(
        "def add(a: int, b: int) -> int:\n    return a + b\ndef mul(a: int, b: int) -> int:\n    return a * b"
    ));
}

#[test]
fn test_function_calling_function() {
    assert!(transpile_ok(
        "def double(x: int) -> int:\n    return x * 2\ndef foo() -> int:\n    return double(5)"
    ));
}

#[test]
fn test_complex_class_with_all_features() {
    assert!(transpile_ok(
        "class Calculator:\n    def __init__(self):\n        self.result = 0\n    def add(self, x: int):\n        self.result = self.result + x\n    def get_result(self) -> int:\n        return self.result\n    def reset(self):\n        self.result = 0"
    ));
}

#[test]
fn test_while_true_with_conditional_break() {
    assert!(transpile_ok(
        "def foo() -> int:\n    x = 0\n    while True:\n        x = x + 1\n        if x >= 10:\n            break\n    return x"
    ));
}

#[test]
fn test_for_with_enumerate_and_condition() {
    assert!(transpile_ok(
        "def foo(items: list) -> int:\n    count = 0\n    for i, item in enumerate(items):\n        if item > 0:\n            count = count + 1\n    return count"
    ));
}

#[test]
fn test_nested_dict_access() {
    assert!(transpile_ok("def foo():\n    data = {'a': 1}\n    x = data['a']\n    return x"));
}

#[test]
fn test_string_formatting_with_fstring_and_method() {
    assert!(transpile_ok("def foo(name: str) -> str:\n    return f'Hello {name.upper()}'"));
}
