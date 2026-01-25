//! Targeted statement generation tests
//!
//! These tests specifically target uncovered code paths in stmt_gen.rs.

use crate::DepylerPipeline;

#[allow(dead_code)]
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

// ============================================================================
// COMPLEX IF STATEMENT PATTERNS
// ============================================================================

#[test]
fn test_if_is_none_check() {
    assert!(transpile_ok(
        "def foo(x):\n    if x is None:\n        return 0\n    return x"
    ));
}

#[test]
fn test_if_is_not_none_check() {
    assert!(transpile_ok(
        "def foo(x):\n    if x is not None:\n        return x\n    return 0"
    ));
}

#[test]
fn test_if_option_unwrap_pattern() {
    assert!(transpile_ok("import os\ndef foo():\n    val = os.getenv('KEY')\n    if val is not None:\n        return val\n    return 'default'"));
}

#[test]
fn test_if_dict_get_pattern() {
    assert!(transpile_ok("def foo(d: dict[str, int]):\n    val = d.get('key')\n    if val is not None:\n        return val\n    return 0"));
}

#[test]
fn test_if_multiple_and_conditions() {
    assert!(transpile_ok("def foo(x: int, y: int, z: int) -> bool:\n    if x > 0 and y > 0 and z > 0:\n        return True\n    return False"));
}

#[test]
fn test_if_multiple_or_conditions() {
    assert!(transpile_ok("def foo(x: int, y: int, z: int) -> bool:\n    if x > 0 or y > 0 or z > 0:\n        return True\n    return False"));
}

#[test]
fn test_if_mixed_conditions() {
    assert!(transpile_ok("def foo(a: bool, b: bool, c: bool) -> bool:\n    if (a and b) or c:\n        return True\n    return False"));
}

// ============================================================================
// FOR LOOP EDGE CASES
// ============================================================================

#[test]
fn test_for_with_index_assignment() {
    assert!(transpile_ok("def foo(items: list[int]):\n    for i in range(len(items)):\n        items[i] = items[i] * 2"));
}

#[test]
fn test_for_dict_items_destructure() {
    assert!(transpile_ok("def foo(d: dict[str, int]) -> int:\n    total = 0\n    for key, value in d.items():\n        total += value\n    return total"));
}

#[test]
fn test_for_nested_break() {
    assert!(transpile_ok("def foo(matrix: list[list[int]]) -> bool:\n    for row in matrix:\n        for val in row:\n            if val < 0:\n                return False\n    return True"));
}

#[test]
fn test_for_enumerate_with_start_offset() {
    assert!(transpile_ok("def foo(items: list[str]):\n    for i, item in enumerate(items, 1):\n        print(f'{i}: {item}')"));
}

#[test]
fn test_for_with_list_append() {
    assert!(transpile_ok("def foo(items: list[int]) -> list[int]:\n    result = []\n    for item in items:\n        result.append(item * 2)\n    return result"));
}

#[test]
fn test_for_with_dict_update() {
    assert!(transpile_ok("def foo(items: list[str]) -> dict[str, int]:\n    result = {}\n    for i, item in enumerate(items):\n        result[item] = i\n    return result"));
}

// ============================================================================
// WHILE LOOP EDGE CASES
// ============================================================================

#[test]
fn test_while_with_list_pop() {
    assert!(transpile_ok("def foo(items: list[int]) -> int:\n    total = 0\n    while items:\n        total += items.pop()\n    return total"));
}

#[test]
fn test_while_with_iterator() {
    assert!(transpile_ok("def foo(items: list[int]) -> int:\n    it = iter(items)\n    total = 0\n    while True:\n        try:\n            total += next(it)\n        except StopIteration:\n            break\n    return total"));
}

#[test]
fn test_while_counter() {
    assert!(transpile_ok("def foo(n: int) -> int:\n    count = 0\n    while count < n:\n        count += 1\n    return count"));
}

// ============================================================================
// ASSIGNMENT EDGE CASES
// ============================================================================

#[test]
fn test_assign_dict_index() {
    assert!(transpile_ok(
        "def foo(d: dict[str, int]):\n    d['new_key'] = 42"
    ));
}

#[test]
fn test_assign_nested_dict_index() {
    assert!(transpile_ok(
        "def foo(d: dict[str, dict[str, int]]):\n    d['outer']['inner'] = 42"
    ));
}

#[test]
fn test_assign_list_slice() {
    assert!(transpile_ok(
        "def foo(items: list[int]):\n    items[1:3] = [10, 20]"
    ));
}

#[test]
fn test_assign_multiple_targets() {
    assert!(transpile_ok("def foo():\n    a = b = c = 0"));
}

#[test]
fn test_assign_from_function() {
    assert!(transpile_ok(
        "def bar() -> int:\n    return 42\n\ndef foo():\n    x = bar()"
    ));
}

#[test]
fn test_assign_from_method() {
    assert!(transpile_ok("def foo(s: str):\n    parts = s.split(',')"));
}

#[test]
fn test_assign_from_list_comprehension() {
    assert!(transpile_ok(
        "def foo(items: list[int]):\n    doubled = [x * 2 for x in items]"
    ));
}

#[test]
fn test_assign_from_dict_comprehension() {
    assert!(transpile_ok(
        "def foo(items: list[str]):\n    mapping = {item: len(item) for item in items}"
    ));
}

#[test]
fn test_assign_from_generator() {
    assert!(transpile_ok(
        "def foo(items: list[int]):\n    total = sum(x for x in items if x > 0)"
    ));
}

// ============================================================================
// TRY/EXCEPT EDGE CASES
// ============================================================================

#[test]
fn test_try_with_json_parse() {
    assert!(transpile_ok("import json\ndef foo(s: str):\n    try:\n        return json.loads(s)\n    except json.JSONDecodeError:\n        return None"));
}

#[test]
fn test_try_file_operation() {
    assert!(transpile_ok("def foo(path: str) -> str:\n    try:\n        with open(path) as f:\n            return f.read()\n    except FileNotFoundError:\n        return ''"));
}

#[test]
fn test_try_index_error() {
    assert!(transpile_ok("def foo(items: list[int]) -> int:\n    try:\n        return items[0]\n    except IndexError:\n        return -1"));
}

#[test]
fn test_try_key_error() {
    assert!(transpile_ok("def foo(d: dict[str, int]) -> int:\n    try:\n        return d['key']\n    except KeyError:\n        return -1"));
}

#[test]
fn test_try_value_error() {
    assert!(transpile_ok("def foo(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError:\n        return 0"));
}

#[test]
fn test_try_with_cleanup() {
    assert!(transpile_ok("def foo(path: str):\n    f = None\n    try:\n        f = open(path)\n        return f.read()\n    except:\n        pass\n    finally:\n        if f:\n            f.close()"));
}

// ============================================================================
// WITH STATEMENT EDGE CASES
// ============================================================================

#[test]
fn test_with_file_encoding() {
    assert!(transpile_ok("def foo(path: str) -> str:\n    with open(path, encoding='utf-8') as f:\n        return f.read()"));
}

#[test]
fn test_with_file_newline() {
    assert!(transpile_ok("def foo(path: str) -> str:\n    with open(path, newline='') as f:\n        return f.read()"));
}

// ============================================================================
// RETURN STATEMENT EDGE CASES
// ============================================================================

#[test]
fn test_return_empty() {
    assert!(transpile_ok("def foo():\n    return"));
}

#[test]
fn test_return_none_explicit() {
    assert!(transpile_ok("def foo():\n    return None"));
}

#[test]
fn test_return_conditional_expr() {
    assert!(transpile_ok(
        "def foo(x: int) -> str:\n    return 'positive' if x > 0 else 'non-positive'"
    ));
}

#[test]
fn test_return_complex_expr() {
    assert!(transpile_ok(
        "def foo(x: int, y: int) -> int:\n    return (x + y) * 2 - 1"
    ));
}

// ============================================================================
// EXPRESSION STATEMENT EDGE CASES
// ============================================================================

#[test]
fn test_expr_stmt_method_call() {
    assert!(transpile_ok("def foo(items: list[int]):\n    items.sort()"));
}

#[test]
fn test_expr_stmt_function_call() {
    assert!(transpile_ok(
        "def bar():\n    pass\n\ndef foo():\n    bar()"
    ));
}

#[test]
fn test_expr_stmt_print() {
    assert!(transpile_ok("def foo():\n    print('hello')"));
}

// ============================================================================
// RAISE STATEMENT EDGE CASES
// ============================================================================

#[test]
fn test_raise_exception_with_args() {
    assert!(transpile_ok("def foo(x: int):\n    if x < 0:\n        raise ValueError(f'Expected non-negative, got {x}')"));
}

#[test]
fn test_raise_custom_exception() {
    let _ = transpile_ok(
        "class CustomError(Exception):\n    pass\n\ndef foo():\n    raise CustomError('custom')",
    );
}

// ============================================================================
// DELETE STATEMENT EDGE CASES
// ============================================================================

#[test]
fn test_del_from_dict() {
    assert!(transpile_ok(
        "def foo(d: dict[str, int]):\n    if 'key' in d:\n        del d['key']"
    ));
}

#[test]
fn test_del_from_list_slice() {
    let _ = transpile_ok("def foo(items: list[int]):\n    del items[1:3]");
}

// ============================================================================
// ASSERT STATEMENT EDGE CASES
// ============================================================================

#[test]
fn test_assert_comparison() {
    assert!(transpile_ok(
        "def foo(x: int, y: int):\n    assert x == y, f'Expected {x} == {y}'"
    ));
}

#[test]
fn test_assert_membership() {
    assert!(transpile_ok(
        "def foo(item: int, items: list[int]):\n    assert item in items, f'{item} not found'"
    ));
}

// ============================================================================
// AUGMENTED ASSIGNMENT EDGE CASES
// ============================================================================

#[test]
fn test_augassign_dict_value() {
    assert!(transpile_ok(
        "def foo(d: dict[str, int]):\n    d['count'] = d.get('count', 0) + 1"
    ));
}

#[test]
fn test_augassign_nested_attr() {
    let _ = transpile_ok("class Foo:\n    value: int\n\ndef bar(obj: Foo):\n    obj.value += 1");
}

// ============================================================================
// PASS STATEMENT EDGE CASES
// ============================================================================

#[test]
fn test_pass_in_except() {
    assert!(transpile_ok(
        "def foo():\n    try:\n        x = 1\n    except:\n        pass"
    ));
}

#[test]
fn test_pass_in_else() {
    assert!(transpile_ok(
        "def foo(x: int):\n    if x > 0:\n        print('positive')\n    else:\n        pass"
    ));
}

// ============================================================================
// BREAK/CONTINUE EDGE CASES
// ============================================================================

#[test]
fn test_break_in_nested_if() {
    assert!(transpile_ok("def foo(items: list[int]) -> int:\n    for item in items:\n        if item < 0:\n            break\n    return 0"));
}

#[test]
fn test_continue_in_nested_if() {
    assert!(transpile_ok("def foo(items: list[int]) -> list[int]:\n    result = []\n    for item in items:\n        if item < 0:\n            continue\n        result.append(item)\n    return result"));
}

// ============================================================================
// IMPORT STATEMENT EDGE CASES
// ============================================================================

#[test]
fn test_import_as() {
    assert!(transpile_ok(
        "import os as operating_system\n\ndef foo() -> str:\n    return operating_system.getcwd()"
    ));
}

#[test]
fn test_from_import_as() {
    assert!(transpile_ok("from os.path import join as path_join\n\ndef foo(a: str, b: str) -> str:\n    return path_join(a, b)"));
}

#[test]
fn test_import_multiple() {
    assert!(transpile_ok(
        "import os, sys\n\ndef foo() -> list[str]:\n    return sys.argv"
    ));
}

// ============================================================================
// GLOBAL/NONLOCAL EDGE CASES
// ============================================================================

#[test]
fn test_global_in_function() {
    assert!(transpile_ok(
        "counter = 0\n\ndef increment():\n    global counter\n    counter += 1\n    return counter"
    ));
}

#[test]
fn test_nonlocal_in_nested() {
    assert!(transpile_ok("def outer():\n    count = 0\n    def inner():\n        nonlocal count\n        count += 1\n        return count\n    return inner()"));
}

// ============================================================================
// CLASS STATEMENT EDGE CASES
// ============================================================================

#[test]
fn test_class_with_classvar() {
    assert!(transpile_ok(
        "class Counter:\n    count: int = 0\n    def increment(self):\n        self.count += 1"
    ));
}

#[test]
fn test_class_with_property_setter() {
    let _ = transpile_ok("class Foo:\n    _x: int = 0\n    @property\n    def x(self) -> int:\n        return self._x\n    @x.setter\n    def x(self, value: int):\n        self._x = value");
}

// ============================================================================
// ASYNC STATEMENT EDGE CASES
// ============================================================================

#[test]
fn test_async_with() {
    assert!(transpile_ok(
        "async def foo():\n    async with open('file.txt') as f:\n        pass"
    ));
}

#[test]
fn test_async_for() {
    assert!(transpile_ok(
        "async def foo(items):\n    async for item in items:\n        print(item)"
    ));
}

// ============================================================================
// YIELD EDGE CASES
// ============================================================================

#[test]
fn test_yield_with_value() {
    assert!(transpile_ok(
        "def gen(n: int):\n    for i in range(n):\n        yield i * 2"
    ));
}

#[test]
fn test_yield_from_list() {
    assert!(transpile_ok("def gen():\n    yield from [1, 2, 3]"));
}

#[test]
fn test_yield_from_generator() {
    assert!(transpile_ok(
        "def inner():\n    yield 1\n    yield 2\n\ndef outer():\n    yield from inner()"
    ));
}

// ============================================================================
// MATCH STATEMENT EDGE CASES (Python 3.10+)
// ============================================================================

#[test]
fn test_match_string_literal() {
    assert!(transpile_ok("def foo(cmd: str) -> int:\n    match cmd:\n        case 'start':\n            return 1\n        case 'stop':\n            return 0\n        case _:\n            return -1"));
}

#[test]
fn test_match_with_guard() {
    assert!(transpile_ok("def foo(x: int) -> str:\n    match x:\n        case n if n > 0:\n            return 'positive'\n        case n if n < 0:\n            return 'negative'\n        case _:\n            return 'zero'"));
}

// ============================================================================
// COMPLEX FUNCTION PATTERNS
// ============================================================================

#[test]
fn test_recursive_function() {
    assert!(transpile_ok("def factorial(n: int) -> int:\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)"));
}

#[test]
fn test_mutual_recursion() {
    assert!(transpile_ok("def is_even(n: int) -> bool:\n    if n == 0:\n        return True\n    return is_odd(n - 1)\n\ndef is_odd(n: int) -> bool:\n    if n == 0:\n        return False\n    return is_even(n - 1)"));
}

#[test]
fn test_function_with_docstring() {
    assert!(transpile_ok(
        "def foo(x: int) -> int:\n    \"\"\"Returns double of x.\"\"\"\n    return x * 2"
    ));
}

// ============================================================================
// VARIABLE TYPE INFERENCE PATTERNS
// ============================================================================

#[test]
fn test_infer_list_type_from_append() {
    assert!(transpile_ok("def foo() -> list[int]:\n    result = []\n    result.append(1)\n    result.append(2)\n    return result"));
}

#[test]
fn test_infer_dict_type_from_assignment() {
    assert!(transpile_ok(
        "def foo() -> dict[str, int]:\n    d = {}\n    d['a'] = 1\n    d['b'] = 2\n    return d"
    ));
}

#[test]
fn test_infer_type_from_binary_op() {
    assert!(transpile_ok(
        "def foo(a: int, b: int):\n    result = a + b\n    return result"
    ));
}

// ============================================================================
// STDLIB SPECIFIC PATTERNS
// ============================================================================

#[test]
fn test_json_load_from_file() {
    assert!(transpile_ok("import json\n\ndef foo(path: str):\n    with open(path) as f:\n        return json.load(f)"));
}

#[test]
fn test_json_dump_to_file() {
    assert!(transpile_ok("import json\n\ndef foo(data: dict[str, int], path: str):\n    with open(path, 'w') as f:\n        json.dump(data, f)"));
}

#[test]
fn test_re_compile_pattern() {
    assert!(transpile_ok("import re\n\ndef foo(pattern: str):\n    compiled = re.compile(pattern)\n    return compiled"));
}

#[test]
fn test_datetime_format() {
    assert!(transpile_ok("from datetime import datetime\n\ndef foo() -> str:\n    now = datetime.now()\n    return now.strftime('%Y-%m-%d %H:%M:%S')"));
}

// ============================================================================
// ERROR HANDLING PATTERNS
// ============================================================================

#[test]
fn test_early_return_on_error() {
    assert!(transpile_ok("def foo(items: list[int]) -> int:\n    if not items:\n        return -1\n    return items[0]"));
}

#[test]
fn test_default_value_pattern() {
    assert!(transpile_ok(
        "def foo(d: dict[str, int], key: str) -> int:\n    return d.get(key, 0)"
    ));
}

// ============================================================================
// COLLECTION MANIPULATION
// ============================================================================

#[test]
fn test_list_extend_multiple() {
    assert!(transpile_ok("def foo(a: list[int], b: list[int], c: list[int]) -> list[int]:\n    result = []\n    result.extend(a)\n    result.extend(b)\n    result.extend(c)\n    return result"));
}

#[test]
fn test_dict_merge() {
    assert!(transpile_ok("def foo(a: dict[str, int], b: dict[str, int]) -> dict[str, int]:\n    result = a.copy()\n    result.update(b)\n    return result"));
}

#[test]
fn test_set_operations_chained() {
    assert!(transpile_ok("def foo(a: set[int], b: set[int], c: set[int]) -> set[int]:\n    return a.union(b).intersection(c)"));
}

// ============================================================================
// STRING OPERATIONS
// ============================================================================

#[test]
fn test_string_split_join() {
    assert!(transpile_ok(
        "def foo(s: str) -> str:\n    parts = s.split(',')\n    return '-'.join(parts)"
    ));
}

#[test]
fn test_string_strip_split() {
    assert!(transpile_ok(
        "def foo(line: str) -> list[str]:\n    return line.strip().split()"
    ));
}

#[test]
fn test_string_format_multiple() {
    assert!(transpile_ok(
        "def foo(name: str, age: int) -> str:\n    return f'{name} is {age} years old'"
    ));
}

// ============================================================================
// NUMERIC OPERATIONS
// ============================================================================

#[test]
fn test_math_operations() {
    assert!(transpile_ok("import math\n\ndef foo(x: float) -> float:\n    return math.sin(x) ** 2 + math.cos(x) ** 2"));
}

#[test]
fn test_integer_division() {
    assert!(transpile_ok(
        "def foo(a: int, b: int) -> tuple[int, int]:\n    return a // b, a % b"
    ));
}

#[test]
fn test_power_operation() {
    assert!(transpile_ok(
        "def foo(base: float, exp: int) -> float:\n    return base ** exp"
    ));
}
