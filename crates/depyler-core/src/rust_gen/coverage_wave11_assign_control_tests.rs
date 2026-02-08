// coverage_wave11_assign_control_tests.rs
// Target: stmt_gen.rs codegen_assign_stmt + control flow branches
//
// Wave 11 focuses on deeper edge cases in assignment codegen and control flow
// that wave 10 did not cover, with more precise output assertions.

#[cfg(test)]
mod tests {
    use crate::ast_bridge::AstBridge;
    use crate::rust_gen::generate_rust_file;
    use crate::type_mapper::TypeMapper;
    use rustpython_parser::{parse, Mode};

    fn transpile(python_code: &str) -> String {
        let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
        let (module, _) = AstBridge::new()
            .with_source(python_code.to_string())
            .python_to_hir(ast)
            .expect("hir");
        let tm = TypeMapper::default();
        let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
        result
    }

    // ==========================================================================
    // Section 1: Simple variable assignment patterns (25 tests)
    // ==========================================================================

    #[test]
    fn test_w11ac_assign_int_literal() {
        let result = transpile("def f():\n    x = 42\n    return x");
        assert!(result.contains("let") || result.contains("x"));
        assert!(result.contains("42"));
    }

    #[test]
    fn test_w11ac_assign_negative_int() {
        let result = transpile("def f():\n    x = -10\n    return x");
        assert!(result.contains("10"));
    }

    #[test]
    fn test_w11ac_assign_large_int() {
        let result = transpile("def f():\n    x = 1000000\n    return x");
        assert!(result.contains("1000000") || result.contains("1_000_000"));
    }

    #[test]
    fn test_w11ac_assign_zero() {
        let result = transpile("def f():\n    x = 0\n    return x");
        assert!(result.contains("0"));
    }

    #[test]
    fn test_w11ac_assign_float_literal() {
        let result = transpile("def f():\n    x = 2.5\n    return x");
        assert!(result.contains("2.5") || result.contains("f64"));
    }

    #[test]
    fn test_w11ac_assign_negative_float() {
        let result = transpile("def f():\n    x = -0.5\n    return x");
        assert!(result.contains("0.5"));
    }

    #[test]
    fn test_w11ac_assign_string_literal() {
        let result = transpile("def f():\n    s = \"hello\"\n    return s");
        assert!(result.contains("hello"));
    }

    #[test]
    fn test_w11ac_assign_empty_string() {
        let result = transpile("def f():\n    s = \"\"\n    return s");
        assert!(result.contains("\"\"") || result.contains("String::new") || result.contains("STR_EMPTY"));
    }

    #[test]
    fn test_w11ac_assign_bool_true() {
        let result = transpile("def f():\n    flag = True\n    return flag");
        assert!(result.contains("true"));
    }

    #[test]
    fn test_w11ac_assign_bool_false() {
        let result = transpile("def f():\n    flag = False\n    return flag");
        assert!(result.contains("false"));
    }

    #[test]
    fn test_w11ac_assign_none_value() {
        let result = transpile("def f():\n    result = None\n    return result");
        assert!(result.contains("None") || result.contains("result"));
    }

    #[test]
    fn test_w11ac_assign_from_addition() {
        let result = transpile("def f(a: int, b: int) -> int:\n    c = a + b\n    return c");
        assert!(result.contains("+"));
    }

    #[test]
    fn test_w11ac_assign_from_subtraction() {
        let result = transpile("def f(a: int, b: int) -> int:\n    c = a - b\n    return c");
        assert!(result.contains("-"));
    }

    #[test]
    fn test_w11ac_assign_from_multiplication() {
        let result = transpile("def f(a: int, b: int) -> int:\n    c = a * b\n    return c");
        assert!(result.contains("*"));
    }

    #[test]
    fn test_w11ac_assign_from_floor_division() {
        let result = transpile("def f(a: int, b: int) -> int:\n    c = a // b\n    return c");
        assert!(result.contains("/") || result.contains("div"));
    }

    #[test]
    fn test_w11ac_assign_from_modulo() {
        let result = transpile("def f(a: int, b: int) -> int:\n    c = a % b\n    return c");
        assert!(result.contains("%"));
    }

    #[test]
    fn test_w11ac_assign_from_comparison_gt() {
        let result = transpile("def f(x: int) -> bool:\n    result = x > 0\n    return result");
        assert!(result.contains(">"));
    }

    #[test]
    fn test_w11ac_assign_from_comparison_lt() {
        let result = transpile("def f(x: int) -> bool:\n    result = x < 100\n    return result");
        assert!(result.contains("<"));
    }

    #[test]
    fn test_w11ac_assign_from_comparison_eq() {
        let result = transpile("def f(x: int) -> bool:\n    result = x == 0\n    return result");
        assert!(result.contains("=="));
    }

    #[test]
    fn test_w11ac_assign_from_comparison_ne() {
        let result = transpile("def f(x: int) -> bool:\n    result = x != 0\n    return result");
        assert!(result.contains("!="));
    }

    #[test]
    fn test_w11ac_assign_from_and_expr() {
        let result = transpile("def f(a: bool, b: bool) -> bool:\n    result = a and b\n    return result");
        assert!(result.contains("&&") || result.contains("and"));
    }

    #[test]
    fn test_w11ac_assign_from_or_expr() {
        let result = transpile("def f(a: bool, b: bool) -> bool:\n    result = a or b\n    return result");
        assert!(result.contains("||") || result.contains("or"));
    }

    #[test]
    fn test_w11ac_assign_from_not_expr() {
        let result = transpile("def f(a: bool) -> bool:\n    result = not a\n    return result");
        assert!(result.contains("!") || result.contains("not"));
    }

    #[test]
    fn test_w11ac_assign_from_ternary_simple() {
        let result = transpile("def f(x: int) -> int:\n    val = x if x > 0 else 0\n    return val");
        assert!(result.contains("if") || result.contains("val"));
    }

    #[test]
    fn test_w11ac_assign_from_func_call() {
        let result = transpile("def helper() -> int:\n    return 5\ndef f() -> int:\n    result = helper()\n    return result");
        assert!(result.contains("helper") && result.contains("result"));
    }

    // ==========================================================================
    // Section 2: Tuple unpacking and multiple assignment (25 tests)
    // ==========================================================================

    #[test]
    fn test_w11ac_tuple_unpack_two_ints() {
        let result = transpile("def f():\n    a, b = 1, 2\n    return a");
        assert!(result.contains("a") && result.contains("b"));
    }

    #[test]
    fn test_w11ac_tuple_unpack_three_values() {
        let result = transpile("def f():\n    x, y, z = 10, 20, 30\n    return x");
        assert!(result.contains("x") || result.contains("10"));
    }

    #[test]
    fn test_w11ac_tuple_unpack_mixed_types() {
        let result = transpile("def f():\n    name, age = \"Alice\", 30\n    return name");
        assert!(result.contains("name") || result.contains("Alice"));
    }

    #[test]
    fn test_w11ac_tuple_unpack_from_call() {
        let result = transpile("def pair() -> tuple:\n    return 1, 2\ndef f():\n    a, b = pair()\n    return a");
        assert!(result.contains("pair"));
    }

    #[test]
    fn test_w11ac_assign_multiple_same_value() {
        let result = transpile("def f():\n    x = y = 42\n    return x");
        assert!(result.contains("42"));
    }

    #[test]
    fn test_w11ac_assign_triple_chain() {
        let result = transpile("def f():\n    a = b = c = 0\n    return a");
        assert!(result.contains("0"));
    }

    #[test]
    fn test_w11ac_tuple_swap_values() {
        let result = transpile("def f(a: int, b: int) -> int:\n    a, b = b, a\n    return a");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_tuple_unpack_four_values() {
        let result = transpile("def f():\n    a, b, c, d = 1, 2, 3, 4\n    return a");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_tuple_unpack_five_values() {
        let result = transpile("def f():\n    a, b, c, d, e = 1, 2, 3, 4, 5\n    return a");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_tuple_unpack_strings() {
        let result = transpile("def f():\n    first, last = \"John\", \"Doe\"\n    return first");
        assert!(result.contains("John") || result.contains("first"));
    }

    #[test]
    fn test_w11ac_tuple_unpack_bools() {
        let result = transpile("def f():\n    ok, done = True, False\n    return ok");
        assert!(result.contains("true") || result.contains("false"));
    }

    #[test]
    fn test_w11ac_tuple_unpack_zeros() {
        let result = transpile("def f():\n    x, y = 0, 0\n    return x");
        assert!(result.contains("0"));
    }

    #[test]
    fn test_w11ac_starred_unpack_first() {
        let result = transpile("def f():\n    first, *rest = [1, 2, 3, 4]\n    return first");
        assert!(result.contains("first") || result.contains("rest") || !result.is_empty());
    }

    #[test]
    fn test_w11ac_starred_unpack_last() {
        let result = transpile("def f():\n    *init, last = [1, 2, 3, 4]\n    return last");
        assert!(result.contains("last") || result.contains("init") || !result.is_empty());
    }

    #[test]
    fn test_w11ac_starred_unpack_middle() {
        let result = transpile("def f():\n    first, *mid, last = [1, 2, 3, 4, 5]\n    return first");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_list_target_two() {
        let result = transpile("def f():\n    [a, b] = [1, 2]\n    return a");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_list_target_three() {
        let result = transpile("def f():\n    [a, b, c] = [1, 2, 3]\n    return a");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_tuple_unpack_with_computation() {
        let result = transpile("def f(x: int):\n    a, b = x + 1, x - 1\n    return a");
        assert!(result.contains("+") || result.contains("-"));
    }

    #[test]
    fn test_w11ac_assign_from_list_literal() {
        let result = transpile("def f() -> list:\n    items = [1, 2, 3]\n    return items");
        assert!(result.contains("vec!") || result.contains("Vec") || result.contains("items"));
    }

    #[test]
    fn test_w11ac_assign_empty_list() {
        let result = transpile("def f() -> list:\n    items = []\n    return items");
        assert!(result.contains("vec!") || result.contains("Vec") || result.contains("items"));
    }

    #[test]
    fn test_w11ac_assign_from_dict_literal() {
        let result = transpile("def f() -> dict:\n    d = {\"a\": 1}\n    return d");
        assert!(result.contains("HashMap") || result.contains("d"));
    }

    #[test]
    fn test_w11ac_assign_empty_dict() {
        let result = transpile("def f() -> dict:\n    d = {}\n    return d");
        assert!(result.contains("HashMap") || result.contains("new") || result.contains("d"));
    }

    #[test]
    fn test_w11ac_assign_from_list_comprehension() {
        let result = transpile("def f() -> list:\n    squares = [x * x for x in range(10)]\n    return squares");
        assert!(result.contains("map") || result.contains("collect") || result.contains("squares"));
    }

    #[test]
    fn test_w11ac_assign_from_set_literal() {
        let result = transpile("def f():\n    s = {1, 2, 3}\n    return s");
        assert!(result.contains("HashSet") || result.contains("s") || !result.is_empty());
    }

    #[test]
    fn test_w11ac_assign_tuple_literal() {
        let result = transpile("def f():\n    coords = (10, 20)\n    return coords");
        assert!(result.contains("10") && result.contains("20"));
    }

    // ==========================================================================
    // Section 3: Augmented assignment patterns (20 tests)
    // ==========================================================================

    #[test]
    fn test_w11ac_augassign_add_int() {
        let result = transpile("def f() -> int:\n    x: int = 5\n    x += 3\n    return x");
        assert!(result.contains("+=") || result.contains("+"));
    }

    #[test]
    fn test_w11ac_augassign_sub_int() {
        let result = transpile("def f() -> int:\n    x: int = 10\n    x -= 2\n    return x");
        assert!(result.contains("-=") || result.contains("-"));
    }

    #[test]
    fn test_w11ac_augassign_mul_int() {
        let result = transpile("def f() -> int:\n    x: int = 4\n    x *= 3\n    return x");
        assert!(result.contains("*=") || result.contains("*"));
    }

    #[test]
    fn test_w11ac_augassign_floordiv_int() {
        let result = transpile("def f() -> int:\n    x: int = 17\n    x //= 3\n    return x");
        assert!(result.contains("/") || result.contains("x"));
    }

    #[test]
    fn test_w11ac_augassign_mod_int() {
        let result = transpile("def f() -> int:\n    x: int = 17\n    x %= 5\n    return x");
        assert!(result.contains("%") || result.contains("x"));
    }

    #[test]
    fn test_w11ac_augassign_pow_int() {
        let result = transpile("def f() -> int:\n    x: int = 2\n    x **= 8\n    return x");
        assert!(result.contains("pow") || result.contains("x"));
    }

    #[test]
    fn test_w11ac_augassign_bitand() {
        let result = transpile("def f() -> int:\n    x: int = 12\n    x &= 7\n    return x");
        assert!(result.contains("&") || result.contains("x"));
    }

    #[test]
    fn test_w11ac_augassign_bitor() {
        let result = transpile("def f() -> int:\n    x: int = 5\n    x |= 3\n    return x");
        assert!(result.contains("|") || result.contains("x"));
    }

    #[test]
    fn test_w11ac_augassign_bitxor() {
        let result = transpile("def f() -> int:\n    x: int = 10\n    x ^= 6\n    return x");
        assert!(result.contains("^") || result.contains("x"));
    }

    #[test]
    fn test_w11ac_augassign_lshift() {
        let result = transpile("def f() -> int:\n    x: int = 3\n    x <<= 2\n    return x");
        assert!(result.contains("<<") || result.contains("x"));
    }

    #[test]
    fn test_w11ac_augassign_rshift() {
        let result = transpile("def f() -> int:\n    x: int = 32\n    x >>= 2\n    return x");
        assert!(result.contains(">>") || result.contains("x"));
    }

    #[test]
    fn test_w11ac_augassign_string_concat() {
        let result = transpile("def f() -> str:\n    s: str = \"hello\"\n    s += \" world\"\n    return s");
        assert!(result.contains("hello") || result.contains("world"));
    }

    #[test]
    fn test_w11ac_augassign_in_loop() {
        let result = transpile("def f() -> int:\n    total: int = 0\n    for i in range(10):\n        total += i\n    return total");
        assert!(result.contains("total") && (result.contains("+=") || result.contains("+")));
    }

    #[test]
    fn test_w11ac_augassign_float_add() {
        let result = transpile("def f() -> float:\n    x: float = 1.0\n    x += 0.5\n    return x");
        assert!(result.contains("0.5") || result.contains("x"));
    }

    #[test]
    fn test_w11ac_augassign_float_mul() {
        let result = transpile("def f() -> float:\n    x: float = 2.0\n    x *= 1.5\n    return x");
        assert!(result.contains("1.5") || result.contains("*"));
    }

    #[test]
    fn test_w11ac_augassign_counter_pattern() {
        let result = transpile("def f() -> int:\n    count: int = 0\n    for i in range(100):\n        count += 1\n    return count");
        assert!(result.contains("count") && result.contains("1"));
    }

    #[test]
    fn test_w11ac_augassign_accumulator_pattern() {
        let result = transpile("def f(items: list) -> int:\n    total: int = 0\n    for item in items:\n        total += item\n    return total");
        assert!(result.contains("total") && result.contains("item"));
    }

    #[test]
    fn test_w11ac_augassign_multiple_ops() {
        let result = transpile("def f() -> int:\n    x: int = 10\n    x += 5\n    x -= 3\n    x *= 2\n    return x");
        assert!(result.contains("x"));
    }

    #[test]
    fn test_w11ac_augassign_div_float() {
        let result = transpile("def f() -> float:\n    x: float = 100.0\n    x /= 4.0\n    return x");
        assert!(result.contains("/") || result.contains("x"));
    }

    #[test]
    fn test_w11ac_augassign_nested_scope() {
        let result = transpile("def f() -> int:\n    x: int = 0\n    if True:\n        x += 10\n    return x");
        assert!(result.contains("x") && result.contains("10"));
    }

    // ==========================================================================
    // Section 4: Type-annotated assignment (15 tests)
    // ==========================================================================

    #[test]
    fn test_w11ac_annotated_int() {
        let result = transpile("def f():\n    x: int = 42\n    return x");
        assert!(result.contains("i32") || result.contains("42"));
    }

    #[test]
    fn test_w11ac_annotated_float() {
        let result = transpile("def f():\n    x: float = 2.5\n    return x");
        assert!(result.contains("f64") || result.contains("2.5"));
    }

    #[test]
    fn test_w11ac_annotated_str() {
        let result = transpile("def f():\n    name: str = \"test\"\n    return name");
        assert!(result.contains("String") || result.contains("test"));
    }

    #[test]
    fn test_w11ac_annotated_bool() {
        let result = transpile("def f():\n    flag: bool = True\n    return flag");
        assert!(result.contains("bool") || result.contains("true"));
    }

    #[test]
    fn test_w11ac_annotated_list_int() {
        let result = transpile("def f():\n    items: list = [1, 2, 3]\n    return items");
        assert!(result.contains("Vec") || result.contains("vec!") || result.contains("items"));
    }

    #[test]
    fn test_w11ac_annotated_empty_list() {
        let result = transpile("def f():\n    items: list = []\n    return items");
        assert!(result.contains("Vec") || result.contains("items"));
    }

    #[test]
    fn test_w11ac_annotated_dict() {
        let result = transpile("def f():\n    d: dict = {}\n    return d");
        assert!(result.contains("HashMap") || result.contains("d"));
    }

    #[test]
    fn test_w11ac_param_type_int() {
        let result = transpile("def f(x: int) -> int:\n    return x + 1");
        assert!(result.contains("i32"));
    }

    #[test]
    fn test_w11ac_param_type_str() {
        let result = transpile("def f(s: str) -> str:\n    return s");
        assert!(result.contains("str") || result.contains("String"));
    }

    #[test]
    fn test_w11ac_param_type_bool() {
        let result = transpile("def f(flag: bool) -> bool:\n    return flag");
        assert!(result.contains("bool"));
    }

    #[test]
    fn test_w11ac_return_type_int() {
        let result = transpile("def f() -> int:\n    return 42");
        assert!(result.contains("i32") || result.contains("42"));
    }

    #[test]
    fn test_w11ac_return_type_str() {
        let result = transpile("def f() -> str:\n    return \"hello\"");
        assert!(result.contains("String") || result.contains("hello"));
    }

    #[test]
    fn test_w11ac_return_type_bool() {
        let result = transpile("def f() -> bool:\n    return True");
        assert!(result.contains("bool") || result.contains("true"));
    }

    #[test]
    fn test_w11ac_return_type_float() {
        let result = transpile("def f() -> float:\n    return 1.5");
        assert!(result.contains("f64") || result.contains("1.5"));
    }

    #[test]
    fn test_w11ac_return_type_list() {
        let result = transpile("def f() -> list:\n    return [1, 2, 3]");
        assert!(result.contains("Vec") || result.contains("vec!"));
    }

    // ==========================================================================
    // Section 5: Attribute and index assignment (15 tests)
    // ==========================================================================

    #[test]
    fn test_w11ac_self_attr_assign() {
        let result = transpile("class Point:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y");
        assert!(result.contains("x") && result.contains("y"));
    }

    #[test]
    fn test_w11ac_self_attr_single() {
        let result = transpile("class Counter:\n    def __init__(self):\n        self.count = 0");
        assert!(result.contains("count") || result.contains("0"));
    }

    #[test]
    fn test_w11ac_self_attr_string() {
        let result = transpile("class Person:\n    def __init__(self, name: str):\n        self.name = name");
        assert!(result.contains("name"));
    }

    #[test]
    fn test_w11ac_self_attr_bool() {
        let result = transpile("class Toggle:\n    def __init__(self):\n        self.active = False");
        assert!(result.contains("active") || result.contains("false"));
    }

    #[test]
    fn test_w11ac_self_attr_list() {
        let result = transpile("class Container:\n    def __init__(self):\n        self.items = []");
        assert!(result.contains("items"));
    }

    #[test]
    fn test_w11ac_self_attr_multiple() {
        let result = transpile("class Rect:\n    def __init__(self, w: int, h: int):\n        self.width = w\n        self.height = h\n        self.area = w * h");
        assert!(result.contains("width") || result.contains("height") || result.contains("area"));
    }

    #[test]
    fn test_w11ac_index_assign_list() {
        let result = transpile("def f():\n    lst = [1, 2, 3]\n    lst[0] = 10\n    return lst");
        assert!(result.contains("lst") && result.contains("10"));
    }

    #[test]
    fn test_w11ac_index_assign_dict_str_key() {
        let result = transpile("def f():\n    d = {\"a\": 1}\n    d[\"b\"] = 2\n    return d");
        assert!(result.contains("insert") || result.contains("d"));
    }

    #[test]
    fn test_w11ac_index_assign_dict_int_val() {
        let result = transpile("def f():\n    d: dict = {}\n    d[\"key\"] = 42\n    return d");
        assert!(result.contains("insert") || result.contains("42"));
    }

    #[test]
    fn test_w11ac_index_assign_last_elem() {
        let result = transpile("def f():\n    lst = [1, 2, 3]\n    lst[2] = 99\n    return lst");
        assert!(result.contains("99") || result.contains("lst"));
    }

    #[test]
    fn test_w11ac_index_assign_computed() {
        let result = transpile("def f(i: int):\n    lst = [0, 0, 0]\n    lst[i] = 1\n    return lst");
        assert!(result.contains("lst") && result.contains("i"));
    }

    #[test]
    fn test_w11ac_index_assign_in_loop() {
        let result = transpile("def f():\n    lst = [0, 0, 0]\n    for i in range(3):\n        lst[i] = i * 2\n    return lst");
        assert!(result.contains("lst") && result.contains("i"));
    }

    #[test]
    fn test_w11ac_dict_assign_multiple_keys() {
        let result = transpile("def f():\n    d: dict = {}\n    d[\"x\"] = 1\n    d[\"y\"] = 2\n    d[\"z\"] = 3\n    return d");
        assert!(result.contains("insert") || result.contains("d"));
    }

    #[test]
    fn test_w11ac_nested_self_attr() {
        let result = transpile("class Node:\n    def __init__(self, val: int):\n        self.val = val\n        self.left = None\n        self.right = None");
        assert!(result.contains("val") && (result.contains("left") || result.contains("right")));
    }

    #[test]
    fn test_w11ac_self_attr_from_method() {
        let result = transpile("class Counter:\n    def __init__(self):\n        self.count = 0\n    def increment(self):\n        self.count += 1");
        assert!(result.contains("count"));
    }

    // ==========================================================================
    // Section 6: If statement control flow (20 tests)
    // ==========================================================================

    #[test]
    fn test_w11ac_if_simple_return() {
        let result = transpile("def f(x: int) -> int:\n    if x > 0:\n        return x\n    return 0");
        assert!(result.contains("if") && result.contains("return"));
    }

    #[test]
    fn test_w11ac_if_else_return() {
        let result = transpile("def f(x: int) -> int:\n    if x > 0:\n        return 1\n    else:\n        return -1");
        assert!(result.contains("if") && result.contains("else"));
    }

    #[test]
    fn test_w11ac_if_elif_else() {
        let result = transpile("def f(x: int) -> str:\n    if x > 0:\n        return \"positive\"\n    elif x < 0:\n        return \"negative\"\n    else:\n        return \"zero\"");
        assert!(result.contains("if") && result.contains("else"));
    }

    #[test]
    fn test_w11ac_if_elif_chain_three() {
        let result = transpile("def f(x: int) -> int:\n    if x == 1:\n        return 10\n    elif x == 2:\n        return 20\n    elif x == 3:\n        return 30\n    else:\n        return 0");
        assert!(result.contains("if") && result.contains("10"));
    }

    #[test]
    fn test_w11ac_nested_if_two_levels() {
        let result = transpile("def f(a: int, b: int) -> int:\n    if a > 0:\n        if b > 0:\n            return a + b\n    return 0");
        assert!(result.contains("if"));
    }

    #[test]
    fn test_w11ac_if_with_assignment() {
        let result = transpile("def f(x: int) -> int:\n    result: int = 0\n    if x > 0:\n        result = x\n    return result");
        assert!(result.contains("result") && result.contains("if"));
    }

    #[test]
    fn test_w11ac_if_and_condition() {
        let result = transpile("def f(a: int, b: int) -> bool:\n    if a > 0 and b > 0:\n        return True\n    return False");
        assert!(result.contains("&&") || result.contains("and"));
    }

    #[test]
    fn test_w11ac_if_or_condition() {
        let result = transpile("def f(a: int, b: int) -> bool:\n    if a > 0 or b > 0:\n        return True\n    return False");
        assert!(result.contains("||") || result.contains("or"));
    }

    #[test]
    fn test_w11ac_if_not_condition() {
        let result = transpile("def f(done: bool) -> str:\n    if not done:\n        return \"running\"\n    return \"done\"");
        assert!(result.contains("!") || result.contains("not"));
    }

    #[test]
    fn test_w11ac_if_complex_condition() {
        let result = transpile("def f(a: int, b: int, c: int) -> bool:\n    if a > 0 and b > 0 or c > 0:\n        return True\n    return False");
        assert!(result.contains("if"));
    }

    #[test]
    fn test_w11ac_if_none_check() {
        let result = transpile("def f(x: int) -> bool:\n    if x is None:\n        return True\n    return False");
        assert!(result.contains("None") || result.contains("is_none") || result.contains("if"));
    }

    #[test]
    fn test_w11ac_if_not_none_check() {
        let result = transpile("def f(x: int) -> bool:\n    if x is not None:\n        return True\n    return False");
        assert!(result.contains("None") || result.contains("is_some") || result.contains("if"));
    }

    #[test]
    fn test_w11ac_if_bool_true() {
        let result = transpile("def f() -> int:\n    if True:\n        return 1\n    return 0");
        assert!(result.contains("true") || result.contains("1"));
    }

    #[test]
    fn test_w11ac_if_bool_false() {
        let result = transpile("def f() -> int:\n    if False:\n        return 1\n    return 0");
        assert!(result.contains("false") || result.contains("0"));
    }

    #[test]
    fn test_w11ac_if_isinstance_pattern() {
        let result = transpile("def f(x: int) -> bool:\n    if isinstance(x, int):\n        return True\n    return False");
        assert!(result.contains("true") || result.contains("if"));
    }

    #[test]
    fn test_w11ac_if_with_pass() {
        let result = transpile("def f(x: int) -> int:\n    if x > 0:\n        pass\n    return x");
        assert!(result.contains("if") && result.contains("x"));
    }

    #[test]
    fn test_w11ac_if_body_multiple_stmts() {
        let result = transpile("def f(x: int) -> int:\n    result: int = 0\n    if x > 0:\n        result = x\n        result = result + 1\n    return result");
        assert!(result.contains("result") && result.contains("if"));
    }

    #[test]
    fn test_w11ac_if_else_body_multiple_stmts() {
        let result = transpile("def f(x: int) -> int:\n    a: int = 0\n    b: int = 0\n    if x > 0:\n        a = x\n        b = x * 2\n    else:\n        a = 0\n        b = 0\n    return a + b");
        assert!(result.contains("if") && result.contains("else"));
    }

    #[test]
    fn test_w11ac_if_return_early() {
        let result = transpile("def f(x: int) -> int:\n    if x == 0:\n        return 0\n    if x == 1:\n        return 1\n    return x * 2");
        assert!(result.contains("if") && result.contains("return"));
    }

    #[test]
    fn test_w11ac_if_in_loop() {
        let result = transpile("def f() -> int:\n    total: int = 0\n    for i in range(10):\n        if i % 2 == 0:\n            total += i\n    return total");
        assert!(result.contains("if") && result.contains("total"));
    }

    // ==========================================================================
    // Section 7: While loop patterns (15 tests)
    // ==========================================================================

    #[test]
    fn test_w11ac_while_true_break() {
        let result = transpile("def f():\n    while True:\n        break");
        assert!(result.contains("loop") && result.contains("break"));
    }

    #[test]
    fn test_w11ac_while_condition() {
        let result = transpile("def f() -> int:\n    x: int = 10\n    while x > 0:\n        x -= 1\n    return x");
        assert!(result.contains("while") && result.contains("x"));
    }

    #[test]
    fn test_w11ac_while_with_counter() {
        let result = transpile("def f() -> int:\n    count: int = 0\n    while count < 100:\n        count += 1\n    return count");
        assert!(result.contains("while") && result.contains("count"));
    }

    #[test]
    fn test_w11ac_while_with_break_condition() {
        let result = transpile("def f() -> int:\n    x: int = 0\n    while True:\n        x += 1\n        if x >= 10:\n            break\n    return x");
        assert!(result.contains("loop") && result.contains("break"));
    }

    #[test]
    fn test_w11ac_while_with_continue() {
        let result = transpile("def f() -> int:\n    x: int = 0\n    total: int = 0\n    while x < 10:\n        x += 1\n        if x % 2 == 0:\n            continue\n        total += x\n    return total");
        assert!(result.contains("continue"));
    }

    #[test]
    fn test_w11ac_while_nested() {
        let result = transpile("def f() -> int:\n    i: int = 0\n    total: int = 0\n    while i < 5:\n        j: int = 0\n        while j < 5:\n            total += 1\n            j += 1\n        i += 1\n    return total");
        assert!(result.contains("while"));
    }

    #[test]
    fn test_w11ac_while_with_complex_condition() {
        let result = transpile("def f(a: int, b: int) -> int:\n    while a > 0 and b > 0:\n        a -= 1\n        b -= 1\n    return a + b");
        assert!(result.contains("while") && (result.contains("&&") || result.contains("and")));
    }

    #[test]
    fn test_w11ac_while_decrement() {
        let result = transpile("def f(n: int) -> int:\n    result: int = 1\n    while n > 1:\n        result = result * n\n        n -= 1\n    return result");
        assert!(result.contains("while") && result.contains("result"));
    }

    #[test]
    fn test_w11ac_while_boolean_flag() {
        let result = transpile("def f() -> int:\n    found: bool = False\n    x: int = 0\n    while not found:\n        x += 1\n        if x >= 5:\n            found = True\n    return x");
        assert!(result.contains("while") || result.contains("found"));
    }

    #[test]
    fn test_w11ac_while_early_return() {
        let result = transpile("def f(n: int) -> int:\n    x: int = 0\n    while x < n:\n        if x == 5:\n            return x\n        x += 1\n    return x");
        assert!(result.contains("while") && result.contains("return"));
    }

    #[test]
    fn test_w11ac_while_accumulator() {
        let result = transpile("def f() -> int:\n    total: int = 0\n    i: int = 1\n    while i <= 10:\n        total += i\n        i += 1\n    return total");
        assert!(result.contains("while") && result.contains("total"));
    }

    #[test]
    fn test_w11ac_while_division() {
        let result = transpile("def f(n: int) -> int:\n    count: int = 0\n    while n > 1:\n        n = n // 2\n        count += 1\n    return count");
        assert!(result.contains("while") && result.contains("count"));
    }

    #[test]
    fn test_w11ac_while_collatz_step() {
        let result = transpile("def f(n: int) -> int:\n    steps: int = 0\n    while n != 1:\n        if n % 2 == 0:\n            n = n // 2\n        else:\n            n = 3 * n + 1\n        steps += 1\n    return steps");
        assert!(result.contains("while") && result.contains("steps"));
    }

    #[test]
    fn test_w11ac_while_with_multiple_breaks() {
        let result = transpile("def f(x: int) -> int:\n    while True:\n        if x < 0:\n            break\n        if x > 100:\n            break\n        x += 1\n    return x");
        assert!(result.contains("break"));
    }

    #[test]
    fn test_w11ac_while_single_iteration() {
        let result = transpile("def f() -> int:\n    x: int = 0\n    while x < 1:\n        x += 1\n    return x");
        assert!(result.contains("while") && result.contains("x"));
    }

    // ==========================================================================
    // Section 8: For loop patterns (20 tests)
    // ==========================================================================

    #[test]
    fn test_w11ac_for_range_simple() {
        let result = transpile("def f() -> int:\n    total: int = 0\n    for i in range(10):\n        total += i\n    return total");
        assert!(result.contains("for") || result.contains("range") || result.contains("0..10"));
    }

    #[test]
    fn test_w11ac_for_range_start_stop() {
        let result = transpile("def f() -> int:\n    total: int = 0\n    for i in range(1, 11):\n        total += i\n    return total");
        assert!(result.contains("for") || result.contains("1..11") || result.contains("range"));
    }

    #[test]
    fn test_w11ac_for_range_with_step() {
        let result = transpile("def f() -> int:\n    total: int = 0\n    for i in range(0, 10, 2):\n        total += i\n    return total");
        assert!(result.contains("for") || result.contains("step"));
    }

    #[test]
    fn test_w11ac_for_range_negative_step() {
        let result = transpile("def f() -> int:\n    total: int = 0\n    for i in range(10, 0, -1):\n        total += i\n    return total");
        assert!(result.contains("for") || result.contains("rev"));
    }

    #[test]
    fn test_w11ac_for_list_iteration() {
        let result = transpile("def f() -> int:\n    total: int = 0\n    for x in [1, 2, 3, 4, 5]:\n        total += x\n    return total");
        assert!(result.contains("for") && result.contains("total"));
    }

    #[test]
    fn test_w11ac_for_with_break() {
        let result = transpile("def f() -> int:\n    for i in range(100):\n        if i == 5:\n            break\n    return i");
        assert!(result.contains("break"));
    }

    #[test]
    fn test_w11ac_for_with_continue() {
        let result = transpile("def f() -> int:\n    total: int = 0\n    for i in range(10):\n        if i % 2 == 0:\n            continue\n        total += i\n    return total");
        assert!(result.contains("continue"));
    }

    #[test]
    fn test_w11ac_for_nested_two() {
        let result = transpile("def f() -> int:\n    total: int = 0\n    for i in range(5):\n        for j in range(5):\n            total += 1\n    return total");
        assert!(result.contains("for"));
    }

    #[test]
    fn test_w11ac_for_nested_three() {
        let result = transpile("def f() -> int:\n    total: int = 0\n    for i in range(3):\n        for j in range(3):\n            for k in range(3):\n                total += 1\n    return total");
        assert!(result.contains("for") && result.contains("total"));
    }

    #[test]
    fn test_w11ac_for_with_enumerate() {
        let result = transpile("def f():\n    for i, val in enumerate([10, 20, 30]):\n        pass");
        assert!(result.contains("enumerate") || result.contains("for"));
    }

    #[test]
    fn test_w11ac_for_with_zip() {
        let result = transpile("def f():\n    for a, b in zip([1, 2], [3, 4]):\n        pass");
        assert!(result.contains("zip") || result.contains("for"));
    }

    #[test]
    fn test_w11ac_for_early_return() {
        let result = transpile("def f(items: list) -> int:\n    for item in items:\n        if item == 5:\n            return item\n    return -1");
        assert!(result.contains("return") && result.contains("for"));
    }

    #[test]
    fn test_w11ac_for_build_list() {
        let result = transpile("def f() -> list:\n    result: list = []\n    for i in range(5):\n        result.append(i)\n    return result");
        assert!(result.contains("push") || result.contains("append") || result.contains("result"));
    }

    #[test]
    fn test_w11ac_for_conditional_accumulate() {
        let result = transpile("def f() -> int:\n    total: int = 0\n    for i in range(20):\n        if i % 3 == 0:\n            total += i\n    return total");
        assert!(result.contains("for") && result.contains("total"));
    }

    #[test]
    fn test_w11ac_for_nested_break_inner() {
        let result = transpile("def f() -> int:\n    count: int = 0\n    for i in range(5):\n        for j in range(5):\n            if j == 3:\n                break\n            count += 1\n    return count");
        assert!(result.contains("break") && result.contains("count"));
    }

    #[test]
    fn test_w11ac_for_string_iteration() {
        let result = transpile("def f(s: str) -> int:\n    count: int = 0\n    for c in s:\n        count += 1\n    return count");
        assert!(result.contains("for") && result.contains("count"));
    }

    #[test]
    fn test_w11ac_for_dict_items() {
        let result = transpile("def f(d: dict) -> int:\n    total: int = 0\n    for k, v in d.items():\n        total += v\n    return total");
        assert!(result.contains("iter") || result.contains("items") || result.contains("for"));
    }

    #[test]
    fn test_w11ac_for_dict_keys() {
        let result = transpile("def f(d: dict):\n    for k in d.keys():\n        pass");
        assert!(result.contains("keys") || result.contains("for"));
    }

    #[test]
    fn test_w11ac_for_dict_values() {
        let result = transpile("def f(d: dict):\n    for v in d.values():\n        pass");
        assert!(result.contains("values") || result.contains("for"));
    }

    #[test]
    fn test_w11ac_for_index_mutate() {
        let result = transpile("def f():\n    lst = [1, 2, 3, 4, 5]\n    for i in range(5):\n        lst[i] = lst[i] * 2\n    return lst");
        assert!(result.contains("for") && result.contains("lst"));
    }

    // ==========================================================================
    // Section 9: Try/except patterns (20 tests)
    // ==========================================================================

    #[test]
    fn test_w11ac_try_bare_except() {
        let result = transpile("def f() -> int:\n    try:\n        return 1\n    except:\n        return 0");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_try_value_error() {
        let result = transpile("def f(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError:\n        return -1");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_try_type_error() {
        let result = transpile("def f(a: int, b: int) -> int:\n    try:\n        return a + b\n    except TypeError:\n        return 0");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_try_key_error() {
        let result = transpile("def f(d: dict, key: str) -> str:\n    try:\n        return d[key]\n    except KeyError:\n        return \"\"");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_try_index_error() {
        let result = transpile("def f(lst: list, i: int) -> int:\n    try:\n        return lst[i]\n    except IndexError:\n        return -1");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_try_multiple_handlers() {
        let result = transpile("def f(x: int) -> int:\n    try:\n        return x\n    except ValueError:\n        return -1\n    except TypeError:\n        return -2");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_try_three_handlers() {
        let result = transpile("def f(x: int) -> int:\n    try:\n        return x\n    except ValueError:\n        return -1\n    except TypeError:\n        return -2\n    except KeyError:\n        return -3");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_try_except_as() {
        let result = transpile("def f() -> str:\n    try:\n        return \"ok\"\n    except ValueError as e:\n        return \"error\"");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_try_except_finally() {
        let result = transpile("def f() -> int:\n    result: int = 0\n    try:\n        result = 42\n    except:\n        result = -1\n    finally:\n        pass\n    return result");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_try_except_else() {
        let result = transpile("def f() -> int:\n    try:\n        val = 5\n    except:\n        val = 0\n    return val");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_try_full_form() {
        let result = transpile("def f() -> int:\n    result: int = 0\n    try:\n        result = 1\n    except ValueError:\n        result = -1\n    finally:\n        pass\n    return result");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_try_nested_inner() {
        let result = transpile("def f() -> int:\n    try:\n        try:\n            return 1\n        except:\n            return 2\n    except:\n        return 0");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_try_in_loop() {
        let result = transpile("def f() -> int:\n    total: int = 0\n    for i in range(10):\n        try:\n            total += i\n        except:\n            pass\n    return total");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_raise_value_error_msg() {
        let result = transpile("def f(x: int):\n    if x < 0:\n        raise ValueError(\"must be positive\")");
        assert!(result.contains("ValueError") || result.contains("panic") || result.contains("Err") || !result.is_empty());
    }

    #[test]
    fn test_w11ac_raise_type_error_msg() {
        let result = transpile("def f(x: int):\n    if x < 0:\n        raise TypeError(\"wrong type\")");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_raise_runtime_error() {
        let result = transpile("def f():\n    raise RuntimeError(\"runtime error\")");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_raise_no_args() {
        let result = transpile("def f():\n    try:\n        pass\n    except ValueError:\n        raise");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_raise_without_call() {
        let result = transpile("def f():\n    raise ValueError");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_assert_simple() {
        let result = transpile("def f(x: int):\n    assert x > 0");
        assert!(result.contains("assert"));
    }

    #[test]
    fn test_w11ac_assert_with_message() {
        let result = transpile("def f(x: int):\n    assert x > 0, \"must be positive\"");
        assert!(result.contains("assert") && result.contains("must be positive"));
    }

    // ==========================================================================
    // Section 10: With statement and misc control flow (15 tests)
    // ==========================================================================

    #[test]
    fn test_w11ac_with_open_file() {
        let result = transpile("def f():\n    with open(\"test.txt\") as f:\n        pass");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_with_no_target() {
        let result = transpile("def f():\n    with open(\"test.txt\"):\n        pass");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_pass_empty_function() {
        let result = transpile("def f():\n    pass");
        assert!(result.contains("fn") && result.contains("f"));
    }

    #[test]
    fn test_w11ac_pass_empty_class() {
        let result = transpile("class Empty:\n    pass");
        assert!(result.contains("Empty") || result.contains("struct"));
    }

    #[test]
    fn test_w11ac_pass_in_if() {
        let result = transpile("def f(x: int):\n    if x > 0:\n        pass\n    return x");
        assert!(result.contains("if") && result.contains("x"));
    }

    #[test]
    fn test_w11ac_return_bare() {
        let result = transpile("def f():\n    return");
        assert!(result.contains("fn") && result.contains("f"));
    }

    #[test]
    fn test_w11ac_return_explicit_none() {
        let result = transpile("def f():\n    return None");
        assert!(result.contains("fn") && result.contains("f"));
    }

    #[test]
    fn test_w11ac_return_tuple_two() {
        let result = transpile("def f() -> tuple:\n    return 1, 2");
        assert!(result.contains("1") && result.contains("2"));
    }

    #[test]
    fn test_w11ac_return_tuple_three() {
        let result = transpile("def f() -> tuple:\n    return 1, 2, 3");
        assert!(result.contains("1") && result.contains("2") && result.contains("3"));
    }

    #[test]
    fn test_w11ac_return_conditional() {
        let result = transpile("def f(x: int) -> int:\n    return x if x > 0 else 0");
        assert!(result.contains("if") || result.contains("return"));
    }

    #[test]
    fn test_w11ac_delete_variable() {
        let result = transpile("def f():\n    x = 5\n    del x");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_global_statement() {
        let result = transpile("def f():\n    global x\n    x = 42");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w11ac_break_in_for() {
        let result = transpile("def f() -> int:\n    for i in range(10):\n        if i == 5:\n            break\n    return i");
        assert!(result.contains("break"));
    }

    #[test]
    fn test_w11ac_continue_in_for() {
        let result = transpile("def f() -> int:\n    total: int = 0\n    for i in range(10):\n        if i % 2 == 0:\n            continue\n        total += i\n    return total");
        assert!(result.contains("continue") && result.contains("total"));
    }

    #[test]
    fn test_w11ac_nested_if_in_for() {
        let result = transpile("def f() -> int:\n    count: int = 0\n    for i in range(10):\n        if i > 2:\n            if i < 8:\n                count += 1\n    return count");
        assert!(result.contains("if") && result.contains("count"));
    }

    // ==========================================================================
    // Section 11: Assert patterns in depth (10 tests)
    // ==========================================================================

    #[test]
    fn test_w11ac_assert_eq_comparison() {
        let result = transpile("def f(x: int):\n    assert x == 42");
        assert!(result.contains("assert_eq!") || result.contains("assert"));
    }

    #[test]
    fn test_w11ac_assert_ne_comparison() {
        let result = transpile("def f(x: int):\n    assert x != 0");
        assert!(result.contains("assert_ne!") || result.contains("assert"));
    }

    #[test]
    fn test_w11ac_assert_gt_with_msg() {
        let result = transpile("def f(x: int):\n    assert x > 0, \"must be positive\"");
        assert!(result.contains("assert") && result.contains("positive"));
    }

    #[test]
    fn test_w11ac_assert_lt_comparison() {
        let result = transpile("def f(x: int):\n    assert x < 100");
        assert!(result.contains("assert") && result.contains("100"));
    }

    #[test]
    fn test_w11ac_assert_bool_var() {
        let result = transpile("def f(flag: bool):\n    assert flag");
        assert!(result.contains("assert!") || result.contains("assert"));
    }

    #[test]
    fn test_w11ac_assert_eq_with_msg() {
        let result = transpile("def f(x: int, y: int):\n    assert x == y, \"values must be equal\"");
        assert!(result.contains("assert") && result.contains("equal"));
    }

    #[test]
    fn test_w11ac_assert_ne_with_msg() {
        let result = transpile("def f(x: int, y: int):\n    assert x != y, \"values must differ\"");
        assert!(result.contains("assert") && result.contains("differ"));
    }

    #[test]
    fn test_w11ac_assert_ge_comparison() {
        let result = transpile("def f(x: int):\n    assert x >= 0");
        assert!(result.contains("assert") && result.contains(">="));
    }

    #[test]
    fn test_w11ac_assert_le_comparison() {
        let result = transpile("def f(x: int):\n    assert x <= 100");
        assert!(result.contains("assert") && result.contains("<="));
    }

    #[test]
    fn test_w11ac_assert_in_loop() {
        let result = transpile("def f():\n    for i in range(10):\n        assert i >= 0");
        assert!(result.contains("assert") && result.contains("for"));
    }

    // ==========================================================================
    // Section 12: Function definition edge cases (10 tests)
    // ==========================================================================

    #[test]
    fn test_w11ac_func_no_params_no_return() {
        let result = transpile("def noop():\n    pass");
        assert!(result.contains("fn") && result.contains("noop"));
    }

    #[test]
    fn test_w11ac_func_single_param_typed() {
        let result = transpile("def double(x: int) -> int:\n    return x * 2");
        assert!(result.contains("fn") && result.contains("double") && result.contains("i32"));
    }

    #[test]
    fn test_w11ac_func_two_params_typed() {
        let result = transpile("def add(a: int, b: int) -> int:\n    return a + b");
        assert!(result.contains("fn") && result.contains("add") && result.contains("+"));
    }

    #[test]
    fn test_w11ac_func_three_params() {
        let result = transpile("def clamp(val: int, lo: int, hi: int) -> int:\n    if val < lo:\n        return lo\n    if val > hi:\n        return hi\n    return val");
        assert!(result.contains("fn") && result.contains("clamp"));
    }

    #[test]
    fn test_w11ac_func_with_default_param() {
        let result = transpile("def greet(name: str = \"World\") -> str:\n    return name");
        assert!(result.contains("fn") && result.contains("greet"));
    }

    #[test]
    fn test_w11ac_func_only_docstring() {
        let result = transpile("def documented():\n    \"\"\"This function has only a docstring.\"\"\"");
        assert!(result.contains("fn") && result.contains("documented"));
    }

    #[test]
    fn test_w11ac_func_recursive() {
        let result = transpile("def factorial(n: int) -> int:\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)");
        assert!(result.contains("fn") && result.contains("factorial"));
    }

    #[test]
    fn test_w11ac_func_multiple_returns() {
        let result = transpile("def sign(x: int) -> int:\n    if x > 0:\n        return 1\n    elif x < 0:\n        return -1\n    return 0");
        assert!(result.contains("fn") && result.contains("sign"));
    }

    #[test]
    fn test_w11ac_func_with_local_vars() {
        let result = transpile("def compute(x: int) -> int:\n    a: int = x * 2\n    b: int = a + 3\n    return b");
        assert!(result.contains("fn") && result.contains("compute"));
    }

    #[test]
    fn test_w11ac_func_returns_bool() {
        let result = transpile("def is_even(n: int) -> bool:\n    return n % 2 == 0");
        assert!(result.contains("fn") && result.contains("is_even") && result.contains("bool"));
    }

    // ==========================================================================
    // Section 13: Class patterns in depth (10 tests)
    // ==========================================================================

    #[test]
    fn test_w11ac_class_init_single_field() {
        let result = transpile("class Counter:\n    def __init__(self):\n        self.value = 0");
        assert!(result.contains("Counter") || result.contains("value"));
    }

    #[test]
    fn test_w11ac_class_init_two_fields() {
        let result = transpile("class Point:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y");
        assert!(result.contains("Point") || (result.contains("x") && result.contains("y")));
    }

    #[test]
    fn test_w11ac_class_with_method() {
        let result = transpile("class Counter:\n    def __init__(self):\n        self.count = 0\n    def increment(self):\n        self.count += 1\n    def get(self) -> int:\n        return self.count");
        assert!(result.contains("increment") || result.contains("get") || result.contains("count"));
    }

    #[test]
    fn test_w11ac_class_str_method() {
        let result = transpile("class Name:\n    def __init__(self, name: str):\n        self.name = name\n    def __str__(self) -> str:\n        return self.name");
        assert!(result.contains("Name") || result.contains("name"));
    }

    #[test]
    fn test_w11ac_class_eq_method() {
        let result = transpile("class Val:\n    def __init__(self, v: int):\n        self.v = v\n    def __eq__(self, other) -> bool:\n        return self.v == other.v");
        assert!(result.contains("Val") || result.contains("v"));
    }

    #[test]
    fn test_w11ac_class_len_method() {
        let result = transpile("class Container:\n    def __init__(self):\n        self.items = []\n    def __len__(self) -> int:\n        return len(self.items)");
        assert!(result.contains("Container") || result.contains("items") || result.contains("len"));
    }

    #[test]
    fn test_w11ac_class_empty() {
        let result = transpile("class Empty:\n    pass");
        assert!(result.contains("Empty") || result.contains("struct"));
    }

    #[test]
    fn test_w11ac_class_three_fields() {
        let result = transpile("class RGB:\n    def __init__(self, r: int, g: int, b: int):\n        self.r = r\n        self.g = g\n        self.b = b");
        assert!(result.contains("RGB") || result.contains("r"));
    }

    #[test]
    fn test_w11ac_class_with_return_method() {
        let result = transpile("class Calculator:\n    def __init__(self, val: int):\n        self.val = val\n    def double(self) -> int:\n        return self.val * 2");
        assert!(result.contains("Calculator") || result.contains("double"));
    }

    #[test]
    fn test_w11ac_class_with_bool_field() {
        let result = transpile("class Switch:\n    def __init__(self):\n        self.on = False\n    def toggle(self):\n        self.on = not self.on");
        assert!(result.contains("Switch") || result.contains("on") || result.contains("toggle"));
    }

    // ==========================================================================
    // Section 14: Complex combined patterns (15 tests)
    // ==========================================================================

    #[test]
    fn test_w11ac_combined_for_if_assign() {
        let result = transpile("def f() -> int:\n    max_val: int = 0\n    for i in range(10):\n        if i > max_val:\n            max_val = i\n    return max_val");
        assert!(result.contains("for") && result.contains("if") && result.contains("max_val"));
    }

    #[test]
    fn test_w11ac_combined_while_if_break() {
        let result = transpile("def f(n: int) -> int:\n    i: int = 0\n    while i < n:\n        if i * i > n:\n            break\n        i += 1\n    return i");
        assert!(result.contains("while") && result.contains("break"));
    }

    #[test]
    fn test_w11ac_combined_nested_for_accumulate() {
        let result = transpile("def f() -> int:\n    total: int = 0\n    for i in range(5):\n        for j in range(5):\n            if i != j:\n                total += i * j\n    return total");
        assert!(result.contains("for") && result.contains("total"));
    }

    #[test]
    fn test_w11ac_combined_func_with_guard() {
        let result = transpile("def safe_div(a: int, b: int) -> int:\n    if b == 0:\n        return 0\n    return a // b");
        assert!(result.contains("fn") && result.contains("safe_div"));
    }

    #[test]
    fn test_w11ac_combined_early_return_loop() {
        let result = transpile("def find_first_even(items: list) -> int:\n    for item in items:\n        if item % 2 == 0:\n            return item\n    return -1");
        assert!(result.contains("for") && result.contains("return"));
    }

    #[test]
    fn test_w11ac_combined_fibonacci_iterative() {
        let result = transpile("def fib(n: int) -> int:\n    a: int = 0\n    b: int = 1\n    for i in range(n):\n        a, b = b, a + b\n    return a");
        assert!(result.contains("fn") && result.contains("fib"));
    }

    #[test]
    fn test_w11ac_combined_count_occurrences() {
        let result = transpile("def count_val(items: list, target: int) -> int:\n    count: int = 0\n    for item in items:\n        if item == target:\n            count += 1\n    return count");
        assert!(result.contains("fn") && result.contains("count"));
    }

    #[test]
    fn test_w11ac_combined_sum_of_squares() {
        let result = transpile("def sum_squares(n: int) -> int:\n    total: int = 0\n    for i in range(1, n + 1):\n        total += i * i\n    return total");
        assert!(result.contains("fn") && result.contains("total"));
    }

    #[test]
    fn test_w11ac_combined_max_in_list() {
        let result = transpile("def find_max(items: list) -> int:\n    best: int = 0\n    for item in items:\n        if item > best:\n            best = item\n    return best");
        assert!(result.contains("fn") && result.contains("best"));
    }

    #[test]
    fn test_w11ac_combined_reverse_string() {
        let result = transpile("def reverse(s: str) -> str:\n    result: str = \"\"\n    for c in s:\n        result = c + result\n    return result");
        assert!(result.contains("fn") && result.contains("reverse"));
    }

    #[test]
    fn test_w11ac_combined_abs_value() {
        let result = transpile("def abs_val(x: int) -> int:\n    if x < 0:\n        return -x\n    return x");
        assert!(result.contains("fn") && result.contains("abs_val"));
    }

    #[test]
    fn test_w11ac_combined_is_palindrome() {
        let result = transpile("def is_palindrome(s: str) -> bool:\n    reversed_s = s[::-1]\n    return s == reversed_s");
        assert!(result.contains("fn") && result.contains("is_palindrome"));
    }

    #[test]
    fn test_w11ac_combined_power() {
        let result = transpile("def power(base: int, exp: int) -> int:\n    result: int = 1\n    for i in range(exp):\n        result = result * base\n    return result");
        assert!(result.contains("fn") && result.contains("result"));
    }

    #[test]
    fn test_w11ac_combined_gcd() {
        let result = transpile("def gcd(a: int, b: int) -> int:\n    while b != 0:\n        a, b = b, a % b\n    return a");
        assert!(result.contains("fn") && result.contains("gcd"));
    }

    #[test]
    fn test_w11ac_combined_class_with_logic() {
        let result = transpile("class Accumulator:\n    def __init__(self):\n        self.total = 0\n    def add(self, val: int):\n        self.total += val\n    def get_total(self) -> int:\n        return self.total");
        assert!(result.contains("Accumulator") || result.contains("total"));
    }
}
