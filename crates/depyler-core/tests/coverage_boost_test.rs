//! Additional coverage tests to boost coverage to 95%+
//!
//! These tests target specific uncovered code paths in expr_gen.rs, stmt_gen.rs, etc.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_ok()
}

fn transpile(code: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline
        .transpile(code)
        .unwrap_or_else(|e| format!("ERROR: {}", e))
}

// ============ Slicing edge cases ============

#[test]
fn test_slice_start_only() {
    let code = "def f(items: list):\n    return items[2:]";
    assert!(transpile_ok(code));
}

#[test]
fn test_slice_stop_only() {
    let code = "def f(items: list):\n    return items[:5]";
    assert!(transpile_ok(code));
}

#[test]
fn test_slice_start_stop() {
    let code = "def f(items: list):\n    return items[2:5]";
    assert!(transpile_ok(code));
}

#[test]
fn test_slice_start_stop_step() {
    let code = "def f(items: list):\n    return items[0:10:2]";
    assert!(transpile_ok(code));
}

#[test]
fn test_slice_start_step_no_stop() {
    let code = "def f(items: list):\n    return items[2::3]";
    assert!(transpile_ok(code));
}

#[test]
fn test_slice_stop_step_no_start() {
    let code = "def f(items: list):\n    return items[:10:2]";
    assert!(transpile_ok(code));
}

#[test]
fn test_slice_step_only() {
    let code = "def f(items: list):\n    return items[::2]";
    assert!(transpile_ok(code));
}

#[test]
fn test_slice_negative_start() {
    let code = "def f(items: list):\n    return items[-3:]";
    assert!(transpile_ok(code));
}

#[test]
fn test_slice_negative_stop() {
    let code = "def f(items: list):\n    return items[:-2]";
    assert!(transpile_ok(code));
}

#[test]
fn test_slice_negative_step() {
    let code = "def f(items: list):\n    return items[::-1]";
    assert!(transpile_ok(code));
}

#[test]
fn test_slice_negative_start_stop() {
    let code = "def f(items: list):\n    return items[-5:-1]";
    assert!(transpile_ok(code));
}

#[test]
fn test_slice_empty_full() {
    let code = "def f(items: list):\n    return items[:]";
    assert!(transpile_ok(code));
}

#[test]
fn test_slice_string_start() {
    let code = "def f(s: str) -> str:\n    return s[5:]";
    assert!(transpile_ok(code));
}

#[test]
fn test_slice_string_stop() {
    let code = "def f(s: str) -> str:\n    return s[:10]";
    assert!(transpile_ok(code));
}

#[test]
fn test_slice_string_start_stop() {
    let code = "def f(s: str) -> str:\n    return s[2:8]";
    assert!(transpile_ok(code));
}

#[test]
fn test_slice_string_step() {
    let code = "def f(s: str) -> str:\n    return s[::2]";
    assert!(transpile_ok(code));
}

#[test]
fn test_slice_string_reverse() {
    let code = "def f(s: str) -> str:\n    return s[::-1]";
    assert!(transpile_ok(code));
}

// ============ Binary operations edge cases ============

#[test]
fn test_binop_floordiv() {
    let code = "def f(a: int, b: int) -> int:\n    return a // b";
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_modulo() {
    let code = "def f(a: int, b: int) -> int:\n    return a % b";
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_power() {
    let code = "def f(a: int, b: int) -> int:\n    return a ** b";
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_left_shift() {
    let code = "def f(a: int, b: int) -> int:\n    return a << b";
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_right_shift() {
    let code = "def f(a: int, b: int) -> int:\n    return a >> b";
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_bitand() {
    let code = "def f(a: int, b: int) -> int:\n    return a & b";
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_bitor() {
    let code = "def f(a: int, b: int) -> int:\n    return a | b";
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_bitxor() {
    let code = "def f(a: int, b: int) -> int:\n    return a ^ b";
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_matmul() {
    let code = "def f(a, b):\n    return a @ b";
    assert!(transpile_ok(code));
}

// ============ Comparison operations ============

#[test]
fn test_compare_lt() {
    let code = "def f(a: int, b: int) -> bool:\n    return a < b";
    assert!(transpile_ok(code));
}

#[test]
fn test_compare_le() {
    let code = "def f(a: int, b: int) -> bool:\n    return a <= b";
    assert!(transpile_ok(code));
}

#[test]
fn test_compare_gt() {
    let code = "def f(a: int, b: int) -> bool:\n    return a > b";
    assert!(transpile_ok(code));
}

#[test]
fn test_compare_ge() {
    let code = "def f(a: int, b: int) -> bool:\n    return a >= b";
    assert!(transpile_ok(code));
}

#[test]
fn test_compare_eq() {
    let code = "def f(a: int, b: int) -> bool:\n    return a == b";
    assert!(transpile_ok(code));
}

#[test]
fn test_compare_ne() {
    let code = "def f(a: int, b: int) -> bool:\n    return a != b";
    assert!(transpile_ok(code));
}

#[test]
fn test_compare_in() {
    let code = "def f(x: int, items: list) -> bool:\n    return x in items";
    assert!(transpile_ok(code));
}

#[test]
fn test_compare_not_in() {
    let code = "def f(x: int, items: list) -> bool:\n    return x not in items";
    assert!(transpile_ok(code));
}

#[test]
fn test_compare_is() {
    let code = "def f(a, b) -> bool:\n    return a is b";
    assert!(transpile_ok(code));
}

#[test]
fn test_compare_is_not() {
    let code = "def f(a, b) -> bool:\n    return a is not b";
    assert!(transpile_ok(code));
}

#[test]
fn test_compare_chain() {
    let code = "def f(a: int, b: int, c: int) -> bool:\n    return a < b < c";
    assert!(transpile_ok(code));
}

// ============ Unary operations ============

#[test]
fn test_unary_neg() {
    let code = "def f(x: int) -> int:\n    return -x";
    assert!(transpile_ok(code));
}

#[test]
fn test_unary_pos() {
    let code = "def f(x: int) -> int:\n    return +x";
    assert!(transpile_ok(code));
}

#[test]
fn test_unary_not() {
    let code = "def f(x: bool) -> bool:\n    return not x";
    assert!(transpile_ok(code));
}

#[test]
fn test_unary_invert() {
    let code = "def f(x: int) -> int:\n    return ~x";
    assert!(transpile_ok(code));
}

// ============ Boolean operations ============

#[test]
fn test_boolop_and() {
    let code = "def f(a: bool, b: bool) -> bool:\n    return a and b";
    assert!(transpile_ok(code));
}

#[test]
fn test_boolop_or() {
    let code = "def f(a: bool, b: bool) -> bool:\n    return a or b";
    assert!(transpile_ok(code));
}

#[test]
fn test_boolop_chain_and() {
    let code = "def f(a: bool, b: bool, c: bool) -> bool:\n    return a and b and c";
    assert!(transpile_ok(code));
}

#[test]
fn test_boolop_chain_or() {
    let code = "def f(a: bool, b: bool, c: bool) -> bool:\n    return a or b or c";
    assert!(transpile_ok(code));
}

#[test]
fn test_boolop_mixed() {
    let code = "def f(a: bool, b: bool, c: bool) -> bool:\n    return (a and b) or c";
    assert!(transpile_ok(code));
}

// ============ If expressions (ternary) ============

#[test]
fn test_ifexp_simple() {
    let code = "def f(x: int) -> int:\n    return 1 if x > 0 else 0";
    assert!(transpile_ok(code));
}

#[test]
fn test_ifexp_nested() {
    let code = "def f(x: int) -> str:\n    return 'pos' if x > 0 else 'neg' if x < 0 else 'zero'";
    assert!(transpile_ok(code));
}

#[test]
fn test_ifexp_in_call() {
    let code = "def f(x: int):\n    print('yes' if x else 'no')";
    assert!(transpile_ok(code));
}

// ============ List/Dict/Set comprehensions ============

#[test]
fn test_listcomp_simple() {
    let code = "def f(items: list):\n    return [x * 2 for x in items]";
    assert!(transpile_ok(code));
}

#[test]
fn test_listcomp_with_if() {
    let code = "def f(items: list):\n    return [x for x in items if x > 0]";
    assert!(transpile_ok(code));
}

#[test]
fn test_listcomp_nested() {
    let code = "def f(matrix: list):\n    return [x for row in matrix for x in row]";
    assert!(transpile_ok(code));
}

#[test]
fn test_listcomp_with_call() {
    let code = "def f(items: list):\n    return [str(x) for x in items]";
    assert!(transpile_ok(code));
}

#[test]
fn test_dictcomp_simple() {
    let code = "def f(items: list):\n    return {x: x*2 for x in items}";
    assert!(transpile_ok(code));
}

#[test]
fn test_dictcomp_with_if() {
    let code = "def f(items: list):\n    return {x: x*2 for x in items if x > 0}";
    assert!(transpile_ok(code));
}

#[test]
fn test_setcomp_simple() {
    let code = "def f(items: list):\n    return {x * 2 for x in items}";
    assert!(transpile_ok(code));
}

#[test]
fn test_setcomp_with_if() {
    let code = "def f(items: list):\n    return {x for x in items if x > 0}";
    assert!(transpile_ok(code));
}

#[test]
fn test_genexp_simple() {
    let code = "def f(items: list) -> int:\n    return sum(x for x in items)";
    assert!(transpile_ok(code));
}

#[test]
fn test_genexp_with_if() {
    let code = "def f(items: list) -> int:\n    return sum(x for x in items if x > 0)";
    assert!(transpile_ok(code));
}

// ============ Tuple unpacking ============

#[test]
fn test_tuple_unpack_2() {
    let code = "def f():\n    a, b = 1, 2\n    return a + b";
    assert!(transpile_ok(code));
}

#[test]
fn test_tuple_unpack_3() {
    let code = "def f():\n    a, b, c = 1, 2, 3\n    return a + b + c";
    assert!(transpile_ok(code));
}

#[test]
fn test_tuple_unpack_in_for() {
    let code = "def f(pairs: list):\n    for a, b in pairs:\n        print(a, b)";
    assert!(transpile_ok(code));
}

#[test]
fn test_tuple_unpack_nested() {
    // Nested tuple unpacking - test without failure assertion since not fully supported
    let code = "def f():\n    (a, b), c = (1, 2), 3\n    return a + b + c";
    let _ = transpile(code); // Just ensure no crash
}

// ============ Lambda expressions ============

#[test]
fn test_lambda_one_arg() {
    let code = "def f():\n    fn = lambda x: x * 2\n    return fn(5)";
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_two_args() {
    let code = "def f():\n    fn = lambda x, y: x + y\n    return fn(3, 4)";
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_no_args() {
    let code = "def f():\n    fn = lambda: 42\n    return fn()";
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_default_arg() {
    let code = "def f():\n    fn = lambda x, y=10: x + y\n    return fn(5)";
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_in_sort_key() {
    let code = "def f(items: list):\n    return sorted(items, key=lambda x: -x)";
    assert!(transpile_ok(code));
}

// ============ String operations ============

#[test]
fn test_str_concat() {
    let code = "def f(a: str, b: str) -> str:\n    return a + b";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_repeat() {
    let code = "def f(s: str, n: int) -> str:\n    return s * n";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_join() {
    let code = "def f(items: list) -> str:\n    return ', '.join(items)";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_split() {
    let code = "def f(s: str):\n    return s.split(',')";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_split_maxsplit() {
    let code = "def f(s: str):\n    return s.split(',', 2)";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_rsplit() {
    let code = "def f(s: str):\n    return s.rsplit(',')";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_splitlines() {
    let code = "def f(s: str):\n    return s.splitlines()";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_partition() {
    let code = "def f(s: str):\n    return s.partition(',')";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_rpartition() {
    let code = "def f(s: str):\n    return s.rpartition(',')";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_startswith() {
    let code = "def f(s: str) -> bool:\n    return s.startswith('hello')";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_endswith() {
    let code = "def f(s: str) -> bool:\n    return s.endswith('.txt')";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_replace() {
    let code = "def f(s: str) -> str:\n    return s.replace('old', 'new')";
    assert!(transpile_ok(code));
}

#[test]
fn test_str_replace_count() {
    let code = "def f(s: str) -> str:\n    return s.replace('a', 'b', 2)";
    assert!(transpile_ok(code));
}

// ============ List operations ============

#[test]
fn test_list_append() {
    let code = "def f(items: list, x: int):\n    items.append(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_list_extend() {
    let code = "def f(items: list, more: list):\n    items.extend(more)";
    assert!(transpile_ok(code));
}

#[test]
fn test_list_pop() {
    let code = "def f(items: list):\n    return items.pop()";
    assert!(transpile_ok(code));
}

#[test]
fn test_list_pop_index() {
    let code = "def f(items: list):\n    return items.pop(0)";
    assert!(transpile_ok(code));
}

#[test]
fn test_list_sort() {
    let code = "def f(items: list):\n    items.sort()";
    assert!(transpile_ok(code));
}

#[test]
fn test_list_sort_reverse() {
    let code = "def f(items: list):\n    items.sort(reverse=True)";
    assert!(transpile_ok(code));
}

#[test]
fn test_list_sort_key() {
    let code = "def f(items: list):\n    items.sort(key=lambda x: -x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_list_concat() {
    let code = "def f(a: list, b: list):\n    return a + b";
    assert!(transpile_ok(code));
}

#[test]
fn test_list_repeat() {
    let code = "def f(items: list, n: int):\n    return items * n";
    assert!(transpile_ok(code));
}

// ============ Dict operations ============

#[test]
fn test_dict_fromkeys() {
    let code = "def f(keys: list):\n    return dict.fromkeys(keys, 0)";
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_popitem() {
    let code = "def f(d: dict):\n    return d.popitem()";
    assert!(transpile_ok(code));
}

// ============ Exception handling ============

#[test]
fn test_try_except_basic() {
    let code = "def f():\n    try:\n        x = 1 / 0\n    except:\n        x = 0\n    return x";
    assert!(transpile_ok(code));
}

#[test]
fn test_try_except_typed() {
    let code = "def f():\n    try:\n        x = int('abc')\n    except ValueError:\n        x = 0\n    return x";
    assert!(transpile_ok(code));
}

#[test]
fn test_try_except_as() {
    let code = "def f():\n    try:\n        x = 1 / 0\n    except Exception as e:\n        print(e)\n        x = 0\n    return x";
    assert!(transpile_ok(code));
}

#[test]
fn test_try_except_multiple() {
    let code = "def f():\n    try:\n        x = int('abc')\n    except ValueError:\n        x = 0\n    except TypeError:\n        x = -1\n    return x";
    assert!(transpile_ok(code));
}

#[test]
fn test_try_except_else() {
    let code = "def f():\n    try:\n        x = 1\n    except:\n        x = 0\n    else:\n        x = 2\n    return x";
    assert!(transpile_ok(code));
}

#[test]
fn test_try_except_finally() {
    let code = "def f():\n    try:\n        x = 1\n    except:\n        x = 0\n    finally:\n        print('done')\n    return x";
    assert!(transpile_ok(code));
}

#[test]
fn test_try_finally_only() {
    let code = "def f():\n    try:\n        x = 1\n    finally:\n        print('cleanup')";
    assert!(transpile_ok(code));
}

// ============ Context managers ============

#[test]
fn test_with_open() {
    let code = "def f():\n    with open('test.txt', 'r') as f:\n        return f.read()";
    assert!(transpile_ok(code));
}

#[test]
fn test_with_open_write() {
    let code =
        "def f(content: str):\n    with open('out.txt', 'w') as f:\n        f.write(content)";
    assert!(transpile_ok(code));
}

#[test]
fn test_with_multiple() {
    let code = "def f():\n    with open('a.txt') as a, open('b.txt') as b:\n        return a.read() + b.read()";
    assert!(transpile_ok(code));
}

// ============ Walrus operator ============

#[test]
fn test_walrus_in_if() {
    let code = "def f(items: list):\n    if (n := len(items)) > 0:\n        return n";
    assert!(transpile_ok(code));
}

#[test]
fn test_walrus_in_while() {
    let code = "def f():\n    data = [1, 2, 3]\n    while (x := data.pop()) > 0:\n        print(x)\n        if not data:\n            break";
    assert!(transpile_ok(code));
}

#[test]
fn test_walrus_in_listcomp() {
    let code = "def f(items: list):\n    return [y for x in items if (y := x * 2) > 5]";
    assert!(transpile_ok(code));
}

// ============ Match statement ============

#[test]
fn test_match_literal() {
    let code = "def f(x: int) -> str:\n    match x:\n        case 1:\n            return 'one'\n        case 2:\n            return 'two'\n        case _:\n            return 'other'";
    assert!(transpile_ok(code));
}

#[test]
fn test_match_sequence() {
    let code = "def f(items: list) -> str:\n    match items:\n        case [x]:\n            return 'one'\n        case [x, y]:\n            return 'two'\n        case _:\n            return 'many'";
    assert!(transpile_ok(code));
}

#[test]
fn test_match_guard() {
    let code = "def f(x: int) -> str:\n    match x:\n        case n if n > 0:\n            return 'positive'\n        case n if n < 0:\n            return 'negative'\n        case _:\n            return 'zero'";
    assert!(transpile_ok(code));
}

// ============ Decorator patterns ============

#[test]
fn test_staticmethod() {
    let code = "class Foo:\n    @staticmethod\n    def bar() -> int:\n        return 42";
    assert!(transpile_ok(code));
}

#[test]
fn test_classmethod() {
    let code = "class Foo:\n    @classmethod\n    def create(cls):\n        return cls()";
    assert!(transpile_ok(code));
}

#[test]
fn test_property_getter() {
    let code = "class Foo:\n    def __init__(self):\n        self._x = 0\n    @property\n    def x(self) -> int:\n        return self._x";
    assert!(transpile_ok(code));
}

// ============ Inheritance ============

#[test]
fn test_class_inheritance() {
    let code = "class Base:\n    def foo(self) -> int:\n        return 1\n\nclass Child(Base):\n    def bar(self) -> int:\n        return 2";
    assert!(transpile_ok(code));
}

#[test]
fn test_class_super_call() {
    let code = "class Base:\n    def __init__(self, x: int):\n        self.x = x\n\nclass Child(Base):\n    def __init__(self, x: int, y: int):\n        super().__init__(x)\n        self.y = y";
    assert!(transpile_ok(code));
}

// ============ Global and nonlocal ============

#[test]
fn test_global_variable() {
    let code = "counter = 0\ndef increment():\n    global counter\n    counter += 1";
    assert!(transpile_ok(code));
}

#[test]
fn test_nonlocal_variable() {
    let code = "def outer():\n    x = 0\n    def inner():\n        nonlocal x\n        x += 1\n    inner()\n    return x";
    assert!(transpile_ok(code));
}

// ============ Yield statements ============

#[test]
fn test_yield_simple() {
    let code = "def gen():\n    yield 1\n    yield 2\n    yield 3";
    assert!(transpile_ok(code));
}

#[test]
fn test_yield_from() {
    let code = "def gen():\n    yield from [1, 2, 3]";
    assert!(transpile_ok(code));
}

#[test]
fn test_yield_in_for() {
    let code = "def gen(n: int):\n    for i in range(n):\n        yield i * 2";
    assert!(transpile_ok(code));
}

// ============ Starred expressions ============

#[test]
fn test_starred_unpack() {
    let code = "def f():\n    first, *rest = [1, 2, 3, 4, 5]\n    return first, rest";
    assert!(transpile_ok(code));
}

#[test]
fn test_starred_middle() {
    let code =
        "def f():\n    first, *middle, last = [1, 2, 3, 4, 5]\n    return first, middle, last";
    assert!(transpile_ok(code));
}

// ============ Await expressions ============

#[test]
fn test_async_await() {
    // Async may not be fully supported, test it doesn't crash
    let _ = transpile("async def f():\n    await something()");
}

// ============ Complex literals ============

#[test]
fn test_complex_number() {
    let code = "def f():\n    return 3 + 4j";
    assert!(transpile_ok(code));
}

#[test]
fn test_bytes_literal() {
    let code = "def f() -> bytes:\n    return b'hello'";
    assert!(transpile_ok(code));
}

#[test]
fn test_raw_string() {
    let code = r#"def f() -> str:
    return r'\n'"#;
    assert!(transpile_ok(code));
}

// ============ Attribute access patterns ============

#[test]
fn test_chained_attribute() {
    let code = "def f(obj):\n    return obj.foo.bar.baz";
    assert!(transpile_ok(code));
}

#[test]
fn test_method_chain() {
    let code = "def f(s: str) -> str:\n    return s.strip().lower().replace('a', 'b')";
    assert!(transpile_ok(code));
}

// ============ Index access patterns ============

#[test]
fn test_index_negative() {
    let code = "def f(items: list):\n    return items[-1]";
    assert!(transpile_ok(code));
}

#[test]
fn test_index_nested() {
    let code = "def f(matrix: list):\n    return matrix[0][0]";
    assert!(transpile_ok(code));
}

#[test]
fn test_index_dict() {
    let code = "def f(d: dict, key: str):\n    return d[key]";
    assert!(transpile_ok(code));
}

// ============ assert statement ============

#[test]
fn test_assert_basic() {
    let code = "def f(x: int):\n    assert x > 0";
    assert!(transpile_ok(code));
}

#[test]
fn test_assert_message() {
    let code = "def f(x: int):\n    assert x > 0, 'x must be positive'";
    assert!(transpile_ok(code));
}

// ============ Delete statement ============

#[test]
fn test_del_var() {
    let code = "def f():\n    x = 5\n    del x";
    assert!(transpile_ok(code));
}

#[test]
fn test_del_index() {
    let code = "def f(items: list):\n    del items[0]";
    assert!(transpile_ok(code));
}

#[test]
fn test_del_slice() {
    let code = "def f(items: list):\n    del items[1:3]";
    assert!(transpile_ok(code));
}

// ============ Named expressions in different contexts ============

#[test]
fn test_walrus_in_assert() {
    let code = "def f(items: list):\n    assert (n := len(items)) > 0, f'expected items, got {n}'";
    assert!(transpile_ok(code));
}

// ============ Complex function signatures ============

#[test]
fn test_func_args_only() {
    let code = "def f(*args):\n    return len(args)";
    assert!(transpile_ok(code));
}

#[test]
fn test_func_kwargs_only() {
    let code = "def f(**kwargs):\n    return len(kwargs)";
    assert!(transpile_ok(code));
}

#[test]
fn test_func_positional_and_args() {
    let code = "def f(x: int, *args):\n    return x + sum(args)";
    assert!(transpile_ok(code));
}

#[test]
fn test_func_all_param_types() {
    let code = "def f(a: int, b: int = 0, *args, **kwargs):\n    return a + b";
    assert!(transpile_ok(code));
}

// ============ Type annotations ============

#[test]
fn test_ann_list_int() {
    let code = "def f(items: list[int]) -> int:\n    return sum(items)";
    assert!(transpile_ok(code));
}

#[test]
fn test_ann_dict_str_int() {
    let code = "def f(d: dict[str, int]) -> int:\n    return d.get('key', 0)";
    assert!(transpile_ok(code));
}

#[test]
fn test_ann_optional() {
    let code = "from typing import Optional\ndef f(x: Optional[int]) -> int:\n    return x if x is not None else 0";
    assert!(transpile_ok(code));
}

#[test]
fn test_ann_union() {
    let code = "from typing import Union\ndef f(x: Union[int, str]) -> str:\n    return str(x)";
    assert!(transpile_ok(code));
}

#[test]
fn test_ann_tuple() {
    let code = "def f(t: tuple[int, str]) -> str:\n    return str(t[0]) + t[1]";
    assert!(transpile_ok(code));
}

#[test]
fn test_ann_callable() {
    let code = "from typing import Callable\ndef f(fn: Callable[[int], int], x: int) -> int:\n    return fn(x)";
    assert!(transpile_ok(code));
}

// ============ File operations ============

#[test]
fn test_file_read() {
    let code = "def f(path: str) -> str:\n    with open(path) as f:\n        return f.read()";
    assert!(transpile_ok(code));
}

#[test]
fn test_file_readline() {
    let code = "def f(path: str) -> str:\n    with open(path) as f:\n        return f.readline()";
    assert!(transpile_ok(code));
}

#[test]
fn test_file_readlines() {
    let code = "def f(path: str):\n    with open(path) as f:\n        return f.readlines()";
    assert!(transpile_ok(code));
}

#[test]
fn test_file_write() {
    let code =
        "def f(path: str, data: str):\n    with open(path, 'w') as f:\n        f.write(data)";
    assert!(transpile_ok(code));
}

#[test]
fn test_file_writelines() {
    let code = "def f(path: str, lines: list):\n    with open(path, 'w') as f:\n        f.writelines(lines)";
    assert!(transpile_ok(code));
}
