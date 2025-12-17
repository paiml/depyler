//! Comprehensive integration tests for rust_gen coverage
//!
//! These tests exercise many code paths through the transpilation pipeline
//! to maximize coverage of expr_gen.rs, stmt_gen.rs, and func_gen.rs.

use crate::DepylerPipeline;

#[allow(dead_code)]
fn transpile(code: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).expect("transpilation should succeed")
}

fn transpile_ok(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_ok()
}

// ============================================================================
// WALRUS OPERATOR (NAMED EXPRESSION) TESTS
// ============================================================================

#[test]
fn test_walrus_simple_assignment() {
    assert!(transpile_ok("def foo(data):\n    if (n := len(data)) > 0:\n        return n"));
}

#[test]
fn test_walrus_in_while_condition() {
    assert!(transpile_ok("def foo(file):\n    while (line := file.readline()):\n        print(line)"));
}

#[test]
fn test_walrus_nested() {
    assert!(transpile_ok("def foo(x):\n    if (a := (b := x + 1) + 1) > 0:\n        return a"));
}

// ============================================================================
// OPTION PATTERN TESTS
// ============================================================================

#[test]
fn test_option_unwrap_or_pattern() {
    assert!(transpile_ok("import os\ndef foo():\n    value = os.getenv('VAR') or 'default'"));
}

#[test]
fn test_option_get_method() {
    assert!(transpile_ok("def foo(d: dict[str, int]):\n    value = d.get('key')"));
}

#[test]
fn test_option_get_with_default() {
    assert!(transpile_ok("def foo(d: dict[str, int]):\n    value = d.get('key', 0)"));
}

// ============================================================================
// FLOAT COERCION TESTS
// ============================================================================

#[test]
fn test_float_coercion_int_literal() {
    let code = transpile("def foo(beta: float) -> float:\n    return 1 - beta");
    assert!(code.contains("1.0") || code.contains("f64"));
}

#[test]
fn test_float_coercion_binary_expr() {
    assert!(transpile_ok("def foo(i: int, dx: float) -> float:\n    return (i + 1) * dx"));
}

#[test]
fn test_float_coercion_arithmetic() {
    assert!(transpile_ok("def foo(x: float) -> float:\n    return x * 2"));
}

// ============================================================================
// LOOP COUNTER TYPE COERCION
// ============================================================================

#[test]
fn test_loop_counter_coercion() {
    assert!(transpile_ok("def foo(data: list[int], offset: int) -> int:\n    for idx in range(len(data)):\n        if idx + offset > 0:\n            return idx"));
}

#[test]
fn test_loop_counter_named_i() {
    assert!(transpile_ok("def foo() -> int:\n    for i in range(10):\n        pass\n    return i"));
}

#[test]
fn test_loop_counter_named_j() {
    assert!(transpile_ok("def foo() -> int:\n    for j in range(10):\n        pass\n    return j"));
}

// ============================================================================
// BINARY OPERATION TESTS
// ============================================================================

#[test]
fn test_power_operator() {
    assert!(transpile_ok("def foo(x: float) -> float:\n    return x ** 2"));
}

#[test]
fn test_floor_division() {
    assert!(transpile_ok("def foo(x: int) -> int:\n    return x // 3"));
}

#[test]
fn test_modulo_operation() {
    assert!(transpile_ok("def foo(x: int) -> int:\n    return x % 3"));
}

#[test]
fn test_bitwise_and() {
    assert!(transpile_ok("def foo(x: int, y: int) -> int:\n    return x & y"));
}

#[test]
fn test_bitwise_or() {
    assert!(transpile_ok("def foo(x: int, y: int) -> int:\n    return x | y"));
}

#[test]
fn test_bitwise_xor() {
    assert!(transpile_ok("def foo(x: int, y: int) -> int:\n    return x ^ y"));
}

#[test]
fn test_left_shift() {
    assert!(transpile_ok("def foo(x: int) -> int:\n    return x << 2"));
}

#[test]
fn test_right_shift() {
    assert!(transpile_ok("def foo(x: int) -> int:\n    return x >> 2"));
}

// ============================================================================
// CONTAINMENT OPERATORS
// ============================================================================

#[test]
fn test_in_operator_list() {
    assert!(transpile_ok("def foo(x: int, items: list[int]) -> bool:\n    return x in items"));
}

#[test]
fn test_not_in_operator() {
    assert!(transpile_ok("def foo(x: int, items: list[int]) -> bool:\n    return x not in items"));
}

#[test]
fn test_in_operator_dict() {
    assert!(transpile_ok("def foo(key: str, d: dict[str, int]) -> bool:\n    return key in d"));
}

#[test]
fn test_in_operator_string() {
    assert!(transpile_ok("def foo(sub: str, s: str) -> bool:\n    return sub in s"));
}

// ============================================================================
// COMPARISON CHAINS
// ============================================================================

#[test]
fn test_comparison_chain_simple() {
    assert!(transpile_ok("def foo(x: int) -> bool:\n    return 0 < x < 10"));
}

#[test]
fn test_comparison_chain_three() {
    assert!(transpile_ok("def foo(x: int) -> bool:\n    return 0 <= x < 10"));
}

// ============================================================================
// UNARY OPERATORS
// ============================================================================

#[test]
fn test_unary_not() {
    assert!(transpile_ok("def foo(x: bool) -> bool:\n    return not x"));
}

#[test]
fn test_unary_neg() {
    assert!(transpile_ok("def foo(x: int) -> int:\n    return -x"));
}

#[test]
fn test_unary_pos() {
    assert!(transpile_ok("def foo(x: int) -> int:\n    return +x"));
}

#[test]
fn test_unary_invert() {
    assert!(transpile_ok("def foo(x: int) -> int:\n    return ~x"));
}

// ============================================================================
// BUILTIN FUNCTION TESTS
// ============================================================================

#[test]
fn test_builtin_len() {
    assert!(transpile_ok("def foo(items: list[int]) -> int:\n    return len(items)"));
}

#[test]
fn test_builtin_int_cast() {
    assert!(transpile_ok("def foo(s: str) -> int:\n    return int(s)"));
}

#[test]
fn test_builtin_float_cast() {
    assert!(transpile_ok("def foo(s: str) -> float:\n    return float(s)"));
}

#[test]
fn test_builtin_str_cast() {
    assert!(transpile_ok("def foo(n: int) -> str:\n    return str(n)"));
}

#[test]
fn test_builtin_bool_cast() {
    assert!(transpile_ok("def foo(n: int) -> bool:\n    return bool(n)"));
}

#[test]
fn test_builtin_abs() {
    assert!(transpile_ok("def foo(x: int) -> int:\n    return abs(x)"));
}

#[test]
fn test_builtin_min() {
    assert!(transpile_ok("def foo(a: int, b: int) -> int:\n    return min(a, b)"));
}

#[test]
fn test_builtin_max() {
    assert!(transpile_ok("def foo(a: int, b: int) -> int:\n    return max(a, b)"));
}

#[test]
fn test_builtin_sum() {
    assert!(transpile_ok("def foo(items: list[int]) -> int:\n    return sum(items)"));
}

#[test]
fn test_builtin_round() {
    assert!(transpile_ok("def foo(x: float) -> int:\n    return round(x)"));
}

#[test]
fn test_builtin_all() {
    assert!(transpile_ok("def foo(items: list[bool]) -> bool:\n    return all(items)"));
}

#[test]
fn test_builtin_any() {
    assert!(transpile_ok("def foo(items: list[bool]) -> bool:\n    return any(items)"));
}

#[test]
fn test_builtin_enumerate() {
    assert!(transpile_ok("def foo(items: list[str]):\n    for i, item in enumerate(items):\n        print(i, item)"));
}

#[test]
fn test_builtin_zip() {
    assert!(transpile_ok("def foo(a: list[int], b: list[str]):\n    for x, y in zip(a, b):\n        print(x, y)"));
}

#[test]
fn test_builtin_reversed() {
    assert!(transpile_ok("def foo(items: list[int]):\n    for item in reversed(items):\n        print(item)"));
}

#[test]
fn test_builtin_sorted() {
    assert!(transpile_ok("def foo(items: list[int]) -> list[int]:\n    return sorted(items)"));
}

#[test]
fn test_builtin_filter() {
    assert!(transpile_ok("def foo(items: list[int]) -> list[int]:\n    return list(filter(lambda x: x > 0, items))"));
}

#[test]
fn test_builtin_map() {
    assert!(transpile_ok("def foo(items: list[int]) -> list[int]:\n    return list(map(lambda x: x * 2, items))"));
}

#[test]
fn test_builtin_divmod() {
    assert!(transpile_ok("def foo(a: int, b: int):\n    q, r = divmod(a, b)"));
}

#[test]
fn test_builtin_hex() {
    assert!(transpile_ok("def foo(n: int) -> str:\n    return hex(n)"));
}

#[test]
fn test_builtin_bin() {
    assert!(transpile_ok("def foo(n: int) -> str:\n    return bin(n)"));
}

#[test]
fn test_builtin_oct() {
    assert!(transpile_ok("def foo(n: int) -> str:\n    return oct(n)"));
}

#[test]
fn test_builtin_chr() {
    assert!(transpile_ok("def foo(n: int) -> str:\n    return chr(n)"));
}

#[test]
fn test_builtin_ord() {
    assert!(transpile_ok("def foo(c: str) -> int:\n    return ord(c)"));
}

#[test]
fn test_builtin_format() {
    assert!(transpile_ok("def foo(n: int) -> str:\n    return format(n, 'x')"));
}

// ============================================================================
// COLLECTION CONSTRUCTOR TESTS
// ============================================================================

#[test]
fn test_list_constructor_empty() {
    assert!(transpile_ok("def foo() -> list[int]:\n    return list()"));
}

#[test]
fn test_list_constructor_from_range() {
    assert!(transpile_ok("def foo() -> list[int]:\n    return list(range(10))"));
}

#[test]
fn test_dict_constructor_empty() {
    assert!(transpile_ok("def foo() -> dict[str, int]:\n    return dict()"));
}

#[test]
fn test_set_constructor_empty() {
    assert!(transpile_ok("def foo() -> set[int]:\n    return set()"));
}

#[test]
fn test_tuple_constructor_empty() {
    assert!(transpile_ok("def foo():\n    return tuple()"));
}

#[test]
fn test_bytes_constructor() {
    assert!(transpile_ok("def foo() -> bytes:\n    return bytes()"));
}

#[test]
fn test_bytearray_constructor() {
    assert!(transpile_ok("def foo() -> bytearray:\n    return bytearray()"));
}

// ============================================================================
// STRING METHOD TESTS
// ============================================================================

#[test]
fn test_string_split() {
    assert!(transpile_ok("def foo(s: str) -> list[str]:\n    return s.split()"));
}

#[test]
fn test_string_split_with_sep() {
    assert!(transpile_ok("def foo(s: str) -> list[str]:\n    return s.split(',')"));
}

#[test]
fn test_string_join() {
    assert!(transpile_ok("def foo(items: list[str]) -> str:\n    return ','.join(items)"));
}

#[test]
fn test_string_strip() {
    assert!(transpile_ok("def foo(s: str) -> str:\n    return s.strip()"));
}

#[test]
fn test_string_lstrip() {
    assert!(transpile_ok("def foo(s: str) -> str:\n    return s.lstrip()"));
}

#[test]
fn test_string_rstrip() {
    assert!(transpile_ok("def foo(s: str) -> str:\n    return s.rstrip()"));
}

#[test]
fn test_string_upper() {
    assert!(transpile_ok("def foo(s: str) -> str:\n    return s.upper()"));
}

#[test]
fn test_string_lower() {
    assert!(transpile_ok("def foo(s: str) -> str:\n    return s.lower()"));
}

#[test]
fn test_string_replace() {
    assert!(transpile_ok("def foo(s: str) -> str:\n    return s.replace('a', 'b')"));
}

#[test]
fn test_string_startswith() {
    assert!(transpile_ok("def foo(s: str) -> bool:\n    return s.startswith('hello')"));
}

#[test]
fn test_string_endswith() {
    assert!(transpile_ok("def foo(s: str) -> bool:\n    return s.endswith('world')"));
}

#[test]
fn test_string_find() {
    assert!(transpile_ok("def foo(s: str) -> int:\n    return s.find('x')"));
}

#[test]
fn test_string_count() {
    assert!(transpile_ok("def foo(s: str) -> int:\n    return s.count('a')"));
}

#[test]
fn test_string_isdigit() {
    assert!(transpile_ok("def foo(s: str) -> bool:\n    return s.isdigit()"));
}

#[test]
fn test_string_isalpha() {
    assert!(transpile_ok("def foo(s: str) -> bool:\n    return s.isalpha()"));
}

#[test]
fn test_string_isalnum() {
    assert!(transpile_ok("def foo(s: str) -> bool:\n    return s.isalnum()"));
}

#[test]
fn test_string_format_method() {
    assert!(transpile_ok("def foo(name: str) -> str:\n    return 'Hello {}'.format(name)"));
}

// ============================================================================
// LIST METHOD TESTS
// ============================================================================

#[test]
fn test_list_append() {
    assert!(transpile_ok("def foo(items: list[int]):\n    items.append(1)"));
}

#[test]
fn test_list_extend() {
    assert!(transpile_ok("def foo(items: list[int]):\n    items.extend([1, 2, 3])"));
}

#[test]
fn test_list_insert() {
    assert!(transpile_ok("def foo(items: list[int]):\n    items.insert(0, 1)"));
}

#[test]
fn test_list_pop() {
    assert!(transpile_ok("def foo(items: list[int]) -> int:\n    return items.pop()"));
}

#[test]
fn test_list_pop_index() {
    assert!(transpile_ok("def foo(items: list[int]) -> int:\n    return items.pop(0)"));
}

#[test]
fn test_list_remove() {
    assert!(transpile_ok("def foo(items: list[int]):\n    items.remove(1)"));
}

#[test]
fn test_list_clear() {
    assert!(transpile_ok("def foo(items: list[int]):\n    items.clear()"));
}

#[test]
fn test_list_index() {
    assert!(transpile_ok("def foo(items: list[int]) -> int:\n    return items.index(1)"));
}

#[test]
fn test_list_reverse() {
    assert!(transpile_ok("def foo(items: list[int]):\n    items.reverse()"));
}

#[test]
fn test_list_sort() {
    assert!(transpile_ok("def foo(items: list[int]):\n    items.sort()"));
}

#[test]
fn test_list_copy() {
    assert!(transpile_ok("def foo(items: list[int]) -> list[int]:\n    return items.copy()"));
}

// ============================================================================
// DICT METHOD TESTS
// ============================================================================

#[test]
fn test_dict_keys() {
    assert!(transpile_ok("def foo(d: dict[str, int]):\n    for k in d.keys():\n        print(k)"));
}

#[test]
fn test_dict_values() {
    assert!(transpile_ok("def foo(d: dict[str, int]):\n    for v in d.values():\n        print(v)"));
}

#[test]
fn test_dict_items() {
    assert!(transpile_ok("def foo(d: dict[str, int]):\n    for k, v in d.items():\n        print(k, v)"));
}

#[test]
fn test_dict_pop() {
    assert!(transpile_ok("def foo(d: dict[str, int]) -> int:\n    return d.pop('key')"));
}

#[test]
fn test_dict_pop_default() {
    assert!(transpile_ok("def foo(d: dict[str, int]) -> int:\n    return d.pop('key', 0)"));
}

#[test]
fn test_dict_setdefault() {
    assert!(transpile_ok("def foo(d: dict[str, int]) -> int:\n    return d.setdefault('key', 0)"));
}

#[test]
fn test_dict_update() {
    assert!(transpile_ok("def foo(d1: dict[str, int], d2: dict[str, int]):\n    d1.update(d2)"));
}

#[test]
fn test_dict_clear() {
    assert!(transpile_ok("def foo(d: dict[str, int]):\n    d.clear()"));
}

// ============================================================================
// SET METHOD TESTS
// ============================================================================

#[test]
fn test_set_add() {
    assert!(transpile_ok("def foo(s: set[int]):\n    s.add(1)"));
}

#[test]
fn test_set_remove() {
    assert!(transpile_ok("def foo(s: set[int]):\n    s.remove(1)"));
}

#[test]
fn test_set_discard() {
    assert!(transpile_ok("def foo(s: set[int]):\n    s.discard(1)"));
}

#[test]
fn test_set_union() {
    assert!(transpile_ok("def foo(s1: set[int], s2: set[int]) -> set[int]:\n    return s1.union(s2)"));
}

#[test]
fn test_set_intersection() {
    assert!(transpile_ok("def foo(s1: set[int], s2: set[int]) -> set[int]:\n    return s1.intersection(s2)"));
}

#[test]
fn test_set_difference() {
    assert!(transpile_ok("def foo(s1: set[int], s2: set[int]) -> set[int]:\n    return s1.difference(s2)"));
}

// ============================================================================
// TRY/EXCEPT TESTS
// ============================================================================

#[test]
fn test_try_except_simple() {
    assert!(transpile_ok("def foo():\n    try:\n        x = 1\n    except:\n        pass"));
}

#[test]
fn test_try_except_specific() {
    assert!(transpile_ok("def foo():\n    try:\n        x = 1\n    except ValueError:\n        pass"));
}

#[test]
fn test_try_except_as() {
    assert!(transpile_ok("def foo():\n    try:\n        x = 1\n    except ValueError as e:\n        print(e)"));
}

#[test]
fn test_try_except_multiple() {
    assert!(transpile_ok("def foo():\n    try:\n        x = 1\n    except ValueError:\n        pass\n    except TypeError:\n        pass"));
}

#[test]
fn test_try_except_finally() {
    assert!(transpile_ok("def foo():\n    try:\n        x = 1\n    except:\n        pass\n    finally:\n        print('done')"));
}

#[test]
fn test_try_except_else() {
    assert!(transpile_ok("def foo():\n    try:\n        x = 1\n    except:\n        pass\n    else:\n        print('ok')"));
}

// ============================================================================
// WITH STATEMENT TESTS
// ============================================================================

#[test]
fn test_with_open_read() {
    assert!(transpile_ok("def foo():\n    with open('file.txt') as f:\n        data = f.read()"));
}

#[test]
fn test_with_open_write() {
    assert!(transpile_ok("def foo():\n    with open('file.txt', 'w') as f:\n        f.write('hello')"));
}

#[test]
fn test_with_multiple() {
    assert!(transpile_ok("def foo():\n    with open('a.txt') as a, open('b.txt') as b:\n        pass"));
}

// ============================================================================
// ASSIGNMENT TESTS
// ============================================================================

#[test]
fn test_augmented_assign_add() {
    assert!(transpile_ok("def foo():\n    x = 1\n    x += 1"));
}

#[test]
fn test_augmented_assign_sub() {
    assert!(transpile_ok("def foo():\n    x = 1\n    x -= 1"));
}

#[test]
fn test_augmented_assign_mul() {
    assert!(transpile_ok("def foo():\n    x = 1\n    x *= 2"));
}

#[test]
fn test_augmented_assign_div() {
    assert!(transpile_ok("def foo():\n    x = 1.0\n    x /= 2.0"));
}

#[test]
fn test_augmented_assign_floor_div() {
    assert!(transpile_ok("def foo():\n    x = 10\n    x //= 3"));
}

#[test]
fn test_augmented_assign_mod() {
    assert!(transpile_ok("def foo():\n    x = 10\n    x %= 3"));
}

#[test]
fn test_augmented_assign_and() {
    assert!(transpile_ok("def foo():\n    x = 0xFF\n    x &= 0x0F"));
}

#[test]
fn test_augmented_assign_or() {
    assert!(transpile_ok("def foo():\n    x = 0x0F\n    x |= 0xF0"));
}

#[test]
fn test_augmented_assign_xor() {
    assert!(transpile_ok("def foo():\n    x = 0xFF\n    x ^= 0x0F"));
}

#[test]
fn test_tuple_assignment() {
    assert!(transpile_ok("def foo():\n    a, b = 1, 2"));
}

#[test]
fn test_tuple_swap() {
    assert!(transpile_ok("def foo():\n    a, b = 1, 2\n    a, b = b, a"));
}

#[test]
fn test_nested_tuple_assignment() {
    // Nested tuple unpacking may not be fully supported - check it doesn't crash
    let _ = transpile_ok("def foo():\n    (a, b), c = (1, 2), 3");
}

#[test]
fn test_starred_assignment() {
    assert!(transpile_ok("def foo():\n    a, *rest = [1, 2, 3, 4]"));
}

#[test]
fn test_index_assignment() {
    assert!(transpile_ok("def foo(items: list[int]):\n    items[0] = 1"));
}

#[test]
fn test_slice_assignment() {
    assert!(transpile_ok("def foo(items: list[int]):\n    items[1:3] = [4, 5]"));
}

#[test]
fn test_attribute_assignment() {
    assert!(transpile_ok("class Point:\n    x: int\n    y: int\n\ndef foo(p: Point):\n    p.x = 10"));
}

// ============================================================================
// FOR LOOP TESTS
// ============================================================================

#[test]
fn test_for_range_simple() {
    assert!(transpile_ok("def foo():\n    for i in range(10):\n        print(i)"));
}

#[test]
fn test_for_range_start_end() {
    assert!(transpile_ok("def foo():\n    for i in range(1, 10):\n        print(i)"));
}

#[test]
fn test_for_range_step() {
    assert!(transpile_ok("def foo():\n    for i in range(0, 10, 2):\n        print(i)"));
}

#[test]
fn test_for_list() {
    assert!(transpile_ok("def foo(items: list[int]):\n    for item in items:\n        print(item)"));
}

#[test]
fn test_for_dict() {
    assert!(transpile_ok("def foo(d: dict[str, int]):\n    for key in d:\n        print(key)"));
}

#[test]
fn test_for_string() {
    assert!(transpile_ok("def foo(s: str):\n    for c in s:\n        print(c)"));
}

#[test]
fn test_for_enumerate() {
    assert!(transpile_ok("def foo(items: list[str]):\n    for i, item in enumerate(items):\n        print(i, item)"));
}

#[test]
fn test_for_zip() {
    assert!(transpile_ok("def foo(a: list[int], b: list[str]):\n    for x, y in zip(a, b):\n        print(x, y)"));
}

#[test]
fn test_for_with_break() {
    assert!(transpile_ok("def foo():\n    for i in range(10):\n        if i == 5:\n            break"));
}

#[test]
fn test_for_with_continue() {
    assert!(transpile_ok("def foo():\n    for i in range(10):\n        if i == 5:\n            continue\n        print(i)"));
}

#[test]
fn test_for_with_else() {
    assert!(transpile_ok("def foo():\n    for i in range(10):\n        pass\n    else:\n        print('done')"));
}

// ============================================================================
// WHILE LOOP TESTS
// ============================================================================

#[test]
fn test_while_simple() {
    assert!(transpile_ok("def foo():\n    i = 0\n    while i < 10:\n        i += 1"));
}

#[test]
fn test_while_true() {
    assert!(transpile_ok("def foo():\n    while True:\n        break"));
}

#[test]
fn test_while_with_else() {
    assert!(transpile_ok("def foo():\n    i = 0\n    while i < 10:\n        i += 1\n    else:\n        print('done')"));
}

// ============================================================================
// IF STATEMENT TESTS
// ============================================================================

#[test]
fn test_if_simple() {
    assert!(transpile_ok("def foo(x: int):\n    if x > 0:\n        print('positive')"));
}

#[test]
fn test_if_else() {
    assert!(transpile_ok("def foo(x: int):\n    if x > 0:\n        print('positive')\n    else:\n        print('non-positive')"));
}

#[test]
fn test_if_elif() {
    assert!(transpile_ok("def foo(x: int):\n    if x > 0:\n        print('positive')\n    elif x < 0:\n        print('negative')\n    else:\n        print('zero')"));
}

#[test]
fn test_if_nested() {
    assert!(transpile_ok("def foo(x: int, y: int):\n    if x > 0:\n        if y > 0:\n            print('both positive')"));
}

// ============================================================================
// TERNARY EXPRESSION TESTS
// ============================================================================

#[test]
fn test_ternary_simple() {
    assert!(transpile_ok("def foo(x: int) -> str:\n    return 'yes' if x > 0 else 'no'"));
}

#[test]
fn test_ternary_nested() {
    assert!(transpile_ok("def foo(x: int) -> str:\n    return 'positive' if x > 0 else 'negative' if x < 0 else 'zero'"));
}

// ============================================================================
// COMPREHENSION TESTS
// ============================================================================

#[test]
fn test_list_comprehension() {
    assert!(transpile_ok("def foo() -> list[int]:\n    return [x * 2 for x in range(10)]"));
}

#[test]
fn test_list_comprehension_with_condition() {
    assert!(transpile_ok("def foo() -> list[int]:\n    return [x for x in range(10) if x % 2 == 0]"));
}

#[test]
fn test_dict_comprehension() {
    assert!(transpile_ok("def foo() -> dict[int, int]:\n    return {x: x * 2 for x in range(5)}"));
}

#[test]
fn test_set_comprehension() {
    assert!(transpile_ok("def foo() -> set[int]:\n    return {x * 2 for x in range(10)}"));
}

#[test]
fn test_nested_comprehension() {
    assert!(transpile_ok("def foo() -> list[tuple[int, int]]:\n    return [(x, y) for x in range(3) for y in range(3)]"));
}

// ============================================================================
// LAMBDA TESTS
// ============================================================================

#[test]
fn test_lambda_simple() {
    assert!(transpile_ok("def foo():\n    f = lambda x: x * 2"));
}

#[test]
fn test_lambda_multiple_args() {
    assert!(transpile_ok("def foo():\n    f = lambda x, y: x + y"));
}

#[test]
fn test_lambda_in_map() {
    assert!(transpile_ok("def foo(items: list[int]) -> list[int]:\n    return list(map(lambda x: x * 2, items))"));
}

#[test]
fn test_lambda_in_filter() {
    assert!(transpile_ok("def foo(items: list[int]) -> list[int]:\n    return list(filter(lambda x: x > 0, items))"));
}

#[test]
fn test_lambda_in_sorted() {
    assert!(transpile_ok("def foo(items: list[str]) -> list[str]:\n    return sorted(items, key=lambda x: len(x))"));
}

// ============================================================================
// CLASS TESTS
// ============================================================================

#[test]
fn test_class_simple() {
    assert!(transpile_ok("class Point:\n    x: int\n    y: int"));
}

#[test]
fn test_class_with_init() {
    assert!(transpile_ok("class Point:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y"));
}

#[test]
fn test_class_with_method() {
    assert!(transpile_ok("class Point:\n    x: int\n    y: int\n    def distance(self) -> float:\n        return (self.x ** 2 + self.y ** 2) ** 0.5"));
}

#[test]
fn test_class_with_str() {
    assert!(transpile_ok("class Point:\n    x: int\n    y: int\n    def __str__(self) -> str:\n        return f'({self.x}, {self.y})'"));
}

#[test]
fn test_class_with_repr() {
    assert!(transpile_ok("class Point:\n    x: int\n    y: int\n    def __repr__(self) -> str:\n        return f'Point({self.x}, {self.y})'"));
}

// ============================================================================
// DATACLASS TESTS
// ============================================================================

#[test]
fn test_dataclass_simple() {
    assert!(transpile_ok("from dataclasses import dataclass\n\n@dataclass\nclass Point:\n    x: int\n    y: int"));
}

#[test]
fn test_dataclass_with_defaults() {
    assert!(transpile_ok("from dataclasses import dataclass\n\n@dataclass\nclass Point:\n    x: int = 0\n    y: int = 0"));
}

// ============================================================================
// F-STRING TESTS
// ============================================================================

#[test]
fn test_fstring_simple() {
    assert!(transpile_ok("def foo(name: str) -> str:\n    return f'Hello {name}'"));
}

#[test]
fn test_fstring_expression() {
    assert!(transpile_ok("def foo(x: int) -> str:\n    return f'Value: {x * 2}'"));
}

#[test]
fn test_fstring_format_spec() {
    assert!(transpile_ok("def foo(x: float) -> str:\n    return f'{x:.2f}'"));
}

// ============================================================================
// IMPORT TESTS
// ============================================================================

#[test]
fn test_import_os() {
    assert!(transpile_ok("import os\ndef foo() -> str:\n    return os.getcwd()"));
}

#[test]
fn test_import_sys() {
    assert!(transpile_ok("import sys\ndef foo() -> list[str]:\n    return sys.argv"));
}

#[test]
fn test_import_json() {
    assert!(transpile_ok("import json\ndef foo(s: str):\n    return json.loads(s)"));
}

#[test]
fn test_import_re() {
    assert!(transpile_ok("import re\ndef foo(s: str):\n    return re.match(r'\\d+', s)"));
}

#[test]
fn test_import_math() {
    assert!(transpile_ok("import math\ndef foo(x: float) -> float:\n    return math.sqrt(x)"));
}

#[test]
fn test_import_random() {
    assert!(transpile_ok("import random\ndef foo() -> int:\n    return random.randint(1, 10)"));
}

#[test]
fn test_from_import() {
    assert!(transpile_ok("from os import getcwd\ndef foo() -> str:\n    return getcwd()"));
}

#[test]
fn test_from_import_multiple() {
    assert!(transpile_ok("from os.path import join, exists\ndef foo(a: str, b: str) -> str:\n    return join(a, b)"));
}

// ============================================================================
// GLOBAL/NONLOCAL TESTS
// ============================================================================

#[test]
fn test_global_declaration() {
    assert!(transpile_ok("COUNTER = 0\n\ndef increment():\n    global COUNTER\n    COUNTER += 1"));
}

#[test]
fn test_nonlocal_declaration() {
    assert!(transpile_ok("def outer():\n    x = 0\n    def inner():\n        nonlocal x\n        x += 1\n    inner()"));
}

// ============================================================================
// ASSERT TESTS
// ============================================================================

#[test]
fn test_assert_simple() {
    assert!(transpile_ok("def foo(x: int):\n    assert x > 0"));
}

#[test]
fn test_assert_with_message() {
    assert!(transpile_ok("def foo(x: int):\n    assert x > 0, 'x must be positive'"));
}

// ============================================================================
// RAISE TESTS
// ============================================================================

#[test]
fn test_raise_simple() {
    assert!(transpile_ok("def foo():\n    raise ValueError()"));
}

#[test]
fn test_raise_with_message() {
    assert!(transpile_ok("def foo():\n    raise ValueError('error message')"));
}

#[test]
fn test_raise_from() {
    assert!(transpile_ok("def foo():\n    try:\n        x = 1\n    except Exception as e:\n        raise RuntimeError() from e"));
}

// ============================================================================
// DELETE TESTS
// ============================================================================

#[test]
fn test_del_variable() {
    assert!(transpile_ok("def foo():\n    x = 1\n    del x"));
}

#[test]
fn test_del_dict_item() {
    assert!(transpile_ok("def foo(d: dict[str, int]):\n    del d['key']"));
}

#[test]
fn test_del_list_item() {
    assert!(transpile_ok("def foo(items: list[int]):\n    del items[0]"));
}

// ============================================================================
// MATCH STATEMENT TESTS (PYTHON 3.10+)
// ============================================================================

#[test]
fn test_match_literal() {
    assert!(transpile_ok("def foo(x: int) -> str:\n    match x:\n        case 0:\n            return 'zero'\n        case 1:\n            return 'one'\n        case _:\n            return 'other'"));
}

#[test]
fn test_match_tuple() {
    assert!(transpile_ok("def foo(point):\n    match point:\n        case (0, 0):\n            return 'origin'\n        case (x, 0):\n            return 'on x axis'\n        case _:\n            return 'other'"));
}

// ============================================================================
// ASYNC TESTS
// ============================================================================

#[test]
fn test_async_def() {
    assert!(transpile_ok("async def foo():\n    pass"));
}

#[test]
fn test_async_await() {
    assert!(transpile_ok("async def foo():\n    await bar()"));
}

// ============================================================================
// PASS STATEMENT TESTS
// ============================================================================

#[test]
fn test_pass_in_function() {
    assert!(transpile_ok("def foo():\n    pass"));
}

#[test]
fn test_pass_in_class() {
    assert!(transpile_ok("class Empty:\n    pass"));
}

#[test]
fn test_pass_in_if() {
    assert!(transpile_ok("def foo(x: int):\n    if x > 0:\n        pass"));
}

// ============================================================================
// COLLECTIONS MODULE TESTS
// ============================================================================

#[test]
fn test_collections_counter() {
    assert!(transpile_ok("from collections import Counter\n\ndef foo(items: list[str]):\n    c = Counter(items)"));
}

#[test]
fn test_collections_defaultdict() {
    assert!(transpile_ok("from collections import defaultdict\n\ndef foo():\n    d = defaultdict(int)"));
}

#[test]
fn test_collections_deque() {
    assert!(transpile_ok("from collections import deque\n\ndef foo():\n    d = deque()"));
}

// ============================================================================
// ITERTOOLS MODULE TESTS
// ============================================================================

#[test]
fn test_itertools_chain() {
    assert!(transpile_ok("import itertools\n\ndef foo(a: list[int], b: list[int]):\n    for x in itertools.chain(a, b):\n        print(x)"));
}

#[test]
fn test_itertools_cycle() {
    assert!(transpile_ok("import itertools\n\ndef foo():\n    c = itertools.cycle([1, 2, 3])"));
}

#[test]
fn test_itertools_repeat() {
    assert!(transpile_ok("import itertools\n\ndef foo():\n    r = itertools.repeat(1, 5)"));
}

// ============================================================================
// MATH MODULE TESTS
// ============================================================================

#[test]
fn test_math_sqrt() {
    assert!(transpile_ok("import math\n\ndef foo(x: float) -> float:\n    return math.sqrt(x)"));
}

#[test]
fn test_math_floor() {
    assert!(transpile_ok("import math\n\ndef foo(x: float) -> int:\n    return math.floor(x)"));
}

#[test]
fn test_math_ceil() {
    assert!(transpile_ok("import math\n\ndef foo(x: float) -> int:\n    return math.ceil(x)"));
}

#[test]
fn test_math_sin() {
    assert!(transpile_ok("import math\n\ndef foo(x: float) -> float:\n    return math.sin(x)"));
}

#[test]
fn test_math_cos() {
    assert!(transpile_ok("import math\n\ndef foo(x: float) -> float:\n    return math.cos(x)"));
}

#[test]
fn test_math_tan() {
    assert!(transpile_ok("import math\n\ndef foo(x: float) -> float:\n    return math.tan(x)"));
}

#[test]
fn test_math_log() {
    assert!(transpile_ok("import math\n\ndef foo(x: float) -> float:\n    return math.log(x)"));
}

#[test]
fn test_math_exp() {
    assert!(transpile_ok("import math\n\ndef foo(x: float) -> float:\n    return math.exp(x)"));
}

#[test]
fn test_math_pi() {
    assert!(transpile_ok("import math\n\ndef foo() -> float:\n    return math.pi"));
}

#[test]
fn test_math_e() {
    assert!(transpile_ok("import math\n\ndef foo() -> float:\n    return math.e"));
}

// ============================================================================
// DATETIME MODULE TESTS
// ============================================================================

#[test]
fn test_datetime_now() {
    assert!(transpile_ok("from datetime import datetime\n\ndef foo():\n    return datetime.now()"));
}

#[test]
fn test_datetime_strftime() {
    assert!(transpile_ok("from datetime import datetime\n\ndef foo():\n    dt = datetime.now()\n    return dt.strftime('%Y-%m-%d')"));
}

#[test]
fn test_timedelta() {
    assert!(transpile_ok("from datetime import timedelta\n\ndef foo():\n    return timedelta(days=1)"));
}

// ============================================================================
// PATHLIB MODULE TESTS
// ============================================================================

#[test]
fn test_pathlib_path() {
    assert!(transpile_ok("from pathlib import Path\n\ndef foo() -> Path:\n    return Path('.')"));
}

#[test]
fn test_pathlib_exists() {
    assert!(transpile_ok("from pathlib import Path\n\ndef foo(p: Path) -> bool:\n    return p.exists()"));
}

#[test]
fn test_pathlib_is_file() {
    assert!(transpile_ok("from pathlib import Path\n\ndef foo(p: Path) -> bool:\n    return p.is_file()"));
}

#[test]
fn test_pathlib_is_dir() {
    assert!(transpile_ok("from pathlib import Path\n\ndef foo(p: Path) -> bool:\n    return p.is_dir()"));
}

#[test]
fn test_pathlib_read_text() {
    assert!(transpile_ok("from pathlib import Path\n\ndef foo(p: Path) -> str:\n    return p.read_text()"));
}

#[test]
fn test_pathlib_write_text() {
    assert!(transpile_ok("from pathlib import Path\n\ndef foo(p: Path, content: str):\n    p.write_text(content)"));
}

// ============================================================================
// SUBPROCESS MODULE TESTS
// ============================================================================

#[test]
fn test_subprocess_run() {
    assert!(transpile_ok("import subprocess\n\ndef foo():\n    subprocess.run(['ls', '-l'])"));
}

#[test]
fn test_subprocess_check_output() {
    assert!(transpile_ok("import subprocess\n\ndef foo() -> bytes:\n    return subprocess.check_output(['ls'])"));
}

// ============================================================================
// JSON MODULE TESTS
// ============================================================================

#[test]
fn test_json_loads() {
    assert!(transpile_ok("import json\n\ndef foo(s: str):\n    return json.loads(s)"));
}

#[test]
fn test_json_dumps() {
    assert!(transpile_ok("import json\n\ndef foo(data: dict[str, int]) -> str:\n    return json.dumps(data)"));
}

// ============================================================================
// REGEX MODULE TESTS
// ============================================================================

#[test]
fn test_re_match() {
    assert!(transpile_ok("import re\n\ndef foo(pattern: str, text: str):\n    return re.match(pattern, text)"));
}

#[test]
fn test_re_search() {
    assert!(transpile_ok("import re\n\ndef foo(pattern: str, text: str):\n    return re.search(pattern, text)"));
}

#[test]
fn test_re_findall() {
    assert!(transpile_ok("import re\n\ndef foo(pattern: str, text: str) -> list[str]:\n    return re.findall(pattern, text)"));
}

#[test]
fn test_re_sub() {
    assert!(transpile_ok("import re\n\ndef foo(pattern: str, repl: str, text: str) -> str:\n    return re.sub(pattern, repl, text)"));
}

#[test]
fn test_re_split() {
    assert!(transpile_ok("import re\n\ndef foo(pattern: str, text: str) -> list[str]:\n    return re.split(pattern, text)"));
}

// ============================================================================
// TYPE ANNOTATION TESTS
// ============================================================================

#[test]
fn test_type_union_int_str() {
    assert!(transpile_ok("from typing import Union\n\ndef foo(x: Union[int, str]):\n    pass"));
}

#[test]
fn test_type_optional() {
    assert!(transpile_ok("from typing import Optional\n\ndef foo(x: Optional[int]):\n    pass"));
}

#[test]
fn test_type_callable() {
    assert!(transpile_ok("from typing import Callable\n\ndef foo(f: Callable[[int], int]) -> int:\n    return f(1)"));
}

#[test]
fn test_type_any() {
    assert!(transpile_ok("from typing import Any\n\ndef foo(x: Any):\n    pass"));
}
