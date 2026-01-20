//! Test error paths and edge cases in stmt_gen.rs for coverage

use depyler_core::DepylerPipeline;

fn transpile(code: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).unwrap_or_else(|e| format!("ERROR: {}", e))
}

fn transpile_ok(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_ok()
}

#[allow(dead_code)]
fn transpile_err(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_err()
}

// ============ For loop variations ============

#[test]
fn test_for_loop_simple() {
    assert!(transpile_ok("def f(items: list):\n    for x in items:\n        print(x)"));
}

#[test]
fn test_for_loop_tuple_unpack() {
    assert!(transpile_ok("def f(items: list):\n    for x, y in items:\n        print(x, y)"));
}

#[test]
fn test_for_loop_enumerate() {
    assert!(transpile_ok("def f(items: list):\n    for i, x in enumerate(items):\n        print(i, x)"));
}

#[test]
fn test_for_loop_with_break() {
    assert!(transpile_ok("def f(items: list):\n    for x in items:\n        if x > 5:\n            break"));
}

#[test]
fn test_for_loop_with_continue() {
    assert!(transpile_ok("def f(items: list):\n    for x in items:\n        if x < 0:\n            continue\n        print(x)"));
}

#[test]
fn test_for_loop_with_else() {
    assert!(transpile_ok("def f(items: list):\n    for x in items:\n        if x > 10:\n            break\n    else:\n        print('done')"));
}

#[test]
fn test_for_loop_range() {
    assert!(transpile_ok("def f(n: int):\n    for i in range(n):\n        print(i)"));
}

#[test]
fn test_for_loop_range_start_stop() {
    assert!(transpile_ok("def f(n: int):\n    for i in range(0, n):\n        print(i)"));
}

#[test]
fn test_for_loop_range_step() {
    assert!(transpile_ok("def f(n: int):\n    for i in range(0, n, 2):\n        print(i)"));
}

// ============ While loop variations ============

#[test]
fn test_while_loop_simple() {
    assert!(transpile_ok("def f():\n    x = 0\n    while x < 10:\n        x += 1"));
}

#[test]
fn test_while_loop_with_break() {
    assert!(transpile_ok("def f():\n    while True:\n        break"));
}

#[test]
fn test_while_loop_with_continue() {
    assert!(transpile_ok("def f():\n    x = 0\n    while x < 10:\n        x += 1\n        if x == 5:\n            continue"));
}

#[test]
fn test_while_loop_with_else() {
    assert!(transpile_ok("def f():\n    x = 0\n    while x < 10:\n        x += 1\n    else:\n        print('done')"));
}

// ============ If statement variations ============

#[test]
fn test_if_simple() {
    assert!(transpile_ok("def f(x: int):\n    if x > 0:\n        print('positive')"));
}

#[test]
fn test_if_else() {
    assert!(transpile_ok("def f(x: int):\n    if x > 0:\n        print('positive')\n    else:\n        print('not positive')"));
}

#[test]
fn test_if_elif_else() {
    assert!(transpile_ok("def f(x: int):\n    if x > 0:\n        print('positive')\n    elif x < 0:\n        print('negative')\n    else:\n        print('zero')"));
}

#[test]
fn test_if_multiple_elif() {
    assert!(transpile_ok("def f(x: int):\n    if x == 1:\n        return 'one'\n    elif x == 2:\n        return 'two'\n    elif x == 3:\n        return 'three'\n    else:\n        return 'other'"));
}

#[test]
fn test_if_nested() {
    assert!(transpile_ok("def f(x: int, y: int):\n    if x > 0:\n        if y > 0:\n            print('both positive')"));
}

// ============ Assignment variations ============

#[test]
fn test_simple_assignment() {
    assert!(transpile_ok("def f():\n    x = 5"));
}

#[test]
fn test_multiple_assignment() {
    assert!(transpile_ok("def f():\n    x = y = 5"));
}

#[test]
fn test_tuple_unpack_assignment() {
    assert!(transpile_ok("def f():\n    x, y = 1, 2"));
}

#[test]
fn test_list_unpack_assignment() {
    assert!(transpile_ok("def f(pair: list):\n    x, y = pair"));
}

#[test]
fn test_augmented_assignment_add() {
    assert!(transpile_ok("def f():\n    x = 0\n    x += 1"));
}

#[test]
fn test_augmented_assignment_sub() {
    assert!(transpile_ok("def f():\n    x = 10\n    x -= 1"));
}

#[test]
fn test_augmented_assignment_mul() {
    assert!(transpile_ok("def f():\n    x = 2\n    x *= 3"));
}

#[test]
fn test_augmented_assignment_div() {
    assert!(transpile_ok("def f():\n    x = 10.0\n    x /= 2.0"));
}

#[test]
fn test_augmented_assignment_floordiv() {
    assert!(transpile_ok("def f():\n    x = 10\n    x //= 3"));
}

#[test]
fn test_augmented_assignment_mod() {
    assert!(transpile_ok("def f():\n    x = 10\n    x %= 3"));
}

#[test]
fn test_augmented_assignment_and() {
    assert!(transpile_ok("def f():\n    x = 0xFF\n    x &= 0x0F"));
}

#[test]
fn test_augmented_assignment_or() {
    assert!(transpile_ok("def f():\n    x = 0x0F\n    x |= 0xF0"));
}

#[test]
fn test_augmented_assignment_xor() {
    assert!(transpile_ok("def f():\n    x = 0xFF\n    x ^= 0x0F"));
}

// ============ Try/Except variations ============

#[test]
fn test_try_except_simple() {
    assert!(transpile_ok("def f():\n    try:\n        x = 1/0\n    except:\n        print('error')"));
}

#[test]
fn test_try_except_specific() {
    assert!(transpile_ok("def f():\n    try:\n        x = 1/0\n    except ZeroDivisionError:\n        print('div by zero')"));
}

#[test]
fn test_try_except_as() {
    assert!(transpile_ok("def f():\n    try:\n        x = 1/0\n    except Exception as e:\n        print(e)"));
}

#[test]
fn test_try_except_multiple() {
    assert!(transpile_ok("def f():\n    try:\n        x = int('a')\n    except ValueError:\n        print('value error')\n    except TypeError:\n        print('type error')"));
}

#[test]
fn test_try_except_else() {
    assert!(transpile_ok("def f():\n    try:\n        x = 1\n    except:\n        print('error')\n    else:\n        print('success')"));
}

#[test]
fn test_try_finally() {
    assert!(transpile_ok("def f():\n    try:\n        x = 1\n    finally:\n        print('cleanup')"));
}

#[test]
fn test_try_except_finally() {
    assert!(transpile_ok("def f():\n    try:\n        x = 1/0\n    except:\n        print('error')\n    finally:\n        print('cleanup')"));
}

// ============ With statement variations ============

#[test]
fn test_with_simple() {
    assert!(transpile_ok("def f():\n    with open('file.txt') as f:\n        print(f.read())"));
}

#[test]
fn test_with_multiple() {
    assert!(transpile_ok("def f():\n    with open('a.txt') as a, open('b.txt') as b:\n        print(a.read(), b.read())"));
}

#[test]
fn test_with_no_as() {
    assert!(transpile_ok("def f(lock):\n    with lock:\n        print('locked')"));
}

// ============ Return statement variations ============

#[test]
fn test_return_none() {
    assert!(transpile_ok("def f():\n    return"));
}

#[test]
fn test_return_value() {
    assert!(transpile_ok("def f() -> int:\n    return 42"));
}

#[test]
fn test_return_expression() {
    assert!(transpile_ok("def f(x: int) -> int:\n    return x * 2 + 1"));
}

#[test]
fn test_return_tuple() {
    assert!(transpile_ok("def f():\n    return 1, 2, 3"));
}

// ============ Raise statement variations ============

#[test]
fn test_raise_simple() {
    assert!(transpile_ok("def f():\n    raise ValueError()"));
}

#[test]
fn test_raise_with_message() {
    assert!(transpile_ok("def f():\n    raise ValueError('error message')"));
}

#[test]
fn test_raise_bare() {
    assert!(transpile_ok("def f():\n    try:\n        x = 1\n    except:\n        raise"));
}

// ============ Assert statement ============

#[test]
fn test_assert_simple() {
    assert!(transpile_ok("def f(x: int):\n    assert x > 0"));
}

#[test]
fn test_assert_with_message() {
    assert!(transpile_ok("def f(x: int):\n    assert x > 0, 'x must be positive'"));
}

// ============ Pass statement ============

#[test]
fn test_pass() {
    assert!(transpile_ok("def f():\n    pass"));
}

#[test]
fn test_pass_in_if() {
    assert!(transpile_ok("def f(x: int):\n    if x > 0:\n        pass"));
}

// ============ Delete statement ============

#[test]
fn test_del_variable() {
    assert!(transpile_ok("def f():\n    x = 5\n    del x"));
}

// ============ Global/Nonlocal ============

#[test]
fn test_global_statement() {
    let code = "x = 0\ndef f():\n    global x\n    x = 1";
    assert!(transpile_ok(code));
}

// ============ Class statements ============

#[test]
fn test_class_simple() {
    assert!(transpile_ok("class Foo:\n    def __init__(self):\n        self.x = 0"));
}

#[test]
fn test_class_with_method() {
    assert!(transpile_ok("class Foo:\n    def __init__(self):\n        self.x = 0\n    def get_x(self) -> int:\n        return self.x"));
}

#[test]
fn test_class_with_class_method() {
    assert!(transpile_ok("class Foo:\n    @classmethod\n    def create(cls) -> 'Foo':\n        return cls()"));
}

#[test]
fn test_class_with_static_method() {
    assert!(transpile_ok("class Foo:\n    @staticmethod\n    def helper() -> int:\n        return 42"));
}

#[test]
fn test_class_with_property() {
    assert!(transpile_ok("class Foo:\n    def __init__(self):\n        self._x = 0\n    @property\n    def x(self) -> int:\n        return self._x"));
}

// ============ Dict augmented assignment error path ============

#[test]
fn test_dict_augmented_add() {
    assert!(transpile_ok("def f(d: dict):\n    d['key'] += 1"));
}

#[test]
fn test_dict_augmented_sub() {
    assert!(transpile_ok("def f(d: dict):\n    d['key'] -= 1"));
}

#[test]
fn test_dict_augmented_mul() {
    assert!(transpile_ok("def f(d: dict):\n    d['key'] *= 2"));
}

// ============ List comprehension statements ============

#[test]
fn test_list_comp_in_assignment() {
    assert!(transpile_ok("def f(items: list):\n    result = [x * 2 for x in items]"));
}

#[test]
fn test_list_comp_with_if() {
    assert!(transpile_ok("def f(items: list):\n    result = [x for x in items if x > 0]"));
}

#[test]
fn test_list_comp_nested() {
    assert!(transpile_ok("def f(matrix: list):\n    flat = [x for row in matrix for x in row]"));
}

// ============ Dict comprehension statements ============

#[test]
fn test_dict_comp_in_assignment() {
    assert!(transpile_ok("def f(items: list):\n    result = {x: x*2 for x in items}"));
}

#[test]
fn test_dict_comp_with_if() {
    assert!(transpile_ok("def f(items: list):\n    result = {x: x*2 for x in items if x > 0}"));
}

// ============ Set comprehension statements ============

#[test]
fn test_set_comp_in_assignment() {
    assert!(transpile_ok("def f(items: list):\n    result = {x * 2 for x in items}"));
}

#[test]
fn test_set_comp_with_if() {
    assert!(transpile_ok("def f(items: list):\n    result = {x for x in items if x > 0}"));
}

// ============ Generator expression statements ============

#[test]
fn test_generator_in_function_call() {
    assert!(transpile_ok("def f(items: list) -> int:\n    return sum(x for x in items)"));
}

#[test]
fn test_generator_with_if() {
    assert!(transpile_ok("def f(items: list) -> int:\n    return sum(x for x in items if x > 0)"));
}

// ============ Match statement ============

#[test]
fn test_match_simple() {
    assert!(transpile_ok("def f(x: int) -> str:\n    match x:\n        case 1:\n            return 'one'\n        case 2:\n            return 'two'\n        case _:\n            return 'other'"));
}

#[test]
fn test_match_with_guard() {
    assert!(transpile_ok("def f(x: int) -> str:\n    match x:\n        case n if n > 0:\n            return 'positive'\n        case _:\n            return 'not positive'"));
}

// ============ Import statements ============

#[test]
fn test_import_simple() {
    assert!(transpile_ok("import os\ndef f():\n    print(os.getcwd())"));
}

#[test]
fn test_import_from() {
    assert!(transpile_ok("from os import path\ndef f():\n    print(path.exists('.'))"));
}

#[test]
fn test_import_as() {
    assert!(transpile_ok("import numpy as np\ndef f():\n    x = np.array([1, 2, 3])"));
}

// ============ Function with decorators ============

#[test]
fn test_function_with_staticmethod() {
    assert!(transpile_ok("class Foo:\n    @staticmethod\n    def bar() -> int:\n        return 42"));
}

// ============ Async variations (if supported) ============

#[test]
fn test_async_function() {
    // Async might not be fully supported, just test it doesn't crash
    let _ = transpile("async def f():\n    await something()");
}

// ============ Expression statements ============

#[test]
fn test_expression_statement_call() {
    assert!(transpile_ok("def f():\n    print('hello')"));
}

#[test]
fn test_expression_statement_method_call() {
    assert!(transpile_ok("def f(lst: list):\n    lst.append(1)"));
}

// ============ Annotations ============

#[test]
fn test_variable_annotation() {
    assert!(transpile_ok("def f():\n    x: int = 5"));
}

#[test]
fn test_variable_annotation_no_value() {
    assert!(transpile_ok("def f():\n    x: int\n    x = 5"));
}

// ============ Walrus operator in statements ============

#[test]
fn test_walrus_in_if_stmt() {
    assert!(transpile_ok("def f(data: list):\n    if (n := len(data)) > 0:\n        print(n)"));
}

#[test]
fn test_walrus_in_while_stmt() {
    assert!(transpile_ok("def f():\n    line = 'test'\n    while (n := len(line)) > 0:\n        print(n)\n        line = ''"));
}

// ============ Floor division edge cases ============

#[test]
fn test_floor_div_in_assignment() {
    assert!(transpile_ok("def f(a: int, b: int) -> int:\n    return a // b"));
}

#[test]
fn test_floor_div_augmented() {
    assert!(transpile_ok("def f():\n    x = 10\n    x //= 3"));
}
