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

    // Binary operations - Integer (tests 1-7)
    #[test]
    fn test_w23ed_001() {
        let result = transpile("def add(a: int, b: int) -> int:\n    return a + b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_002() {
        let result = transpile("def sub(a: int, b: int) -> int:\n    return a - b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_003() {
        let result = transpile("def mul(a: int, b: int) -> int:\n    return a * b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_004() {
        let result = transpile("def div(a: int, b: int) -> int:\n    return a / b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_005() {
        let result = transpile("def mod_op(a: int, b: int) -> int:\n    return a % b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_006() {
        let result = transpile("def floordiv(a: int, b: int) -> int:\n    return a // b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_007() {
        let result = transpile("def power(a: int, b: int) -> int:\n    return a ** b");
        assert!(!result.is_empty());
    }

    // Binary operations - Float (tests 8-14)
    #[test]
    fn test_w23ed_008() {
        let result = transpile("def add_f(a: float, b: float) -> float:\n    return a + b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_009() {
        let result = transpile("def sub_f(a: float, b: float) -> float:\n    return a - b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_010() {
        let result = transpile("def mul_f(a: float, b: float) -> float:\n    return a * b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_011() {
        let result = transpile("def div_f(a: float, b: float) -> float:\n    return a / b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_012() {
        let result = transpile("def mod_f(a: float, b: float) -> float:\n    return a % b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_013() {
        let result = transpile("def floordiv_f(a: float, b: float) -> float:\n    return a // b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_014() {
        let result = transpile("def power_f(a: float, b: float) -> float:\n    return a ** b");
        assert!(!result.is_empty());
    }

    // Binary operations - Mixed (tests 15-16)
    #[test]
    fn test_w23ed_015() {
        let result = transpile("def mixed_add(a: int, b: float) -> float:\n    return a + b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_016() {
        let result = transpile("def mixed_sub(a: float, b: int) -> float:\n    return a - b");
        assert!(!result.is_empty());
    }

    // Binary operations - Bitwise (tests 17-21)
    #[test]
    fn test_w23ed_017() {
        let result = transpile("def bitwise_and(a: int, b: int) -> int:\n    return a & b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_018() {
        let result = transpile("def bitwise_or(a: int, b: int) -> int:\n    return a | b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_019() {
        let result = transpile("def bitwise_xor(a: int, b: int) -> int:\n    return a ^ b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_020() {
        let result = transpile("def left_shift(a: int, b: int) -> int:\n    return a << b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_021() {
        let result = transpile("def right_shift(a: int, b: int) -> int:\n    return a >> b");
        assert!(!result.is_empty());
    }

    // Binary operations - Comparison (tests 22-27)
    #[test]
    fn test_w23ed_022() {
        let result = transpile("def eq(a: int, b: int) -> bool:\n    return a == b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_023() {
        let result = transpile("def ne(a: int, b: int) -> bool:\n    return a != b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_024() {
        let result = transpile("def lt(a: int, b: int) -> bool:\n    return a < b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_025() {
        let result = transpile("def gt(a: int, b: int) -> bool:\n    return a > b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_026() {
        let result = transpile("def le(a: int, b: int) -> bool:\n    return a <= b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_027() {
        let result = transpile("def ge(a: int, b: int) -> bool:\n    return a >= b");
        assert!(!result.is_empty());
    }

    // Binary operations - Boolean (tests 28-30)
    #[test]
    fn test_w23ed_028() {
        let result = transpile("def bool_and(a: bool, b: bool) -> bool:\n    return a and b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_029() {
        let result = transpile("def bool_or(a: bool, b: bool) -> bool:\n    return a or b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_030() {
        let result = transpile("def bool_not(a: bool) -> bool:\n    return not a");
        assert!(!result.is_empty());
    }

    // Binary operations - String (tests 31-32)
    #[test]
    fn test_w23ed_031() {
        let result = transpile("def str_concat(s1: str, s2: str) -> str:\n    return s1 + s2");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_032() {
        let result = transpile("def str_repeat(s: str, n: int) -> str:\n    return s * n");
        assert!(!result.is_empty());
    }

    // Binary operations - In/Not in (tests 33-34)
    #[test]
    fn test_w23ed_033() {
        let result = transpile("def contains(x: int, lst: list) -> bool:\n    return x in lst");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_034() {
        let result = transpile("def not_contains(x: int, lst: list) -> bool:\n    return x not in lst");
        assert!(!result.is_empty());
    }

    // Binary operations - Is/Is not (tests 35-36)
    #[test]
    fn test_w23ed_035() {
        let result = transpile("def is_none(x) -> bool:\n    return x is None");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_036() {
        let result = transpile("def is_not_none(x) -> bool:\n    return x is not None");
        assert!(!result.is_empty());
    }

    // Binary operations - Chained comparisons (tests 37-40)
    #[test]
    fn test_w23ed_037() {
        let result = transpile("def chained_lt(a: int, b: int, c: int) -> bool:\n    return a < b < c");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_038() {
        let result = transpile("def chained_le(a: int, b: int, c: int) -> bool:\n    return a <= b <= c");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_039() {
        let result = transpile("def chained_eq(a: int, b: int, c: int) -> bool:\n    return a == b == c");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_040() {
        let result = transpile("def chained_mixed(a: int, b: int, c: int, d: int) -> bool:\n    return a < b <= c < d");
        assert!(!result.is_empty());
    }

    // Unary operations (tests 41-60)
    #[test]
    fn test_w23ed_041() {
        let result = transpile("def neg_int(x: int) -> int:\n    return -x");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_042() {
        let result = transpile("def neg_literal() -> int:\n    return -5");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_043() {
        let result = transpile("def neg_float(x: float) -> float:\n    return -x");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_044() {
        let result = transpile("def neg_float_literal() -> float:\n    return -3.5");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_045() {
        let result = transpile("def bool_not_var(x: bool) -> bool:\n    return not x");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_046() {
        let result = transpile("def bool_not_true() -> bool:\n    return not True");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_047() {
        let result = transpile("def bool_not_false() -> bool:\n    return not False");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_048() {
        let result = transpile("def bitwise_not(x: int) -> int:\n    return ~x");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_049() {
        let result = transpile("def bitwise_not_literal() -> int:\n    return ~42");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_050() {
        let result = transpile("def positive_int(x: int) -> int:\n    return +x");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_051() {
        let result = transpile("def positive_float(x: float) -> float:\n    return +x");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_052() {
        let result = transpile("def double_neg(x: int) -> int:\n    return --x");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_053() {
        let result = transpile("def not_not(x: bool) -> bool:\n    return not not x");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_054() {
        let result = transpile("def neg_expr(a: int, b: int) -> int:\n    return -(a + b)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_055() {
        let result = transpile("def not_expr(a: int, b: int) -> bool:\n    return not (a > b)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_056() {
        let result = transpile("def bitwise_not_expr(a: int, b: int) -> int:\n    return ~(a & b)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_057() {
        let result = transpile("def neg_in_binop(x: int, y: int) -> int:\n    return -x + y");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_058() {
        let result = transpile("def not_in_binop(a: bool, b: bool) -> bool:\n    return not a and b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_059() {
        let result = transpile("def complex_unary(x: int) -> int:\n    return -(+x)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_060() {
        let result = transpile("def unary_chain(x: int) -> int:\n    return -(-(-x))");
        assert!(!result.is_empty());
    }

    // Comprehensions - List (tests 61-70)
    #[test]
    fn test_w23ed_061() {
        let result = transpile("def list_comp_simple(lst: list) -> list:\n    return [x for x in lst]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_062() {
        let result = transpile("def list_comp_range() -> list:\n    return [x for x in range(10)]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_063() {
        let result = transpile("def list_comp_expr() -> list:\n    return [x*2 for x in range(10)]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_064() {
        let result = transpile("def list_comp_filter(lst: list) -> list:\n    return [x for x in lst if x > 0]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_065() {
        let result = transpile("def list_comp_filter_expr(lst: list) -> list:\n    return [x*2 for x in lst if x > 0]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_066() {
        let result = transpile("def list_comp_nested(a: list, b: list) -> list:\n    return [x+y for x in a for y in b]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_067() {
        let result = transpile("def list_comp_nested_filter(a: list, b: list) -> list:\n    return [x+y for x in a for y in b if x > 0 and y > 0]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_068() {
        let result = transpile("def list_comp_tuple() -> list:\n    return [(x, x*2) for x in range(5)]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_069() {
        let result = transpile("def list_comp_str(words: list) -> list:\n    return [w.upper() for w in words]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_070() {
        let result = transpile("def list_comp_complex() -> list:\n    return [x**2 + x + 1 for x in range(10) if x % 2 == 0]");
        assert!(!result.is_empty());
    }

    // Comprehensions - Dict (tests 71-78)
    #[test]
    fn test_w23ed_071() {
        let result = transpile("def dict_comp_simple(items: list) -> dict:\n    return {k: v for k, v in items}");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_072() {
        let result = transpile("def dict_comp_range() -> dict:\n    return {x: x*2 for x in range(5)}");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_073() {
        let result = transpile("def dict_comp_filter(d: dict) -> dict:\n    return {k: v for k, v in d.items() if v > 0}");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_074() {
        let result = transpile("def dict_comp_expr() -> dict:\n    return {x: x**2 for x in range(10)}");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_075() {
        let result = transpile("def dict_comp_str(words: list) -> dict:\n    return {w: len(w) for w in words}");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_076() {
        let result = transpile("def dict_comp_tuple_key() -> dict:\n    return {(x, y): x+y for x in range(3) for y in range(3)}");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_077() {
        let result = transpile("def dict_comp_enumerate(lst: list) -> dict:\n    return {i: v for i, v in enumerate(lst)}");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_078() {
        let result = transpile("def dict_comp_zip(keys: list, vals: list) -> dict:\n    return {k: v for k, v in zip(keys, vals)}");
        assert!(!result.is_empty());
    }

    // Comprehensions - Set (tests 79-83)
    #[test]
    fn test_w23ed_079() {
        let result = transpile("def set_comp_simple(lst: list) -> set:\n    return {x for x in lst}");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_080() {
        let result = transpile("def set_comp_expr() -> set:\n    return {x*2 for x in range(10)}");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_081() {
        let result = transpile("def set_comp_filter(lst: list) -> set:\n    return {x for x in lst if x > 0}");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_082() {
        let result = transpile("def set_comp_nested(a: list, b: list) -> set:\n    return {x+y for x in a for y in b}");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_083() {
        let result = transpile("def set_comp_str(words: list) -> set:\n    return {w.lower() for w in words}");
        assert!(!result.is_empty());
    }

    // Comprehensions - Generator (tests 84-100)
    #[test]
    fn test_w23ed_084() {
        let result = transpile("def gen_expr_sum(lst: list) -> int:\n    return sum(x for x in lst)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_085() {
        let result = transpile("def gen_expr_any(lst: list) -> bool:\n    return any(x > 0 for x in lst)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_086() {
        let result = transpile("def gen_expr_all(lst: list) -> bool:\n    return all(x > 0 for x in lst)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_087() {
        let result = transpile("def gen_expr_max(lst: list) -> int:\n    return max(x for x in lst)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_088() {
        let result = transpile("def gen_expr_min(lst: list) -> int:\n    return min(x for x in lst)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_089() {
        let result = transpile("def gen_expr_expr() -> int:\n    return sum(x*2 for x in range(10))");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_090() {
        let result = transpile("def gen_expr_filter() -> int:\n    return sum(x for x in range(10) if x % 2 == 0)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_091() {
        let result = transpile("def gen_expr_nested(a: list, b: list) -> int:\n    return sum(x+y for x in a for y in b)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_092() {
        let result = transpile("def gen_expr_tuple() -> list:\n    return list((x, x*2) for x in range(5))");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_093() {
        let result = transpile("def gen_expr_str(words: list) -> list:\n    return list(w.upper() for w in words)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_094() {
        let result = transpile("def nested_list_comp() -> list:\n    return [[j for j in range(3)] for i in range(3)]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_095() {
        let result = transpile("def nested_list_comp_expr() -> list:\n    return [[i*j for j in range(3)] for i in range(3)]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_096() {
        let result = transpile("def nested_list_comp_filter() -> list:\n    return [[j for j in range(10) if j > i] for i in range(5)]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_097() {
        let result = transpile("def list_comp_triple_nested() -> list:\n    return [x+y+z for x in range(2) for y in range(2) for z in range(2)]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_098() {
        let result = transpile("def gen_expr_complex() -> int:\n    return sum(x**2 for x in range(100) if x % 3 == 0)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_099() {
        let result = transpile("def gen_expr_in_list() -> list:\n    return [sum(y for y in range(x)) for x in range(5)]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_100() {
        let result = transpile("def dict_comp_nested() -> dict:\n    return {x: [y for y in range(x)] for x in range(5)}");
        assert!(!result.is_empty());
    }

    // Lambda expressions (tests 101-120)
    #[test]
    fn test_w23ed_101() {
        let result = transpile("def lambda_simple() -> int:\n    f = lambda x: x + 1\n    return f(5)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_102() {
        let result = transpile("def lambda_multi_arg() -> int:\n    f = lambda x, y: x + y\n    return f(3, 4)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_103() {
        let result = transpile("def lambda_in_sorted(lst: list) -> list:\n    return sorted(lst, key=lambda x: x[1])");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_104() {
        let result = transpile("def lambda_in_map(lst: list) -> list:\n    return list(map(lambda x: x*2, lst))");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_105() {
        let result = transpile("def lambda_in_filter(lst: list) -> list:\n    return list(filter(lambda x: x > 0, lst))");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_106() {
        let result = transpile("def lambda_expr() -> int:\n    f = lambda x: x * 2 + 1\n    return f(5)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_107() {
        let result = transpile("def lambda_nested() -> int:\n    f = lambda x: (lambda y: x + y)\n    return f(3)(4)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_108() {
        let result = transpile("def lambda_three_args() -> int:\n    f = lambda x, y, z: x + y + z\n    return f(1, 2, 3)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_109() {
        let result = transpile("def lambda_tuple_arg() -> int:\n    f = lambda t: t[0] + t[1]\n    return f((3, 4))");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_110() {
        let result = transpile("def lambda_dict_access() -> int:\n    f = lambda d: d['key']\n    return f({'key': 42})");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_111() {
        let result = transpile("def lambda_comparison() -> bool:\n    f = lambda x, y: x > y\n    return f(5, 3)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_112() {
        let result = transpile("def lambda_boolean() -> bool:\n    f = lambda x, y: x and y\n    return f(True, False)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_113() {
        let result = transpile("def lambda_str_op() -> str:\n    f = lambda s: s.upper()\n    return f('hello')");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_114() {
        let result = transpile("def lambda_list_slice() -> list:\n    f = lambda lst: lst[1:3]\n    return f([1, 2, 3, 4, 5])");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_115() {
        let result = transpile("def lambda_power() -> int:\n    f = lambda x: x ** 2\n    return f(5)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_116() {
        let result = transpile("def lambda_bitwise() -> int:\n    f = lambda x, y: x & y\n    return f(12, 10)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_117() {
        let result = transpile("def lambda_in_reduce(lst: list) -> int:\n    from functools import reduce\n    return reduce(lambda x, y: x + y, lst)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_118() {
        let result = transpile("def lambda_conditional() -> int:\n    f = lambda x: x if x > 0 else -x\n    return f(-5)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_119() {
        let result = transpile("def lambda_complex_expr() -> int:\n    f = lambda x, y, z: (x + y) * z\n    return f(2, 3, 4)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_120() {
        let result = transpile("def lambda_method_chain() -> str:\n    f = lambda s: s.strip().upper()\n    return f('  hello  ')");
        assert!(!result.is_empty());
    }

    // F-strings (tests 121-150)
    #[test]
    fn test_w23ed_121() {
        let result = transpile("def fstring_simple(name: str) -> str:\n    return f'hello {name}'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_122() {
        let result = transpile("def fstring_expr(a: int, b: int) -> str:\n    return f'result = {a + b}'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_123() {
        let result = transpile("def fstring_format_float(value: float) -> str:\n    return f'{value:.2f}'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_124() {
        let result = transpile("def fstring_format_width(x: int) -> str:\n    return f'{x:>10}'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_125() {
        let result = transpile("def fstring_multiple(a: int, b: int, c: int) -> str:\n    return f'{a} and {b} and {c}'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_126() {
        let result = transpile("def fstring_nested_ternary(x: int) -> str:\n    return f\"{'yes' if x else 'no'}\"");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_127() {
        let result = transpile("def fstring_method_call(s: str) -> str:\n    return f'{s.upper()}'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_128() {
        let result = transpile("def fstring_index(lst: list) -> str:\n    return f'{lst[0]}'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_129() {
        let result = transpile("def fstring_empty() -> str:\n    return f''");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_130() {
        let result = transpile("def fstring_literal_only() -> str:\n    return f'hello world'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_131() {
        let result = transpile("def fstring_arithmetic(x: int, y: int) -> str:\n    return f'{x * y}'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_132() {
        let result = transpile("def fstring_comparison(a: int, b: int) -> str:\n    return f'{a > b}'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_133() {
        let result = transpile("def fstring_slice(s: str) -> str:\n    return f'{s[1:3]}'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_134() {
        let result = transpile("def fstring_dict_access(d: dict) -> str:\n    return f\"{d['key']}\"");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_135() {
        let result = transpile("def fstring_concat(s1: str, s2: str) -> str:\n    return f'{s1}' + f'{s2}'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_136() {
        let result = transpile("def fstring_padding(x: int) -> str:\n    return f'{x:05d}'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_137() {
        let result = transpile("def fstring_percent(value: float) -> str:\n    return f'{value:.1%}'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_138() {
        let result = transpile("def fstring_hex(n: int) -> str:\n    return f'{n:x}'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_139() {
        let result = transpile("def fstring_binary(n: int) -> str:\n    return f'{n:b}'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_140() {
        let result = transpile("def fstring_octal(n: int) -> str:\n    return f'{n:o}'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_141() {
        let result = transpile("def fstring_scientific(value: float) -> str:\n    return f'{value:e}'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_142() {
        let result = transpile("def fstring_align_left(s: str) -> str:\n    return f'{s:<20}'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_143() {
        let result = transpile("def fstring_align_center(s: str) -> str:\n    return f'{s:^20}'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_144() {
        let result = transpile("def fstring_sign(x: int) -> str:\n    return f'{x:+d}'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_145() {
        let result = transpile("def fstring_comma(n: int) -> str:\n    return f'{n:,}'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_146() {
        let result = transpile("def fstring_nested_expr(a: int, b: int, c: int) -> str:\n    return f'{a + (b * c)}'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_147() {
        let result = transpile("def fstring_len_call(s: str) -> str:\n    return f'length: {len(s)}'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_148() {
        let result = transpile("def fstring_tuple(t: tuple) -> str:\n    return f'{t[0]}, {t[1]}'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_149() {
        let result = transpile("def fstring_multiline() -> str:\n    x = 42\n    return f'value is {x}'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_150() {
        let result = transpile("def fstring_escape_brace() -> str:\n    x = 5\n    return f'{{x}} = {x}'");
        assert!(!result.is_empty());
    }

    // Ternary/conditional expressions (tests 151-170)
    #[test]
    fn test_w23ed_151() {
        let result = transpile("def ternary_simple(x: int, cond: bool) -> int:\n    return x if cond else 0");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_152() {
        let result = transpile("def ternary_nested(c1: bool, c2: bool) -> str:\n    return 'a' if c1 else 'b' if c2 else 'c'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_153() {
        let result = transpile("def ternary_in_assign(x: int) -> int:\n    result = x if x > 0 else -x\n    return result");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_154() {
        let result = transpile("def ternary_in_return(x: int) -> int:\n    return x if x else 0");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_155() {
        let result = transpile("def ternary_in_call(x: int) -> None:\n    print(x if x else 'none')");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_156() {
        let result = transpile("def ternary_comparison(a: int, b: int) -> int:\n    return a if a > b else b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_157() {
        let result = transpile("def ternary_expr_cond(x: int, y: int) -> int:\n    return 1 if x > y else 0");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_158() {
        let result = transpile("def ternary_both_expr(x: int, y: int) -> int:\n    return x*2 if x > 0 else y*2");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_159() {
        let result = transpile("def ternary_boolean_cond(flag: bool) -> str:\n    return 'yes' if flag else 'no'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_160() {
        let result = transpile("def ternary_none_check(x) -> int:\n    return x if x is not None else 0");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_161() {
        let result = transpile("def ternary_in_binop(x: int, y: int) -> int:\n    return (x if x > 0 else -x) + y");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_162() {
        let result = transpile("def ternary_str(s: str) -> str:\n    return s.upper() if s else ''");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_163() {
        let result = transpile("def ternary_list(lst: list) -> list:\n    return lst if lst else []");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_164() {
        let result = transpile("def ternary_dict(d: dict) -> dict:\n    return d if d else {}");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_165() {
        let result = transpile("def ternary_triple_nested(c1: bool, c2: bool, c3: bool) -> int:\n    return 1 if c1 else 2 if c2 else 3 if c3 else 4");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_166() {
        let result = transpile("def ternary_in_list(x: int) -> list:\n    return [x if x > 0 else 0]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_167() {
        let result = transpile("def ternary_in_tuple(x: int) -> tuple:\n    return (x if x > 0 else 0,)");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_168() {
        let result = transpile("def ternary_in_dict(x: int) -> dict:\n    return {'value': x if x > 0 else 0}");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_169() {
        let result = transpile("def ternary_arithmetic(x: int, y: int) -> int:\n    return x + y if x > 0 else x - y");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_170() {
        let result = transpile("def ternary_complex_cond(x: int, y: int, z: int) -> int:\n    return x if x > y and y > z else z");
        assert!(!result.is_empty());
    }

    // Subscript/index expressions (tests 171-200)
    #[test]
    fn test_w23ed_171() {
        let result = transpile("def list_index_first(lst: list) -> int:\n    return lst[0]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_172() {
        let result = transpile("def list_index_last(lst: list) -> int:\n    return lst[-1]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_173() {
        let result = transpile("def list_index_var(lst: list, i: int) -> int:\n    return lst[i]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_174() {
        let result = transpile("def dict_index_str(d: dict) -> int:\n    return d['key']");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_175() {
        let result = transpile("def dict_index_var(d: dict, key: str) -> int:\n    return d[key]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_176() {
        let result = transpile("def list_slice_range(lst: list) -> list:\n    return lst[1:3]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_177() {
        let result = transpile("def list_slice_from_start(lst: list) -> list:\n    return lst[:5]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_178() {
        let result = transpile("def list_slice_step(lst: list) -> list:\n    return lst[::2]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_179() {
        let result = transpile("def list_slice_full(lst: list) -> list:\n    return lst[1:10:2]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_180() {
        let result = transpile("def list_slice_negative(lst: list) -> list:\n    return lst[-3:]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_181() {
        let result = transpile("def str_index_first(s: str) -> str:\n    return s[0]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_182() {
        let result = transpile("def str_index_last(s: str) -> str:\n    return s[-1]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_183() {
        let result = transpile("def str_slice(s: str) -> str:\n    return s[1:3]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_184() {
        let result = transpile("def nested_index(matrix: list) -> int:\n    return matrix[0][1]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_185() {
        let result = transpile("def tuple_index_first(t: tuple) -> int:\n    return t[0]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_186() {
        let result = transpile("def tuple_index_second(t: tuple) -> int:\n    return t[1]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_187() {
        let result = transpile("def index_in_expr(lst: list, i: int, j: int) -> int:\n    return lst[i] + lst[j]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_188() {
        let result = transpile("def index_expr(lst: list, a: int, b: int) -> int:\n    return lst[a + b]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_189() {
        let result = transpile("def slice_negative_step(lst: list) -> list:\n    return lst[::-1]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_190() {
        let result = transpile("def slice_to_end(lst: list) -> list:\n    return lst[5:]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_191() {
        let result = transpile("def slice_with_vars(lst: list, start: int, end: int) -> list:\n    return lst[start:end]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_192() {
        let result = transpile("def triple_nested_index(data: list) -> int:\n    return data[0][1][2]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_193() {
        let result = transpile("def dict_of_lists(d: dict) -> int:\n    return d['key'][0]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_194() {
        let result = transpile("def list_of_dicts(lst: list) -> int:\n    return lst[0]['key']");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_195() {
        let result = transpile("def index_negative_two(lst: list) -> int:\n    return lst[-2]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_196() {
        let result = transpile("def slice_negative_start(lst: list) -> list:\n    return lst[-5:-2]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_197() {
        let result = transpile("def slice_all(lst: list) -> list:\n    return lst[:]");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_198() {
        let result = transpile("def index_in_call(lst: list, i: int) -> None:\n    print(lst[i])");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_199() {
        let result = transpile("def index_in_condition(lst: list, i: int) -> bool:\n    return lst[i] > 0");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w23ed_200() {
        let result = transpile("def complex_index_expr(lst: list, a: int, b: int) -> int:\n    return lst[(a * 2) + (b - 1)]");
        assert!(!result.is_empty());
    }
}
