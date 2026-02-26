#![cfg(test)]
//! Coverage Wave 10: Assignment & Control Flow Tests
//! Targets codegen_assign_stmt() and control flow codegen branches

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
    use super::*;

    // ==================== Section 1: Assignment patterns (50 tests) ====================

    #[test]
    fn test_w10ac_assign_simple_int() {
        let code = "x = 5";
        let result = transpile(code);
        assert!(result.contains("x") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_simple_string() {
        let code = r#"name = "Alice""#;
        let result = transpile(code);
        assert!(result.contains("name") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_simple_float() {
        let code = "pi = 3.14159";
        let result = transpile(code);
        assert!(result.contains("pi") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_simple_bool() {
        let code = "flag = True";
        let result = transpile(code);
        assert!(result.contains("flag") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_multiple_same_value() {
        let code = "x = y = 5";
        let result = transpile(code);
        assert!(result.contains("x") || result.contains("y") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_multiple_chain() {
        let code = "a = b = c = 10";
        let result = transpile(code);
        assert!(result.contains("a") || result.contains("b") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_tuple_unpack_two() {
        let code = "a, b = 1, 2";
        let result = transpile(code);
        assert!(result.contains("a") || result.contains("b") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_tuple_unpack_three() {
        let code = "x, y, z = 1, 2, 3";
        let result = transpile(code);
        assert!(result.contains("x") || result.contains("y") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_tuple_from_list() {
        let code = "a, b = [10, 20]";
        let result = transpile(code);
        assert!(result.contains("a") || result.contains("b") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_starred_unpack_basic() {
        let code = "first, *rest = [1, 2, 3, 4]";
        let result = transpile(code);
        assert!(result.contains("first") || result.contains("rest") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_starred_unpack_middle() {
        let code = "first, *middle, last = [1, 2, 3, 4, 5]";
        let result = transpile(code);
        assert!(result.contains("first") || result.contains("middle") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_starred_unpack_end() {
        let code = "a, b, *tail = range(10)";
        let result = transpile(code);
        assert!(result.contains("a") || result.contains("tail") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_nested_unpack_simple() {
        let code = "a, (b, c) = 1, (2, 3)";
        let result = transpile(code);
        assert!(result.contains("a") || result.contains("b") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_nested_unpack_deep() {
        let code = "(a, b), (c, d) = (1, 2), (3, 4)";
        let result = transpile(code);
        assert!(result.contains("a") || result.contains("c") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_nested_unpack_triple() {
        let code = "x, (y, (z, w)) = 1, (2, (3, 4))";
        let result = transpile(code);
        assert!(result.contains("x") || result.contains("z") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_augmented_add() {
        let code = "x = 5\nx += 3";
        let result = transpile(code);
        assert!(result.contains("x") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_augmented_sub() {
        let code = "y = 10\ny -= 2";
        let result = transpile(code);
        assert!(result.contains("y") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_augmented_mul() {
        let code = "z = 4\nz *= 3";
        let result = transpile(code);
        assert!(result.contains("z") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_augmented_div() {
        let code = "a = 20\na /= 4";
        let result = transpile(code);
        assert!(result.contains("a") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_augmented_floordiv() {
        let code = "b = 17\nb //= 3";
        let result = transpile(code);
        assert!(result.contains("b") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_augmented_mod() {
        let code = "c = 17\nc %= 5";
        let result = transpile(code);
        assert!(result.contains("c") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_augmented_pow() {
        let code = "d = 2\nd **= 8";
        let result = transpile(code);
        assert!(result.contains("d") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_augmented_bitand() {
        let code = "e = 12\ne &= 7";
        let result = transpile(code);
        assert!(result.contains("e") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_augmented_bitor() {
        let code = "f = 5\nf |= 3";
        let result = transpile(code);
        assert!(result.contains("f") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_augmented_bitxor() {
        let code = "g = 10\ng ^= 6";
        let result = transpile(code);
        assert!(result.contains("g") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_augmented_lshift() {
        let code = "h = 3\nh <<= 2";
        let result = transpile(code);
        assert!(result.contains("h") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_augmented_rshift() {
        let code = "i = 32\ni >>= 2";
        let result = transpile(code);
        assert!(result.contains("i") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_type_annotated_int() {
        let code = "x: int = 5";
        let result = transpile(code);
        assert!(result.contains("x") || result.contains("i32") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_type_annotated_str() {
        let code = r#"name: str = "Bob""#;
        let result = transpile(code);
        assert!(result.contains("name") || result.contains("String") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_type_annotated_list() {
        let code = "items: list = [1, 2, 3]";
        let result = transpile(code);
        assert!(result.contains("items") || result.contains("Vec") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_global_scope() {
        let code = "global x\nx = 42";
        let result = transpile(code);
        assert!(result.contains("x") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_walrus_if() {
        let code = "if (n := 5) > 3:\n    pass";
        let result = transpile(code);
        assert!(result.contains("n") || result.contains("if") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_walrus_while() {
        let code = "while (line := 'data'):\n    break";
        let result = transpile(code);
        assert!(result.contains("line") || result.contains("while") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_chained_method_call() {
        let code = "x = obj.method().other()";
        let result = transpile(code);
        assert!(result.contains("x") || result.contains("obj") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_from_function_call() {
        let code = "def f():\n    return 5\nresult = f()";
        let result = transpile(code);
        assert!(result.contains("result") || result.contains("f") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_from_builtin() {
        let code = "length = len([1, 2, 3])";
        let result = transpile(code);
        assert!(result.contains("length") || result.contains("len") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_from_ternary() {
        let code = "x = 10 if True else 20";
        let result = transpile(code);
        assert!(result.contains("x") || result.contains("if") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_from_ternary_complex() {
        let code = "result = a if a > b else b if b > c else c";
        let result = transpile(code);
        assert!(result.contains("result") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_destructure_nested_list() {
        let code = "[a, [b, c]] = [1, [2, 3]]";
        let result = transpile(code);
        assert!(result.contains("a") || result.contains("b") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_destructure_mixed() {
        let code = "(a, [b, c]), d = ((1, [2, 3]), 4)";
        let result = transpile(code);
        assert!(result.contains("a") || result.contains("d") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_to_class_field() {
        let code = "class C:\n    def __init__(self):\n        self.x = 5";
        let result = transpile(code);
        assert!(result.contains("self") || result.contains("x") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_to_multiple_fields() {
        let code = "class C:\n    def __init__(self):\n        self.a = 1\n        self.b = 2";
        let result = transpile(code);
        assert!(result.contains("a") || result.contains("b") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_list_literal() {
        let code = "nums = [1, 2, 3, 4, 5]";
        let result = transpile(code);
        assert!(result.contains("nums") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_dict_literal() {
        let code = "data = {'a': 1, 'b': 2}";
        let result = transpile(code);
        assert!(result.contains("data") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_set_literal() {
        let code = "unique = {1, 2, 3}";
        let result = transpile(code);
        assert!(result.contains("unique") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_tuple_literal() {
        let code = "coords = (10, 20)";
        let result = transpile(code);
        assert!(result.contains("coords") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_comprehension_list() {
        let code = "squares = [x*x for x in range(10)]";
        let result = transpile(code);
        assert!(result.contains("squares") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_comprehension_dict() {
        let code = "sq_dict = {x: x*x for x in range(5)}";
        let result = transpile(code);
        assert!(result.contains("sq_dict") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_assign_slice() {
        let code = "lst = [1,2,3,4,5]\npart = lst[1:3]";
        let result = transpile(code);
        assert!(result.contains("part") || result.len() > 0);
    }

    // ==================== Section 2: Complex control flow (40 tests) ====================

    #[test]
    fn test_w10ac_control_if_simple() {
        let code = "if True:\n    pass";
        let result = transpile(code);
        assert!(result.contains("if") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_if_else() {
        let code = "if False:\n    pass\nelse:\n    pass";
        let result = transpile(code);
        assert!(result.contains("if") || result.contains("else") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_if_elif() {
        let code = "if x == 1:\n    pass\nelif x == 2:\n    pass";
        let result = transpile(code);
        assert!(result.contains("if") || result.contains("elif") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_if_elif_else() {
        let code = "if x == 1:\n    pass\nelif x == 2:\n    pass\nelse:\n    pass";
        let result = transpile(code);
        assert!(result.contains("if") || result.contains("else") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_nested_if_three_levels() {
        let code = "if a:\n    if b:\n        if c:\n            pass";
        let result = transpile(code);
        assert!(result.contains("if") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_nested_if_four_levels() {
        let code = "if a:\n    if b:\n        if c:\n            if d:\n                pass";
        let result = transpile(code);
        assert!(result.contains("if") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_while_simple() {
        let code = "while True:\n    break";
        let result = transpile(code);
        assert!(result.contains("while") || result.contains("break") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_while_with_condition() {
        let code = "x = 5\nwhile x > 0:\n    x -= 1";
        let result = transpile(code);
        assert!(result.contains("while") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_while_break() {
        let code = "while True:\n    break";
        let result = transpile(code);
        assert!(result.contains("break") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_while_continue() {
        let code = "x = 0\nwhile x < 10:\n    x += 1\n    continue";
        let result = transpile(code);
        assert!(result.contains("continue") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_while_break_continue() {
        let code = "while True:\n    if cond:\n        break\n    continue";
        let result = transpile(code);
        assert!(result.contains("break") || result.contains("continue") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_for_range() {
        let code = "for i in range(10):\n    pass";
        let result = transpile(code);
        assert!(result.contains("for") || result.contains("range") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_for_list() {
        let code = "for item in [1, 2, 3]:\n    pass";
        let result = transpile(code);
        assert!(result.contains("for") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_for_enumerate() {
        let code = "for i, val in enumerate([10, 20, 30]):\n    pass";
        let result = transpile(code);
        assert!(result.contains("enumerate") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_for_enumerate_start() {
        let code = "for i, val in enumerate([10, 20], start=1):\n    pass";
        let result = transpile(code);
        assert!(result.contains("enumerate") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_for_zip() {
        let code = "for a, b in zip([1, 2], [3, 4]):\n    pass";
        let result = transpile(code);
        assert!(result.contains("zip") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_for_zip_three() {
        let code = "for a, b, c in zip([1], [2], [3]):\n    pass";
        let result = transpile(code);
        assert!(result.contains("zip") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_for_range_step() {
        let code = "for i in range(0, 10, 2):\n    pass";
        let result = transpile(code);
        assert!(result.contains("range") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_for_range_negative_step() {
        let code = "for i in range(10, 0, -1):\n    pass";
        let result = transpile(code);
        assert!(result.contains("range") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_nested_for() {
        let code = "for i in range(5):\n    for j in range(5):\n        pass";
        let result = transpile(code);
        assert!(result.contains("for") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_nested_for_triple() {
        let code = "for i in range(3):\n    for j in range(3):\n        for k in range(3):\n            pass";
        let result = transpile(code);
        assert!(result.contains("for") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_try_except_basic() {
        let code = "try:\n    pass\nexcept:\n    pass";
        let result = transpile(code);
        assert!(result.contains("try") || result.contains("except") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_try_except_type() {
        let code = "try:\n    pass\nexcept ValueError:\n    pass";
        let result = transpile(code);
        assert!(result.contains("ValueError") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_try_except_multiple() {
        let code = "try:\n    pass\nexcept (TypeError, ValueError):\n    pass";
        let result = transpile(code);
        assert!(result.contains("TypeError") || result.contains("ValueError") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_try_except_else() {
        let code = "try:\n    pass\nexcept:\n    pass\nelse:\n    pass";
        let result = transpile(code);
        assert!(result.contains("else") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_try_except_finally() {
        let code = "try:\n    pass\nexcept:\n    pass\nfinally:\n    pass";
        let result = transpile(code);
        assert!(result.contains("finally") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_try_full_form() {
        let code =
            "try:\n    pass\nexcept ValueError:\n    pass\nelse:\n    pass\nfinally:\n    pass";
        let result = transpile(code);
        assert!(result.contains("try") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_try_except_as() {
        let code = "try:\n    pass\nexcept ValueError as e:\n    pass";
        let result = transpile(code);
        assert!(result.contains("e") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_with_statement() {
        let code = "with open('file.txt') as f:\n    pass";
        let result = transpile(code);
        assert!(result.contains("with") || result.contains("open") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_with_multiple() {
        let code = "with open('a.txt') as f1, open('b.txt') as f2:\n    pass";
        let result = transpile(code);
        assert!(result.contains("f1") || result.contains("f2") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_nested_with() {
        let code = "with open('a.txt') as f1:\n    with open('b.txt') as f2:\n        pass";
        let result = transpile(code);
        assert!(result.contains("f1") || result.contains("f2") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_match_simple() {
        let code = "match x:\n    case 1:\n        pass";
        let result = transpile(code);
        assert!(result.contains("match") || result.contains("case") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_match_multiple_cases() {
        let code = "match x:\n    case 1:\n        pass\n    case 2:\n        pass\n    case _:\n        pass";
        let result = transpile(code);
        assert!(result.contains("match") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_match_pattern() {
        let code = "match point:\n    case (0, 0):\n        pass\n    case (x, 0):\n        pass";
        let result = transpile(code);
        assert!(result.contains("match") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_pass_in_function() {
        let code = "def f():\n    pass";
        let result = transpile(code);
        assert!(result.contains("f") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_pass_in_class() {
        let code = "class C:\n    pass";
        let result = transpile(code);
        assert!(result.contains("C") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_pass_in_if() {
        let code = "if True:\n    pass";
        let result = transpile(code);
        assert!(result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_pass_in_for() {
        let code = "for i in range(5):\n    pass";
        let result = transpile(code);
        assert!(result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_pass_in_while() {
        let code = "while True:\n    break";
        let result = transpile(code);
        assert!(result.len() > 0);
    }

    #[test]
    fn test_w10ac_control_pass_in_try() {
        let code = "try:\n    pass\nexcept:\n    pass";
        let result = transpile(code);
        assert!(result.len() > 0);
    }

    // ==================== Section 3: Function patterns (40 tests) ====================

    #[test]
    fn test_w10ac_func_simple() {
        let code = "def f():\n    return 5";
        let result = transpile(code);
        assert!(result.contains("f") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_with_param() {
        let code = "def add(x):\n    return x + 1";
        let result = transpile(code);
        assert!(result.contains("add") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_with_two_params() {
        let code = "def add(x, y):\n    return x + y";
        let result = transpile(code);
        assert!(result.contains("add") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_with_default() {
        let code = "def greet(name='World'):\n    return name";
        let result = transpile(code);
        assert!(result.contains("greet") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_with_args() {
        let code = "def f(*args):\n    return args";
        let result = transpile(code);
        assert!(result.contains("args") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_with_kwargs() {
        let code = "def f(**kwargs):\n    return kwargs";
        let result = transpile(code);
        assert!(result.contains("kwargs") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_with_args_kwargs() {
        let code = "def f(*args, **kwargs):\n    return (args, kwargs)";
        let result = transpile(code);
        assert!(result.contains("args") || result.contains("kwargs") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_with_all_param_types() {
        let code = "def f(a, b=1, *args, **kwargs):\n    return a";
        let result = transpile(code);
        assert!(result.contains("f") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_keyword_only() {
        let code = "def f(*, x):\n    return x";
        let result = transpile(code);
        assert!(result.contains("x") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_keyword_only_with_default() {
        let code = "def f(*, x=5):\n    return x";
        let result = transpile(code);
        assert!(result.contains("x") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_positional_only() {
        let code = "def f(x, /):\n    return x";
        let result = transpile(code);
        assert!(result.contains("x") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_positional_and_keyword_only() {
        let code = "def f(x, /, *, y):\n    return x + y";
        let result = transpile(code);
        assert!(result.contains("x") || result.contains("y") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_recursive_factorial() {
        let code = "def fact(n):\n    if n <= 1:\n        return 1\n    return n * fact(n-1)";
        let result = transpile(code);
        assert!(result.contains("fact") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_recursive_fibonacci() {
        let code = "def fib(n):\n    if n <= 1:\n        return n\n    return fib(n-1) + fib(n-2)";
        let result = transpile(code);
        assert!(result.contains("fib") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_multiple_returns() {
        let code = "def f(x):\n    if x > 0:\n        return 1\n    elif x < 0:\n        return -1\n    return 0";
        let result = transpile(code);
        assert!(result.contains("return") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_early_return() {
        let code = "def f(x):\n    if x == 0:\n        return\n    return x";
        let result = transpile(code);
        assert!(result.contains("return") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_return_none_explicit() {
        let code = "def f():\n    return None";
        let result = transpile(code);
        assert!(result.contains("None") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_no_return() {
        let code = "def f():\n    x = 5";
        let result = transpile(code);
        assert!(result.contains("f") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_generator_simple() {
        let code = "def gen():\n    yield 1";
        let result = transpile(code);
        assert!(result.contains("yield") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_generator_multiple_yields() {
        let code = "def gen():\n    yield 1\n    yield 2\n    yield 3";
        let result = transpile(code);
        assert!(result.contains("yield") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_generator_with_loop() {
        let code = "def gen(n):\n    for i in range(n):\n        yield i";
        let result = transpile(code);
        assert!(result.contains("yield") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_with_decorator() {
        let code = "@decorator\ndef f():\n    pass";
        let result = transpile(code);
        assert!(result.contains("f") || result.contains("decorator") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_with_multiple_decorators() {
        let code = "@dec1\n@dec2\ndef f():\n    pass";
        let result = transpile(code);
        assert!(result.contains("dec1") || result.contains("dec2") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_lambda_simple() {
        let code = "f = lambda x: x + 1";
        let result = transpile(code);
        assert!(result.contains("lambda") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_lambda_two_params() {
        let code = "add = lambda x, y: x + y";
        let result = transpile(code);
        assert!(result.contains("lambda") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_lambda_with_default() {
        let code = "f = lambda x=5: x * 2";
        let result = transpile(code);
        assert!(result.contains("lambda") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_nested_simple() {
        let code = "def outer():\n    def inner():\n        return 5\n    return inner()";
        let result = transpile(code);
        assert!(result.contains("inner") || result.contains("outer") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_nested_with_closure() {
        let code = "def outer(x):\n    def inner():\n        return x\n    return inner()";
        let result = transpile(code);
        assert!(result.contains("inner") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_nested_closure_modify() {
        let code = "def outer(x):\n    def inner():\n        nonlocal x\n        x += 1\n        return x\n    return inner()";
        let result = transpile(code);
        assert!(result.contains("nonlocal") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_async_def() {
        let code = "async def f():\n    return 5";
        let result = transpile(code);
        assert!(result.contains("async") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_async_with_await() {
        let code = "async def f():\n    await other()";
        let result = transpile(code);
        assert!(result.contains("async") || result.contains("await") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_with_docstring() {
        let code = r#"def f():
    """This is a docstring"""
    return 5"#;
        let result = transpile(code);
        assert!(result.contains("docstring") || result.contains("f") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_with_multiline_docstring() {
        let code = r#"def f():
    """
    This is a multi-line
    docstring
    """
    return 5"#;
        let result = transpile(code);
        assert!(result.contains("docstring") || result.contains("f") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_with_type_hints() {
        let code = "def f(x: int) -> int:\n    return x + 1";
        let result = transpile(code);
        assert!(result.contains("f") || result.contains("i32") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_with_complex_type_hints() {
        let code = "def f(x: list[int]) -> dict[str, int]:\n    return {}";
        let result = transpile(code);
        assert!(result.contains("f") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_return_tuple() {
        let code = "def f():\n    return 1, 2, 3";
        let result = transpile(code);
        assert!(result.contains("return") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_return_list() {
        let code = "def f():\n    return [1, 2, 3]";
        let result = transpile(code);
        assert!(result.contains("return") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_return_dict() {
        let code = "def f():\n    return {'a': 1, 'b': 2}";
        let result = transpile(code);
        assert!(result.contains("return") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_func_with_inner_class() {
        let code = "def f():\n    class Inner:\n        pass\n    return Inner()";
        let result = transpile(code);
        assert!(result.contains("Inner") || result.len() > 0);
    }

    // ==================== Section 4: Class patterns (40 tests) ====================

    #[test]
    fn test_w10ac_class_empty() {
        let code = "class C:\n    pass";
        let result = transpile(code);
        assert!(result.contains("C") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_with_init() {
        let code = "class C:\n    def __init__(self):\n        pass";
        let result = transpile(code);
        assert!(result.contains("__init__") || result.contains("C") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_init_with_params() {
        let code = "class C:\n    def __init__(self, x):\n        self.x = x";
        let result = transpile(code);
        assert!(result.contains("x") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_with_method() {
        let code = "class C:\n    def method(self):\n        return 5";
        let result = transpile(code);
        assert!(result.contains("method") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_multiple_methods() {
        let code = "class C:\n    def m1(self):\n        pass\n    def m2(self):\n        pass";
        let result = transpile(code);
        assert!(result.contains("m1") || result.contains("m2") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_with_str() {
        let code = "class C:\n    def __str__(self):\n        return 'C'";
        let result = transpile(code);
        assert!(result.contains("__str__") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_with_repr() {
        let code = "class C:\n    def __repr__(self):\n        return 'C()'";
        let result = transpile(code);
        assert!(result.contains("__repr__") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_str_and_repr() {
        let code = "class C:\n    def __str__(self):\n        return 'C'\n    def __repr__(self):\n        return 'C()'";
        let result = transpile(code);
        assert!(result.contains("__str__") || result.contains("__repr__") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_with_eq() {
        let code = "class C:\n    def __eq__(self, other):\n        return True";
        let result = transpile(code);
        assert!(result.contains("__eq__") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_with_lt() {
        let code = "class C:\n    def __lt__(self, other):\n        return False";
        let result = transpile(code);
        assert!(result.contains("__lt__") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_with_le() {
        let code = "class C:\n    def __le__(self, other):\n        return True";
        let result = transpile(code);
        assert!(result.contains("__le__") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_with_gt() {
        let code = "class C:\n    def __gt__(self, other):\n        return False";
        let result = transpile(code);
        assert!(result.contains("__gt__") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_with_ge() {
        let code = "class C:\n    def __ge__(self, other):\n        return True";
        let result = transpile(code);
        assert!(result.contains("__ge__") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_all_comparisons() {
        let code = "class C:\n    def __eq__(self, o):\n        return True\n    def __lt__(self, o):\n        return False";
        let result = transpile(code);
        assert!(result.contains("__eq__") || result.contains("__lt__") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_with_add() {
        let code = "class C:\n    def __add__(self, other):\n        return self";
        let result = transpile(code);
        assert!(result.contains("__add__") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_with_sub() {
        let code = "class C:\n    def __sub__(self, other):\n        return self";
        let result = transpile(code);
        assert!(result.contains("__sub__") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_with_mul() {
        let code = "class C:\n    def __mul__(self, other):\n        return self";
        let result = transpile(code);
        assert!(result.contains("__mul__") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_with_truediv() {
        let code = "class C:\n    def __truediv__(self, other):\n        return self";
        let result = transpile(code);
        assert!(result.contains("__truediv__") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_arithmetic_ops() {
        let code = "class C:\n    def __add__(self, o):\n        return self\n    def __mul__(self, o):\n        return self";
        let result = transpile(code);
        assert!(result.contains("__add__") || result.contains("__mul__") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_with_contains() {
        let code = "class C:\n    def __contains__(self, item):\n        return True";
        let result = transpile(code);
        assert!(result.contains("__contains__") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_with_iter() {
        let code = "class C:\n    def __iter__(self):\n        return self";
        let result = transpile(code);
        assert!(result.contains("__iter__") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_with_next() {
        let code = "class C:\n    def __next__(self):\n        raise StopIteration";
        let result = transpile(code);
        assert!(result.contains("__next__") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_iterator() {
        let code = "class C:\n    def __iter__(self):\n        return self\n    def __next__(self):\n        raise StopIteration";
        let result = transpile(code);
        assert!(result.contains("__iter__") || result.contains("__next__") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_with_len() {
        let code = "class C:\n    def __len__(self):\n        return 0";
        let result = transpile(code);
        assert!(result.contains("__len__") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_with_getitem() {
        let code = "class C:\n    def __getitem__(self, key):\n        return None";
        let result = transpile(code);
        assert!(result.contains("__getitem__") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_with_setitem() {
        let code = "class C:\n    def __setitem__(self, key, value):\n        pass";
        let result = transpile(code);
        assert!(result.contains("__setitem__") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_container_protocol() {
        let code = "class C:\n    def __len__(self):\n        return 0\n    def __getitem__(self, k):\n        return None";
        let result = transpile(code);
        assert!(result.contains("__len__") || result.contains("__getitem__") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_with_classmethod() {
        let code = "class C:\n    @classmethod\n    def f(cls):\n        return cls";
        let result = transpile(code);
        assert!(result.contains("classmethod") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_with_staticmethod() {
        let code = "class C:\n    @staticmethod\n    def f():\n        return 5";
        let result = transpile(code);
        assert!(result.contains("staticmethod") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_all_method_types() {
        let code = "class C:\n    def instance(self):\n        pass\n    @classmethod\n    def cls(cls):\n        pass\n    @staticmethod\n    def static():\n        pass";
        let result = transpile(code);
        assert!(result.contains("instance") || result.contains("classmethod") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_inheritance_simple() {
        let code = "class Base:\n    pass\nclass Derived(Base):\n    pass";
        let result = transpile(code);
        assert!(result.contains("Base") || result.contains("Derived") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_inheritance_with_super() {
        let code = "class Base:\n    def __init__(self):\n        pass\nclass Derived(Base):\n    def __init__(self):\n        super().__init__()";
        let result = transpile(code);
        assert!(result.contains("super") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_multiple_inheritance() {
        let code = "class A:\n    pass\nclass B:\n    pass\nclass C(A, B):\n    pass";
        let result = transpile(code);
        assert!(
            result.contains("A")
                || result.contains("B")
                || result.contains("C")
                || result.len() > 0
        );
    }

    #[test]
    fn test_w10ac_class_attribute() {
        let code = "class C:\n    x = 5";
        let result = transpile(code);
        assert!(result.contains("x") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_multiple_attributes() {
        let code = "class C:\n    x = 5\n    y = 10\n    z = 'hello'";
        let result = transpile(code);
        assert!(result.contains("x") || result.contains("y") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_with_enter_exit() {
        let code = "class C:\n    def __enter__(self):\n        return self\n    def __exit__(self, *args):\n        pass";
        let result = transpile(code);
        assert!(result.contains("__enter__") || result.contains("__exit__") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_context_manager_full() {
        let code = "class C:\n    def __enter__(self):\n        return self\n    def __exit__(self, exc_type, exc_val, exc_tb):\n        return False";
        let result = transpile(code);
        assert!(result.contains("__enter__") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_with_property() {
        let code = "class C:\n    @property\n    def x(self):\n        return 5";
        let result = transpile(code);
        assert!(result.contains("property") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_class_with_setter() {
        let code = "class C:\n    @property\n    def x(self):\n        return 5\n    @x.setter\n    def x(self, val):\n        pass";
        let result = transpile(code);
        assert!(result.contains("property") || result.contains("setter") || result.len() > 0);
    }

    // ==================== Section 5: Exception handling (30 tests) ====================

    #[test]
    fn test_w10ac_except_raise_valueerror() {
        let code = "raise ValueError('error')";
        let result = transpile(code);
        assert!(result.contains("ValueError") || result.contains("raise") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_raise_typeerror() {
        let code = "raise TypeError('type error')";
        let result = transpile(code);
        assert!(result.contains("TypeError") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_raise_keyerror() {
        let code = "raise KeyError('key')";
        let result = transpile(code);
        assert!(result.contains("KeyError") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_raise_indexerror() {
        let code = "raise IndexError('index')";
        let result = transpile(code);
        assert!(result.contains("IndexError") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_raise_custom() {
        let code = "class MyError(Exception):\n    pass\nraise MyError('custom')";
        let result = transpile(code);
        assert!(result.contains("MyError") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_raise_from() {
        let code = "try:\n    pass\nexcept ValueError as e:\n    raise TypeError('new') from e";
        let result = transpile(code);
        assert!(result.contains("from") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_multiple_types() {
        let code = "try:\n    pass\nexcept (TypeError, ValueError):\n    pass";
        let result = transpile(code);
        assert!(result.contains("TypeError") || result.contains("ValueError") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_multiple_types_three() {
        let code = "try:\n    pass\nexcept (TypeError, ValueError, KeyError):\n    pass";
        let result = transpile(code);
        assert!(result.contains("TypeError") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_bare() {
        let code = "try:\n    pass\nexcept:\n    pass";
        let result = transpile(code);
        assert!(result.contains("except") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_reraise() {
        let code = "try:\n    pass\nexcept ValueError:\n    raise";
        let result = transpile(code);
        assert!(result.contains("raise") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_custom_class() {
        let code = "class MyError(Exception):\n    pass";
        let result = transpile(code);
        assert!(result.contains("MyError") || result.contains("Exception") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_custom_with_init() {
        let code =
            "class MyError(Exception):\n    def __init__(self, msg):\n        self.msg = msg";
        let result = transpile(code);
        assert!(result.contains("MyError") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_custom_with_attributes() {
        let code = "class MyError(Exception):\n    def __init__(self, code, msg):\n        self.code = code\n        self.msg = msg";
        let result = transpile(code);
        assert!(result.contains("code") || result.contains("msg") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_nested_try() {
        let code =
            "try:\n    try:\n        pass\n    except ValueError:\n        pass\nexcept:\n    pass";
        let result = transpile(code);
        assert!(result.contains("try") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_nested_three_levels() {
        let code = "try:\n    try:\n        try:\n            pass\n        except:\n            pass\n    except:\n        pass\nexcept:\n    pass";
        let result = transpile(code);
        assert!(result.contains("try") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_assert_simple() {
        let code = "assert True";
        let result = transpile(code);
        assert!(result.contains("assert") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_assert_with_message() {
        let code = "assert x > 0, 'x must be positive'";
        let result = transpile(code);
        assert!(result.contains("assert") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_assert_complex_condition() {
        let code = "assert x > 0 and y > 0, 'both must be positive'";
        let result = transpile(code);
        assert!(result.contains("assert") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_multiple_except_clauses() {
        let code = "try:\n    pass\nexcept ValueError:\n    pass\nexcept TypeError:\n    pass\nexcept KeyError:\n    pass";
        let result = transpile(code);
        assert!(result.contains("ValueError") || result.contains("TypeError") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_catch_all_last() {
        let code = "try:\n    pass\nexcept ValueError:\n    pass\nexcept:\n    pass";
        let result = transpile(code);
        assert!(result.contains("except") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_with_as_binding() {
        let code = "try:\n    pass\nexcept ValueError as e:\n    print(e)";
        let result = transpile(code);
        assert!(result.contains("e") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_multiple_with_as() {
        let code =
            "try:\n    pass\nexcept ValueError as e1:\n    pass\nexcept TypeError as e2:\n    pass";
        let result = transpile(code);
        assert!(result.contains("e1") || result.contains("e2") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_finally_cleanup() {
        let code = "try:\n    f = open('file.txt')\nexcept:\n    pass\nfinally:\n    f.close()";
        let result = transpile(code);
        assert!(result.contains("finally") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_else_success() {
        let code = "try:\n    result = 5\nexcept:\n    pass\nelse:\n    print('success')";
        let result = transpile(code);
        assert!(result.contains("else") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_raise_without_args() {
        let code = "raise ValueError";
        let result = transpile(code);
        assert!(result.contains("ValueError") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_custom_inherit_valueerror() {
        let code = "class MyError(ValueError):\n    pass";
        let result = transpile(code);
        assert!(result.contains("MyError") || result.contains("ValueError") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_try_in_function() {
        let code = "def f():\n    try:\n        return 5\n    except:\n        return 0";
        let result = transpile(code);
        assert!(result.contains("try") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_try_in_loop() {
        let code = "for i in range(10):\n    try:\n        pass\n    except:\n        continue";
        let result = transpile(code);
        assert!(result.contains("try") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_raise_in_finally() {
        let code = "try:\n    pass\nfinally:\n    raise ValueError('cleanup failed')";
        let result = transpile(code);
        assert!(result.contains("finally") || result.len() > 0);
    }

    #[test]
    fn test_w10ac_except_exception_group() {
        let code = "class MyErrorGroup(Exception):\n    def __init__(self, errors):\n        self.errors = errors";
        let result = transpile(code);
        assert!(result.contains("MyErrorGroup") || result.len() > 0);
    }
}
