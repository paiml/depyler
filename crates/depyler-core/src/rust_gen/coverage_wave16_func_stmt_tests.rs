//! Wave 16: Coverage tests for func_gen, stmt_gen, and type coercion/binary ops.
//!
//! Tests target UNCOVERED code paths in:
//! - func_gen: nested functions, closures, recursion, *args, **kwargs, keyword-only params,
//!   default mutable args, lambda, multiple returns, type annotations, generators,
//!   decorators, docstrings, empty functions, single-expression functions
//! - stmt_gen: with statement, context managers, async with, try/except/finally,
//!   multiple except, except as, bare except, raise from, re-raise, assert with message,
//!   global, nonlocal, del, augmented assignments, tuple unpacking, chained comparisons,
//!   walrus operator
//! - type coercion/binary ops: int+float, floor division, true division, power, modulo,
//!   bitwise ops, boolean ops, comparison chains, identity, membership, string concat,
//!   string repetition, list concat, unary ops, complex expressions, ternary, f-strings
//!
//! Status: 200 tests (test_w16fs_func_001..070, test_w16fs_stmt_071..140,
//!         test_w16fs_type_141..200)

#[cfg(test)]
mod tests {
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

    // =========================================================================
    // FUNC_GEN PATTERNS (70 tests: test_w16fs_func_001 through test_w16fs_func_070)
    // =========================================================================

    #[test]
    fn test_w16fs_func_001_nested_function_basic() {
        let result =
            transpile("def outer():\n    def inner():\n        return 42\n    return inner()\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w16fs_func_002_nested_function_with_param() {
        let result = transpile("def outer(x: int):\n    def inner(y: int) -> int:\n        return y + 1\n    return inner(x)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_003_nested_function_two_levels() {
        let result =
            transpile("def level1():\n    def level2():\n        return 10\n    return level2()\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_004_nested_function_returns_string() {
        let result = transpile("def outer() -> str:\n    def inner() -> str:\n        return \"hello\"\n    return inner()\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_005_closure_captures_variable() {
        let result = transpile("def outer(x: int):\n    def inner(y: int) -> int:\n        return x + y\n    return inner(10)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_006_closure_captures_string() {
        let result = transpile("def outer(prefix: str):\n    def inner(name: str) -> str:\n        return prefix + name\n    return inner(\"world\")\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_007_closure_captures_multiple() {
        let result = transpile("def outer(a: int, b: int):\n    def inner(c: int) -> int:\n        return a + b + c\n    return inner(3)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_008_recursive_factorial() {
        let result = transpile("def factorial(n: int) -> int:\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)\n");
        assert!(!result.is_empty());
        assert!(result.contains("factorial"));
    }

    #[test]
    fn test_w16fs_func_009_recursive_fibonacci() {
        let result = transpile("def fib(n: int) -> int:\n    if n <= 1:\n        return n\n    return fib(n - 1) + fib(n - 2)\n");
        assert!(!result.is_empty());
        assert!(result.contains("fib"));
    }

    #[test]
    fn test_w16fs_func_010_recursive_sum() {
        let result = transpile("def rec_sum(n: int) -> int:\n    if n == 0:\n        return 0\n    return n + rec_sum(n - 1)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_011_varargs_basic() {
        let result = transpile("def func(*args):\n    return args\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_012_varargs_with_regular() {
        let result = transpile("def func(first: int, *rest):\n    return first\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_013_kwargs_basic() {
        let result = transpile("def func(**kwargs):\n    return kwargs\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_014_args_and_kwargs() {
        let result = transpile("def func(*args, **kwargs):\n    return args\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_015_args_kwargs_with_regular() {
        let result = transpile("def func(name: str, *args, **kwargs):\n    return name\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_016_keyword_only_with_default() {
        let result = transpile("def func(*, key: int = 0):\n    return key\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_017_keyword_only_multiple() {
        let result = transpile(
            "def func(*, width: int = 10, height: int = 20):\n    return width * height\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_018_default_none_guard() {
        let result = transpile(
            "def func(lst = None):\n    if lst is None:\n        lst = []\n    return lst\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_019_default_int_param() {
        let result = transpile("def func(x: int = 5) -> int:\n    return x * 2\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_020_default_string_param() {
        let result =
            transpile("def greet(name: str = \"world\") -> str:\n    return \"hello \" + name\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_021_default_bool_param() {
        let result = transpile("def toggle(flag: bool = False) -> bool:\n    return not flag\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_022_lambda_simple() {
        let result = transpile("double = lambda x: x * 2\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_023_lambda_two_params() {
        let result = transpile("add = lambda x, y: x + y\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_024_lambda_ternary() {
        let result = transpile("bigger = lambda x, y: x if x > y else y\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_025_lambda_no_params() {
        let result = transpile("const_fn = lambda: 42\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_026_lambda_string() {
        let result = transpile("shout = lambda s: s.upper()\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_027_multiple_return_tuple() {
        let result = transpile("def func() -> tuple:\n    return 1, 2, 3\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_028_multiple_return_two() {
        let result = transpile("def swap(a: int, b: int):\n    return b, a\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_029_return_computed_tuple() {
        let result = transpile(
            "def minmax(a: int, b: int):\n    if a < b:\n        return a, b\n    return b, a\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_030_typed_params_int_str() {
        let result = transpile("def describe(n: int, label: str) -> str:\n    return label\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_031_typed_params_bool_return() {
        let result =
            transpile("def is_valid(x: int, y: int) -> bool:\n    return x > 0 and y > 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_032_typed_float_return() {
        let result =
            transpile("def average(a: float, b: float) -> float:\n    return (a + b) / 2.0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_033_typed_list_param() {
        let result = transpile("def first(items: list) -> int:\n    return items[0]\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_034_typed_dict_param() {
        let result = transpile("def get_val(d: dict, key: str):\n    return d[key]\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_035_generator_yield_single() {
        let result = transpile("def gen():\n    yield 1\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_036_generator_yield_multiple() {
        let result = transpile("def gen():\n    yield 1\n    yield 2\n    yield 3\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_037_generator_yield_in_loop() {
        let result = transpile("def gen(n: int):\n    for i in range(n):\n        yield i\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_038_generator_yield_string() {
        let result = transpile("def words():\n    yield \"hello\"\n    yield \"world\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_039_generator_with_condition() {
        let result = transpile("def even_gen(n: int):\n    for i in range(n):\n        if i % 2 == 0:\n            yield i\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_040_staticmethod_basic() {
        let result = transpile("class Util:\n    @staticmethod\n    def compute(x: int) -> int:\n        return x * 2\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_041_classmethod_basic() {
        let result = transpile("class Builder:\n    @classmethod\n    def create(cls) -> str:\n        return \"instance\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_042_property_getter() {
        let result = transpile("class Box:\n    def __init__(self, w: int, h: int):\n        self.w = w\n        self.h = h\n    @property\n    def area(self) -> int:\n        return self.w * self.h\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_043_docstring_single_line() {
        let result =
            transpile("def func():\n    \"\"\"Single line docstring.\"\"\"\n    return 1\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_044_docstring_multiline() {
        let result = transpile(
            "def func():\n    \"\"\"First line.\n\n    More details.\n    \"\"\"\n    return 1\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_045_empty_function_pass() {
        let result = transpile("def noop():\n    pass\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w16fs_func_046_empty_function_ellipsis() {
        let result = transpile("def placeholder():\n    ...\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_047_single_expr_add() {
        let result = transpile("def add(x: int, y: int) -> int:\n    return x + y\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w16fs_func_048_single_expr_multiply() {
        let result = transpile("def mul(x: int, y: int) -> int:\n    return x * y\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_049_single_expr_negate() {
        let result = transpile("def negate(x: int) -> int:\n    return -x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_050_function_returning_list() {
        let result = transpile("def make_list() -> list:\n    return [1, 2, 3]\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_051_function_returning_dict() {
        let result = transpile("def make_dict() -> dict:\n    return {\"a\": 1, \"b\": 2}\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_052_function_with_conditional_return() {
        let result = transpile("def sign(x: int) -> int:\n    if x > 0:\n        return 1\n    elif x < 0:\n        return -1\n    else:\n        return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_053_function_with_early_return() {
        let result = transpile("def check(x: int) -> str:\n    if x < 0:\n        return \"negative\"\n    return \"non-negative\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_054_function_with_loop_and_return() {
        let result = transpile("def find_first(items: list, target: int) -> int:\n    for item in items:\n        if item == target:\n            return item\n    return -1\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_055_function_with_while_loop() {
        let result = transpile("def countdown(n: int) -> int:\n    count: int = 0\n    while n > 0:\n        n = n - 1\n        count = count + 1\n    return count\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_056_function_multiple_defaults() {
        let result = transpile(
            "def config(host: str = \"localhost\", port: int = 8080) -> str:\n    return host\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_057_nested_function_sibling() {
        let result = transpile("def outer():\n    def helper_a() -> int:\n        return 1\n    def helper_b() -> int:\n        return 2\n    return helper_a() + helper_b()\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_058_function_with_list_append() {
        let result = transpile("def build() -> list:\n    items = []\n    items.append(1)\n    items.append(2)\n    return items\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_059_function_with_string_methods() {
        let result = transpile("def process(s: str) -> str:\n    return s.strip().lower()\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_060_function_returning_none() {
        let result = transpile("def side_effect(msg: str):\n    print(msg)\n    return None\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_061_function_with_for_range() {
        let result = transpile("def sum_range(n: int) -> int:\n    total: int = 0\n    for i in range(n):\n        total = total + i\n    return total\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_062_function_with_nested_if() {
        let result = transpile("def classify(x: int) -> str:\n    if x > 0:\n        if x > 100:\n            return \"large\"\n        return \"small\"\n    return \"non-positive\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_063_function_with_boolean_logic() {
        let result = transpile("def both_true(a: bool, b: bool) -> bool:\n    return a and b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_064_function_with_or_logic() {
        let result = transpile("def either_true(a: bool, b: bool) -> bool:\n    return a or b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_065_function_with_not_logic() {
        let result = transpile("def invert(flag: bool) -> bool:\n    return not flag\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_066_function_with_type_str_int() {
        let result = transpile("def to_str(x: int) -> str:\n    return str(x)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_067_function_with_type_int_cast() {
        let result = transpile("def to_int(s: str) -> int:\n    return int(s)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_068_function_with_enumerate() {
        let result = transpile(
            "def indexed(items: list):\n    for i, item in enumerate(items):\n        print(i)\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_069_function_with_zip() {
        let result = transpile(
            "def paired(a: list, b: list):\n    for x, y in zip(a, b):\n        print(x)\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_func_070_function_returning_set() {
        let result = transpile("def make_set() -> set:\n    return {1, 2, 3}\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // STMT_GEN PATTERNS (70 tests: test_w16fs_stmt_071 through test_w16fs_stmt_140)
    // =========================================================================

    #[test]
    fn test_w16fs_stmt_071_with_open_read() {
        let result = transpile("def read_file():\n    with open(\"data.txt\") as f:\n        data = f.read()\n    return data\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_072_with_open_write() {
        let result = transpile("def write_file():\n    with open(\"out.txt\", \"w\") as f:\n        f.write(\"hello\")\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_073_with_no_as() {
        let result =
            transpile("def use_ctx():\n    with open(\"f.txt\"):\n        print(\"done\")\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_074_try_except_basic() {
        let result = transpile("def safe_div(a: int, b: int) -> int:\n    try:\n        return a // b\n    except:\n        return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_075_try_except_value_error() {
        let result = transpile("def safe_parse(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError:\n        return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_076_try_except_as() {
        let result = transpile("def handle():\n    try:\n        x = int(\"abc\")\n    except ValueError as e:\n        print(e)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_077_try_except_finally() {
        let result = transpile("def cleanup():\n    try:\n        x = 1\n    except:\n        x = 0\n    finally:\n        print(\"done\")\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_078_try_except_else() {
        let result = transpile("def with_else():\n    try:\n        x: int = 1\n    except:\n        x = 0\n    else:\n        x = 2\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_079_try_multiple_except() {
        let result = transpile("def multi_catch():\n    try:\n        x = int(\"abc\")\n    except ValueError:\n        x = 0\n    except TypeError:\n        x = -1\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_080_try_except_tuple() {
        let result = transpile("def catch_many():\n    try:\n        x = int(\"abc\")\n    except (ValueError, TypeError):\n        x = 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_081_raise_basic() {
        let result = transpile("def fail():\n    raise ValueError(\"bad input\")\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_082_raise_runtime_error() {
        let result = transpile("def boom():\n    raise RuntimeError(\"unexpected\")\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_083_raise_from() {
        let result = transpile("def chain_error():\n    try:\n        x = int(\"abc\")\n    except ValueError as orig:\n        raise RuntimeError(\"wrap\") from orig\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_084_reraise() {
        let result = transpile(
            "def reraise():\n    try:\n        x = int(\"abc\")\n    except:\n        raise\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_085_assert_basic() {
        let result = transpile("def validate(x: int):\n    assert x > 0\n");
        assert!(!result.is_empty());
        assert!(result.contains("assert"));
    }

    #[test]
    fn test_w16fs_stmt_086_assert_with_message() {
        let result = transpile("def validate(x: int):\n    assert x > 0, \"must be positive\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_087_assert_equality() {
        let result = transpile("def check_result(a: int, b: int):\n    assert a == b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_088_global_statement() {
        let result = transpile(
            "counter = 0\ndef increment():\n    global counter\n    counter = counter + 1\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_089_global_multiple() {
        let result = transpile("total = 0\ncount = 0\ndef update():\n    global total, count\n    total = total + 1\n    count = count + 1\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_090_nonlocal_statement() {
        let result = transpile("def outer():\n    x: int = 0\n    def inner():\n        nonlocal x\n        x = x + 1\n    inner()\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_091_del_variable() {
        let result = transpile("def cleanup():\n    x: int = 5\n    del x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_092_augassign_add() {
        let result = transpile("def inc(x: int) -> int:\n    x += 1\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_093_augassign_sub() {
        let result = transpile("def dec(x: int) -> int:\n    x -= 1\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_094_augassign_mul() {
        let result = transpile("def triple(x: int) -> int:\n    x *= 3\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_095_augassign_div() {
        let result = transpile("def halve(x: float) -> float:\n    x /= 2.0\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_096_augassign_floor_div() {
        let result = transpile("def half_int(x: int) -> int:\n    x //= 2\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_097_augassign_modulo() {
        let result = transpile("def mod_assign(x: int) -> int:\n    x %= 7\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_098_augassign_power() {
        let result = transpile("def square(x: int) -> int:\n    x **= 2\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_099_augassign_bitand() {
        let result = transpile("def mask(x: int) -> int:\n    x &= 0xff\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_100_augassign_bitor() {
        let result = transpile("def set_bit(x: int) -> int:\n    x |= 1\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_101_augassign_bitxor() {
        let result = transpile("def flip(x: int) -> int:\n    x ^= 16\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_102_augassign_lshift() {
        let result = transpile("def shift_left(x: int) -> int:\n    x <<= 2\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_103_augassign_rshift() {
        let result = transpile("def shift_right(x: int) -> int:\n    x >>= 1\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_104_tuple_unpack_two() {
        let result = transpile("def unpack():\n    a, b = 1, 2\n    return a\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_105_tuple_unpack_three() {
        let result = transpile("def unpack3():\n    a, b, c = 1, 2, 3\n    return a + b + c\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_106_tuple_unpack_from_call() {
        let result = transpile("def split_pair():\n    a, b = divmod(10, 3)\n    return a\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_107_chained_comparison() {
        let result = transpile("def in_range(x: int) -> bool:\n    return 0 < x < 100\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_108_chained_comparison_leq() {
        let result = transpile("def in_bounds(x: int) -> bool:\n    return 0 <= x <= 255\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_109_walrus_in_if() {
        let result = transpile("def process(data: list):\n    if (n := len(data)) > 10:\n        return n\n    return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_110_walrus_in_while() {
        let result = transpile("def read_until():\n    line = \"data\"\n    while (n := len(line)) > 0:\n        print(n)\n        break\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_111_multiple_assign_same_value() {
        let result =
            transpile("def init():\n    a = 0\n    b = 0\n    c = 0\n    return a + b + c\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_112_assign_from_function() {
        let result = transpile("def process():\n    result = len(\"hello\")\n    return result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_113_for_with_break() {
        let result = transpile("def find_five(items: list) -> bool:\n    for item in items:\n        if item == 5:\n            break\n    return True\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_114_for_with_continue() {
        let result = transpile("def skip_neg(items: list):\n    for item in items:\n        if item < 0:\n            continue\n        print(item)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_115_while_with_break() {
        let result = transpile("def until_zero(n: int) -> int:\n    while True:\n        if n <= 0:\n            break\n        n = n - 1\n    return n\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_116_while_with_continue() {
        let result = transpile("def skip_even() -> int:\n    i: int = 0\n    total: int = 0\n    while i < 10:\n        i += 1\n        if i % 2 == 0:\n            continue\n        total += i\n    return total\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_117_if_elif_else_chain() {
        let result = transpile("def grade(score: int) -> str:\n    if score >= 90:\n        return \"A\"\n    elif score >= 80:\n        return \"B\"\n    elif score >= 70:\n        return \"C\"\n    else:\n        return \"F\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_118_nested_for_loops() {
        let result = transpile("def matrix_sum() -> int:\n    total: int = 0\n    for i in range(3):\n        for j in range(3):\n            total += i + j\n    return total\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_119_for_else() {
        let result = transpile("def search(items: list, target: int) -> bool:\n    for item in items:\n        if item == target:\n            return True\n    return False\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_120_assign_list_literal() {
        let result = transpile("def make():\n    items = [1, 2, 3, 4, 5]\n    return items\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_121_assign_dict_literal() {
        let result = transpile("def make_map():\n    d = {\"a\": 1, \"b\": 2}\n    return d\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_122_assign_set_literal() {
        let result = transpile("def make_set():\n    s = {1, 2, 3}\n    return s\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_123_augassign_string_concat() {
        let result = transpile("def build_string() -> str:\n    s: str = \"hello\"\n    s += \" world\"\n    return s\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_124_try_except_index_error() {
        let result = transpile("def safe_get(items: list, idx: int):\n    try:\n        return items[idx]\n    except IndexError:\n        return None\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_125_try_except_key_error() {
        let result = transpile("def safe_lookup(d: dict, key: str):\n    try:\n        return d[key]\n    except KeyError:\n        return None\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_126_raise_type_error() {
        let result = transpile("def strict(x: int):\n    if x < 0:\n        raise TypeError(\"expected positive\")\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_127_raise_not_implemented() {
        let result = transpile(
            "def abstract_method():\n    raise NotImplementedError(\"subclass must implement\")\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_128_pass_in_if() {
        let result = transpile(
            "def maybe(x: int):\n    if x > 0:\n        pass\n    else:\n        return x\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_129_pass_in_except() {
        let result = transpile(
            "def silent():\n    try:\n        x = int(\"abc\")\n    except:\n        pass\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_130_return_none_explicit() {
        let result = transpile("def void_func():\n    print(\"side effect\")\n    return None\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_131_return_empty() {
        let result =
            transpile("def early_exit(x: int):\n    if x < 0:\n        return\n    print(x)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_132_assign_negative_literal() {
        let result = transpile("def neg() -> int:\n    x: int = -1\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_133_assign_bool_true() {
        let result = transpile("def flag() -> bool:\n    active: bool = True\n    return active\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_134_assign_bool_false() {
        let result = transpile("def no_flag() -> bool:\n    done: bool = False\n    return done\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_135_assign_empty_string() {
        let result = transpile("def blank() -> str:\n    s: str = \"\"\n    return s\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_136_assign_float_literal() {
        let result = transpile("def ratio() -> float:\n    r: float = 0.5\n    return r\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_137_assign_from_binary_op() {
        let result = transpile(
            "def compute(a: int, b: int) -> int:\n    result: int = a + b\n    return result\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_138_assign_from_comparison() {
        let result =
            transpile("def is_big(x: int) -> bool:\n    big: bool = x > 100\n    return big\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_139_for_range_step() {
        let result = transpile("def evens() -> int:\n    total: int = 0\n    for i in range(0, 10, 2):\n        total += i\n    return total\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_stmt_140_nested_try_except() {
        let result = transpile("def nested_try():\n    try:\n        try:\n            x = int(\"abc\")\n        except ValueError:\n            x = 0\n    except:\n        x = -1\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // TYPE COERCION / BINARY OPS (60 tests: test_w16fs_type_141 through test_w16fs_type_200)
    // =========================================================================

    #[test]
    fn test_w16fs_type_141_int_plus_float() {
        let result = transpile("def mixed() -> float:\n    return 5 + 3.5\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_142_int_mul_float() {
        let result = transpile("def scaled() -> float:\n    return 3 * 2.5\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_143_floor_division_int() {
        let result = transpile("def floor_div(a: int, b: int) -> int:\n    return a // b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_144_true_division() {
        let result = transpile("def true_div(a: int, b: int) -> float:\n    return a / b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_145_power_int() {
        let result = transpile("def power(base: int, exp: int) -> int:\n    return base ** exp\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_146_modulo_int() {
        let result = transpile("def modulo(a: int, b: int) -> int:\n    return a % b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_147_bitwise_and() {
        let result = transpile("def bit_and(x: int, y: int) -> int:\n    return x & y\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_148_bitwise_or() {
        let result = transpile("def bit_or(x: int, y: int) -> int:\n    return x | y\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_149_bitwise_xor() {
        let result = transpile("def bit_xor(x: int, y: int) -> int:\n    return x ^ y\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_150_bitwise_not() {
        let result = transpile("def bit_not(x: int) -> int:\n    return ~x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_151_left_shift() {
        let result = transpile("def lshift(x: int) -> int:\n    return x << 2\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_152_right_shift() {
        let result = transpile("def rshift(x: int) -> int:\n    return x >> 1\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_153_boolean_and() {
        let result = transpile("def logic_and(a: bool, b: bool) -> bool:\n    return a and b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_154_boolean_or() {
        let result = transpile("def logic_or(a: bool, b: bool) -> bool:\n    return a or b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_155_boolean_not() {
        let result = transpile("def logic_not(a: bool) -> bool:\n    return not a\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_156_compare_eq() {
        let result = transpile("def eq(a: int, b: int) -> bool:\n    return a == b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_157_compare_neq() {
        let result = transpile("def neq(a: int, b: int) -> bool:\n    return a != b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_158_compare_lt() {
        let result = transpile("def lt(a: int, b: int) -> bool:\n    return a < b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_159_compare_lte() {
        let result = transpile("def lte(a: int, b: int) -> bool:\n    return a <= b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_160_compare_gt() {
        let result = transpile("def gt(a: int, b: int) -> bool:\n    return a > b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_161_compare_gte() {
        let result = transpile("def gte(a: int, b: int) -> bool:\n    return a >= b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_162_identity_is_none() {
        let result = transpile("def check_none(x) -> bool:\n    return x is None\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_163_identity_is_not_none() {
        let result = transpile("def has_value(x) -> bool:\n    return x is not None\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_164_membership_in_list() {
        let result =
            transpile("def contains(items: list, val: int) -> bool:\n    return val in items\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_165_membership_not_in() {
        let result =
            transpile("def missing(items: list, val: int) -> bool:\n    return val not in items\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_166_string_concat() {
        let result = transpile("def greet() -> str:\n    return \"hello\" + \" \" + \"world\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_167_string_repetition() {
        let result = transpile("def repeat() -> str:\n    return \"abc\" * 3\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_168_list_concat() {
        let result = transpile("def merge():\n    return [1, 2] + [3, 4]\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_169_unary_neg() {
        let result = transpile("def negate(x: int) -> int:\n    return -x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_170_unary_pos() {
        let result = transpile("def identity(x: int) -> int:\n    return +x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_171_unary_bitnot() {
        let result = transpile("def complement(x: int) -> int:\n    return ~x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_172_complex_expr_add_mul() {
        let result = transpile(
            "def calc(a: int, b: int, c: int, d: int) -> int:\n    return (a + b) * (c - d)\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_173_complex_expr_nested_parens() {
        let result =
            transpile("def nested(x: int, y: int) -> int:\n    return ((x + 1) * (y - 1)) + 2\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_174_ternary_basic() {
        let result =
            transpile("def pick(x: int) -> str:\n    return \"pos\" if x > 0 else \"neg\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_175_ternary_int() {
        let result = transpile("def abs_val(x: int) -> int:\n    return x if x >= 0 else -x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_176_ternary_nested() {
        let result = transpile("def classify(x: int) -> str:\n    return \"pos\" if x > 0 else \"zero\" if x == 0 else \"neg\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_177_fstring_simple_var() {
        let result = transpile("def fmt(name: str) -> str:\n    return f\"hello {name}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_178_fstring_expression() {
        let result =
            transpile("def fmt_expr(a: int, b: int) -> str:\n    return f\"sum = {a + b}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_179_fstring_format_spec() {
        let result =
            transpile("def fmt_float(value: float) -> str:\n    return f\"{value:.2f}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_180_fstring_multiple_values() {
        let result =
            transpile("def fmt_multi(x: int, y: int) -> str:\n    return f\"{x} and {y}\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_181_int_sub_float() {
        let result = transpile("def diff() -> float:\n    return 10 - 2.5\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_182_float_floor_div() {
        let result = transpile("def fdiv(a: float, b: float) -> float:\n    return a // b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_183_float_modulo() {
        let result = transpile("def fmod(a: float, b: float) -> float:\n    return a % b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_184_power_float() {
        let result = transpile("def fpow(x: float, y: float) -> float:\n    return x ** y\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_185_complex_boolean_expr() {
        let result = transpile(
            "def check(a: bool, b: bool, c: bool) -> bool:\n    return (a and b) or (not c)\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_186_mixed_comparisons() {
        let result = transpile("def in_band(x: int) -> bool:\n    return x >= 10 and x <= 100\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_187_string_eq() {
        let result = transpile("def same(a: str, b: str) -> bool:\n    return a == b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_188_string_neq() {
        let result = transpile("def different(a: str, b: str) -> bool:\n    return a != b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_189_float_comparison() {
        let result = transpile("def bigger(a: float, b: float) -> bool:\n    return a > b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_190_int_to_float_coercion() {
        let result = transpile("def coerce(x: int) -> float:\n    return x + 0.0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_191_bool_to_int_context() {
        let result = transpile("def count_true(a: bool, b: bool) -> int:\n    total: int = 0\n    if a:\n        total += 1\n    if b:\n        total += 1\n    return total\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_192_multi_op_precedence() {
        let result = transpile("def prec(a: int, b: int, c: int) -> int:\n    return a + b * c\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_193_multi_op_with_sub() {
        let result = transpile("def calc(a: int, b: int, c: int) -> int:\n    return a - b + c\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_194_div_and_mod() {
        let result = transpile(
            "def divmod_manual(a: int, b: int):\n    q = a // b\n    r = a % b\n    return q, r\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_195_bitwise_combo() {
        let result = transpile("def mask_and_shift(x: int) -> int:\n    return (x & 0xff) << 8\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_196_list_comprehension_simple() {
        let result =
            transpile("def squares(n: int) -> list:\n    return [i * i for i in range(n)]\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_197_list_comprehension_filter() {
        let result = transpile(
            "def evens(n: int) -> list:\n    return [i for i in range(n) if i % 2 == 0]\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_198_dict_comprehension() {
        let result =
            transpile("def mapping(n: int) -> dict:\n    return {i: i * i for i in range(n)}\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_199_set_comprehension() {
        let result =
            transpile("def unique_squares(n: int) -> set:\n    return {i * i for i in range(n)}\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w16fs_type_200_fstring_with_method_call() {
        let result =
            transpile("def fmt_upper(name: str) -> str:\n    return f\"HELLO {name.upper()}\"\n");
        assert!(!result.is_empty());
    }
}
