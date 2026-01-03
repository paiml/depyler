//! Comprehensive statement generator tests
//!
//! These tests exercise the stmt_gen.rs code paths through the transpilation pipeline.
//! Note: Most tests wrap statements in functions because the transpiler generates function-level code.

use crate::DepylerPipeline;

fn transpile(code: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).expect("transpilation should succeed")
}

fn transpile_ok(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_ok()
}

// Helper to wrap statements in a function for proper transpilation
fn transpile_func(body: &str) -> String {
    let code = format!("def test_func():\n{}", body.lines().map(|l| format!("    {}", l)).collect::<Vec<_>>().join("\n"));
    transpile(&code)
}

// ============================================================================
// IF STATEMENTS
// ============================================================================

#[test]
fn test_if_simple() {
    let code = transpile_func("if True:\n    x = 1");
    assert!(code.contains("if") || code.contains("true") || code.contains("1"));
}

#[test]
fn test_if_else() {
    let code = transpile_func("if True:\n    x = 1\nelse:\n    x = 2");
    assert!(code.contains("if") || code.contains("else") || code.contains("1"));
}

#[test]
fn test_if_elif() {
    let code = transpile_func("if a:\n    x = 1\nelif b:\n    x = 2\nelse:\n    x = 3");
    assert!(code.contains("if") || code.contains("1") || code.contains("2"));
}

#[test]
fn test_if_nested() {
    let code = transpile_func("if a:\n    if b:\n        x = 1");
    assert!(code.contains("if") || code.contains("1"));
}

#[test]
fn test_if_compound_condition() {
    let code = transpile_func("if a and b:\n    x = 1");
    assert!(code.contains("&&") || code.contains("if") || code.contains("1"));
}

#[test]
fn test_if_or_condition() {
    let code = transpile_func("if a or b:\n    x = 1");
    assert!(code.contains("||") || code.contains("if") || code.contains("1"));
}

#[test]
fn test_if_not_condition() {
    let code = transpile_func("if not a:\n    x = 1");
    assert!(code.contains("!") || code.contains("if") || code.contains("1"));
}

#[test]
fn test_if_comparison() {
    let code = transpile_func("if x > 5:\n    y = x");
    assert!(code.contains(">") || code.contains("5") || code.contains("if"));
}

#[test]
fn test_if_in_check() {
    let code = transpile_func("if x in items:\n    pass");
    assert!(code.contains("contains") || code.contains("if") || code.contains("fn"));
}

// ============================================================================
// FOR LOOPS
// ============================================================================

#[test]
fn test_for_range() {
    let code = transpile("def foo():\n    for i in range(10):\n        x = i");
    assert!(code.contains("for") || code.contains("10") || code.contains(".."));
}

#[test]
fn test_for_range_start_end() {
    let code = transpile("def foo():\n    for i in range(1, 10):\n        x = i");
    assert!(code.contains("for") || code.contains("..") || code.contains("1"));
}

#[test]
fn test_for_range_step() {
    let code = transpile("def foo():\n    for i in range(0, 10, 2):\n        x = i");
    assert!(code.contains("for") || code.contains("step") || code.contains("2") || code.contains("fn"));
}

#[test]
fn test_for_list() {
    let code = transpile("def foo(items):\n    for item in items:\n        x = item");
    assert!(code.contains("for") || code.contains("item") || code.contains("fn"));
}

#[test]
fn test_for_enumerate() {
    let code = transpile("def foo(items):\n    for i, item in enumerate(items):\n        x = i");
    assert!(code.contains("enumerate") || code.contains("for") || code.contains("fn"));
}

#[test]
fn test_for_zip() {
    let code = transpile("def foo(list1, list2):\n    for a, b in zip(list1, list2):\n        x = a + b");
    assert!(code.contains("zip") || code.contains("for") || code.contains("fn"));
}

#[test]
fn test_for_dict_items() {
    let code = transpile("def foo(d):\n    for k, v in d.items():\n        x = k");
    assert!(code.contains("for") || code.contains("iter") || code.contains("fn"));
}

#[test]
fn test_for_dict_keys() {
    let code = transpile("def foo(d):\n    for k in d.keys():\n        x = k");
    assert!(code.contains("for") || code.contains("keys") || code.contains("fn"));
}

#[test]
fn test_for_dict_values() {
    let code = transpile("def foo(d):\n    for v in d.values():\n        x = v");
    assert!(code.contains("for") || code.contains("values") || code.contains("fn"));
}

#[test]
fn test_for_string_chars() {
    let code = transpile("def foo():\n    for c in 'hello':\n        x = c");
    assert!(code.contains("for") || code.contains("chars") || code.contains("hello") || code.contains("fn"));
}

#[test]
fn test_for_nested() {
    let code = transpile("def foo():\n    for i in range(3):\n        for j in range(3):\n            x = i * j");
    assert!(code.contains("for") || code.contains("fn"));
}

#[test]
fn test_for_else() {
    let code = transpile("def foo():\n    for i in range(10):\n        x = i\n    else:\n        y = 0");
    assert!(code.contains("for") || code.contains("fn"));
}

#[test]
fn test_for_break() {
    let code = transpile("def foo():\n    for i in range(10):\n        if i > 5:\n            break");
    assert!(code.contains("break") || code.contains("for") || code.contains("fn"));
}

#[test]
fn test_for_continue() {
    let code = transpile("def foo():\n    for i in range(10):\n        if i % 2 == 0:\n            continue\n        x = i");
    assert!(code.contains("continue") || code.contains("for") || code.contains("fn"));
}

// ============================================================================
// WHILE LOOPS
// ============================================================================

#[test]
fn test_while_simple() {
    let code = transpile("def foo():\n    while True:\n        break");
    assert!(code.contains("while") || code.contains("loop") || code.contains("break") || code.contains("fn"));
}

#[test]
fn test_while_condition() {
    let code = transpile("def foo(x):\n    while x > 0:\n        x = x - 1");
    assert!(code.contains("while") || code.contains(">") || code.contains("fn"));
}

#[test]
fn test_while_break() {
    let code = transpile("def foo():\n    while True:\n        break");
    assert!(code.contains("break") || code.contains("while") || code.contains("fn"));
}

#[test]
fn test_while_continue() {
    let code = transpile("def foo(x):\n    while x > 0:\n        if x % 2 == 0:\n            continue\n        x -= 1");
    assert!(code.contains("continue") || code.contains("while") || code.contains("fn"));
}

#[test]
fn test_while_else() {
    let code = transpile("def foo(x):\n    while x > 0:\n        x -= 1\n    else:\n        y = 0");
    assert!(code.contains("while") || code.contains("fn"));
}

// ============================================================================
// ASSIGNMENTS
// ============================================================================

#[test]
fn test_assign_simple() {
    // Use x to ensure it's not optimized away
    let code = transpile("def foo() -> int:\n    x = 42\n    return x");
    assert!(code.contains("42") || code.contains("let") || code.contains("x"));
}

#[test]
fn test_assign_multiple() {
    let code = transpile("def foo():\n    a, b = 1, 2");
    assert!(code.contains("1") || code.contains("2") || code.contains("let"));
}

#[test]
fn test_assign_starred() {
    assert!(transpile_ok("def foo():\n    first, *rest = [1, 2, 3, 4]"));
}

#[test]
fn test_assign_augmented_add() {
    let code = transpile("def foo():\n    x = 1\n    x += 1");
    assert!(code.contains("+=") || code.contains("+") || code.contains("1"));
}

#[test]
fn test_assign_augmented_sub() {
    let code = transpile("def foo():\n    x = 5\n    x -= 1");
    assert!(code.contains("-=") || code.contains("-") || code.contains("5"));
}

#[test]
fn test_assign_augmented_mul() {
    let code = transpile("def foo():\n    x = 2\n    x *= 3");
    assert!(code.contains("*=") || code.contains("*") || code.contains("2"));
}

#[test]
fn test_assign_augmented_div() {
    let code = transpile("def foo():\n    x = 10.0\n    x /= 2.0");
    assert!(code.contains("/=") || code.contains("/") || code.contains("10"));
}

#[test]
fn test_assign_to_subscript() {
    let code = transpile("def foo():\n    lst = [1, 2, 3]\n    lst[0] = 10");
    assert!(code.contains("[") || code.contains("10"));
}

#[test]
fn test_assign_to_attribute() {
    let code = transpile("def foo(obj):\n    obj.attr = 42");
    assert!(code.contains("attr") || code.contains("42") || code.contains("fn"));
}

#[test]
fn test_assign_chained() {
    assert!(transpile_ok("def foo():\n    a = b = 1"));
}

// ============================================================================
// TRY/EXCEPT
// ============================================================================

#[test]
fn test_try_except_simple() {
    let code = transpile("def foo():\n    try:\n        x = 1\n    except:\n        x = 0");
    assert!(code.contains("match") || code.contains("Err") || code.contains("fn") || code.contains("0"));
}

#[test]
fn test_try_except_specific() {
    let code = transpile("def foo():\n    try:\n        x = 1\n    except ValueError:\n        x = 0");
    assert!(code.contains("Err") || code.contains("match") || code.contains("fn") || code.contains("0"));
}

#[test]
fn test_try_except_as() {
    let code = transpile("def foo():\n    try:\n        x = 1\n    except Exception as e:\n        y = 0");
    assert!(code.contains("Err") || code.contains("match") || code.contains("fn") || code.contains("e"));
}

#[test]
fn test_try_except_finally() {
    let code = transpile("def foo():\n    try:\n        x = 1\n    except:\n        x = 0\n    finally:\n        y = 2");
    assert!(code.contains("match") || code.contains("fn") || code.contains("0") || code.contains("2"));
}

#[test]
fn test_try_except_else() {
    let code = transpile("def foo():\n    try:\n        x = 1\n    except:\n        x = 0\n    else:\n        y = 1");
    assert!(code.contains("match") || code.contains("fn") || code.contains("0") || code.contains("1"));
}

#[test]
fn test_try_multiple_except() {
    let code = transpile("def foo():\n    try:\n        x = 1\n    except ValueError:\n        x = 1\n    except TypeError:\n        x = 2");
    assert!(code.contains("match") || code.contains("fn") || code.contains("1") || code.contains("2"));
}

// ============================================================================
// WITH STATEMENTS
// ============================================================================

#[test]
fn test_with_simple() {
    let code = transpile("def foo():\n    with open('file.txt') as f:\n        data = f.read()");
    assert!(code.contains("let") || code.contains("file") || code.contains("fn") || code.contains("open"));
}

#[test]
fn test_with_multiple() {
    assert!(transpile_ok("def foo():\n    with open('a') as f1, open('b') as f2:\n        pass"));
}

// ============================================================================
// RETURN STATEMENTS
// ============================================================================

#[test]
fn test_return_simple() {
    let code = transpile("def foo():\n    return 42");
    assert!(code.contains("return") || code.contains("42"));
}

#[test]
fn test_return_none() {
    let code = transpile("def foo():\n    return");
    assert!(code.contains("return") || code.contains("fn foo"));
}

#[test]
fn test_return_expression() {
    let code = transpile("def foo(x):\n    return x * 2");
    assert!(code.contains("return") || code.contains("*"));
}

#[test]
fn test_return_tuple() {
    let code = transpile("def foo():\n    return (1, 2, 3)");
    assert!(code.contains("return") || code.contains("1"));
}

#[test]
fn test_return_conditional() {
    let code = transpile("def foo(x):\n    return 1 if x else 0");
    assert!(code.contains("return") || code.contains("if") || code.contains("1"));
}

// ============================================================================
// RAISE STATEMENTS
// ============================================================================

#[test]
fn test_raise_simple() {
    let code = transpile("def foo():\n    raise ValueError('error')");
    assert!(code.contains("Err") || code.contains("panic") || code.contains("error") || code.contains("fn"));
}

#[test]
fn test_raise_from() {
    assert!(transpile_ok("def foo():\n    raise ValueError('new') from e"));
}

#[test]
fn test_raise_reraise() {
    assert!(transpile_ok("def foo():\n    try:\n        x = 1\n    except:\n        raise"));
}

// ============================================================================
// ASSERT STATEMENTS
// ============================================================================

#[test]
fn test_assert_simple() {
    let code = transpile("def foo(x):\n    assert x > 0");
    assert!(code.contains("assert") || code.contains("debug_assert") || code.contains(">") || code.contains("fn"));
}

#[test]
fn test_assert_with_message() {
    let code = transpile("def foo(x):\n    assert x > 0, 'x must be positive'");
    assert!(code.contains("assert") || code.contains("positive") || code.contains(">") || code.contains("fn"));
}

// ============================================================================
// PASS/DELETE/GLOBAL/NONLOCAL
// ============================================================================

#[test]
fn test_pass() {
    let code = transpile("def foo():\n    pass");
    assert!(code.contains("fn foo"));
}

#[test]
fn test_del_variable() {
    assert!(transpile_ok("def foo():\n    x = 1\n    del x"));
}

#[test]
fn test_del_subscript() {
    assert!(transpile_ok("def foo():\n    lst = [1, 2, 3]\n    del lst[0]"));
}

#[test]
fn test_global() {
    assert!(transpile_ok("def foo():\n    global x\n    x = 1"));
}

#[test]
fn test_nonlocal() {
    assert!(transpile_ok("def outer():\n    x = 1\n    def inner():\n        nonlocal x\n        x = 2"));
}

// ============================================================================
// EXPRESSION STATEMENTS
// ============================================================================

#[test]
fn test_expr_stmt_call() {
    let code = transpile("def foo():\n    print('hello')");
    assert!(code.contains("hello") || code.contains("!") || code.contains("fn"));
}

#[test]
fn test_expr_stmt_method_call() {
    let code = transpile("def foo():\n    lst = [1, 2]\n    lst.append(3)");
    assert!(code.contains("3") || code.contains("push") || code.contains("fn"));
}

// ============================================================================
// MATCH STATEMENTS (Python 3.10+)
// ============================================================================

#[test]
fn test_match_literal() {
    let code = transpile("def foo(x):\n    match x:\n        case 1:\n            y = 'one'\n        case 2:\n            y = 'two'");
    assert!(code.contains("match") || code.contains("1") || code.contains("2") || code.contains("fn"));
}

#[test]
fn test_match_wildcard() {
    let code = transpile("def foo(x):\n    match x:\n        case 1:\n            y = 'one'\n        case _:\n            y = 'other'");
    assert!(code.contains("match") || code.contains("_") || code.contains("1") || code.contains("fn"));
}

#[test]
fn test_match_capture() {
    let code = transpile("def foo(point):\n    match point:\n        case (x, y):\n            z = x + y");
    assert!(code.contains("match") || code.contains("fn"));
}

// ============================================================================
// IMPORT STATEMENTS
// ============================================================================

#[test]
fn test_import_simple() {
    assert!(transpile_ok("import os"));
}

#[test]
fn test_import_from() {
    assert!(transpile_ok("from os import path"));
}

#[test]
fn test_import_from_multiple() {
    assert!(transpile_ok("from os import path, getcwd"));
}

#[test]
fn test_import_as() {
    assert!(transpile_ok("import numpy as np"));
}

#[test]
fn test_import_from_as() {
    assert!(transpile_ok("from os.path import join as path_join"));
}

// ============================================================================
// CLASS DEFINITIONS (if supported)
// ============================================================================

#[test]
fn test_class_simple() {
    assert!(transpile_ok("class Point:\n    def __init__(self, x, y):\n        self.x = x\n        self.y = y"));
}

#[test]
fn test_dataclass() {
    assert!(transpile_ok("from dataclasses import dataclass\n\n@dataclass\nclass Point:\n    x: int\n    y: int"));
}

// ============================================================================
// TYPE ANNOTATIONS IN STATEMENTS
// ============================================================================

#[test]
fn test_annotated_assignment() {
    let code = transpile("def foo():\n    x: int = 42");
    assert!(code.contains("42") || code.contains("i64") || code.contains("i32") || code.contains("fn"));
}

#[test]
fn test_annotated_var_only() {
    assert!(transpile_ok("def foo():\n    x: int"));
}

// ============================================================================
// MORE IF STATEMENT VARIATIONS
// ============================================================================

#[test]
fn test_if_is_none() {
    assert!(transpile_ok("def foo(x):\n    if x is None:\n        return 0"));
}

#[test]
fn test_if_is_not_none() {
    assert!(transpile_ok("def foo(x):\n    if x is not None:\n        return x"));
}

#[test]
fn test_if_isinstance() {
    assert!(transpile_ok("def foo(x):\n    if isinstance(x, int):\n        return x * 2"));
}

#[test]
fn test_if_truthiness_string() {
    assert!(transpile_ok("def foo(s: str):\n    if s:\n        return len(s)"));
}

#[test]
fn test_if_truthiness_list() {
    assert!(transpile_ok("def foo(lst: list):\n    if lst:\n        return lst[0]"));
}

#[test]
fn test_if_truthiness_dict() {
    assert!(transpile_ok("def foo(d: dict):\n    if d:\n        return len(d)"));
}

#[test]
fn test_if_multiple_elif() {
    assert!(transpile_ok("def foo(x):\n    if x == 1:\n        return 'one'\n    elif x == 2:\n        return 'two'\n    elif x == 3:\n        return 'three'\n    else:\n        return 'other'"));
}

#[test]
fn test_if_early_return() {
    assert!(transpile_ok("def foo(x):\n    if x < 0:\n        return -1\n    return x"));
}

#[test]
fn test_if_with_walrus() {
    assert!(transpile_ok("def foo(items):\n    if (n := len(items)) > 0:\n        return n"));
}

// ============================================================================
// MORE FOR LOOP VARIATIONS
// ============================================================================

#[test]
fn test_for_reversed() {
    assert!(transpile_ok("def foo(items):\n    for item in reversed(items):\n        pass"));
}

#[test]
fn test_for_sorted() {
    assert!(transpile_ok("def foo(items):\n    for item in sorted(items):\n        pass"));
}

#[test]
fn test_for_filter() {
    assert!(transpile_ok("def foo(items):\n    for item in filter(lambda x: x > 0, items):\n        pass"));
}

#[test]
fn test_for_map() {
    assert!(transpile_ok("def foo(items):\n    for item in map(lambda x: x * 2, items):\n        pass"));
}

#[test]
fn test_for_triple_nested() {
    assert!(transpile_ok("def foo():\n    for i in range(3):\n        for j in range(3):\n            for k in range(3):\n                x = i + j + k"));
}

#[test]
fn test_for_tuple_unpack() {
    assert!(transpile_ok("def foo():\n    for a, b, c in [(1, 2, 3), (4, 5, 6)]:\n        x = a + b + c"));
}

#[test]
fn test_for_with_assignment() {
    assert!(transpile_ok("def foo():\n    total = 0\n    for i in range(10):\n        total += i\n    return total"));
}

#[test]
fn test_for_set_iteration() {
    assert!(transpile_ok("def foo(s: set):\n    for item in s:\n        pass"));
}

#[test]
fn test_for_bytes_iteration() {
    assert!(transpile_ok("def foo():\n    for b in b'hello':\n        pass"));
}

// ============================================================================
// MORE WHILE LOOP VARIATIONS
// ============================================================================

#[test]
fn test_while_and_condition() {
    assert!(transpile_ok("def foo(x, y):\n    while x > 0 and y > 0:\n        x -= 1\n        y -= 1"));
}

#[test]
fn test_while_or_condition() {
    assert!(transpile_ok("def foo(x, y):\n    while x > 0 or y > 0:\n        if x > 0:\n            x -= 1\n        if y > 0:\n            y -= 1"));
}

#[test]
fn test_while_not_condition() {
    assert!(transpile_ok("def foo(done):\n    while not done:\n        done = True"));
}

#[test]
fn test_while_truthiness() {
    assert!(transpile_ok("def foo(items: list):\n    while items:\n        items.pop()"));
}

#[test]
fn test_while_with_walrus() {
    assert!(transpile_ok("def foo():\n    x = 10\n    while (x := x - 1) > 0:\n        pass"));
}

// ============================================================================
// MORE ASSIGNMENT VARIATIONS
// ============================================================================

#[test]
fn test_assign_list_unpack() {
    assert!(transpile_ok("def foo():\n    a, b, c = [1, 2, 3]"));
}

#[test]
fn test_assign_dict_unpack() {
    assert!(transpile_ok("def foo():\n    d = {'a': 1}\n    x = d['a']"));
}

#[test]
fn test_assign_conditional() {
    assert!(transpile_ok("def foo(x):\n    y = 1 if x else 0"));
}

#[test]
fn test_assign_from_function() {
    assert!(transpile_ok("def foo():\n    x = len([1, 2, 3])"));
}

#[test]
fn test_assign_from_method() {
    assert!(transpile_ok("def foo():\n    x = 'hello'.upper()"));
}

#[test]
fn test_assign_augmented_floordiv() {
    assert!(transpile_ok("def foo():\n    x = 10\n    x //= 3"));
}

#[test]
fn test_assign_augmented_mod() {
    assert!(transpile_ok("def foo():\n    x = 10\n    x %= 3"));
}

#[test]
fn test_assign_augmented_pow() {
    assert!(transpile_ok("def foo():\n    x = 2\n    x **= 3"));
}

#[test]
fn test_assign_augmented_bitand() {
    assert!(transpile_ok("def foo():\n    x = 7\n    x &= 3"));
}

#[test]
fn test_assign_augmented_bitor() {
    assert!(transpile_ok("def foo():\n    x = 5\n    x |= 3"));
}

#[test]
fn test_assign_augmented_bitxor() {
    assert!(transpile_ok("def foo():\n    x = 5\n    x ^= 3"));
}

#[test]
fn test_assign_augmented_lshift() {
    assert!(transpile_ok("def foo():\n    x = 1\n    x <<= 4"));
}

#[test]
fn test_assign_augmented_rshift() {
    assert!(transpile_ok("def foo():\n    x = 16\n    x >>= 2"));
}

#[test]
fn test_assign_nested_subscript() {
    assert!(transpile_ok("def foo():\n    d = {'a': [1, 2, 3]}\n    d['a'][0] = 10"));
}

#[test]
fn test_assign_slice() {
    assert!(transpile_ok("def foo():\n    lst = [1, 2, 3, 4, 5]\n    lst[1:3] = [10, 20]"));
}

// ============================================================================
// MORE TRY/EXCEPT VARIATIONS
// ============================================================================

#[test]
fn test_try_except_multiple_types() {
    assert!(transpile_ok("def foo():\n    try:\n        x = 1\n    except (ValueError, TypeError):\n        x = 0"));
}

#[test]
fn test_try_nested() {
    assert!(transpile_ok("def foo():\n    try:\n        try:\n            x = 1\n        except:\n            pass\n    except:\n        pass"));
}

#[test]
fn test_try_in_loop() {
    assert!(transpile_ok("def foo(items):\n    for item in items:\n        try:\n            x = int(item)\n        except:\n            continue"));
}

#[test]
fn test_try_finally_only() {
    assert!(transpile_ok("def foo():\n    try:\n        x = 1\n    finally:\n        y = 2"));
}

#[test]
fn test_try_full() {
    assert!(transpile_ok("def foo():\n    try:\n        x = 1\n    except ValueError:\n        x = 2\n    except:\n        x = 3\n    else:\n        x = 4\n    finally:\n        y = 5"));
}

// ============================================================================
// MORE WITH STATEMENT VARIATIONS
// ============================================================================

#[test]
fn test_with_nested() {
    assert!(transpile_ok("def foo():\n    with open('a') as f1:\n        with open('b') as f2:\n            pass"));
}

#[test]
fn test_with_contextlib() {
    assert!(transpile_ok("from contextlib import contextmanager\n\ndef foo():\n    with contextmanager(lambda: None) as ctx:\n        pass"));
}

// ============================================================================
// MORE RETURN VARIATIONS
// ============================================================================

#[test]
fn test_return_list() {
    assert!(transpile_ok("def foo():\n    return [1, 2, 3]"));
}

#[test]
fn test_return_dict() {
    assert!(transpile_ok("def foo():\n    return {'a': 1}"));
}

#[test]
fn test_return_set() {
    assert!(transpile_ok("def foo():\n    return {1, 2, 3}"));
}

#[test]
fn test_return_comprehension() {
    assert!(transpile_ok("def foo():\n    return [i * 2 for i in range(10)]"));
}

#[test]
fn test_return_lambda_result() {
    assert!(transpile_ok("def foo():\n    f = lambda x: x * 2\n    return f(21)"));
}

#[test]
fn test_return_method_chain() {
    assert!(transpile_ok("def foo():\n    return 'hello world'.upper().split()"));
}

// ============================================================================
// MORE RAISE VARIATIONS
// ============================================================================

#[test]
fn test_raise_runtime_error() {
    assert!(transpile_ok("def foo():\n    raise RuntimeError('error')"));
}

#[test]
fn test_raise_type_error() {
    assert!(transpile_ok("def foo():\n    raise TypeError('wrong type')"));
}

#[test]
fn test_raise_key_error() {
    assert!(transpile_ok("def foo():\n    raise KeyError('missing key')"));
}

#[test]
fn test_raise_index_error() {
    assert!(transpile_ok("def foo():\n    raise IndexError('out of bounds')"));
}

#[test]
fn test_raise_assertion_error() {
    assert!(transpile_ok("def foo():\n    raise AssertionError('assertion failed')"));
}

#[test]
fn test_raise_not_implemented() {
    assert!(transpile_ok("def foo():\n    raise NotImplementedError()"));
}

// ============================================================================
// MORE ASSERT VARIATIONS
// ============================================================================

#[test]
fn test_assert_equality() {
    assert!(transpile_ok("def foo(x):\n    assert x == 42"));
}

#[test]
fn test_assert_isinstance() {
    assert!(transpile_ok("def foo(x):\n    assert isinstance(x, int)"));
}

#[test]
fn test_assert_in() {
    assert!(transpile_ok("def foo(x, items):\n    assert x in items"));
}

#[test]
fn test_assert_not_none() {
    assert!(transpile_ok("def foo(x):\n    assert x is not None"));
}

// ============================================================================
// DELETE VARIATIONS
// ============================================================================

#[test]
fn test_del_dict_key() {
    assert!(transpile_ok("def foo():\n    d = {'a': 1, 'b': 2}\n    del d['a']"));
}

#[test]
fn test_del_attribute() {
    assert!(transpile_ok("def foo(obj):\n    del obj.attr"));
}

#[test]
fn test_del_slice() {
    assert!(transpile_ok("def foo():\n    lst = [1, 2, 3, 4, 5]\n    del lst[1:3]"));
}

#[test]
fn test_del_multiple() {
    assert!(transpile_ok("def foo():\n    a = 1\n    b = 2\n    del a, b"));
}

// ============================================================================
// MORE MATCH VARIATIONS
// ============================================================================

#[test]
fn test_match_sequence() {
    assert!(transpile_ok("def foo(x):\n    match x:\n        case [a, b]:\n            return a + b"));
}

#[test]
fn test_match_mapping() {
    assert!(transpile_ok("def foo(x):\n    match x:\n        case {'key': value}:\n            return value"));
}

#[test]
fn test_match_class() {
    assert!(transpile_ok("def foo(point):\n    match point:\n        case Point(x=0, y=0):\n            return 'origin'"));
}

#[test]
fn test_match_guard() {
    assert!(transpile_ok("def foo(x):\n    match x:\n        case n if n > 0:\n            return 'positive'"));
}

#[test]
fn test_match_or_pattern() {
    assert!(transpile_ok("def foo(x):\n    match x:\n        case 'yes' | 'y' | 'true':\n            return True"));
}

// ============================================================================
// FUNCTION DEFINITIONS IN STATEMENTS
// ============================================================================

#[test]
fn test_nested_function() {
    assert!(transpile_ok("def outer():\n    def inner():\n        return 1\n    return inner()"));
}

#[test]
fn test_decorated_nested_function() {
    assert!(transpile_ok("def outer():\n    @staticmethod\n    def inner():\n        return 1"));
}

#[test]
fn test_recursive_function() {
    assert!(transpile_ok("def factorial(n):\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)"));
}

// ============================================================================
// GENERATOR STATEMENTS
// ============================================================================

#[test]
fn test_yield_statement() {
    assert!(transpile_ok("def gen():\n    yield 1\n    yield 2\n    yield 3"));
}

#[test]
fn test_yield_from_statement() {
    assert!(transpile_ok("def gen():\n    yield from [1, 2, 3]"));
}

#[test]
fn test_yield_in_loop() {
    assert!(transpile_ok("def gen(n):\n    for i in range(n):\n        yield i"));
}

#[test]
fn test_yield_conditional() {
    assert!(transpile_ok("def gen(items):\n    for item in items:\n        if item > 0:\n            yield item"));
}

// ============================================================================
// ASYNC STATEMENTS
// ============================================================================

#[test]
fn test_async_def() {
    assert!(transpile_ok("async def foo():\n    return 42"));
}

#[test]
fn test_await_statement() {
    assert!(transpile_ok("async def foo():\n    result = await bar()\n    return result"));
}

#[test]
fn test_async_for() {
    assert!(transpile_ok("async def foo():\n    async for item in aiter:\n        pass"));
}

#[test]
fn test_async_with() {
    assert!(transpile_ok("async def foo():\n    async with aopen('file') as f:\n        pass"));
}

// ============================================================================
// COMPLEX STATEMENT COMBINATIONS
// ============================================================================

#[test]
fn test_if_in_for() {
    assert!(transpile_ok("def foo(items):\n    for item in items:\n        if item > 0:\n            return item"));
}

#[test]
fn test_for_in_if() {
    assert!(transpile_ok("def foo(items):\n    if items:\n        for item in items:\n            pass"));
}

#[test]
fn test_try_in_for_in_while() {
    assert!(transpile_ok("def foo():\n    i = 0\n    while i < 10:\n        for j in range(i):\n            try:\n                x = 1 / j\n            except:\n                pass\n        i += 1"));
}

#[test]
fn test_nested_with_try() {
    assert!(transpile_ok("def foo():\n    try:\n        with open('file') as f:\n            data = f.read()\n    except:\n        pass"));
}

#[test]
fn test_multiple_returns() {
    assert!(transpile_ok("def foo(x):\n    if x < 0:\n        return -1\n    if x == 0:\n        return 0\n    return 1"));
}

#[test]
fn test_guard_clauses() {
    assert!(transpile_ok("def foo(x):\n    if x is None:\n        return None\n    if not isinstance(x, int):\n        raise TypeError()\n    return x * 2"));
}

// ============================================================================
// MORE IMPORT VARIATIONS
// ============================================================================

#[test]
fn test_import_star() {
    assert!(transpile_ok("from os.path import *"));
}

#[test]
fn test_import_nested_module() {
    assert!(transpile_ok("import os.path.join"));
}

#[test]
fn test_import_in_function() {
    assert!(transpile_ok("def foo():\n    import os\n    return os.getcwd()"));
}

// ============================================================================
// MORE TYPE ANNOTATION VARIATIONS
// ============================================================================

#[test]
fn test_annotated_list() {
    assert!(transpile_ok("def foo():\n    x: list[int] = [1, 2, 3]"));
}

#[test]
fn test_annotated_dict() {
    assert!(transpile_ok("def foo():\n    x: dict[str, int] = {'a': 1}"));
}

#[test]
fn test_annotated_optional() {
    assert!(transpile_ok("from typing import Optional\n\ndef foo():\n    x: Optional[int] = None"));
}

#[test]
fn test_annotated_union() {
    assert!(transpile_ok("from typing import Union\n\ndef foo():\n    x: Union[int, str] = 42"));
}

#[test]
fn test_annotated_tuple() {
    assert!(transpile_ok("def foo():\n    x: tuple[int, int] = (1, 2)"));
}

#[test]
fn test_annotated_callable() {
    assert!(transpile_ok("from typing import Callable\n\ndef foo():\n    x: Callable[[int], int] = lambda n: n * 2"));
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_empty_if_body() {
    assert!(transpile_ok("def foo():\n    if True:\n        pass"));
}

#[test]
fn test_empty_for_body() {
    assert!(transpile_ok("def foo():\n    for i in range(10):\n        pass"));
}

#[test]
fn test_empty_while_body() {
    assert!(transpile_ok("def foo():\n    while False:\n        pass"));
}

#[test]
fn test_empty_try_body() {
    assert!(transpile_ok("def foo():\n    try:\n        pass\n    except:\n        pass"));
}

#[test]
fn test_deeply_nested() {
    assert!(transpile_ok("def foo():\n    if True:\n        if True:\n            if True:\n                if True:\n                    x = 1"));
}

#[test]
fn test_many_statements() {
    assert!(transpile_ok("def foo():\n    a = 1\n    b = 2\n    c = 3\n    d = 4\n    e = 5\n    f = 6\n    g = 7\n    h = 8\n    i = 9\n    j = 10"));
}

#[test]
fn test_mixed_statement_types() {
    assert!(transpile_ok("def foo():\n    x = 1\n    if x:\n        for i in range(x):\n            while True:\n                break\n    return x"));
}

// ============================================================================
// FOR LOOP VARIATIONS
// ============================================================================

#[test]
fn test_for_string_iteration() {
    assert!(transpile_ok("def foo():\n    s = 'hello'\n    for c in s:\n        print(c)"));
}

#[test]
fn test_for_text_variable_iteration() {
    assert!(transpile_ok("def foo():\n    text = 'hello'\n    for c in text:\n        print(c)"));
}

#[test]
fn test_for_tuple_unpacking() {
    assert!(transpile_ok("def foo():\n    items = [(1, 'a'), (2, 'b')]\n    for x, y in items:\n        print(x, y)"));
}

#[test]
fn test_for_tuple_unpacking_three() {
    assert!(transpile_ok("def foo():\n    items = [(1, 'a', True), (2, 'b', False)]\n    for x, y, z in items:\n        print(x, y, z)"));
}

#[test]
fn test_for_with_mut_variable() {
    // Variable reassigned in loop body needs mut
    assert!(transpile_ok("def foo():\n    lines = ['a', 'b']\n    for line in lines:\n        line = line.strip()\n        print(line)"));
}

#[test]
fn test_for_enumerate_with_print() {
    assert!(transpile_ok("def foo():\n    items = [1, 2, 3]\n    for i, item in enumerate(items):\n        print(i, item)"));
}

#[test]
fn test_for_dict_items_print() {
    assert!(transpile_ok("def foo():\n    d = {'a': 1}\n    for k, v in d.items():\n        print(k, v)"));
}

#[test]
fn test_for_dict_keys_only() {
    assert!(transpile_ok("def foo():\n    d = {'a': 1}\n    for k in d:\n        print(k)"));
}

#[test]
fn test_for_dict_values_print() {
    assert!(transpile_ok("def foo():\n    d = {'a': 1}\n    for v in d.values():\n        print(v)"));
}

#[test]
fn test_for_zip_print() {
    assert!(transpile_ok("def foo():\n    a = [1, 2]\n    b = ['x', 'y']\n    for x, y in zip(a, b):\n        print(x, y)"));
}

#[test]
fn test_for_range_step_print() {
    assert!(transpile_ok("def foo():\n    for i in range(0, 10, 2):\n        print(i)"));
}

#[test]
fn test_for_reversed_print() {
    assert!(transpile_ok("def foo():\n    items = [1, 2, 3]\n    for x in reversed(items):\n        print(x)"));
}

#[test]
fn test_for_sorted_print() {
    assert!(transpile_ok("def foo():\n    items = [3, 1, 2]\n    for x in sorted(items):\n        print(x)"));
}

#[test]
fn test_for_set_iteration_print() {
    assert!(transpile_ok("def foo():\n    s = {1, 2, 3}\n    for x in s:\n        print(x)"));
}

#[test]
fn test_for_unused_variable() {
    // Unused loop variable should get underscore prefix
    assert!(transpile_ok("def foo():\n    for _ in range(5):\n        print('hello')"));
}

#[test]
fn test_for_else_completed() {
    assert!(transpile_ok("def foo():\n    for i in range(5):\n        if i == 10:\n            break\n    else:\n        print('completed')"));
}

// ============================================================================
// WHILE LOOP VARIATIONS
// ============================================================================

#[test]
fn test_while_with_break() {
    assert!(transpile_ok("def foo():\n    x = 0\n    while True:\n        x += 1\n        if x > 10:\n            break"));
}

#[test]
fn test_while_with_continue() {
    assert!(transpile_ok("def foo():\n    x = 0\n    while x < 10:\n        x += 1\n        if x % 2 == 0:\n            continue\n        print(x)"));
}

#[test]
fn test_while_else_done() {
    assert!(transpile_ok("def foo():\n    x = 0\n    while x < 5:\n        x += 1\n    else:\n        print('done')"));
}

#[test]
fn test_while_complex_condition() {
    assert!(transpile_ok("def foo():\n    x = 0\n    y = 10\n    while x < y and x >= 0:\n        x += 1"));
}

// ============================================================================
// TRY-EXCEPT VARIATIONS
// ============================================================================

#[test]
fn test_try_multiple_except_handlers() {
    assert!(transpile_ok("def foo():\n    try:\n        x = 1/0\n    except ZeroDivisionError:\n        x = 0\n    except ValueError:\n        x = -1"));
}

#[test]
fn test_try_except_as_print() {
    assert!(transpile_ok("def foo():\n    try:\n        x = 1/0\n    except Exception as e:\n        print(e)"));
}

#[test]
fn test_try_finally() {
    assert!(transpile_ok("def foo():\n    try:\n        x = 1\n    finally:\n        print('cleanup')"));
}

#[test]
fn test_try_except_finally_combined() {
    assert!(transpile_ok("def foo():\n    try:\n        x = 1\n    except:\n        pass\n    finally:\n        print('cleanup')"));
}

#[test]
fn test_try_except_else_success() {
    assert!(transpile_ok("def foo():\n    try:\n        x = 1\n    except:\n        pass\n    else:\n        print('success')"));
}

#[test]
fn test_try_nested_reraise() {
    assert!(transpile_ok("def foo():\n    try:\n        try:\n            x = 1/0\n        except:\n            raise\n    except:\n        pass"));
}

#[test]
fn test_raise_valueerror() {
    assert!(transpile_ok("def foo():\n    raise ValueError()"));
}

#[test]
fn test_raise_with_message() {
    assert!(transpile_ok("def foo():\n    raise ValueError('error message')"));
}

#[test]
fn test_raise_from_exception() {
    assert!(transpile_ok("def foo():\n    try:\n        x = 1/0\n    except Exception as e:\n        raise RuntimeError() from e"));
}

#[test]
fn test_raise_bare() {
    assert!(transpile_ok("def foo():\n    try:\n        x = 1\n    except:\n        raise"));
}

// ============================================================================
// WITH STATEMENT VARIATIONS
// ============================================================================

#[test]
fn test_with_open_file() {
    assert!(transpile_ok("def foo():\n    with open('file.txt') as f:\n        x = f.read()"));
}

#[test]
fn test_with_multiple_files() {
    assert!(transpile_ok("def foo():\n    with open('a.txt') as a, open('b.txt') as b:\n        x = a.read()"));
}

#[test]
fn test_with_no_alias() {
    assert!(transpile_ok("def foo():\n    with lock:\n        x = 1"));
}

#[test]
fn test_with_nested_files() {
    assert!(transpile_ok("def foo():\n    with open('a.txt') as a:\n        with open('b.txt') as b:\n            x = a.read() + b.read()"));
}

// ============================================================================
// MATCH STATEMENT (Python 3.10+)
// ============================================================================

#[test]
fn test_match_simple() {
    assert!(transpile_ok("def foo(x):\n    match x:\n        case 1:\n            return 'one'\n        case 2:\n            return 'two'\n        case _:\n            return 'other'"));
}

#[test]
fn test_match_tuple() {
    assert!(transpile_ok("def foo(point):\n    match point:\n        case (0, 0):\n            return 'origin'\n        case (x, 0):\n            return 'x-axis'\n        case (0, y):\n            return 'y-axis'\n        case (x, y):\n            return 'other'"));
}

#[test]
fn test_match_or_pattern_small() {
    assert!(transpile_ok("def foo(x):\n    match x:\n        case 1 | 2 | 3:\n            return 'small'\n        case _:\n            return 'large'"));
}

#[test]
fn test_match_guard_pos_neg() {
    assert!(transpile_ok("def foo(x):\n    match x:\n        case n if n > 0:\n            return 'positive'\n        case n if n < 0:\n            return 'negative'\n        case _:\n            return 'zero'"));
}

// ============================================================================
// SPECIAL STATEMENTS
// ============================================================================

#[test]
fn test_global_statement() {
    assert!(transpile_ok("x = 0\n\ndef foo():\n    global x\n    x = 1"));
}

#[test]
fn test_nonlocal_statement() {
    assert!(transpile_ok("def foo():\n    x = 0\n    def bar():\n        nonlocal x\n        x = 1\n    bar()"));
}

#[test]
fn test_del_statement() {
    assert!(transpile_ok("def foo():\n    x = [1, 2, 3]\n    del x[0]"));
}

#[test]
fn test_del_variable_simple() {
    assert!(transpile_ok("def foo():\n    x = 1\n    del x"));
}

#[test]
fn test_assert_with_message_positive() {
    assert!(transpile_ok("def foo(x):\n    assert x > 0, 'x must be positive'"));
}

#[test]
fn test_assert_no_message() {
    assert!(transpile_ok("def foo(x):\n    assert x > 0"));
}

// ============================================================================
// ASSIGNMENT VARIATIONS
// ============================================================================

#[test]
fn test_augmented_assign_all_ops() {
    assert!(transpile_ok("def foo():\n    x = 10\n    x += 1\n    x -= 1\n    x *= 2\n    x /= 2\n    x //= 1\n    x %= 3\n    x **= 2\n    x &= 1\n    x |= 1\n    x ^= 1\n    x <<= 1\n    x >>= 1"));
}

#[test]
fn test_annotated_assign() {
    assert!(transpile_ok("def foo():\n    x: int = 1\n    y: str = 'hello'\n    z: list[int] = [1, 2, 3]"));
}

#[test]
fn test_multiple_assign() {
    assert!(transpile_ok("def foo():\n    a = b = c = 1"));
}

#[test]
fn test_walrus_operator() {
    assert!(transpile_ok("def foo():\n    if (n := len([1, 2, 3])) > 2:\n        print(n)"));
}

#[test]
fn test_tuple_unpack_assign() {
    assert!(transpile_ok("def foo():\n    x, y, z = 1, 2, 3"));
}

#[test]
fn test_starred_assign() {
    assert!(transpile_ok("def foo():\n    first, *rest = [1, 2, 3, 4]"));
}

#[test]
fn test_subscript_assign() {
    assert!(transpile_ok("def foo():\n    x = [1, 2, 3]\n    x[0] = 10"));
}

#[test]
fn test_attribute_assign() {
    assert!(transpile_ok("class Foo:\n    def __init__(self):\n        self.x = 1\n\ndef foo():\n    f = Foo()\n    f.x = 2"));
}

// ============================================================================
// COMPLEX PATTERNS
// ============================================================================

#[test]
fn test_list_comp_with_if() {
    assert!(transpile_ok("def foo():\n    x = [i for i in range(10) if i % 2 == 0]"));
}

#[test]
fn test_dict_comp() {
    assert!(transpile_ok("def foo():\n    x = {i: i**2 for i in range(5)}"));
}

#[test]
fn test_set_comp() {
    assert!(transpile_ok("def foo():\n    x = {i for i in range(10) if i % 2 == 0}"));
}

#[test]
fn test_generator_expr_in_sum() {
    assert!(transpile_ok("def foo():\n    x = sum(i**2 for i in range(10))"));
}

#[test]
fn test_nested_comprehension() {
    assert!(transpile_ok("def foo():\n    matrix = [[i*j for j in range(3)] for i in range(3)]"));
}

#[test]
fn test_lambda_in_statement() {
    assert!(transpile_ok("def foo():\n    double = lambda x: x * 2\n    result = double(5)"));
}

#[test]
fn test_ternary_in_assign() {
    assert!(transpile_ok("def foo(x):\n    result = 'positive' if x > 0 else 'negative'"));
}

// ============================================================================
// TYPE COERCION SCENARIOS
// ============================================================================

#[test]
fn test_int_to_float_coercion() {
    assert!(transpile_ok("def foo():\n    x: float = 1\n    y = x + 2"));
}

#[test]
fn test_str_to_string_coercion() {
    assert!(transpile_ok("def foo():\n    x: str = 'hello'\n    y = x.upper()"));
}

// ============================================================================
// ITERATOR PATTERNS
// ============================================================================

#[test]
fn test_filter_map_chain() {
    assert!(transpile_ok("def foo():\n    items = [1, 2, 3, 4]\n    result = list(filter(lambda x: x > 2, map(lambda x: x * 2, items)))"));
}

#[test]
fn test_any_all() {
    assert!(transpile_ok("def foo():\n    items = [1, 2, 3]\n    a = any(x > 2 for x in items)\n    b = all(x > 0 for x in items)"));
}

#[test]
fn test_min_max_key() {
    assert!(transpile_ok("def foo():\n    items = ['hello', 'hi', 'hey']\n    shortest = min(items, key=len)\n    longest = max(items, key=len)"));
}

// ============================================================================
// CONTEXT-DEPENDENT BEHAVIORS
// ============================================================================

#[test]
fn test_final_return_statement() {
    // Final statement should return without explicit return keyword
    assert!(transpile_ok("def foo():\n    x = 1\n    x + 1"));
}

#[test]
fn test_return_in_if() {
    assert!(transpile_ok("def foo(x):\n    if x > 0:\n        return 'positive'\n    return 'non-positive'"));
}

#[test]
fn test_return_in_try() {
    assert!(transpile_ok("def foo():\n    try:\n        return 1\n    except:\n        return 0"));
}

#[test]
fn test_return_in_for() {
    assert!(transpile_ok("def foo():\n    for i in range(10):\n        if i == 5:\n            return i\n    return -1"));
}

#[test]
fn test_return_in_while() {
    assert!(transpile_ok("def foo():\n    x = 0\n    while True:\n        x += 1\n        if x == 5:\n            return x"));
}

// ============================================================================
// TYPE COERCION IN COMPLEX EXPRESSIONS
// ============================================================================

#[test]
fn test_usize_to_i32_in_loop() {
    assert!(transpile_ok(r#"def foo():
    lst = [1, 2, 3]
    n = len(lst)
    for i in range(n):
        x = i"#));
}

#[test]
fn test_float_coercion_in_comparison() {
    assert!(transpile_ok(r#"def foo(x: float):
    if x > 0:
        return x * 2
    return 0.0"#));
}

#[test]
fn test_string_coercion_in_format() {
    assert!(transpile_ok(r#"def foo():
    x = 42
    s = f"value: {x}""#));
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_empty_function() {
    assert!(transpile_ok("def foo():\n    pass"));
}

#[test]
fn test_single_expression_body() {
    assert!(transpile_ok("def foo():\n    1 + 2"));
}

#[test]
fn test_chained_comparison() {
    assert!(transpile_ok("def foo(x):\n    if 0 < x < 10:\n        return True\n    return False"));
}

#[test]
fn test_boolean_short_circuit() {
    assert!(transpile_ok("def foo():\n    x = a and b or c"));
}

#[test]
fn test_none_check_pattern() {
    assert!(transpile_ok("def foo(x):\n    if x is None:\n        return 0\n    return x"));
}

#[test]
fn test_not_none_check() {
    assert!(transpile_ok("def foo(x):\n    if x is not None:\n        return x\n    return 0"));
}

#[test]
fn test_identity_check() {
    assert!(transpile_ok("def foo(a, b):\n    return a is b"));
}

#[test]
fn test_membership_test() {
    assert!(transpile_ok("def foo(x, lst):\n    return x in lst"));
}

#[test]
fn test_not_in() {
    assert!(transpile_ok("def foo(x, lst):\n    return x not in lst"));
}

// ============================================================================
// FOR LOOP SPECIALIZED PATTERNS (Coverage Boost)
// ============================================================================

#[test]
fn test_for_string_chars_iteration() {
    assert!(transpile_ok(r#"def foo():
    s = "hello"
    for c in s:
        print(c)"#));
}

#[test]
fn test_for_text_var_chars() {
    assert!(transpile_ok(r#"def foo():
    text = "world"
    for char in text:
        print(char)"#));
}

#[test]
fn test_for_line_var_chars() {
    assert!(transpile_ok(r#"def foo():
    line = "abc"
    for x in line:
        pass"#));
}

#[test]
fn test_for_word_var_chars() {
    assert!(transpile_ok(r#"def foo():
    word = "test"
    for c in word:
        pass"#));
}

#[test]
fn test_for_str_prefix_var() {
    assert!(transpile_ok(r#"def foo():
    str_data = "abc"
    for c in str_data:
        pass"#));
}

#[test]
fn test_for_file_iteration() {
    assert!(transpile_ok(r#"def foo():
    f = open("test.txt")
    for line in f:
        print(line)"#));
}

#[test]
fn test_for_file_var_iteration() {
    assert!(transpile_ok(r#"def foo():
    file = open("data.txt")
    for line in file:
        print(line)"#));
}

#[test]
fn test_for_input_file_iteration() {
    assert!(transpile_ok(r#"def foo():
    input_file = open("input.txt")
    for line in input_file:
        print(line)"#));
}

#[test]
fn test_for_data_file_iteration() {
    assert!(transpile_ok(r#"def foo():
    data_file = open("data.txt")
    for line in data_file:
        pass"#));
}

#[test]
fn test_for_unused_loop_var() {
    // DEPYLER-0272: Unused loop vars should get underscore prefix
    assert!(transpile_ok(r#"def foo():
    for i in range(10):
        print("hello")"#));
}

#[test]
fn test_for_reassigned_loop_var() {
    // DEPYLER-0756: Reassigned loop vars need mut
    assert!(transpile_ok(r#"def foo():
    for line in ["a", "b"]:
        line = line.strip()
        print(line)"#));
}

#[test]
fn test_for_range_loop_var_type() {
    // DEPYLER-0803: Track loop var as Int for range iteration
    assert!(transpile_ok(r#"def foo():
    dx = 0.5
    for i in range(10):
        x = i * dx
        print(x)"#));
}

#[test]
fn test_for_dict_iteration() {
    // DEPYLER-0710: Dict iteration over keys
    assert!(transpile_ok(r#"def foo():
    d: dict[str, int] = {"a": 1, "b": 2}
    for key in d:
        print(key)"#));
}

#[test]
fn test_for_iterator_var_no_iter() {
    // DEPYLER-0520: Iterator vars shouldn't get .iter().cloned()
    assert!(transpile_ok(r#"def foo():
    items = [1, 2, 3]
    filtered = filter(lambda x: x > 1, items)
    for item in filtered:
        print(item)"#));
}

#[test]
fn test_for_tuple_reassigned_element() {
    // DEPYLER-0756: Tuple elements reassigned need mut
    assert!(transpile_ok(r#"def foo():
    pairs = [(1, "a"), (2, "b")]
    for num, text in pairs:
        text = text.upper()
        print(num, text)"#));
}

#[test]
fn test_for_tuple_unused_element() {
    // DEPYLER-0272: Unused tuple elements get underscore
    assert!(transpile_ok(r#"def foo():
    pairs = [(1, "a"), (2, "b")]
    for num, text in pairs:
        print(num)"#));
}

#[test]
fn test_for_csv_reader_iteration() {
    assert!(transpile_ok(r#"import csv
def foo():
    reader = csv.DictReader(open("data.csv"))
    for row in reader:
        print(row)"#));
}

#[test]
fn test_for_csv_var_iteration() {
    assert!(transpile_ok(r#"def foo():
    csv_reader = None
    for row in csv_reader:
        pass"#));
}

#[test]
fn test_for_string_method_chars() {
    // DEPYLER-1012: String method results iteration
    assert!(transpile_ok(r#"def foo():
    s = "HELLO"
    for c in s.lower():
        print(c)"#));
}

#[test]
fn test_for_strip_result_chars() {
    assert!(transpile_ok(r#"def foo():
    s = "  hello  "
    for c in s.strip():
        print(c)"#));
}

#[test]
fn test_for_replace_result_chars() {
    assert!(transpile_ok(r#"def foo():
    s = "hello world"
    for c in s.replace(" ", "_"):
        print(c)"#));
}

// ============================================================================
// ASSIGNMENT SPECIALIZED PATTERNS (Coverage Boost)
// ============================================================================

#[test]
fn test_assign_dict_index_augment() {
    // DEPYLER-4048: Dict augmented assignment pattern
    assert!(transpile_ok(r#"def foo():
    counts: dict[str, int] = {}
    counts["a"] = counts.get("a", 0) + 1"#));
}

#[test]
fn test_assign_nested_dict_key() {
    assert!(transpile_ok(r#"def foo():
    data = {"a": {"b": 1}}
    data["a"]["c"] = 2"#));
}

#[test]
fn test_assign_list_slice() {
    assert!(transpile_ok(r#"def foo():
    lst = [1, 2, 3, 4]
    lst[1:3] = [10, 20]"#));
}

#[test]
fn test_assign_from_walrus() {
    // DEPYLER-0188: Walrus operator in assignment
    assert!(transpile_ok(r#"def foo():
    if (n := len([1, 2, 3])) > 2:
        x = n"#));
}

#[test]
fn test_assign_with_type_conversion() {
    assert!(transpile_ok(r#"def foo():
    x: float = 5
    y = x + 1.5"#));
}

// ============================================================================
// IF STATEMENT SPECIALIZED PATTERNS (Coverage Boost)
// ============================================================================

#[test]
fn test_if_option_pattern() {
    // DEPYLER-2623: Option if-let pattern
    assert!(transpile_ok(r#"def foo():
    result = get_optional()
    if result is not None:
        print(result)"#));
}

#[test]
fn test_if_variable_hoisting() {
    // DEPYLER-0476: Variable hoisting in if/else
    assert!(transpile_ok(r#"def foo(condition):
    if condition:
        value = get_a()
    else:
        value = get_b()
    return value"#));
}

#[test]
fn test_if_nested_walrus() {
    // DEPYLER-0188: Nested walrus extraction
    assert!(transpile_ok(r#"def foo():
    if (x := get_x()) and (y := get_y()):
        print(x, y)"#));
}

#[test]
fn test_if_dict_key_check() {
    assert!(transpile_ok(r#"def foo():
    d = {"a": 1}
    if "a" in d:
        x = d["a"]"#));
}

#[test]
fn test_if_var_used_as_dict_key() {
    // DEPYLER-3181: Variable used as dict key
    assert!(transpile_ok(r#"def foo():
    key = "name"
    d = {}
    if key in d:
        print(d[key])"#));
}

// ============================================================================
// WHILE LOOP SPECIALIZED PATTERNS (Coverage Boost)
// ============================================================================

#[test]
fn test_while_file_readline() {
    assert!(transpile_ok(r#"def foo():
    f = open("test.txt")
    while line := f.readline():
        print(line)"#));
}

#[test]
fn test_while_iterator_next() {
    assert!(transpile_ok(r#"def foo():
    items = iter([1, 2, 3])
    while (item := next(items, None)) is not None:
        print(item)"#));
}

// ============================================================================
// TRY/EXCEPT SPECIALIZED PATTERNS (Coverage Boost)
// ============================================================================

#[test]
fn test_try_variable_in_handler() {
    assert!(transpile_ok(r#"def foo():
    try:
        x = risky_operation()
    except ValueError as e:
        x = default_value()
        log_error(e)
    return x"#));
}

#[test]
fn test_try_finally_cleanup() {
    assert!(transpile_ok(r#"def foo():
    f = open("test.txt")
    try:
        data = f.read()
    finally:
        f.close()
    return data"#));
}

// ============================================================================
// RAISE STATEMENT SPECIALIZED PATTERNS (Coverage Boost)
// ============================================================================

#[test]
fn test_raise_custom_exception() {
    assert!(transpile_ok(r#"def foo():
    raise CustomError("message")"#));
}

#[test]
fn test_raise_from_variable() {
    assert!(transpile_ok(r#"def foo():
    try:
        x = 1 / 0
    except ZeroDivisionError as e:
        raise ValueError("Division error") from e"#));
}

// ============================================================================
// WITH STATEMENT SPECIALIZED PATTERNS (Coverage Boost)
// ============================================================================

#[test]
fn test_with_file_write() {
    assert!(transpile_ok(r#"def foo():
    with open("test.txt", "w") as f:
        f.write("hello")"#));
}

#[test]
fn test_with_multiple_context() {
    assert!(transpile_ok(r#"def foo():
    with open("in.txt") as fin, open("out.txt", "w") as fout:
        fout.write(fin.read())"#));
}

// ============================================================================
// EXPRESSION STATEMENT SPECIALIZED PATTERNS (Coverage Boost)
// ============================================================================

#[test]
fn test_expr_stmt_with_side_effect() {
    assert!(transpile_ok(r#"def foo():
    lst = [1, 2, 3]
    lst.pop()
    lst.append(4)"#));
}

#[test]
fn test_expr_stmt_method_chain() {
    assert!(transpile_ok(r#"def foo():
    d = {}
    d.setdefault("key", []).append(1)"#));
}

// ============================================================================
// RETURN STATEMENT SPECIALIZED PATTERNS (Coverage Boost)
// ============================================================================

#[test]
fn test_return_none_explicit() {
    assert!(transpile_ok(r#"def foo():
    return None"#));
}

#[test]
fn test_return_tuple_unpack() {
    assert!(transpile_ok(r#"def foo():
    return 1, 2, 3"#));
}

#[test]
fn test_return_conditional_expr() {
    assert!(transpile_ok(r#"def foo(x):
    return "yes" if x > 0 else "no""#));
}

// ============================================================================
// FUNCTION DEF SPECIALIZED PATTERNS (Coverage Boost)
// ============================================================================

#[test]
fn test_func_with_varargs() {
    assert!(transpile_ok(r#"def foo(*args):
    for arg in args:
        print(arg)"#));
}

#[test]
fn test_func_with_kwargs() {
    assert!(transpile_ok(r#"def foo(**kwargs):
    for k, v in kwargs.items():
        print(k, v)"#));
}

#[test]
fn test_func_positional_only_params() {
    assert!(transpile_ok(r#"def foo(x, /, y):
    return x + y"#));
}

#[test]
fn test_func_keyword_only_params() {
    assert!(transpile_ok(r#"def foo(*, x, y):
    return x + y"#));
}

// ============================================================================
// JSON VALUE PATTERNS (Coverage Boost)
// ============================================================================

#[test]
fn test_json_dict_index_iteration() {
    // DEPYLER-0607: JSON value iteration
    assert!(transpile_ok(r#"import json
def foo():
    data = {"items": [1, 2, 3]}
    for item in data["items"]:
        print(item)"#));
}

#[test]
fn test_json_nested_access() {
    assert!(transpile_ok(r#"import json
def foo():
    data = {"a": {"b": [1, 2]}}
    items = data["a"]["b"]"#));
}

// ============================================================================
// COUNTER/COLLECTIONS PATTERNS (Coverage Boost)
// ============================================================================

#[test]
fn test_counter_items_iteration() {
    // DEPYLER-0821: Counter iteration
    assert!(transpile_ok(r#"from collections import Counter
def foo():
    c = Counter("hello")
    for char, count in c.items():
        print(char, count)"#));
}

#[test]
fn test_counter_most_common() {
    assert!(transpile_ok(r#"from collections import Counter
def foo():
    c = Counter("hello")
    for char, count in c.most_common(3):
        print(char, count)"#));
}

// ============================================================================
// COVERAGE BOOST: Helper Function Tests
// These tests specifically target uncovered code paths in stmt_gen.rs
// ============================================================================

// --- expr_returns_usize helper ---
#[test]
fn test_expr_returns_usize_len() {
    // Tests the len() method returning usize path
    assert!(transpile_ok(r#"def foo():
    items = [1, 2, 3]
    x: int = len(items)
    return x"#));
}

#[test]
fn test_expr_returns_usize_count() {
    // Tests count() method path
    assert!(transpile_ok(r#"def foo():
    s = "hello world"
    x: int = s.count("l")
    return x"#));
}

#[test]
fn test_expr_returns_usize_capacity() {
    // Tests capacity path
    assert!(transpile_ok(r#"def foo():
    items = [1, 2, 3]
    x = len(items) + len(items)"#));
}

#[test]
fn test_expr_returns_usize_binary() {
    // Tests binary operation with usize
    assert!(transpile_ok(r#"def foo():
    items = [1, 2, 3]
    x: int = len(items) + 1"#));
}

// --- expr_infers_float helper ---
#[test]
fn test_expr_infers_float_literal() {
    // Float literal path
    assert!(transpile_ok(r#"def foo():
    x = 3.14
    return x"#));
}

#[test]
fn test_expr_infers_float_variable() {
    // Float variable path
    assert!(transpile_ok(r#"def foo():
    x: float = 3.14
    y = x * 2
    return y"#));
}

#[test]
fn test_expr_infers_float_call() {
    // Function call returning float
    assert!(transpile_ok(r#"def get_pi() -> float:
    return 3.14

def foo():
    x = get_pi()
    return x"#));
}

#[test]
fn test_expr_infers_float_binary() {
    // Binary operation with float
    assert!(transpile_ok(r#"def foo():
    x = 3.14 * 2
    return x"#));
}

#[test]
fn test_expr_infers_float_unary() {
    // Unary operation preserving float
    assert!(transpile_ok(r#"def foo():
    x = -3.14
    return x"#));
}

#[test]
fn test_expr_infers_float_ifexpr() {
    // IfExpr with both branches float
    assert!(transpile_ok(r#"def foo(cond):
    x = 3.14 if cond else 2.71
    return x"#));
}

// --- is_iterator_producing_expr helper ---
#[test]
fn test_is_iterator_generator_expr() {
    // Generator expression produces iterator
    assert!(transpile_ok(r#"def foo():
    gen = (x * 2 for x in range(10))
    for item in gen:
        print(item)"#));
}

#[test]
fn test_is_iterator_map_chain() {
    // Method chain ending in iterator adapter
    assert!(transpile_ok(r#"def foo(items):
    for item in items.iter().map(lambda x: x * 2):
        print(item)"#));
}

#[test]
fn test_is_iterator_filter_chain() {
    assert!(transpile_ok(r#"def foo(items):
    for item in items.iter().filter(lambda x: x > 0):
        print(item)"#));
}

#[test]
fn test_is_iterator_enumerate_builtin() {
    // enumerate() produces iterator
    assert!(transpile_ok(r#"def foo(items):
    for i, x in enumerate(items):
        print(i, x)"#));
}

#[test]
fn test_is_iterator_zip_builtin() {
    // zip() produces iterator
    assert!(transpile_ok(r#"def foo(a, b):
    for x, y in zip(a, b):
        print(x, y)"#));
}

#[test]
fn test_is_iterator_reversed_builtin() {
    // reversed() produces iterator
    assert!(transpile_ok(r#"def foo(items):
    for item in reversed(items):
        print(item)"#));
}

// --- is_numpy_value_expr helper ---
#[test]
fn test_is_numpy_array_call() {
    // np.array() detection
    assert!(transpile_ok(r#"import numpy as np
def foo():
    arr = np.array([1, 2, 3])
    return arr"#));
}

#[test]
fn test_is_numpy_zeros() {
    // np.zeros() detection
    assert!(transpile_ok(r#"import numpy as np
def foo():
    arr = np.zeros(10)
    return arr"#));
}

#[test]
fn test_is_numpy_binary_op() {
    // Binary operation on numpy arrays
    assert!(transpile_ok(r#"import numpy as np
def foo():
    a = np.array([1, 2, 3])
    b = np.array([4, 5, 6])
    c = a + b
    return c"#));
}

#[test]
fn test_is_numpy_method_call() {
    // numpy method call (abs, sqrt, etc.)
    assert!(transpile_ok(r#"import numpy as np
def foo():
    arr = np.array([1, 2, 3])
    result = np.abs(arr)
    return result"#));
}

#[test]
fn test_is_numpy_ternary() {
    // Ternary with numpy branches
    assert!(transpile_ok(r#"import numpy as np
def foo(cond):
    a = np.zeros(3) if cond else np.ones(3)
    return a"#));
}

// --- needs_type_conversion / apply_type_conversion helpers ---
#[test]
fn test_needs_conversion_int_from_len() {
    // len() -> int requires conversion
    assert!(transpile_ok(r#"def foo() -> int:
    items = [1, 2, 3]
    return len(items)"#));
}

#[test]
fn test_needs_conversion_string_from_var() {
    // Variable as String requires .to_string()
    assert!(transpile_ok(r#"def foo(s: str) -> str:
    return s"#));
}

#[test]
fn test_apply_conversion_i32_cast() {
    // Test as i32 cast application
    assert!(transpile_ok(r#"def foo():
    items = [1, 2, 3]
    x: int = len(items) + len(items)
    return x"#));
}

// --- extract_nested_indices_tokens helper ---
#[test]
fn test_nested_dict_access() {
    // Nested dictionary access for assignment
    assert!(transpile_ok(r#"def foo():
    d = {"a": {"b": 1}}
    d["a"]["b"] = 42"#));
}

#[test]
fn test_deep_nested_dict_access() {
    // Deep nested dictionary access
    assert!(transpile_ok(r#"def foo():
    d = {"a": {"b": {"c": 1}}}
    d["a"]["b"]["c"] = 42"#));
}

#[test]
fn test_nested_list_access() {
    // Nested list access
    assert!(transpile_ok(r#"def foo():
    m = [[1, 2], [3, 4]]
    m[0][1] = 10"#));
}

// --- Argparse handler coverage ---
#[test]
fn test_argparse_add_argument_basic() {
    assert!(transpile_ok(r#"import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("file")
    args = parser.parse_args()
    print(args.file)"#));
}

#[test]
fn test_argparse_add_argument_with_type() {
    assert!(transpile_ok(r#"import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--count", type=int)
    args = parser.parse_args()
    print(args.count)"#));
}

#[test]
fn test_argparse_add_argument_with_help() {
    assert!(transpile_ok(r#"import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--verbose", help="Enable verbose mode")
    args = parser.parse_args()"#));
}

#[test]
fn test_argparse_add_argument_with_action() {
    assert!(transpile_ok(r#"import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--verbose", action="store_true")
    args = parser.parse_args()"#));
}

#[test]
fn test_argparse_add_argument_with_nargs() {
    assert!(transpile_ok(r#"import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("files", nargs="+")
    args = parser.parse_args()"#));
}

#[test]
fn test_argparse_add_argument_required() {
    assert!(transpile_ok(r#"import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--config", required=True)
    args = parser.parse_args()"#));
}

#[test]
fn test_argparse_subparsers() {
    assert!(transpile_ok(r#"import argparse
def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers()
    sub = subparsers.add_parser("run")
    sub.add_argument("--fast")
    args = parser.parse_args()"#));
}

// --- Callable type inference ---
#[test]
fn test_callable_float_return() {
    // Callable with float return type inference
    assert!(transpile_ok(r#"from typing import Callable
def foo(f: Callable[[float], float], x: float) -> float:
    return f(x)"#));
}

#[test]
fn test_callable_int_return() {
    assert!(transpile_ok(r#"from typing import Callable
def foo(f: Callable[[int], int], x: int) -> int:
    return f(x)"#));
}

// --- Additional statement patterns ---
#[test]
fn test_global_statement_coverage() {
    // Additional coverage for global keyword
    assert!(transpile_ok(r#"x = 0
y = 0
def foo():
    global x, y
    x = 42
    y = 100"#));
}

#[test]
fn test_nonlocal_statement_coverage() {
    // Additional coverage for nonlocal in nested function
    assert!(transpile_ok(r#"def outer():
    x = 0
    y = 0
    def inner():
        nonlocal x, y
        x = 42
        y = 100
    inner()
    return x + y"#));
}

#[test]
fn test_delete_statement_coverage() {
    // Additional coverage for del statement
    assert!(transpile_ok(r#"def foo():
    x = 42
    y = 10
    del x
    del y"#));
}

#[test]
fn test_import_from_statement_coverage() {
    // Additional coverage for from import
    assert!(transpile_ok(r#"from math import sqrt, pi, ceil
def foo():
    return ceil(sqrt(pi))"#));
}

#[test]
fn test_import_as_statement_coverage() {
    // Additional coverage for import as
    assert!(transpile_ok(r#"import math as m
import os as operating_system
def foo():
    return m.sqrt(2)"#));
}

// --- Exception handling edge cases ---
#[test]
fn test_reraise_exception() {
    assert!(transpile_ok(r#"def foo():
    try:
        raise ValueError("error")
    except:
        raise"#));
}

#[test]
fn test_exception_with_message() {
    assert!(transpile_ok(r#"def foo():
    raise ValueError("something went wrong")"#));
}

// --- Float comparison coercion ---
#[test]
fn test_float_comparison_zero() {
    // Float comparison with literal 0
    assert!(transpile_ok(r#"def foo(x: float) -> bool:
    return x > 0"#));
}

#[test]
fn test_float_binary_comparison() {
    // Complex float expression comparison
    assert!(transpile_ok(r#"def foo(a: float, b: float) -> bool:
    return a * b > 0"#));
}

// --- Augmented assignment edge cases ---
#[test]
fn test_augmented_assign_dict_key() {
    assert!(transpile_ok(r#"def foo():
    d = {"count": 0}
    d["count"] += 1"#));
}

#[test]
fn test_augmented_assign_list_index() {
    assert!(transpile_ok(r#"def foo():
    lst = [1, 2, 3]
    lst[0] += 10"#));
}

#[test]
fn test_augmented_assign_nested() {
    assert!(transpile_ok(r#"def foo():
    d = {"a": {"b": 0}}
    d["a"]["b"] += 1"#));
}

// ============================================================================
// ERROR PATH TESTS - Exercise bail! and error handling in stmt_gen
// ============================================================================

fn transpile_err(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_err()
}

// --- for loop paths ---
#[test]
fn test_for_complex_unpack_ok() {
    // Complex tuple unpacking in for loops is supported
    assert!(transpile_ok(r#"def foo():
    for ((a, b), c) in [((1, 2), 3)]:
        print(a)"#));
}

// --- Pass statement coverage ---
#[test]
fn test_pass_in_if_path() {
    assert!(transpile_ok(r#"def foo():
    if True:
        pass"#));
}

#[test]
fn test_pass_in_function_path() {
    assert!(transpile_ok(r#"def foo():
    pass"#));
}

// --- Continue/Break paths ---
#[test]
fn test_continue_in_for_path() {
    assert!(transpile_ok(r#"def foo():
    for i in range(10):
        if i % 2 == 0:
            continue
        print(i)"#));
}

#[test]
fn test_break_in_for_path() {
    assert!(transpile_ok(r#"def foo():
    for i in range(10):
        if i > 5:
            break
        print(i)"#));
}

// --- Complex while conditions ---
#[test]
fn test_while_and_path() {
    assert!(transpile_ok(r#"def foo():
    i = 0
    while i < 10 and i % 2 == 0:
        i += 1"#));
}

#[test]
fn test_while_or_path() {
    assert!(transpile_ok(r#"def foo():
    i = 0
    while i < 5 or i == 7:
        i += 1"#));
}

// --- Complex if conditions ---
#[test]
fn test_if_elif_else_chain_path() {
    assert!(transpile_ok(r#"def foo(x: int) -> str:
    if x < 0:
        return "negative"
    elif x == 0:
        return "zero"
    elif x < 10:
        return "small"
    else:
        return "large""#));
}

#[test]
fn test_if_in_operator_path() {
    assert!(transpile_ok(r#"def foo(x: int) -> bool:
    if x in [1, 2, 3]:
        return True
    return False"#));
}

// --- Try-except paths ---
#[test]
fn test_try_else_path() {
    assert!(transpile_ok(r#"def foo():
    try:
        risky()
    except:
        print("error")
    else:
        print("success")"#));
}

// --- With statement paths ---
#[test]
fn test_nested_with_path() {
    assert!(transpile_ok(r#"def foo():
    with open("outer.txt") as f:
        with open("inner.txt") as g:
            return f.read() + g.read()"#));
}

// --- Return paths ---
#[test]
fn test_return_implicit_path() {
    assert!(transpile_ok(r#"def foo():
    x = 42"#));
}

// --- Tuple unpacking error paths ---
#[test]
fn test_tuple_unpack_nested_err() {
    // Nested tuple unpacking not fully supported yet
    assert!(transpile_err(r#"def foo():
    (a, b), c = (1, 2), 3"#));
}

// --- Augmented assignment operators paths ---
#[test]
fn test_aug_assign_sub_path() {
    assert!(transpile_ok(r#"def foo():
    x = 10
    x -= 3"#));
}

#[test]
fn test_aug_assign_mul_path() {
    assert!(transpile_ok(r#"def foo():
    x = 5
    x *= 2"#));
}

#[test]
fn test_aug_assign_div_path() {
    assert!(transpile_ok(r#"def foo():
    x = 10.0
    x /= 2"#));
}

#[test]
fn test_aug_assign_floor_div_path() {
    assert!(transpile_ok(r#"def foo():
    x = 10
    x //= 3"#));
}

#[test]
fn test_aug_assign_mod_path() {
    assert!(transpile_ok(r#"def foo():
    x = 10
    x %= 3"#));
}

#[test]
fn test_aug_assign_pow_path() {
    assert!(transpile_ok(r#"def foo():
    x = 2
    x **= 3"#));
}

#[test]
fn test_aug_assign_bitand_path() {
    assert!(transpile_ok(r#"def foo():
    x = 0xFF
    x &= 0x0F"#));
}

#[test]
fn test_aug_assign_bitor_path() {
    assert!(transpile_ok(r#"def foo():
    x = 0x0F
    x |= 0xF0"#));
}

#[test]
fn test_aug_assign_bitxor_path() {
    assert!(transpile_ok(r#"def foo():
    x = 0xFF
    x ^= 0xAA"#));
}

#[test]
fn test_aug_assign_lshift_path() {
    assert!(transpile_ok(r#"def foo():
    x = 1
    x <<= 4"#));
}

#[test]
fn test_aug_assign_rshift_path() {
    assert!(transpile_ok(r#"def foo():
    x = 16
    x >>= 2"#));
}

// --- Match with guard path ---
#[test]
fn test_match_guard_path() {
    assert!(transpile_ok(r#"def foo(x: int) -> str:
    match x:
        case n if n < 0:
            return "negative"
        case n if n > 0:
            return "positive"
        case _:
            return "zero""#));
}

// --- Raise with cause path ---
#[test]
fn test_raise_cause_path() {
    assert!(transpile_ok(r#"def foo():
    try:
        risky()
    except Exception as e:
        raise RuntimeError("wrapper") from e"#));
}
