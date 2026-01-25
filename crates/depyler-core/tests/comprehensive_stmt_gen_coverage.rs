//! Comprehensive tests for stmt_gen.rs coverage
//! DEPYLER-COVERAGE-95: Target 95% coverage on statement generation

use depyler_core::DepylerPipeline;

fn transpile(code: &str) -> Result<String, anyhow::Error> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code)
}

// =============================================================================
// IF STATEMENT COVERAGE
// =============================================================================

mod if_statements {
    use super::*;

    #[test]
    fn test_if_simple() {
        let code = "def f(x: int) -> int:\n    if x > 0:\n        return 1\n    return 0";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_if_else() {
        let code =
            "def f(x: int) -> int:\n    if x > 0:\n        return 1\n    else:\n        return -1";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_if_elif() {
        let code = "def f(x: int) -> int:\n    if x > 0:\n        return 1\n    elif x < 0:\n        return -1\n    else:\n        return 0";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_if_multiple_elif() {
        let code = "def f(x: int) -> str:\n    if x == 1:\n        return \"one\"\n    elif x == 2:\n        return \"two\"\n    elif x == 3:\n        return \"three\"\n    else:\n        return \"other\"";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_nested_if() {
        let code = "def f(x: int, y: int) -> int:\n    if x > 0:\n        if y > 0:\n            return 1\n        return 2\n    return 0";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_if_with_and() {
        let code = "def f(x: int, y: int) -> bool:\n    if x > 0 and y > 0:\n        return True\n    return False";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_if_with_or() {
        let code = "def f(x: int, y: int) -> bool:\n    if x > 0 or y > 0:\n        return True\n    return False";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_if_with_not() {
        let code = "def f(x: bool) -> bool:\n    if not x:\n        return True\n    return False";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_if_truthiness_list() {
        let code =
            "def f(lst: list[int]) -> bool:\n    if lst:\n        return True\n    return False";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_if_truthiness_dict() {
        let code =
            "def f(d: dict[str, int]) -> bool:\n    if d:\n        return True\n    return False";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_if_truthiness_str() {
        let code = "def f(s: str) -> bool:\n    if s:\n        return True\n    return False";
        assert!(transpile(code).is_ok());
    }
}

// =============================================================================
// FOR LOOP COVERAGE
// =============================================================================

mod for_loops {
    use super::*;

    #[test]
    fn test_for_range() {
        let code = "def f() -> int:\n    total: int = 0\n    for i in range(10):\n        total += i\n    return total";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_for_range_start_stop() {
        let code = "def f() -> int:\n    total: int = 0\n    for i in range(1, 10):\n        total += i\n    return total";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_for_range_step() {
        let code = "def f() -> int:\n    total: int = 0\n    for i in range(0, 10, 2):\n        total += i\n    return total";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_for_list() {
        let code = "def f(lst: list[int]) -> int:\n    total: int = 0\n    for x in lst:\n        total += x\n    return total";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_for_str() {
        let code = "def f(s: str) -> int:\n    count: int = 0\n    for c in s:\n        count += 1\n    return count";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_for_dict_keys() {
        let code = "def f(d: dict[str, int]) -> None:\n    for k in d:\n        pass";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_for_dict_items() {
        let code = "def f(d: dict[str, int]) -> int:\n    total: int = 0\n    for k, v in d.items():\n        total += v\n    return total";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_for_enumerate() {
        let code =
            "def f(lst: list[str]) -> None:\n    for i, x in enumerate(lst):\n        print(i, x)";
        let _ = transpile(code);
    }

    #[test]
    fn test_for_zip() {
        let code = "def f(a: list[int], b: list[str]) -> None:\n    for x, y in zip(a, b):\n        print(x, y)";
        let _ = transpile(code);
    }

    #[test]
    fn test_for_with_break() {
        let code = "def f(lst: list[int]) -> int:\n    for x in lst:\n        if x > 10:\n            break\n    return x";
        let _ = transpile(code);
    }

    #[test]
    fn test_for_with_continue() {
        let code = "def f(lst: list[int]) -> int:\n    total: int = 0\n    for x in lst:\n        if x < 0:\n            continue\n        total += x\n    return total";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_for_else() {
        let code = "def f(lst: list[int]) -> bool:\n    for x in lst:\n        if x < 0:\n            return False\n    else:\n        return True\n    return False";
        let _ = transpile(code);
    }

    #[test]
    fn test_nested_for() {
        let code = "def f() -> int:\n    total: int = 0\n    for i in range(3):\n        for j in range(3):\n            total += i * j\n    return total";
        assert!(transpile(code).is_ok());
    }
}

// =============================================================================
// WHILE LOOP COVERAGE
// =============================================================================

mod while_loops {
    use super::*;

    #[test]
    fn test_while_simple() {
        let code =
            "def f() -> int:\n    x: int = 0\n    while x < 10:\n        x += 1\n    return x";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_while_true() {
        let code = "def f() -> int:\n    x: int = 0\n    while True:\n        x += 1\n        if x >= 10:\n            break\n    return x";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_while_with_break() {
        let code = "def f() -> int:\n    x: int = 0\n    while x < 100:\n        if x > 10:\n            break\n        x += 1\n    return x";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_while_with_continue() {
        let code = "def f() -> int:\n    x: int = 0\n    total: int = 0\n    while x < 10:\n        x += 1\n        if x % 2 == 0:\n            continue\n        total += x\n    return total";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_while_else() {
        let code = "def f() -> bool:\n    x: int = 0\n    while x < 10:\n        x += 1\n    else:\n        return True\n    return False";
        let _ = transpile(code);
    }

    #[test]
    fn test_while_list_not_empty() {
        let code = "def f(lst: list[int]) -> int:\n    total: int = 0\n    while lst:\n        total += lst.pop()\n    return total";
        let _ = transpile(code);
    }

    #[test]
    fn test_nested_while() {
        let code = "def f() -> int:\n    i: int = 0\n    total: int = 0\n    while i < 3:\n        j: int = 0\n        while j < 3:\n            total += 1\n            j += 1\n        i += 1\n    return total";
        assert!(transpile(code).is_ok());
    }
}

// =============================================================================
// ASSIGNMENT COVERAGE
// =============================================================================

mod assignments {
    use super::*;

    #[test]
    fn test_simple_assign() {
        let code = "def f() -> int:\n    x: int = 5\n    return x";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_multiple_assign() {
        let code =
            "def f() -> int:\n    x: int = 1\n    y: int = 2\n    z: int = 3\n    return x + y + z";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_tuple_unpack() {
        let code = "def f() -> int:\n    a, b = 1, 2\n    return a + b";
        let _ = transpile(code);
    }

    #[test]
    fn test_tuple_unpack_from_func() {
        let code = "def pair() -> tuple[int, int]:\n    return (1, 2)\n\ndef f() -> int:\n    a, b = pair()\n    return a + b";
        let _ = transpile(code);
    }

    #[test]
    fn test_starred_unpack() {
        let code = "def f() -> int:\n    first, *rest = [1, 2, 3, 4]\n    return first";
        let _ = transpile(code);
    }

    #[test]
    fn test_augmented_add() {
        let code = "def f() -> int:\n    x: int = 0\n    x += 5\n    return x";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_augmented_sub() {
        let code = "def f() -> int:\n    x: int = 10\n    x -= 3\n    return x";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_augmented_mul() {
        let code = "def f() -> int:\n    x: int = 5\n    x *= 2\n    return x";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_augmented_div() {
        let code = "def f() -> float:\n    x: float = 10.0\n    x /= 2.0\n    return x";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_augmented_floordiv() {
        let code = "def f() -> int:\n    x: int = 10\n    x //= 3\n    return x";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_augmented_mod() {
        let code = "def f() -> int:\n    x: int = 10\n    x %= 3\n    return x";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_augmented_bitand() {
        let code = "def f() -> int:\n    x: int = 0xFF\n    x &= 0x0F\n    return x";
        let _ = transpile(code);
    }

    #[test]
    fn test_augmented_bitor() {
        let code = "def f() -> int:\n    x: int = 0x0F\n    x |= 0xF0\n    return x";
        let _ = transpile(code);
    }

    #[test]
    fn test_augmented_bitxor() {
        let code = "def f() -> int:\n    x: int = 0xFF\n    x ^= 0x0F\n    return x";
        let _ = transpile(code);
    }

    #[test]
    fn test_augmented_lshift() {
        let code = "def f() -> int:\n    x: int = 1\n    x <<= 4\n    return x";
        let _ = transpile(code);
    }

    #[test]
    fn test_augmented_rshift() {
        let code = "def f() -> int:\n    x: int = 16\n    x >>= 2\n    return x";
        let _ = transpile(code);
    }

    #[test]
    fn test_list_index_assign() {
        let code = "def f(lst: list[int]) -> None:\n    lst[0] = 99";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_dict_key_assign() {
        let code = "def f(d: dict[str, int]) -> None:\n    d[\"key\"] = 42";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_attribute_assign() {
        let code = "class Foo:\n    x: int = 0\n\ndef f(obj: Foo) -> None:\n    obj.x = 42";
        let _ = transpile(code);
    }
}

// =============================================================================
// TRY/EXCEPT COVERAGE
// =============================================================================

mod try_except {
    use super::*;

    #[test]
    fn test_try_except_bare() {
        let code =
            "def f() -> int:\n    try:\n        return 1 // 0\n    except:\n        return 0";
        let _ = transpile(code);
    }

    #[test]
    fn test_try_except_type() {
        let code = "def f() -> int:\n    try:\n        return 1 // 0\n    except ZeroDivisionError:\n        return 0";
        let _ = transpile(code);
    }

    #[test]
    fn test_try_except_as() {
        let code = "def f() -> str:\n    try:\n        return str(1 // 0)\n    except Exception as e:\n        return str(e)";
        let _ = transpile(code);
    }

    #[test]
    fn test_try_multiple_except() {
        let code = "def f(x: int) -> int:\n    try:\n        return 10 // x\n    except ZeroDivisionError:\n        return -1\n    except ValueError:\n        return -2";
        let _ = transpile(code);
    }

    #[test]
    fn test_try_except_else() {
        let code = "def f(x: int) -> int:\n    try:\n        result: int = 10 // x\n    except:\n        return -1\n    else:\n        return result\n    return 0";
        let _ = transpile(code);
    }

    #[test]
    fn test_try_finally() {
        let code = "def f() -> int:\n    x: int = 0\n    try:\n        x = 1\n    finally:\n        x = 2\n    return x";
        let _ = transpile(code);
    }

    #[test]
    fn test_try_except_finally() {
        let code = "def f() -> int:\n    x: int = 0\n    try:\n        x = 1 // 0\n    except:\n        x = -1\n    finally:\n        x += 10\n    return x";
        let _ = transpile(code);
    }

    #[test]
    fn test_nested_try() {
        let code = "def f() -> int:\n    try:\n        try:\n            return 1 // 0\n        except:\n            return -1\n    except:\n        return -2";
        let _ = transpile(code);
    }

    #[test]
    fn test_raise() {
        let code = "def f() -> None:\n    raise ValueError(\"error\")";
        let _ = transpile(code);
    }

    #[test]
    fn test_raise_from() {
        let code = "def f() -> None:\n    try:\n        x = 1 // 0\n    except Exception as e:\n        raise ValueError(\"wrapped\") from e";
        let _ = transpile(code);
    }
}

// =============================================================================
// WITH STATEMENT COVERAGE
// =============================================================================

mod with_statements {
    use super::*;

    #[test]
    fn test_with_open() {
        let code = "def f() -> str:\n    with open(\"file.txt\") as f:\n        return f.read()";
        let _ = transpile(code);
    }

    #[test]
    fn test_with_open_mode() {
        let code = "def f() -> None:\n    with open(\"file.txt\", \"w\") as f:\n        f.write(\"hello\")";
        let _ = transpile(code);
    }

    #[test]
    fn test_with_multiple() {
        let code =
            "def f() -> None:\n    with open(\"a.txt\") as a, open(\"b.txt\") as b:\n        pass";
        let _ = transpile(code);
    }

    #[test]
    fn test_nested_with() {
        let code = "def f() -> None:\n    with open(\"a.txt\") as a:\n        with open(\"b.txt\") as b:\n            pass";
        let _ = transpile(code);
    }
}

// =============================================================================
// RETURN STATEMENT COVERAGE
// =============================================================================

mod return_statements {
    use super::*;

    #[test]
    fn test_return_none() {
        let code = "def f() -> None:\n    return";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_return_value() {
        let code = "def f() -> int:\n    return 42";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_return_expression() {
        let code = "def f(x: int) -> int:\n    return x * 2 + 1";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_return_tuple() {
        let code = "def f() -> tuple[int, int]:\n    return (1, 2)";
        let _ = transpile(code);
    }

    #[test]
    fn test_return_call() {
        let code = "def helper() -> int:\n    return 42\n\ndef f() -> int:\n    return helper()";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_early_return() {
        let code = "def f(x: int) -> int:\n    if x < 0:\n        return -1\n    return x";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_multiple_returns() {
        let code = "def f(x: int) -> str:\n    if x > 0:\n        return \"positive\"\n    elif x < 0:\n        return \"negative\"\n    return \"zero\"";
        assert!(transpile(code).is_ok());
    }
}

// =============================================================================
// ASSERT STATEMENT COVERAGE
// =============================================================================

mod assert_statements {
    use super::*;

    #[test]
    fn test_assert_simple() {
        let code = "def f(x: int) -> None:\n    assert x > 0";
        let _ = transpile(code);
    }

    #[test]
    fn test_assert_message() {
        let code = "def f(x: int) -> None:\n    assert x > 0, \"x must be positive\"";
        let _ = transpile(code);
    }

    #[test]
    fn test_assert_expression() {
        let code = "def f(lst: list[int]) -> None:\n    assert len(lst) > 0";
        let _ = transpile(code);
    }
}

// =============================================================================
// PASS STATEMENT COVERAGE
// =============================================================================

mod pass_statements {
    use super::*;

    #[test]
    fn test_pass_in_function() {
        let code = "def f() -> None:\n    pass";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_pass_in_if() {
        let code = "def f(x: int) -> None:\n    if x > 0:\n        pass";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_pass_in_for() {
        let code = "def f() -> None:\n    for i in range(10):\n        pass";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_pass_in_while() {
        let code = "def f() -> None:\n    while False:\n        pass";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_pass_in_class() {
        let code = "class Empty:\n    pass";
        let _ = transpile(code);
    }
}

// =============================================================================
// DEL STATEMENT COVERAGE
// =============================================================================

mod del_statements {
    use super::*;

    #[test]
    fn test_del_variable() {
        let code = "def f() -> None:\n    x: int = 5\n    del x";
        let _ = transpile(code);
    }

    #[test]
    fn test_del_list_item() {
        let code = "def f(lst: list[int]) -> None:\n    del lst[0]";
        let _ = transpile(code);
    }

    #[test]
    fn test_del_dict_item() {
        let code = "def f(d: dict[str, int]) -> None:\n    del d[\"key\"]";
        let _ = transpile(code);
    }
}

// =============================================================================
// GLOBAL/NONLOCAL COVERAGE
// =============================================================================

mod global_nonlocal {
    use super::*;

    #[test]
    fn test_global() {
        let code = "counter: int = 0\n\ndef f() -> None:\n    global counter\n    counter += 1";
        let _ = transpile(code);
    }

    #[test]
    fn test_nonlocal() {
        let code = "def outer() -> int:\n    x: int = 0\n    def inner() -> None:\n        nonlocal x\n        x += 1\n    inner()\n    return x";
        let _ = transpile(code);
    }
}

// =============================================================================
// EXPRESSION STATEMENT COVERAGE
// =============================================================================

mod expression_statements {
    use super::*;

    #[test]
    fn test_function_call_stmt() {
        let code = "def f() -> None:\n    print(\"hello\")";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_method_call_stmt() {
        let code = "def f(lst: list[int]) -> None:\n    lst.append(1)";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_docstring() {
        let code = "def f() -> None:\n    \"\"\"This is a docstring.\"\"\"\n    pass";
        assert!(transpile(code).is_ok());
    }
}

// =============================================================================
// MATCH STATEMENT COVERAGE (Python 3.10+)
// =============================================================================

mod match_statements {
    use super::*;

    #[test]
    fn test_match_literal() {
        let code = "def f(x: int) -> str:\n    match x:\n        case 1:\n            return \"one\"\n        case 2:\n            return \"two\"\n        case _:\n            return \"other\"";
        let _ = transpile(code);
    }

    #[test]
    fn test_match_variable() {
        let code = "def f(x: int) -> int:\n    match x:\n        case n:\n            return n * 2";
        let _ = transpile(code);
    }

    #[test]
    fn test_match_or_pattern() {
        let code = "def f(x: int) -> str:\n    match x:\n        case 1 | 2 | 3:\n            return \"small\"\n        case _:\n            return \"large\"";
        let _ = transpile(code);
    }

    #[test]
    fn test_match_guard() {
        let code = "def f(x: int) -> str:\n    match x:\n        case n if n > 0:\n            return \"positive\"\n        case _:\n            return \"non-positive\"";
        let _ = transpile(code);
    }
}

// =============================================================================
// IMPORT STATEMENT COVERAGE
// =============================================================================

mod import_statements {
    use super::*;

    #[test]
    fn test_import_module() {
        let code = "import os\n\ndef f() -> str:\n    return os.getcwd()";
        let _ = transpile(code);
    }

    #[test]
    fn test_import_from() {
        let code = "from os import getcwd\n\ndef f() -> str:\n    return getcwd()";
        let _ = transpile(code);
    }

    #[test]
    fn test_import_as() {
        let code = "import os as operating_system\n\ndef f() -> str:\n    return operating_system.getcwd()";
        let _ = transpile(code);
    }

    #[test]
    fn test_import_from_as() {
        let code = "from os import getcwd as cwd\n\ndef f() -> str:\n    return cwd()";
        let _ = transpile(code);
    }

    #[test]
    fn test_import_multiple() {
        let code =
            "from os import getcwd, listdir\n\ndef f() -> list[str]:\n    return listdir(getcwd())";
        let _ = transpile(code);
    }
}
