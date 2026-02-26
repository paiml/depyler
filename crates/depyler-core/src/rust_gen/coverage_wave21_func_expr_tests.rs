//! Wave 21 coverage tests: function generation, expression generation, and type inference
//!
//! Targets deep uncovered code paths in:
//! - func_gen: default params, *args, **kwargs, mixed params, return types,
//!   recursion, generators, async, decorators, closures, multiple returns
//! - expr_gen: binary/comparison/unary ops, ternary, f-strings, comprehensions,
//!   lambdas, slicing, star unpacking, walrus, chained comparisons, method chaining
//! - type inference: literal types, collection types, operation result types,
//!   annotation overrides, optional/union/callable inference
//!
//! 200 tests total

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

    // ========================================================================
    // SECTION 1: Function generation deep coverage (tests 001-080)
    // ========================================================================

    #[test]
    fn test_w21fe_001_func_default_int_param() {
        let result = transpile("def f(x: int, y: int = 0) -> int:\n    return x + y\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_002_func_default_string_param() {
        let result =
            transpile("def greet(name: str = \"world\") -> str:\n    return \"hello \" + name\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_003_func_default_float_param() {
        let result = transpile(
            "def scale(x: float, factor: float = 1.0) -> float:\n    return x * factor\n",
        );
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_004_func_default_bool_param() {
        let result =
            transpile("def process(data: int, verbose: bool = False) -> int:\n    return data\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_005_func_default_none_param() {
        let result = transpile("def maybe(x: int = None) -> int:\n    return 0\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_006_func_multiple_defaults() {
        let result =
            transpile("def f(a: int, b: int = 1, c: str = \"x\") -> int:\n    return a + b\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_007_func_args_variadic() {
        let result = transpile("def f(*args):\n    return len(args)\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_008_func_kwargs_variadic() {
        let result = transpile("def f(**kwargs):\n    return len(kwargs)\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_009_func_mixed_params_args_kwargs() {
        let result = transpile("def f(a: int, b: int = 1, *args, **kwargs):\n    return a\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_010_func_return_optional_int() {
        let result = transpile("from typing import Optional\ndef f(x: int) -> Optional[int]:\n    if x > 0:\n        return x\n    return None\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_011_func_return_optional_str() {
        let result = transpile("from typing import Optional\ndef find(items: list, key: str) -> Optional[str]:\n    return None\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_012_func_return_bool_with_raise() {
        let result = transpile("def validate(x: int) -> bool:\n    if x < 0:\n        raise ValueError(\"negative\")\n    return True\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_013_func_return_list_str() {
        let result = transpile(
            "from typing import List\ndef words(s: str) -> List[str]:\n    return s.split()\n",
        );
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_014_func_return_dict_str_int() {
        let result = transpile("from typing import Dict\ndef count_chars(s: str) -> Dict[str, int]:\n    result = {}\n    return result\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_015_func_return_tuple() {
        let result = transpile("from typing import Tuple\ndef swap(a: int, b: int) -> Tuple[int, int]:\n    return b, a\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_016_recursive_fibonacci() {
        let result = transpile("def fib(n: int) -> int:\n    if n <= 1:\n        return n\n    return fib(n - 1) + fib(n - 2)\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
        assert!(result.contains("fib"));
    }

    #[test]
    fn test_w21fe_017_recursive_factorial() {
        let result = transpile("def factorial(n: int) -> int:\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)\n");
        assert!(!result.is_empty());
        assert!(result.contains("factorial"));
    }

    #[test]
    fn test_w21fe_018_func_calls_another_with_string() {
        let result = transpile("def helper(s: str) -> str:\n    return s.upper()\n\ndef main_func(name: str) -> str:\n    return helper(name)\n");
        assert!(!result.is_empty());
        assert!(result.contains("helper"));
        assert!(result.contains("main_func"));
    }

    #[test]
    fn test_w21fe_019_generator_function_yield() {
        let result = transpile(
            "def gen(n: int):\n    i = 0\n    while i < n:\n        yield i\n        i = i + 1\n",
        );
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_020_async_function_basic() {
        let result = transpile("async def fetch() -> str:\n    return \"data\"\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_021_func_with_docstring() {
        let result = transpile(
            "def documented(x: int) -> int:\n    \"\"\"Return x doubled.\"\"\"\n    return x * 2\n",
        );
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_022_func_with_multiline_docstring() {
        let result = transpile("def compute(x: int) -> int:\n    \"\"\"Compute value.\n\n    Args:\n        x: Input value.\n    \"\"\"\n    return x + 1\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_023_nested_function_closure() {
        let result = transpile("def outer(x: int) -> int:\n    def inner(y: int) -> int:\n        return x + y\n    return inner(10)\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_024_func_with_global() {
        let result = transpile(
            "counter = 0\ndef increment():\n    global counter\n    counter = counter + 1\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_025_func_with_type_annotations() {
        let result = transpile("def typed(x: int, y: str, z: float) -> bool:\n    return True\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_026_func_with_list_annotation() {
        let result = transpile("from typing import List\ndef process(items: List[int]) -> int:\n    return len(items)\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_027_lambda_in_function() {
        let result = transpile(
            "def apply(x: int) -> int:\n    double = lambda a: a * 2\n    return double(x)\n",
        );
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_028_func_with_assert() {
        let result = transpile(
            "def safe_div(a: int, b: int) -> int:\n    assert b != 0\n    return a // b\n",
        );
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_029_func_with_assert_message() {
        let result = transpile(
            "def check(x: int) -> int:\n    assert x > 0, \"must be positive\"\n    return x\n",
        );
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_030_func_with_del() {
        let result = transpile("def cleanup():\n    x = 10\n    del x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_031_func_with_pass() {
        let result = transpile("def noop():\n    pass\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_032_func_multiple_return_points() {
        let result = transpile("def classify(x: int) -> str:\n    if x > 0:\n        return \"positive\"\n    if x < 0:\n        return \"negative\"\n    return \"zero\"\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_033_func_early_return() {
        let result = transpile("def find_first(items: list) -> int:\n    for item in items:\n        if item > 0:\n            return item\n    return -1\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_034_func_try_except() {
        let result = transpile("def safe(x: int) -> int:\n    try:\n        return x // x\n    except:\n        return 0\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_035_func_try_except_finally() {
        let result = transpile("def cleanup_func() -> int:\n    try:\n        return 1\n    except:\n        return 0\n    finally:\n        pass\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_036_func_for_else() {
        let result = transpile("def search(items: list, target: int) -> bool:\n    for item in items:\n        if item == target:\n            return True\n    else:\n        return False\n    return False\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_037_func_while_break() {
        let result = transpile("def count_up(limit: int) -> int:\n    i = 0\n    while True:\n        if i >= limit:\n            break\n        i = i + 1\n    return i\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_038_func_while_continue() {
        let result = transpile("def sum_positive(items: list) -> int:\n    total = 0\n    i = 0\n    while i < len(items):\n        i = i + 1\n        if items[i - 1] < 0:\n            continue\n        total = total + items[i - 1]\n    return total\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_039_func_raise_valueerror() {
        let result = transpile("def validate(x: int) -> int:\n    if x < 0:\n        raise ValueError(\"negative\")\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_040_func_raise_typeerror() {
        let result = transpile("def check_type(x) -> str:\n    if not isinstance(x, str):\n        raise TypeError(\"expected str\")\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_041_func_return_none_explicitly() {
        let result = transpile(
            "def maybe_none(x: int):\n    if x > 0:\n        return x\n    return None\n",
        );
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_042_func_only_side_effects() {
        let result = transpile("def log_value(x: int):\n    print(x)\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_043_func_chained_calls() {
        let result = transpile("def transform(s: str) -> str:\n    return s.strip().lower()\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_044_func_with_enumerate() {
        let result = transpile("def index_items(items: list) -> list:\n    result = []\n    for i, item in enumerate(items):\n        result.append(i)\n    return result\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_045_func_with_zip() {
        let result = transpile("def combine(a: list, b: list) -> list:\n    result = []\n    for x, y in zip(a, b):\n        result.append(x)\n    return result\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_046_func_with_isinstance() {
        let result = transpile("def check(x) -> bool:\n    return isinstance(x, int)\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_047_func_returning_empty_list() {
        let result = transpile("def empty_list() -> list:\n    return []\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_048_func_returning_empty_dict() {
        let result = transpile("def empty_dict() -> dict:\n    return {}\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_049_func_returning_tuple_literal() {
        let result = transpile("def pair(x: int, y: int) -> tuple:\n    return (x, y)\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_050_func_multiple_assignment() {
        let result =
            transpile("def multi() -> int:\n    a = 1\n    b = 2\n    c = a + b\n    return c\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_051_func_augmented_assign() {
        let result = transpile("def accum(n: int) -> int:\n    total = 0\n    i = 0\n    while i < n:\n        total += i\n        i += 1\n    return total\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_052_func_string_concatenation() {
        let result = transpile(
            "def greet(first: str, last: str) -> str:\n    return first + \" \" + last\n",
        );
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_053_func_list_append_in_loop() {
        let result = transpile("def squares(n: int) -> list:\n    result = []\n    for i in range(n):\n        result.append(i * i)\n    return result\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_054_func_dict_update() {
        let result = transpile("def build_map(keys: list, val: int) -> dict:\n    d = {}\n    for k in keys:\n        d[k] = val\n    return d\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_055_func_nested_if() {
        let result = transpile("def classify(x: int, y: int) -> str:\n    if x > 0:\n        if y > 0:\n            return \"both positive\"\n        return \"x positive\"\n    return \"x not positive\"\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_056_func_elif_chain() {
        let result = transpile("def grade(score: int) -> str:\n    if score >= 90:\n        return \"A\"\n    elif score >= 80:\n        return \"B\"\n    elif score >= 70:\n        return \"C\"\n    else:\n        return \"F\"\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_057_func_with_print() {
        let result = transpile("def debug(msg: str) -> str:\n    print(msg)\n    return msg\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_058_func_with_len() {
        let result = transpile("def size(items: list) -> int:\n    return len(items)\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_059_func_with_max_min() {
        let result = transpile(
            "def clamp(x: int, lo: int, hi: int) -> int:\n    return min(max(x, lo), hi)\n",
        );
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_060_func_with_abs() {
        let result = transpile("def magnitude(x: int) -> int:\n    return abs(x)\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_061_func_with_sum() {
        let result = transpile("def total(items: list) -> int:\n    return sum(items)\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_062_func_with_range_loop() {
        let result = transpile("def count(n: int) -> int:\n    total = 0\n    for i in range(n):\n        total = total + i\n    return total\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_063_func_with_range_start_stop() {
        let result = transpile("def range_sum(start: int, stop: int) -> int:\n    total = 0\n    for i in range(start, stop):\n        total = total + i\n    return total\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_064_func_with_range_step() {
        let result = transpile("def evens(n: int) -> list:\n    result = []\n    for i in range(0, n, 2):\n        result.append(i)\n    return result\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_065_func_string_methods() {
        let result = transpile("def process(s: str) -> str:\n    return s.strip().upper()\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_066_func_string_split() {
        let result = transpile("def tokenize(s: str) -> list:\n    return s.split(\" \")\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_067_func_string_join() {
        let result =
            transpile("def join_words(words: list) -> str:\n    return \" \".join(words)\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_068_func_string_replace() {
        let result =
            transpile("def clean(s: str) -> str:\n    return s.replace(\"old\", \"new\")\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_069_func_string_startswith() {
        let result = transpile(
            "def is_prefix(s: str, prefix: str) -> bool:\n    return s.startswith(prefix)\n",
        );
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_070_func_multiple_functions() {
        let result = transpile("def add(a: int, b: int) -> int:\n    return a + b\n\ndef sub(a: int, b: int) -> int:\n    return a - b\n\ndef mul(a: int, b: int) -> int:\n    return a * b\n");
        assert!(!result.is_empty());
        assert!(result.contains("add"));
        assert!(result.contains("sub"));
        assert!(result.contains("mul"));
    }

    #[test]
    fn test_w21fe_071_func_with_boolean_logic() {
        let result = transpile("def both(a: bool, b: bool) -> bool:\n    return a and b\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_072_func_with_not() {
        let result = transpile("def negate(a: bool) -> bool:\n    return not a\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_073_func_with_or() {
        let result = transpile("def either(a: bool, b: bool) -> bool:\n    return a or b\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_074_func_with_comparison_chain() {
        let result = transpile("def in_range(x: int) -> bool:\n    return 0 < x and x < 100\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_075_func_with_nested_calls() {
        let result = transpile("def process(x: int) -> int:\n    return abs(x) + len(str(x))\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_076_func_with_conditional_assign() {
        let result = transpile("def pick(x: int) -> str:\n    label = \"big\" if x > 100 else \"small\"\n    return label\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_077_func_with_list_index() {
        let result = transpile("def first(items: list) -> int:\n    return items[0]\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_078_func_with_negative_index() {
        let result = transpile("def last(items: list) -> int:\n    return items[-1]\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_079_func_two_param_with_default_none() {
        let result = transpile("def search(items: list, default = None):\n    if len(items) == 0:\n        return default\n    return items[0]\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_080_func_with_complex_body() {
        let result = transpile("def process(data: list) -> list:\n    result = []\n    for item in data:\n        if item > 0:\n            result.append(item * 2)\n        else:\n            result.append(0)\n    return result\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    // ========================================================================
    // SECTION 2: Expression generation deep coverage (tests 081-160)
    // ========================================================================

    #[test]
    fn test_w21fe_081_binop_add() {
        let result = transpile("def f(a: int, b: int) -> int:\n    return a + b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_082_binop_subtract() {
        let result = transpile("def f(a: int, b: int) -> int:\n    return a - b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_083_binop_multiply() {
        let result = transpile("def f(a: int, b: int) -> int:\n    return a * b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_084_binop_divide() {
        let result = transpile("def f(a: float, b: float) -> float:\n    return a / b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_085_binop_floor_divide() {
        let result = transpile("def f(a: int, b: int) -> int:\n    return a // b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_086_binop_modulo() {
        let result = transpile("def f(a: int, b: int) -> int:\n    return a % b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_087_binop_power() {
        let result = transpile("def f(a: int, b: int) -> int:\n    return a ** b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_088_binop_bitwise_and() {
        let result = transpile("def f(a: int, b: int) -> int:\n    return a & b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_089_binop_bitwise_or() {
        let result = transpile("def f(a: int, b: int) -> int:\n    return a | b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_090_binop_bitwise_xor() {
        let result = transpile("def f(a: int, b: int) -> int:\n    return a ^ b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_091_binop_left_shift() {
        let result = transpile("def f(a: int, b: int) -> int:\n    return a << b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_092_binop_right_shift() {
        let result = transpile("def f(a: int, b: int) -> int:\n    return a >> b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_093_cmpop_equal() {
        let result = transpile("def f(a: int, b: int) -> bool:\n    return a == b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_094_cmpop_not_equal() {
        let result = transpile("def f(a: int, b: int) -> bool:\n    return a != b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_095_cmpop_less_than() {
        let result = transpile("def f(a: int, b: int) -> bool:\n    return a < b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_096_cmpop_greater_than() {
        let result = transpile("def f(a: int, b: int) -> bool:\n    return a > b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_097_cmpop_less_equal() {
        let result = transpile("def f(a: int, b: int) -> bool:\n    return a <= b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_098_cmpop_greater_equal() {
        let result = transpile("def f(a: int, b: int) -> bool:\n    return a >= b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_099_cmpop_is() {
        let result = transpile("def f(x) -> bool:\n    return x is None\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_100_cmpop_is_not() {
        let result = transpile("def f(x) -> bool:\n    return x is not None\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_101_cmpop_in() {
        let result = transpile("def f(x: int, items: list) -> bool:\n    return x in items\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_102_cmpop_not_in() {
        let result = transpile("def f(x: int, items: list) -> bool:\n    return x not in items\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_103_unaryop_negate() {
        let result = transpile("def f(x: int) -> int:\n    return -x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_104_unaryop_positive() {
        let result = transpile("def f(x: int) -> int:\n    return +x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_105_unaryop_bitwise_not() {
        let result = transpile("def f(x: int) -> int:\n    return ~x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_106_unaryop_not() {
        let result = transpile("def f(x: bool) -> bool:\n    return not x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_107_ternary_conditional() {
        let result = transpile("def f(x: int) -> str:\n    return \"pos\" if x > 0 else \"neg\"\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_108_ternary_nested() {
        let result = transpile("def f(x: int) -> str:\n    return \"pos\" if x > 0 else \"zero\" if x == 0 else \"neg\"\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_109_fstring_simple_var() {
        let result = transpile("def f(name: str) -> str:\n    return f\"hello {name}\"\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_110_fstring_with_expression() {
        let result = transpile("def f(x: int) -> str:\n    return f\"value is {x + 1}\"\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_111_fstring_with_format_spec_float() {
        let result = transpile("def f(x: float) -> str:\n    return f\"{x:.2f}\"\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_112_fstring_with_format_spec_align() {
        let result = transpile("def f(x: int) -> str:\n    return f\"{x:>10}\"\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_113_fstring_multiple_parts() {
        let result =
            transpile("def f(a: str, b: int) -> str:\n    return f\"{a} has {b} items\"\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_114_list_comprehension_basic() {
        let result = transpile("def f(n: int) -> list:\n    return [x * 2 for x in range(n)]\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_115_list_comprehension_with_filter() {
        let result =
            transpile("def f(items: list) -> list:\n    return [x for x in items if x > 0]\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_116_list_comprehension_with_transform_and_filter() {
        let result =
            transpile("def f(items: list) -> list:\n    return [x * 2 for x in items if x > 0]\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_117_dict_comprehension() {
        let result =
            transpile("def f(keys: list) -> dict:\n    return {k: len(k) for k in keys}\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_118_set_comprehension() {
        let result = transpile("def f(items: list) -> set:\n    return {x * 2 for x in items}\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_119_generator_expression_sum() {
        let result = transpile("def f(n: int) -> int:\n    return sum(x * x for x in range(n))\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_120_nested_comprehension() {
        let result =
            transpile("def f() -> list:\n    return [i + j for i in range(3) for j in range(3)]\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_121_lambda_basic() {
        let result =
            transpile("def f() -> int:\n    double = lambda x: x * 2\n    return double(5)\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_122_lambda_multi_arg() {
        let result =
            transpile("def f() -> int:\n    add = lambda x, y: x + y\n    return add(3, 4)\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_123_lambda_in_sort_key() {
        let result = transpile(
            "def f(items: list) -> list:\n    items.sort(key=lambda x: x)\n    return items\n",
        );
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_124_slice_start_stop() {
        let result = transpile("def f(items: list) -> list:\n    return items[1:3]\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_125_slice_with_step() {
        let result = transpile("def f(items: list) -> list:\n    return items[::2]\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_126_slice_reverse() {
        let result = transpile("def f(items: list) -> list:\n    return items[::-1]\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_127_slice_start_stop_step() {
        let result = transpile("def f(items: list) -> list:\n    return items[1:10:2]\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_128_slice_from_start() {
        let result = transpile("def f(items: list) -> list:\n    return items[:5]\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_129_slice_to_end() {
        let result = transpile("def f(items: list) -> list:\n    return items[3:]\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_130_walrus_operator() {
        let result = transpile("def f(items: list) -> int:\n    if (n := len(items)) > 0:\n        return n\n    return 0\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_131_chained_comparison_and() {
        let result = transpile("def f(x: int) -> bool:\n    return 0 < x and x < 100\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_132_attribute_access() {
        let result = transpile("def f(s: str) -> int:\n    x = s.count(\"a\")\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_133_method_chaining_strip_lower() {
        let result = transpile("def f(s: str) -> str:\n    return s.strip().lower()\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_134_method_chaining_upper_strip() {
        let result = transpile("def f(s: str) -> str:\n    return s.upper().strip()\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_135_index_with_variable() {
        let result = transpile("def f(items: list, idx: int) -> int:\n    return items[idx]\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_136_negative_index_access() {
        let result = transpile("def f(items: list) -> int:\n    return items[-1]\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_137_negative_index_minus_two() {
        let result = transpile("def f(items: list) -> int:\n    return items[-2]\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_138_string_multiplication() {
        let result = transpile("def f() -> str:\n    return \"=\" * 80\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_139_boolean_and_or_combined() {
        let result =
            transpile("def f(a: bool, b: bool, c: bool) -> bool:\n    return a and b or c\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_140_boolean_complex_expression() {
        let result = transpile(
            "def f(x: int, y: int) -> bool:\n    return (x > 0 and y > 0) or (x < 0 and y < 0)\n",
        );
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_141_binop_add_float() {
        let result = transpile("def f(a: float, b: float) -> float:\n    return a + b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_142_binop_mixed_int_ops() {
        let result =
            transpile("def f(a: int, b: int, c: int) -> int:\n    return (a + b) * c - a\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_143_string_format_method() {
        let result = transpile("def f(name: str) -> str:\n    return \"hello {}\".format(name)\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_144_string_in_check() {
        let result = transpile("def f(s: str) -> bool:\n    return \"hello\" in s\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_145_list_concatenation() {
        let result = transpile("def f(a: list, b: list) -> list:\n    return a + b\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_146_list_multiplication() {
        let result = transpile("def f() -> list:\n    return [0] * 10\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_147_nested_function_calls() {
        let result = transpile("def f(s: str) -> int:\n    return int(str(len(s)))\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_148_comprehension_string_method() {
        let result =
            transpile("def f(words: list) -> list:\n    return [w.upper() for w in words]\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_149_comprehension_with_len() {
        let result = transpile("def f(words: list) -> list:\n    return [len(w) for w in words]\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_150_dict_access() {
        let result = transpile("def f(d: dict, key: str) -> int:\n    return d[key]\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_151_dict_get_method() {
        let result = transpile("def f(d: dict, key: str) -> int:\n    return d.get(key, 0)\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_152_multiple_augmented_ops() {
        let result =
            transpile("def f(x: int) -> int:\n    x += 1\n    x -= 2\n    x *= 3\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_153_tuple_unpacking_in_assignment() {
        let result = transpile("def f() -> int:\n    a, b = 1, 2\n    return a + b\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_154_expression_parenthesized() {
        let result = transpile("def f(a: int, b: int) -> int:\n    return (a + b) * (a - b)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21fe_155_fstring_with_call() {
        let result = transpile("def f(items: list) -> str:\n    return f\"count: {len(items)}\"\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_156_comparison_with_string() {
        let result = transpile("def f(s: str) -> bool:\n    return s == \"hello\"\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_157_list_empty_check() {
        let result = transpile("def f(items: list) -> bool:\n    return len(items) == 0\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_158_dict_keys_call() {
        let result = transpile("def f(d: dict) -> list:\n    return list(d.keys())\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_159_dict_values_call() {
        let result = transpile("def f(d: dict) -> list:\n    return list(d.values())\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_160_complex_expression_chain() {
        let result = transpile(
            "def f(x: int, y: int, z: int) -> bool:\n    return x > 0 and y > 0 and z > 0\n",
        );
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    // ========================================================================
    // SECTION 3: Type inference deep coverage (tests 161-200)
    // ========================================================================

    #[test]
    fn test_w21fe_161_infer_int_literal() {
        let result = transpile("def f() -> int:\n    x = 42\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_162_infer_float_literal() {
        let result = transpile("def f() -> float:\n    x = 3.14\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_163_infer_string_literal() {
        let result = transpile("def f() -> str:\n    x = \"hello\"\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_164_infer_bool_literal_true() {
        let result = transpile("def f() -> bool:\n    x = True\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_165_infer_bool_literal_false() {
        let result = transpile("def f() -> bool:\n    x = False\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_166_infer_list_literal_int() {
        let result = transpile("def f() -> list:\n    x = [1, 2, 3]\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_167_infer_list_literal_str() {
        let result = transpile("def f() -> list:\n    x = [\"a\", \"b\", \"c\"]\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_168_infer_dict_literal() {
        let result = transpile("def f() -> dict:\n    x = {\"a\": 1, \"b\": 2}\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_169_infer_set_literal() {
        let result = transpile("def f() -> set:\n    x = {1, 2, 3}\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_170_infer_tuple_literal() {
        let result = transpile("def f() -> tuple:\n    x = (1, \"hello\")\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_171_infer_from_function_call_len() {
        let result = transpile("def f(s: str) -> int:\n    x = len(s)\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_172_infer_from_function_call_str() {
        let result = transpile("def f(n: int) -> str:\n    x = str(n)\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_173_infer_from_method_call_upper() {
        let result = transpile("def f(s: str) -> str:\n    x = s.upper()\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_174_infer_from_method_call_split() {
        let result = transpile("def f(s: str) -> list:\n    x = s.split()\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_175_infer_from_binary_op_add_int() {
        let result = transpile("def f(a: int, b: int) -> int:\n    x = a + b\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_176_infer_from_binary_op_concat_str() {
        let result = transpile("def f(a: str, b: str) -> str:\n    x = a + b\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_177_infer_from_comparison() {
        let result = transpile("def f(a: int, b: int) -> bool:\n    x = a > b\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_178_infer_from_comprehension() {
        let result =
            transpile("def f() -> list:\n    x = [i * 2 for i in range(10)]\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_179_infer_through_assignment_chain() {
        let result = transpile("def f() -> int:\n    a = 10\n    b = a\n    c = b\n    return c\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_180_type_annotation_override_int() {
        let result = transpile("def f() -> int:\n    x: int = 0\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_181_type_annotation_override_str() {
        let result = transpile("def f() -> str:\n    x: str = \"\"\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_182_type_annotation_list_int() {
        let result = transpile(
            "from typing import List\ndef f() -> List[int]:\n    x: List[int] = []\n    return x\n",
        );
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_183_optional_type_from_none_default() {
        let result = transpile("from typing import Optional\ndef f(x: Optional[int] = None) -> int:\n    if x is None:\n        return 0\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_184_infer_float_division() {
        let result = transpile("def f(a: int, b: int) -> float:\n    return a / b\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_185_infer_floor_division() {
        let result = transpile("def f(a: int, b: int) -> int:\n    x = a // b\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_186_infer_modulo() {
        let result = transpile("def f(a: int, b: int) -> int:\n    x = a % b\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_187_infer_power() {
        let result = transpile("def f(a: int, b: int) -> int:\n    x = a ** b\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_188_infer_negation() {
        let result = transpile("def f(x: int) -> int:\n    y = -x\n    return y\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_189_infer_bool_logic_and() {
        let result = transpile("def f(a: bool, b: bool) -> bool:\n    x = a and b\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_190_infer_bool_logic_or() {
        let result = transpile("def f(a: bool, b: bool) -> bool:\n    x = a or b\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_191_infer_int_from_range_sum() {
        let result = transpile("def f(n: int) -> int:\n    return sum(range(n))\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_192_infer_str_from_join() {
        let result = transpile("def f(items: list) -> str:\n    return \",\".join(items)\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_193_infer_dict_from_comprehension() {
        let result = transpile("def f() -> dict:\n    return {str(i): i for i in range(5)}\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_194_infer_set_from_comprehension() {
        let result = transpile("def f() -> set:\n    return {i * 2 for i in range(10)}\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_195_infer_type_through_conditional() {
        let result = transpile("def f(x: int) -> int:\n    y = x if x > 0 else -x\n    return y\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_196_infer_type_from_abs() {
        let result = transpile("def f(x: int) -> int:\n    y = abs(x)\n    return y\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_197_infer_type_from_max() {
        let result = transpile("def f(a: int, b: int) -> int:\n    y = max(a, b)\n    return y\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_198_infer_type_from_min() {
        let result = transpile("def f(a: int, b: int) -> int:\n    y = min(a, b)\n    return y\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_199_infer_type_from_sorted() {
        let result = transpile("def f(items: list) -> list:\n    return sorted(items)\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w21fe_200_infer_type_from_reversed() {
        let result = transpile("def f(items: list) -> list:\n    return list(reversed(items))\n");
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }
}
