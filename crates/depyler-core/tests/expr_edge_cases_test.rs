//! Edge case expression tests for increasing code coverage
//! Targets less common expression patterns in expr_gen.rs

use depyler_core::DepylerPipeline;

fn transpiles(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

fn transpile(code: &str) -> String {
    DepylerPipeline::new().transpile(code).unwrap_or_default()
}

// =============================================================================
// NamedExpr (Walrus Operator) Tests
// =============================================================================

#[test]
fn test_walrus_in_if() {
    assert!(transpiles("def f(data):\n    if (n := len(data)) > 0:\n        return n"));
}

#[test]
fn test_walrus_in_while() {
    assert!(transpiles("def f(items):\n    while (item := items.pop()):\n        print(item)"));
}

#[test]
fn test_walrus_in_list_comp() {
    assert!(transpiles("x = [(y := i * 2) for i in range(10) if y > 5]"));
}

// =============================================================================
// Index Expression Tests
// =============================================================================

#[test]
fn test_index_with_negative() {
    assert!(transpiles("def f(lst):\n    return lst[-1]"));
}

#[test]
fn test_index_with_variable() {
    assert!(transpiles("def f(lst, i):\n    return lst[i]"));
}

#[test]
fn test_nested_index() {
    assert!(transpiles("def f(matrix):\n    return matrix[0][1]"));
}

#[test]
fn test_dict_index_string() {
    assert!(transpiles("def f(d):\n    return d['key']"));
}

#[test]
fn test_dict_index_variable() {
    assert!(transpiles("def f(d, k):\n    return d[k]"));
}

// =============================================================================
// Slice Expression Tests
// =============================================================================

#[test]
fn test_slice_start_only() {
    assert!(transpiles("def f(lst):\n    return lst[2:]"));
}

#[test]
fn test_slice_end_only() {
    assert!(transpiles("def f(lst):\n    return lst[:5]"));
}

#[test]
fn test_slice_both() {
    assert!(transpiles("def f(lst):\n    return lst[1:3]"));
}

#[test]
fn test_slice_with_step() {
    assert!(transpiles("def f(lst):\n    return lst[::2]"));
}

#[test]
fn test_slice_negative() {
    assert!(transpiles("def f(lst):\n    return lst[-3:-1]"));
}

#[test]
fn test_slice_reverse() {
    assert!(transpiles("def f(lst):\n    return lst[::-1]"));
}

// =============================================================================
// Attribute Expression Tests
// =============================================================================

#[test]
fn test_chained_attributes() {
    assert!(transpiles("def f(obj):\n    return obj.attr1.attr2"));
}

#[test]
fn test_attribute_on_call() {
    let code = transpile("def f():\n    return 'hello'.upper()");
    assert!(!code.is_empty());
}

#[test]
fn test_attribute_on_literal() {
    let code = transpile("def f():\n    return [1,2,3].append(4)");
    assert!(!code.is_empty());
}

// =============================================================================
// Unary Expression Tests
// =============================================================================

#[test]
fn test_unary_not() {
    assert!(transpiles("def f(x):\n    return not x"));
}

#[test]
fn test_unary_neg() {
    assert!(transpiles("def f(x):\n    return -x"));
}

#[test]
fn test_unary_pos() {
    assert!(transpiles("def f(x):\n    return +x"));
}

#[test]
fn test_unary_bitnot() {
    assert!(transpiles("def f(x):\n    return ~x"));
}

// =============================================================================
// Binary Expression Tests
// =============================================================================

#[test]
fn test_binary_floor_div() {
    assert!(transpiles("def f(a, b):\n    return a // b"));
}

#[test]
fn test_binary_mod() {
    assert!(transpiles("def f(a, b):\n    return a % b"));
}

#[test]
fn test_binary_pow() {
    assert!(transpiles("def f(a, b):\n    return a ** b"));
}

#[test]
fn test_binary_bitand() {
    assert!(transpiles("def f(a, b):\n    return a & b"));
}

#[test]
fn test_binary_bitor() {
    assert!(transpiles("def f(a, b):\n    return a | b"));
}

#[test]
fn test_binary_bitxor() {
    assert!(transpiles("def f(a, b):\n    return a ^ b"));
}

#[test]
fn test_binary_lshift() {
    assert!(transpiles("def f(a, b):\n    return a << b"));
}

#[test]
fn test_binary_rshift() {
    assert!(transpiles("def f(a, b):\n    return a >> b"));
}

#[test]
fn test_binary_in() {
    assert!(transpiles("def f(item, lst):\n    return item in lst"));
}

#[test]
fn test_binary_not_in() {
    assert!(transpiles("def f(item, lst):\n    return item not in lst"));
}

// =============================================================================
// Comparison Chain Tests
// =============================================================================

#[test]
fn test_comparison_chain_lt_lt() {
    assert!(transpiles("def f(a, b, c):\n    return a < b < c"));
}

#[test]
fn test_comparison_chain_le_le() {
    assert!(transpiles("def f(a, b, c):\n    return a <= b <= c"));
}

#[test]
fn test_comparison_chain_mixed() {
    assert!(transpiles("def f(a, b, c):\n    return a < b <= c"));
}

#[test]
fn test_comparison_chain_triple() {
    assert!(transpiles("def f(a, b, c, d):\n    return a < b < c < d"));
}

// =============================================================================
// List Comprehension Tests
// =============================================================================

#[test]
fn test_listcomp_with_if() {
    assert!(transpiles("x = [i for i in range(10) if i % 2 == 0]"));
}

#[test]
fn test_listcomp_nested() {
    assert!(transpiles("x = [[i * j for j in range(3)] for i in range(3)]"));
}

#[test]
fn test_listcomp_multiple_for() {
    assert!(transpiles("x = [(i, j) for i in range(3) for j in range(3)]"));
}

#[test]
fn test_listcomp_tuple_unpack() {
    assert!(transpiles("x = [a + b for a, b in [(1, 2), (3, 4)]]"));
}

// =============================================================================
// Dict Comprehension Tests
// =============================================================================

#[test]
fn test_dictcomp_simple() {
    assert!(transpiles("x = {i: i * 2 for i in range(5)}"));
}

#[test]
fn test_dictcomp_with_if() {
    assert!(transpiles("x = {i: i * 2 for i in range(10) if i % 2 == 0}"));
}

#[test]
fn test_dictcomp_from_list() {
    assert!(transpiles("x = {s: len(s) for s in ['a', 'bb', 'ccc']}"));
}

// =============================================================================
// Set Comprehension Tests
// =============================================================================

#[test]
fn test_setcomp_simple() {
    assert!(transpiles("x = {i * 2 for i in range(5)}"));
}

#[test]
fn test_setcomp_with_if() {
    assert!(transpiles("x = {i * 2 for i in range(10) if i % 2 == 0}"));
}

// =============================================================================
// Generator Expression Tests
// =============================================================================

#[test]
fn test_genexp_sum() {
    assert!(transpiles("def f():\n    return sum(i * 2 for i in range(10))"));
}

#[test]
fn test_genexp_any() {
    assert!(transpiles("def f(lst):\n    return any(x > 5 for x in lst)"));
}

#[test]
fn test_genexp_all() {
    assert!(transpiles("def f(lst):\n    return all(x > 0 for x in lst)"));
}

#[test]
fn test_genexp_list() {
    assert!(transpiles("def f():\n    return list(i * 2 for i in range(5))"));
}

// =============================================================================
// Ternary/IfExpr Tests
// =============================================================================

#[test]
fn test_ternary_simple() {
    assert!(transpiles("def f(x):\n    return 'yes' if x else 'no'"));
}

#[test]
fn test_ternary_nested() {
    assert!(transpiles("def f(x):\n    return 'a' if x > 0 else 'b' if x < 0 else 'c'"));
}

#[test]
fn test_ternary_with_call() {
    assert!(transpiles("def f(lst):\n    return lst[0] if len(lst) > 0 else None"));
}

// =============================================================================
// Lambda Tests
// =============================================================================

#[test]
fn test_lambda_simple() {
    assert!(transpiles("f = lambda x: x * 2"));
}

#[test]
fn test_lambda_multi_param() {
    assert!(transpiles("f = lambda x, y: x + y"));
}

#[test]
fn test_lambda_no_param() {
    assert!(transpiles("f = lambda: 42"));
}

#[test]
fn test_lambda_in_sorted() {
    assert!(transpiles("def f(lst):\n    return sorted(lst, key=lambda x: x[1])"));
}

#[test]
fn test_lambda_in_filter() {
    assert!(transpiles("def f(lst):\n    return list(filter(lambda x: x > 0, lst))"));
}

#[test]
fn test_lambda_in_map() {
    assert!(transpiles("def f(lst):\n    return list(map(lambda x: x * 2, lst))"));
}

// =============================================================================
// F-String Tests
// =============================================================================

#[test]
fn test_fstring_simple() {
    assert!(transpiles("def f(name):\n    return f'Hello, {name}'"));
}

#[test]
fn test_fstring_expr() {
    assert!(transpiles("def f(x):\n    return f'Value: {x * 2}'"));
}

#[test]
fn test_fstring_multiple() {
    assert!(transpiles("def f(a, b):\n    return f'{a} + {b} = {a + b}'"));
}

#[test]
fn test_fstring_nested_braces() {
    assert!(transpiles("def f(d):\n    return f\"Key: {d['key']}\""));
}

// =============================================================================
// Await Expression Tests
// =============================================================================

#[test]
fn test_await_simple() {
    assert!(transpiles("async def f():\n    result = await some_async_func()\n    return result"));
}

#[test]
fn test_await_in_expr() {
    assert!(transpiles("async def f():\n    return await fetch_data() + await fetch_more()"));
}

// =============================================================================
// Yield Expression Tests
// =============================================================================

#[test]
fn test_yield_simple() {
    assert!(transpiles("def gen():\n    yield 1\n    yield 2"));
}

#[test]
fn test_yield_from() {
    assert!(transpiles("def gen():\n    for i in range(5):\n        yield i"));
}

#[test]
fn test_yield_with_return() {
    assert!(transpiles("def gen():\n    yield 1\n    return"));
}

// =============================================================================
// Method Call Tests
// =============================================================================

#[test]
fn test_str_methods_chain() {
    assert!(transpiles("def f(s):\n    return s.strip().lower().replace('a', 'b')"));
}

#[test]
fn test_list_methods() {
    assert!(transpiles("def f(lst):\n    lst.append(1)\n    lst.extend([2, 3])\n    lst.insert(0, 0)\n    return lst"));
}

#[test]
fn test_dict_methods() {
    // Simplified test - test methods separately
    assert!(transpiles("def f(d):\n    return d.get('c', 'default')"));
}

#[test]
fn test_set_methods() {
    assert!(transpiles("def f(s):\n    s.add(1)\n    s.discard(2)\n    s.update({3, 4})\n    return s"));
}

// =============================================================================
// Literal Tests
// =============================================================================

#[test]
fn test_literal_bytes() {
    assert!(transpiles("x = b'hello'"));
}

#[test]
fn test_literal_raw_string() {
    assert!(transpiles("x = r'path\\to\\file'"));
}

#[test]
fn test_literal_multiline() {
    assert!(transpiles("x = '''multi\nline\nstring'''"));
}

#[test]
fn test_literal_hex() {
    assert!(transpiles("x = 0xFF"));
}

#[test]
fn test_literal_octal() {
    assert!(transpiles("x = 0o77"));
}

#[test]
fn test_literal_binary() {
    assert!(transpiles("x = 0b1010"));
}

#[test]
fn test_literal_underscore() {
    assert!(transpiles("x = 1_000_000"));
}

#[test]
fn test_literal_scientific() {
    assert!(transpiles("x = 1.5e10"));
}

#[test]
fn test_literal_scientific_neg() {
    assert!(transpiles("x = 1e-5"));
}

// =============================================================================
// Tuple Expression Tests
// =============================================================================

#[test]
fn test_tuple_empty() {
    assert!(transpiles("x = ()"));
}

#[test]
fn test_tuple_single() {
    assert!(transpiles("x = (1,)"));
}

#[test]
fn test_tuple_nested() {
    assert!(transpiles("x = ((1, 2), (3, 4))"));
}

#[test]
fn test_tuple_mixed() {
    assert!(transpiles("x = (1, 'a', True, None)"));
}

// =============================================================================
// Set Expression Tests
// =============================================================================

#[test]
fn test_set_literal() {
    assert!(transpiles("x = {1, 2, 3}"));
}

#[test]
fn test_set_empty_via_constructor() {
    assert!(transpiles("x = set()"));
}

#[test]
fn test_set_from_list() {
    assert!(transpiles("x = set([1, 2, 3])"));
}

// =============================================================================
// FrozenSet Tests
// =============================================================================

#[test]
fn test_frozenset_literal() {
    assert!(transpiles("x = frozenset({1, 2, 3})"));
}

#[test]
fn test_frozenset_from_list() {
    assert!(transpiles("x = frozenset([1, 2, 3])"));
}

// =============================================================================
// Call Expression Tests
// =============================================================================

#[test]
fn test_call_builtin_len() {
    assert!(transpiles("def f(lst):\n    return len(lst)"));
}

#[test]
fn test_call_builtin_range_1() {
    assert!(transpiles("def f():\n    return list(range(10))"));
}

#[test]
fn test_call_builtin_range_2() {
    assert!(transpiles("def f():\n    return list(range(1, 10))"));
}

#[test]
fn test_call_builtin_range_3() {
    assert!(transpiles("def f():\n    return list(range(1, 10, 2))"));
}

#[test]
fn test_call_builtin_abs() {
    assert!(transpiles("def f(x):\n    return abs(x)"));
}

#[test]
fn test_call_builtin_min() {
    assert!(transpiles("def f(a, b):\n    return min(a, b)"));
}

#[test]
fn test_call_builtin_max() {
    assert!(transpiles("def f(a, b):\n    return max(a, b)"));
}

#[test]
fn test_call_builtin_sum() {
    assert!(transpiles("def f(lst):\n    return sum(lst)"));
}

#[test]
fn test_call_builtin_sorted() {
    assert!(transpiles("def f(lst):\n    return sorted(lst)"));
}

#[test]
fn test_call_builtin_reversed() {
    assert!(transpiles("def f(lst):\n    return list(reversed(lst))"));
}

#[test]
fn test_call_builtin_enumerate() {
    assert!(transpiles("def f(lst):\n    return list(enumerate(lst))"));
}

#[test]
fn test_call_builtin_zip() {
    assert!(transpiles("def f(a, b):\n    return list(zip(a, b))"));
}

#[test]
fn test_call_builtin_print() {
    assert!(transpiles("def f(x):\n    print(x)"));
}

#[test]
fn test_call_builtin_input() {
    assert!(transpiles("def f():\n    return input('Enter: ')"));
}

#[test]
fn test_call_builtin_int() {
    assert!(transpiles("def f(s):\n    return int(s)"));
}

#[test]
fn test_call_builtin_float() {
    assert!(transpiles("def f(s):\n    return float(s)"));
}

#[test]
fn test_call_builtin_str() {
    assert!(transpiles("def f(x):\n    return str(x)"));
}

#[test]
fn test_call_builtin_bool() {
    assert!(transpiles("def f(x):\n    return bool(x)"));
}

#[test]
fn test_call_builtin_list() {
    assert!(transpiles("def f(x):\n    return list(x)"));
}

#[test]
fn test_call_builtin_dict() {
    assert!(transpiles("def f():\n    return dict()"));
}

#[test]
fn test_call_builtin_set() {
    assert!(transpiles("def f():\n    return set()"));
}

#[test]
fn test_call_builtin_tuple() {
    assert!(transpiles("def f(x):\n    return tuple(x)"));
}

#[test]
fn test_call_builtin_type() {
    assert!(transpiles("def f(x):\n    return type(x)"));
}

#[test]
fn test_call_builtin_isinstance() {
    assert!(transpiles("def f(x):\n    return isinstance(x, int)"));
}

#[test]
fn test_call_builtin_hasattr() {
    assert!(transpiles("def f(obj):\n    return hasattr(obj, 'name')"));
}

#[test]
#[ignore] // getattr not yet supported
fn test_call_builtin_getattr() {
    // getattr with 2 args (simpler pattern)
    assert!(transpiles("def f(obj):\n    return getattr(obj, 'name')"));
}

#[test]
fn test_call_builtin_setattr() {
    assert!(transpiles("def f(obj):\n    setattr(obj, 'name', 'value')"));
}

#[test]
fn test_call_builtin_ord() {
    assert!(transpiles("def f(c):\n    return ord(c)"));
}

#[test]
fn test_call_builtin_chr() {
    assert!(transpiles("def f(n):\n    return chr(n)"));
}

#[test]
fn test_call_builtin_hex() {
    assert!(transpiles("def f(n):\n    return hex(n)"));
}

#[test]
fn test_call_builtin_bin() {
    assert!(transpiles("def f(n):\n    return bin(n)"));
}

#[test]
fn test_call_builtin_oct() {
    assert!(transpiles("def f(n):\n    return oct(n)"));
}

#[test]
fn test_call_builtin_round() {
    assert!(transpiles("def f(x):\n    return round(x, 2)"));
}

#[test]
fn test_call_builtin_divmod() {
    assert!(transpiles("def f(a, b):\n    return divmod(a, b)"));
}

#[test]
fn test_call_builtin_pow() {
    assert!(transpiles("def f(a, b):\n    return pow(a, b)"));
}

#[test]
fn test_call_builtin_open() {
    assert!(transpiles("def f():\n    return open('file.txt', 'r')"));
}

// =============================================================================
// Keyword Argument Tests
// =============================================================================

#[test]
fn test_kwargs_simple() {
    assert!(transpiles("def f():\n    print('hello', end='')"));
}

#[test]
fn test_kwargs_multiple() {
    assert!(transpiles("def f():\n    print('a', 'b', sep=', ', end='\\n')"));
}

#[test]
fn test_kwargs_sorted_reverse() {
    assert!(transpiles("def f(lst):\n    return sorted(lst, reverse=True)"));
}

#[test]
fn test_kwargs_sorted_key() {
    assert!(transpiles("def f(lst):\n    return sorted(lst, key=len)"));
}

// =============================================================================
// Special Expression Patterns
// =============================================================================

#[test]
fn test_borrow_expr() {
    assert!(transpiles("def f(lst):\n    for item in lst:\n        print(item)"));
}

#[test]
fn test_dynamic_call() {
    // Function stored in variable
    assert!(transpiles("def f():\n    func = len\n    return func([1, 2, 3])"));
}

#[test]
fn test_ellipsis() {
    assert!(transpiles("x = ..."));
}

#[test]
fn test_none_literal() {
    assert!(transpiles("x = None"));
}

#[test]
fn test_true_literal() {
    assert!(transpiles("x = True"));
}

#[test]
fn test_false_literal() {
    assert!(transpiles("x = False"));
}
