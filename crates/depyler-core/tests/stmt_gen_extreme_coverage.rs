//! EXTREME TDD Tests for stmt_gen.rs
//!
//! Comprehensive coverage for statement generation including:
//! - All statement types
//! - Error paths and edge cases
//! - Boundary conditions

use depyler_core::DepylerPipeline;

fn transpile(code: &str) -> Result<String, String> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).map_err(|e| e.to_string())
}

fn transpile_ok(code: &str) -> bool {
    transpile(code).is_ok()
}

fn transpile_contains(code: &str, expected: &str) -> bool {
    match transpile(code) {
        Ok(result) => result.contains(expected),
        Err(_) => false,
    }
}

// ============================================================================
// IF STATEMENTS - Comprehensive Coverage
// ============================================================================

mod if_statements {
    use super::*;

    #[test]
    fn test_simple_if() {
        assert!(transpile_ok("def f(x: int):\n    if x > 0:\n        print('positive')"));
    }

    #[test]
    fn test_if_else() {
        assert!(transpile_ok("def f(x: int):\n    if x > 0:\n        print('pos')\n    else:\n        print('neg')"));
    }

    #[test]
    fn test_if_elif() {
        assert!(transpile_ok("def f(x: int):\n    if x > 0:\n        print('pos')\n    elif x < 0:\n        print('neg')\n    else:\n        print('zero')"));
    }

    #[test]
    fn test_if_multiple_elif() {
        assert!(transpile_ok("def f(x: int) -> str:\n    if x == 1:\n        return 'one'\n    elif x == 2:\n        return 'two'\n    elif x == 3:\n        return 'three'\n    elif x == 4:\n        return 'four'\n    else:\n        return 'other'"));
    }

    #[test]
    fn test_nested_if() {
        assert!(transpile_ok("def f(x: int, y: int):\n    if x > 0:\n        if y > 0:\n            print('both positive')"));
    }

    #[test]
    fn test_if_complex_condition() {
        assert!(transpile_ok("def f(a: int, b: int, c: int):\n    if a > 0 and b > 0 or c < 0:\n        print('complex')"));
    }

    #[test]
    fn test_if_with_in() {
        assert!(transpile_ok("def f(x: int, lst: list):\n    if x in lst:\n        print('found')"));
    }

    #[test]
    fn test_if_with_not_in() {
        assert!(transpile_ok("def f(x: int, lst: list):\n    if x not in lst:\n        print('not found')"));
    }

    #[test]
    fn test_if_with_is_none() {
        assert!(transpile_ok("def f(x):\n    if x is None:\n        print('none')"));
    }

    #[test]
    fn test_if_with_is_not_none() {
        assert!(transpile_ok("def f(x):\n    if x is not None:\n        print('not none')"));
    }

    #[test]
    fn test_if_single_line() {
        // Python allows single-line if
        assert!(transpile_ok("def f(x: int):\n    if x > 0: print('pos')"));
    }
}

// ============================================================================
// FOR LOOPS - Comprehensive Coverage
// ============================================================================

mod for_loops {
    use super::*;

    #[test]
    fn test_for_in_list() {
        assert!(transpile_ok("def f(lst: list):\n    for x in lst:\n        print(x)"));
    }

    #[test]
    fn test_for_in_range() {
        assert!(transpile_ok("def f(n: int):\n    for i in range(n):\n        print(i)"));
    }

    #[test]
    fn test_for_in_range_start_stop() {
        assert!(transpile_ok("def f():\n    for i in range(1, 10):\n        print(i)"));
    }

    #[test]
    fn test_for_in_range_step() {
        assert!(transpile_ok("def f():\n    for i in range(0, 10, 2):\n        print(i)"));
    }

    #[test]
    fn test_for_in_string() {
        assert!(transpile_ok("def f(s: str):\n    for c in s:\n        print(c)"));
    }

    #[test]
    fn test_for_in_dict() {
        assert!(transpile_ok("def f(d: dict):\n    for k in d:\n        print(k)"));
    }

    #[test]
    fn test_for_in_dict_items() {
        assert!(transpile_ok("def f(d: dict):\n    for k, v in d.items():\n        print(k, v)"));
    }

    #[test]
    fn test_for_in_dict_keys() {
        assert!(transpile_ok("def f(d: dict):\n    for k in d.keys():\n        print(k)"));
    }

    #[test]
    fn test_for_in_dict_values() {
        assert!(transpile_ok("def f(d: dict):\n    for v in d.values():\n        print(v)"));
    }

    #[test]
    fn test_for_enumerate() {
        assert!(transpile_ok("def f(lst: list):\n    for i, x in enumerate(lst):\n        print(i, x)"));
    }

    #[test]
    fn test_for_zip() {
        assert!(transpile_ok("def f(a: list, b: list):\n    for x, y in zip(a, b):\n        print(x, y)"));
    }

    #[test]
    fn test_for_with_break() {
        assert!(transpile_ok("def f(lst: list):\n    for x in lst:\n        if x > 10:\n            break\n        print(x)"));
    }

    #[test]
    fn test_for_with_continue() {
        assert!(transpile_ok("def f(lst: list):\n    for x in lst:\n        if x < 0:\n            continue\n        print(x)"));
    }

    #[test]
    fn test_for_with_else() {
        assert!(transpile_ok("def f(lst: list):\n    for x in lst:\n        if x > 100:\n            break\n    else:\n        print('completed')"));
    }

    #[test]
    fn test_for_tuple_unpack() {
        assert!(transpile_ok("def f(pairs: list):\n    for a, b in pairs:\n        print(a, b)"));
    }

    #[test]
    fn test_for_triple_unpack() {
        assert!(transpile_ok("def f(triples: list):\n    for a, b, c in triples:\n        print(a, b, c)"));
    }

    #[test]
    fn test_nested_for() {
        assert!(transpile_ok("def f(matrix: list):\n    for row in matrix:\n        for x in row:\n            print(x)"));
    }

    #[test]
    fn test_for_in_reversed() {
        assert!(transpile_ok("def f(lst: list):\n    for x in reversed(lst):\n        print(x)"));
    }

    #[test]
    fn test_for_in_sorted() {
        assert!(transpile_ok("def f(lst: list):\n    for x in sorted(lst):\n        print(x)"));
    }
}

// ============================================================================
// WHILE LOOPS - Comprehensive Coverage
// ============================================================================

mod while_loops {
    use super::*;

    #[test]
    fn test_while_simple() {
        assert!(transpile_ok("def f():\n    x = 0\n    while x < 10:\n        x += 1"));
    }

    #[test]
    fn test_while_true() {
        assert!(transpile_ok("def f():\n    while True:\n        break"));
    }

    #[test]
    fn test_while_with_break() {
        assert!(transpile_ok("def f():\n    x = 0\n    while True:\n        x += 1\n        if x > 10:\n            break"));
    }

    #[test]
    fn test_while_with_continue() {
        assert!(transpile_ok("def f():\n    x = 0\n    while x < 10:\n        x += 1\n        if x == 5:\n            continue\n        print(x)"));
    }

    #[test]
    fn test_while_with_else() {
        assert!(transpile_ok("def f():\n    x = 0\n    while x < 5:\n        x += 1\n    else:\n        print('done')"));
    }

    #[test]
    fn test_while_complex_condition() {
        assert!(transpile_ok("def f(lst: list):\n    i = 0\n    while i < len(lst) and lst[i] > 0:\n        i += 1"));
    }
}

// ============================================================================
// ASSIGNMENTS - Comprehensive Coverage
// ============================================================================

mod assignments {
    use super::*;

    #[test]
    fn test_simple_assign() {
        assert!(transpile_ok("def f():\n    x = 5"));
    }

    #[test]
    fn test_multiple_assign() {
        assert!(transpile_ok("def f():\n    x = y = z = 0"));
    }

    #[test]
    fn test_tuple_unpack() {
        assert!(transpile_ok("def f():\n    a, b = 1, 2"));
    }

    #[test]
    fn test_list_unpack() {
        assert!(transpile_ok("def f(pair: list):\n    x, y = pair"));
    }

    #[test]
    fn test_starred_unpack() {
        assert!(transpile_ok("def f(lst: list):\n    first, *rest = lst"));
    }

    #[test]
    fn test_starred_middle() {
        assert!(transpile_ok("def f(lst: list):\n    first, *middle, last = lst"));
    }

    #[test]
    fn test_subscript_assign() {
        assert!(transpile_ok("def f(lst: list):\n    lst[0] = 42"));
    }

    #[test]
    fn test_dict_assign() {
        assert!(transpile_ok("def f(d: dict):\n    d['key'] = 'value'"));
    }

    #[test]
    fn test_attribute_assign() {
        assert!(transpile_ok("class Foo:\n    def __init__(self):\n        self.x = 0\ndef f(obj: Foo):\n    obj.x = 42"));
    }

    #[test]
    fn test_augmented_add() {
        assert!(transpile_ok("def f():\n    x = 0\n    x += 5"));
    }

    #[test]
    fn test_augmented_sub() {
        assert!(transpile_ok("def f():\n    x = 10\n    x -= 3"));
    }

    #[test]
    fn test_augmented_mul() {
        assert!(transpile_ok("def f():\n    x = 2\n    x *= 3"));
    }

    #[test]
    fn test_augmented_div() {
        assert!(transpile_ok("def f():\n    x = 10.0\n    x /= 2.0"));
    }

    #[test]
    fn test_augmented_floordiv() {
        assert!(transpile_ok("def f():\n    x = 10\n    x //= 3"));
    }

    #[test]
    fn test_augmented_mod() {
        assert!(transpile_ok("def f():\n    x = 10\n    x %= 3"));
    }

    #[test]
    fn test_augmented_pow() {
        assert!(transpile_ok("def f():\n    x = 2\n    x **= 3"));
    }

    #[test]
    fn test_augmented_and() {
        assert!(transpile_ok("def f():\n    x = 0xFF\n    x &= 0x0F"));
    }

    #[test]
    fn test_augmented_or() {
        assert!(transpile_ok("def f():\n    x = 0x0F\n    x |= 0xF0"));
    }

    #[test]
    fn test_augmented_xor() {
        assert!(transpile_ok("def f():\n    x = 0xFF\n    x ^= 0x0F"));
    }

    #[test]
    fn test_augmented_lshift() {
        assert!(transpile_ok("def f():\n    x = 1\n    x <<= 4"));
    }

    #[test]
    fn test_augmented_rshift() {
        assert!(transpile_ok("def f():\n    x = 16\n    x >>= 2"));
    }

    #[test]
    fn test_annotated_assign() {
        assert!(transpile_ok("def f():\n    x: int = 5"));
    }

    #[test]
    fn test_annotation_no_value() {
        assert!(transpile_ok("def f():\n    x: int\n    x = 5"));
    }
}

// ============================================================================
// RETURN STATEMENTS - Comprehensive Coverage
// ============================================================================

mod returns {
    use super::*;

    #[test]
    fn test_return_none() {
        assert!(transpile_ok("def f():\n    return"));
    }

    #[test]
    fn test_return_none_explicit() {
        assert!(transpile_ok("def f():\n    return None"));
    }

    #[test]
    fn test_return_int() {
        assert!(transpile_ok("def f() -> int:\n    return 42"));
    }

    #[test]
    fn test_return_expr() {
        assert!(transpile_ok("def f(x: int) -> int:\n    return x * 2 + 1"));
    }

    #[test]
    fn test_return_tuple() {
        assert!(transpile_ok("def f():\n    return 1, 2, 3"));
    }

    #[test]
    fn test_return_tuple_explicit() {
        assert!(transpile_ok("def f():\n    return (1, 2)"));
    }

    #[test]
    fn test_return_list() {
        assert!(transpile_ok("def f() -> list:\n    return [1, 2, 3]"));
    }

    #[test]
    fn test_return_dict() {
        assert!(transpile_ok("def f() -> dict:\n    return {'a': 1}"));
    }

    #[test]
    fn test_early_return() {
        assert!(transpile_ok("def f(x: int) -> int:\n    if x < 0:\n        return 0\n    return x"));
    }

    #[test]
    fn test_return_in_loop() {
        assert!(transpile_ok("def f(lst: list) -> int:\n    for x in lst:\n        if x > 10:\n            return x\n    return -1"));
    }
}

// ============================================================================
// TRY/EXCEPT - Comprehensive Coverage
// ============================================================================

mod try_except {
    use super::*;

    #[test]
    fn test_try_bare_except() {
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
    fn test_try_multiple_except() {
        assert!(transpile_ok("def f():\n    try:\n        x = int('a')\n    except ValueError:\n        print('value')\n    except TypeError:\n        print('type')"));
    }

    #[test]
    fn test_try_tuple_except() {
        assert!(transpile_ok("def f():\n    try:\n        x = int('a')\n    except (ValueError, TypeError):\n        print('error')"));
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

    #[test]
    fn test_try_except_else_finally() {
        assert!(transpile_ok("def f():\n    try:\n        x = 1\n    except:\n        print('error')\n    else:\n        print('ok')\n    finally:\n        print('done')"));
    }

    #[test]
    fn test_nested_try() {
        assert!(transpile_ok("def f():\n    try:\n        try:\n            x = 1/0\n        except:\n            raise\n    except:\n        print('outer')"));
    }
}

// ============================================================================
// RAISE STATEMENTS - Comprehensive Coverage
// ============================================================================

mod raise_statements {
    use super::*;

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

    #[test]
    fn test_raise_from() {
        assert!(transpile_ok("def f():\n    try:\n        x = 1/0\n    except Exception as e:\n        raise RuntimeError('wrapped') from e"));
    }

    #[test]
    fn test_raise_from_none() {
        assert!(transpile_ok("def f():\n    try:\n        x = 1/0\n    except:\n        raise ValueError('new') from None"));
    }
}

// ============================================================================
// ASSERT STATEMENTS - Comprehensive Coverage
// ============================================================================

mod assert_statements {
    use super::*;

    #[test]
    fn test_assert_simple() {
        assert!(transpile_ok("def f(x: int):\n    assert x > 0"));
    }

    #[test]
    fn test_assert_with_message() {
        assert!(transpile_ok("def f(x: int):\n    assert x > 0, 'x must be positive'"));
    }

    #[test]
    fn test_assert_complex_condition() {
        assert!(transpile_ok("def f(x: int, y: int):\n    assert x > 0 and y > 0, 'both must be positive'"));
    }
}

// ============================================================================
// WITH STATEMENTS - Comprehensive Coverage
// ============================================================================

mod with_statements {
    use super::*;

    #[test]
    fn test_with_simple() {
        assert!(transpile_ok("def f(path: str):\n    with open(path) as f:\n        return f.read()"));
    }

    #[test]
    fn test_with_no_as() {
        assert!(transpile_ok("def f(lock):\n    with lock:\n        print('locked')"));
    }

    #[test]
    fn test_with_multiple() {
        assert!(transpile_ok("def f(a: str, b: str):\n    with open(a) as fa, open(b) as fb:\n        print(fa.read(), fb.read())"));
    }

    #[test]
    fn test_nested_with() {
        assert!(transpile_ok("def f(a: str, b: str):\n    with open(a) as fa:\n        with open(b) as fb:\n            print(fa.read(), fb.read())"));
    }

    #[test]
    fn test_with_exception() {
        assert!(transpile_ok("def f(path: str):\n    try:\n        with open(path) as f:\n            return f.read()\n    except:\n        return ''"));
    }
}

// ============================================================================
// PASS/BREAK/CONTINUE - Comprehensive Coverage
// ============================================================================

mod control_flow {
    use super::*;

    #[test]
    fn test_pass_function() {
        assert!(transpile_ok("def f():\n    pass"));
    }

    #[test]
    fn test_pass_in_if() {
        assert!(transpile_ok("def f(x: int):\n    if x > 0:\n        pass"));
    }

    #[test]
    fn test_pass_in_class() {
        assert!(transpile_ok("class Empty:\n    pass"));
    }

    #[test]
    fn test_break_for() {
        assert!(transpile_ok("def f():\n    for i in range(10):\n        break"));
    }

    #[test]
    fn test_break_while() {
        assert!(transpile_ok("def f():\n    while True:\n        break"));
    }

    #[test]
    fn test_continue_for() {
        assert!(transpile_ok("def f():\n    for i in range(10):\n        continue"));
    }

    #[test]
    fn test_continue_while() {
        assert!(transpile_ok("def f():\n    x = 0\n    while x < 10:\n        x += 1\n        continue"));
    }
}

// ============================================================================
// DELETE STATEMENTS - Comprehensive Coverage
// ============================================================================

mod delete_statements {
    use super::*;

    #[test]
    fn test_del_variable() {
        assert!(transpile_ok("def f():\n    x = 5\n    del x"));
    }

    #[test]
    fn test_del_subscript() {
        assert!(transpile_ok("def f(lst: list):\n    del lst[0]"));
    }

    #[test]
    fn test_del_dict_key() {
        assert!(transpile_ok("def f(d: dict):\n    del d['key']"));
    }

    #[test]
    fn test_del_multiple() {
        assert!(transpile_ok("def f():\n    a = 1\n    b = 2\n    del a, b"));
    }
}

// ============================================================================
// GLOBAL/NONLOCAL - Comprehensive Coverage
// ============================================================================

mod global_nonlocal {
    use super::*;

    #[test]
    fn test_global_simple() {
        assert!(transpile_ok("x = 0\ndef f():\n    global x\n    x = 1"));
    }

    #[test]
    fn test_global_multiple() {
        assert!(transpile_ok("a = 0\nb = 0\ndef f():\n    global a, b\n    a = 1\n    b = 2"));
    }

    #[test]
    fn test_nonlocal() {
        assert!(transpile_ok("def outer():\n    x = 0\n    def inner():\n        nonlocal x\n        x = 1\n    inner()"));
    }
}

// ============================================================================
// CLASS DEFINITIONS - Comprehensive Coverage
// ============================================================================

mod classes {
    use super::*;

    #[test]
    fn test_class_empty() {
        assert!(transpile_ok("class Empty:\n    pass"));
    }

    #[test]
    fn test_class_with_init() {
        assert!(transpile_ok("class Foo:\n    def __init__(self):\n        self.x = 0"));
    }

    #[test]
    fn test_class_with_methods() {
        assert!(transpile_ok("class Foo:\n    def __init__(self, x: int):\n        self.x = x\n    def get_x(self) -> int:\n        return self.x"));
    }

    #[test]
    fn test_class_with_classmethod() {
        assert!(transpile_ok("class Foo:\n    @classmethod\n    def create(cls) -> 'Foo':\n        return cls()"));
    }

    #[test]
    fn test_class_with_staticmethod() {
        assert!(transpile_ok("class Foo:\n    @staticmethod\n    def helper() -> int:\n        return 42"));
    }

    #[test]
    fn test_class_with_property() {
        assert!(transpile_ok("class Foo:\n    def __init__(self):\n        self._x = 0\n    @property\n    def x(self) -> int:\n        return self._x"));
    }

    #[test]
    fn test_class_inheritance() {
        assert!(transpile_ok("class Base:\n    pass\nclass Derived(Base):\n    pass"));
    }

    #[test]
    fn test_dataclass() {
        assert!(transpile_ok("from dataclasses import dataclass\n@dataclass\nclass Point:\n    x: int\n    y: int"));
    }
}

// ============================================================================
// MATCH STATEMENTS - Comprehensive Coverage
// ============================================================================

mod match_statements {
    use super::*;

    #[test]
    fn test_match_simple() {
        assert!(transpile_ok("def f(x: int) -> str:\n    match x:\n        case 1:\n            return 'one'\n        case 2:\n            return 'two'\n        case _:\n            return 'other'"));
    }

    #[test]
    fn test_match_multiple_patterns() {
        assert!(transpile_ok("def f(x: int) -> str:\n    match x:\n        case 1 | 2 | 3:\n            return 'small'\n        case _:\n            return 'big'"));
    }

    #[test]
    fn test_match_with_guard() {
        assert!(transpile_ok("def f(x: int) -> str:\n    match x:\n        case n if n > 0:\n            return 'positive'\n        case _:\n            return 'not positive'"));
    }

    #[test]
    fn test_match_capture() {
        assert!(transpile_ok("def f(x: int) -> int:\n    match x:\n        case n:\n            return n * 2"));
    }

    #[test]
    fn test_match_sequence() {
        assert!(transpile_ok("def f(lst: list):\n    match lst:\n        case [a, b]:\n            return a + b\n        case _:\n            return 0"));
    }
}

// ============================================================================
// FUNCTION DEFINITIONS - Comprehensive Coverage
// ============================================================================

mod functions {
    use super::*;

    #[test]
    fn test_function_no_params() {
        assert!(transpile_ok("def f():\n    pass"));
    }

    #[test]
    fn test_function_one_param() {
        assert!(transpile_ok("def f(x: int):\n    pass"));
    }

    #[test]
    fn test_function_multiple_params() {
        assert!(transpile_ok("def f(a: int, b: str, c: float):\n    pass"));
    }

    #[test]
    fn test_function_default_value() {
        assert!(transpile_ok("def f(x: int = 0):\n    pass"));
    }

    #[test]
    fn test_function_multiple_defaults() {
        assert!(transpile_ok("def f(a: int, b: int = 0, c: str = 'default'):\n    pass"));
    }

    #[test]
    fn test_function_return_type() {
        assert!(transpile_ok("def f() -> int:\n    return 42"));
    }

    #[test]
    fn test_function_varargs() {
        assert!(transpile_ok("def f(*args):\n    for arg in args:\n        print(arg)"));
    }

    #[test]
    fn test_function_kwargs() {
        assert!(transpile_ok("def f(**kwargs):\n    for k, v in kwargs.items():\n        print(k, v)"));
    }

    #[test]
    fn test_function_args_kwargs() {
        assert!(transpile_ok("def f(*args, **kwargs):\n    pass"));
    }

    #[test]
    fn test_nested_function() {
        assert!(transpile_ok("def outer():\n    def inner():\n        return 42\n    return inner()"));
    }

    #[test]
    fn test_recursive_function() {
        assert!(transpile_ok("def factorial(n: int) -> int:\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)"));
    }

    #[test]
    fn test_docstring() {
        assert!(transpile_ok("def f():\n    '''This is a docstring'''\n    pass"));
    }

    #[test]
    fn test_decorator_simple() {
        assert!(transpile_ok("def decorator(f):\n    return f\n@decorator\ndef f():\n    pass"));
    }
}

// ============================================================================
// IMPORT STATEMENTS - Comprehensive Coverage
// ============================================================================

mod imports {
    use super::*;

    #[test]
    fn test_import_simple() {
        assert!(transpile_ok("import os\ndef f():\n    return os.getcwd()"));
    }

    #[test]
    fn test_import_multiple() {
        assert!(transpile_ok("import os, sys\ndef f():\n    pass"));
    }

    #[test]
    fn test_import_as() {
        assert!(transpile_ok("import numpy as np\ndef f():\n    pass"));
    }

    #[test]
    fn test_from_import() {
        assert!(transpile_ok("from os import path\ndef f():\n    return path.exists('.')"));
    }

    #[test]
    fn test_from_import_multiple() {
        assert!(transpile_ok("from os import path, getcwd\ndef f():\n    pass"));
    }

    #[test]
    fn test_from_import_as() {
        assert!(transpile_ok("from os import path as p\ndef f():\n    pass"));
    }

    #[test]
    fn test_from_import_star() {
        assert!(transpile_ok("from typing import *\ndef f(x: Optional[int]):\n    pass"));
    }
}

// ============================================================================
// EXPRESSION STATEMENTS - Comprehensive Coverage
// ============================================================================

mod expr_statements {
    use super::*;

    #[test]
    fn test_function_call_statement() {
        assert!(transpile_ok("def f():\n    print('hello')"));
    }

    #[test]
    fn test_method_call_statement() {
        assert!(transpile_ok("def f(lst: list):\n    lst.append(1)"));
    }

    #[test]
    fn test_string_literal_statement() {
        // String literals are valid statements (often docstrings)
        assert!(transpile_ok("def f():\n    'docstring'\n    pass"));
    }
}

// ============================================================================
// EDGE CASES AND BOUNDARY CONDITIONS
// ============================================================================

mod edge_cases {
    use super::*;

    #[test]
    fn test_empty_body_pass() {
        assert!(transpile_ok("def f():\n    pass"));
    }

    #[test]
    fn test_many_elif() {
        let mut code = "def f(x: int) -> int:\n    if x == 0:\n        return 0".to_string();
        for i in 1..20 {
            code.push_str(&format!("\n    elif x == {}:\n        return {}", i, i));
        }
        code.push_str("\n    else:\n        return -1");
        assert!(transpile_ok(&code));
    }

    #[test]
    fn test_deeply_nested_loops() {
        assert!(transpile_ok("def f():\n    for i in range(5):\n        for j in range(5):\n            for k in range(5):\n                print(i, j, k)"));
    }

    #[test]
    fn test_complex_comprehension() {
        assert!(transpile_ok("def f(matrix: list) -> list:\n    return [[x*2 for x in row if x > 0] for row in matrix if len(row) > 0]"));
    }

    #[test]
    fn test_chained_method_calls() {
        assert!(transpile_ok("def f(s: str) -> str:\n    return s.strip().lower().replace('a', 'b').upper().split()[0]"));
    }

    #[test]
    fn test_multiline_string_in_statement() {
        assert!(transpile_ok("def f():\n    s = '''line1\nline2\nline3'''\n    return s"));
    }
}
