//! Edge case statement tests for increasing code coverage
//! Targets less common statement patterns in stmt_gen.rs

use depyler_core::DepylerPipeline;

fn transpiles(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// =============================================================================
// Assignment Statement Tests
// =============================================================================

#[test]
fn test_assign_simple() {
    assert!(transpiles("x = 42"));
}

#[test]
fn test_assign_with_type() {
    assert!(transpiles("x: int = 42"));
}

#[test]
fn test_assign_tuple_unpack() {
    assert!(transpiles("a, b = 1, 2"));
}

#[test]
fn test_assign_tuple_unpack_nested() {
    assert!(transpiles("(a, b), c = (1, 2), 3"));
}

#[test]
fn test_assign_chained() {
    assert!(transpiles("a = b = c = 0"));
}

#[test]
fn test_assign_subscript() {
    assert!(transpiles("def f(lst):\n    lst[0] = 42"));
}

#[test]
fn test_assign_attribute() {
    assert!(transpiles("def f(obj):\n    obj.value = 42"));
}

#[test]
fn test_assign_augmented_add() {
    assert!(transpiles("def f():\n    x = 0\n    x += 1"));
}

#[test]
fn test_assign_augmented_sub() {
    assert!(transpiles("def f():\n    x = 10\n    x -= 1"));
}

#[test]
fn test_assign_augmented_mul() {
    assert!(transpiles("def f():\n    x = 2\n    x *= 3"));
}

#[test]
fn test_assign_augmented_div() {
    assert!(transpiles("def f():\n    x = 10.0\n    x /= 2"));
}

#[test]
fn test_assign_augmented_floordiv() {
    assert!(transpiles("def f():\n    x = 10\n    x //= 3"));
}

#[test]
fn test_assign_augmented_mod() {
    assert!(transpiles("def f():\n    x = 10\n    x %= 3"));
}

#[test]
fn test_assign_augmented_pow() {
    assert!(transpiles("def f():\n    x = 2\n    x **= 3"));
}

#[test]
fn test_assign_augmented_bitand() {
    assert!(transpiles("def f():\n    x = 0xFF\n    x &= 0x0F"));
}

#[test]
fn test_assign_augmented_bitor() {
    assert!(transpiles("def f():\n    x = 0x0F\n    x |= 0xF0"));
}

#[test]
fn test_assign_augmented_bitxor() {
    assert!(transpiles("def f():\n    x = 0xFF\n    x ^= 0x0F"));
}

#[test]
fn test_assign_augmented_lshift() {
    assert!(transpiles("def f():\n    x = 1\n    x <<= 2"));
}

#[test]
fn test_assign_augmented_rshift() {
    assert!(transpiles("def f():\n    x = 8\n    x >>= 2"));
}

// =============================================================================
// If Statement Tests
// =============================================================================

#[test]
fn test_if_simple() {
    assert!(transpiles(
        "def f(x):\n    if x > 0:\n        return 1\n    return 0"
    ));
}

#[test]
fn test_if_else() {
    assert!(transpiles(
        "def f(x):\n    if x > 0:\n        return 1\n    else:\n        return -1"
    ));
}

#[test]
fn test_if_elif() {
    assert!(transpiles("def f(x):\n    if x > 0:\n        return 1\n    elif x < 0:\n        return -1\n    else:\n        return 0"));
}

#[test]
fn test_if_elif_chain() {
    assert!(transpiles("def f(x):\n    if x == 1:\n        return 'one'\n    elif x == 2:\n        return 'two'\n    elif x == 3:\n        return 'three'\n    else:\n        return 'other'"));
}

#[test]
fn test_if_nested() {
    assert!(transpiles("def f(x, y):\n    if x > 0:\n        if y > 0:\n            return 1\n        else:\n            return 2\n    else:\n        return 3"));
}

#[test]
fn test_if_no_else() {
    assert!(transpiles(
        "def f(x):\n    if x > 0:\n        print('positive')"
    ));
}

// =============================================================================
// For Loop Tests
// =============================================================================

#[test]
fn test_for_range() {
    assert!(transpiles(
        "def f():\n    for i in range(10):\n        print(i)"
    ));
}

#[test]
fn test_for_list() {
    assert!(transpiles(
        "def f(lst):\n    for item in lst:\n        print(item)"
    ));
}

#[test]
fn test_for_enumerate() {
    assert!(transpiles(
        "def f(lst):\n    for i, item in enumerate(lst):\n        print(i, item)"
    ));
}

#[test]
fn test_for_zip() {
    assert!(transpiles(
        "def f(a, b):\n    for x, y in zip(a, b):\n        print(x, y)"
    ));
}

#[test]
fn test_for_dict_items() {
    assert!(transpiles(
        "def f(d):\n    for k, v in d.items():\n        print(k, v)"
    ));
}

#[test]
fn test_for_dict_keys() {
    assert!(transpiles(
        "def f(d):\n    for k in d.keys():\n        print(k)"
    ));
}

#[test]
fn test_for_dict_values() {
    assert!(transpiles(
        "def f(d):\n    for v in d.values():\n        print(v)"
    ));
}

#[test]
fn test_for_string() {
    assert!(transpiles("def f(s):\n    for c in s:\n        print(c)"));
}

#[test]
fn test_for_nested() {
    assert!(transpiles(
        "def f():\n    for i in range(3):\n        for j in range(3):\n            print(i, j)"
    ));
}

#[test]
fn test_for_with_break() {
    assert!(transpiles("def f(lst):\n    for item in lst:\n        if item == 0:\n            break\n        print(item)"));
}

#[test]
fn test_for_with_continue() {
    assert!(transpiles("def f(lst):\n    for item in lst:\n        if item == 0:\n            continue\n        print(item)"));
}

#[test]
fn test_for_else() {
    assert!(transpiles("def f(lst):\n    for item in lst:\n        if item < 0:\n            break\n    else:\n        print('no negatives')"));
}

// =============================================================================
// While Loop Tests
// =============================================================================

#[test]
fn test_while_simple() {
    assert!(transpiles(
        "def f():\n    x = 0\n    while x < 10:\n        x += 1"
    ));
}

#[test]
fn test_while_true() {
    assert!(transpiles("def f():\n    while True:\n        break"));
}

#[test]
fn test_while_with_break() {
    assert!(transpiles("def f():\n    x = 0\n    while True:\n        x += 1\n        if x > 10:\n            break"));
}

#[test]
fn test_while_with_continue() {
    assert!(transpiles("def f():\n    x = 0\n    while x < 10:\n        x += 1\n        if x == 5:\n            continue\n        print(x)"));
}

#[test]
fn test_while_else() {
    assert!(transpiles(
        "def f(n):\n    while n > 0:\n        n -= 1\n    else:\n        print('done')"
    ));
}

// =============================================================================
// Try/Except Statement Tests
// =============================================================================

#[test]
fn test_try_except_bare() {
    assert!(transpiles(
        "def f():\n    try:\n        x = 1 / 0\n    except:\n        pass"
    ));
}

#[test]
fn test_try_except_typed() {
    assert!(transpiles(
        "def f():\n    try:\n        x = 1 / 0\n    except ZeroDivisionError:\n        pass"
    ));
}

#[test]
fn test_try_except_as() {
    assert!(transpiles(
        "def f():\n    try:\n        x = 1 / 0\n    except Exception as e:\n        print(e)"
    ));
}

#[test]
fn test_try_except_multiple() {
    assert!(transpiles("def f():\n    try:\n        x = int('a')\n    except ValueError:\n        pass\n    except TypeError:\n        pass"));
}

#[test]
fn test_try_except_tuple() {
    assert!(transpiles("def f():\n    try:\n        x = int('a')\n    except (ValueError, TypeError):\n        pass"));
}

#[test]
fn test_try_finally() {
    assert!(transpiles(
        "def f():\n    try:\n        x = 1\n    finally:\n        print('cleanup')"
    ));
}

#[test]
fn test_try_except_finally() {
    assert!(transpiles("def f():\n    try:\n        x = 1 / 0\n    except:\n        pass\n    finally:\n        print('cleanup')"));
}

#[test]
fn test_try_except_else() {
    assert!(transpiles("def f():\n    try:\n        x = 1\n    except:\n        pass\n    else:\n        print('success')"));
}

#[test]
fn test_try_except_else_finally() {
    assert!(transpiles("def f():\n    try:\n        x = 1\n    except:\n        pass\n    else:\n        print('success')\n    finally:\n        print('cleanup')"));
}

// =============================================================================
// Raise Statement Tests
// =============================================================================

#[test]
fn test_raise_simple() {
    assert!(transpiles("def f():\n    raise ValueError()"));
}

#[test]
fn test_raise_with_message() {
    assert!(transpiles(
        "def f():\n    raise ValueError('error message')"
    ));
}

#[test]
fn test_raise_from() {
    assert!(transpiles("def f():\n    try:\n        x = 1 / 0\n    except Exception as e:\n        raise RuntimeError('failed') from e"));
}

#[test]
fn test_raise_bare() {
    assert!(transpiles(
        "def f():\n    try:\n        x = 1 / 0\n    except:\n        raise"
    ));
}

// =============================================================================
// With Statement Tests
// =============================================================================

#[test]
fn test_with_simple() {
    assert!(transpiles(
        "def f():\n    with open('file.txt') as f:\n        pass"
    ));
}

#[test]
fn test_with_no_as() {
    assert!(transpiles("def f(lock):\n    with lock:\n        pass"));
}

#[test]
fn test_with_multiple() {
    assert!(transpiles(
        "def f():\n    with open('a.txt') as a, open('b.txt') as b:\n        pass"
    ));
}

#[test]
fn test_async_with() {
    assert!(transpiles(
        "async def f():\n    async with some_async_context() as ctx:\n        pass"
    ));
}

// =============================================================================
// Return Statement Tests
// =============================================================================

#[test]
fn test_return_none() {
    assert!(transpiles("def f():\n    return"));
}

#[test]
fn test_return_value() {
    assert!(transpiles("def f():\n    return 42"));
}

#[test]
fn test_return_tuple() {
    assert!(transpiles("def f():\n    return 1, 2, 3"));
}

#[test]
fn test_return_conditional() {
    assert!(transpiles(
        "def f(x):\n    return 'positive' if x > 0 else 'non-positive'"
    ));
}

// =============================================================================
// Assert Statement Tests
// =============================================================================

#[test]
fn test_assert_simple() {
    assert!(transpiles("def f(x):\n    assert x > 0"));
}

#[test]
fn test_assert_with_message() {
    assert!(transpiles(
        "def f(x):\n    assert x > 0, 'x must be positive'"
    ));
}

// =============================================================================
// Pass Statement Tests
// =============================================================================

#[test]
fn test_pass_in_function() {
    assert!(transpiles("def f():\n    pass"));
}

#[test]
fn test_pass_in_class() {
    assert!(transpiles("class C:\n    pass"));
}

#[test]
fn test_pass_in_if() {
    assert!(transpiles("def f(x):\n    if x > 0:\n        pass"));
}

// =============================================================================
// Expression Statement Tests
// =============================================================================

#[test]
fn test_expr_stmt_call() {
    assert!(transpiles("def f():\n    print('hello')"));
}

#[test]
fn test_expr_stmt_method() {
    assert!(transpiles("def f(lst):\n    lst.append(1)"));
}

// =============================================================================
// Break/Continue Statement Tests
// =============================================================================

#[test]
fn test_break_in_for() {
    assert!(transpiles(
        "def f():\n    for i in range(10):\n        break"
    ));
}

#[test]
fn test_break_in_while() {
    assert!(transpiles("def f():\n    while True:\n        break"));
}

#[test]
fn test_continue_in_for() {
    assert!(transpiles(
        "def f():\n    for i in range(10):\n        continue"
    ));
}

#[test]
fn test_continue_in_while() {
    assert!(transpiles(
        "def f():\n    x = 0\n    while x < 10:\n        x += 1\n        continue"
    ));
}

// =============================================================================
// Match Statement Tests
// =============================================================================

#[test]
fn test_match_simple() {
    assert!(transpiles("def f(x):\n    match x:\n        case 1:\n            return 'one'\n        case 2:\n            return 'two'\n        case _:\n            return 'other'"));
}

#[test]
fn test_match_tuple() {
    assert!(transpiles("def f(point):\n    match point:\n        case (0, 0):\n            return 'origin'\n        case (x, 0):\n            return 'x-axis'\n        case (0, y):\n            return 'y-axis'\n        case (x, y):\n            return 'other'"));
}

#[test]
fn test_match_sequence() {
    assert!(transpiles("def f(lst):\n    match lst:\n        case []:\n            return 'empty'\n        case [x]:\n            return 'single'\n        case [x, y]:\n            return 'pair'\n        case _:\n            return 'many'"));
}

#[test]
fn test_match_mapping() {
    assert!(transpiles("def f(d):\n    match d:\n        case {'type': 'A'}:\n            return 'type A'\n        case {'type': 'B'}:\n            return 'type B'\n        case _:\n            return 'unknown'"));
}

#[test]
fn test_match_guard() {
    assert!(transpiles("def f(x):\n    match x:\n        case n if n > 0:\n            return 'positive'\n        case n if n < 0:\n            return 'negative'\n        case _:\n            return 'zero'"));
}

#[test]
fn test_match_or_pattern() {
    assert!(transpiles("def f(x):\n    match x:\n        case 1 | 2 | 3:\n            return 'small'\n        case _:\n            return 'other'"));
}

// =============================================================================
// Nested Function Tests
// =============================================================================

#[test]
fn test_nested_function_simple() {
    assert!(transpiles(
        "def outer():\n    def inner():\n        return 1\n    return inner()"
    ));
}

#[test]
fn test_nested_function_closure() {
    assert!(transpiles(
        "def outer(x):\n    def inner():\n        return x * 2\n    return inner()"
    ));
}

#[test]
fn test_nested_function_multiple() {
    assert!(transpiles("def outer():\n    def inner1():\n        return 1\n    def inner2():\n        return 2\n    return inner1() + inner2()"));
}

// =============================================================================
// Global/Nonlocal Tests
// =============================================================================

#[test]
fn test_global_stmt() {
    assert!(transpiles(
        "counter = 0\ndef increment():\n    global counter\n    counter += 1"
    ));
}

#[test]
fn test_nonlocal_stmt() {
    assert!(transpiles("def outer():\n    x = 0\n    def inner():\n        nonlocal x\n        x += 1\n    inner()\n    return x"));
}

// =============================================================================
// Del Statement Tests
// =============================================================================

#[test]
fn test_del_variable() {
    assert!(transpiles("def f():\n    x = 1\n    del x"));
}

#[test]
fn test_del_subscript() {
    assert!(transpiles("def f(d):\n    del d['key']"));
}

#[test]
fn test_del_attribute() {
    assert!(transpiles("def f(obj):\n    del obj.attr"));
}

// =============================================================================
// Async Statement Tests
// =============================================================================

#[test]
fn test_async_def() {
    assert!(transpiles("async def f():\n    return 1"));
}

#[test]
fn test_async_for() {
    assert!(transpiles(
        "async def f(async_iter):\n    async for item in async_iter:\n        print(item)"
    ));
}

#[test]
fn test_async_with_statement() {
    assert!(transpiles(
        "async def f():\n    async with some_context() as ctx:\n        pass"
    ));
}

// =============================================================================
// Generator Tests
// =============================================================================

#[test]
fn test_generator_simple() {
    assert!(transpiles(
        "def gen():\n    yield 1\n    yield 2\n    yield 3"
    ));
}

#[test]
fn test_generator_from_loop() {
    assert!(transpiles(
        "def gen(n):\n    for i in range(n):\n        yield i"
    ));
}

#[test]
fn test_generator_with_condition() {
    assert!(transpiles(
        "def gen(n):\n    for i in range(n):\n        if i % 2 == 0:\n            yield i"
    ));
}

// =============================================================================
// Complex Statement Combinations
// =============================================================================

#[test]
fn test_nested_loops_with_breaks() {
    assert!(transpiles("def f():\n    for i in range(10):\n        for j in range(10):\n            if i * j > 20:\n                break\n        else:\n            continue\n        break"));
}

#[test]
fn test_try_in_loop() {
    assert!(transpiles("def f(items):\n    for item in items:\n        try:\n            process(item)\n        except:\n            continue"));
}

#[test]
fn test_with_in_try() {
    assert!(transpiles("def f():\n    try:\n        with open('file.txt') as f:\n            return f.read()\n    except FileNotFoundError:\n        return ''"));
}

#[test]
fn test_match_in_loop() {
    assert!(transpiles("def f(items):\n    for item in items:\n        match item:\n            case 'stop':\n                break\n            case _:\n                pass"));
}
