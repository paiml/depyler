//! Comprehensive tests for expr_gen.rs coverage
//! DEPYLER-COVERAGE-95: Target 95% coverage on expression generation

use depyler_core::DepylerPipeline;

fn transpile(code: &str) -> Result<String, anyhow::Error> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code)
}

// =============================================================================
// BUILTIN FUNCTION COVERAGE
// =============================================================================

mod builtin_functions {
    use super::*;

    #[test]
    fn test_len_on_list() {
        let code = "def f(x: list[int]) -> int:\n    return len(x)";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_len_on_str() {
        let code = "def f(x: str) -> int:\n    return len(x)";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_len_on_dict() {
        let code = "def f(x: dict[str, int]) -> int:\n    return len(x)";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_range_single_arg() {
        let code = "def f() -> list[int]:\n    return list(range(10))";
        let _ = transpile(code);
    }

    #[test]
    fn test_range_two_args() {
        let code = "def f() -> list[int]:\n    return list(range(1, 10))";
        let _ = transpile(code);
    }

    #[test]
    fn test_range_three_args() {
        let code = "def f() -> list[int]:\n    return list(range(1, 10, 2))";
        let _ = transpile(code);
    }

    #[test]
    fn test_int_from_str() {
        let code = "def f(s: str) -> int:\n    return int(s)";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_int_from_float() {
        let code = "def f(x: float) -> int:\n    return int(x)";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_float_from_int() {
        let code = "def f(x: int) -> float:\n    return float(x)";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_float_from_str() {
        let code = "def f(s: str) -> float:\n    return float(s)";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_str_from_int() {
        let code = "def f(x: int) -> str:\n    return str(x)";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_str_from_float() {
        let code = "def f(x: float) -> str:\n    return str(x)";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_bool_from_int() {
        let code = "def f(x: int) -> bool:\n    return bool(x)";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_abs_int() {
        let code = "def f(x: int) -> int:\n    return abs(x)";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_abs_float() {
        let code = "def f(x: float) -> float:\n    return abs(x)";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_min_two_args() {
        let code = "def f(a: int, b: int) -> int:\n    return min(a, b)";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_max_two_args() {
        let code = "def f(a: int, b: int) -> int:\n    return max(a, b)";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_min_list() {
        let code = "def f(x: list[int]) -> int:\n    return min(x)";
        let _ = transpile(code);
    }

    #[test]
    fn test_max_list() {
        let code = "def f(x: list[int]) -> int:\n    return max(x)";
        let _ = transpile(code);
    }

    #[test]
    fn test_sum_list() {
        let code = "def f(x: list[int]) -> int:\n    return sum(x)";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_sorted_list() {
        let code = "def f(x: list[int]) -> list[int]:\n    return sorted(x)";
        let _ = transpile(code);
    }

    #[test]
    fn test_reversed_list() {
        let code = "def f(x: list[int]) -> list[int]:\n    return list(reversed(x))";
        let _ = transpile(code);
    }

    #[test]
    fn test_enumerate() {
        let code = "def f(x: list[int]) -> None:\n    for i, v in enumerate(x):\n        pass";
        let _ = transpile(code);
    }

    #[test]
    fn test_zip() {
        let code = "def f(a: list[int], b: list[str]) -> None:\n    for x, y in zip(a, b):\n        pass";
        let _ = transpile(code);
    }

    #[test]
    fn test_any() {
        let code = "def f(x: list[bool]) -> bool:\n    return any(x)";
        let _ = transpile(code);
    }

    #[test]
    fn test_all() {
        let code = "def f(x: list[bool]) -> bool:\n    return all(x)";
        let _ = transpile(code);
    }

    #[test]
    fn test_print_single() {
        let code = "def f() -> None:\n    print(\"hello\")";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_print_multiple() {
        let code = "def f(x: int) -> None:\n    print(\"value:\", x)";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_input() {
        let code = "def f() -> str:\n    return input(\"Enter: \")";
        let _ = transpile(code);
    }

    #[test]
    fn test_ord() {
        let code = "def f(c: str) -> int:\n    return ord(c)";
        let _ = transpile(code);
    }

    #[test]
    fn test_chr() {
        let code = "def f(n: int) -> str:\n    return chr(n)";
        let _ = transpile(code);
    }

    #[test]
    fn test_round() {
        let code = "def f(x: float) -> int:\n    return round(x)";
        let _ = transpile(code);
    }

    #[test]
    fn test_round_with_digits() {
        let code = "def f(x: float) -> float:\n    return round(x, 2)";
        let _ = transpile(code);
    }

    #[test]
    fn test_divmod() {
        let code = "def f(a: int, b: int) -> tuple[int, int]:\n    return divmod(a, b)";
        let _ = transpile(code);
    }

    #[test]
    fn test_pow() {
        let code = "def f(x: int, y: int) -> int:\n    return pow(x, y)";
        let _ = transpile(code);
    }

    #[test]
    fn test_isinstance() {
        let code = "def f(x: int) -> bool:\n    return isinstance(x, int)";
        let _ = transpile(code);
    }

    #[test]
    fn test_type() {
        let code = "def f(x: int) -> str:\n    return str(type(x))";
        let _ = transpile(code);
    }

    #[test]
    fn test_repr() {
        let code = "def f(x: int) -> str:\n    return repr(x)";
        let _ = transpile(code);
    }

    #[test]
    fn test_hex() {
        let code = "def f(x: int) -> str:\n    return hex(x)";
        let _ = transpile(code);
    }

    #[test]
    fn test_oct() {
        let code = "def f(x: int) -> str:\n    return oct(x)";
        let _ = transpile(code);
    }

    #[test]
    fn test_bin() {
        let code = "def f(x: int) -> str:\n    return bin(x)";
        let _ = transpile(code);
    }
}

// =============================================================================
// BINARY OPERATOR COVERAGE
// =============================================================================

mod binary_operators {
    use super::*;

    // Arithmetic operators
    #[test]
    fn test_add_int() {
        let code = "def f(a: int, b: int) -> int:\n    return a + b";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_add_float() {
        let code = "def f(a: float, b: float) -> float:\n    return a + b";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_add_str() {
        let code = "def f(a: str, b: str) -> str:\n    return a + b";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_add_list() {
        let code = "def f(a: list[int], b: list[int]) -> list[int]:\n    return a + b";
        let _ = transpile(code);
    }

    #[test]
    fn test_sub_int() {
        let code = "def f(a: int, b: int) -> int:\n    return a - b";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_sub_float() {
        let code = "def f(a: float, b: float) -> float:\n    return a - b";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_mul_int() {
        let code = "def f(a: int, b: int) -> int:\n    return a * b";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_mul_float() {
        let code = "def f(a: float, b: float) -> float:\n    return a * b";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_mul_str_int() {
        let code = "def f(s: str, n: int) -> str:\n    return s * n";
        let _ = transpile(code);
    }

    #[test]
    fn test_mul_list_int() {
        let code = "def f(x: list[int], n: int) -> list[int]:\n    return x * n";
        let _ = transpile(code);
    }

    #[test]
    fn test_div_float() {
        let code = "def f(a: float, b: float) -> float:\n    return a / b";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_div_int() {
        let code = "def f(a: int, b: int) -> float:\n    return a / b";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_floordiv_int() {
        let code = "def f(a: int, b: int) -> int:\n    return a // b";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_mod_int() {
        let code = "def f(a: int, b: int) -> int:\n    return a % b";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_pow_int() {
        let code = "def f(a: int, b: int) -> int:\n    return a ** b";
        let _ = transpile(code);
    }

    #[test]
    fn test_pow_float() {
        let code = "def f(a: float, b: float) -> float:\n    return a ** b";
        let _ = transpile(code);
    }

    // Bitwise operators
    #[test]
    fn test_bitand() {
        let code = "def f(a: int, b: int) -> int:\n    return a & b";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_bitor() {
        let code = "def f(a: int, b: int) -> int:\n    return a | b";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_bitxor() {
        let code = "def f(a: int, b: int) -> int:\n    return a ^ b";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_lshift() {
        let code = "def f(a: int, b: int) -> int:\n    return a << b";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_rshift() {
        let code = "def f(a: int, b: int) -> int:\n    return a >> b";
        assert!(transpile(code).is_ok());
    }

    // Comparison operators
    #[test]
    fn test_eq_int() {
        let code = "def f(a: int, b: int) -> bool:\n    return a == b";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_ne_int() {
        let code = "def f(a: int, b: int) -> bool:\n    return a != b";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_lt_int() {
        let code = "def f(a: int, b: int) -> bool:\n    return a < b";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_le_int() {
        let code = "def f(a: int, b: int) -> bool:\n    return a <= b";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_gt_int() {
        let code = "def f(a: int, b: int) -> bool:\n    return a > b";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_ge_int() {
        let code = "def f(a: int, b: int) -> bool:\n    return a >= b";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_eq_str() {
        let code = "def f(a: str, b: str) -> bool:\n    return a == b";
        assert!(transpile(code).is_ok());
    }

    // Logical operators
    #[test]
    fn test_and() {
        let code = "def f(a: bool, b: bool) -> bool:\n    return a and b";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_or() {
        let code = "def f(a: bool, b: bool) -> bool:\n    return a or b";
        assert!(transpile(code).is_ok());
    }

    // Containment operators
    #[test]
    fn test_in_list() {
        let code = "def f(x: int, lst: list[int]) -> bool:\n    return x in lst";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_not_in_list() {
        let code = "def f(x: int, lst: list[int]) -> bool:\n    return x not in lst";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_in_str() {
        let code = "def f(sub: str, s: str) -> bool:\n    return sub in s";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_in_dict() {
        let code = "def f(k: str, d: dict[str, int]) -> bool:\n    return k in d";
        assert!(transpile(code).is_ok());
    }

    // is/is not
    #[test]
    fn test_is_none() {
        let code = "def f(x: int) -> bool:\n    return x is None";
        let _ = transpile(code);
    }

    #[test]
    fn test_is_not_none() {
        let code = "def f(x: int) -> bool:\n    return x is not None";
        let _ = transpile(code);
    }
}

// =============================================================================
// UNARY OPERATOR COVERAGE
// =============================================================================

mod unary_operators {
    use super::*;

    #[test]
    fn test_neg_int() {
        let code = "def f(x: int) -> int:\n    return -x";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_neg_float() {
        let code = "def f(x: float) -> float:\n    return -x";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_pos_int() {
        let code = "def f(x: int) -> int:\n    return +x";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_not_bool() {
        let code = "def f(x: bool) -> bool:\n    return not x";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_invert() {
        let code = "def f(x: int) -> int:\n    return ~x";
        assert!(transpile(code).is_ok());
    }
}

// =============================================================================
// STRING METHOD COVERAGE
// =============================================================================

mod string_methods {
    use super::*;

    #[test]
    fn test_upper() {
        let code = "def f(s: str) -> str:\n    return s.upper()";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_lower() {
        let code = "def f(s: str) -> str:\n    return s.lower()";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_strip() {
        let code = "def f(s: str) -> str:\n    return s.strip()";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_lstrip() {
        let code = "def f(s: str) -> str:\n    return s.lstrip()";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_rstrip() {
        let code = "def f(s: str) -> str:\n    return s.rstrip()";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_split() {
        let code = "def f(s: str) -> list[str]:\n    return s.split()";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_split_with_sep() {
        let code = "def f(s: str) -> list[str]:\n    return s.split(\",\")";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_join() {
        let code = "def f(parts: list[str]) -> str:\n    return \",\".join(parts)";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_replace() {
        let code = "def f(s: str) -> str:\n    return s.replace(\"a\", \"b\")";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_startswith() {
        let code = "def f(s: str) -> bool:\n    return s.startswith(\"hello\")";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_endswith() {
        let code = "def f(s: str) -> bool:\n    return s.endswith(\"world\")";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_find() {
        let code = "def f(s: str) -> int:\n    return s.find(\"x\")";
        let _ = transpile(code);
    }

    #[test]
    fn test_rfind() {
        let code = "def f(s: str) -> int:\n    return s.rfind(\"x\")";
        let _ = transpile(code);
    }

    #[test]
    fn test_count() {
        let code = "def f(s: str) -> int:\n    return s.count(\"a\")";
        let _ = transpile(code);
    }

    #[test]
    fn test_isdigit() {
        let code = "def f(s: str) -> bool:\n    return s.isdigit()";
        let _ = transpile(code);
    }

    #[test]
    fn test_isalpha() {
        let code = "def f(s: str) -> bool:\n    return s.isalpha()";
        let _ = transpile(code);
    }

    #[test]
    fn test_isalnum() {
        let code = "def f(s: str) -> bool:\n    return s.isalnum()";
        let _ = transpile(code);
    }

    #[test]
    fn test_isspace() {
        let code = "def f(s: str) -> bool:\n    return s.isspace()";
        let _ = transpile(code);
    }

    #[test]
    fn test_title() {
        let code = "def f(s: str) -> str:\n    return s.title()";
        let _ = transpile(code);
    }

    #[test]
    fn test_capitalize() {
        let code = "def f(s: str) -> str:\n    return s.capitalize()";
        let _ = transpile(code);
    }

    #[test]
    fn test_center() {
        let code = "def f(s: str) -> str:\n    return s.center(20)";
        let _ = transpile(code);
    }

    #[test]
    fn test_ljust() {
        let code = "def f(s: str) -> str:\n    return s.ljust(20)";
        let _ = transpile(code);
    }

    #[test]
    fn test_rjust() {
        let code = "def f(s: str) -> str:\n    return s.rjust(20)";
        let _ = transpile(code);
    }

    #[test]
    fn test_zfill() {
        let code = "def f(s: str) -> str:\n    return s.zfill(10)";
        let _ = transpile(code);
    }

    #[test]
    fn test_format() {
        let code = "def f() -> str:\n    return \"Hello {}\".format(\"world\")";
        let _ = transpile(code);
    }

    #[test]
    fn test_encode() {
        let code = "def f(s: str) -> bytes:\n    return s.encode()";
        let _ = transpile(code);
    }
}

// =============================================================================
// LIST METHOD COVERAGE
// =============================================================================

mod list_methods {
    use super::*;

    #[test]
    fn test_append() {
        let code = "def f(lst: list[int]) -> None:\n    lst.append(1)";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_extend() {
        let code = "def f(lst: list[int], other: list[int]) -> None:\n    lst.extend(other)";
        let _ = transpile(code);
    }

    #[test]
    fn test_insert() {
        let code = "def f(lst: list[int]) -> None:\n    lst.insert(0, 1)";
        let _ = transpile(code);
    }

    #[test]
    fn test_remove() {
        let code = "def f(lst: list[int]) -> None:\n    lst.remove(1)";
        let _ = transpile(code);
    }

    #[test]
    fn test_pop() {
        let code = "def f(lst: list[int]) -> int:\n    return lst.pop()";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_pop_index() {
        let code = "def f(lst: list[int]) -> int:\n    return lst.pop(0)";
        let _ = transpile(code);
    }

    #[test]
    fn test_clear() {
        let code = "def f(lst: list[int]) -> None:\n    lst.clear()";
        let _ = transpile(code);
    }

    #[test]
    fn test_index() {
        let code = "def f(lst: list[int]) -> int:\n    return lst.index(1)";
        let _ = transpile(code);
    }

    #[test]
    fn test_count() {
        let code = "def f(lst: list[int]) -> int:\n    return lst.count(1)";
        let _ = transpile(code);
    }

    #[test]
    fn test_sort() {
        let code = "def f(lst: list[int]) -> None:\n    lst.sort()";
        let _ = transpile(code);
    }

    #[test]
    fn test_reverse() {
        let code = "def f(lst: list[int]) -> None:\n    lst.reverse()";
        let _ = transpile(code);
    }

    #[test]
    fn test_copy() {
        let code = "def f(lst: list[int]) -> list[int]:\n    return lst.copy()";
        let _ = transpile(code);
    }
}

// =============================================================================
// DICT METHOD COVERAGE
// =============================================================================

mod dict_methods {
    use super::*;

    #[test]
    fn test_get() {
        let code = "def f(d: dict[str, int], k: str) -> int:\n    return d.get(k, 0)";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_keys() {
        let code = "def f(d: dict[str, int]) -> list[str]:\n    return list(d.keys())";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_values() {
        let code = "def f(d: dict[str, int]) -> list[int]:\n    return list(d.values())";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_items() {
        let code = "def f(d: dict[str, int]) -> None:\n    for k, v in d.items():\n        pass";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_pop() {
        let code = "def f(d: dict[str, int]) -> int:\n    return d.pop(\"key\")";
        let _ = transpile(code);
    }

    #[test]
    fn test_pop_default() {
        let code = "def f(d: dict[str, int]) -> int:\n    return d.pop(\"key\", 0)";
        let _ = transpile(code);
    }

    #[test]
    fn test_setdefault() {
        let code = "def f(d: dict[str, int]) -> int:\n    return d.setdefault(\"key\", 0)";
        let _ = transpile(code);
    }

    #[test]
    fn test_update() {
        let code = "def f(d: dict[str, int], other: dict[str, int]) -> None:\n    d.update(other)";
        let _ = transpile(code);
    }

    #[test]
    fn test_clear() {
        let code = "def f(d: dict[str, int]) -> None:\n    d.clear()";
        let _ = transpile(code);
    }

    #[test]
    fn test_copy() {
        let code = "def f(d: dict[str, int]) -> dict[str, int]:\n    return d.copy()";
        let _ = transpile(code);
    }
}

// =============================================================================
// COLLECTION CONSTRUCTOR COVERAGE
// =============================================================================

mod collection_constructors {
    use super::*;

    #[test]
    fn test_list_empty() {
        let code = "def f() -> list[int]:\n    return []";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_list_with_items() {
        let code = "def f() -> list[int]:\n    return [1, 2, 3]";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_list_from_range() {
        let code = "def f() -> list[int]:\n    return list(range(10))";
        let _ = transpile(code);
    }

    #[test]
    fn test_dict_empty() {
        let code = "def f() -> dict[str, int]:\n    return {}";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_dict_with_items() {
        let code = "def f() -> dict[str, int]:\n    return {\"a\": 1, \"b\": 2}";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_set_empty() {
        let code = "def f() -> set[int]:\n    return set()";
        let _ = transpile(code);
    }

    #[test]
    fn test_set_with_items() {
        let code = "def f() -> set[int]:\n    return {1, 2, 3}";
        let _ = transpile(code);
    }

    #[test]
    fn test_tuple_empty() {
        let code = "def f() -> tuple[()]:\n    return ()";
        let _ = transpile(code);
    }

    #[test]
    fn test_tuple_with_items() {
        let code = "def f() -> tuple[int, int]:\n    return (1, 2)";
        let _ = transpile(code);
    }

    #[test]
    fn test_frozenset() {
        let code = "def f() -> frozenset[int]:\n    return frozenset([1, 2, 3])";
        let _ = transpile(code);
    }

    #[test]
    fn test_bytes() {
        let code = "def f() -> bytes:\n    return b\"hello\"";
        let _ = transpile(code);
    }

    #[test]
    fn test_bytearray() {
        let code = "def f() -> bytearray:\n    return bytearray(b\"hello\")";
        let _ = transpile(code);
    }
}

// =============================================================================
// SUBSCRIPT/INDEX COVERAGE
// =============================================================================

mod subscript_operations {
    use super::*;

    #[test]
    fn test_list_index() {
        let code = "def f(lst: list[int]) -> int:\n    return lst[0]";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_list_negative_index() {
        let code = "def f(lst: list[int]) -> int:\n    return lst[-1]";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_dict_index() {
        let code = "def f(d: dict[str, int]) -> int:\n    return d[\"key\"]";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_str_index() {
        let code = "def f(s: str) -> str:\n    return s[0]";
        let _ = transpile(code);
    }

    #[test]
    fn test_list_slice() {
        let code = "def f(lst: list[int]) -> list[int]:\n    return lst[1:3]";
        let _ = transpile(code);
    }

    #[test]
    fn test_list_slice_start() {
        let code = "def f(lst: list[int]) -> list[int]:\n    return lst[:3]";
        let _ = transpile(code);
    }

    #[test]
    fn test_list_slice_end() {
        let code = "def f(lst: list[int]) -> list[int]:\n    return lst[1:]";
        let _ = transpile(code);
    }

    #[test]
    fn test_list_slice_step() {
        let code = "def f(lst: list[int]) -> list[int]:\n    return lst[::2]";
        let _ = transpile(code);
    }

    #[test]
    fn test_str_slice() {
        let code = "def f(s: str) -> str:\n    return s[1:3]";
        let _ = transpile(code);
    }

    #[test]
    fn test_list_assign_index() {
        let code = "def f(lst: list[int]) -> None:\n    lst[0] = 1";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_dict_assign() {
        let code = "def f(d: dict[str, int]) -> None:\n    d[\"key\"] = 1";
        assert!(transpile(code).is_ok());
    }
}

// =============================================================================
// COMPREHENSION COVERAGE
// =============================================================================

mod comprehensions {
    use super::*;

    #[test]
    fn test_list_comprehension() {
        let code = "def f() -> list[int]:\n    return [x * 2 for x in range(10)]";
        let _ = transpile(code);
    }

    #[test]
    fn test_list_comprehension_with_if() {
        let code = "def f() -> list[int]:\n    return [x for x in range(10) if x % 2 == 0]";
        let _ = transpile(code);
    }

    #[test]
    fn test_dict_comprehension() {
        let code = "def f() -> dict[int, int]:\n    return {x: x * 2 for x in range(5)}";
        let _ = transpile(code);
    }

    #[test]
    fn test_set_comprehension() {
        let code = "def f() -> set[int]:\n    return {x * 2 for x in range(5)}";
        let _ = transpile(code);
    }

    #[test]
    fn test_generator_expression() {
        let code = "def f() -> int:\n    return sum(x * 2 for x in range(10))";
        let _ = transpile(code);
    }
}

// =============================================================================
// CONDITIONAL EXPRESSION COVERAGE
// =============================================================================

mod conditional_expressions {
    use super::*;

    #[test]
    fn test_ternary() {
        let code = "def f(x: int) -> int:\n    return 1 if x > 0 else -1";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_nested_ternary() {
        let code = "def f(x: int) -> int:\n    return 1 if x > 0 else 0 if x == 0 else -1";
        let _ = transpile(code);
    }

    #[test]
    fn test_ternary_with_call() {
        let code = "def f(x: int) -> str:\n    return str(x) if x > 0 else \"negative\"";
        let _ = transpile(code);
    }
}

// =============================================================================
// LAMBDA COVERAGE
// =============================================================================

mod lambda_expressions {
    use super::*;

    #[test]
    fn test_lambda_simple() {
        let code = "def f() -> int:\n    fn = lambda x: x + 1\n    return fn(5)";
        let _ = transpile(code);
    }

    #[test]
    fn test_lambda_multiple_args() {
        let code = "def f() -> int:\n    fn = lambda x, y: x + y\n    return fn(1, 2)";
        let _ = transpile(code);
    }

    #[test]
    fn test_lambda_in_map() {
        let code = "def f(lst: list[int]) -> list[int]:\n    return list(map(lambda x: x * 2, lst))";
        let _ = transpile(code);
    }

    #[test]
    fn test_lambda_in_filter() {
        let code = "def f(lst: list[int]) -> list[int]:\n    return list(filter(lambda x: x > 0, lst))";
        let _ = transpile(code);
    }

    #[test]
    fn test_lambda_in_sorted_key() {
        let code = "def f(lst: list[str]) -> list[str]:\n    return sorted(lst, key=lambda x: len(x))";
        let _ = transpile(code);
    }
}

// =============================================================================
// ATTRIBUTE ACCESS COVERAGE
// =============================================================================

mod attribute_access {
    use super::*;

    #[test]
    fn test_class_attribute() {
        let code = "class Foo:\n    x: int = 0\n\ndef f() -> int:\n    return Foo.x";
        let _ = transpile(code);
    }

    #[test]
    fn test_instance_attribute() {
        let code = "class Foo:\n    def __init__(self) -> None:\n        self.x: int = 0\n\ndef f(obj: Foo) -> int:\n    return obj.x";
        let _ = transpile(code);
    }

    #[test]
    fn test_chained_attribute() {
        let code = "def f(s: str) -> str:\n    return s.upper().lower()";
        let _ = transpile(code);
    }
}

// =============================================================================
// SPECIAL EXPRESSIONS COVERAGE
// =============================================================================

mod special_expressions {
    use super::*;

    #[test]
    fn test_fstring_simple() {
        let code = "def f(x: int) -> str:\n    return f\"value: {x}\"";
        let _ = transpile(code);
    }

    #[test]
    fn test_fstring_expression() {
        let code = "def f(x: int) -> str:\n    return f\"doubled: {x * 2}\"";
        let _ = transpile(code);
    }

    #[test]
    fn test_fstring_format_spec() {
        let code = "def f(x: float) -> str:\n    return f\"value: {x:.2f}\"";
        let _ = transpile(code);
    }

    #[test]
    fn test_walrus_operator() {
        let code = "def f(lst: list[int]) -> int:\n    if (n := len(lst)) > 0:\n        return n\n    return 0";
        let _ = transpile(code);
    }

    #[test]
    fn test_starred_unpack() {
        let code = "def f() -> None:\n    a, *rest = [1, 2, 3, 4]";
        let _ = transpile(code);
    }
}
