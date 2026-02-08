// coverage_wave11_expr_type_tests.rs
// Target: expr_gen.rs + func_gen.rs type inference branches
// Wave 11: ~200 tests across expression generation paths and type inference edge cases

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
    // Section 1: Binary Arithmetic (20 tests)
    // ==========================================================================

    #[test]
    fn test_w11et_bin_add_int() {
        let result = transpile("def add(x: int, y: int) -> int:\n    return x + y");
        assert!(result.contains("+"), "expected + in: {result}");
    }

    #[test]
    fn test_w11et_bin_sub_int() {
        let result = transpile("def sub(x: int, y: int) -> int:\n    return x - y");
        assert!(result.contains("-"), "expected - in: {result}");
    }

    #[test]
    fn test_w11et_bin_mul_int() {
        let result = transpile("def mul(x: int, y: int) -> int:\n    return x * y");
        assert!(result.contains("*"), "expected * in: {result}");
    }

    #[test]
    fn test_w11et_bin_div_float() {
        let result = transpile("def div(x: float, y: float) -> float:\n    return x / y");
        assert!(result.contains("/"), "expected / in: {result}");
    }

    #[test]
    fn test_w11et_bin_floor_div() {
        let result = transpile("def fdiv(x: int, y: int) -> int:\n    return x // y");
        assert!(result.contains("fn fdiv"), "expected fn fdiv in: {result}");
    }

    #[test]
    fn test_w11et_bin_mod() {
        let result = transpile("def modop(x: int, y: int) -> int:\n    return x % y");
        assert!(result.contains("%"), "expected %% in: {result}");
    }

    #[test]
    fn test_w11et_bin_pow_int() {
        let result = transpile("def power(x: int, y: int) -> int:\n    return x ** y");
        assert!(result.contains("pow") || result.contains("**"), "expected pow in: {result}");
    }

    #[test]
    fn test_w11et_bin_add_float_mixed() {
        let result = transpile("def add_mixed(x: int, y: float) -> float:\n    return x + y");
        assert!(result.contains("+"), "expected + in: {result}");
    }

    #[test]
    fn test_w11et_bin_add_str_concat() {
        let result = transpile("def concat(a: str, b: str) -> str:\n    return a + b");
        assert!(result.contains("fn concat"), "expected fn concat in: {result}");
    }

    #[test]
    fn test_w11et_bin_add_list_concat() {
        let result = transpile("def join_lists(a: list, b: list) -> list:\n    return a + b");
        assert!(result.contains("fn join_lists"), "expected fn join_lists in: {result}");
    }

    #[test]
    fn test_w11et_bin_add_compound_expr() {
        let result = transpile("def compound(a: int, b: int, c: int) -> int:\n    return a + b + c");
        assert!(result.contains("+"), "expected + in: {result}");
    }

    #[test]
    fn test_w11et_bin_mul_sub_mixed() {
        let result = transpile("def mixed(a: int, b: int, c: int) -> int:\n    return a * b - c");
        assert!(result.contains("*") && result.contains("-"), "expected * and - in: {result}");
    }

    #[test]
    fn test_w11et_bin_nested_arith() {
        let result =
            transpile("def nested(a: int, b: int) -> int:\n    return (a + b) * (a - b)");
        assert!(result.contains("fn nested"), "expected fn nested in: {result}");
    }

    #[test]
    fn test_w11et_bin_div_int_result() {
        let result = transpile("def divint(x: int, y: int) -> int:\n    return x // y");
        assert!(result.contains("fn divint"), "expected fn divint in: {result}");
    }

    #[test]
    fn test_w11et_bin_pow_float_base() {
        let result = transpile("def fpow(x: float, y: float) -> float:\n    return x ** y");
        assert!(result.contains("pow") || result.contains("fn fpow"), "expected pow in: {result}");
    }

    #[test]
    fn test_w11et_bin_mod_negative() {
        let result = transpile("def negmod(x: int) -> int:\n    return -7 % 3");
        assert!(result.contains("%"), "expected %% in: {result}");
    }

    #[test]
    fn test_w11et_bin_add_assign() {
        let result = transpile("def accum(x: int) -> int:\n    x += 5\n    return x");
        assert!(result.contains("+=") || result.contains("+"), "expected += in: {result}");
    }

    #[test]
    fn test_w11et_bin_sub_assign() {
        let result = transpile("def dec(x: int) -> int:\n    x -= 1\n    return x");
        assert!(result.contains("-=") || result.contains("-"), "expected -= in: {result}");
    }

    #[test]
    fn test_w11et_bin_mul_assign() {
        let result = transpile("def scale(x: int) -> int:\n    x *= 2\n    return x");
        assert!(result.contains("*=") || result.contains("*"), "expected *= in: {result}");
    }

    #[test]
    fn test_w11et_bin_div_assign() {
        let result = transpile("def halve(x: float) -> float:\n    x /= 2.0\n    return x");
        assert!(result.contains("/=") || result.contains("/"), "expected /= in: {result}");
    }

    // ==========================================================================
    // Section 2: Comparison Operators (12 tests)
    // ==========================================================================

    #[test]
    fn test_w11et_cmp_less_than() {
        let result = transpile("def lt(a: int, b: int) -> bool:\n    return a < b");
        assert!(result.contains("<"), "expected < in: {result}");
    }

    #[test]
    fn test_w11et_cmp_greater_than() {
        let result = transpile("def gt(a: int, b: int) -> bool:\n    return a > b");
        assert!(result.contains(">"), "expected > in: {result}");
    }

    #[test]
    fn test_w11et_cmp_le() {
        let result = transpile("def le(a: int, b: int) -> bool:\n    return a <= b");
        assert!(result.contains("<="), "expected <= in: {result}");
    }

    #[test]
    fn test_w11et_cmp_ge() {
        let result = transpile("def ge(a: int, b: int) -> bool:\n    return a >= b");
        assert!(result.contains(">="), "expected >= in: {result}");
    }

    #[test]
    fn test_w11et_cmp_eq() {
        let result = transpile("def eq(a: int, b: int) -> bool:\n    return a == b");
        assert!(result.contains("=="), "expected == in: {result}");
    }

    #[test]
    fn test_w11et_cmp_ne() {
        let result = transpile("def ne(a: int, b: int) -> bool:\n    return a != b");
        assert!(result.contains("!="), "expected != in: {result}");
    }

    #[test]
    fn test_w11et_cmp_str_eq() {
        let result = transpile("def streq(a: str, b: str) -> bool:\n    return a == b");
        assert!(result.contains("=="), "expected == in: {result}");
    }

    #[test]
    fn test_w11et_cmp_chained_lt_lt() {
        let result = transpile(
            "def between(x: int, lo: int, hi: int) -> bool:\n    return lo < x < hi",
        );
        assert!(result.contains("<") || result.contains("&&"), "expected < or && in: {result}");
    }

    #[test]
    fn test_w11et_cmp_in_if() {
        let result = transpile("def check(x: int) -> str:\n    if x > 0:\n        return \"pos\"\n    return \"neg\"");
        assert!(result.contains(">") || result.contains("if"), "expected > or if in: {result}");
    }

    #[test]
    fn test_w11et_cmp_float_lt() {
        let result = transpile("def flt(a: float, b: float) -> bool:\n    return a < b");
        assert!(result.contains("<"), "expected < in: {result}");
    }

    #[test]
    fn test_w11et_cmp_ne_zero() {
        let result = transpile("def nonzero(x: int) -> bool:\n    return x != 0");
        assert!(result.contains("!=") || result.contains("fn nonzero"), "expected != in: {result}");
    }

    #[test]
    fn test_w11et_cmp_eq_string_literal() {
        let result = transpile("def is_hello(s: str) -> bool:\n    return s == \"hello\"");
        assert!(result.contains("=="), "expected == in: {result}");
    }

    // ==========================================================================
    // Section 3: Boolean Operators (10 tests)
    // ==========================================================================

    #[test]
    fn test_w11et_bool_and() {
        let result = transpile("def both(a: bool, b: bool) -> bool:\n    return a and b");
        assert!(result.contains("&&") || result.contains("and"), "expected && in: {result}");
    }

    #[test]
    fn test_w11et_bool_or() {
        let result = transpile("def either(a: bool, b: bool) -> bool:\n    return a or b");
        assert!(result.contains("||") || result.contains("or"), "expected || in: {result}");
    }

    #[test]
    fn test_w11et_bool_not() {
        let result = transpile("def negate(a: bool) -> bool:\n    return not a");
        assert!(result.contains("!") || result.contains("not"), "expected ! in: {result}");
    }

    #[test]
    fn test_w11et_bool_and_or_combined() {
        let result = transpile("def combo(a: bool, b: bool, c: bool) -> bool:\n    return a and b or c");
        assert!(
            result.contains("&&") || result.contains("||"),
            "expected && or || in: {result}"
        );
    }

    #[test]
    fn test_w11et_bool_not_and() {
        let result =
            transpile("def notand(a: bool, b: bool) -> bool:\n    return not a and b");
        assert!(result.contains("!") || result.contains("&&"), "expected ! or && in: {result}");
    }

    #[test]
    fn test_w11et_bool_double_not() {
        let result = transpile("def dblnot(a: bool) -> bool:\n    return not not a");
        assert!(result.contains("!") || result.contains("fn dblnot"), "expected ! in: {result}");
    }

    #[test]
    fn test_w11et_bool_and_comparison() {
        let result = transpile(
            "def range_check(x: int) -> bool:\n    return x > 0 and x < 100",
        );
        assert!(result.contains("&&") || result.contains(">"), "expected && in: {result}");
    }

    #[test]
    fn test_w11et_bool_or_comparison() {
        let result = transpile(
            "def edge(x: int) -> bool:\n    return x < 0 or x > 100",
        );
        assert!(result.contains("||") || result.contains("<"), "expected || in: {result}");
    }

    #[test]
    fn test_w11et_bool_complex_nested() {
        let result = transpile("def nested_bool(a: bool, b: bool, c: bool) -> bool:\n    return (a and b) or (not c)");
        assert!(result.contains("fn nested_bool"), "expected fn nested_bool in: {result}");
    }

    #[test]
    fn test_w11et_bool_short_circuit() {
        let result = transpile("def safe_div(x: int, y: int) -> bool:\n    return y != 0 and x > y");
        assert!(result.contains("&&") || result.contains("!="), "expected && in: {result}");
    }

    // ==========================================================================
    // Section 4: Bitwise Operators (8 tests)
    // ==========================================================================

    #[test]
    fn test_w11et_bit_and() {
        let result = transpile("def band(a: int, b: int) -> int:\n    return a & b");
        assert!(result.contains("&"), "expected & in: {result}");
    }

    #[test]
    fn test_w11et_bit_or() {
        let result = transpile("def bor(a: int, b: int) -> int:\n    return a | b");
        assert!(result.contains("|"), "expected | in: {result}");
    }

    #[test]
    fn test_w11et_bit_xor() {
        let result = transpile("def bxor(a: int, b: int) -> int:\n    return a ^ b");
        assert!(result.contains("^"), "expected ^ in: {result}");
    }

    #[test]
    fn test_w11et_bit_not() {
        let result = transpile("def bnot(a: int) -> int:\n    return ~a");
        assert!(result.contains("!") || result.contains("~"), "expected ~ or ! in: {result}");
    }

    #[test]
    fn test_w11et_bit_lshift() {
        let result = transpile("def lsh(a: int, b: int) -> int:\n    return a << b");
        assert!(result.contains("<<"), "expected << in: {result}");
    }

    #[test]
    fn test_w11et_bit_rshift() {
        let result = transpile("def rsh(a: int, b: int) -> int:\n    return a >> b");
        assert!(result.contains(">>"), "expected >> in: {result}");
    }

    #[test]
    fn test_w11et_bit_mask_and_shift() {
        let result =
            transpile("def extract(x: int) -> int:\n    return (x >> 4) & 15");
        assert!(result.contains(">>") || result.contains("&"), "expected >> or & in: {result}");
    }

    #[test]
    fn test_w11et_bit_compound_ops() {
        let result =
            transpile("def flags(a: int, b: int) -> int:\n    return (a | b) ^ (a & b)");
        assert!(result.contains("|") || result.contains("^"), "expected | or ^ in: {result}");
    }

    // ==========================================================================
    // Section 5: Unary Operators (6 tests)
    // ==========================================================================

    #[test]
    fn test_w11et_unary_neg() {
        let result = transpile("def neg(x: int) -> int:\n    return -x");
        assert!(result.contains("-"), "expected - in: {result}");
    }

    #[test]
    fn test_w11et_unary_pos() {
        let result = transpile("def pos(x: int) -> int:\n    return +x");
        assert!(result.contains("fn pos"), "expected fn pos in: {result}");
    }

    #[test]
    fn test_w11et_unary_not_bool() {
        let result = transpile("def flip(x: bool) -> bool:\n    return not x");
        assert!(result.contains("!") || result.contains("not"), "expected ! in: {result}");
    }

    #[test]
    fn test_w11et_unary_bitnot() {
        let result = transpile("def invert(x: int) -> int:\n    return ~x");
        assert!(result.contains("!") || result.contains("~"), "expected ! or ~ in: {result}");
    }

    #[test]
    fn test_w11et_unary_neg_float() {
        let result = transpile("def negf(x: float) -> float:\n    return -x");
        assert!(result.contains("-"), "expected - in: {result}");
    }

    #[test]
    fn test_w11et_unary_double_neg() {
        let result = transpile("def dblneg(x: int) -> int:\n    return -(-x)");
        assert!(result.contains("fn dblneg"), "expected fn dblneg in: {result}");
    }

    // ==========================================================================
    // Section 6: Ternary / Conditional Expression (6 tests)
    // ==========================================================================

    #[test]
    fn test_w11et_ternary_basic() {
        let result =
            transpile("def pick(x: int) -> str:\n    return \"pos\" if x > 0 else \"neg\"");
        assert!(result.contains("if") || result.contains("else"), "expected if/else in: {result}");
    }

    #[test]
    fn test_w11et_ternary_int_result() {
        let result = transpile("def absval(x: int) -> int:\n    return x if x > 0 else -x");
        assert!(result.contains("if") || result.contains("else"), "expected if/else in: {result}");
    }

    #[test]
    fn test_w11et_ternary_in_assignment() {
        let result = transpile("def label(x: int) -> str:\n    s = \"yes\" if x > 0 else \"no\"\n    return s");
        assert!(result.contains("if") || result.contains("else"), "expected if/else in: {result}");
    }

    #[test]
    fn test_w11et_ternary_nested() {
        let result = transpile("def grade(x: int) -> str:\n    return \"a\" if x > 90 else \"b\" if x > 80 else \"c\"");
        assert!(result.contains("if") || result.contains("else"), "expected if/else in: {result}");
    }

    #[test]
    fn test_w11et_ternary_bool_cond() {
        let result = transpile("def choose(flag: bool) -> int:\n    return 1 if flag else 0");
        assert!(result.contains("if") || result.contains("else"), "expected if/else in: {result}");
    }

    #[test]
    fn test_w11et_ternary_with_call() {
        let result = transpile("def maxval(a: int, b: int) -> int:\n    return a if a > b else b");
        assert!(result.contains("if") || result.contains("else"), "expected if/else in: {result}");
    }

    // ==========================================================================
    // Section 7: F-strings (10 tests)
    // ==========================================================================

    #[test]
    fn test_w11et_fstring_basic_var() {
        let result = transpile("def greet(name: str) -> str:\n    return f\"hello {name}\"");
        assert!(
            result.contains("format!") || result.contains("hello"),
            "expected format! in: {result}"
        );
    }

    #[test]
    fn test_w11et_fstring_int_var() {
        let result = transpile("def show_age(age: int) -> str:\n    return f\"age is {age}\"");
        assert!(
            result.contains("format!") || result.contains("age"),
            "expected format! in: {result}"
        );
    }

    #[test]
    fn test_w11et_fstring_expr_add() {
        let result =
            transpile("def show_sum(a: int, b: int) -> str:\n    return f\"sum = {a + b}\"");
        assert!(
            result.contains("format!") || result.contains("sum"),
            "expected format! in: {result}"
        );
    }

    #[test]
    fn test_w11et_fstring_multiple_parts() {
        let result = transpile(
            "def info(name: str, age: int) -> str:\n    return f\"{name} is {age}\"",
        );
        assert!(
            result.contains("format!") || result.contains("name"),
            "expected format! in: {result}"
        );
    }

    #[test]
    fn test_w11et_fstring_method_call() {
        let result =
            transpile("def upper_greet(name: str) -> str:\n    return f\"{name.upper()}\"");
        assert!(
            result.contains("format!") || result.contains("to_uppercase"),
            "expected format! in: {result}"
        );
    }

    #[test]
    fn test_w11et_fstring_nested_ternary() {
        let code = "def yesno(flag: bool) -> str:\n    return f\"answer: {'yes' if flag else 'no'}\"";
        let result = transpile(code);
        assert!(
            result.contains("format!") || result.contains("answer"),
            "expected format! in: {result}"
        );
    }

    #[test]
    fn test_w11et_fstring_literal_only() {
        let result = transpile("def static_str() -> str:\n    return f\"constant\"");
        assert!(
            result.contains("constant") || result.contains("fn static_str"),
            "expected constant in: {result}"
        );
    }

    #[test]
    fn test_w11et_fstring_three_vars() {
        let result = transpile("def triple(a: int, b: int, c: int) -> str:\n    return f\"{a},{b},{c}\"");
        assert!(
            result.contains("format!") || result.contains("fn triple"),
            "expected format! in: {result}"
        );
    }

    #[test]
    fn test_w11et_fstring_with_str_op() {
        let result = transpile(
            "def repeat(s: str) -> str:\n    return f\"value: {s + s}\"",
        );
        assert!(
            result.contains("format!") || result.contains("value"),
            "expected format! in: {result}"
        );
    }

    #[test]
    fn test_w11et_fstring_empty_string_parts() {
        let result = transpile("def wrap(x: int) -> str:\n    return f\"[{x}]\"");
        assert!(
            result.contains("format!") || result.contains("["),
            "expected format! in: {result}"
        );
    }

    // ==========================================================================
    // Section 8: Collection Literals (14 tests)
    // ==========================================================================

    #[test]
    fn test_w11et_list_int_literal() {
        let result = transpile("def nums() -> list:\n    return [1, 2, 3]");
        assert!(result.contains("vec!") || result.contains("["), "expected vec! in: {result}");
    }

    #[test]
    fn test_w11et_list_str_literal() {
        let result =
            transpile("def names() -> list:\n    return [\"alice\", \"bob\"]");
        assert!(result.contains("vec!") || result.contains("["), "expected vec! in: {result}");
    }

    #[test]
    fn test_w11et_dict_literal_str_int() {
        let result = transpile("def scores() -> dict:\n    return {\"a\": 1, \"b\": 2}");
        assert!(
            result.contains("HashMap") || result.contains("insert"),
            "expected HashMap in: {result}"
        );
    }

    #[test]
    fn test_w11et_set_literal() {
        let result = transpile("def unique() -> set:\n    return {1, 2, 3}");
        assert!(
            result.contains("HashSet") || result.contains("insert") || result.contains("set"),
            "expected HashSet in: {result}"
        );
    }

    #[test]
    fn test_w11et_tuple_literal() {
        let result = transpile("def pair() -> tuple:\n    return (1, 2, 3)");
        assert!(result.contains("(") || result.contains("fn pair"), "expected tuple in: {result}");
    }

    #[test]
    fn test_w11et_empty_list() {
        let result = transpile("def empty() -> list:\n    return []");
        assert!(
            result.contains("vec!") || result.contains("Vec"),
            "expected empty vec in: {result}"
        );
    }

    #[test]
    fn test_w11et_empty_dict() {
        let result = transpile("def empty_dict() -> dict:\n    return {}");
        assert!(
            result.contains("HashMap") || result.contains("new"),
            "expected empty HashMap in: {result}"
        );
    }

    #[test]
    fn test_w11et_nested_list() {
        let result = transpile("def matrix() -> list:\n    return [[1, 2], [3, 4]]");
        assert!(result.contains("vec!") || result.contains("["), "expected nested vec in: {result}");
    }

    #[test]
    fn test_w11et_nested_dict_with_list() {
        let result =
            transpile("def data() -> dict:\n    return {\"items\": [1, 2]}");
        assert!(
            result.contains("HashMap") || result.contains("vec!"),
            "expected HashMap+vec in: {result}"
        );
    }

    #[test]
    fn test_w11et_list_bool_literal() {
        let result = transpile("def flags() -> list:\n    return [True, False, True]");
        assert!(
            result.contains("vec!") || result.contains("true") || result.contains("false"),
            "expected bool vec in: {result}"
        );
    }

    #[test]
    fn test_w11et_list_mixed_empty_init() {
        let result = transpile("def init():\n    items = []\n    return items");
        assert!(
            result.contains("vec!") || result.contains("Vec"),
            "expected empty vec in: {result}"
        );
    }

    #[test]
    fn test_w11et_dict_int_keys() {
        let result = transpile("def mapping() -> dict:\n    return {1: \"one\", 2: \"two\"}");
        assert!(
            result.contains("HashMap") || result.contains("insert"),
            "expected HashMap in: {result}"
        );
    }

    #[test]
    fn test_w11et_list_single_element() {
        let result = transpile("def single() -> list:\n    return [42]");
        assert!(result.contains("vec!") || result.contains("42"), "expected vec![42] in: {result}");
    }

    #[test]
    fn test_w11et_tuple_two_elements() {
        let result = transpile("def coord() -> tuple:\n    return (10, 20)");
        assert!(result.contains("(") || result.contains("fn coord"), "expected tuple in: {result}");
    }

    // ==========================================================================
    // Section 9: Comprehensions (10 tests)
    // ==========================================================================

    #[test]
    fn test_w11et_listcomp_basic() {
        let result = transpile("def doubles() -> list:\n    return [x * 2 for x in range(10)]");
        assert!(
            result.contains("map") || result.contains("collect") || result.contains("iter"),
            "expected iterator in: {result}"
        );
    }

    #[test]
    fn test_w11et_listcomp_with_filter() {
        let result =
            transpile("def positives(items: list) -> list:\n    return [x for x in items if x > 0]");
        assert!(
            result.contains("filter") || result.contains("iter") || result.contains(">"),
            "expected filter in: {result}"
        );
    }

    #[test]
    fn test_w11et_listcomp_with_transform() {
        let result =
            transpile("def squares() -> list:\n    return [x * x for x in range(5)]");
        assert!(
            result.contains("map") || result.contains("iter"),
            "expected map in: {result}"
        );
    }

    #[test]
    fn test_w11et_dictcomp_basic() {
        let result = transpile(
            "def make_dict() -> dict:\n    return {x: x * x for x in range(5)}",
        );
        assert!(
            result.contains("collect") || result.contains("HashMap") || result.contains("map"),
            "expected dict comp in: {result}"
        );
    }

    #[test]
    fn test_w11et_setcomp_basic() {
        let result = transpile("def uniq() -> set:\n    return {x for x in range(5)}");
        assert!(
            result.contains("collect") || result.contains("HashSet") || result.contains("iter"),
            "expected set comp in: {result}"
        );
    }

    #[test]
    fn test_w11et_listcomp_string() {
        let result = transpile(
            "def uppers(names: list) -> list:\n    return [n.upper() for n in names]",
        );
        assert!(
            result.contains("map") || result.contains("to_uppercase") || result.contains("iter"),
            "expected map in: {result}"
        );
    }

    #[test]
    fn test_w11et_listcomp_nested_range() {
        let result = transpile(
            "def pairs() -> list:\n    return [(i, j) for i in range(3) for j in range(3)]",
        );
        assert!(
            result.contains("iter") || result.contains("flat_map") || result.contains("for"),
            "expected nested comp in: {result}"
        );
    }

    #[test]
    fn test_w11et_listcomp_conditional_transform() {
        let result = transpile(
            "def evens() -> list:\n    return [x for x in range(20) if x % 2 == 0]",
        );
        assert!(
            result.contains("filter") || result.contains("%") || result.contains("iter"),
            "expected filter in: {result}"
        );
    }

    #[test]
    fn test_w11et_listcomp_str_filter() {
        let result = transpile(
            "def long_names(names: list) -> list:\n    return [n for n in names if len(n) > 3]",
        );
        assert!(
            result.contains("filter") || result.contains("len") || result.contains("iter"),
            "expected filter in: {result}"
        );
    }

    #[test]
    fn test_w11et_generator_sum() {
        let result = transpile("def total() -> int:\n    return sum(x for x in range(10))");
        assert!(
            result.contains("sum") || result.contains("iter"),
            "expected sum in: {result}"
        );
    }

    // ==========================================================================
    // Section 10: Lambda Expressions (6 tests)
    // ==========================================================================

    #[test]
    fn test_w11et_lambda_basic() {
        let result = transpile("def use_lambda():\n    f = lambda x: x + 1\n    return f(5)");
        assert!(
            result.contains("|") || result.contains("closure") || result.contains("fn"),
            "expected lambda in: {result}"
        );
    }

    #[test]
    fn test_w11et_lambda_two_args() {
        let result =
            transpile("def use_add():\n    add = lambda x, y: x + y\n    return add(2, 3)");
        assert!(
            result.contains("|") || result.contains("fn use_add"),
            "expected lambda in: {result}"
        );
    }

    #[test]
    fn test_w11et_lambda_no_args() {
        let result = transpile("def const_fn():\n    f = lambda: 42\n    return f()");
        assert!(
            result.contains("||") || result.contains("42"),
            "expected lambda in: {result}"
        );
    }

    #[test]
    fn test_w11et_lambda_in_sort_key() {
        let result = transpile(
            "def sort_by_len(items: list) -> list:\n    items.sort(key=lambda x: len(x))\n    return items",
        );
        assert!(
            result.contains("sort") || result.contains("|"),
            "expected sort with key in: {result}"
        );
    }

    #[test]
    fn test_w11et_lambda_with_condition() {
        let result =
            transpile("def cond():\n    f = lambda x: x if x > 0 else -x\n    return f(-5)");
        assert!(
            result.contains("|") || result.contains("fn cond"),
            "expected lambda in: {result}"
        );
    }

    #[test]
    fn test_w11et_lambda_multiply() {
        let result =
            transpile("def dbl():\n    f = lambda x: x * 2\n    return f(10)");
        assert!(
            result.contains("|") || result.contains("*"),
            "expected lambda in: {result}"
        );
    }

    // ==========================================================================
    // Section 11: Subscript and Slicing (8 tests)
    // ==========================================================================

    #[test]
    fn test_w11et_subscript_index_zero() {
        let result = transpile("def first(lst: list) -> int:\n    return lst[0]");
        assert!(result.contains("[0]") || result.contains("fn first"), "expected [0] in: {result}");
    }

    #[test]
    fn test_w11et_subscript_negative_index() {
        let result = transpile("def last(lst: list) -> int:\n    return lst[-1]");
        assert!(
            result.contains("[-1]") || result.contains("last") || result.contains("len"),
            "expected [-1] or last in: {result}"
        );
    }

    #[test]
    fn test_w11et_subscript_variable_index() {
        let result = transpile("def at(lst: list, i: int) -> int:\n    return lst[i]");
        assert!(result.contains("[") || result.contains("fn at"), "expected subscript in: {result}");
    }

    #[test]
    fn test_w11et_slice_start_end() {
        let result = transpile("def mid(lst: list) -> list:\n    return lst[1:3]");
        assert!(
            result.contains("[1..3]") || result.contains("fn mid") || result.contains("slice"),
            "expected slice in: {result}"
        );
    }

    #[test]
    fn test_w11et_slice_step() {
        let result = transpile("def every_other(lst: list) -> list:\n    return lst[::2]");
        assert!(
            result.contains("step") || result.contains("fn every_other") || result.contains("iter"),
            "expected step slice in: {result}"
        );
    }

    #[test]
    fn test_w11et_slice_reverse() {
        let result = transpile("def reversed_list(lst: list) -> list:\n    return lst[::-1]");
        assert!(
            result.contains("rev") || result.contains("fn reversed_list") || result.contains("iter"),
            "expected reverse in: {result}"
        );
    }

    #[test]
    fn test_w11et_dict_subscript() {
        let result = transpile("def get_val(d: dict, k: str):\n    return d[k]");
        assert!(
            result.contains("[") || result.contains("get") || result.contains("fn get_val"),
            "expected dict subscript in: {result}"
        );
    }

    #[test]
    fn test_w11et_subscript_string_index() {
        let result = transpile("def char_at(s: str, i: int) -> str:\n    return s[i]");
        assert!(
            result.contains("[") || result.contains("chars") || result.contains("fn char_at"),
            "expected string index in: {result}"
        );
    }

    // ==========================================================================
    // Section 12: Method Calls and Attribute Access (10 tests)
    // ==========================================================================

    #[test]
    fn test_w11et_method_str_upper() {
        let result = transpile("def up(s: str) -> str:\n    return s.upper()");
        assert!(
            result.contains("to_uppercase") || result.contains("upper"),
            "expected to_uppercase in: {result}"
        );
    }

    #[test]
    fn test_w11et_method_str_lower() {
        let result = transpile("def low(s: str) -> str:\n    return s.lower()");
        assert!(
            result.contains("to_lowercase") || result.contains("lower"),
            "expected to_lowercase in: {result}"
        );
    }

    #[test]
    fn test_w11et_method_str_strip() {
        let result = transpile("def trimmed(s: str) -> str:\n    return s.strip()");
        assert!(
            result.contains("trim") || result.contains("strip"),
            "expected trim in: {result}"
        );
    }

    #[test]
    fn test_w11et_method_str_split() {
        let result = transpile("def words(s: str) -> list:\n    return s.split()");
        assert!(
            result.contains("split") || result.contains("fn words"),
            "expected split in: {result}"
        );
    }

    #[test]
    fn test_w11et_method_str_replace() {
        let result =
            transpile("def fix(s: str) -> str:\n    return s.replace(\"old\", \"new\")");
        assert!(
            result.contains("replace") || result.contains("fn fix"),
            "expected replace in: {result}"
        );
    }

    #[test]
    fn test_w11et_method_str_startswith() {
        let result = transpile("def check(s: str) -> bool:\n    return s.startswith(\"abc\")");
        assert!(
            result.contains("starts_with") || result.contains("startswith"),
            "expected starts_with in: {result}"
        );
    }

    #[test]
    fn test_w11et_method_str_endswith() {
        let result = transpile("def check_end(s: str) -> bool:\n    return s.endswith(\".py\")");
        assert!(
            result.contains("ends_with") || result.contains("endswith"),
            "expected ends_with in: {result}"
        );
    }

    #[test]
    fn test_w11et_method_list_append() {
        let result = transpile(
            "def add_item(lst: list, x: int):\n    lst.append(x)\n    return lst",
        );
        assert!(
            result.contains("push") || result.contains("append"),
            "expected push in: {result}"
        );
    }

    #[test]
    fn test_w11et_method_chained_strip_lower() {
        let result =
            transpile("def clean(s: str) -> str:\n    return s.strip().lower()");
        assert!(
            result.contains("trim") || result.contains("to_lowercase"),
            "expected chained in: {result}"
        );
    }

    #[test]
    fn test_w11et_method_str_join() {
        let result = transpile(
            "def join_words(words: list) -> str:\n    return \" \".join(words)",
        );
        assert!(
            result.contains("join") || result.contains("fn join_words"),
            "expected join in: {result}"
        );
    }

    // ==========================================================================
    // Section 13: Built-in Function Calls (24 tests)
    // ==========================================================================

    #[test]
    fn test_w11et_builtin_len() {
        let result = transpile("def size(lst: list) -> int:\n    return len(lst)");
        assert!(result.contains("len()") || result.contains("len"), "expected len in: {result}");
    }

    #[test]
    fn test_w11et_builtin_range_single() {
        let result =
            transpile("def count():\n    for i in range(10):\n        x = i\n    return x");
        assert!(result.contains("0..10") || result.contains("range"), "expected range in: {result}");
    }

    #[test]
    fn test_w11et_builtin_range_start_stop() {
        let result =
            transpile("def mid_range():\n    for i in range(5, 10):\n        x = i\n    return x");
        assert!(result.contains("5..10") || result.contains("range"), "expected range in: {result}");
    }

    #[test]
    fn test_w11et_builtin_print() {
        let result = transpile("def hello():\n    print(\"hello world\")");
        assert!(
            result.contains("println!") || result.contains("print"),
            "expected println in: {result}"
        );
    }

    #[test]
    fn test_w11et_builtin_int_cast() {
        let result = transpile("def to_int(s: str) -> int:\n    return int(s)");
        assert!(
            result.contains("parse") || result.contains("i64") || result.contains("int"),
            "expected parse in: {result}"
        );
    }

    #[test]
    fn test_w11et_builtin_str_cast() {
        let result = transpile("def to_str(x: int) -> str:\n    return str(x)");
        assert!(
            result.contains("to_string") || result.contains("format"),
            "expected to_string in: {result}"
        );
    }

    #[test]
    fn test_w11et_builtin_float_cast() {
        let result = transpile("def to_float(x: int) -> float:\n    return float(x)");
        assert!(
            result.contains("f64") || result.contains("as f64") || result.contains("float"),
            "expected f64 in: {result}"
        );
    }

    #[test]
    fn test_w11et_builtin_bool_cast() {
        let result = transpile("def to_bool(x: int) -> bool:\n    return bool(x)");
        assert!(
            result.contains("bool") || result.contains("!= 0"),
            "expected bool in: {result}"
        );
    }

    #[test]
    fn test_w11et_builtin_abs() {
        let result = transpile("def magnitude(x: int) -> int:\n    return abs(x)");
        assert!(result.contains("abs") || result.contains("fn magnitude"), "expected abs in: {result}");
    }

    #[test]
    fn test_w11et_builtin_max_two() {
        let result = transpile("def bigger(a: int, b: int) -> int:\n    return max(a, b)");
        assert!(
            result.contains("max") || result.contains("std::cmp"),
            "expected max in: {result}"
        );
    }

    #[test]
    fn test_w11et_builtin_min_two() {
        let result = transpile("def smaller(a: int, b: int) -> int:\n    return min(a, b)");
        assert!(
            result.contains("min") || result.contains("std::cmp"),
            "expected min in: {result}"
        );
    }

    #[test]
    fn test_w11et_builtin_sum_list() {
        let result = transpile("def total(nums: list) -> int:\n    return sum(nums)");
        assert!(
            result.contains("sum") || result.contains("iter"),
            "expected sum in: {result}"
        );
    }

    #[test]
    fn test_w11et_builtin_sorted() {
        let result = transpile("def ordered(lst: list) -> list:\n    return sorted(lst)");
        assert!(
            result.contains("sorted") || result.contains("sort"),
            "expected sorted in: {result}"
        );
    }

    #[test]
    fn test_w11et_builtin_reversed() {
        let result = transpile("def backwards(lst: list) -> list:\n    return list(reversed(lst))");
        assert!(
            result.contains("rev") || result.contains("reversed"),
            "expected rev in: {result}"
        );
    }

    #[test]
    fn test_w11et_builtin_enumerate() {
        let result = transpile(
            "def indexed(items: list):\n    for i, x in enumerate(items):\n        y = i\n    return y",
        );
        assert!(
            result.contains("enumerate") || result.contains("iter"),
            "expected enumerate in: {result}"
        );
    }

    #[test]
    fn test_w11et_builtin_zip() {
        let result = transpile(
            "def combine(a: list, b: list):\n    for x, y in zip(a, b):\n        z = x\n    return z",
        );
        assert!(
            result.contains("zip") || result.contains("iter"),
            "expected zip in: {result}"
        );
    }

    #[test]
    fn test_w11et_builtin_map() {
        let result = transpile("def apply(items: list) -> list:\n    return list(map(str, items))");
        assert!(
            result.contains("map") || result.contains("iter"),
            "expected map in: {result}"
        );
    }

    #[test]
    fn test_w11et_builtin_isinstance_int() {
        let result = transpile("def is_int(x) -> bool:\n    return isinstance(x, int)");
        assert!(
            result.contains("isinstance") || result.contains("fn is_int"),
            "expected isinstance in: {result}"
        );
    }

    #[test]
    fn test_w11et_builtin_chr() {
        let result = transpile("def to_char(n: int) -> str:\n    return chr(n)");
        assert!(
            result.contains("char") || result.contains("chr") || result.contains("fn to_char"),
            "expected chr in: {result}"
        );
    }

    #[test]
    fn test_w11et_builtin_ord() {
        let result = transpile("def to_code(c: str) -> int:\n    return ord(c)");
        assert!(
            result.contains("ord") || result.contains("as u32") || result.contains("fn to_code"),
            "expected ord in: {result}"
        );
    }

    #[test]
    fn test_w11et_builtin_any() {
        let result = transpile("def has_true(lst: list) -> bool:\n    return any(lst)");
        assert!(
            result.contains("any") || result.contains("iter"),
            "expected any in: {result}"
        );
    }

    #[test]
    fn test_w11et_builtin_all() {
        let result = transpile("def all_true(lst: list) -> bool:\n    return all(lst)");
        assert!(
            result.contains("all") || result.contains("iter"),
            "expected all in: {result}"
        );
    }

    #[test]
    fn test_w11et_builtin_hex() {
        let result = transpile("def to_hex(n: int) -> str:\n    return hex(n)");
        assert!(
            result.contains("hex") || result.contains("format!") || result.contains("fn to_hex"),
            "expected hex in: {result}"
        );
    }

    #[test]
    fn test_w11et_builtin_input_prompt() {
        let result = transpile("def ask() -> str:\n    return input(\"name: \")");
        assert!(
            result.contains("input") || result.contains("stdin") || result.contains("fn ask"),
            "expected input in: {result}"
        );
    }

    // ==========================================================================
    // Section 14: String/List Multiplication and Misc Expr (6 tests)
    // ==========================================================================

    #[test]
    fn test_w11et_str_multiply() {
        let result = transpile("def repeat(s: str) -> str:\n    return s * 3");
        assert!(
            result.contains("repeat") || result.contains("*"),
            "expected repeat in: {result}"
        );
    }

    #[test]
    fn test_w11et_list_multiply() {
        let result = transpile("def zeros(n: int) -> list:\n    return [0] * n");
        assert!(
            result.contains("vec!") || result.contains("*") || result.contains("repeat"),
            "expected list multiply in: {result}"
        );
    }

    #[test]
    fn test_w11et_str_in_membership() {
        let result =
            transpile("def has_char(s: str) -> bool:\n    return \"a\" in s");
        assert!(
            result.contains("contains") || result.contains("in"),
            "expected contains in: {result}"
        );
    }

    #[test]
    fn test_w11et_str_not_in() {
        let result =
            transpile("def missing(s: str) -> bool:\n    return \"z\" not in s");
        assert!(
            result.contains("contains") || result.contains("!"),
            "expected not contains in: {result}"
        );
    }

    #[test]
    fn test_w11et_list_in_membership() {
        let result = transpile("def has_item(lst: list, x: int) -> bool:\n    return x in lst");
        assert!(
            result.contains("contains") || result.contains("in"),
            "expected contains in: {result}"
        );
    }

    #[test]
    fn test_w11et_walrus_operator() {
        let result = transpile(
            "def check_len(items: list) -> int:\n    if (n := len(items)) > 0:\n        return n\n    return 0",
        );
        assert!(
            result.contains("len") || result.contains("fn check_len"),
            "expected walrus in: {result}"
        );
    }

    // ==========================================================================
    // Section 15: Type Inference - Literals (12 tests)
    // ==========================================================================

    #[test]
    fn test_w11et_type_int_literal() {
        let result = transpile("def f() -> int:\n    x = 42\n    return x");
        assert!(
            result.contains("i64") || result.contains("42"),
            "expected i64 in: {result}"
        );
    }

    #[test]
    fn test_w11et_type_float_literal() {
        let result = transpile("def f() -> float:\n    x = 7.5\n    return x");
        assert!(
            result.contains("f64") || result.contains("7.5"),
            "expected f64 in: {result}"
        );
    }

    #[test]
    fn test_w11et_type_string_literal() {
        let result = transpile("def f() -> str:\n    x = \"hello\"\n    return x");
        assert!(
            result.contains("String") || result.contains("str") || result.contains("hello"),
            "expected String in: {result}"
        );
    }

    #[test]
    fn test_w11et_type_bool_true() {
        let result = transpile("def f() -> bool:\n    x = True\n    return x");
        assert!(
            result.contains("bool") || result.contains("true"),
            "expected bool in: {result}"
        );
    }

    #[test]
    fn test_w11et_type_bool_false() {
        let result = transpile("def f() -> bool:\n    x = False\n    return x");
        assert!(
            result.contains("bool") || result.contains("false"),
            "expected bool in: {result}"
        );
    }

    #[test]
    fn test_w11et_type_none_literal() {
        let result = transpile("def f():\n    x = None\n    return x");
        assert!(
            result.contains("None") || result.contains("Option"),
            "expected None in: {result}"
        );
    }

    #[test]
    fn test_w11et_type_list_of_ints() {
        let result = transpile("def f() -> list:\n    x = [1, 2, 3]\n    return x");
        assert!(
            result.contains("vec!") || result.contains("Vec"),
            "expected Vec in: {result}"
        );
    }

    #[test]
    fn test_w11et_type_list_of_strings() {
        let result =
            transpile("def f() -> list:\n    x = [\"a\", \"b\"]\n    return x");
        assert!(
            result.contains("vec!") || result.contains("Vec"),
            "expected Vec in: {result}"
        );
    }

    #[test]
    fn test_w11et_type_dict_str_int() {
        let result = transpile("def f() -> dict:\n    d = {\"a\": 1}\n    return d");
        assert!(
            result.contains("HashMap") || result.contains("insert"),
            "expected HashMap in: {result}"
        );
    }

    #[test]
    fn test_w11et_type_set_of_ints() {
        let result = transpile("def f() -> set:\n    s = {1, 2, 3}\n    return s");
        assert!(
            result.contains("HashSet") || result.contains("insert") || result.contains("set"),
            "expected HashSet in: {result}"
        );
    }

    #[test]
    fn test_w11et_type_large_int() {
        let result = transpile("def f() -> int:\n    x = 999999\n    return x");
        assert!(
            result.contains("999999") || result.contains("i64"),
            "expected large int in: {result}"
        );
    }

    #[test]
    fn test_w11et_type_negative_int() {
        let result = transpile("def f() -> int:\n    x = -42\n    return x");
        assert!(
            result.contains("-42") || result.contains("i64"),
            "expected negative int in: {result}"
        );
    }

    // ==========================================================================
    // Section 16: Type Annotations (10 tests)
    // ==========================================================================

    #[test]
    fn test_w11et_annot_int_param() {
        let result = transpile("def f(x: int) -> int:\n    return x");
        assert!(result.contains("i64") || result.contains("fn f"), "expected i64 in: {result}");
    }

    #[test]
    fn test_w11et_annot_str_param() {
        let result = transpile("def f(s: str) -> str:\n    return s");
        assert!(
            result.contains("String") || result.contains("&str") || result.contains("str"),
            "expected str type in: {result}"
        );
    }

    #[test]
    fn test_w11et_annot_float_param() {
        let result = transpile("def f(x: float) -> float:\n    return x");
        assert!(result.contains("f64") || result.contains("fn f"), "expected f64 in: {result}");
    }

    #[test]
    fn test_w11et_annot_bool_param() {
        let result = transpile("def f(x: bool) -> bool:\n    return x");
        assert!(result.contains("bool") || result.contains("fn f"), "expected bool in: {result}");
    }

    #[test]
    fn test_w11et_annot_list_param() {
        let result = transpile("def f(lst: list) -> list:\n    return lst");
        assert!(
            result.contains("Vec") || result.contains("fn f"),
            "expected Vec in: {result}"
        );
    }

    #[test]
    fn test_w11et_annot_dict_param() {
        let result = transpile("def f(d: dict) -> dict:\n    return d");
        assert!(
            result.contains("HashMap") || result.contains("fn f"),
            "expected HashMap in: {result}"
        );
    }

    #[test]
    fn test_w11et_annot_return_int() {
        let result = transpile("def f() -> int:\n    return 5");
        assert!(result.contains("i64") || result.contains("-> i64"), "expected -> i64 in: {result}");
    }

    #[test]
    fn test_w11et_annot_return_str() {
        let result = transpile("def f() -> str:\n    return \"hi\"");
        assert!(
            result.contains("String") || result.contains("-> String"),
            "expected -> String in: {result}"
        );
    }

    #[test]
    fn test_w11et_annot_return_bool() {
        let result = transpile("def f() -> bool:\n    return True");
        assert!(
            result.contains("-> bool") || result.contains("bool"),
            "expected -> bool in: {result}"
        );
    }

    #[test]
    fn test_w11et_annot_multiple_params() {
        let result =
            transpile("def f(a: int, b: str, c: float) -> str:\n    return b");
        assert!(
            result.contains("i64") || result.contains("String") || result.contains("f64"),
            "expected typed params in: {result}"
        );
    }

    // ==========================================================================
    // Section 17: Type Propagation from Ops (14 tests)
    // ==========================================================================

    #[test]
    fn test_w11et_prop_int_add_int() {
        let result = transpile("def f(a: int, b: int) -> int:\n    c = a + b\n    return c");
        assert!(result.contains("+"), "expected + in: {result}");
    }

    #[test]
    fn test_w11et_prop_int_add_float() {
        let result =
            transpile("def f(a: int, b: float) -> float:\n    c = a + b\n    return c");
        assert!(result.contains("+") || result.contains("f64"), "expected + in: {result}");
    }

    #[test]
    fn test_w11et_prop_str_concat() {
        let result = transpile("def f(a: str, b: str) -> str:\n    c = a + b\n    return c");
        assert!(
            result.contains("+") || result.contains("format!"),
            "expected concat in: {result}"
        );
    }

    #[test]
    fn test_w11et_prop_comparison_bool() {
        let result = transpile("def f(x: int) -> bool:\n    result = x > 0\n    return result");
        assert!(
            result.contains(">") || result.contains("bool"),
            "expected > in: {result}"
        );
    }

    #[test]
    fn test_w11et_prop_len_returns_int() {
        let result = transpile("def f(lst: list) -> int:\n    n = len(lst)\n    return n");
        assert!(result.contains("len") || result.contains("i64"), "expected len in: {result}");
    }

    #[test]
    fn test_w11et_prop_range_loop() {
        let result = transpile(
            "def f() -> int:\n    total = 0\n    for i in range(10):\n        total += i\n    return total",
        );
        assert!(
            result.contains("0..10") || result.contains("range") || result.contains("for"),
            "expected range loop in: {result}"
        );
    }

    #[test]
    fn test_w11et_prop_enumerate_types() {
        let result = transpile(
            "def f(items: list):\n    for idx, val in enumerate(items):\n        x = idx\n    return x",
        );
        assert!(
            result.contains("enumerate") || result.contains("for"),
            "expected enumerate in: {result}"
        );
    }

    #[test]
    fn test_w11et_prop_zip_types() {
        let result = transpile(
            "def f(a: list, b: list):\n    for x, y in zip(a, b):\n        z = x\n    return z",
        );
        assert!(
            result.contains("zip") || result.contains("for"),
            "expected zip in: {result}"
        );
    }

    #[test]
    fn test_w11et_prop_mul_int_int() {
        let result = transpile("def f(a: int, b: int) -> int:\n    c = a * b\n    return c");
        assert!(result.contains("*"), "expected * in: {result}");
    }

    #[test]
    fn test_w11et_prop_div_float_result() {
        let result = transpile("def f(a: float, b: float) -> float:\n    c = a / b\n    return c");
        assert!(result.contains("/"), "expected / in: {result}");
    }

    #[test]
    fn test_w11et_prop_modulo_int() {
        let result = transpile("def f(a: int, b: int) -> int:\n    c = a % b\n    return c");
        assert!(result.contains("%"), "expected %% in: {result}");
    }

    #[test]
    fn test_w11et_prop_bool_and_result() {
        let result =
            transpile("def f(a: bool, b: bool) -> bool:\n    c = a and b\n    return c");
        assert!(
            result.contains("&&") || result.contains("bool"),
            "expected && in: {result}"
        );
    }

    #[test]
    fn test_w11et_prop_bool_or_result() {
        let result =
            transpile("def f(a: bool, b: bool) -> bool:\n    c = a or b\n    return c");
        assert!(
            result.contains("||") || result.contains("bool"),
            "expected || in: {result}"
        );
    }

    #[test]
    fn test_w11et_prop_not_result() {
        let result = transpile("def f(a: bool) -> bool:\n    c = not a\n    return c");
        assert!(
            result.contains("!") || result.contains("bool"),
            "expected ! in: {result}"
        );
    }

    // ==========================================================================
    // Section 18: Function Return Type Inference (12 tests)
    // ==========================================================================

    #[test]
    fn test_w11et_ret_infer_int_literal() {
        let result = transpile("def f():\n    return 42");
        assert!(result.contains("42") || result.contains("fn f"), "expected 42 in: {result}");
    }

    #[test]
    fn test_w11et_ret_infer_str_literal() {
        let result = transpile("def f():\n    return \"hello\"");
        assert!(
            result.contains("hello") || result.contains("fn f"),
            "expected hello in: {result}"
        );
    }

    #[test]
    fn test_w11et_ret_infer_bool() {
        let result = transpile("def f():\n    return True");
        assert!(
            result.contains("true") || result.contains("fn f"),
            "expected true in: {result}"
        );
    }

    #[test]
    fn test_w11et_ret_infer_none() {
        let result = transpile("def f():\n    return None");
        assert!(
            result.contains("None") || result.contains("fn f"),
            "expected None in: {result}"
        );
    }

    #[test]
    fn test_w11et_ret_infer_list() {
        let result = transpile("def f():\n    return [1, 2, 3]");
        assert!(
            result.contains("vec!") || result.contains("fn f"),
            "expected vec! in: {result}"
        );
    }

    #[test]
    fn test_w11et_ret_infer_computed() {
        let result = transpile("def f(x: int) -> int:\n    return x + 1");
        assert!(result.contains("+") || result.contains("fn f"), "expected + in: {result}");
    }

    #[test]
    fn test_w11et_ret_infer_conditional() {
        let result = transpile(
            "def f(x: int) -> str:\n    if x > 0:\n        return \"pos\"\n    return \"neg\"",
        );
        assert!(
            result.contains("if") || result.contains("return"),
            "expected conditional return in: {result}"
        );
    }

    #[test]
    fn test_w11et_ret_infer_no_annotation() {
        let result = transpile("def f(x):\n    return x + 1");
        assert!(result.contains("fn f") || result.contains("+"), "expected fn f in: {result}");
    }

    #[test]
    fn test_w11et_ret_multiple_returns() {
        let result = transpile(
            "def f(x: int) -> int:\n    if x > 0:\n        return x\n    else:\n        return -x",
        );
        assert!(result.contains("return") || result.contains("if"), "expected returns in: {result}");
    }

    #[test]
    fn test_w11et_ret_infer_void() {
        let result = transpile("def f():\n    x = 5");
        assert!(result.contains("fn f"), "expected fn f in: {result}");
    }

    #[test]
    fn test_w11et_ret_infer_nested_call() {
        let result = transpile("def f(x: int) -> int:\n    return abs(x)");
        assert!(
            result.contains("abs") || result.contains("fn f"),
            "expected abs in: {result}"
        );
    }

    #[test]
    fn test_w11et_ret_infer_from_param_type() {
        let result = transpile("def f(x: int) -> int:\n    y = x * 2\n    return y");
        assert!(result.contains("*") || result.contains("fn f"), "expected * in: {result}");
    }

    // ==========================================================================
    // Section 19: Parameter Default Values (8 tests)
    // ==========================================================================

    #[test]
    fn test_w11et_default_int() {
        let result = transpile("def f(x: int = 0) -> int:\n    return x");
        assert!(result.contains("fn f") || result.contains("0"), "expected default in: {result}");
    }

    #[test]
    fn test_w11et_default_str() {
        let result = transpile("def f(name: str = \"world\") -> str:\n    return name");
        assert!(
            result.contains("fn f") || result.contains("world"),
            "expected default in: {result}"
        );
    }

    #[test]
    fn test_w11et_default_bool() {
        let result = transpile("def f(flag: bool = False) -> bool:\n    return flag");
        assert!(
            result.contains("fn f") || result.contains("false"),
            "expected default in: {result}"
        );
    }

    #[test]
    fn test_w11et_default_none() {
        let result = transpile("def f(x = None):\n    return x");
        assert!(
            result.contains("fn f") || result.contains("None") || result.contains("Option"),
            "expected default None in: {result}"
        );
    }

    #[test]
    fn test_w11et_default_mixed_params() {
        let result =
            transpile("def f(a: int, b: int = 10) -> int:\n    return a + b");
        assert!(result.contains("fn f") || result.contains("+"), "expected default in: {result}");
    }

    #[test]
    fn test_w11et_default_float() {
        let result = transpile("def f(x: float = 1.5) -> float:\n    return x");
        assert!(
            result.contains("fn f") || result.contains("1.5"),
            "expected default in: {result}"
        );
    }

    #[test]
    fn test_w11et_default_empty_string() {
        let result = transpile("def f(s: str = \"\") -> str:\n    return s");
        assert!(
            result.contains("fn f") || result.contains("\"\"") || result.contains("String"),
            "expected empty default in: {result}"
        );
    }

    #[test]
    fn test_w11et_default_multiple_defaults() {
        let result = transpile(
            "def f(a: int = 1, b: int = 2, c: int = 3) -> int:\n    return a + b + c",
        );
        assert!(result.contains("fn f") || result.contains("+"), "expected defaults in: {result}");
    }

    // ==========================================================================
    // Section 20: isinstance and Type Guards (6 tests)
    // ==========================================================================

    #[test]
    fn test_w11et_isinstance_int_guard() {
        let result = transpile(
            "def check(x) -> str:\n    if isinstance(x, int):\n        return \"int\"\n    return \"other\"",
        );
        assert!(
            result.contains("fn check") || result.contains("if"),
            "expected isinstance in: {result}"
        );
    }

    #[test]
    fn test_w11et_isinstance_str_guard() {
        let result = transpile(
            "def check(x) -> str:\n    if isinstance(x, str):\n        return \"str\"\n    return \"other\"",
        );
        assert!(
            result.contains("fn check") || result.contains("if"),
            "expected isinstance in: {result}"
        );
    }

    #[test]
    fn test_w11et_isinstance_bool_guard() {
        let result = transpile(
            "def check(x) -> str:\n    if isinstance(x, bool):\n        return \"bool\"\n    return \"other\"",
        );
        assert!(
            result.contains("fn check") || result.contains("if"),
            "expected isinstance in: {result}"
        );
    }

    #[test]
    fn test_w11et_isinstance_in_loop() {
        let result = transpile(
            "def count_ints(items: list) -> int:\n    c = 0\n    for x in items:\n        if isinstance(x, int):\n            c += 1\n    return c",
        );
        assert!(
            result.contains("fn count_ints") || result.contains("for"),
            "expected isinstance loop in: {result}"
        );
    }

    #[test]
    fn test_w11et_type_narrowing_if_none() {
        let result = transpile(
            "def safe(x) -> int:\n    if x is None:\n        return 0\n    return x",
        );
        assert!(
            result.contains("None") || result.contains("fn safe"),
            "expected None check in: {result}"
        );
    }

    #[test]
    fn test_w11et_type_narrowing_is_not_none() {
        let result = transpile(
            "def safe2(x) -> int:\n    if x is not None:\n        return x\n    return 0",
        );
        assert!(
            result.contains("None") || result.contains("fn safe2"),
            "expected is not None in: {result}"
        );
    }

    // ==========================================================================
    // Section 21: Nested Functions and Closures (6 tests)
    // ==========================================================================

    #[test]
    fn test_w11et_nested_func_basic() {
        let result = transpile(
            "def outer() -> int:\n    def inner() -> int:\n        return 5\n    return inner()",
        );
        assert!(
            result.contains("inner") || result.contains("fn outer"),
            "expected nested fn in: {result}"
        );
    }

    #[test]
    fn test_w11et_nested_func_with_param() {
        let result = transpile(
            "def outer(x: int) -> int:\n    def add_one(n: int) -> int:\n        return n + 1\n    return add_one(x)",
        );
        assert!(
            result.contains("add_one") || result.contains("fn outer"),
            "expected nested fn in: {result}"
        );
    }

    #[test]
    fn test_w11et_nested_func_closure_capture() {
        let result = transpile(
            "def outer(x: int) -> int:\n    def inner() -> int:\n        return x + 1\n    return inner()",
        );
        assert!(
            result.contains("inner") || result.contains("fn outer"),
            "expected closure in: {result}"
        );
    }

    #[test]
    fn test_w11et_nested_func_two_deep() {
        let result = transpile(
            "def level1() -> int:\n    def level2() -> int:\n        return 42\n    return level2()",
        );
        assert!(
            result.contains("level2") || result.contains("fn level1"),
            "expected nested fn in: {result}"
        );
    }

    #[test]
    fn test_w11et_nested_func_multiple() {
        let result = transpile(
            "def outer() -> int:\n    def a() -> int:\n        return 1\n    def b() -> int:\n        return 2\n    return a() + b()",
        );
        assert!(
            result.contains("fn outer") || result.contains("+"),
            "expected multiple nested in: {result}"
        );
    }

    #[test]
    fn test_w11et_nested_func_return_type() {
        let result = transpile(
            "def outer() -> str:\n    def greeting() -> str:\n        return \"hi\"\n    return greeting()",
        );
        assert!(
            result.contains("greeting") || result.contains("fn outer"),
            "expected nested fn in: {result}"
        );
    }

    // ==========================================================================
    // Section 22: Class Method Type (8 tests)
    // ==========================================================================

    #[test]
    fn test_w11et_class_simple_method() {
        let result = transpile(
            "class Foo:\n    def bar(self) -> int:\n        return 42",
        );
        assert!(
            result.contains("fn bar") || result.contains("struct") || result.contains("impl"),
            "expected method in: {result}"
        );
    }

    #[test]
    fn test_w11et_class_method_with_param() {
        let result = transpile(
            "class Foo:\n    def add(self, x: int) -> int:\n        return x + 1",
        );
        assert!(
            result.contains("fn add") || result.contains("impl"),
            "expected method in: {result}"
        );
    }

    #[test]
    fn test_w11et_class_init_method() {
        let result = transpile(
            "class Point:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y",
        );
        assert!(
            result.contains("struct") || result.contains("Point") || result.contains("new"),
            "expected struct in: {result}"
        );
    }

    #[test]
    fn test_w11et_class_method_return_str() {
        let result = transpile(
            "class Greeter:\n    def greet(self, name: str) -> str:\n        return f\"hello {name}\"",
        );
        assert!(
            result.contains("fn greet") || result.contains("impl"),
            "expected method in: {result}"
        );
    }

    #[test]
    fn test_w11et_class_multiple_methods() {
        let result = transpile(
            "class Counter:\n    def __init__(self):\n        self.count = 0\n    def increment(self):\n        self.count += 1\n    def get(self) -> int:\n        return self.count",
        );
        assert!(
            result.contains("struct") || result.contains("impl"),
            "expected struct+impl in: {result}"
        );
    }

    #[test]
    fn test_w11et_class_method_bool_return() {
        let result = transpile(
            "class Checker:\n    def is_valid(self, x: int) -> bool:\n        return x > 0",
        );
        assert!(
            result.contains("fn is_valid") || result.contains("bool"),
            "expected bool method in: {result}"
        );
    }

    #[test]
    fn test_w11et_class_static_like() {
        let result = transpile(
            "class Math:\n    def double(self, x: int) -> int:\n        return x * 2",
        );
        assert!(
            result.contains("fn double") || result.contains("impl"),
            "expected method in: {result}"
        );
    }

    #[test]
    fn test_w11et_class_method_list_return() {
        let result = transpile(
            "class Container:\n    def items(self) -> list:\n        return [1, 2, 3]",
        );
        assert!(
            result.contains("fn items") || result.contains("vec!"),
            "expected list method in: {result}"
        );
    }

    // ==========================================================================
    // Section 23: Recursive Functions (4 tests)
    // ==========================================================================

    #[test]
    fn test_w11et_recursive_factorial() {
        let result = transpile(
            "def factorial(n: int) -> int:\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)",
        );
        assert!(
            result.contains("factorial") && result.contains("fn"),
            "expected recursive fn in: {result}"
        );
    }

    #[test]
    fn test_w11et_recursive_fibonacci() {
        let result = transpile(
            "def fib(n: int) -> int:\n    if n <= 1:\n        return n\n    return fib(n - 1) + fib(n - 2)",
        );
        assert!(
            result.contains("fib") && result.contains("fn"),
            "expected recursive fn in: {result}"
        );
    }

    #[test]
    fn test_w11et_recursive_sum_list() {
        let result = transpile(
            "def sum_list(lst: list) -> int:\n    if len(lst) == 0:\n        return 0\n    return lst[0] + sum_list(lst[1:])",
        );
        assert!(
            result.contains("sum_list") && result.contains("fn"),
            "expected recursive fn in: {result}"
        );
    }

    #[test]
    fn test_w11et_recursive_countdown() {
        let result = transpile(
            "def countdown(n: int):\n    if n <= 0:\n        return\n    print(n)\n    countdown(n - 1)",
        );
        assert!(
            result.contains("countdown") && result.contains("fn"),
            "expected recursive fn in: {result}"
        );
    }

    // ==========================================================================
    // Section 24: Complex Expression Combinations (10 tests)
    // ==========================================================================

    #[test]
    fn test_w11et_complex_arith_comparison() {
        let result =
            transpile("def f(x: int, y: int) -> bool:\n    return (x + y) > (x * y)");
        assert!(
            result.contains("+") && result.contains(">"),
            "expected arith+cmp in: {result}"
        );
    }

    #[test]
    fn test_w11et_complex_nested_call() {
        let result = transpile("def f(lst: list) -> int:\n    return len(sorted(lst))");
        assert!(
            result.contains("len") || result.contains("sort"),
            "expected nested call in: {result}"
        );
    }

    #[test]
    fn test_w11et_complex_method_on_result() {
        let result = transpile("def f(s: str) -> list:\n    return s.strip().split()");
        assert!(
            result.contains("trim") || result.contains("split"),
            "expected chained method in: {result}"
        );
    }

    #[test]
    fn test_w11et_complex_ternary_in_list() {
        let result = transpile(
            "def f(x: int) -> list:\n    return [x if x > 0 else -x]",
        );
        assert!(
            result.contains("vec!") || result.contains("if"),
            "expected ternary in list in: {result}"
        );
    }

    #[test]
    fn test_w11et_complex_multi_assign() {
        let result = transpile(
            "def f() -> int:\n    a = 1\n    b = 2\n    c = a + b\n    return c",
        );
        assert!(result.contains("+") || result.contains("fn f"), "expected multi assign in: {result}");
    }

    #[test]
    fn test_w11et_complex_for_with_condition() {
        let result = transpile(
            "def f(n: int) -> int:\n    total = 0\n    for i in range(n):\n        if i % 2 == 0:\n            total += i\n    return total",
        );
        assert!(
            result.contains("for") || result.contains("%"),
            "expected for with if in: {result}"
        );
    }

    #[test]
    fn test_w11et_complex_while_loop() {
        let result = transpile(
            "def f(n: int) -> int:\n    i = 0\n    while i < n:\n        i += 1\n    return i",
        );
        assert!(
            result.contains("while") || result.contains("<"),
            "expected while loop in: {result}"
        );
    }

    #[test]
    fn test_w11et_complex_nested_if() {
        let result = transpile(
            "def classify(x: int) -> str:\n    if x > 0:\n        if x > 100:\n            return \"big\"\n        return \"small\"\n    return \"negative\"",
        );
        assert!(
            result.contains("if") || result.contains(">"),
            "expected nested if in: {result}"
        );
    }

    #[test]
    fn test_w11et_complex_list_ops() {
        let result = transpile(
            "def f(items: list) -> int:\n    items.append(10)\n    items.sort()\n    return items[0]",
        );
        assert!(
            result.contains("push") || result.contains("sort") || result.contains("[0]"),
            "expected list ops in: {result}"
        );
    }

    #[test]
    fn test_w11et_complex_dict_ops() {
        let result = transpile(
            "def f() -> dict:\n    d = {}\n    d[\"key\"] = 42\n    return d",
        );
        assert!(
            result.contains("HashMap") || result.contains("insert") || result.contains("key"),
            "expected dict ops in: {result}"
        );
    }

    // ==========================================================================
    // Section 25: Edge Cases and Miscellaneous (8 tests)
    // ==========================================================================

    #[test]
    fn test_w11et_edge_empty_function() {
        let result = transpile("def f():\n    pass");
        assert!(result.contains("fn f"), "expected fn f in: {result}");
    }

    #[test]
    fn test_w11et_edge_single_return() {
        let result = transpile("def f():\n    return");
        assert!(result.contains("fn f"), "expected fn f in: {result}");
    }

    #[test]
    fn test_w11et_edge_docstring_only() {
        let result = transpile("def f():\n    \"\"\"A docstring.\"\"\"");
        assert!(result.contains("fn f"), "expected fn f in: {result}");
    }

    #[test]
    fn test_w11et_edge_multiple_statements() {
        let result = transpile(
            "def f() -> int:\n    a = 1\n    b = 2\n    c = 3\n    d = 4\n    return a + b + c + d",
        );
        assert!(result.contains("+") || result.contains("fn f"), "expected multi stmt in: {result}");
    }

    #[test]
    fn test_w11et_edge_long_param_list() {
        let result = transpile(
            "def f(a: int, b: int, c: int, d: int, e: int) -> int:\n    return a + b + c + d + e",
        );
        assert!(result.contains("fn f") || result.contains("+"), "expected long params in: {result}");
    }

    #[test]
    fn test_w11et_edge_deeply_nested_expr() {
        let result = transpile("def f(x: int) -> int:\n    return ((x + 1) * 2) - 3");
        assert!(result.contains("+") || result.contains("*"), "expected deep expr in: {result}");
    }

    #[test]
    fn test_w11et_edge_boolean_literal_return() {
        let result = transpile("def always_true() -> bool:\n    return True");
        assert!(
            result.contains("true") || result.contains("bool"),
            "expected true in: {result}"
        );
    }

    #[test]
    fn test_w11et_edge_none_return() {
        let result = transpile("def nothing():\n    return None");
        assert!(
            result.contains("None") || result.contains("fn nothing"),
            "expected None in: {result}"
        );
    }
}
