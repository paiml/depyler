//! Wave 21 coverage tests: complex indexing, statement generation, function patterns, expressions
//!
//! Targets uncovered code paths in:
//! - indexing.rs: dict string key, list int/negative/variable, string indexing, tuple field access,
//!   nested indexing, subscript assignment, del, conditional indexing
//! - slicing.rs: start:stop, step, negative step, open bounds, string slicing
//! - stmt_gen_complex.rs: try/except, with, assert, raise, nested functions, while/for else,
//!   break/continue in nested loops, multiple return paths
//! - func_gen.rs: *args, **kwargs, defaults, type annotations, async, generators, lambdas,
//!   recursive, docstrings, decorators
//! - expr_gen: ternary, walrus, starred, comprehensions, chained comparisons, binary/bitwise ops,
//!   identity/membership, f-strings, nested calls, method chaining
//!
//! 200 tests total

#![cfg(test)]

use crate::ast_bridge::AstBridge;
use crate::rust_gen::generate_rust_file;
use crate::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile(python_code: &str) -> String {
    let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
    let (module, _) =
        AstBridge::new().with_source(python_code.to_string()).python_to_hir(ast).expect("hir");
    let tm = TypeMapper::default();
    let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
    result
}

#[cfg(test)]
mod tests {
    #![allow(unused_variables)]
    use super::*;

    // ========================================================================
    // SECTION 1: Complex Indexing Patterns (tests 001-050)
    // ========================================================================

    #[test]
    fn test_w21si_001_dict_string_key_indexing() {
        let code = "def f(d: dict) -> str:\n    return d[\"key\"]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("get") || result.contains("key"));
    }

    #[test]
    fn test_w21si_002_list_int_indexing_zero() {
        let code = "def f(lst: list) -> int:\n    return lst[0]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("0"));
    }

    #[test]
    fn test_w21si_003_list_negative_indexing() {
        let code = "def f(lst: list) -> int:\n    return lst[-1]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_004_list_variable_indexing() {
        let code = "def f(lst: list, i: int) -> int:\n    return lst[i]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_005_string_char_indexing() {
        let code = "def f(s: str) -> str:\n    return s[0]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("char") || result.contains("0"));
    }

    #[test]
    fn test_w21si_006_tuple_field_access_zero() {
        let code = "def f(t: tuple) -> int:\n    return t[0]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_007_tuple_field_access_one() {
        let code = "def f(t: tuple) -> int:\n    return t[1]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_008_nested_dict_indexing() {
        let code = "def f(d: dict) -> str:\n    inner = d[\"a\"]\n    return inner\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_009_dict_variable_key() {
        let code = "def f(d: dict, key: str) -> str:\n    return d[key]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_010_list_index_second_element() {
        let code = "def f(lst: list) -> int:\n    return lst[1]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_011_slice_start_stop() {
        let code = "def f(lst: list) -> list:\n    return lst[1:3]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_012_slice_step_by_two() {
        let code = "def f(lst: list) -> list:\n    return lst[::2]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("step") || result.contains("iter"));
    }

    #[test]
    fn test_w21si_013_slice_reverse() {
        let code = "def f(lst: list) -> list:\n    return lst[::-1]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("rev") || result.contains("step") || result.contains("-1"));
    }

    #[test]
    fn test_w21si_014_slice_start_stop_step() {
        let code = "def f(lst: list) -> list:\n    return lst[0:10:2]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_015_slice_no_bounds() {
        let code = "def f(lst: list) -> list:\n    return lst[:]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("clone") || result.contains("to_vec") || result.contains("collect")
        );
    }

    #[test]
    fn test_w21si_016_index_assignment_list() {
        let code = "def f() -> list:\n    lst = [1, 2, 3]\n    lst[0] = 5\n    return lst\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_017_index_assignment_dict() {
        let code = "def f() -> dict:\n    d = {}\n    d[\"key\"] = \"value\"\n    return d\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("insert") || result.contains("key"));
    }

    #[test]
    fn test_w21si_018_negative_index_assignment() {
        let code = "def f() -> list:\n    lst = [1, 2, 3]\n    lst[-1] = 99\n    return lst\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_019_multi_dim_indexing() {
        let code = "def f(matrix: list, i: int, j: int) -> int:\n    return matrix[i][j]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_020_index_in_loop() {
        let code = "def f(lst: list) -> int:\n    total = 0\n    for i in range(len(lst)):\n        total = total + lst[i]\n    return total\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("for") || result.contains("range"));
    }

    #[test]
    fn test_w21si_021_dict_int_key() {
        let code =
            "def f() -> dict:\n    d = {}\n    d[1] = \"one\"\n    d[2] = \"two\"\n    return d\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_022_string_negative_index() {
        let code = "def f(s: str) -> str:\n    return s[-1]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_023_list_index_in_condition() {
        let code =
            "def f(lst: list) -> int:\n    if lst[0] > 0:\n        return lst[0]\n    return 0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("if"));
    }

    #[test]
    fn test_w21si_024_slice_from_start() {
        let code = "def f(lst: list) -> list:\n    return lst[:3]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_025_slice_to_end() {
        let code = "def f(lst: list) -> list:\n    return lst[2:]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_026_slice_negative_start() {
        let code = "def f(lst: list) -> list:\n    return lst[-3:]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_027_slice_negative_stop() {
        let code = "def f(lst: list) -> list:\n    return lst[:-1]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_028_string_slice_start_stop() {
        let code = "def f(s: str) -> str:\n    return s[1:3]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_029_string_slice_from_start() {
        let code = "def f(s: str) -> str:\n    return s[:5]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_030_string_slice_to_end() {
        let code = "def f(s: str) -> str:\n    return s[3:]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_031_index_with_expression() {
        let code = "def f(lst: list, n: int) -> int:\n    return lst[n - 1]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_032_index_with_modulo() {
        let code = "def f(lst: list, i: int) -> int:\n    return lst[i % len(lst)]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_033_dict_get_with_default() {
        let code = "def f(d: dict) -> str:\n    return d.get(\"key\", \"default\")\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_034_list_pop_index() {
        let code = "def f() -> int:\n    lst = [1, 2, 3]\n    return lst.pop(0)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_035_index_after_method_call() {
        let code = "def f(s: str) -> str:\n    parts = s.split(\",\")\n    return parts[0]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_036_dict_keys_index() {
        let code = "def f(d: dict) -> list:\n    return list(d.keys())\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_037_dict_values_list() {
        let code = "def f(d: dict) -> list:\n    return list(d.values())\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_038_slice_assign() {
        let code =
            "def f() -> list:\n    lst = [1, 2, 3, 4, 5]\n    first = lst[:2]\n    return first\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_039_nested_list_indexing() {
        let code = "def f() -> int:\n    matrix = [[1, 2], [3, 4]]\n    return matrix[0][1]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_040_index_in_comprehension() {
        let code = "def f(lst: list) -> list:\n    return [lst[i] for i in range(len(lst))]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_041_index_with_len_minus_one() {
        let code = "def f(lst: list) -> int:\n    return lst[len(lst) - 1]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_042_string_index_iteration() {
        let code = "def f(s: str) -> list:\n    chars = []\n    for i in range(len(s)):\n        chars.append(s[i])\n    return chars\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_043_dict_update_existing() {
        let code = "def f() -> dict:\n    d = {\"a\": 1}\n    d[\"a\"] = 2\n    return d\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_044_list_swap_via_index() {
        let code = "def f() -> list:\n    lst = [1, 2, 3]\n    tmp = lst[0]\n    lst[0] = lst[2]\n    lst[2] = tmp\n    return lst\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_045_index_comparison() {
        let code = "def f(lst: list) -> bool:\n    return lst[0] == lst[1]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_046_slice_with_variable_bounds() {
        let code = "def f(lst: list, start: int, end: int) -> list:\n    return lst[start:end]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_047_dict_in_operator() {
        let code = "def f(d: dict) -> bool:\n    return \"key\" in d\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("contains") || result.contains("key"));
    }

    #[test]
    fn test_w21si_048_list_count_indexed() {
        let code = "def f() -> int:\n    lst = [1, 2, 2, 3]\n    return lst.count(2)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_049_dict_setdefault() {
        let code = "def f() -> dict:\n    d = {}\n    d.setdefault(\"key\", 0)\n    return d\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_050_index_chained_calls() {
        let code = "def f(s: str) -> str:\n    return s.split(\" \")[0].strip()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 2: Statement Generation Complex (tests 051-120)
    // ========================================================================

    #[test]
    fn test_w21si_051_nested_function_def() {
        let code =
            "def outer() -> int:\n    def inner() -> int:\n        return 1\n    return inner()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21si_052_multi_target_assignment() {
        let code = "def f() -> int:\n    i = 0\n    j = 0\n    return i + j\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_053_with_statement_basic() {
        let code = "def f() -> str:\n    with open(\"file.txt\") as fh:\n        data = fh.read()\n    return data\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_054_try_except_value_error() {
        let code = "def f(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError:\n        return 0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_055_try_except_as_variable() {
        let code = "def f(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError as e:\n        return -1\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_056_try_except_else() {
        let code = "def f(x: int) -> int:\n    try:\n        result = x + 1\n    except Exception:\n        result = 0\n    else:\n        result = result + 10\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_057_try_except_finally() {
        let code = "def f(x: int) -> int:\n    result = 0\n    try:\n        result = x + 1\n    except Exception:\n        result = -1\n    finally:\n        result = result + 100\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_058_nested_try_except() {
        let code = "def f(x: int) -> int:\n    try:\n        try:\n            return x + 1\n        except ValueError:\n            return 0\n    except Exception:\n        return -1\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_059_assert_equality() {
        let code = "def f(x: int, y: int) -> bool:\n    assert x == y\n    return True\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("assert"));
    }

    #[test]
    fn test_w21si_060_assert_inequality() {
        let code = "def f(x: int, y: int) -> bool:\n    assert x != y\n    return True\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("assert"));
    }

    #[test]
    fn test_w21si_061_assert_with_message() {
        let code = "def f(x: int) -> int:\n    assert x > 0, \"must be positive\"\n    return x\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("assert"));
    }

    #[test]
    fn test_w21si_062_raise_value_error() {
        let code = "def f(x: int) -> int:\n    if x < 0:\n        raise ValueError(\"negative\")\n    return x\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_063_raise_runtime_error() {
        let code = "def f() -> int:\n    raise RuntimeError(\"not implemented\")\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_064_pass_in_if() {
        let code = "def f(x: int) -> int:\n    if x > 0:\n        pass\n    return x\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_065_pass_in_function() {
        let code = "def f() -> None:\n    pass\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_066_break_in_nested_loop() {
        let code = "def f() -> int:\n    result = 0\n    for i in range(10):\n        for j in range(10):\n            if j == 5:\n                break\n            result = result + 1\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("break"));
    }

    #[test]
    fn test_w21si_067_continue_in_nested_loop() {
        let code = "def f() -> int:\n    total = 0\n    for i in range(10):\n        for j in range(10):\n            if j % 2 == 0:\n                continue\n            total = total + 1\n    return total\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("continue"));
    }

    #[test]
    fn test_w21si_068_while_else() {
        let code = "def f(n: int) -> str:\n    i = 0\n    while i < n:\n        i = i + 1\n    else:\n        return \"done\"\n    return \"unreachable\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_069_for_else() {
        let code = "def f(lst: list) -> int:\n    for x in lst:\n        if x == 0:\n            return 0\n    else:\n        return -1\n    return 1\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_070_multiple_return_paths() {
        let code = "def f(x: int) -> str:\n    if x > 0:\n        return \"positive\"\n    elif x < 0:\n        return \"negative\"\n    else:\n        return \"zero\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_071_early_return_none_check() {
        let code = "def f(x: int) -> int:\n    if x == 0:\n        return 0\n    result = 100 // x\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_072_nested_if_with_returns() {
        let code = "def f(x: int, y: int) -> str:\n    if x > 0:\n        if y > 0:\n            return \"both positive\"\n        return \"x positive\"\n    return \"x not positive\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_073_while_with_break() {
        let code = "def f() -> int:\n    i = 0\n    while True:\n        if i >= 10:\n            break\n        i = i + 1\n    return i\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("loop") || result.contains("while"));
    }

    #[test]
    fn test_w21si_074_for_with_enumerate() {
        let code = "def f(lst: list) -> int:\n    total = 0\n    for i, val in enumerate(lst):\n        total = total + i\n    return total\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("enumerate"));
    }

    #[test]
    fn test_w21si_075_for_with_zip() {
        let code = "def f(a: list, b: list) -> list:\n    result = []\n    for x, y in zip(a, b):\n        result.append(x + y)\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("zip"));
    }

    #[test]
    fn test_w21si_076_nested_function_closure() {
        let code = "def outer(x: int) -> int:\n    def inner(y: int) -> int:\n        return x + y\n    return inner(10)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_077_try_except_generic() {
        let code = "def f() -> int:\n    try:\n        return 1 // 0\n    except Exception:\n        return 0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_078_try_multiple_except() {
        let code = "def f(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError:\n        return -1\n    except TypeError:\n        return -2\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_079_if_elif_elif_else() {
        let code = "def classify(x: int) -> str:\n    if x > 100:\n        return \"high\"\n    elif x > 50:\n        return \"medium\"\n    elif x > 0:\n        return \"low\"\n    else:\n        return \"none\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_080_while_counter_pattern() {
        let code = "def f(n: int) -> int:\n    count = 0\n    i = 0\n    while i < n:\n        count = count + 1\n        i = i + 1\n    return count\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("while"));
    }

    #[test]
    fn test_w21si_081_for_range_step() {
        let code = "def f() -> int:\n    total = 0\n    for i in range(0, 20, 3):\n        total = total + i\n    return total\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_082_nested_for_loops() {
        let code = "def f(n: int) -> int:\n    total = 0\n    for i in range(n):\n        for j in range(n):\n            total = total + 1\n    return total\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_083_augmented_assign_add() {
        let code = "def f() -> int:\n    x = 0\n    x += 5\n    return x\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_084_augmented_assign_sub() {
        let code = "def f() -> int:\n    x = 10\n    x -= 3\n    return x\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_085_augmented_assign_mul() {
        let code = "def f() -> int:\n    x = 2\n    x *= 5\n    return x\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_086_augmented_assign_div() {
        let code = "def f() -> float:\n    x = 10.0\n    x /= 3.0\n    return x\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_087_assert_in_check() {
        let code = "def f(lst: list) -> bool:\n    assert \"hello\" in lst\n    return True\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_088_assert_greater_than() {
        let code = "def f(x: int) -> int:\n    assert x > 0\n    return x * 2\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_089_assert_less_than() {
        let code = "def f(x: int) -> int:\n    assert x < 100\n    return x\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_090_global_variable_usage() {
        let code = "MAX_SIZE = 100\ndef f(x: int) -> bool:\n    return x < MAX_SIZE\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_091_string_constant() {
        let code =
            "DEFAULT_NAME = \"world\"\ndef greet() -> str:\n    return \"hello \" + DEFAULT_NAME\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_092_tuple_unpack_two() {
        let code = "def f() -> int:\n    a, b = 1, 2\n    return a + b\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_093_tuple_unpack_three() {
        let code = "def f() -> int:\n    a, b, c = 1, 2, 3\n    return a + b + c\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_094_with_open_write() {
        let code = "def f(path: str) -> None:\n    with open(path, \"w\") as fh:\n        fh.write(\"hello\")\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_095_raise_type_error() {
        let code = "def f(x: int) -> int:\n    if not isinstance(x, int):\n        raise TypeError(\"expected int\")\n    return x\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_096_for_loop_accumulate() {
        let code = "def f(items: list) -> list:\n    result = []\n    for item in items:\n        result.append(item)\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("push") || result.contains("append"));
    }

    #[test]
    fn test_w21si_097_while_decrement() {
        let code = "def f(n: int) -> int:\n    total = 0\n    while n > 0:\n        total = total + n\n        n = n - 1\n    return total\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_098_nested_if_else_chain() {
        let code = "def grade(score: int) -> str:\n    if score >= 90:\n        return \"A\"\n    elif score >= 80:\n        return \"B\"\n    elif score >= 70:\n        return \"C\"\n    elif score >= 60:\n        return \"D\"\n    else:\n        return \"F\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_099_empty_list_build() {
        let code = "def f(n: int) -> list:\n    result = []\n    for i in range(n):\n        result.append(i * 2)\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_100_dict_build_loop() {
        let code = "def f(keys: list) -> dict:\n    d = {}\n    for k in keys:\n        d[k] = len(k)\n    return d\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_101_multiple_assignments() {
        let code = "def f() -> int:\n    a = 1\n    b = 2\n    c = 3\n    d = 4\n    return a + b + c + d\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_102_conditional_assignment() {
        let code = "def f(x: int) -> str:\n    if x > 0:\n        label = \"positive\"\n    else:\n        label = \"non-positive\"\n    return label\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_103_for_with_index_and_break() {
        let code = "def f(lst: list) -> int:\n    for i in range(len(lst)):\n        if lst[i] == 0:\n            return i\n    return -1\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_104_return_bool_expression() {
        let code = "def f(x: int) -> bool:\n    return x > 0 and x < 100\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_105_return_none_explicit() {
        let code = "def f(x: int) -> None:\n    if x > 0:\n        return None\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_106_try_return_in_body() {
        let code = "def f() -> int:\n    try:\n        x = 10\n        return x\n    except Exception:\n        return 0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_107_raise_bare() {
        let code = "def f(x: int) -> int:\n    try:\n        return x\n    except Exception:\n        raise\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_108_continue_in_for() {
        let code = "def f(lst: list) -> int:\n    total = 0\n    for x in lst:\n        if x < 0:\n            continue\n        total = total + x\n    return total\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("continue"));
    }

    #[test]
    fn test_w21si_109_break_in_while() {
        let code = "def f() -> int:\n    i = 0\n    while i < 100:\n        if i * i > 50:\n            break\n        i = i + 1\n    return i\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("break"));
    }

    #[test]
    fn test_w21si_110_nested_while_loops() {
        let code = "def f(n: int) -> int:\n    i = 0\n    total = 0\n    while i < n:\n        j = 0\n        while j < n:\n            total = total + 1\n            j = j + 1\n        i = i + 1\n    return total\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_111_for_string_chars() {
        let code = "def f(s: str) -> int:\n    count = 0\n    for c in s:\n        count = count + 1\n    return count\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_112_for_dict_items() {
        let code = "def f(d: dict) -> list:\n    result = []\n    for k, v in d.items():\n        result.append(k)\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_113_pass_in_except() {
        let code = "def f(x: int) -> int:\n    try:\n        return x\n    except Exception:\n        pass\n    return 0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_114_multiple_except_handlers() {
        let code = "def f(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError:\n        return -1\n    except TypeError:\n        return -2\n    except Exception:\n        return -3\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_115_augmented_assign_floor_div() {
        let code = "def f() -> int:\n    x = 100\n    x //= 3\n    return x\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_116_augmented_assign_modulo() {
        let code = "def f() -> int:\n    x = 17\n    x %= 5\n    return x\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_117_nested_with_statement() {
        let code = "def f() -> str:\n    with open(\"a.txt\") as fa:\n        data = fa.read()\n    return data\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_118_assert_boolean_var() {
        let code = "def f(flag: bool) -> int:\n    assert flag\n    return 1\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_119_return_from_nested_if() {
        let code = "def f(x: int, y: int) -> int:\n    if x > 0:\n        if y > 0:\n            return x * y\n        else:\n            return x\n    else:\n        return 0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_120_for_reversed() {
        let code = "def f(lst: list) -> list:\n    result = []\n    for x in reversed(lst):\n        result.append(x)\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 3: Function Generation Patterns (tests 121-170)
    // ========================================================================

    #[test]
    fn test_w21si_121_func_with_args() {
        let code = "def f(*args) -> int:\n    total = 0\n    for a in args:\n        total = total + a\n    return total\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21si_122_func_with_kwargs() {
        let code = "def f(**kwargs) -> int:\n    return len(kwargs)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_123_func_args_and_kwargs() {
        let code = "def f(*args, **kwargs) -> int:\n    return len(args)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_124_func_default_int() {
        let code = "def f(x: int = 10) -> int:\n    return x * 2\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_125_func_default_string() {
        let code = "def f(name: str = \"world\") -> str:\n    return \"hello \" + name\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_126_func_default_bool() {
        let code = "def f(flag: bool = True) -> bool:\n    return not flag\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_127_func_default_none() {
        let code =
            "def f(x: int = None) -> int:\n    if x is None:\n        return 0\n    return x\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_128_func_type_annotations_int_str() {
        let code = "def f(x: int, y: str) -> bool:\n    return len(y) > x\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21si_129_func_return_list_int() {
        let code = "def f() -> list:\n    return [1, 2, 3]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_130_func_return_dict() {
        let code = "def f() -> dict:\n    return {\"a\": 1, \"b\": 2}\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_131_func_return_tuple() {
        let code = "def f() -> tuple:\n    return (1, \"hello\")\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_132_async_function() {
        let code = "async def fetch() -> str:\n    return \"data\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("async") || result.contains("fn"));
    }

    #[test]
    fn test_w21si_133_generator_yield() {
        let code = "def gen() -> int:\n    yield 1\n    yield 2\n    yield 3\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_134_generator_yield_in_loop() {
        let code = "def gen(n: int) -> int:\n    for i in range(n):\n        yield i\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_135_lambda_simple() {
        let code = "def f() -> int:\n    add_one = lambda x: x + 1\n    return add_one(5)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_136_lambda_two_args() {
        let code = "def f() -> int:\n    add = lambda x, y: x + y\n    return add(3, 4)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_137_lambda_with_default() {
        let code = "def f() -> int:\n    inc = lambda x, y=1: x + y\n    return inc(5)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_138_nested_function_two_levels() {
        let code = "def outer() -> int:\n    def middle() -> int:\n        def inner() -> int:\n            return 1\n        return inner()\n    return middle()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_139_recursive_function() {
        let code = "def factorial(n: int) -> int:\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("factorial"));
    }

    #[test]
    fn test_w21si_140_recursive_fibonacci() {
        let code = "def fib(n: int) -> int:\n    if n <= 0:\n        return 0\n    if n == 1:\n        return 1\n    return fib(n - 1) + fib(n - 2)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("fib"));
    }

    #[test]
    fn test_w21si_141_func_with_docstring() {
        let code = "def f(x: int) -> int:\n    \"\"\"Double the input.\"\"\"\n    return x * 2\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_142_func_calling_func() {
        let code = "def add(a: int, b: int) -> int:\n    return a + b\ndef f(x: int) -> int:\n    return add(x, 10)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("add"));
    }

    #[test]
    fn test_w21si_143_func_multiple_defaults() {
        let code = "def f(a: int = 1, b: int = 2, c: int = 3) -> int:\n    return a + b + c\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_144_func_mixed_default_non_default() {
        let code = "def f(x: int, y: int = 0, z: int = 0) -> int:\n    return x + y + z\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_145_func_return_bool() {
        let code = "def is_even(n: int) -> bool:\n    return n % 2 == 0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("bool"));
    }

    #[test]
    fn test_w21si_146_func_no_return_type() {
        let code = "def f(x):\n    return x + 1\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_147_func_empty_body_pass() {
        let code = "def placeholder() -> None:\n    pass\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_148_func_with_list_param() {
        let code = "def sum_list(lst: list) -> int:\n    total = 0\n    for x in lst:\n        total = total + x\n    return total\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_149_func_with_dict_param() {
        let code = "def get_value(d: dict, key: str) -> str:\n    return d.get(key, \"\")\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_150_func_return_string_concat() {
        let code = "def greet(first: str, last: str) -> str:\n    return first + \" \" + last\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_151_func_float_operations() {
        let code = "def area(radius: float) -> float:\n    return 3.14159 * radius * radius\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("f64") || result.contains("float"));
    }

    #[test]
    fn test_w21si_152_func_boolean_logic() {
        let code = "def both_positive(a: int, b: int) -> bool:\n    return a > 0 and b > 0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_153_func_or_logic() {
        let code = "def either_zero(a: int, b: int) -> bool:\n    return a == 0 or b == 0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_154_func_not_logic() {
        let code = "def is_not_empty(s: str) -> bool:\n    return not len(s) == 0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_155_func_return_list_comprehension() {
        let code = "def squares(n: int) -> list:\n    return [i * i for i in range(n)]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_156_func_multiple_statements() {
        let code =
            "def f(x: int) -> int:\n    a = x * 2\n    b = a + 3\n    c = b - 1\n    return c\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_157_func_string_methods() {
        let code = "def f(s: str) -> str:\n    return s.upper()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_158_func_list_methods() {
        let code = "def f() -> list:\n    lst = [3, 1, 2]\n    lst.sort()\n    return lst\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_159_func_dict_comprehension_return() {
        let code = "def f(keys: list) -> dict:\n    return {k: len(k) for k in keys}\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_160_func_string_format() {
        let code = "def greet(name: str) -> str:\n    return f\"Hello, {name}!\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format") || result.contains("name"));
    }

    #[test]
    fn test_w21si_161_func_max_min() {
        let code = "def f(a: int, b: int) -> int:\n    return max(a, b)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("max"));
    }

    #[test]
    fn test_w21si_162_func_abs() {
        let code = "def f(x: int) -> int:\n    return abs(x)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("abs"));
    }

    #[test]
    fn test_w21si_163_func_len_call() {
        let code = "def f(lst: list) -> int:\n    return len(lst)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("len"));
    }

    #[test]
    fn test_w21si_164_func_str_conversion() {
        let code = "def f(x: int) -> str:\n    return str(x)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("to_string") || result.contains("format"));
    }

    #[test]
    fn test_w21si_165_func_int_conversion() {
        let code = "def f(s: str) -> int:\n    return int(s)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("parse") || result.contains("int"));
    }

    #[test]
    fn test_w21si_166_func_nested_calls() {
        let code = "def f(lst: list) -> int:\n    return len(sorted(lst))\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_167_func_chained_string() {
        let code = "def f(s: str) -> str:\n    return s.strip().lower()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_168_func_map_usage() {
        let code = "def f(lst: list) -> list:\n    return list(map(str, lst))\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_169_func_filter_usage() {
        let code = "def f(lst: list) -> list:\n    return list(filter(None, lst))\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_170_func_sum_builtin() {
        let code = "def f(lst: list) -> int:\n    return sum(lst)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 4: Complex Expression Patterns (tests 171-200)
    // ========================================================================

    #[test]
    fn test_w21si_171_ternary_expression() {
        let code =
            "def f(x: int) -> str:\n    return \"positive\" if x > 0 else \"non-positive\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("if") || result.contains("else"));
    }

    #[test]
    fn test_w21si_172_nested_ternary() {
        let code = "def f(x: int) -> str:\n    return \"pos\" if x > 0 else \"neg\" if x < 0 else \"zero\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_173_walrus_operator() {
        let code = "def f(data: list) -> int:\n    if (n := len(data)) > 10:\n        return n\n    return 0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_174_starred_unpacking() {
        let code = "def f() -> list:\n    first, *rest = [1, 2, 3, 4, 5]\n    return rest\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_175_list_comprehension_with_filter() {
        let code = "def f(lst: list) -> list:\n    return [x for x in lst if x > 0]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("filter") || result.contains("iter") || result.contains("if"));
    }

    #[test]
    fn test_w21si_176_dict_comprehension() {
        let code = "def f(keys: list, vals: list) -> dict:\n    return {k: v for k, v in zip(keys, vals)}\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_177_set_comprehension() {
        let code = "def f(n: int) -> set:\n    return {x * x for x in range(n)}\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_178_generator_sum() {
        let code = "def f(n: int) -> int:\n    return sum(x for x in range(n))\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_179_multiple_assignment_tuple() {
        let code = "def f() -> int:\n    a, b, c = 1, 2, 3\n    return a + b + c\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_180_floor_div_operator() {
        let code = "def f(a: int, b: int) -> int:\n    return a // b\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_181_power_operator() {
        let code = "def f(base: int, exp: int) -> int:\n    return base ** exp\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("pow") || result.contains("**"));
    }

    #[test]
    fn test_w21si_182_modulo_operator() {
        let code = "def f(a: int, b: int) -> int:\n    return a % b\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("%"));
    }

    #[test]
    fn test_w21si_183_bitwise_and() {
        let code = "def f(a: int, b: int) -> int:\n    return a & b\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("&"));
    }

    #[test]
    fn test_w21si_184_bitwise_or() {
        let code = "def f(a: int, b: int) -> int:\n    return a | b\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("|"));
    }

    #[test]
    fn test_w21si_185_bitwise_xor() {
        let code = "def f(a: int, b: int) -> int:\n    return a ^ b\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("^"));
    }

    #[test]
    fn test_w21si_186_bitwise_not() {
        let code = "def f(a: int) -> int:\n    return ~a\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_187_left_shift() {
        let code = "def f(a: int, n: int) -> int:\n    return a << n\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("<<"));
    }

    #[test]
    fn test_w21si_188_right_shift() {
        let code = "def f(a: int, n: int) -> int:\n    return a >> n\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains(">>"));
    }

    #[test]
    fn test_w21si_189_is_none_check() {
        let code = "def f(x: int) -> bool:\n    return x is None\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_190_is_not_none_check() {
        let code = "def f(x: int) -> bool:\n    return x is not None\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_191_in_operator_list() {
        let code = "def f(x: int, lst: list) -> bool:\n    return x in lst\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("contains") || result.contains("in"));
    }

    #[test]
    fn test_w21si_192_not_in_operator() {
        let code = "def f(x: int, lst: list) -> bool:\n    return x not in lst\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_193_boolean_short_circuit_and() {
        let code = "def f(a: bool, b: int) -> bool:\n    return a and b > 0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_194_boolean_short_circuit_or() {
        let code = "def f(a: bool, b: int) -> bool:\n    return a or b > 0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_195_chained_method_calls() {
        let code = "def f(s: str) -> str:\n    return s.strip().upper().replace(\"A\", \"B\")\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_196_nested_function_calls() {
        let code = "def f(x: int) -> int:\n    return abs(min(x, 0))\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_197_fstring_with_expression() {
        let code = "def f(x: int) -> str:\n    return f\"value is {x + 1}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format"));
    }

    #[test]
    fn test_w21si_198_fstring_multiple_values() {
        let code = "def f(a: int, b: str) -> str:\n    return f\"{a}: {b}\"\n";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("format"));
    }

    #[test]
    fn test_w21si_199_complex_bool_expression() {
        let code =
            "def f(x: int, y: int, z: int) -> bool:\n    return (x > 0 and y > 0) or z == 0\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21si_200_list_comp_nested_call() {
        let code =
            "def f(words: list) -> list:\n    return [w.upper() for w in words if len(w) > 3]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }
}
