//! EXTREME TDD Tests for expr_gen.rs
//!
//! These tests aim to:
//! 1. Exercise every code path
//! 2. Test boundary conditions
//! 3. Falsification tests that try to BREAK the code
//! 4. Mutation-resistant tests

use depyler_core::DepylerPipeline;

fn transpile(code: &str) -> Result<String, String> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).map_err(|e| e.to_string())
}

fn transpile_ok(code: &str) -> bool {
    transpile(code).is_ok()
}

#[allow(dead_code)]
fn transpile_contains(code: &str, expected: &str) -> bool {
    match transpile(code) {
        Ok(result) => result.contains(expected),
        Err(_) => false,
    }
}

// ============================================================================
// BINARY OPERATIONS - Comprehensive Coverage
// ============================================================================

mod binary_ops {
    use super::*;

    #[test]
    fn test_add_int_int() {
        assert!(transpile_ok(
            "def f(a: int, b: int) -> int:\n    return a + b"
        ));
    }

    #[test]
    fn test_add_float_float() {
        assert!(transpile_ok(
            "def f(a: float, b: float) -> float:\n    return a + b"
        ));
    }

    #[test]
    fn test_add_int_float_coercion() {
        assert!(transpile_ok(
            "def f(a: int, b: float) -> float:\n    return a + b"
        ));
    }

    #[test]
    fn test_add_string_concat() {
        assert!(transpile_ok(
            "def f(a: str, b: str) -> str:\n    return a + b"
        ));
    }

    #[test]
    fn test_add_list_concat() {
        let code = "def f(a: list, b: list) -> list:\n    return a + b";
        assert!(transpile_ok(code));
    }

    #[test]
    fn test_sub_int_int() {
        assert!(transpile_ok(
            "def f(a: int, b: int) -> int:\n    return a - b"
        ));
    }

    #[test]
    fn test_sub_float_float() {
        assert!(transpile_ok(
            "def f(a: float, b: float) -> float:\n    return a - b"
        ));
    }

    #[test]
    fn test_mul_int_int() {
        assert!(transpile_ok(
            "def f(a: int, b: int) -> int:\n    return a * b"
        ));
    }

    #[test]
    fn test_mul_string_repeat() {
        assert!(transpile_ok(
            "def f(s: str, n: int) -> str:\n    return s * n"
        ));
    }

    #[test]
    fn test_mul_list_repeat() {
        assert!(transpile_ok(
            "def f(lst: list, n: int) -> list:\n    return lst * n"
        ));
    }

    #[test]
    fn test_div_float() {
        assert!(transpile_ok(
            "def f(a: float, b: float) -> float:\n    return a / b"
        ));
    }

    #[test]
    fn test_floordiv_int() {
        assert!(transpile_ok(
            "def f(a: int, b: int) -> int:\n    return a // b"
        ));
    }

    #[test]
    fn test_mod_int() {
        assert!(transpile_ok(
            "def f(a: int, b: int) -> int:\n    return a % b"
        ));
    }

    #[test]
    fn test_pow_int() {
        assert!(transpile_ok(
            "def f(a: int, b: int) -> int:\n    return a ** b"
        ));
    }

    #[test]
    fn test_pow_float() {
        assert!(transpile_ok(
            "def f(a: float, b: float) -> float:\n    return a ** b"
        ));
    }

    #[test]
    fn test_bitand() {
        assert!(transpile_ok(
            "def f(a: int, b: int) -> int:\n    return a & b"
        ));
    }

    #[test]
    fn test_bitor() {
        assert!(transpile_ok(
            "def f(a: int, b: int) -> int:\n    return a | b"
        ));
    }

    #[test]
    fn test_bitxor() {
        assert!(transpile_ok(
            "def f(a: int, b: int) -> int:\n    return a ^ b"
        ));
    }

    #[test]
    fn test_lshift() {
        assert!(transpile_ok(
            "def f(a: int, b: int) -> int:\n    return a << b"
        ));
    }

    #[test]
    fn test_rshift() {
        assert!(transpile_ok(
            "def f(a: int, b: int) -> int:\n    return a >> b"
        ));
    }

    #[test]
    fn test_eq_int() {
        assert!(transpile_ok(
            "def f(a: int, b: int) -> bool:\n    return a == b"
        ));
    }

    #[test]
    fn test_ne_int() {
        assert!(transpile_ok(
            "def f(a: int, b: int) -> bool:\n    return a != b"
        ));
    }

    #[test]
    fn test_lt_int() {
        assert!(transpile_ok(
            "def f(a: int, b: int) -> bool:\n    return a < b"
        ));
    }

    #[test]
    fn test_le_int() {
        assert!(transpile_ok(
            "def f(a: int, b: int) -> bool:\n    return a <= b"
        ));
    }

    #[test]
    fn test_gt_int() {
        assert!(transpile_ok(
            "def f(a: int, b: int) -> bool:\n    return a > b"
        ));
    }

    #[test]
    fn test_ge_int() {
        assert!(transpile_ok(
            "def f(a: int, b: int) -> bool:\n    return a >= b"
        ));
    }

    #[test]
    fn test_and_bool() {
        assert!(transpile_ok(
            "def f(a: bool, b: bool) -> bool:\n    return a and b"
        ));
    }

    #[test]
    fn test_or_bool() {
        assert!(transpile_ok(
            "def f(a: bool, b: bool) -> bool:\n    return a or b"
        ));
    }

    #[test]
    fn test_in_list() {
        assert!(transpile_ok(
            "def f(x: int, lst: list) -> bool:\n    return x in lst"
        ));
    }

    #[test]
    fn test_not_in_list() {
        assert!(transpile_ok(
            "def f(x: int, lst: list) -> bool:\n    return x not in lst"
        ));
    }

    #[test]
    fn test_in_string() {
        assert!(transpile_ok(
            "def f(sub: str, s: str) -> bool:\n    return sub in s"
        ));
    }

    #[test]
    fn test_in_dict() {
        assert!(transpile_ok(
            "def f(key: str, d: dict) -> bool:\n    return key in d"
        ));
    }

    #[test]
    fn test_chained_comparison() {
        assert!(transpile_ok(
            "def f(a: int, b: int, c: int) -> bool:\n    return a < b < c"
        ));
    }

    #[test]
    fn test_complex_expression_precedence() {
        assert!(transpile_ok(
            "def f(a: int, b: int, c: int) -> int:\n    return a + b * c"
        ));
    }

    #[test]
    fn test_parenthesized_expr() {
        assert!(transpile_ok(
            "def f(a: int, b: int, c: int) -> int:\n    return (a + b) * c"
        ));
    }
}

// ============================================================================
// UNARY OPERATIONS - Comprehensive Coverage
// ============================================================================

mod unary_ops {
    use super::*;

    #[test]
    fn test_not_bool() {
        assert!(transpile_ok("def f(a: bool) -> bool:\n    return not a"));
    }

    #[test]
    fn test_neg_int() {
        assert!(transpile_ok("def f(a: int) -> int:\n    return -a"));
    }

    #[test]
    fn test_neg_float() {
        assert!(transpile_ok("def f(a: float) -> float:\n    return -a"));
    }

    #[test]
    fn test_pos_int() {
        assert!(transpile_ok("def f(a: int) -> int:\n    return +a"));
    }

    #[test]
    fn test_invert_int() {
        assert!(transpile_ok("def f(a: int) -> int:\n    return ~a"));
    }

    #[test]
    fn test_double_neg() {
        assert!(transpile_ok("def f(a: int) -> int:\n    return --a"));
    }

    #[test]
    fn test_not_not() {
        assert!(transpile_ok(
            "def f(a: bool) -> bool:\n    return not not a"
        ));
    }
}

// ============================================================================
// BUILTIN FUNCTION CALLS - Comprehensive Coverage
// ============================================================================

mod builtins {
    use super::*;

    #[test]
    fn test_len_list() {
        assert!(transpile_ok(
            "def f(lst: list) -> int:\n    return len(lst)"
        ));
    }

    #[test]
    fn test_len_string() {
        assert!(transpile_ok("def f(s: str) -> int:\n    return len(s)"));
    }

    #[test]
    fn test_len_dict() {
        assert!(transpile_ok("def f(d: dict) -> int:\n    return len(d)"));
    }

    #[test]
    fn test_range_one_arg() {
        assert!(transpile_ok(
            "def f(n: int):\n    for i in range(n):\n        print(i)"
        ));
    }

    #[test]
    fn test_range_two_args() {
        assert!(transpile_ok(
            "def f(start: int, stop: int):\n    for i in range(start, stop):\n        print(i)"
        ));
    }

    #[test]
    fn test_range_three_args() {
        assert!(transpile_ok("def f(start: int, stop: int, step: int):\n    for i in range(start, stop, step):\n        print(i)"));
    }

    #[test]
    fn test_int_cast() {
        assert!(transpile_ok("def f(s: str) -> int:\n    return int(s)"));
    }

    #[test]
    fn test_int_cast_float() {
        assert!(transpile_ok("def f(x: float) -> int:\n    return int(x)"));
    }

    #[test]
    fn test_float_cast() {
        assert!(transpile_ok("def f(s: str) -> float:\n    return float(s)"));
    }

    #[test]
    fn test_float_cast_int() {
        assert!(transpile_ok("def f(x: int) -> float:\n    return float(x)"));
    }

    #[test]
    fn test_str_cast() {
        assert!(transpile_ok("def f(x: int) -> str:\n    return str(x)"));
    }

    #[test]
    fn test_bool_cast() {
        assert!(transpile_ok("def f(x: int) -> bool:\n    return bool(x)"));
    }

    #[test]
    fn test_abs_int() {
        assert!(transpile_ok("def f(x: int) -> int:\n    return abs(x)"));
    }

    #[test]
    fn test_abs_float() {
        assert!(transpile_ok("def f(x: float) -> float:\n    return abs(x)"));
    }

    #[test]
    fn test_min_two_args() {
        assert!(transpile_ok(
            "def f(a: int, b: int) -> int:\n    return min(a, b)"
        ));
    }

    #[test]
    fn test_min_list() {
        assert!(transpile_ok(
            "def f(lst: list) -> int:\n    return min(lst)"
        ));
    }

    #[test]
    fn test_max_two_args() {
        assert!(transpile_ok(
            "def f(a: int, b: int) -> int:\n    return max(a, b)"
        ));
    }

    #[test]
    fn test_max_list() {
        assert!(transpile_ok(
            "def f(lst: list) -> int:\n    return max(lst)"
        ));
    }

    #[test]
    fn test_sum_list() {
        assert!(transpile_ok(
            "def f(lst: list) -> int:\n    return sum(lst)"
        ));
    }

    #[test]
    fn test_sum_with_start() {
        assert!(transpile_ok(
            "def f(lst: list) -> int:\n    return sum(lst, 10)"
        ));
    }

    #[test]
    fn test_round_one_arg() {
        assert!(transpile_ok("def f(x: float) -> int:\n    return round(x)"));
    }

    #[test]
    fn test_round_two_args() {
        assert!(transpile_ok(
            "def f(x: float) -> float:\n    return round(x, 2)"
        ));
    }

    #[test]
    fn test_pow_builtin() {
        assert!(transpile_ok(
            "def f(x: int, y: int) -> int:\n    return pow(x, y)"
        ));
    }

    #[test]
    fn test_divmod() {
        assert!(transpile_ok(
            "def f(a: int, b: int):\n    q, r = divmod(a, b)"
        ));
    }

    #[test]
    fn test_enumerate() {
        assert!(transpile_ok(
            "def f(lst: list):\n    for i, x in enumerate(lst):\n        print(i, x)"
        ));
    }

    #[test]
    fn test_enumerate_with_start() {
        assert!(transpile_ok(
            "def f(lst: list):\n    for i, x in enumerate(lst, 1):\n        print(i, x)"
        ));
    }

    #[test]
    fn test_zip_two_lists() {
        assert!(transpile_ok(
            "def f(a: list, b: list):\n    for x, y in zip(a, b):\n        print(x, y)"
        ));
    }

    #[test]
    fn test_reversed() {
        assert!(transpile_ok(
            "def f(lst: list):\n    for x in reversed(lst):\n        print(x)"
        ));
    }

    #[test]
    fn test_sorted() {
        assert!(transpile_ok(
            "def f(lst: list) -> list:\n    return sorted(lst)"
        ));
    }

    #[test]
    fn test_sorted_reverse() {
        assert!(transpile_ok(
            "def f(lst: list) -> list:\n    return sorted(lst, reverse=True)"
        ));
    }

    #[test]
    fn test_filter_lambda() {
        assert!(transpile_ok(
            "def f(lst: list) -> list:\n    return list(filter(lambda x: x > 0, lst))"
        ));
    }

    #[test]
    fn test_any_list() {
        assert!(transpile_ok(
            "def f(lst: list) -> bool:\n    return any(lst)"
        ));
    }

    #[test]
    fn test_all_list() {
        assert!(transpile_ok(
            "def f(lst: list) -> bool:\n    return all(lst)"
        ));
    }

    #[test]
    fn test_hex() {
        assert!(transpile_ok("def f(x: int) -> str:\n    return hex(x)"));
    }

    #[test]
    fn test_bin() {
        assert!(transpile_ok("def f(x: int) -> str:\n    return bin(x)"));
    }

    #[test]
    fn test_oct() {
        assert!(transpile_ok("def f(x: int) -> str:\n    return oct(x)"));
    }

    #[test]
    fn test_chr() {
        assert!(transpile_ok("def f(x: int) -> str:\n    return chr(x)"));
    }

    #[test]
    fn test_ord() {
        assert!(transpile_ok("def f(s: str) -> int:\n    return ord(s)"));
    }

    #[test]
    fn test_repr() {
        assert!(transpile_ok("def f(x: int) -> str:\n    return repr(x)"));
    }

    #[test]
    fn test_hash() {
        assert!(transpile_ok("def f(s: str) -> int:\n    return hash(s)"));
    }

    #[test]
    fn test_format_builtin() {
        assert!(transpile_ok(
            "def f(x: int) -> str:\n    return format(x, '08b')"
        ));
    }

    #[test]
    fn test_print_no_args() {
        assert!(transpile_ok("def f():\n    print()"));
    }

    #[test]
    fn test_print_one_arg() {
        assert!(transpile_ok("def f(x: int):\n    print(x)"));
    }

    #[test]
    fn test_print_multiple_args() {
        assert!(transpile_ok("def f(a: int, b: str):\n    print(a, b)"));
    }

    #[test]
    fn test_print_sep() {
        assert!(transpile_ok(
            "def f(a: int, b: int):\n    print(a, b, sep=', ')"
        ));
    }

    #[test]
    fn test_print_end() {
        assert!(transpile_ok("def f(x: int):\n    print(x, end='')"));
    }

    #[test]
    fn test_input_no_prompt() {
        assert!(transpile_ok("def f() -> str:\n    return input()"));
    }

    #[test]
    fn test_input_with_prompt() {
        assert!(transpile_ok("def f() -> str:\n    return input('Enter: ')"));
    }

    #[test]
    fn test_open_read() {
        assert!(transpile_ok(
            "def f(path: str):\n    with open(path, 'r') as f:\n        return f.read()"
        ));
    }

    #[test]
    fn test_open_write() {
        assert!(transpile_ok(
            "def f(path: str, data: str):\n    with open(path, 'w') as f:\n        f.write(data)"
        ));
    }
}

// ============================================================================
// COLLECTION CONSTRUCTORS - Comprehensive Coverage
// ============================================================================

mod collections {
    use super::*;

    #[test]
    fn test_list_empty() {
        assert!(transpile_ok("def f() -> list:\n    return []"));
    }

    #[test]
    fn test_list_literal() {
        assert!(transpile_ok("def f() -> list:\n    return [1, 2, 3]"));
    }

    #[test]
    fn test_list_constructor_empty() {
        assert!(transpile_ok("def f() -> list:\n    return list()"));
    }

    #[test]
    fn test_list_constructor_iterable() {
        assert!(transpile_ok("def f(s: str) -> list:\n    return list(s)"));
    }

    #[test]
    fn test_dict_empty() {
        assert!(transpile_ok("def f() -> dict:\n    return {}"));
    }

    #[test]
    fn test_dict_literal() {
        assert!(transpile_ok(
            "def f() -> dict:\n    return {'a': 1, 'b': 2}"
        ));
    }

    #[test]
    fn test_dict_constructor() {
        assert!(transpile_ok("def f() -> dict:\n    return dict()"));
    }

    #[test]
    fn test_set_literal() {
        assert!(transpile_ok("def f() -> set:\n    return {1, 2, 3}"));
    }

    #[test]
    fn test_set_constructor() {
        assert!(transpile_ok("def f() -> set:\n    return set()"));
    }

    #[test]
    fn test_set_from_list() {
        assert!(transpile_ok(
            "def f(lst: list) -> set:\n    return set(lst)"
        ));
    }

    #[test]
    fn test_tuple_literal() {
        assert!(transpile_ok("def f():\n    return (1, 2, 3)"));
    }

    #[test]
    fn test_tuple_constructor() {
        assert!(transpile_ok("def f(lst: list):\n    return tuple(lst)"));
    }

    #[test]
    fn test_frozenset() {
        assert!(transpile_ok(
            "def f() -> frozenset:\n    return frozenset([1, 2, 3])"
        ));
    }

    #[test]
    fn test_bytes_literal() {
        assert!(transpile_ok("def f() -> bytes:\n    return b'hello'"));
    }

    #[test]
    fn test_bytes_constructor() {
        assert!(transpile_ok(
            "def f() -> bytes:\n    return bytes([65, 66, 67])"
        ));
    }

    #[test]
    fn test_bytearray_constructor() {
        assert!(transpile_ok(
            "def f() -> bytearray:\n    return bytearray(10)"
        ));
    }
}

// ============================================================================
// METHOD CALLS - Comprehensive Coverage
// ============================================================================

mod method_calls {
    use super::*;

    // String methods
    #[test]
    fn test_str_upper() {
        assert!(transpile_ok("def f(s: str) -> str:\n    return s.upper()"));
    }

    #[test]
    fn test_str_lower() {
        assert!(transpile_ok("def f(s: str) -> str:\n    return s.lower()"));
    }

    #[test]
    fn test_str_strip() {
        assert!(transpile_ok("def f(s: str) -> str:\n    return s.strip()"));
    }

    #[test]
    fn test_str_split() {
        assert!(transpile_ok("def f(s: str) -> list:\n    return s.split()"));
    }

    #[test]
    fn test_str_split_sep() {
        assert!(transpile_ok(
            "def f(s: str) -> list:\n    return s.split(',')"
        ));
    }

    #[test]
    fn test_str_join() {
        assert!(transpile_ok(
            "def f(lst: list) -> str:\n    return ', '.join(lst)"
        ));
    }

    #[test]
    fn test_str_replace() {
        assert!(transpile_ok(
            "def f(s: str) -> str:\n    return s.replace('a', 'b')"
        ));
    }

    #[test]
    fn test_str_startswith() {
        assert!(transpile_ok(
            "def f(s: str) -> bool:\n    return s.startswith('pre')"
        ));
    }

    #[test]
    fn test_str_endswith() {
        assert!(transpile_ok(
            "def f(s: str) -> bool:\n    return s.endswith('suf')"
        ));
    }

    #[test]
    fn test_str_find() {
        assert!(transpile_ok(
            "def f(s: str) -> int:\n    return s.find('sub')"
        ));
    }

    #[test]
    fn test_str_count() {
        assert!(transpile_ok(
            "def f(s: str) -> int:\n    return s.count('a')"
        ));
    }

    #[test]
    fn test_str_isdigit() {
        assert!(transpile_ok(
            "def f(s: str) -> bool:\n    return s.isdigit()"
        ));
    }

    #[test]
    fn test_str_isalpha() {
        assert!(transpile_ok(
            "def f(s: str) -> bool:\n    return s.isalpha()"
        ));
    }

    #[test]
    fn test_str_format() {
        assert!(transpile_ok(
            "def f(name: str) -> str:\n    return 'Hello, {}'.format(name)"
        ));
    }

    // List methods
    #[test]
    fn test_list_append() {
        assert!(transpile_ok("def f(lst: list, x: int):\n    lst.append(x)"));
    }

    #[test]
    fn test_list_extend() {
        assert!(transpile_ok("def f(a: list, b: list):\n    a.extend(b)"));
    }

    #[test]
    fn test_list_insert() {
        assert!(transpile_ok(
            "def f(lst: list, x: int):\n    lst.insert(0, x)"
        ));
    }

    #[test]
    fn test_list_remove() {
        assert!(transpile_ok("def f(lst: list, x: int):\n    lst.remove(x)"));
    }

    #[test]
    fn test_list_pop() {
        assert!(transpile_ok(
            "def f(lst: list) -> int:\n    return lst.pop()"
        ));
    }

    #[test]
    fn test_list_pop_index() {
        assert!(transpile_ok(
            "def f(lst: list) -> int:\n    return lst.pop(0)"
        ));
    }

    #[test]
    fn test_list_clear() {
        assert!(transpile_ok("def f(lst: list):\n    lst.clear()"));
    }

    #[test]
    fn test_list_index() {
        assert!(transpile_ok(
            "def f(lst: list, x: int) -> int:\n    return lst.index(x)"
        ));
    }

    #[test]
    fn test_list_count() {
        assert!(transpile_ok(
            "def f(lst: list, x: int) -> int:\n    return lst.count(x)"
        ));
    }

    #[test]
    fn test_list_sort() {
        assert!(transpile_ok("def f(lst: list):\n    lst.sort()"));
    }

    #[test]
    fn test_list_reverse() {
        assert!(transpile_ok("def f(lst: list):\n    lst.reverse()"));
    }

    #[test]
    fn test_list_copy() {
        assert!(transpile_ok(
            "def f(lst: list) -> list:\n    return lst.copy()"
        ));
    }

    // Dict methods
    #[test]
    fn test_dict_get() {
        assert!(transpile_ok(
            "def f(d: dict, key: str):\n    return d.get(key)"
        ));
    }

    #[test]
    fn test_dict_get_default() {
        assert!(transpile_ok(
            "def f(d: dict, key: str):\n    return d.get(key, 0)"
        ));
    }

    #[test]
    fn test_dict_keys() {
        assert!(transpile_ok("def f(d: dict):\n    return d.keys()"));
    }

    #[test]
    fn test_dict_values() {
        assert!(transpile_ok("def f(d: dict):\n    return d.values()"));
    }

    #[test]
    fn test_dict_items() {
        assert!(transpile_ok(
            "def f(d: dict):\n    for k, v in d.items():\n        print(k, v)"
        ));
    }

    #[test]
    fn test_dict_pop() {
        assert!(transpile_ok(
            "def f(d: dict, key: str):\n    return d.pop(key)"
        ));
    }

    #[test]
    fn test_dict_update() {
        assert!(transpile_ok("def f(a: dict, b: dict):\n    a.update(b)"));
    }

    #[test]
    fn test_dict_clear() {
        assert!(transpile_ok("def f(d: dict):\n    d.clear()"));
    }

    #[test]
    fn test_dict_setdefault() {
        assert!(transpile_ok(
            "def f(d: dict, key: str):\n    return d.setdefault(key, 0)"
        ));
    }

    // Set methods
    #[test]
    fn test_set_add() {
        assert!(transpile_ok("def f(s: set, x: int):\n    s.add(x)"));
    }

    #[test]
    fn test_set_remove() {
        assert!(transpile_ok("def f(s: set, x: int):\n    s.remove(x)"));
    }

    #[test]
    fn test_set_discard() {
        assert!(transpile_ok("def f(s: set, x: int):\n    s.discard(x)"));
    }

    #[test]
    fn test_set_union() {
        assert!(transpile_ok(
            "def f(a: set, b: set) -> set:\n    return a.union(b)"
        ));
    }

    #[test]
    fn test_set_intersection() {
        assert!(transpile_ok(
            "def f(a: set, b: set) -> set:\n    return a.intersection(b)"
        ));
    }

    #[test]
    fn test_set_difference() {
        assert!(transpile_ok(
            "def f(a: set, b: set) -> set:\n    return a.difference(b)"
        ));
    }
}

// ============================================================================
// SUBSCRIPT AND ATTRIBUTE ACCESS - Comprehensive Coverage
// ============================================================================

mod access {
    use super::*;

    #[test]
    fn test_list_index_access() {
        assert!(transpile_ok("def f(lst: list) -> int:\n    return lst[0]"));
    }

    #[test]
    fn test_list_negative_index() {
        assert!(transpile_ok("def f(lst: list) -> int:\n    return lst[-1]"));
    }

    #[test]
    fn test_list_slice() {
        assert!(transpile_ok(
            "def f(lst: list) -> list:\n    return lst[1:3]"
        ));
    }

    #[test]
    fn test_list_slice_step() {
        assert!(transpile_ok(
            "def f(lst: list) -> list:\n    return lst[::2]"
        ));
    }

    #[test]
    fn test_list_slice_reverse() {
        assert!(transpile_ok(
            "def f(lst: list) -> list:\n    return lst[::-1]"
        ));
    }

    #[test]
    fn test_dict_key_access() {
        assert!(transpile_ok("def f(d: dict) -> int:\n    return d['key']"));
    }

    #[test]
    fn test_string_index() {
        assert!(transpile_ok("def f(s: str) -> str:\n    return s[0]"));
    }

    #[test]
    fn test_string_slice() {
        assert!(transpile_ok("def f(s: str) -> str:\n    return s[1:4]"));
    }

    #[test]
    fn test_nested_subscript() {
        assert!(transpile_ok(
            "def f(matrix: list) -> int:\n    return matrix[0][0]"
        ));
    }

    #[test]
    fn test_attribute_access() {
        assert!(transpile_ok("class Foo:\n    def __init__(self):\n        self.x = 0\ndef f(obj: Foo) -> int:\n    return obj.x"));
    }
}

// ============================================================================
// COMPREHENSIONS - Comprehensive Coverage
// ============================================================================

mod comprehensions {
    use super::*;

    #[test]
    fn test_list_comp_simple() {
        assert!(transpile_ok(
            "def f(lst: list) -> list:\n    return [x * 2 for x in lst]"
        ));
    }

    #[test]
    fn test_list_comp_with_if() {
        assert!(transpile_ok(
            "def f(lst: list) -> list:\n    return [x for x in lst if x > 0]"
        ));
    }

    #[test]
    fn test_list_comp_nested() {
        assert!(transpile_ok(
            "def f(matrix: list) -> list:\n    return [x for row in matrix for x in row]"
        ));
    }

    #[test]
    fn test_dict_comp() {
        assert!(transpile_ok(
            "def f(lst: list) -> dict:\n    return {x: x*2 for x in lst}"
        ));
    }

    #[test]
    fn test_dict_comp_with_if() {
        assert!(transpile_ok(
            "def f(lst: list) -> dict:\n    return {x: x*2 for x in lst if x > 0}"
        ));
    }

    #[test]
    fn test_set_comp() {
        assert!(transpile_ok(
            "def f(lst: list) -> set:\n    return {x * 2 for x in lst}"
        ));
    }

    #[test]
    fn test_generator_expr() {
        assert!(transpile_ok(
            "def f(lst: list) -> int:\n    return sum(x * 2 for x in lst)"
        ));
    }

    #[test]
    fn test_generator_with_if() {
        assert!(transpile_ok(
            "def f(lst: list) -> int:\n    return sum(x for x in lst if x > 0)"
        ));
    }
}

// ============================================================================
// LAMBDA EXPRESSIONS - Comprehensive Coverage
// ============================================================================

mod lambdas {
    use super::*;

    #[test]
    fn test_lambda_simple() {
        assert!(transpile_ok(
            "def f():\n    fn = lambda x: x * 2\n    return fn(5)"
        ));
    }

    #[test]
    fn test_lambda_two_params() {
        assert!(transpile_ok(
            "def f():\n    fn = lambda x, y: x + y\n    return fn(2, 3)"
        ));
    }

    #[test]
    fn test_lambda_no_params() {
        assert!(transpile_ok(
            "def f():\n    fn = lambda: 42\n    return fn()"
        ));
    }

    #[test]
    fn test_lambda_in_sorted() {
        assert!(transpile_ok(
            "def f(lst: list) -> list:\n    return sorted(lst, key=lambda x: -x)"
        ));
    }

    #[test]
    fn test_lambda_in_map() {
        assert!(transpile_ok(
            "def f(lst: list) -> list:\n    return list(map(lambda x: x * 2, lst))"
        ));
    }

    #[test]
    fn test_lambda_in_filter() {
        assert!(transpile_ok(
            "def f(lst: list) -> list:\n    return list(filter(lambda x: x > 0, lst))"
        ));
    }
}

// ============================================================================
// CONDITIONAL EXPRESSIONS (TERNARY) - Comprehensive Coverage
// ============================================================================

mod ternary {
    use super::*;

    #[test]
    fn test_simple_ternary() {
        assert!(transpile_ok(
            "def f(x: int) -> str:\n    return 'pos' if x > 0 else 'neg'"
        ));
    }

    #[test]
    fn test_nested_ternary() {
        assert!(transpile_ok(
            "def f(x: int) -> str:\n    return 'pos' if x > 0 else ('zero' if x == 0 else 'neg')"
        ));
    }

    #[test]
    fn test_ternary_with_call() {
        assert!(transpile_ok(
            "def f(x: int) -> int:\n    return abs(x) if x < 0 else x"
        ));
    }
}

// ============================================================================
// WALRUS OPERATOR - Comprehensive Coverage
// ============================================================================

mod walrus {
    use super::*;

    #[test]
    fn test_walrus_in_if() {
        assert!(transpile_ok(
            "def f(lst: list):\n    if (n := len(lst)) > 0:\n        print(n)"
        ));
    }

    #[test]
    fn test_walrus_in_while() {
        assert!(transpile_ok(
            "def f():\n    data = [1, 2, 3]\n    while (x := data.pop()) != 1:\n        print(x)"
        ));
    }

    #[test]
    fn test_walrus_in_comprehension() {
        assert!(transpile_ok(
            "def f(lst: list) -> list:\n    return [y for x in lst if (y := x * 2) > 5]"
        ));
    }
}

// ============================================================================
// F-STRINGS - Comprehensive Coverage
// ============================================================================

mod fstrings {
    use super::*;

    #[test]
    fn test_fstring_simple() {
        assert!(transpile_ok(
            "def f(name: str) -> str:\n    return f'Hello, {name}'"
        ));
    }

    #[test]
    fn test_fstring_expr() {
        assert!(transpile_ok(
            "def f(x: int) -> str:\n    return f'Result: {x * 2}'"
        ));
    }

    #[test]
    fn test_fstring_multiple() {
        assert!(transpile_ok(
            "def f(a: int, b: str) -> str:\n    return f'{a} and {b}'"
        ));
    }

    #[test]
    fn test_fstring_format_spec() {
        assert!(transpile_ok(
            "def f(x: float) -> str:\n    return f'{x:.2f}'"
        ));
    }

    #[test]
    fn test_fstring_nested_braces() {
        assert!(transpile_ok(
            "def f() -> str:\n    return f'Value: {{escaped}}'"
        ));
    }
}

// ============================================================================
// EDGE CASES AND BOUNDARY CONDITIONS
// ============================================================================

mod edge_cases {
    use super::*;

    #[test]
    fn test_empty_function() {
        assert!(transpile_ok("def f():\n    pass"));
    }

    #[test]
    fn test_deeply_nested_expr() {
        assert!(transpile_ok(
            "def f(a: int) -> int:\n    return ((((a + 1) * 2) - 3) // 4) % 5"
        ));
    }

    #[test]
    fn test_very_long_chain() {
        assert!(transpile_ok(
            "def f(s: str) -> str:\n    return s.strip().lower().replace('a', 'b').upper()"
        ));
    }

    #[test]
    fn test_complex_subscript() {
        assert!(transpile_ok(
            "def f(data: dict):\n    return data['outer']['inner'][0]"
        ));
    }

    #[test]
    fn test_keyword_variable_name() {
        // Python allows 'type' as variable, Rust doesn't
        assert!(transpile_ok("def f(type_: str) -> str:\n    return type_"));
    }

    #[test]
    fn test_underscore_variable() {
        assert!(transpile_ok("def f():\n    _ = 5"));
    }

    #[test]
    fn test_multiple_targets_assign() {
        assert!(transpile_ok("def f():\n    a = b = c = 1"));
    }

    #[test]
    fn test_starred_unpack() {
        assert!(transpile_ok("def f(lst: list):\n    first, *rest = lst"));
    }

    #[test]
    fn test_large_int_literal() {
        assert!(transpile_ok(
            "def f() -> int:\n    return 9999999999999999999"
        ));
    }

    #[test]
    fn test_negative_float() {
        assert!(transpile_ok("def f() -> float:\n    return -3.14159"));
    }

    #[test]
    fn test_scientific_notation() {
        assert!(transpile_ok("def f() -> float:\n    return 1.23e-10"));
    }

    #[test]
    fn test_multiline_string() {
        assert!(transpile_ok(
            "def f() -> str:\n    return '''line1\nline2'''"
        ));
    }

    #[test]
    fn test_raw_string() {
        assert!(transpile_ok("def f() -> str:\n    return r'\\n\\t'"));
    }

    #[test]
    fn test_boolean_literals() {
        assert!(transpile_ok("def f() -> bool:\n    return True"));
    }

    #[test]
    fn test_none_literal() {
        assert!(transpile_ok("def f():\n    return None"));
    }

    #[test]
    fn test_ellipsis() {
        assert!(transpile_ok("def f():\n    ..."));
    }
}

// ============================================================================
// FALSIFICATION TESTS - Try to BREAK the code
// ============================================================================

mod falsification {
    use super::*;

    #[test]
    fn test_invalid_syntax_rejected() {
        assert!(!transpile_ok("def f(\n    invalid syntax here"));
    }

    #[test]
    fn test_incomplete_expr_rejected() {
        assert!(!transpile_ok("def f():\n    return 1 +"));
    }

    #[test]
    fn test_mismatched_parens_rejected() {
        assert!(!transpile_ok("def f():\n    return ((1 + 2)"));
    }

    #[test]
    fn test_mismatched_brackets_rejected() {
        assert!(!transpile_ok("def f():\n    return [1, 2, 3"));
    }

    #[test]
    fn test_invalid_augmented_assign() {
        // This should parse but might cause issues
        let _ = transpile("def f():\n    1 += 2");
    }

    #[test]
    fn test_deeply_nested_doesnt_stack_overflow() {
        let mut code = "def f(x: int) -> int:\n    return ".to_string();
        for _ in 0..50 {
            code.push('(');
        }
        code.push('x');
        for _ in 0..50 {
            code.push_str(" + 1)");
        }
        // Should not stack overflow
        let _ = transpile(&code);
    }

    #[test]
    fn test_very_long_string_literal() {
        let long_str = "a".repeat(10000);
        let code = format!("def f() -> str:\n    return '{}'", long_str);
        // Should handle long strings
        let _ = transpile(&code);
    }

    #[test]
    fn test_unicode_identifiers() {
        // Python allows unicode identifiers
        let _ = transpile("def f(名前: str) -> str:\n    return 名前");
    }

    #[test]
    fn test_empty_list_operations() {
        assert!(transpile_ok("def f() -> int:\n    return len([])"));
    }

    #[test]
    fn test_empty_dict_operations() {
        assert!(transpile_ok("def f() -> int:\n    return len({})"));
    }
}
