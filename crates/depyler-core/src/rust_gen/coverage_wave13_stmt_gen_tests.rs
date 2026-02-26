#[cfg(test)]
mod tests {
    use crate::DepylerPipeline;

    fn transpile(python_code: &str) -> String {
        let pipeline = DepylerPipeline::new();
        pipeline.transpile(python_code).expect("transpilation should succeed")
    }

    // Assignment Patterns (30 tests)

    #[test]
    fn test_w13sg_assign_001_simple_int_annotation() {
        let code = "x: int = 42";
        let result = transpile(code);
        assert!(result.contains("42") || result.contains("x"), "should generate assignment");
    }

    #[test]
    fn test_w13sg_assign_002_simple_str_annotation() {
        let code = "s: str = \"hello\"";
        let result = transpile(code);
        assert!(result.contains("let mut s"), "should generate string binding");
    }

    #[test]
    fn test_w13sg_assign_003_tuple_unpack_two() {
        let code = "a, b = 1, 2";
        let result = transpile(code);
        assert!(result.contains("let"), "should generate tuple unpacking");
    }

    #[test]
    fn test_w13sg_assign_004_tuple_unpack_three() {
        let code = "x, y, z = 1, 2, 3";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle three-element tuple");
    }

    #[test]
    fn test_w13sg_assign_005_augmented_add() {
        let code = "x = 5\nx += 3";
        let result = transpile(code);
        assert!(result.contains("+=") || result.contains("x = x"), "should handle += operator");
    }

    #[test]
    fn test_w13sg_assign_006_augmented_sub() {
        let code = "x = 10\nx -= 2";
        let result = transpile(code);
        assert!(result.contains("-=") || result.contains("x = x"), "should handle -= operator");
    }

    #[test]
    fn test_w13sg_assign_007_augmented_mul() {
        let code = "x = 4\nx *= 3";
        let result = transpile(code);
        assert!(
            !result.is_empty() && (result.contains("*") || result.contains("x")),
            "should handle *= operator"
        );
    }

    #[test]
    fn test_w13sg_assign_008_augmented_div() {
        let code = "x = 10\nx /= 2";
        let result = transpile(code);
        assert!(
            !result.is_empty() && (result.contains("/") || result.contains("x")),
            "should handle /= operator"
        );
    }

    #[test]
    fn test_w13sg_assign_009_augmented_floordiv() {
        let code = "x = 10\nx //= 3";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle //= operator");
    }

    #[test]
    fn test_w13sg_assign_010_augmented_mod() {
        let code = "x = 10\nx %= 3";
        let result = transpile(code);
        assert!(
            !result.is_empty() && (result.contains("%") || result.contains("x")),
            "should handle %= operator"
        );
    }

    #[test]
    fn test_w13sg_assign_011_augmented_pow() {
        let code = "x = 2\nx **= 3";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle **= operator");
    }

    #[test]
    fn test_w13sg_assign_012_augmented_bitand() {
        let code = "x = 15\nx &= 7";
        let result = transpile(code);
        assert!(
            !result.is_empty() && (result.contains("&") || result.contains("x")),
            "should handle &= operator"
        );
    }

    #[test]
    fn test_w13sg_assign_013_augmented_bitor() {
        let code = "x = 8\nx |= 4";
        let result = transpile(code);
        assert!(
            !result.is_empty() && (result.contains("|") || result.contains("x")),
            "should handle |= operator"
        );
    }

    #[test]
    fn test_w13sg_assign_014_augmented_bitxor() {
        let code = "x = 12\nx ^= 5";
        let result = transpile(code);
        assert!(
            !result.is_empty() && (result.contains("^") || result.contains("x")),
            "should handle ^= operator"
        );
    }

    #[test]
    fn test_w13sg_assign_015_augmented_lshift() {
        let code = "x = 2\nx <<= 3";
        let result = transpile(code);
        assert!(
            !result.is_empty() && (result.contains("<<") || result.contains("x")),
            "should handle <<= operator"
        );
    }

    #[test]
    fn test_w13sg_assign_016_augmented_rshift() {
        let code = "x = 16\nx >>= 2";
        let result = transpile(code);
        assert!(
            !result.is_empty() && (result.contains(">>") || result.contains("x")),
            "should handle >>= operator"
        );
    }

    #[test]
    fn test_w13sg_assign_017_chained_assignment() {
        let code = "a = b = 5";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle chained assignment");
    }

    #[test]
    fn test_w13sg_assign_018_subscript_assign_list() {
        let code = "lst = [1, 2, 3]\nlst[0] = 5";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle subscript assignment");
    }

    #[test]
    fn test_w13sg_assign_019_subscript_assign_dict() {
        let code = "d = {}\nd['key'] = 'value'";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle dict subscript assignment");
    }

    #[test]
    fn test_w13sg_assign_020_attribute_assign() {
        let code = "class C:\n    pass\nobj = C()\nobj.attr = 42";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle attribute assignment");
    }

    #[test]
    fn test_w13sg_assign_021_starred_unpack() {
        let code = "a, *b = [1, 2, 3, 4]";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle starred unpacking");
    }

    #[test]
    fn test_w13sg_assign_022_starred_middle() {
        let code = "a, *b, c = [1, 2, 3, 4, 5]";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle starred in middle");
    }

    #[test]
    fn test_w13sg_assign_023_function_call_rhs() {
        let code = "def f():\n    return 42\nx = f()";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle function call RHS");
    }

    #[test]
    fn test_w13sg_assign_024_list_comp_rhs() {
        let code = "x = [i * 2 for i in range(5)]";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle list comprehension RHS");
    }

    #[test]
    fn test_w13sg_assign_025_dict_comp_rhs() {
        let code = "x = {i: i * 2 for i in range(5)}";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle dict comprehension RHS");
    }

    #[test]
    fn test_w13sg_assign_026_lambda_rhs() {
        let code = "f = lambda x: x + 1";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle lambda RHS");
    }

    #[test]
    fn test_w13sg_assign_027_binary_expr_rhs() {
        let code = "x = 5 + 3 * 2";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle binary expression RHS");
    }

    #[test]
    fn test_w13sg_assign_028_nested_tuple_unpack() {
        let code = "(a, b), c = (1, 2), 3";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle nested tuple unpacking");
    }

    #[test]
    fn test_w13sg_assign_029_multiple_targets() {
        let code = "x = y = z = 0";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle multiple assignment targets");
    }

    #[test]
    fn test_w13sg_assign_030_bool_annotation() {
        let code = "flag: bool = True";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle bool type annotation");
    }

    // Control Flow (30 tests)

    #[test]
    fn test_w13sg_control_001_simple_if() {
        let code = "if True:\n    x = 1";
        let result = transpile(code);
        assert!(result.contains("if"), "should generate if statement");
    }

    #[test]
    fn test_w13sg_control_002_if_else() {
        let code = "if True:\n    x = 1\nelse:\n    x = 2";
        let result = transpile(code);
        assert!(result.contains("if") && result.contains("else"), "should generate if-else");
    }

    #[test]
    fn test_w13sg_control_003_if_elif() {
        let code = "if x == 1:\n    y = 1\nelif x == 2:\n    y = 2";
        let result = transpile(code);
        assert!(result.contains("if") && result.contains("else"), "should generate if-elif");
    }

    #[test]
    fn test_w13sg_control_004_if_elif_else() {
        let code = "if x == 1:\n    y = 1\nelif x == 2:\n    y = 2\nelse:\n    y = 3";
        let result = transpile(code);
        assert!(
            result.contains("if") && result.contains("else"),
            "should generate full if-elif-else"
        );
    }

    #[test]
    fn test_w13sg_control_005_nested_if_three_levels() {
        let code = "if a:\n    if b:\n        if c:\n            x = 1";
        let result = transpile(code);
        assert!(result.matches("if").count() >= 3, "should generate three nested ifs");
    }

    #[test]
    fn test_w13sg_control_006_while_simple() {
        let code = "while x < 10:\n    x += 1";
        let result = transpile(code);
        assert!(result.contains("while") || result.contains("loop"), "should generate while loop");
    }

    #[test]
    fn test_w13sg_control_007_while_break() {
        let code = "while True:\n    if x > 5:\n        break\n    x += 1";
        let result = transpile(code);
        assert!(result.contains("break"), "should generate break statement");
    }

    #[test]
    fn test_w13sg_control_008_while_continue() {
        let code =
            "while x < 10:\n    if x % 2 == 0:\n        x += 1\n        continue\n    x += 1";
        let result = transpile(code);
        assert!(result.contains("continue"), "should generate continue statement");
    }

    #[test]
    fn test_w13sg_control_009_while_else() {
        let code = "while x < 5:\n    x += 1\nelse:\n    print('done')";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle while-else");
    }

    #[test]
    fn test_w13sg_control_010_for_range() {
        let code = "for i in range(5):\n    print(i)";
        let result = transpile(code);
        assert!(result.contains("for"), "should generate for loop");
    }

    #[test]
    fn test_w13sg_control_011_for_list() {
        let code = "for item in [1, 2, 3]:\n    print(item)";
        let result = transpile(code);
        assert!(result.contains("for"), "should generate for over list");
    }

    #[test]
    fn test_w13sg_control_012_for_else() {
        let code =
            "for i in range(5):\n    if i == 3:\n        break\nelse:\n    print('completed')";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle for-else");
    }

    #[test]
    fn test_w13sg_control_013_nested_for_while() {
        let code = "for i in range(5):\n    while j < i:\n        j += 1";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle nested for-while");
    }

    #[test]
    fn test_w13sg_control_014_nested_while_for() {
        let code = "while x < 10:\n    for i in range(3):\n        x += 1";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle nested while-for");
    }

    #[test]
    fn test_w13sg_control_015_for_enumerate() {
        let code = "for i, val in enumerate([1, 2, 3]):\n    print(i, val)";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle enumerate");
    }

    #[test]
    fn test_w13sg_control_016_for_zip() {
        let code = "for a, b in zip([1, 2], [3, 4]):\n    print(a, b)";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle zip");
    }

    #[test]
    fn test_w13sg_control_017_if_multiple_elif() {
        let code = "if x == 1:\n    y = 1\nelif x == 2:\n    y = 2\nelif x == 3:\n    y = 3\nelse:\n    y = 0";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle multiple elif branches");
    }

    #[test]
    fn test_w13sg_control_018_while_nested_if() {
        let code = "while x < 10:\n    if x % 2 == 0:\n        x += 1\n    else:\n        x += 2";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle while with nested if");
    }

    #[test]
    fn test_w13sg_control_019_for_dict_items() {
        let code = "for k, v in {'a': 1}.items():\n    print(k, v)";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle dict.items()");
    }

    #[test]
    fn test_w13sg_control_020_for_dict_keys() {
        let code = "for k in {'a': 1}.keys():\n    print(k)";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle dict.keys()");
    }

    #[test]
    fn test_w13sg_control_021_for_dict_values() {
        let code = "for v in {'a': 1}.values():\n    print(v)";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle dict.values()");
    }

    #[test]
    fn test_w13sg_control_022_for_string() {
        let code = "for ch in 'hello':\n    print(ch)";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle for over string");
    }

    #[test]
    fn test_w13sg_control_023_while_compound_condition() {
        let code = "while x < 10 and y > 0:\n    x += 1\n    y -= 1";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle compound while condition");
    }

    #[test]
    fn test_w13sg_control_024_if_compound_condition() {
        let code = "if x > 0 and y < 10:\n    z = 1";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle compound if condition");
    }

    #[test]
    fn test_w13sg_control_025_nested_loops_double_for() {
        let code = "for i in range(3):\n    for j in range(3):\n        print(i, j)";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle nested for loops");
    }

    #[test]
    fn test_w13sg_control_026_if_not_condition() {
        let code = "if not x:\n    y = 1";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle not condition");
    }

    #[test]
    fn test_w13sg_control_027_if_in_condition() {
        let code = "if x in [1, 2, 3]:\n    y = 1";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle 'in' condition");
    }

    #[test]
    fn test_w13sg_control_028_if_is_condition() {
        let code = "if x is None:\n    y = 1";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle 'is' condition");
    }

    #[test]
    fn test_w13sg_control_029_for_tuple_unpack() {
        let code = "for a, b in [(1, 2), (3, 4)]:\n    print(a, b)";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle tuple unpacking in for");
    }

    #[test]
    fn test_w13sg_control_030_while_true_conditional_break() {
        let code = "while True:\n    x = input()\n    if x == 'quit':\n        break";
        let result = transpile(code);
        assert!(result.contains("break"), "should handle while True with conditional break");
    }

    // Try/Except Patterns (30 tests)

    #[test]
    fn test_w13sg_except_001_simple_try_except() {
        let code = "try:\n    x = 1\nexcept:\n    x = 0";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle simple try-except");
    }

    #[test]
    fn test_w13sg_except_002_try_except_value_error() {
        let code = "try:\n    x = int('a')\nexcept ValueError:\n    x = 0";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle ValueError exception");
    }

    #[test]
    fn test_w13sg_except_003_try_except_type_error() {
        let code = "try:\n    x = 1 + 'a'\nexcept TypeError:\n    x = 0";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle TypeError exception");
    }

    #[test]
    fn test_w13sg_except_004_try_except_key_error() {
        let code = "try:\n    x = {}['key']\nexcept KeyError:\n    x = None";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle KeyError exception");
    }

    #[test]
    fn test_w13sg_except_005_try_except_as_binding() {
        let code = "try:\n    x = int('a')\nexcept ValueError as e:\n    print(e)";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle exception binding");
    }

    #[test]
    fn test_w13sg_except_006_try_multiple_except() {
        let code = "try:\n    x = 1\nexcept ValueError:\n    x = 2\nexcept TypeError:\n    x = 3";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle multiple except clauses");
    }

    #[test]
    fn test_w13sg_except_007_try_except_else() {
        let code = "try:\n    x = 1\nexcept:\n    x = 2\nelse:\n    x = 3";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle try-except-else");
    }

    #[test]
    fn test_w13sg_except_008_try_except_finally() {
        let code = "try:\n    x = 1\nexcept:\n    x = 2\nfinally:\n    y = 0";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle try-except-finally");
    }

    #[test]
    fn test_w13sg_except_009_try_except_else_finally() {
        let code = "try:\n    x = 1\nexcept:\n    x = 2\nelse:\n    x = 3\nfinally:\n    y = 0";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle try-except-else-finally");
    }

    #[test]
    fn test_w13sg_except_010_nested_try() {
        let code = "try:\n    try:\n        x = 1\n    except:\n        x = 2\nexcept:\n    x = 3";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle nested try-except");
    }

    #[test]
    fn test_w13sg_except_011_try_in_loop() {
        let code = "for i in range(5):\n    try:\n        x = i\n    except:\n        pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle try in loop");
    }

    #[test]
    fn test_w13sg_except_012_try_except_index_error() {
        let code = "try:\n    x = [][0]\nexcept IndexError:\n    x = None";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle IndexError exception");
    }

    #[test]
    fn test_w13sg_except_013_try_except_attribute_error() {
        let code = "try:\n    x = None.foo\nexcept AttributeError:\n    x = 0";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle AttributeError exception");
    }

    #[test]
    fn test_w13sg_except_014_try_except_zero_division() {
        let code = "try:\n    x = 1 / 0\nexcept ZeroDivisionError:\n    x = 0";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle ZeroDivisionError exception");
    }

    #[test]
    fn test_w13sg_except_015_try_except_import_error() {
        let code = "try:\n    import nonexistent\nexcept ImportError:\n    pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle ImportError exception");
    }

    #[test]
    fn test_w13sg_except_016_try_except_runtime_error() {
        let code = "try:\n    raise RuntimeError()\nexcept RuntimeError:\n    pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle RuntimeError exception");
    }

    #[test]
    fn test_w13sg_except_017_try_except_exception() {
        let code = "try:\n    x = 1\nexcept Exception:\n    x = 0";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle generic Exception");
    }

    #[test]
    fn test_w13sg_except_018_try_except_tuple() {
        let code = "try:\n    x = 1\nexcept (ValueError, TypeError):\n    x = 0";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle tuple of exceptions");
    }

    #[test]
    fn test_w13sg_except_019_raise_simple() {
        let code = "raise ValueError('error')";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle raise statement");
    }

    #[test]
    fn test_w13sg_except_020_raise_no_arg() {
        let code = "try:\n    x = 1\nexcept:\n    raise";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle bare raise");
    }

    #[test]
    fn test_w13sg_except_021_try_finally_no_except() {
        let code = "try:\n    x = 1\nfinally:\n    y = 0";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle try-finally without except");
    }

    #[test]
    fn test_w13sg_except_022_try_except_name_error() {
        let code = "try:\n    x = undefined\nexcept NameError:\n    x = 0";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle NameError exception");
    }

    #[test]
    fn test_w13sg_except_023_try_except_os_error() {
        let code = "try:\n    open('nonexistent')\nexcept OSError:\n    pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle OSError exception");
    }

    #[test]
    fn test_w13sg_except_024_try_except_file_not_found() {
        let code = "try:\n    open('nonexistent')\nexcept FileNotFoundError:\n    pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle FileNotFoundError exception");
    }

    #[test]
    fn test_w13sg_except_025_nested_try_in_except() {
        let code = "try:\n    x = 1\nexcept:\n    try:\n        y = 2\n    except:\n        pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle try nested in except");
    }

    #[test]
    fn test_w13sg_except_026_try_in_while() {
        let code = "while True:\n    try:\n        break\n    except:\n        pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle try in while loop");
    }

    #[test]
    fn test_w13sg_except_027_multiple_except_with_binding() {
        let code = "try:\n    x = 1\nexcept ValueError as e1:\n    pass\nexcept TypeError as e2:\n    pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle multiple except with bindings");
    }

    #[test]
    fn test_w13sg_except_028_try_except_assertion_error() {
        let code = "try:\n    assert False\nexcept AssertionError:\n    pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle AssertionError exception");
    }

    #[test]
    fn test_w13sg_except_029_raise_from() {
        let code = "try:\n    x = 1\nexcept Exception as e:\n    raise ValueError() from e";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle raise from");
    }

    #[test]
    fn test_w13sg_except_030_try_except_stop_iteration() {
        let code = "try:\n    next(iter([]))\nexcept StopIteration:\n    pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle StopIteration exception");
    }

    // Import Patterns (20 tests)

    #[test]
    fn test_w13sg_import_001_import_os() {
        let code = "import os";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle import os");
    }

    #[test]
    fn test_w13sg_import_002_import_sys() {
        let code = "import sys";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle import sys");
    }

    #[test]
    fn test_w13sg_import_003_import_math() {
        let code = "import math";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle import math");
    }

    #[test]
    fn test_w13sg_import_004_from_os_import_path() {
        let code = "from os import path";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle from os import path");
    }

    #[test]
    fn test_w13sg_import_005_from_os_path_import_join() {
        let code = "from os.path import join";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle from os.path import join");
    }

    #[test]
    fn test_w13sg_import_006_from_os_path_multiple() {
        let code = "from os.path import join, exists";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle multiple imports from os.path");
    }

    #[test]
    fn test_w13sg_import_007_from_collections() {
        let code = "from collections import defaultdict";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle from collections import");
    }

    #[test]
    fn test_w13sg_import_008_from_collections_multiple() {
        let code = "from collections import defaultdict, OrderedDict";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle multiple collections imports");
    }

    #[test]
    fn test_w13sg_import_009_from_typing_list() {
        let code = "from typing import List";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle from typing import List");
    }

    #[test]
    fn test_w13sg_import_010_from_typing_multiple() {
        let code = "from typing import List, Dict, Optional";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle multiple typing imports");
    }

    #[test]
    fn test_w13sg_import_011_import_json() {
        let code = "import json";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle import json");
    }

    #[test]
    fn test_w13sg_import_012_import_re() {
        let code = "import re";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle import re");
    }

    #[test]
    fn test_w13sg_import_013_import_csv() {
        let code = "import csv";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle import csv");
    }

    #[test]
    fn test_w13sg_import_014_multiple_imports() {
        let code = "import os, sys, math";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle multiple imports in one statement");
    }

    #[test]
    fn test_w13sg_import_015_import_as() {
        let code = "import numpy as np";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle import with alias");
    }

    #[test]
    fn test_w13sg_import_016_from_import_as() {
        let code = "from collections import defaultdict as dd";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle from import with alias");
    }

    #[test]
    fn test_w13sg_import_017_import_datetime() {
        let code = "import datetime";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle import datetime");
    }

    #[test]
    fn test_w13sg_import_018_from_typing_tuple() {
        let code = "from typing import Tuple";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle from typing import Tuple");
    }

    #[test]
    fn test_w13sg_import_019_import_time() {
        let code = "import time";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle import time");
    }

    #[test]
    fn test_w13sg_import_020_import_random() {
        let code = "import random";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle import random");
    }

    // Class Definitions (30 tests)

    #[test]
    fn test_w13sg_class_001_simple_class() {
        let code = "class C:\n    pass";
        let result = transpile(code);
        assert!(result.contains("struct") || result.contains("C"), "should generate class");
    }

    #[test]
    fn test_w13sg_class_002_class_with_init() {
        let code = "class C:\n    def __init__(self):\n        pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should generate class with __init__");
    }

    #[test]
    fn test_w13sg_class_003_class_init_with_param() {
        let code = "class C:\n    def __init__(self, x):\n        self.x = x";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle __init__ with parameter");
    }

    #[test]
    fn test_w13sg_class_004_class_with_method() {
        let code = "class C:\n    def method(self):\n        return 42";
        let result = transpile(code);
        assert!(!result.is_empty(), "should generate class with method");
    }

    #[test]
    fn test_w13sg_class_005_class_multiple_methods() {
        let code = "class C:\n    def m1(self):\n        pass\n    def m2(self):\n        pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle multiple methods");
    }

    #[test]
    fn test_w13sg_class_006_class_with_staticmethod() {
        let code = "class C:\n    @staticmethod\n    def m():\n        return 42";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle @staticmethod");
    }

    #[test]
    fn test_w13sg_class_007_class_with_classmethod() {
        let code = "class C:\n    @classmethod\n    def m(cls):\n        return 42";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle @classmethod");
    }

    #[test]
    fn test_w13sg_class_008_class_with_property() {
        let code = "class C:\n    @property\n    def x(self):\n        return 42";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle @property");
    }

    #[test]
    fn test_w13sg_class_009_class_with_str() {
        let code = "class C:\n    def __str__(self):\n        return 'C'";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle __str__");
    }

    #[test]
    fn test_w13sg_class_010_class_with_repr() {
        let code = "class C:\n    def __repr__(self):\n        return 'C()'";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle __repr__");
    }

    #[test]
    fn test_w13sg_class_011_class_with_eq() {
        let code = "class C:\n    def __eq__(self, other):\n        return True";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle __eq__");
    }

    #[test]
    fn test_w13sg_class_012_class_with_ne() {
        let code = "class C:\n    def __ne__(self, other):\n        return False";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle __ne__");
    }

    #[test]
    fn test_w13sg_class_013_class_with_lt() {
        let code = "class C:\n    def __lt__(self, other):\n        return True";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle __lt__");
    }

    #[test]
    fn test_w13sg_class_014_class_with_gt() {
        let code = "class C:\n    def __gt__(self, other):\n        return False";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle __gt__");
    }

    #[test]
    fn test_w13sg_class_015_class_with_getitem() {
        let code = "class C:\n    def __getitem__(self, key):\n        return key";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle __getitem__");
    }

    #[test]
    fn test_w13sg_class_016_class_with_setitem() {
        let code = "class C:\n    def __setitem__(self, key, value):\n        pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle __setitem__");
    }

    #[test]
    fn test_w13sg_class_017_class_with_len() {
        let code = "class C:\n    def __len__(self):\n        return 0";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle __len__");
    }

    #[test]
    fn test_w13sg_class_018_class_with_enter_exit() {
        let code = "class C:\n    def __enter__(self):\n        return self\n    def __exit__(self, *args):\n        pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle context manager");
    }

    #[test]
    fn test_w13sg_class_019_class_with_inheritance() {
        let code = "class Base:\n    pass\nclass Derived(Base):\n    pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle inheritance");
    }

    #[test]
    fn test_w13sg_class_020_class_init_multiple_params() {
        let code =
            "class C:\n    def __init__(self, x, y):\n        self.x = x\n        self.y = y";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle __init__ with multiple params");
    }

    #[test]
    fn test_w13sg_class_021_class_with_docstring() {
        let code = "class C:\n    \"\"\"A class.\"\"\"\n    pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle class docstring");
    }

    #[test]
    fn test_w13sg_class_022_class_method_with_docstring() {
        let code = "class C:\n    def m(self):\n        \"\"\"A method.\"\"\"\n        pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle method docstring");
    }

    #[test]
    fn test_w13sg_class_023_class_with_add() {
        let code = "class C:\n    def __add__(self, other):\n        return self";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle __add__");
    }

    #[test]
    fn test_w13sg_class_024_class_with_sub() {
        let code = "class C:\n    def __sub__(self, other):\n        return self";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle __sub__");
    }

    #[test]
    fn test_w13sg_class_025_class_with_mul() {
        let code = "class C:\n    def __mul__(self, other):\n        return self";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle __mul__");
    }

    #[test]
    fn test_w13sg_class_026_class_with_call() {
        let code = "class C:\n    def __call__(self):\n        return 42";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle __call__");
    }

    #[test]
    fn test_w13sg_class_027_class_with_iter() {
        let code = "class C:\n    def __iter__(self):\n        return self";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle __iter__");
    }

    #[test]
    fn test_w13sg_class_028_class_with_next() {
        let code = "class C:\n    def __next__(self):\n        raise StopIteration";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle __next__");
    }

    #[test]
    fn test_w13sg_class_029_class_with_hash() {
        let code = "class C:\n    def __hash__(self):\n        return 0";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle __hash__");
    }

    #[test]
    fn test_w13sg_class_030_class_with_bool() {
        let code = "class C:\n    def __bool__(self):\n        return True";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle __bool__");
    }

    // With Statement (20 tests)

    #[test]
    fn test_w13sg_with_001_open_read() {
        let code = "with open('file.txt') as f:\n    content = f.read()";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle with open");
    }

    #[test]
    fn test_w13sg_with_002_open_write() {
        let code = "with open('file.txt', 'w') as f:\n    f.write('hello')";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle with open write mode");
    }

    #[test]
    fn test_w13sg_with_003_open_append() {
        let code = "with open('file.txt', 'a') as f:\n    f.write('hello')";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle with open append mode");
    }

    #[test]
    fn test_w13sg_with_004_nested_with() {
        let code = "with open('f1.txt') as f1:\n    with open('f2.txt') as f2:\n        pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle nested with statements");
    }

    #[test]
    fn test_w13sg_with_005_with_no_as() {
        let code = "with open('file.txt'):\n    pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle with without as clause");
    }

    #[test]
    fn test_w13sg_with_006_multiple_context_managers() {
        let code = "with open('f1.txt') as f1, open('f2.txt') as f2:\n    pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle multiple context managers");
    }

    #[test]
    fn test_w13sg_with_007_with_custom_manager() {
        let code = "class CM:\n    def __enter__(self):\n        return self\n    def __exit__(self, *args):\n        pass\nwith CM() as cm:\n    pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle custom context manager");
    }

    #[test]
    fn test_w13sg_with_008_with_lock() {
        let code = "import threading\nlock = threading.Lock()\nwith lock:\n    x = 1";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle with lock");
    }

    #[test]
    fn test_w13sg_with_009_with_in_function() {
        let code = "def f():\n    with open('file.txt') as f:\n        return f.read()";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle with in function");
    }

    #[test]
    fn test_w13sg_with_010_with_in_loop() {
        let code = "for i in range(5):\n    with open(f'file{i}.txt') as f:\n        pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle with in loop");
    }

    #[test]
    fn test_w13sg_with_011_with_readline() {
        let code = "with open('file.txt') as f:\n    line = f.readline()";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle readline in with");
    }

    #[test]
    fn test_w13sg_with_012_with_readlines() {
        let code = "with open('file.txt') as f:\n    lines = f.readlines()";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle readlines in with");
    }

    #[test]
    fn test_w13sg_with_013_with_iteration() {
        let code = "with open('file.txt') as f:\n    for line in f:\n        print(line)";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle iteration in with");
    }

    #[test]
    fn test_w13sg_with_014_with_binary_mode() {
        let code = "with open('file.bin', 'rb') as f:\n    data = f.read()";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle binary read mode");
    }

    #[test]
    fn test_w13sg_with_015_with_binary_write() {
        let code = "with open('file.bin', 'wb') as f:\n    f.write(b'data')";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle binary write mode");
    }

    #[test]
    fn test_w13sg_with_016_with_exception_handling() {
        let code = "try:\n    with open('file.txt') as f:\n        content = f.read()\nexcept IOError:\n    pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle with in try-except");
    }

    #[test]
    fn test_w13sg_with_017_with_multiple_statements() {
        let code = "with open('file.txt') as f:\n    x = 1\n    y = 2\n    z = f.read()";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle multiple statements in with");
    }

    #[test]
    fn test_w13sg_with_018_with_conditional() {
        let code = "with open('file.txt') as f:\n    if True:\n        content = f.read()";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle conditional in with");
    }

    #[test]
    fn test_w13sg_with_019_with_return() {
        let code = "def f():\n    with open('file.txt') as file:\n        if True:\n            return file.read()";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle return in with");
    }

    #[test]
    fn test_w13sg_with_020_with_encoding() {
        let code = "with open('file.txt', encoding='utf-8') as f:\n    content = f.read()";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle with encoding parameter");
    }

    // Complex Statements (20 tests)

    #[test]
    fn test_w13sg_complex_001_assert_with_message() {
        let code = "assert x > 0, 'x must be positive'";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle assert with message");
    }

    #[test]
    fn test_w13sg_complex_002_assert_no_message() {
        let code = "assert x > 0";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle assert without message");
    }

    #[test]
    fn test_w13sg_complex_003_global_declaration() {
        let code = "x = 0\ndef f():\n    global x\n    x = 1";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle global declaration");
    }

    #[test]
    fn test_w13sg_complex_004_del_variable() {
        let code = "x = 1\ndel x";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle del statement");
    }

    #[test]
    fn test_w13sg_complex_005_del_list_item() {
        let code = "lst = [1, 2, 3]\ndel lst[0]";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle del on list item");
    }

    #[test]
    fn test_w13sg_complex_006_del_dict_item() {
        let code = "d = {'a': 1}\ndel d['a']";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle del on dict item");
    }

    #[test]
    fn test_w13sg_complex_007_pass_in_if() {
        let code = "if True:\n    pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle pass in if");
    }

    #[test]
    fn test_w13sg_complex_008_pass_in_function() {
        let code = "def f():\n    pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle pass in function");
    }

    #[test]
    fn test_w13sg_complex_009_pass_in_class() {
        let code = "class C:\n    pass";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle pass in class");
    }

    #[test]
    fn test_w13sg_complex_010_print_simple() {
        let code = "print('hello')";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle print statement");
    }

    #[test]
    fn test_w13sg_complex_011_print_multiple_args() {
        let code = "print('hello', 'world')";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle print with multiple args");
    }

    #[test]
    fn test_w13sg_complex_012_return_value() {
        let code = "def f():\n    return 42";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle return with value");
    }

    #[test]
    fn test_w13sg_complex_013_return_none() {
        let code = "def f():\n    return";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle return without value");
    }

    #[test]
    fn test_w13sg_complex_014_multiple_returns() {
        let code = "def f(x):\n    if x > 0:\n        return 1\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle multiple returns");
    }

    #[test]
    fn test_w13sg_complex_015_yield_value() {
        let code = "def f():\n    yield 42";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle yield statement");
    }

    #[test]
    fn test_w13sg_complex_016_yield_in_loop() {
        let code = "def f():\n    for i in range(5):\n        yield i";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle yield in loop");
    }

    #[test]
    fn test_w13sg_complex_017_assert_complex_condition() {
        let code = "assert x > 0 and y < 10";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle assert with complex condition");
    }

    #[test]
    fn test_w13sg_complex_018_nonlocal_declaration() {
        let code = "def outer():\n    x = 1\n    def inner():\n        nonlocal x\n        x = 2";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle nonlocal declaration");
    }

    #[test]
    fn test_w13sg_complex_019_return_tuple() {
        let code = "def f():\n    return 1, 2";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle return tuple");
    }

    #[test]
    fn test_w13sg_complex_020_yield_from() {
        let code = "def f():\n    yield from [1, 2, 3]";
        let result = transpile(code);
        assert!(!result.is_empty(), "should handle yield from");
    }

    // Comprehension Statements (20 tests)

    #[test]
    fn test_w13sg_comp_001_list_comp_simple() {
        let code = "result = [x for x in range(5)]";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle simple list comprehension");
    }

    #[test]
    fn test_w13sg_comp_002_list_comp_with_condition() {
        let code = "result = [x for x in range(10) if x % 2 == 0]";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle list comp with condition");
    }

    #[test]
    fn test_w13sg_comp_003_list_comp_with_transform() {
        let code = "result = [x * 2 for x in range(5)]";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle list comp with transform");
    }

    #[test]
    fn test_w13sg_comp_004_dict_comp_simple() {
        let code = "result = {x: x * 2 for x in range(5)}";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle simple dict comprehension");
    }

    #[test]
    fn test_w13sg_comp_005_dict_comp_with_condition() {
        let code = "result = {x: x * 2 for x in range(10) if x % 2 == 0}";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle dict comp with condition");
    }

    #[test]
    fn test_w13sg_comp_006_set_comp_simple() {
        let code = "result = {x for x in range(5)}";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle simple set comprehension");
    }

    #[test]
    fn test_w13sg_comp_007_set_comp_with_condition() {
        let code = "result = {x for x in range(10) if x % 2 == 0}";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle set comp with condition");
    }

    #[test]
    fn test_w13sg_comp_008_nested_list_comp() {
        let code = "result = [[y for y in range(3)] for x in range(3)]";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle nested list comprehension");
    }

    #[test]
    fn test_w13sg_comp_009_list_comp_multiple_conditions() {
        let code = "result = [x for x in range(20) if x % 2 == 0 if x % 3 == 0]";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle list comp with multiple conditions");
    }

    #[test]
    fn test_w13sg_comp_010_dict_comp_from_zip() {
        let code = "result = {k: v for k, v in zip([1, 2], [3, 4])}";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle dict comp from zip");
    }

    #[test]
    fn test_w13sg_comp_011_generator_in_sum() {
        let code = "result = sum(x for x in range(10))";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle generator expression in sum");
    }

    #[test]
    fn test_w13sg_comp_012_generator_in_max() {
        let code = "result = max(x for x in range(10))";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle generator expression in max");
    }

    #[test]
    fn test_w13sg_comp_013_generator_in_min() {
        let code = "result = min(x for x in range(10) if x > 0)";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle generator expression in min");
    }

    #[test]
    fn test_w13sg_comp_014_generator_in_any() {
        let code = "result = any(x > 5 for x in range(10))";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle generator expression in any");
    }

    #[test]
    fn test_w13sg_comp_015_generator_in_all() {
        let code = "result = all(x > 0 for x in range(1, 10))";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle generator expression in all");
    }

    #[test]
    fn test_w13sg_comp_016_list_comp_with_method_call() {
        let code = "result = [s.upper() for s in ['a', 'b', 'c']]";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle list comp with method call");
    }

    #[test]
    fn test_w13sg_comp_017_dict_comp_with_str_keys() {
        let code = "result = {str(x): x for x in range(5)}";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle dict comp with str keys");
    }

    #[test]
    fn test_w13sg_comp_018_set_comp_with_transform() {
        let code = "result = {x * x for x in range(5)}";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle set comp with transform");
    }

    #[test]
    fn test_w13sg_comp_019_list_comp_nested_loop() {
        let code = "result = [x + y for x in range(3) for y in range(3)]";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle list comp with nested loop");
    }

    #[test]
    fn test_w13sg_comp_020_generator_in_list() {
        let code = "result = list(x * 2 for x in range(5))";
        let result = transpile(code);
        assert!(result.contains("let"), "should handle generator expression in list()");
    }
}
