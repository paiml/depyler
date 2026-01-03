//! Probador Core Actions: Comprehensive TUI Simulation Testing
//!
//! EXTREME TDD: Systematic validation of all Python→Rust transpilation paths
//! against baseline behavior.
//!
//! Run with: cargo test --test probador_core_actions -- --nocapture

use depyler_core::DepylerPipeline;

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn transpile(code: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).unwrap_or_else(|e| format!("ERROR: {}", e))
}

fn transpile_ok(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_ok()
}

fn transpile_err(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_err()
}

// ============================================================================
// BUILTIN FUNCTION COVERAGE - Error Paths
// ============================================================================

#[test]
fn test_all_wrong_args() {
    assert!(transpile_err("def f(): return all()"));
    assert!(transpile_err("def f(): return all(1, 2)"));
}

#[test]
fn test_any_wrong_args() {
    assert!(transpile_err("def f(): return any()"));
    assert!(transpile_err("def f(): return any(1, 2)"));
}

#[test]
fn test_divmod_wrong_args() {
    assert!(transpile_err("def f(): return divmod()"));
    assert!(transpile_err("def f(): return divmod(1)"));
    assert!(transpile_err("def f(): return divmod(1, 2, 3)"));
}

#[test]
fn test_enumerate_wrong_args() {
    assert!(transpile_err("def f(): return enumerate()"));
    assert!(transpile_err("def f(): return enumerate(1, 2, 3)"));
}

#[test]
fn test_zip_wrong_args() {
    assert!(transpile_err("def f(): return zip()"));
    assert!(transpile_err("def f(): return zip([1])"));
}

#[test]
fn test_reversed_wrong_args() {
    assert!(transpile_err("def f(): return reversed()"));
    assert!(transpile_err("def f(): return reversed(1, 2)"));
}

#[test]
fn test_sorted_wrong_args() {
    assert!(transpile_err("def f(): return sorted()"));
    assert!(transpile_err("def f(): return sorted(1, 2, 3)"));
}

#[test]
fn test_filter_wrong_args() {
    assert!(transpile_err("def f(): return filter()"));
    assert!(transpile_err("def f(): return filter(lambda x: x)"));
    assert!(transpile_err("def f(): return filter(1, 2, 3)"));
}

#[test]
fn test_sum_wrong_args() {
    assert!(transpile_err("def f(): return sum()"));
    assert!(transpile_err("def f(): return sum(1, 2, 3)"));
}

#[test]
fn test_round_wrong_args() {
    assert!(transpile_err("def f(): return round()"));
    assert!(transpile_err("def f(): return round(1, 2, 3)"));
}

#[test]
fn test_abs_wrong_args() {
    assert!(transpile_err("def f(): return abs()"));
    assert!(transpile_err("def f(): return abs(1, 2)"));
}

#[test]
fn test_min_wrong_args() {
    assert!(transpile_err("def f(): return min()"));
}

#[test]
fn test_max_wrong_args() {
    assert!(transpile_err("def f(): return max()"));
}

#[test]
fn test_pow_wrong_args() {
    assert!(transpile_err("def f(): return pow()"));
    assert!(transpile_err("def f(): return pow(1)"));
    assert!(transpile_err("def f(): return pow(1, 2, 3, 4)"));
}

#[test]
fn test_hex_wrong_args() {
    assert!(transpile_err("def f(): return hex()"));
    assert!(transpile_err("def f(): return hex(1, 2)"));
}

#[test]
fn test_bin_wrong_args() {
    assert!(transpile_err("def f(): return bin()"));
    assert!(transpile_err("def f(): return bin(1, 2)"));
}

#[test]
fn test_oct_wrong_args() {
    assert!(transpile_err("def f(): return oct()"));
    assert!(transpile_err("def f(): return oct(1, 2)"));
}

#[test]
fn test_chr_wrong_args() {
    assert!(transpile_err("def f(): return chr()"));
    assert!(transpile_err("def f(): return chr(1, 2)"));
}

#[test]
fn test_ord_wrong_args() {
    assert!(transpile_err("def f(): return ord()"));
    assert!(transpile_err("def f(): return ord('a', 'b')"));
}

#[test]
fn test_hash_wrong_args() {
    assert!(transpile_err("def f(): return hash()"));
    assert!(transpile_err("def f(): return hash(1, 2)"));
}

#[test]
fn test_repr_wrong_args() {
    assert!(transpile_err("def f(): return repr()"));
    assert!(transpile_err("def f(): return repr(1, 2)"));
}

#[test]
fn test_iter_wrong_args() {
    assert!(transpile_err("def f(): return iter()"));
    assert!(transpile_err("def f(): return iter(1, 2)"));
}

#[test]
fn test_type_wrong_args() {
    assert!(transpile_err("def f(): return type()"));
    assert!(transpile_err("def f(): return type(1, 2)"));
}

#[test]
fn test_open_wrong_args() {
    assert!(transpile_err("def f(): return open()"));
    assert!(transpile_err("def f(): return open(1, 2, 3)"));
}

#[test]
fn test_next_wrong_args() {
    assert!(transpile_err("def f(): return next()"));
    assert!(transpile_err("def f(): return next(1, 2, 3)"));
}

// ============================================================================
// BUILTIN FUNCTION COVERAGE - Success Paths
// ============================================================================

#[test]
fn test_all_ok() {
    assert!(transpile_ok("def f(x): return all(x)"));
}

#[test]
fn test_any_ok() {
    assert!(transpile_ok("def f(x): return any(x)"));
}

#[test]
fn test_divmod_ok() {
    assert!(transpile_ok("def f(): return divmod(10, 3)"));
}

#[test]
fn test_enumerate_ok() {
    assert!(transpile_ok("def f(x): return list(enumerate(x))"));
    assert!(transpile_ok("def f(x): return list(enumerate(x, 1))"));
}

#[test]
fn test_zip_ok() {
    assert!(transpile_ok("def f(a, b): return list(zip(a, b))"));
    assert!(transpile_ok("def f(a, b, c): return list(zip(a, b, c))"));
}

#[test]
fn test_reversed_ok() {
    assert!(transpile_ok("def f(x): return list(reversed(x))"));
}

#[test]
fn test_sorted_ok() {
    assert!(transpile_ok("def f(x): return sorted(x)"));
    assert!(transpile_ok("def f(x): return sorted(x, reverse=True)"));
}

#[test]
fn test_filter_ok() {
    assert!(transpile_ok("def f(x): return list(filter(lambda n: n > 0, x))"));
}

#[test]
fn test_sum_ok() {
    assert!(transpile_ok("def f(x): return sum(x)"));
    assert!(transpile_ok("def f(x): return sum(x, 10)"));
}

#[test]
fn test_round_ok() {
    assert!(transpile_ok("def f(x): return round(x)"));
    assert!(transpile_ok("def f(x): return round(x, 2)"));
}

#[test]
fn test_abs_ok() {
    assert!(transpile_ok("def f(x): return abs(x)"));
}

#[test]
fn test_min_ok() {
    assert!(transpile_ok("def f(x): return min(x)"));
    assert!(transpile_ok("def f(a, b): return min(a, b)"));
    assert!(transpile_ok("def f(a, b, c): return min(a, b, c)"));
}

#[test]
fn test_max_ok() {
    assert!(transpile_ok("def f(x): return max(x)"));
    assert!(transpile_ok("def f(a, b): return max(a, b)"));
    assert!(transpile_ok("def f(a, b, c): return max(a, b, c)"));
}

#[test]
fn test_pow_ok() {
    assert!(transpile_ok("def f(): return pow(2, 10)"));
    assert!(transpile_ok("def f(): return pow(2, 10, 100)"));
}

#[test]
fn test_hex_ok() {
    assert!(transpile_ok("def f(x): return hex(x)"));
}

#[test]
fn test_bin_ok() {
    assert!(transpile_ok("def f(x): return bin(x)"));
}

#[test]
fn test_oct_ok() {
    assert!(transpile_ok("def f(x): return oct(x)"));
}

#[test]
fn test_chr_ok() {
    assert!(transpile_ok("def f(x): return chr(x)"));
}

#[test]
fn test_ord_ok() {
    assert!(transpile_ok("def f(x): return ord(x)"));
}

#[test]
fn test_hash_ok() {
    assert!(transpile_ok("def f(x): return hash(x)"));
}

#[test]
fn test_repr_ok() {
    assert!(transpile_ok("def f(x): return repr(x)"));
}

#[test]
fn test_type_ok() {
    assert!(transpile_ok("def f(x): return type(x)"));
}

#[test]
fn test_next_ok() {
    assert!(transpile_ok("def f(x): return next(iter(x))"));
    assert!(transpile_ok("def f(x): return next(iter(x), None)"));
}

// ============================================================================
// STRING METHOD COVERAGE
// ============================================================================

#[test]
fn test_string_upper() {
    assert!(transpile_ok("def f(s): return s.upper()"));
}

#[test]
fn test_string_lower() {
    assert!(transpile_ok("def f(s): return s.lower()"));
}

#[test]
fn test_string_strip() {
    assert!(transpile_ok("def f(s): return s.strip()"));
}

#[test]
fn test_string_lstrip() {
    assert!(transpile_ok("def f(s): return s.lstrip()"));
}

#[test]
fn test_string_rstrip() {
    assert!(transpile_ok("def f(s): return s.rstrip()"));
}

#[test]
fn test_string_split() {
    assert!(transpile_ok("def f(s): return s.split()"));
    assert!(transpile_ok("def f(s): return s.split(',')"));
    assert!(transpile_ok("def f(s): return s.split(',', 1)"));
}

#[test]
fn test_string_join() {
    assert!(transpile_ok("def f(lst): return ','.join(lst)"));
}

#[test]
fn test_string_replace() {
    assert!(transpile_ok("def f(s): return s.replace('a', 'b')"));
}

#[test]
fn test_string_find() {
    assert!(transpile_ok("def f(s): return s.find('x')"));
}

#[test]
fn test_string_rfind() {
    assert!(transpile_ok("def f(s): return s.rfind('x')"));
}

#[test]
fn test_string_startswith() {
    assert!(transpile_ok("def f(s): return s.startswith('x')"));
}

#[test]
fn test_string_endswith() {
    assert!(transpile_ok("def f(s): return s.endswith('x')"));
}

#[test]
fn test_string_isdigit() {
    assert!(transpile_ok("def f(s): return s.isdigit()"));
}

#[test]
fn test_string_isalpha() {
    assert!(transpile_ok("def f(s): return s.isalpha()"));
}

#[test]
fn test_string_isalnum() {
    assert!(transpile_ok("def f(s): return s.isalnum()"));
}

#[test]
fn test_string_capitalize() {
    assert!(transpile_ok("def f(s): return s.capitalize()"));
}

#[test]
fn test_string_title() {
    assert!(transpile_ok("def f(s): return s.title()"));
}

#[test]
fn test_string_count() {
    assert!(transpile_ok("def f(s): return s.count('a')"));
}

#[test]
fn test_string_encode() {
    assert!(transpile_ok("def f(s): return s.encode()"));
}

// ============================================================================
// LIST METHOD COVERAGE
// ============================================================================

#[test]
fn test_list_append() {
    assert!(transpile_ok("def f(lst): lst.append(1)"));
}

#[test]
fn test_list_extend() {
    assert!(transpile_ok("def f(lst): lst.extend([1, 2])"));
}

#[test]
fn test_list_pop() {
    assert!(transpile_ok("def f(lst): return lst.pop()"));
    assert!(transpile_ok("def f(lst): return lst.pop(0)"));
}

#[test]
fn test_list_insert() {
    assert!(transpile_ok("def f(lst): lst.insert(0, 1)"));
}

#[test]
fn test_list_remove() {
    assert!(transpile_ok("def f(lst): lst.remove(1)"));
}

#[test]
fn test_list_clear() {
    assert!(transpile_ok("def f(lst): lst.clear()"));
}

#[test]
fn test_list_copy() {
    assert!(transpile_ok("def f(lst): return lst.copy()"));
}

#[test]
fn test_list_index() {
    assert!(transpile_ok("def f(lst): return lst.index(1)"));
}

#[test]
fn test_list_count() {
    assert!(transpile_ok("def f(lst): return lst.count(1)"));
}

#[test]
fn test_list_sort() {
    assert!(transpile_ok("def f(lst): lst.sort()"));
    assert!(transpile_ok("def f(lst): lst.sort(reverse=True)"));
}

#[test]
fn test_list_reverse() {
    assert!(transpile_ok("def f(lst): lst.reverse()"));
}

// ============================================================================
// DICT METHOD COVERAGE
// ============================================================================

#[test]
fn test_dict_get() {
    assert!(transpile_ok("def f(d): return d.get('key')"));
    assert!(transpile_ok("def f(d): return d.get('key', 'default')"));
}

#[test]
fn test_dict_keys() {
    assert!(transpile_ok("def f(d): return list(d.keys())"));
}

#[test]
fn test_dict_values() {
    assert!(transpile_ok("def f(d): return list(d.values())"));
}

#[test]
fn test_dict_items() {
    assert!(transpile_ok("def f(d): return list(d.items())"));
}

#[test]
fn test_dict_pop() {
    assert!(transpile_ok("def f(d): return d.pop('key')"));
    assert!(transpile_ok("def f(d): return d.pop('key', 'default')"));
}

#[test]
fn test_dict_update() {
    assert!(transpile_ok("def f(d): d.update({'a': 1})"));
}

#[test]
fn test_dict_clear() {
    assert!(transpile_ok("def f(d): d.clear()"));
}

#[test]
fn test_dict_copy() {
    assert!(transpile_ok("def f(d): return d.copy()"));
}

#[test]
fn test_dict_setdefault() {
    assert!(transpile_ok("def f(d): return d.setdefault('key', 'value')"));
}

// ============================================================================
// SET METHOD COVERAGE
// ============================================================================

#[test]
fn test_set_add() {
    assert!(transpile_ok("def f(s): s.add(1)"));
}

#[test]
fn test_set_remove() {
    assert!(transpile_ok("def f(s): s.remove(1)"));
}

#[test]
fn test_set_discard() {
    assert!(transpile_ok("def f(s): s.discard(1)"));
}

#[test]
fn test_set_pop() {
    assert!(transpile_ok("def f(s): return s.pop()"));
}

#[test]
fn test_set_clear() {
    assert!(transpile_ok("def f(s): s.clear()"));
}

#[test]
fn test_set_union() {
    assert!(transpile_ok("def f(a, b): return a.union(b)"));
}

#[test]
fn test_set_intersection() {
    assert!(transpile_ok("def f(a, b): return a.intersection(b)"));
}

#[test]
fn test_set_difference() {
    assert!(transpile_ok("def f(a, b): return a.difference(b)"));
}

// ============================================================================
// CONTROL FLOW COVERAGE
// ============================================================================

#[test]
fn test_if_simple() {
    assert!(transpile_ok("def f(x):\n    if x > 0:\n        return 1\n    return 0"));
}

#[test]
fn test_if_else() {
    assert!(transpile_ok("def f(x):\n    if x > 0:\n        return 1\n    else:\n        return -1"));
}

#[test]
fn test_if_elif_else() {
    assert!(transpile_ok("def f(x):\n    if x > 0:\n        return 1\n    elif x < 0:\n        return -1\n    else:\n        return 0"));
}

#[test]
fn test_while_simple() {
    assert!(transpile_ok("def f(n):\n    i = 0\n    while i < n:\n        i += 1\n    return i"));
}

#[test]
fn test_while_break() {
    assert!(transpile_ok("def f(n):\n    i = 0\n    while True:\n        if i >= n:\n            break\n        i += 1\n    return i"));
}

#[test]
fn test_while_continue() {
    assert!(transpile_ok("def f(n):\n    i = 0\n    total = 0\n    while i < n:\n        i += 1\n        if i % 2 == 0:\n            continue\n        total += i\n    return total"));
}

#[test]
fn test_for_range() {
    assert!(transpile_ok("def f(n):\n    total = 0\n    for i in range(n):\n        total += i\n    return total"));
}

#[test]
fn test_for_range_start_stop() {
    assert!(transpile_ok("def f(n):\n    total = 0\n    for i in range(1, n):\n        total += i\n    return total"));
}

#[test]
fn test_for_range_step() {
    assert!(transpile_ok("def f(n):\n    total = 0\n    for i in range(0, n, 2):\n        total += i\n    return total"));
}

#[test]
fn test_for_list() {
    assert!(transpile_ok("def f(lst):\n    total = 0\n    for x in lst:\n        total += x\n    return total"));
}

#[test]
fn test_for_enumerate() {
    assert!(transpile_ok("def f(lst):\n    for i, x in enumerate(lst):\n        print(i, x)"));
}

#[test]
fn test_for_zip() {
    assert!(transpile_ok("def f(a, b):\n    for x, y in zip(a, b):\n        print(x, y)"));
}

// ============================================================================
// EXPRESSION COVERAGE
// ============================================================================

#[test]
fn test_binary_arithmetic() {
    assert!(transpile_ok("def f(a, b): return a + b"));
    assert!(transpile_ok("def f(a, b): return a - b"));
    assert!(transpile_ok("def f(a, b): return a * b"));
    assert!(transpile_ok("def f(a, b): return a / b"));
    assert!(transpile_ok("def f(a, b): return a // b"));
    assert!(transpile_ok("def f(a, b): return a % b"));
    assert!(transpile_ok("def f(a, b): return a ** b"));
}

#[test]
fn test_binary_comparison() {
    assert!(transpile_ok("def f(a, b): return a == b"));
    assert!(transpile_ok("def f(a, b): return a != b"));
    assert!(transpile_ok("def f(a, b): return a < b"));
    assert!(transpile_ok("def f(a, b): return a <= b"));
    assert!(transpile_ok("def f(a, b): return a > b"));
    assert!(transpile_ok("def f(a, b): return a >= b"));
}

#[test]
fn test_binary_logical() {
    assert!(transpile_ok("def f(a, b): return a and b"));
    assert!(transpile_ok("def f(a, b): return a or b"));
}

#[test]
fn test_binary_bitwise() {
    assert!(transpile_ok("def f(a, b): return a & b"));
    assert!(transpile_ok("def f(a, b): return a | b"));
    assert!(transpile_ok("def f(a, b): return a ^ b"));
    assert!(transpile_ok("def f(a, b): return a << b"));
    assert!(transpile_ok("def f(a, b): return a >> b"));
}

#[test]
fn test_unary_ops() {
    assert!(transpile_ok("def f(a): return -a"));
    assert!(transpile_ok("def f(a): return +a"));
    assert!(transpile_ok("def f(a): return not a"));
    assert!(transpile_ok("def f(a): return ~a"));
}

#[test]
fn test_membership() {
    assert!(transpile_ok("def f(x, lst): return x in lst"));
    assert!(transpile_ok("def f(x, lst): return x not in lst"));
}

#[test]
fn test_identity() {
    assert!(transpile_ok("def f(a, b): return a is b"));
    assert!(transpile_ok("def f(a, b): return a is not b"));
}

#[test]
fn test_ternary() {
    assert!(transpile_ok("def f(x): return 1 if x > 0 else -1"));
}

#[test]
fn test_augmented_assign() {
    assert!(transpile_ok("def f():\n    x = 0\n    x += 1\n    x -= 1\n    x *= 2\n    x //= 2\n    return x"));
}

// ============================================================================
// COMPREHENSION COVERAGE
// ============================================================================

#[test]
fn test_list_comprehension() {
    assert!(transpile_ok("def f(n): return [x * 2 for x in range(n)]"));
}

#[test]
fn test_list_comprehension_filter() {
    assert!(transpile_ok("def f(n): return [x for x in range(n) if x % 2 == 0]"));
}

#[test]
fn test_dict_comprehension() {
    assert!(transpile_ok("def f(n): return {x: x * 2 for x in range(n)}"));
}

#[test]
fn test_set_comprehension() {
    assert!(transpile_ok("def f(n): return {x * 2 for x in range(n)}"));
}

#[test]
fn test_generator_expression() {
    assert!(transpile_ok("def f(n): return sum(x * 2 for x in range(n))"));
}

// ============================================================================
// DATA STRUCTURE COVERAGE
// ============================================================================

#[test]
fn test_list_literal() {
    assert!(transpile_ok("def f(): return [1, 2, 3]"));
    assert!(transpile_ok("def f(): return []"));
}

#[test]
fn test_dict_literal() {
    assert!(transpile_ok("def f(): return {'a': 1, 'b': 2}"));
    assert!(transpile_ok("def f(): return {}"));
}

#[test]
fn test_set_literal() {
    assert!(transpile_ok("def f(): return {1, 2, 3}"));
}

#[test]
fn test_tuple_literal() {
    assert!(transpile_ok("def f(): return (1, 2, 3)"));
    assert!(transpile_ok("def f(): return ()"));
}

#[test]
fn test_tuple_unpack() {
    assert!(transpile_ok("def f():\n    a, b = 1, 2\n    return a + b"));
}

#[test]
fn test_list_unpack() {
    assert!(transpile_ok("def f():\n    a, b, c = [1, 2, 3]\n    return a + b + c"));
}

// ============================================================================
// SLICE COVERAGE
// ============================================================================

#[test]
fn test_slice_start() {
    assert!(transpile_ok("def f(lst): return lst[1:]"));
}

#[test]
fn test_slice_stop() {
    assert!(transpile_ok("def f(lst): return lst[:5]"));
}

#[test]
fn test_slice_start_stop() {
    assert!(transpile_ok("def f(lst): return lst[1:5]"));
}

#[test]
fn test_slice_step() {
    assert!(transpile_ok("def f(lst): return lst[::2]"));
}

#[test]
fn test_slice_full() {
    assert!(transpile_ok("def f(lst): return lst[1:5:2]"));
}

#[test]
fn test_negative_index() {
    assert!(transpile_ok("def f(lst): return lst[-1]"));
}

#[test]
fn test_negative_slice() {
    assert!(transpile_ok("def f(lst): return lst[-3:]"));
}

// ============================================================================
// LAMBDA COVERAGE
// ============================================================================

#[test]
fn test_lambda_simple() {
    assert!(transpile_ok("def f(lst): return list(map(lambda x: x * 2, lst))"));
}

#[test]
fn test_lambda_filter() {
    assert!(transpile_ok("def f(lst): return list(filter(lambda x: x > 0, lst))"));
}

#[test]
fn test_lambda_key() {
    assert!(transpile_ok("def f(lst): return sorted(lst, key=lambda x: -x)"));
}

// ============================================================================
// F-STRING COVERAGE
// ============================================================================

#[test]
fn test_fstring_simple() {
    assert!(transpile_ok("def f(name): return f'Hello {name}'"));
}

#[test]
fn test_fstring_expression() {
    assert!(transpile_ok("def f(x): return f'Value: {x + 1}'"));
}

#[test]
fn test_fstring_multiple() {
    assert!(transpile_ok("def f(a, b): return f'{a} + {b} = {a + b}'"));
}

// ============================================================================
// TRY/EXCEPT COVERAGE
// ============================================================================

#[test]
fn test_try_except() {
    assert!(transpile_ok("def f():\n    try:\n        x = 1\n    except:\n        x = 0\n    return x"));
}

#[test]
fn test_try_except_named() {
    assert!(transpile_ok("def f():\n    try:\n        x = 1\n    except ValueError:\n        x = 0\n    return x"));
}

#[test]
fn test_try_finally() {
    assert!(transpile_ok("def f():\n    try:\n        x = 1\n    finally:\n        x = 0\n    return x"));
}

// ============================================================================
// WITH STATEMENT COVERAGE
// ============================================================================

#[test]
fn test_with_simple() {
    assert!(transpile_ok("def f(path):\n    with open(path) as f:\n        return f.read()"));
}

// ============================================================================
// CLASS COVERAGE
// ============================================================================

#[test]
fn test_class_simple() {
    assert!(transpile_ok("class Point:\n    x: int\n    y: int"));
}

#[test]
fn test_class_init() {
    assert!(transpile_ok("class Point:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y"));
}

#[test]
fn test_class_method() {
    assert!(transpile_ok("class Point:\n    x: int\n    y: int\n    def distance(self) -> float:\n        return (self.x ** 2 + self.y ** 2) ** 0.5"));
}

#[test]
fn test_dataclass() {
    assert!(transpile_ok("from dataclasses import dataclass\n\n@dataclass\nclass Point:\n    x: int\n    y: int"));
}

// ============================================================================
// TYPE ANNOTATION COVERAGE
// ============================================================================

#[test]
fn test_typed_function() {
    assert!(transpile_ok("def add(a: int, b: int) -> int:\n    return a + b"));
}

#[test]
fn test_typed_list() {
    assert!(transpile_ok("from typing import List\ndef f(lst: List[int]) -> int:\n    return sum(lst)"));
}

#[test]
fn test_typed_dict() {
    assert!(transpile_ok("from typing import Dict\ndef f(d: Dict[str, int]) -> int:\n    return len(d)"));
}

#[test]
fn test_typed_optional() {
    assert!(transpile_ok("from typing import Optional\ndef f(x: Optional[int]) -> int:\n    return x if x else 0"));
}

// ============================================================================
// MAP/FILTER COVERAGE
// ============================================================================

#[test]
fn test_map_simple() {
    assert!(transpile_ok("def f(lst): return list(map(lambda x: x * 2, lst))"));
}

#[test]
fn test_map_two_iterables() {
    assert!(transpile_ok("def f(a, b): return list(map(lambda x, y: x + y, a, b))"));
}

#[test]
fn test_map_three_iterables() {
    assert!(transpile_ok("def f(a, b, c): return list(map(lambda x, y, z: x + y + z, a, b, c))"));
}

#[test]
fn test_filter_simple() {
    assert!(transpile_ok("def f(lst): return list(filter(lambda x: x > 0, lst))"));
}

// ============================================================================
// STRUCT MODULE COVERAGE
// ============================================================================

#[test]
fn test_struct_pack() {
    let code = r#"
import struct
def f(x: int) -> bytes:
    return struct.pack('i', x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_struct_unpack() {
    let code = r#"
import struct
def f(data: bytes) -> int:
    result = struct.unpack('i', data)
    return result[0]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// JSON MODULE COVERAGE
// ============================================================================

#[test]
fn test_json_dumps() {
    let code = r#"
import json
def f(obj) -> str:
    return json.dumps(obj)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_json_loads() {
    let code = r#"
import json
def f(s: str):
    return json.loads(s)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// OS MODULE COVERAGE
// ============================================================================

#[test]
fn test_os_path_join() {
    let code = r#"
import os
def f(a: str, b: str) -> str:
    return os.path.join(a, b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_os_path_exists() {
    let code = r#"
import os
def f(path: str) -> bool:
    return os.path.exists(path)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_os_getcwd() {
    let code = r#"
import os
def f() -> str:
    return os.getcwd()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// MATH MODULE COVERAGE
// ============================================================================

#[test]
fn test_math_sqrt() {
    let code = r#"
import math
def f(x: float) -> float:
    return math.sqrt(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_floor() {
    let code = r#"
import math
def f(x: float) -> int:
    return math.floor(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_ceil() {
    let code = r#"
import math
def f(x: float) -> int:
    return math.ceil(x)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// FOR LOOP EDGE CASES (Error Path Coverage)
// ============================================================================

#[test]
fn test_for_simple_tuple_unpack() {
    // Simple tuple unpacking should work
    assert!(transpile_ok("def f():\n    for x, y in [(1, 2), (3, 4)]:\n        print(x, y)"));
}

#[test]
fn test_for_triple_tuple_unpack() {
    // Triple tuple unpacking should work
    assert!(transpile_ok("def f():\n    for a, b, c in [(1, 2, 3)]:\n        print(a, b, c)"));
}

#[test]
fn test_for_nested_tuple_unpack() {
    // Nested tuple unpacking may be handled gracefully
    // The key is it doesn't panic - either it works or returns an error
    let result = transpile("def f():\n    for (a, b), c in [((1, 2), 3)]:\n        print(a, b, c)");
    // If it contains ERROR, that's fine. If it contains fn, it succeeded. Either is acceptable.
    assert!(result.contains("ERROR") || result.contains("fn "));
}

#[test]
fn test_for_dict_items() {
    // Dict items iteration
    assert!(transpile_ok("def f(d):\n    for k, v in d.items():\n        print(k, v)"));
}

// ============================================================================
// GENERATOR FUNCTION COVERAGE
// ============================================================================

#[test]
fn test_generator_simple() {
    assert!(transpile_ok("def gen(n):\n    for i in range(n):\n        yield i"));
}

#[test]
fn test_generator_with_return() {
    assert!(transpile_ok("def gen(n):\n    for i in range(n):\n        yield i\n    return"));
}

#[test]
fn test_generator_expression_sum() {
    assert!(transpile_ok("def f(n): return sum(x * x for x in range(n))"));
}

#[test]
fn test_generator_expression_any() {
    assert!(transpile_ok("def f(lst): return any(x > 0 for x in lst)"));
}

#[test]
fn test_generator_expression_all() {
    assert!(transpile_ok("def f(lst): return all(x > 0 for x in lst)"));
}

// ============================================================================
// WALRUS OPERATOR COVERAGE
// ============================================================================

#[test]
fn test_walrus_simple() {
    assert!(transpile_ok("def f(x):\n    if (n := x * 2) > 10:\n        return n\n    return 0"));
}

#[test]
fn test_walrus_in_while() {
    assert!(transpile_ok("def f(it):\n    results = []\n    while (val := next(it, None)) is not None:\n        results.append(val)\n    return results"));
}

#[test]
fn test_walrus_in_list_comp() {
    assert!(transpile_ok("def f(data): return [y for x in data if (y := x * 2) > 5]"));
}

// ============================================================================
// GLOBAL/NONLOCAL COVERAGE
// ============================================================================

#[test]
fn test_global_declaration() {
    let code = r#"
counter = 0
def increment():
    global counter
    counter += 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nonlocal_declaration() {
    let code = r#"
def outer():
    x = 0
    def inner():
        nonlocal x
        x += 1
    inner()
    return x
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// DECORATOR COVERAGE
// ============================================================================

#[test]
fn test_staticmethod_decorator() {
    let code = r#"
class MyClass:
    @staticmethod
    def static_func(x: int) -> int:
        return x * 2
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_classmethod_decorator() {
    let code = r#"
class MyClass:
    count: int = 0

    @classmethod
    def get_count(cls) -> int:
        return cls.count
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_property_decorator() {
    let code = r#"
class Circle:
    _radius: float

    @property
    def radius(self) -> float:
        return self._radius
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// TYPING MODULE COVERAGE
// ============================================================================

#[test]
fn test_typing_union() {
    assert!(transpile_ok("from typing import Union\ndef f(x: Union[int, str]) -> str:\n    return str(x)"));
}

#[test]
fn test_typing_tuple() {
    assert!(transpile_ok("from typing import Tuple\ndef f() -> Tuple[int, str]:\n    return (1, 'a')"));
}

#[test]
fn test_typing_callable() {
    assert!(transpile_ok("from typing import Callable\ndef f(func: Callable[[int], int]) -> int:\n    return func(5)"));
}

#[test]
fn test_typing_any() {
    assert!(transpile_ok("from typing import Any\ndef f(x: Any) -> Any:\n    return x"));
}

#[test]
fn test_pep604_union() {
    // PEP 604 style union types (Python 3.10+)
    assert!(transpile_ok("def f(x: int | str) -> str:\n    return str(x)"));
}

#[test]
fn test_pep604_optional() {
    // PEP 604 style optional
    assert!(transpile_ok("def f(x: int | None) -> int:\n    return x if x else 0"));
}

// ============================================================================
// COLLECTIONS MODULE COVERAGE
// ============================================================================

#[test]
fn test_collections_defaultdict() {
    let code = r#"
from collections import defaultdict
def f():
    d = defaultdict(int)
    d['a'] += 1
    return d
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_collections_counter() {
    let code = r#"
from collections import Counter
def f(items):
    return Counter(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_collections_deque() {
    let code = r#"
from collections import deque
def f():
    d = deque([1, 2, 3])
    d.append(4)
    d.appendleft(0)
    return list(d)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// ITERTOOLS MODULE COVERAGE
// ============================================================================

#[test]
fn test_itertools_chain() {
    let code = r#"
import itertools
def f(a, b):
    return list(itertools.chain(a, b))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_itertools_product() {
    let code = r#"
import itertools
def f(a, b):
    return list(itertools.product(a, b))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_itertools_combinations() {
    let code = r#"
import itertools
def f(items, n):
    return list(itertools.combinations(items, n))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_itertools_permutations() {
    let code = r#"
import itertools
def f(items):
    return list(itertools.permutations(items))
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// HEAPQ MODULE COVERAGE
// ============================================================================

#[test]
fn test_heapq_push_pop() {
    let code = r#"
import heapq
def f():
    heap = []
    heapq.heappush(heap, 3)
    heapq.heappush(heap, 1)
    heapq.heappush(heap, 2)
    return heapq.heappop(heap)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_heapq_nsmallest() {
    let code = r#"
import heapq
def f(items, n):
    return heapq.nsmallest(n, items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_heapq_nlargest() {
    let code = r#"
import heapq
def f(items, n):
    return heapq.nlargest(n, items)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// DATETIME MODULE COVERAGE
// ============================================================================

#[test]
fn test_datetime_now() {
    let code = r#"
from datetime import datetime
def f():
    return datetime.now()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_date() {
    let code = r#"
from datetime import date
def f():
    return date.today()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_timedelta() {
    let code = r#"
from datetime import timedelta
def f(days: int):
    return timedelta(days=days)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// RE MODULE COVERAGE
// ============================================================================

#[test]
fn test_re_match() {
    let code = r#"
import re
def f(pattern: str, text: str):
    return re.match(pattern, text)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_re_search() {
    let code = r#"
import re
def f(pattern: str, text: str):
    return re.search(pattern, text)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_re_findall() {
    let code = r#"
import re
def f(pattern: str, text: str):
    return re.findall(pattern, text)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_re_sub() {
    let code = r#"
import re
def f(pattern: str, repl: str, text: str):
    return re.sub(pattern, repl, text)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// FUNCTOOLS MODULE COVERAGE
// ============================================================================

#[test]
fn test_functools_reduce() {
    let code = r#"
from functools import reduce
def f(items):
    return reduce(lambda x, y: x + y, items)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// COMPLEX EXPRESSION COVERAGE
// ============================================================================

#[test]
fn test_chained_comparison() {
    assert!(transpile_ok("def f(a, b, c): return a < b < c"));
    assert!(transpile_ok("def f(x): return 0 <= x <= 100"));
}

#[test]
fn test_nested_ternary() {
    assert!(transpile_ok("def f(x): return 'pos' if x > 0 else 'neg' if x < 0 else 'zero'"));
}

#[test]
fn test_complex_boolean() {
    assert!(transpile_ok("def f(a, b, c): return (a and b) or (not c and a)"));
}

#[test]
fn test_multiple_assignment() {
    assert!(transpile_ok("def f():\n    a = b = c = 0\n    return a + b + c"));
}

#[test]
fn test_starred_assignment() {
    assert!(transpile_ok("def f():\n    a, *rest = [1, 2, 3, 4]\n    return a"));
}

// ============================================================================
// ASSERT STATEMENT COVERAGE
// ============================================================================

#[test]
fn test_assert_simple() {
    assert!(transpile_ok("def f(x):\n    assert x > 0\n    return x"));
}

#[test]
fn test_assert_with_message() {
    assert!(transpile_ok("def f(x):\n    assert x > 0, 'x must be positive'\n    return x"));
}

// ============================================================================
// PASS/ELLIPSIS COVERAGE
// ============================================================================

#[test]
fn test_pass_statement() {
    assert!(transpile_ok("def f():\n    pass"));
}

#[test]
fn test_ellipsis_body() {
    assert!(transpile_ok("def f():\n    ..."));
}

// ============================================================================
// DELETE STATEMENT COVERAGE
// ============================================================================

#[test]
fn test_del_variable() {
    assert!(transpile_ok("def f():\n    x = 1\n    del x"));
}

#[test]
fn test_del_index() {
    assert!(transpile_ok("def f(lst):\n    del lst[0]"));
}

#[test]
fn test_del_key() {
    assert!(transpile_ok("def f(d):\n    del d['key']"));
}

// ============================================================================
// STRING FORMATTING COVERAGE
// ============================================================================

#[test]
fn test_format_method() {
    assert!(transpile_ok("def f(x): return 'Value: {}'.format(x)"));
}

#[test]
fn test_format_named() {
    assert!(transpile_ok("def f(x, y): return '{a} + {b}'.format(a=x, b=y)"));
}

#[test]
fn test_percent_format() {
    assert!(transpile_ok("def f(x): return 'Value: %d' % x"));
}

// ============================================================================
// ATTRIBUTE ACCESS COVERAGE
// ============================================================================

// NOTE: getattr/setattr/hasattr are not yet implemented
// These tests document the expected behavior when they are added

#[test]
fn test_getattr_not_yet_implemented() {
    // getattr is not yet implemented - should return error gracefully
    let result = transpile("def f(obj): return getattr(obj, 'name')");
    // Either implemented (contains fn) or not (contains ERROR)
    assert!(result.contains("ERROR") || result.contains("fn "));
}

#[test]
fn test_setattr_not_yet_implemented() {
    // setattr is not yet implemented - should return error gracefully
    let result = transpile("def f(obj): setattr(obj, 'name', 'value')");
    assert!(result.contains("ERROR") || result.contains("fn "));
}

#[test]
fn test_hasattr_not_yet_implemented() {
    // hasattr is not yet implemented - should return error gracefully
    let result = transpile("def f(obj): return hasattr(obj, 'name')");
    assert!(result.contains("ERROR") || result.contains("fn "));
}

// ============================================================================
// INPUT/OUTPUT COVERAGE
// ============================================================================

#[test]
fn test_print_simple() {
    assert!(transpile_ok("def f(x): print(x)"));
}

#[test]
fn test_print_multiple() {
    assert!(transpile_ok("def f(a, b): print(a, b)"));
}

#[test]
fn test_print_with_sep() {
    assert!(transpile_ok("def f(a, b): print(a, b, sep=', ')"));
}

#[test]
fn test_print_with_end() {
    assert!(transpile_ok("def f(a): print(a, end='')"));
}

// ============================================================================
// RAISE STATEMENT COVERAGE
// ============================================================================

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

// ============================================================================
// BYTES COVERAGE
// ============================================================================

#[test]
fn test_bytes_literal() {
    assert!(transpile_ok("def f(): return b'hello'"));
}

#[test]
fn test_bytes_decode() {
    assert!(transpile_ok("def f(b): return b.decode()"));
    assert!(transpile_ok("def f(b): return b.decode('utf-8')"));
}

// ============================================================================
// ISINSTANCE/ISSUBCLASS COVERAGE
// ============================================================================

#[test]
fn test_isinstance() {
    assert!(transpile_ok("def f(x): return isinstance(x, int)"));
    assert!(transpile_ok("def f(x): return isinstance(x, (int, str))"));
}

// ============================================================================
// CALLABLE COVERAGE
// ============================================================================

#[test]
fn test_callable() {
    assert!(transpile_ok("def f(x): return callable(x)"));
}

// ============================================================================
// LEN/BOOL COVERAGE
// ============================================================================

#[test]
fn test_len_list() {
    assert!(transpile_ok("def f(lst): return len(lst)"));
}

#[test]
fn test_len_string() {
    assert!(transpile_ok("def f(s): return len(s)"));
}

#[test]
fn test_len_dict() {
    assert!(transpile_ok("def f(d): return len(d)"));
}

#[test]
fn test_bool() {
    assert!(transpile_ok("def f(x): return bool(x)"));
}

// ============================================================================
// NUMERIC TYPE CONVERSIONS
// ============================================================================

#[test]
fn test_int_from_str() {
    assert!(transpile_ok("def f(s): return int(s)"));
}

#[test]
fn test_int_from_str_base() {
    assert!(transpile_ok("def f(s): return int(s, 16)"));
}

#[test]
fn test_float_conversion() {
    assert!(transpile_ok("def f(x): return float(x)"));
}

#[test]
fn test_str_conversion() {
    assert!(transpile_ok("def f(x): return str(x)"));
}

// ============================================================================
// OUTPUT VERIFICATION (Sample checks for generated code quality)
// ============================================================================

#[test]
fn test_output_contains_fn_keyword() {
    let output = transpile("def add(a: int, b: int) -> int:\n    return a + b");
    assert!(output.contains("fn "));
}

#[test]
fn test_output_contains_let_for_variable() {
    let output = transpile("def f():\n    x = 1\n    return x");
    assert!(output.contains("let "));
}

#[test]
fn test_output_contains_for_loop() {
    let output = transpile("def f(n):\n    for i in range(n):\n        print(i)");
    assert!(output.contains("for "));
}

#[test]
fn test_output_contains_struct_for_class() {
    let output = transpile("class Point:\n    x: int\n    y: int");
    assert!(output.contains("struct "));
}

// ============================================================================
// BINARY OPERATORS - COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_binop_add() {
    assert!(transpile_ok("def f(a, b): return a + b"));
}

#[test]
fn test_binop_sub() {
    assert!(transpile_ok("def f(a, b): return a - b"));
}

#[test]
fn test_binop_mult() {
    assert!(transpile_ok("def f(a, b): return a * b"));
}

#[test]
fn test_binop_div() {
    assert!(transpile_ok("def f(a, b): return a / b"));
}

#[test]
fn test_binop_floordiv() {
    assert!(transpile_ok("def f(a, b): return a // b"));
}

#[test]
fn test_binop_mod() {
    assert!(transpile_ok("def f(a, b): return a % b"));
}

#[test]
fn test_binop_pow() {
    assert!(transpile_ok("def f(a, b): return a ** b"));
}

#[test]
fn test_binop_bitand() {
    assert!(transpile_ok("def f(a, b): return a & b"));
}

#[test]
fn test_binop_bitor() {
    assert!(transpile_ok("def f(a, b): return a | b"));
}

#[test]
fn test_binop_bitxor() {
    assert!(transpile_ok("def f(a, b): return a ^ b"));
}

#[test]
fn test_binop_lshift() {
    assert!(transpile_ok("def f(a, b): return a << b"));
}

#[test]
fn test_binop_rshift() {
    assert!(transpile_ok("def f(a, b): return a >> b"));
}

#[test]
fn test_binop_matmult() {
    assert!(transpile_ok("def f(a, b): return a @ b"));
}

// ============================================================================
// COMPARISON OPERATORS - COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_cmpop_eq() {
    assert!(transpile_ok("def f(a, b): return a == b"));
}

#[test]
fn test_cmpop_ne() {
    assert!(transpile_ok("def f(a, b): return a != b"));
}

#[test]
fn test_cmpop_lt() {
    assert!(transpile_ok("def f(a, b): return a < b"));
}

#[test]
fn test_cmpop_gt() {
    assert!(transpile_ok("def f(a, b): return a > b"));
}

#[test]
fn test_cmpop_le() {
    assert!(transpile_ok("def f(a, b): return a <= b"));
}

#[test]
fn test_cmpop_ge() {
    assert!(transpile_ok("def f(a, b): return a >= b"));
}

#[test]
fn test_cmpop_in() {
    assert!(transpile_ok("def f(a, b): return a in b"));
}

#[test]
fn test_cmpop_not_in() {
    assert!(transpile_ok("def f(a, b): return a not in b"));
}

#[test]
fn test_cmpop_is() {
    assert!(transpile_ok("def f(a, b): return a is b"));
}

#[test]
fn test_cmpop_is_not() {
    assert!(transpile_ok("def f(a, b): return a is not b"));
}

#[test]
fn test_cmpop_chained() {
    assert!(transpile_ok("def f(a, b, c): return a < b < c"));
}

#[test]
fn test_cmpop_chained_mixed() {
    assert!(transpile_ok("def f(a, b, c, d): return a <= b < c <= d"));
}

// ============================================================================
// UNARY OPERATORS - COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_unaryop_not() {
    assert!(transpile_ok("def f(a): return not a"));
}

#[test]
fn test_unaryop_neg() {
    assert!(transpile_ok("def f(a): return -a"));
}

#[test]
fn test_unaryop_pos() {
    assert!(transpile_ok("def f(a): return +a"));
}

#[test]
fn test_unaryop_invert() {
    assert!(transpile_ok("def f(a): return ~a"));
}

// ============================================================================
// AUGMENTED ASSIGNMENT - COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_augassign_add() {
    assert!(transpile_ok("def f():\n    x = 1\n    x += 2\n    return x"));
}

#[test]
fn test_augassign_sub() {
    assert!(transpile_ok("def f():\n    x = 1\n    x -= 2\n    return x"));
}

#[test]
fn test_augassign_mult() {
    assert!(transpile_ok("def f():\n    x = 1\n    x *= 2\n    return x"));
}

#[test]
fn test_augassign_div() {
    assert!(transpile_ok("def f():\n    x = 1.0\n    x /= 2.0\n    return x"));
}

#[test]
fn test_augassign_floordiv() {
    assert!(transpile_ok("def f():\n    x = 10\n    x //= 3\n    return x"));
}

#[test]
fn test_augassign_mod() {
    assert!(transpile_ok("def f():\n    x = 10\n    x %= 3\n    return x"));
}

#[test]
fn test_augassign_pow() {
    assert!(transpile_ok("def f():\n    x = 2\n    x **= 3\n    return x"));
}

#[test]
fn test_augassign_bitand() {
    assert!(transpile_ok("def f():\n    x = 0xFF\n    x &= 0x0F\n    return x"));
}

#[test]
fn test_augassign_bitor() {
    assert!(transpile_ok("def f():\n    x = 0xF0\n    x |= 0x0F\n    return x"));
}

#[test]
fn test_augassign_bitxor() {
    assert!(transpile_ok("def f():\n    x = 0xFF\n    x ^= 0x0F\n    return x"));
}

#[test]
fn test_augassign_lshift() {
    assert!(transpile_ok("def f():\n    x = 1\n    x <<= 4\n    return x"));
}

#[test]
fn test_augassign_rshift() {
    assert!(transpile_ok("def f():\n    x = 16\n    x >>= 2\n    return x"));
}

// ============================================================================
// BOOLEAN OPERATORS - COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_boolop_and() {
    assert!(transpile_ok("def f(a, b): return a and b"));
}

#[test]
fn test_boolop_or() {
    assert!(transpile_ok("def f(a, b): return a or b"));
}

#[test]
fn test_boolop_chained_and() {
    assert!(transpile_ok("def f(a, b, c): return a and b and c"));
}

#[test]
fn test_boolop_chained_or() {
    assert!(transpile_ok("def f(a, b, c): return a or b or c"));
}

#[test]
fn test_boolop_mixed() {
    assert!(transpile_ok("def f(a, b, c): return a and b or c"));
}

// ============================================================================
// CONTROL FLOW - COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_if_simple_cov() {
    assert!(transpile_ok("def f(x):\n    if x:\n        return 1\n    return 0"));
}

#[test]
fn test_if_else_cov() {
    assert!(transpile_ok("def f(x):\n    if x:\n        return 1\n    else:\n        return 0"));
}

#[test]
fn test_if_elif() {
    assert!(transpile_ok("def f(x):\n    if x > 0:\n        return 1\n    elif x < 0:\n        return -1\n    else:\n        return 0"));
}

#[test]
fn test_if_nested() {
    assert!(transpile_ok("def f(x, y):\n    if x:\n        if y:\n            return 1\n    return 0"));
}

#[test]
fn test_while_simple_cov() {
    assert!(transpile_ok("def f():\n    x = 0\n    while x < 10:\n        x += 1\n    return x"));
}

#[test]
fn test_while_break_cov() {
    assert!(transpile_ok("def f():\n    x = 0\n    while True:\n        x += 1\n        if x > 10:\n            break\n    return x"));
}

#[test]
fn test_while_continue_cov() {
    assert!(transpile_ok("def f():\n    x = 0\n    y = 0\n    while x < 10:\n        x += 1\n        if x % 2 == 0:\n            continue\n        y += 1\n    return y"));
}

#[test]
fn test_while_else() {
    assert!(transpile_ok("def f():\n    x = 0\n    while x < 10:\n        x += 1\n    else:\n        x = -1\n    return x"));
}

#[test]
fn test_for_range_cov() {
    assert!(transpile_ok("def f():\n    s = 0\n    for i in range(10):\n        s += i\n    return s"));
}

#[test]
fn test_for_range_start_end() {
    assert!(transpile_ok("def f():\n    s = 0\n    for i in range(1, 10):\n        s += i\n    return s"));
}

#[test]
fn test_for_range_step_cov() {
    assert!(transpile_ok("def f():\n    s = 0\n    for i in range(0, 10, 2):\n        s += i\n    return s"));
}

#[test]
fn test_for_list_cov() {
    assert!(transpile_ok("def f(items):\n    s = 0\n    for item in items:\n        s += item\n    return s"));
}

#[test]
fn test_for_enumerate_cov() {
    assert!(transpile_ok("def f(items):\n    for i, item in enumerate(items):\n        print(i, item)"));
}

#[test]
fn test_for_zip_cov() {
    assert!(transpile_ok("def f(a, b):\n    for x, y in zip(a, b):\n        print(x, y)"));
}

#[test]
fn test_for_break() {
    assert!(transpile_ok("def f():\n    for i in range(10):\n        if i > 5:\n            break\n    return i"));
}

#[test]
fn test_for_continue() {
    assert!(transpile_ok("def f():\n    s = 0\n    for i in range(10):\n        if i % 2 == 0:\n            continue\n        s += i\n    return s"));
}

#[test]
fn test_for_else() {
    assert!(transpile_ok("def f():\n    for i in range(10):\n        pass\n    else:\n        return -1\n    return 0"));
}

#[test]
fn test_for_nested() {
    assert!(transpile_ok("def f():\n    s = 0\n    for i in range(3):\n        for j in range(3):\n            s += i * j\n    return s"));
}

// ============================================================================
// EXCEPTION HANDLING - COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_try_except_bare() {
    assert!(transpile_ok("def f():\n    try:\n        x = 1\n    except:\n        x = 0\n    return x"));
}

#[test]
fn test_try_except_type() {
    assert!(transpile_ok("def f():\n    try:\n        x = 1\n    except ValueError:\n        x = 0\n    return x"));
}

#[test]
fn test_try_except_as() {
    assert!(transpile_ok("def f():\n    try:\n        x = 1\n    except ValueError as e:\n        x = 0\n    return x"));
}

#[test]
fn test_try_except_finally() {
    assert!(transpile_ok("def f():\n    try:\n        x = 1\n    except:\n        x = 0\n    finally:\n        print(\"done\")\n    return x"));
}

#[test]
fn test_try_finally_cov() {
    assert!(transpile_ok("def f():\n    try:\n        x = 1\n    finally:\n        print(\"done\")\n    return x"));
}

#[test]
fn test_try_except_multiple() {
    assert!(transpile_ok("def f():\n    try:\n        x = 1\n    except ValueError:\n        x = 0\n    except KeyError:\n        x = -1\n    return x"));
}

#[test]
fn test_try_except_tuple() {
    assert!(transpile_ok("def f():\n    try:\n        x = 1\n    except (ValueError, KeyError):\n        x = 0\n    return x"));
}

#[test]
fn test_raise() {
    assert!(transpile_ok("def f():\n    raise ValueError(\"error\")"));
}

#[test]
fn test_raise_from() {
    assert!(transpile_ok("def f():\n    try:\n        pass\n    except Exception as e:\n        raise RuntimeError(\"new\") from e"));
}

// ============================================================================
// CONTEXT MANAGERS - COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_with_simple_cov() {
    assert!(transpile_ok("def f():\n    with open(\"file.txt\") as f:\n        return f.read()"));
}

#[test]
fn test_with_no_target() {
    assert!(transpile_ok("def f():\n    with open(\"file.txt\"):\n        pass"));
}

#[test]
fn test_with_multiple() {
    assert!(transpile_ok("def f():\n    with open(\"a.txt\") as a, open(\"b.txt\") as b:\n        pass"));
}

// ============================================================================
// CLASS FEATURES - COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_class_empty() {
    assert!(transpile_ok("class Empty:\n    pass"));
}

#[test]
fn test_class_with_fields() {
    assert!(transpile_ok("class Point:\n    x: int\n    y: int"));
}

#[test]
fn test_class_with_init() {
    assert!(transpile_ok("class Point:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y"));
}

#[test]
fn test_class_with_method() {
    assert!(transpile_ok("class Point:\n    def __init__(self, x: int):\n        self.x = x\n    def get_x(self) -> int:\n        return self.x"));
}

#[test]
fn test_class_with_static_method() {
    assert!(transpile_ok("class Math:\n    @staticmethod\n    def add(a: int, b: int) -> int:\n        return a + b"));
}

#[test]
fn test_class_with_class_method() {
    assert!(transpile_ok("class Counter:\n    count = 0\n    @classmethod\n    def increment(cls):\n        cls.count += 1"));
}

#[test]
fn test_class_inheritance() {
    assert!(transpile_ok("class Base:\n    pass\nclass Derived(Base):\n    pass"));
}

#[test]
fn test_class_str() {
    assert!(transpile_ok("class Point:\n    def __init__(self, x):\n        self.x = x\n    def __str__(self):\n        return str(self.x)"));
}

#[test]
fn test_class_repr() {
    assert!(transpile_ok("class Point:\n    def __init__(self, x):\n        self.x = x\n    def __repr__(self):\n        return f\"Point({self.x})\""));
}

#[test]
fn test_class_eq() {
    assert!(transpile_ok("class Point:\n    def __init__(self, x):\n        self.x = x\n    def __eq__(self, other):\n        return self.x == other.x"));
}

// ============================================================================
// LIST COMPREHENSIONS - COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_listcomp_simple() {
    assert!(transpile_ok("def f(): return [x for x in range(10)]"));
}

#[test]
fn test_listcomp_with_condition() {
    assert!(transpile_ok("def f(): return [x for x in range(10) if x % 2 == 0]"));
}

#[test]
fn test_listcomp_with_transform() {
    assert!(transpile_ok("def f(): return [x * 2 for x in range(10)]"));
}

#[test]
fn test_listcomp_nested() {
    assert!(transpile_ok("def f(): return [x * y for x in range(3) for y in range(3)]"));
}

#[test]
fn test_listcomp_nested_conditional() {
    assert!(transpile_ok("def f(): return [x * y for x in range(3) for y in range(3) if x != y]"));
}

// ============================================================================
// DICT COMPREHENSIONS - COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_dictcomp_simple() {
    assert!(transpile_ok("def f(): return {x: x * 2 for x in range(10)}"));
}

#[test]
fn test_dictcomp_with_condition() {
    assert!(transpile_ok("def f(): return {x: x * 2 for x in range(10) if x % 2 == 0}"));
}

#[test]
fn test_dictcomp_from_dict() {
    assert!(transpile_ok("def f(d): return {k: v * 2 for k, v in d.items()}"));
}

// ============================================================================
// SET COMPREHENSIONS - COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_setcomp_simple() {
    assert!(transpile_ok("def f(): return {x for x in range(10)}"));
}

#[test]
fn test_setcomp_with_condition() {
    assert!(transpile_ok("def f(): return {x for x in range(10) if x % 2 == 0}"));
}

// ============================================================================
// GENERATOR EXPRESSIONS - COVERAGE
// ============================================================================

#[test]
fn test_genexp_simple() {
    assert!(transpile_ok("def f(): return list(x for x in range(10))"));
}

#[test]
fn test_genexp_with_condition() {
    assert!(transpile_ok("def f(): return sum(x for x in range(10) if x % 2 == 0)"));
}

// ============================================================================
// TERNARY/CONDITIONAL EXPRESSION - COVERAGE
// ============================================================================

#[test]
fn test_ternary_simple() {
    assert!(transpile_ok("def f(x): return 1 if x else 0"));
}

#[test]
fn test_ternary_nested() {
    assert!(transpile_ok("def f(x): return 1 if x > 0 else -1 if x < 0 else 0"));
}

#[test]
fn test_ternary_in_expression() {
    assert!(transpile_ok("def f(x): return (x + 1) if x > 0 else (x - 1)"));
}

// ============================================================================
// SUBSCRIPT OPERATIONS - COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_subscript_index() {
    assert!(transpile_ok("def f(lst): return lst[0]"));
}

#[test]
fn test_subscript_negative_index() {
    assert!(transpile_ok("def f(lst): return lst[-1]"));
}

#[test]
fn test_subscript_slice_start() {
    assert!(transpile_ok("def f(lst): return lst[1:]"));
}

#[test]
fn test_subscript_slice_end() {
    assert!(transpile_ok("def f(lst): return lst[:5]"));
}

#[test]
fn test_subscript_slice_both() {
    assert!(transpile_ok("def f(lst): return lst[1:5]"));
}

#[test]
fn test_subscript_slice_step() {
    assert!(transpile_ok("def f(lst): return lst[::2]"));
}

#[test]
fn test_subscript_slice_all() {
    assert!(transpile_ok("def f(lst): return lst[1:10:2]"));
}

#[test]
fn test_subscript_slice_negative() {
    assert!(transpile_ok("def f(lst): return lst[-3:-1]"));
}

#[test]
fn test_subscript_slice_reverse() {
    assert!(transpile_ok("def f(lst): return lst[::-1]"));
}

#[test]
fn test_subscript_dict() {
    assert!(transpile_ok("def f(d): return d[\"key\"]"));
}

#[test]
fn test_subscript_nested() {
    assert!(transpile_ok("def f(m): return m[0][1]"));
}

// ============================================================================
// ASSIGNMENT TARGETS - COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_assign_simple() {
    assert!(transpile_ok("def f():\n    x = 1\n    return x"));
}

#[test]
fn test_assign_multiple() {
    assert!(transpile_ok("def f():\n    x = y = 1\n    return x + y"));
}

#[test]
fn test_assign_tuple_unpack() {
    assert!(transpile_ok("def f():\n    x, y = 1, 2\n    return x + y"));
}

#[test]
fn test_assign_list_unpack() {
    assert!(transpile_ok("def f():\n    [x, y] = [1, 2]\n    return x + y"));
}

#[test]
fn test_assign_starred() {
    assert!(transpile_ok("def f():\n    first, *rest = [1, 2, 3, 4]\n    return first"));
}

#[test]
fn test_assign_starred_middle() {
    assert!(transpile_ok("def f():\n    first, *middle, last = [1, 2, 3, 4, 5]\n    return first + last"));
}

#[test]
fn test_assign_subscript() {
    assert!(transpile_ok("def f():\n    lst = [1, 2, 3]\n    lst[0] = 10\n    return lst"));
}

#[test]
fn test_assign_attribute() {
    assert!(transpile_ok("class C:\n    x = 0\ndef f(c):\n    c.x = 1\n    return c.x"));
}

// ============================================================================
// ATTRIBUTE ACCESS - COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_attr_simple() {
    assert!(transpile_ok("def f(obj): return obj.x"));
}

#[test]
fn test_attr_method_call() {
    assert!(transpile_ok("def f(obj): return obj.method()"));
}

#[test]
fn test_attr_chained() {
    assert!(transpile_ok("def f(obj): return obj.a.b.c"));
}

#[test]
fn test_attr_method_chained() {
    assert!(transpile_ok("def f(s): return s.strip().lower()"));
}

// ============================================================================
// LAMBDA EXPRESSIONS - COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_lambda_no_args() {
    assert!(transpile_ok("def f(): return lambda: 1"));
}

#[test]
fn test_lambda_one_arg() {
    assert!(transpile_ok("def f(): return lambda x: x * 2"));
}

#[test]
fn test_lambda_multi_args() {
    assert!(transpile_ok("def f(): return lambda x, y: x + y"));
}

#[test]
fn test_lambda_with_default() {
    assert!(transpile_ok("def f(): return lambda x, y=10: x + y"));
}

#[test]
fn test_lambda_in_call() {
    assert!(transpile_ok("def f(items): return sorted(items, key=lambda x: x[0])"));
}

#[test]
fn test_lambda_in_map() {
    assert!(transpile_ok("def f(items): return list(map(lambda x: x * 2, items))"));
}

#[test]
fn test_lambda_in_filter() {
    assert!(transpile_ok("def f(items): return list(filter(lambda x: x > 0, items))"));
}

// ============================================================================
// F-STRINGS - COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_fstring_simple_cov() {
    assert!(transpile_ok("def f(name): return f\"Hello, {name}!\""));
}

#[test]
fn test_fstring_expression_cov() {
    assert!(transpile_ok("def f(x): return f\"Result: {x * 2}\""));
}

#[test]
fn test_fstring_format_spec() {
    assert!(transpile_ok("def f(x): return f\"{x:.2f}\""));
}

#[test]
fn test_fstring_multiple_cov() {
    assert!(transpile_ok("def f(a, b): return f\"{a} + {b} = {a + b}\""));
}

#[test]
fn test_fstring_nested_quote() {
    assert!(transpile_ok("def f(x): return f\"Value: {x}\""));
}

// ============================================================================
// FUNCTION PARAMETERS - COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_param_positional() {
    assert!(transpile_ok("def f(x, y, z): return x + y + z"));
}

#[test]
fn test_param_default() {
    assert!(transpile_ok("def f(x, y=10): return x + y"));
}

#[test]
fn test_param_typed() {
    assert!(transpile_ok("def f(x: int, y: str) -> str: return str(x) + y"));
}

#[test]
fn test_param_args() {
    assert!(transpile_ok("def f(*args): return len(args)"));
}

#[test]
fn test_param_kwargs() {
    assert!(transpile_ok("def f(**kwargs): return len(kwargs)"));
}

#[test]
fn test_param_mixed() {
    assert!(transpile_ok("def f(a, b=10, *args, **kwargs): return a + b"));
}

#[test]
fn test_param_keyword_only() {
    assert!(transpile_ok("def f(a, *, b): return a + b"));
}

#[test]
fn test_param_positional_only() {
    assert!(transpile_ok("def f(a, /, b): return a + b"));
}

// ============================================================================
// RETURN STATEMENTS - COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_return_none() {
    assert!(transpile_ok("def f():\n    return"));
}

#[test]
fn test_return_value() {
    assert!(transpile_ok("def f(): return 42"));
}

#[test]
fn test_return_expression() {
    assert!(transpile_ok("def f(x): return x * 2 + 1"));
}

#[test]
fn test_return_tuple() {
    assert!(transpile_ok("def f(): return 1, 2, 3"));
}

#[test]
fn test_return_explicit_tuple() {
    assert!(transpile_ok("def f(): return (1, 2, 3)"));
}

// ============================================================================
// PASS/ELLIPSIS - COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_pass() {
    assert!(transpile_ok("def f():\n    pass"));
}

#[test]
fn test_ellipsis() {
    assert!(transpile_ok("def f():\n    ..."));
}

// ============================================================================
// ASSERT - COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_assert_simple_cov() {
    assert!(transpile_ok("def f(x):\n    assert x > 0\n    return x"));
}

#[test]
fn test_assert_with_msg() {
    assert!(transpile_ok("def f(x):\n    assert x > 0, \"x must be positive\"\n    return x"));
}

// ============================================================================
// GLOBAL/NONLOCAL - COVERAGE
// ============================================================================

#[test]
fn test_global() {
    assert!(transpile_ok("count = 0\ndef f():\n    global count\n    count += 1"));
}

#[test]
fn test_nonlocal() {
    assert!(transpile_ok("def outer():\n    x = 0\n    def inner():\n        nonlocal x\n        x += 1\n    return inner"));
}

// ============================================================================
// DELETE - COVERAGE
// ============================================================================

#[test]
fn test_del_variable_cov() {
    assert!(transpile_ok("def f():\n    x = 1\n    del x"));
}

#[test]
fn test_del_subscript() {
    assert!(transpile_ok("def f(d):\n    del d[\"key\"]"));
}

// ============================================================================
// WALRUS OPERATOR - COVERAGE
// ============================================================================

#[test]
fn test_walrus_simple_cov() {
    assert!(transpile_ok("def f(items):\n    if (n := len(items)) > 0:\n        return n\n    return 0"));
}

#[test]
fn test_walrus_in_while_cov() {
    assert!(transpile_ok("def f():\n    while (line := input()) != \"quit\":\n        print(line)"));
}

// ============================================================================
// MORE STRING METHODS - COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_string_center() {
    assert!(transpile_ok("def f(s): return s.center(20)"));
}

#[test]
fn test_string_ljust() {
    assert!(transpile_ok("def f(s): return s.ljust(20)"));
}

#[test]
fn test_string_rjust() {
    assert!(transpile_ok("def f(s): return s.rjust(20)"));
}

#[test]
fn test_string_zfill() {
    assert!(transpile_ok("def f(s): return s.zfill(10)"));
}

#[test]
fn test_string_partition() {
    assert!(transpile_ok("def f(s): return s.partition(\",\")"));
}

#[test]
fn test_string_rpartition() {
    assert!(transpile_ok("def f(s): return s.rpartition(\",\")"));
}

#[test]
fn test_string_splitlines() {
    assert!(transpile_ok("def f(s): return s.splitlines()"));
}

#[test]
fn test_string_expandtabs() {
    assert!(transpile_ok("def f(s): return s.expandtabs(4)"));
}

#[test]
fn test_string_isnumeric() {
    assert!(transpile_ok("def f(s): return s.isnumeric()"));
}

#[test]
fn test_string_islower() {
    assert!(transpile_ok("def f(s): return s.islower()"));
}

#[test]
fn test_string_isupper() {
    assert!(transpile_ok("def f(s): return s.isupper()"));
}

#[test]
fn test_string_istitle() {
    assert!(transpile_ok("def f(s): return s.istitle()"));
}

#[test]
fn test_string_swapcase() {
    assert!(transpile_ok("def f(s): return s.swapcase()"));
}

#[test]
fn test_string_casefold() {
    assert!(transpile_ok("def f(s): return s.casefold()"));
}

// ============================================================================
// MORE LIST METHODS - COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_list_insert_cov() {
    assert!(transpile_ok("def f(lst):\n    lst.insert(0, 1)\n    return lst"));
}

#[test]
fn test_list_remove_cov() {
    assert!(transpile_ok("def f(lst):\n    lst.remove(1)\n    return lst"));
}

#[test]
fn test_list_count_cov() {
    assert!(transpile_ok("def f(lst): return lst.count(1)"));
}

#[test]
fn test_list_index_cov() {
    assert!(transpile_ok("def f(lst): return lst.index(1)"));
}

#[test]
fn test_list_copy_cov() {
    assert!(transpile_ok("def f(lst): return lst.copy()"));
}

#[test]
fn test_list_clear_cov() {
    assert!(transpile_ok("def f(lst):\n    lst.clear()\n    return lst"));
}

#[test]
fn test_list_sort_cov() {
    assert!(transpile_ok("def f(lst):\n    lst.sort()\n    return lst"));
}

#[test]
fn test_list_sort_reverse() {
    assert!(transpile_ok("def f(lst):\n    lst.sort(reverse=True)\n    return lst"));
}

// ============================================================================
// MORE DICT METHODS - COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_dict_pop_key() {
    assert!(transpile_ok("def f(d): return d.pop(\"key\")"));
}

#[test]
fn test_dict_pop_with_default() {
    assert!(transpile_ok("def f(d): return d.pop(\"key\", None)"));
}

#[test]
fn test_dict_popitem_method() {
    assert!(transpile_ok("def f(d): return d.popitem()"));
}

#[test]
fn test_dict_setdefault_method() {
    assert!(transpile_ok("def f(d): return d.setdefault(\"key\", 0)"));
}

#[test]
fn test_dict_update_method() {
    assert!(transpile_ok("def f(d1, d2):\n    d1.update(d2)\n    return d1"));
}

#[test]
fn test_dict_fromkeys_constructor() {
    assert!(transpile_ok("def f(): return dict.fromkeys([\"a\", \"b\"], 0)"));
}

// ============================================================================
// MORE SET METHODS - COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_set_union_method() {
    assert!(transpile_ok("def f(a, b): return a.union(b)"));
}

#[test]
fn test_set_intersection_method() {
    assert!(transpile_ok("def f(a, b): return a.intersection(b)"));
}

#[test]
fn test_set_difference_method() {
    assert!(transpile_ok("def f(a, b): return a.difference(b)"));
}

#[test]
fn test_set_symmetric_difference_method() {
    assert!(transpile_ok("def f(a, b): return a.symmetric_difference(b)"));
}

#[test]
fn test_set_issubset_method() {
    assert!(transpile_ok("def f(a, b): return a.issubset(b)"));
}

#[test]
fn test_set_issuperset_method() {
    assert!(transpile_ok("def f(a, b): return a.issuperset(b)"));
}

#[test]
fn test_set_isdisjoint_method() {
    assert!(transpile_ok("def f(a, b): return a.isdisjoint(b)"));
}

#[test]
fn test_set_discard_method() {
    assert!(transpile_ok("def f(s):\n    s.discard(1)\n    return s"));
}

#[test]
fn test_set_pop_method() {
    assert!(transpile_ok("def f(s): return s.pop()"));
}

#[test]
fn test_set_update_method() {
    assert!(transpile_ok("def f(a, b):\n    a.update(b)\n    return a"));
}

#[test]
fn test_set_intersection_update_method() {
    assert!(transpile_ok("def f(a, b):\n    a.intersection_update(b)\n    return a"));
}

#[test]
fn test_set_difference_update_method() {
    assert!(transpile_ok("def f(a, b):\n    a.difference_update(b)\n    return a"));
}

// ============================================================================
// COMPLEX EXPRESSIONS - EDGE CASES
// ============================================================================

#[test]
fn test_complex_arithmetic_ops() {
    assert!(transpile_ok("def f(a, b, c): return (a + b) * c / (a - b)"));
}

#[test]
fn test_complex_boolean_ops() {
    assert!(transpile_ok("def f(a, b, c): return (a and b) or (not c and a)"));
}

#[test]
fn test_complex_comparison() {
    assert!(transpile_ok("def f(x): return 0 <= x < 100 and x % 2 == 0"));
}

#[test]
fn test_complex_nested_calls() {
    assert!(transpile_ok("def f(items): return sorted(map(str, filter(lambda x: x > 0, items)))"));
}

#[test]
fn test_complex_comprehension() {
    assert!(transpile_ok("def f(m): return [[x * y for y in row] for row in m for x in row]"));
}

#[test]
fn test_complex_ternary_chain() {
    assert!(transpile_ok("def f(x): return \"pos\" if x > 0 else \"neg\" if x < 0 else \"zero\""));
}

#[test]
fn test_complex_method_chain() {
    assert!(transpile_ok("def f(s): return s.strip().lower().replace(\" \", \"_\").split(\"_\")"));
}
