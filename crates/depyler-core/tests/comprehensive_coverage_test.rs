//! Comprehensive coverage tests targeting specific uncovered code paths
//!
//! This file contains tests designed to maximize coverage in:
//! - stmt_gen.rs
//! - expr_gen.rs
//! - func_gen.rs
//! - direct_rules.rs

use depyler_core::DepylerPipeline;

fn ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// ASSIGNMENT VARIATIONS (codegen_assign_*)
// ============================================================================

#[test] fn test_assign_simple() { assert!(ok("x = 1")); }
#[test] fn test_assign_float() { assert!(ok("x = 1.5")); }
#[test] fn test_assign_string() { assert!(ok("x = 'hello'")); }
#[test] fn test_assign_bytes() { assert!(ok("x = b'hello'")); }
#[test] fn test_assign_none() { assert!(ok("x = None")); }
#[test] fn test_assign_true() { assert!(ok("x = True")); }
#[test] fn test_assign_false() { assert!(ok("x = False")); }
#[test] fn test_assign_list_empty() { assert!(ok("x = []")); }
#[test] fn test_assign_list_ints() { assert!(ok("x = [1, 2, 3]")); }
#[test] fn test_assign_list_mixed() { assert!(ok("x: list = [1, 'a']")); }
#[test] fn test_assign_dict_empty() { assert!(ok("x = {}")); }
#[test] fn test_assign_dict_simple() { assert!(ok("x = {'a': 1}")); }
#[test] fn test_assign_set_empty() { assert!(ok("x = set()")); }
#[test] fn test_assign_set_literal() { assert!(ok("x = {1, 2, 3}")); }
#[test] fn test_assign_tuple() { assert!(ok("x = (1, 2)")); }
#[test] fn test_assign_tuple_single() { assert!(ok("x = (1,)")); }

#[test] fn test_unpack_two() { assert!(ok("a, b = (1, 2)")); }
#[test] fn test_unpack_three() { assert!(ok("a, b, c = (1, 2, 3)")); }
#[test] fn test_unpack_nested() { assert!(ok("(a, b), c = ((1, 2), 3)")); }
#[test] fn test_unpack_starred_first() { assert!(ok("*a, b = [1, 2, 3]")); }
#[test] fn test_unpack_starred_last() { assert!(ok("a, *b = [1, 2, 3]")); }
#[test] fn test_unpack_starred_middle() { assert!(ok("a, *b, c = [1, 2, 3, 4]")); }

#[test] fn test_multi_assign() { assert!(ok("a = b = 1")); }
#[test] fn test_multi_assign_three() { assert!(ok("a = b = c = 1")); }

#[test] fn test_aug_add() { assert!(ok("def f():\n    x = 1\n    x += 1")); }
#[test] fn test_aug_sub() { assert!(ok("def f():\n    x = 1\n    x -= 1")); }
#[test] fn test_aug_mul() { assert!(ok("def f():\n    x = 1\n    x *= 2")); }
#[test] fn test_aug_div() { assert!(ok("def f():\n    x = 1.0\n    x /= 2")); }
#[test] fn test_aug_floordiv() { assert!(ok("def f():\n    x = 10\n    x //= 3")); }
#[test] fn test_aug_mod() { assert!(ok("def f():\n    x = 10\n    x %= 3")); }
#[test] fn test_aug_pow() { assert!(ok("def f():\n    x = 2\n    x **= 3")); }
#[test] fn test_aug_and() { assert!(ok("def f():\n    x = 0xFF\n    x &= 0x0F")); }
#[test] fn test_aug_or() { assert!(ok("def f():\n    x = 0x0F\n    x |= 0xF0")); }
#[test] fn test_aug_xor() { assert!(ok("def f():\n    x = 0xFF\n    x ^= 0x0F")); }
#[test] fn test_aug_lshift() { assert!(ok("def f():\n    x = 1\n    x <<= 2")); }
#[test] fn test_aug_rshift() { assert!(ok("def f():\n    x = 8\n    x >>= 2")); }

#[test] fn test_assign_index_list() { assert!(ok("def f():\n    x = [1, 2]\n    x[0] = 3")); }
#[test] fn test_assign_index_dict() { assert!(ok("def f():\n    x = {'a': 1}\n    x['b'] = 2")); }
#[test] fn test_assign_attr() { assert!(ok("class C:\n    def __init__(self):\n        self.x = 1\n    def set(self, v):\n        self.x = v")); }

// ============================================================================
// IF STATEMENT VARIATIONS (codegen_if_stmt)
// ============================================================================

#[test] fn test_if_simple() { assert!(ok("def f(x):\n    if x:\n        return 1")); }
#[test] fn test_if_else() { assert!(ok("def f(x):\n    if x:\n        return 1\n    else:\n        return 0")); }
#[test] fn test_if_elif() { assert!(ok("def f(x):\n    if x > 0:\n        return 1\n    elif x < 0:\n        return -1\n    else:\n        return 0")); }
#[test] fn test_if_elif_chain() { assert!(ok("def f(x):\n    if x == 1:\n        return 'a'\n    elif x == 2:\n        return 'b'\n    elif x == 3:\n        return 'c'\n    else:\n        return 'd'")); }
#[test] fn test_if_nested() { assert!(ok("def f(x, y):\n    if x:\n        if y:\n            return 1\n    return 0")); }
#[test] fn test_if_and() { assert!(ok("def f(x, y):\n    if x and y:\n        return 1")); }
#[test] fn test_if_or() { assert!(ok("def f(x, y):\n    if x or y:\n        return 1")); }
#[test] fn test_if_not() { assert!(ok("def f(x):\n    if not x:\n        return 1")); }
#[test] fn test_if_compare() { assert!(ok("def f(x):\n    if x == 0:\n        return True")); }
#[test] fn test_if_compare_chain() { assert!(ok("def f(x):\n    if 0 < x < 10:\n        return True")); }
#[test] fn test_if_in() { assert!(ok("def f(x):\n    if x in [1, 2, 3]:\n        return True")); }
#[test] fn test_if_not_in() { assert!(ok("def f(x):\n    if x not in [1, 2, 3]:\n        return True")); }
#[test] fn test_if_is_none() { assert!(ok("def f(x):\n    if x is None:\n        return True")); }
#[test] fn test_if_is_not_none() { assert!(ok("def f(x):\n    if x is not None:\n        return True")); }
#[test] fn test_if_isinstance() { assert!(ok("def f(x):\n    if isinstance(x, int):\n        return True")); }
#[test] fn test_if_callable() { assert!(ok("def f(x):\n    if callable(x):\n        return True")); }
#[test] fn test_if_hasattr() { assert!(ok("def f(x):\n    if hasattr(x, 'name'):\n        return True")); }

// ============================================================================
// FOR LOOP VARIATIONS (codegen_for_stmt)
// ============================================================================

#[test] fn test_for_range() { assert!(ok("def f():\n    for i in range(10):\n        pass")); }
#[test] fn test_for_range_start() { assert!(ok("def f():\n    for i in range(5, 10):\n        pass")); }
#[test] fn test_for_range_step() { assert!(ok("def f():\n    for i in range(0, 10, 2):\n        pass")); }
#[test] fn test_for_range_neg() { assert!(ok("def f():\n    for i in range(10, 0, -1):\n        pass")); }
#[test] fn test_for_list() { assert!(ok("def f():\n    for x in [1, 2, 3]:\n        pass")); }
#[test] fn test_for_string() { assert!(ok("def f():\n    for c in 'hello':\n        pass")); }
#[test] fn test_for_dict_keys() { assert!(ok("def f():\n    d = {'a': 1}\n    for k in d:\n        pass")); }
#[test] fn test_for_dict_items() { assert!(ok("def f():\n    d = {'a': 1}\n    for k, v in d.items():\n        pass")); }
#[test] fn test_for_dict_values() { assert!(ok("def f():\n    d = {'a': 1}\n    for v in d.values():\n        pass")); }
#[test] fn test_for_enumerate() { assert!(ok("def f():\n    for i, x in enumerate([1, 2]):\n        pass")); }
#[test] fn test_for_enumerate_start() { assert!(ok("def f():\n    for i, x in enumerate([1, 2], 1):\n        pass")); }
#[test] fn test_for_zip() { assert!(ok("def f():\n    for a, b in zip([1], [2]):\n        pass")); }
#[test] fn test_for_zip_three() { assert!(ok("def f():\n    for a, b, c in zip([1], [2], [3]):\n        pass")); }
#[test] fn test_for_reversed() { assert!(ok("def f():\n    for x in reversed([1, 2]):\n        pass")); }
#[test] fn test_for_sorted() { assert!(ok("def f():\n    for x in sorted([3, 1, 2]):\n        pass")); }
#[test] fn test_for_filter() { assert!(ok("def f():\n    for x in filter(lambda x: x > 0, [1, -1]):\n        pass")); }
#[test] fn test_for_map() { assert!(ok("def f():\n    for x in map(lambda x: x * 2, [1, 2]):\n        pass")); }
#[test] fn test_for_break() { assert!(ok("def f():\n    for i in range(10):\n        if i == 5:\n            break")); }
#[test] fn test_for_continue() { assert!(ok("def f():\n    for i in range(10):\n        if i % 2 == 0:\n            continue")); }
#[test] fn test_for_else() { assert!(ok("def f():\n    for i in range(3):\n        pass\n    else:\n        return True")); }
#[test] fn test_for_nested() { assert!(ok("def f():\n    for i in range(3):\n        for j in range(3):\n            pass")); }

// ============================================================================
// WHILE LOOP VARIATIONS (codegen_while_stmt)
// ============================================================================

#[test] fn test_while_simple() { assert!(ok("def f():\n    x = 0\n    while x < 10:\n        x += 1")); }
#[test] fn test_while_true() { assert!(ok("def f():\n    while True:\n        break")); }
#[test] fn test_while_break() { assert!(ok("def f():\n    x = 0\n    while True:\n        x += 1\n        if x > 5:\n            break")); }
#[test] fn test_while_continue() { assert!(ok("def f():\n    x = 0\n    while x < 10:\n        x += 1\n        if x % 2 == 0:\n            continue")); }
#[test] fn test_while_else() { assert!(ok("def f():\n    x = 0\n    while x < 3:\n        x += 1\n    else:\n        return True")); }
#[test] fn test_while_nested() { assert!(ok("def f():\n    i = 0\n    while i < 3:\n        j = 0\n        while j < 3:\n            j += 1\n        i += 1")); }

// ============================================================================
// TRY/EXCEPT VARIATIONS (codegen_try_stmt)
// ============================================================================

#[test] fn test_try_except() { assert!(ok("def f():\n    try:\n        x = 1\n    except:\n        pass")); }
#[test] fn test_try_except_type() { assert!(ok("def f():\n    try:\n        x = 1\n    except ValueError:\n        pass")); }
#[test] fn test_try_except_as() { assert!(ok("def f():\n    try:\n        x = 1\n    except ValueError as e:\n        print(e)")); }
#[test] fn test_try_except_multi() { assert!(ok("def f():\n    try:\n        x = 1\n    except ValueError:\n        pass\n    except TypeError:\n        pass")); }
#[test] fn test_try_except_tuple() { assert!(ok("def f():\n    try:\n        x = 1\n    except (ValueError, TypeError):\n        pass")); }
#[test] fn test_try_else() { assert!(ok("def f():\n    try:\n        x = 1\n    except:\n        pass\n    else:\n        return x")); }
#[test] fn test_try_finally() { assert!(ok("def f():\n    try:\n        x = 1\n    finally:\n        print('done')")); }
#[test] fn test_try_except_finally() { assert!(ok("def f():\n    try:\n        x = 1\n    except:\n        pass\n    finally:\n        print('done')")); }
#[test] fn test_try_full() { assert!(ok("def f():\n    try:\n        x = 1\n    except ValueError:\n        pass\n    else:\n        return x\n    finally:\n        print('done')")); }

// ============================================================================
// WITH STATEMENT (codegen_with_stmt)
// ============================================================================

#[test] fn test_with_simple() { assert!(ok("def f():\n    with open('f') as f:\n        pass")); }
#[test] fn test_with_no_as() { assert!(ok("def f():\n    with open('f'):\n        pass")); }
#[test] fn test_with_multi() { assert!(ok("def f():\n    with open('a') as a, open('b') as b:\n        pass")); }

// ============================================================================
// MATCH STATEMENT
// ============================================================================

#[test] fn test_match_literal() { assert!(ok("def f(x):\n    match x:\n        case 1:\n            return 'one'\n        case 2:\n            return 'two'\n        case _:\n            return 'other'")); }
#[test] fn test_match_string() { assert!(ok("def f(x):\n    match x:\n        case 'a':\n            return 1\n        case 'b':\n            return 2\n        case _:\n            return 0")); }
#[test] fn test_match_tuple() { assert!(ok("def f(x):\n    match x:\n        case (a, b):\n            return a + b\n        case _:\n            return 0")); }
#[test] fn test_match_list() { assert!(ok("def f(x):\n    match x:\n        case [a, b]:\n            return a + b\n        case _:\n            return 0")); }
#[test] fn test_match_guard() { assert!(ok("def f(x):\n    match x:\n        case n if n > 0:\n            return 'positive'\n        case _:\n            return 'non-positive'")); }

// ============================================================================
// FUNCTION VARIATIONS
// ============================================================================

#[test] fn test_func_simple() { assert!(ok("def f():\n    pass")); }
#[test] fn test_func_return() { assert!(ok("def f():\n    return 1")); }
#[test] fn test_func_return_none() { assert!(ok("def f():\n    return None")); }
#[test] fn test_func_return_expr() { assert!(ok("def f():\n    return 1 + 2")); }
#[test] fn test_func_args() { assert!(ok("def f(a, b):\n    return a + b")); }
#[test] fn test_func_typed_args() { assert!(ok("def f(a: int, b: int) -> int:\n    return a + b")); }
#[test] fn test_func_default() { assert!(ok("def f(a, b=0):\n    return a + b")); }
#[test] fn test_func_kwargs_only() { assert!(ok("def f(*, a, b):\n    return a + b")); }
#[test] fn test_func_args_star() { assert!(ok("def f(*args):\n    return len(args)")); }
#[test] fn test_func_kwargs_star() { assert!(ok("def f(**kwargs):\n    return len(kwargs)")); }
#[test] fn test_func_docstring() { assert!(ok("def f():\n    '''Docstring'''\n    pass")); }
#[test] fn test_func_nested() { assert!(ok("def outer():\n    def inner():\n        return 1\n    return inner()")); }
#[test] fn test_func_closure() { assert!(ok("def outer(x):\n    def inner():\n        return x\n    return inner")); }
#[test] fn test_func_recursive() { assert!(ok("def fib(n):\n    if n <= 1:\n        return n\n    return fib(n - 1) + fib(n - 2)")); }

// ============================================================================
// ASYNC FUNCTIONS
// ============================================================================

#[test] fn test_async_func() { assert!(ok("async def f():\n    pass")); }
#[test] fn test_async_return() { assert!(ok("async def f():\n    return 1")); }
#[test] fn test_async_await() { assert!(ok("async def f():\n    async def g():\n        return 1\n    return await g()")); }

// ============================================================================
// GENERATORS
// ============================================================================

#[test] fn test_generator_simple() { assert!(ok("def gen():\n    yield 1")); }
#[test] fn test_generator_loop() { assert!(ok("def gen(n):\n    for i in range(n):\n        yield i")); }
#[test] fn test_generator_yield_from() { assert!(ok("def gen():\n    yield from [1, 2, 3]")); }
#[test] fn test_generator_with_return() { assert!(ok("def gen():\n    yield 1\n    return")); }

// ============================================================================
// CLASS VARIATIONS
// ============================================================================

#[test] fn test_class_simple() { assert!(ok("class C:\n    pass")); }
#[test] fn test_class_init() { assert!(ok("class C:\n    def __init__(self):\n        self.x = 0")); }
#[test] fn test_class_init_args() { assert!(ok("class C:\n    def __init__(self, x):\n        self.x = x")); }
#[test] fn test_class_method() { assert!(ok("class C:\n    def method(self):\n        return 1")); }
#[test] fn test_class_staticmethod() { assert!(ok("class C:\n    @staticmethod\n    def f():\n        return 1")); }
#[test] fn test_class_classmethod() { assert!(ok("class C:\n    @classmethod\n    def f(cls):\n        return cls()")); }
#[test] fn test_class_property() { assert!(ok("class C:\n    @property\n    def x(self):\n        return self._x")); }
#[test] fn test_class_dunder_str() { assert!(ok("class C:\n    def __str__(self):\n        return 'C'")); }
#[test] fn test_class_dunder_repr() { assert!(ok("class C:\n    def __repr__(self):\n        return 'C()'")); }
#[test] fn test_class_dunder_eq() { assert!(ok("class C:\n    def __eq__(self, other):\n        return True")); }
#[test] fn test_class_dunder_lt() { assert!(ok("class C:\n    def __lt__(self, other):\n        return True")); }
#[test] fn test_class_dunder_len() { assert!(ok("class C:\n    def __len__(self):\n        return 0")); }
#[test] fn test_class_dunder_iter() { assert!(ok("class C:\n    def __iter__(self):\n        return iter([])")); }
#[test] fn test_class_dunder_getitem() { assert!(ok("class C:\n    def __getitem__(self, key):\n        return None")); }
#[test] fn test_class_dunder_setitem() { assert!(ok("class C:\n    def __setitem__(self, key, val):\n        pass")); }
#[test] fn test_class_dunder_contains() { assert!(ok("class C:\n    def __contains__(self, item):\n        return False")); }
#[test] fn test_class_dunder_add() { assert!(ok("class C:\n    def __add__(self, other):\n        return self")); }
#[test] fn test_class_dunder_sub() { assert!(ok("class C:\n    def __sub__(self, other):\n        return self")); }
#[test] fn test_class_dunder_mul() { assert!(ok("class C:\n    def __mul__(self, other):\n        return self")); }
#[test] fn test_class_dunder_call() { assert!(ok("class C:\n    def __call__(self):\n        return 1")); }
#[test] fn test_class_dunder_enter_exit() { assert!(ok("class C:\n    def __enter__(self):\n        return self\n    def __exit__(self, *args):\n        pass")); }

// ============================================================================
// DATACLASS
// ============================================================================

#[test] fn test_dataclass_simple() { assert!(ok("from dataclasses import dataclass\n\n@dataclass\nclass C:\n    x: int")); }
#[test] fn test_dataclass_multi_fields() { assert!(ok("from dataclasses import dataclass\n\n@dataclass\nclass C:\n    x: int\n    y: str")); }
#[test] fn test_dataclass_default() { assert!(ok("from dataclasses import dataclass\n\n@dataclass\nclass C:\n    x: int = 0")); }
#[test] fn test_dataclass_frozen() { assert!(ok("from dataclasses import dataclass\n\n@dataclass(frozen=True)\nclass C:\n    x: int")); }
#[test] fn test_dataclass_order() { assert!(ok("from dataclasses import dataclass\n\n@dataclass(order=True)\nclass C:\n    x: int")); }

// ============================================================================
// EXPRESSION VARIATIONS
// ============================================================================

#[test] fn test_expr_add() { assert!(ok("x = 1 + 2")); }
#[test] fn test_expr_sub() { assert!(ok("x = 1 - 2")); }
#[test] fn test_expr_mul() { assert!(ok("x = 1 * 2")); }
#[test] fn test_expr_div() { assert!(ok("x = 1 / 2")); }
#[test] fn test_expr_floordiv() { assert!(ok("x = 1 // 2")); }
#[test] fn test_expr_mod() { assert!(ok("x = 1 % 2")); }
#[test] fn test_expr_pow() { assert!(ok("x = 2 ** 3")); }
#[test] fn test_expr_neg() { assert!(ok("x = -1")); }
#[test] fn test_expr_pos() { assert!(ok("x = +1")); }
#[test] fn test_expr_not() { assert!(ok("x = not True")); }
#[test] fn test_expr_bitnot() { assert!(ok("x = ~1")); }
#[test] fn test_expr_bitand() { assert!(ok("x = 1 & 2")); }
#[test] fn test_expr_bitor() { assert!(ok("x = 1 | 2")); }
#[test] fn test_expr_bitxor() { assert!(ok("x = 1 ^ 2")); }
#[test] fn test_expr_lshift() { assert!(ok("x = 1 << 2")); }
#[test] fn test_expr_rshift() { assert!(ok("x = 4 >> 2")); }
#[test] fn test_expr_and() { assert!(ok("x = True and False")); }
#[test] fn test_expr_or() { assert!(ok("x = True or False")); }
#[test] fn test_expr_eq() { assert!(ok("x = 1 == 2")); }
#[test] fn test_expr_ne() { assert!(ok("x = 1 != 2")); }
#[test] fn test_expr_lt() { assert!(ok("x = 1 < 2")); }
#[test] fn test_expr_le() { assert!(ok("x = 1 <= 2")); }
#[test] fn test_expr_gt() { assert!(ok("x = 1 > 2")); }
#[test] fn test_expr_ge() { assert!(ok("x = 1 >= 2")); }
#[test] fn test_expr_is() { assert!(ok("x = None is None")); }
#[test] fn test_expr_is_not() { assert!(ok("x = 1 is not None")); }
#[test] fn test_expr_in() { assert!(ok("x = 1 in [1, 2]")); }
#[test] fn test_expr_not_in() { assert!(ok("x = 3 not in [1, 2]")); }

#[test] fn test_expr_ternary() { assert!(ok("x = 1 if True else 2")); }
#[test] fn test_expr_walrus() { assert!(ok("def f():\n    if (n := 5) > 0:\n        return n")); }

#[test] fn test_expr_call() { assert!(ok("def f():\n    return len([1, 2])")); }
#[test] fn test_expr_call_kwarg() { assert!(ok("def f():\n    return print('hi', end='')")); }
#[test] fn test_expr_method_call() { assert!(ok("x = 'hello'.upper()")); }
#[test] fn test_expr_method_chain() { assert!(ok("x = 'hello'.strip().upper()")); }

#[test] fn test_expr_index_list() { assert!(ok("x = [1, 2][0]")); }
#[test] fn test_expr_index_dict() { assert!(ok("x = {'a': 1}['a']")); }
#[test] fn test_expr_index_neg() { assert!(ok("x = [1, 2][-1]")); }
#[test] fn test_expr_slice_start() { assert!(ok("x = [1, 2, 3][1:]")); }
#[test] fn test_expr_slice_end() { assert!(ok("x = [1, 2, 3][:2]")); }
#[test] fn test_expr_slice_both() { assert!(ok("x = [1, 2, 3][1:2]")); }
#[test] fn test_expr_slice_step() { assert!(ok("x = [1, 2, 3, 4][::2]")); }
#[test] fn test_expr_slice_neg_step() { assert!(ok("x = [1, 2, 3][::-1]")); }

#[test] fn test_expr_attr() { assert!(ok("class C:\n    x = 1\nc = C()\ny = c.x")); }

#[test] fn test_expr_fstring() { assert!(ok("x = f'hello {1}'")); }
#[test] fn test_expr_fstring_expr() { assert!(ok("x = f'value: {1 + 2}'")); }
#[test] fn test_expr_fstring_format() { assert!(ok("x = f'{3.14:.2f}'")); }

#[test] fn test_expr_lambda() { assert!(ok("f = lambda x: x + 1")); }
#[test] fn test_expr_lambda_multi() { assert!(ok("f = lambda x, y: x + y")); }
#[test] fn test_expr_lambda_default() { assert!(ok("f = lambda x, y=0: x + y")); }

// ============================================================================
// COMPREHENSIONS
// ============================================================================

#[test] fn test_listcomp_simple() { assert!(ok("x = [i for i in range(10)]")); }
#[test] fn test_listcomp_if() { assert!(ok("x = [i for i in range(10) if i % 2 == 0]")); }
#[test] fn test_listcomp_transform() { assert!(ok("x = [i * 2 for i in range(10)]")); }
#[test] fn test_listcomp_nested() { assert!(ok("x = [i + j for i in range(3) for j in range(3)]")); }

#[test] fn test_dictcomp_simple() { assert!(ok("x = {i: i * 2 for i in range(5)}")); }
#[test] fn test_dictcomp_if() { assert!(ok("x = {i: i for i in range(5) if i % 2 == 0}")); }

#[test] fn test_setcomp_simple() { assert!(ok("x = {i for i in range(5)}")); }
#[test] fn test_setcomp_if() { assert!(ok("x = {i for i in range(10) if i % 2 == 0}")); }

#[test] fn test_genexp_simple() { assert!(ok("x = (i for i in range(5))")); }
#[test] fn test_genexp_if() { assert!(ok("x = (i for i in range(10) if i > 5)")); }

// ============================================================================
// BUILTIN FUNCTIONS
// ============================================================================

#[test] fn test_builtin_len() { assert!(ok("x = len([1, 2, 3])")); }
#[test] fn test_builtin_abs() { assert!(ok("x = abs(-5)")); }
#[test] fn test_builtin_min() { assert!(ok("x = min(1, 2, 3)")); }
#[test] fn test_builtin_max() { assert!(ok("x = max(1, 2, 3)")); }
#[test] fn test_builtin_sum() { assert!(ok("x = sum([1, 2, 3])")); }
#[test] fn test_builtin_all() { assert!(ok("x = all([True, True])")); }
#[test] fn test_builtin_any() { assert!(ok("x = any([True, False])")); }
#[test] fn test_builtin_sorted() { assert!(ok("x = sorted([3, 1, 2])")); }
#[test] fn test_builtin_reversed() { assert!(ok("x = list(reversed([1, 2, 3]))")); }
#[test] fn test_builtin_enumerate() { assert!(ok("x = list(enumerate([1, 2]))")); }
#[test] fn test_builtin_zip() { assert!(ok("x = list(zip([1], [2]))")); }
#[test] fn test_builtin_map() { assert!(ok("x = list(map(lambda x: x * 2, [1, 2]))")); }
#[test] fn test_builtin_filter() { assert!(ok("x = list(filter(lambda x: x > 0, [-1, 1]))")); }
#[test] fn test_builtin_range() { assert!(ok("x = list(range(5))")); }
#[test] fn test_builtin_int() { assert!(ok("x = int('42')")); }
#[test] fn test_builtin_float() { assert!(ok("x = float('3.14')")); }
#[test] fn test_builtin_str() { assert!(ok("x = str(42)")); }
#[test] fn test_builtin_bool() { assert!(ok("x = bool(1)")); }
#[test] fn test_builtin_list_call() { assert!(ok("x = list('abc')")); }
#[test] fn test_builtin_dict_call() { assert!(ok("x = dict(a=1)")); }
#[test] fn test_builtin_set_call() { assert!(ok("x = set([1, 1, 2])")); }
#[test] fn test_builtin_tuple_call() { assert!(ok("x = tuple([1, 2])")); }
#[test] fn test_builtin_print() { assert!(ok("print('hello')")); }
#[test] fn test_builtin_input() { assert!(ok("def f():\n    x = input('prompt')")); }
#[test] fn test_builtin_open() { assert!(ok("def f():\n    f = open('file.txt')")); }
#[test] fn test_builtin_isinstance() { assert!(ok("x = isinstance(1, int)")); }
#[test] fn test_builtin_type() { assert!(ok("x = type(1)")); }
#[test] fn test_builtin_id() { assert!(ok("x = id(1)")); }
#[test] fn test_builtin_hash() { assert!(ok("x = hash('hello')")); }
#[test] fn test_builtin_repr() { assert!(ok("x = repr([1, 2])")); }
#[test] fn test_builtin_ord() { assert!(ok("x = ord('a')")); }
#[test] fn test_builtin_chr() { assert!(ok("x = chr(97)")); }
#[test] fn test_builtin_hex() { assert!(ok("x = hex(255)")); }
#[test] fn test_builtin_oct() { assert!(ok("x = oct(8)")); }
#[test] fn test_builtin_bin() { assert!(ok("x = bin(10)")); }
#[test] fn test_builtin_round() { assert!(ok("x = round(3.14, 1)")); }
#[test] fn test_builtin_pow() { assert!(ok("x = pow(2, 3)")); }
#[test] fn test_builtin_divmod() { assert!(ok("x = divmod(10, 3)")); }

// ============================================================================
// STRING METHODS
// ============================================================================

#[test] fn test_str_upper() { assert!(ok("x = 'hello'.upper()")); }
#[test] fn test_str_lower() { assert!(ok("x = 'HELLO'.lower()")); }
#[test] fn test_str_strip() { assert!(ok("x = ' hello '.strip()")); }
#[test] fn test_str_lstrip() { assert!(ok("x = ' hello'.lstrip()")); }
#[test] fn test_str_rstrip() { assert!(ok("x = 'hello '.rstrip()")); }
#[test] fn test_str_split() { assert!(ok("x = 'a,b,c'.split(',')")); }
#[test] fn test_str_rsplit() { assert!(ok("x = 'a,b,c'.rsplit(',')")); }
#[test] fn test_str_splitlines() { assert!(ok("x = 'a\\nb'.splitlines()")); }
#[test] fn test_str_join() { assert!(ok("x = ','.join(['a', 'b'])")); }
#[test] fn test_str_replace() { assert!(ok("x = 'hello'.replace('l', 'r')")); }
#[test] fn test_str_find() { assert!(ok("x = 'hello'.find('l')")); }
#[test] fn test_str_rfind() { assert!(ok("x = 'hello'.rfind('l')")); }
#[test] fn test_str_index() { assert!(ok("x = 'hello'.index('l')")); }
#[test] fn test_str_rindex() { assert!(ok("x = 'hello'.rindex('l')")); }
#[test] fn test_str_count() { assert!(ok("x = 'hello'.count('l')")); }
#[test] fn test_str_startswith() { assert!(ok("x = 'hello'.startswith('he')")); }
#[test] fn test_str_endswith() { assert!(ok("x = 'hello'.endswith('lo')")); }
#[test] fn test_str_isalpha() { assert!(ok("x = 'hello'.isalpha()")); }
#[test] fn test_str_isdigit() { assert!(ok("x = '123'.isdigit()")); }
#[test] fn test_str_isalnum() { assert!(ok("x = 'abc123'.isalnum()")); }
#[test] fn test_str_isspace() { assert!(ok("x = ' '.isspace()")); }
#[test] fn test_str_isupper() { assert!(ok("x = 'HELLO'.isupper()")); }
#[test] fn test_str_islower() { assert!(ok("x = 'hello'.islower()")); }
#[test] fn test_str_title() { assert!(ok("x = 'hello world'.title()")); }
#[test] fn test_str_capitalize() { assert!(ok("x = 'hello'.capitalize()")); }
#[test] fn test_str_swapcase() { assert!(ok("x = 'Hello'.swapcase()")); }
#[test] fn test_str_center() { assert!(ok("x = 'hi'.center(10)")); }
#[test] fn test_str_ljust() { assert!(ok("x = 'hi'.ljust(10)")); }
#[test] fn test_str_rjust() { assert!(ok("x = 'hi'.rjust(10)")); }
#[test] fn test_str_zfill() { assert!(ok("x = '42'.zfill(5)")); }
#[test] fn test_str_encode() { assert!(ok("x = 'hello'.encode()")); }
#[test] fn test_str_format() { assert!(ok("x = '{} {}'.format('a', 'b')")); }
#[test] fn test_str_partition() { assert!(ok("x = 'a:b:c'.partition(':')")); }
#[test] fn test_str_rpartition() { assert!(ok("x = 'a:b:c'.rpartition(':')")); }

// ============================================================================
// LIST METHODS
// ============================================================================

#[test] fn test_list_append() { assert!(ok("def f():\n    x = []\n    x.append(1)")); }
#[test] fn test_list_extend() { assert!(ok("def f():\n    x = [1]\n    x.extend([2, 3])")); }
#[test] fn test_list_insert() { assert!(ok("def f():\n    x = [1, 3]\n    x.insert(1, 2)")); }
#[test] fn test_list_remove() { assert!(ok("def f():\n    x = [1, 2]\n    x.remove(1)")); }
#[test] fn test_list_pop() { assert!(ok("def f():\n    x = [1, 2]\n    x.pop()")); }
#[test] fn test_list_pop_index() { assert!(ok("def f():\n    x = [1, 2]\n    x.pop(0)")); }
#[test] fn test_list_clear() { assert!(ok("def f():\n    x = [1, 2]\n    x.clear()")); }
#[test] fn test_list_index() { assert!(ok("x = [1, 2, 3].index(2)")); }
#[test] fn test_list_count() { assert!(ok("x = [1, 1, 2].count(1)")); }
#[test] fn test_list_sort() { assert!(ok("def f():\n    x = [3, 1, 2]\n    x.sort()")); }
#[test] fn test_list_reverse() { assert!(ok("def f():\n    x = [1, 2, 3]\n    x.reverse()")); }
#[test] fn test_list_copy() { assert!(ok("x = [1, 2].copy()")); }

// ============================================================================
// DICT METHODS
// ============================================================================

#[test] fn test_dict_get() { assert!(ok("x = {'a': 1}.get('a')")); }
#[test] fn test_dict_get_default() { assert!(ok("x = {'a': 1}.get('b', 0)")); }
#[test] fn test_dict_keys() { assert!(ok("x = list({'a': 1}.keys())")); }
#[test] fn test_dict_values() { assert!(ok("x = list({'a': 1}.values())")); }
#[test] fn test_dict_items() { assert!(ok("x = list({'a': 1}.items())")); }
#[test] fn test_dict_pop() { assert!(ok("def f():\n    d = {'a': 1}\n    d.pop('a')")); }
#[test] fn test_dict_pop_default() { assert!(ok("def f():\n    d = {'a': 1}\n    d.pop('b', 0)")); }
#[test] fn test_dict_update() { assert!(ok("def f():\n    d = {'a': 1}\n    d.update({'b': 2})")); }
#[test] fn test_dict_clear() { assert!(ok("def f():\n    d = {'a': 1}\n    d.clear()")); }
#[test] fn test_dict_setdefault() { assert!(ok("def f():\n    d = {}\n    d.setdefault('a', 1)")); }

// ============================================================================
// SET METHODS
// ============================================================================

#[test] fn test_set_add() { assert!(ok("def f():\n    s = set()\n    s.add(1)")); }
#[test] fn test_set_remove() { assert!(ok("def f():\n    s = {1, 2}\n    s.remove(1)")); }
#[test] fn test_set_discard() { assert!(ok("def f():\n    s = {1, 2}\n    s.discard(1)")); }
#[test] fn test_set_pop() { assert!(ok("def f():\n    s = {1}\n    s.pop()")); }
#[test] fn test_set_clear() { assert!(ok("def f():\n    s = {1, 2}\n    s.clear()")); }
#[test] fn test_set_union() { assert!(ok("x = {1, 2}.union({3})")); }
#[test] fn test_set_intersection() { assert!(ok("x = {1, 2}.intersection({2, 3})")); }
#[test] fn test_set_difference() { assert!(ok("x = {1, 2}.difference({2})")); }
#[test] fn test_set_symmetric_difference() { assert!(ok("x = {1, 2}.symmetric_difference({2, 3})")); }
#[test] fn test_set_issubset() { assert!(ok("x = {1}.issubset({1, 2})")); }
#[test] fn test_set_issuperset() { assert!(ok("x = {1, 2}.issuperset({1})")); }
#[test] fn test_set_isdisjoint() { assert!(ok("x = {1, 2}.isdisjoint({3})")); }

// ============================================================================
// SPECIAL STATEMENTS
// ============================================================================

#[test] fn test_pass() { assert!(ok("def f():\n    pass")); }
#[test] fn test_assert() { assert!(ok("assert True")); }
#[test] fn test_assert_msg() { assert!(ok("assert True, 'message'")); }
#[test] fn test_raise() { assert!(ok("def f():\n    raise ValueError()")); }
#[test] fn test_raise_from() { assert!(ok("def f():\n    try:\n        pass\n    except:\n        raise RuntimeError() from None")); }
#[test] fn test_del_var() { assert!(ok("def f():\n    x = 1\n    del x")); }
#[test] fn test_del_index() { assert!(ok("def f():\n    x = [1, 2]\n    del x[0]")); }
#[test] fn test_global() { assert!(ok("x = 0\ndef f():\n    global x\n    x = 1")); }
#[test] fn test_nonlocal() { assert!(ok("def outer():\n    x = 0\n    def inner():\n        nonlocal x\n        x = 1\n    inner()")); }

// ============================================================================
// TYPE HINTS
// ============================================================================

#[test] fn test_type_int() { assert!(ok("x: int = 1")); }
#[test] fn test_type_str() { assert!(ok("x: str = 'hello'")); }
#[test] fn test_type_float() { assert!(ok("x: float = 1.0")); }
#[test] fn test_type_bool() { assert!(ok("x: bool = True")); }
#[test] fn test_type_list() { assert!(ok("from typing import List\nx: List[int] = [1, 2]")); }
#[test] fn test_type_dict() { assert!(ok("from typing import Dict\nx: Dict[str, int] = {}")); }
#[test] fn test_type_set() { assert!(ok("from typing import Set\nx: Set[int] = {1, 2}")); }
#[test] fn test_type_tuple() { assert!(ok("from typing import Tuple\nx: Tuple[int, str] = (1, 'a')")); }
#[test] fn test_type_optional() { assert!(ok("from typing import Optional\nx: Optional[int] = None")); }
#[test] fn test_type_union() { assert!(ok("from typing import Union\nx: Union[int, str] = 1")); }
#[test] fn test_type_callable() { assert!(ok("from typing import Callable\nf: Callable[[int], int] = lambda x: x")); }
#[test] fn test_type_any() { assert!(ok("from typing import Any\nx: Any = 1")); }
#[test] fn test_type_generic_t() { assert!(ok("from typing import TypeVar, Generic\nT = TypeVar('T')\nclass Box(Generic[T]):\n    def __init__(self, val: T):\n        self.val = val")); }

// ============================================================================
// IMPORTS
// ============================================================================

#[test] fn test_import() { assert!(ok("import os")); }
#[test] fn test_import_as() { assert!(ok("import os as operating_system")); }
#[test] fn test_from_import() { assert!(ok("from os import path")); }
#[test] fn test_from_import_as() { assert!(ok("from os import path as p")); }
#[test] fn test_from_import_multi() { assert!(ok("from os import path, getcwd")); }

// ============================================================================
// PROTOCOL
// ============================================================================

#[test] fn test_protocol_simple() { assert!(ok("from typing import Protocol\n\nclass Drawable(Protocol):\n    def draw(self) -> None: ...")); }
#[test] fn test_protocol_method() { assert!(ok("from typing import Protocol\n\nclass Sizeable(Protocol):\n    def size(self) -> int: ...")); }
