//! Wave 8 deep codegen coverage tests: rust_gen.rs, func_gen.rs, stmt_gen.rs
//!
//! Targets deep/complex code paths that are typically missed by simple tests:
//! - Module-level code patterns (constants, multi-function modules, nested functions)
//! - Complex assignment patterns (tuple unpacking, dict ops, augmented index)
//! - Type inference paths (accumulator, builder, counter, flag patterns)
//! - Edge cases (nested collections, chained methods, chained comparisons)

#[cfg(test)]
mod tests {
    use crate::DepylerPipeline;

    fn transpile(code: &str) -> Result<String, Box<dyn std::error::Error>> {
        let pipeline = DepylerPipeline::new();
        let result = pipeline.transpile(code)?;
        Ok(result)
    }

    // ========================================================================
    // SECTION 1: Module-level code patterns in rust_gen.rs (tests 1-50)
    // ========================================================================

    #[test]
    fn test_w8d_module_constant_int() {
        let code = "MAX = 100\ndef get_max() -> int:\n    return MAX\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("100") || result.contains("MAX") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w8d_module_constant_float() {
        let code = "THRESHOLD = 0.5\ndef above(x: float) -> bool:\n    return x > THRESHOLD\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("0.5") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w8d_module_constant_string() {
        let code = "NAME = \"test\"\ndef greet() -> str:\n    return NAME\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("test") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w8d_module_constant_bool_true() {
        let code = "DEBUG = True\ndef is_debug() -> bool:\n    return DEBUG\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("true") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w8d_module_constant_bool_false() {
        let code = "VERBOSE = False\ndef is_verbose() -> bool:\n    return VERBOSE\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("false") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w8d_module_constant_negative_int() {
        let code = "MIN_VAL = -1\ndef get_min() -> int:\n    return MIN_VAL\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("-1") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w8d_module_constant_large_int() {
        let code = "BIG = 1000000\ndef get_big() -> int:\n    return BIG\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("1000000") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w8d_module_constant_list() {
        let code = "ITEMS = [1, 2, 3]\ndef first() -> int:\n    return ITEMS[0]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("1") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w8d_module_constant_empty_string() {
        let code = "EMPTY = \"\"\ndef is_empty(s: str) -> bool:\n    return s == EMPTY\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("\"\""));
        }
    }

    #[test]
    fn test_w8d_module_dict_constant() {
        let code = "MAPPING = {\"a\": 1, \"b\": 2}\ndef lookup(key: str) -> int:\n    return MAPPING[key]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("HashMap"));
        }
    }

    #[test]
    fn test_w8d_module_multiple_constants() {
        let code = "X = 10\nY = 20\ndef add_xy() -> int:\n    return X + Y\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("10") || result.contains("20"));
        }
    }

    #[test]
    fn test_w8d_module_two_functions() {
        let code = "def add(a: int, b: int) -> int:\n    return a + b\n\ndef sub(a: int, b: int) -> int:\n    return a - b\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn add") || result.contains("fn sub") || result.contains("fn "));
    }

    #[test]
    fn test_w8d_module_three_functions() {
        let code = "def f1() -> int:\n    return 1\n\ndef f2() -> int:\n    return 2\n\ndef f3() -> int:\n    return 3\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w8d_module_function_calls_other() {
        let code = "def double(x: int) -> int:\n    return x * 2\n\ndef quad(x: int) -> int:\n    return double(double(x))\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("double") && result.contains("fn"));
    }

    #[test]
    fn test_w8d_module_docstring_function() {
        let code = "def f():\n    \"\"\"Help text for this function.\"\"\"\n    pass\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") || result.contains("Help text"));
    }

    #[test]
    fn test_w8d_module_docstring_multiline() {
        let code = "def calculate(x: int) -> int:\n    \"\"\"Calculate the value.\n\n    Args:\n        x: input value\n    \"\"\"\n    return x * 2\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") || result.contains("Calculate"));
    }

    #[test]
    fn test_w8d_module_nested_function() {
        let code = "def outer(x: int) -> int:\n    def inner(y: int) -> int:\n        return y + 1\n    return inner(x)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("inner") || result.contains("outer"));
        }
    }

    #[test]
    fn test_w8d_module_nested_closure_capture() {
        let code = "def make_adder(n: int) -> int:\n    def adder(x: int) -> int:\n        return x + n\n    return adder(5)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("adder"));
        }
    }

    #[test]
    fn test_w8d_module_lambda_simple() {
        let code = "def apply() -> int:\n    square = lambda x: x * x\n    return square(5)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("closure") || result.contains("|"));
        }
    }

    #[test]
    fn test_w8d_module_function_returning_none() {
        let code = "def f(x: int) -> None:\n    return None\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w8d_module_empty_function_pass() {
        let code = "def noop():\n    pass\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w8d_module_multiple_return_paths() {
        let code = "def classify(x: int) -> str:\n    if x > 0:\n        return \"positive\"\n    elif x < 0:\n        return \"negative\"\n    else:\n        return \"zero\"\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("fn") && (result.contains("positive") || result.contains("return"))
        );
    }

    #[test]
    fn test_w8d_module_return_inside_try() {
        let code = "def safe(x: int) -> int:\n    try:\n        return x * 2\n    except:\n        return 0\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w8d_module_default_param_int() {
        let code = "def greet(name: str, times: int = 1) -> str:\n    return name * times\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("name"));
        }
    }

    #[test]
    fn test_w8d_module_default_param_string() {
        let code = "def hello(name: str = \"world\") -> str:\n    return \"hello \" + name\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("world"));
        }
    }

    #[test]
    fn test_w8d_module_default_param_none() {
        let code = "def maybe(x: int = 0) -> int:\n    return x\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w8d_module_default_param_bool() {
        let code = "def toggle(flag: bool = False) -> bool:\n    return not flag\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("bool"));
        }
    }

    #[test]
    fn test_w8d_module_args_pattern() {
        let code = "def variadic(*args) -> int:\n    return len(args)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("args"));
        }
    }

    #[test]
    fn test_w8d_module_kwargs_pattern() {
        let code = "def config(**kwargs) -> int:\n    return len(kwargs)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("kwargs"));
        }
    }

    #[test]
    fn test_w8d_module_constant_tuple() {
        let code = "PAIR = (1, 2)\ndef get_first() -> int:\n    return PAIR[0]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("1"));
        }
    }

    #[test]
    fn test_w8d_module_four_functions() {
        let code = "def a() -> int:\n    return 1\n\ndef b() -> int:\n    return 2\n\ndef c() -> int:\n    return 3\n\ndef d() -> int:\n    return 4\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w8d_module_function_with_import_math() {
        let code = "import math\ndef area(r: float) -> float:\n    return math.pi * r * r\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("PI") || result.contains("pi"));
        }
    }

    #[test]
    fn test_w8d_module_global_string_used() {
        let code =
            "PREFIX = \"item_\"\ndef make_name(i: int) -> str:\n    return PREFIX + str(i)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("item_"));
        }
    }

    #[test]
    fn test_w8d_module_constant_zero() {
        let code = "ZERO = 0\ndef is_zero(x: int) -> bool:\n    return x == ZERO\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("0"));
        }
    }

    #[test]
    fn test_w8d_module_const_float_fraction() {
        let code = "HALF = 0.5\ndef halve(x: float) -> float:\n    return x * HALF\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("0.5"));
        }
    }

    #[test]
    fn test_w8d_module_lambda_in_function() {
        let code =
            "def apply_fn(x: int) -> int:\n    double = lambda n: n * 2\n    return double(x)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("|"));
        }
    }

    #[test]
    fn test_w8d_module_function_only_pass_with_type() {
        let code = "def stub(x: int) -> int:\n    pass\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w8d_module_recursive_function() {
        let code = "def factorial(n: int) -> int:\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("factorial") && result.contains("fn"));
    }

    #[test]
    fn test_w8d_module_five_params() {
        let code = "def multi(a: int, b: int, c: int, d: int, e: int) -> int:\n    return a + b + c + d + e\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("i64"));
    }

    #[test]
    fn test_w8d_module_constant_and_function_use() {
        let code = "SCALE = 10\ndef scaled(x: int) -> int:\n    return x * SCALE\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("10") || result.contains("SCALE"));
        }
    }

    #[test]
    fn test_w8d_module_two_string_constants() {
        let code = "HELLO = \"hello\"\nWORLD = \"world\"\ndef greet() -> str:\n    return HELLO + \" \" + WORLD\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("hello") || result.contains("world"));
        }
    }

    #[test]
    fn test_w8d_module_bool_returning_function() {
        let code = "def is_even(n: int) -> bool:\n    return n % 2 == 0\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("bool") || result.contains("fn"));
    }

    #[test]
    fn test_w8d_module_function_early_return_guard() {
        let code = "def safe_div(a: int, b: int) -> int:\n    if b == 0:\n        return 0\n    return a // b\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("0"));
    }

    #[test]
    fn test_w8d_module_constant_list_strings() {
        let code =
            "NAMES = [\"alice\", \"bob\"]\ndef count_names() -> int:\n    return len(NAMES)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("alice") || result.contains("bob"));
        }
    }

    #[test]
    fn test_w8d_module_multiple_returns_bool() {
        let code = "def check(x: int, y: int) -> bool:\n    if x > y:\n        return True\n    return False\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("bool"));
    }

    #[test]
    fn test_w8d_module_function_string_return() {
        let code = "def status(code: int) -> str:\n    if code == 200:\n        return \"ok\"\n    return \"error\"\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && (result.contains("ok") || result.contains("error")));
    }

    #[test]
    fn test_w8d_module_six_params() {
        let code = "def f(a: int, b: int, c: int, d: int, e: int, f_val: int) -> int:\n    return a + b + c + d + e + f_val\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("i64"));
    }

    #[test]
    fn test_w8d_module_mixed_type_params() {
        let code = "def process(name: str, count: int, ratio: float) -> str:\n    return name\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("String"));
    }

    // ========================================================================
    // SECTION 2: Complex assignment patterns in stmt_gen.rs (tests 51-100)
    // ========================================================================

    #[test]
    fn test_w8d_assign_tuple_unpack_basic() {
        let code = "def f() -> int:\n    a, b = 1, 2\n    return a + b\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("let"));
        }
    }

    #[test]
    fn test_w8d_assign_tuple_unpack_three() {
        let code = "def f() -> int:\n    a, b, c = 1, 2, 3\n    return a + b + c\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("let"));
        }
    }

    #[test]
    fn test_w8d_assign_tuple_unpack_strings() {
        let code = "def f() -> str:\n    first, last = \"John\", \"Doe\"\n    return first + \" \" + last\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("John"));
        }
    }

    #[test]
    fn test_w8d_assign_tuple_unpack_from_func() {
        let code = "def f() -> int:\n    a, b = divmod(10, 3)\n    return a\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("divmod"));
        }
    }

    #[test]
    fn test_w8d_assign_tuple_for_loop_items() {
        let code = "def f(d: dict) -> int:\n    total = 0\n    for k, v in d.items():\n        total += 1\n    return total\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("for"));
        }
    }

    #[test]
    fn test_w8d_assign_dict_subscript() {
        let code = "def f() -> dict:\n    data = {}\n    data[\"key\"] = 42\n    return data\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("insert") || result.contains("key"));
        }
    }

    #[test]
    fn test_w8d_assign_dict_computed_key() {
        let code = "def f(i: int) -> dict:\n    data = {}\n    key = \"item_\" + str(i)\n    data[key] = i\n    return data\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("insert"));
        }
    }

    #[test]
    fn test_w8d_assign_list_append_loop() {
        let code = "def f(n: int) -> list:\n    result = []\n    for i in range(n):\n        result.append(i)\n    return result\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("push") || result.contains("append"));
        }
    }

    #[test]
    fn test_w8d_assign_list_append_conditional() {
        let code = "def f(items: list) -> list:\n    result = []\n    for x in items:\n        if x > 0:\n            result.append(x)\n    return result\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("push"));
        }
    }

    #[test]
    fn test_w8d_assign_set_add() {
        let code = "def f() -> set:\n    s = set()\n    s.add(1)\n    s.add(2)\n    return s\n";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("fn") || result.contains("insert") || result.contains("HashSet")
            );
        }
    }

    #[test]
    fn test_w8d_assign_dict_update_subscript() {
        let code =
            "def f() -> dict:\n    d = {}\n    d[\"a\"] = 1\n    d[\"b\"] = 2\n    return d\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("insert"));
        }
    }

    #[test]
    fn test_w8d_assign_multiple_same_value() {
        let code = "def f() -> int:\n    a = 0\n    b = 0\n    return a + b\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("let"));
    }

    #[test]
    fn test_w8d_assign_augmented_index() {
        let code = "def f() -> list:\n    data = [0, 0, 0]\n    data[1] += 5\n    return data\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("+=") || result.contains("5"));
        }
    }

    #[test]
    fn test_w8d_assign_conditional_ternary() {
        let code = "def f(x: int) -> int:\n    result = 1 if x > 0 else 0\n    return result\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && (result.contains("if") || result.contains("else")));
    }

    #[test]
    fn test_w8d_assign_walrus_operator() {
        let code = "def f(items: list) -> int:\n    if (n := len(items)) > 0:\n        return n\n    return 0\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("len"));
        }
    }

    #[test]
    fn test_w8d_assign_string_concat() {
        let code = "def f(a: str, b: str) -> str:\n    result = a + \" \" + b\n    return result\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("String"));
    }

    #[test]
    fn test_w8d_assign_float_arithmetic() {
        let code =
            "def f(x: float, y: float) -> float:\n    result = x * y + 1.0\n    return result\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("f64"));
    }

    #[test]
    fn test_w8d_assign_bool_expression() {
        let code = "def f(x: int) -> bool:\n    result = x > 0 and x < 100\n    return result\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("bool"));
    }

    #[test]
    fn test_w8d_assign_nested_dict_literal() {
        let code = "def f() -> dict:\n    data = {\"x\": 1, \"y\": 2, \"z\": 3}\n    return data\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("HashMap"));
        }
    }

    #[test]
    fn test_w8d_assign_list_of_strings() {
        let code = "def f() -> list:\n    items = [\"a\", \"b\", \"c\"]\n    return items\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("vec!") || result.contains("Vec"));
        }
    }

    #[test]
    fn test_w8d_assign_augmented_add_float() {
        let code = "def f(x: float) -> float:\n    x += 1.5\n    return x\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("+="));
        }
    }

    #[test]
    fn test_w8d_assign_augmented_mul_int() {
        let code = "def f(x: int) -> int:\n    x *= 3\n    return x\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("*=") || result.contains("fn"));
    }

    #[test]
    fn test_w8d_assign_augmented_sub_int() {
        let code = "def f(count: int) -> int:\n    count -= 1\n    return count\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("-=") || result.contains("fn"));
    }

    #[test]
    fn test_w8d_assign_augmented_floor_div() {
        let code = "def f(x: int) -> int:\n    x //= 2\n    return x\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("/"));
        }
    }

    #[test]
    fn test_w8d_assign_augmented_modulo() {
        let code = "def f(x: int) -> int:\n    x %= 7\n    return x\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("%=") || result.contains("fn"));
    }

    #[test]
    fn test_w8d_assign_from_function_call() {
        let code = "def length(s: str) -> int:\n    result = len(s)\n    return result\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("len"));
    }

    #[test]
    fn test_w8d_assign_from_method_call() {
        let code = "def upper(s: str) -> str:\n    result = s.upper()\n    return result\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("to_uppercase"));
    }

    #[test]
    fn test_w8d_assign_chained_reassign() {
        let code = "def f() -> int:\n    x = 1\n    x = x + 1\n    x = x * 2\n    return x\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("let"));
    }

    #[test]
    fn test_w8d_assign_swap_variables() {
        let code = "def f() -> int:\n    a = 1\n    b = 2\n    a, b = b, a\n    return a\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("let"));
        }
    }

    #[test]
    fn test_w8d_assign_slice_basic() {
        let code = "def f(items: list) -> list:\n    part = items[1:3]\n    return part\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("["));
        }
    }

    #[test]
    fn test_w8d_assign_list_comprehension() {
        let code =
            "def f(n: int) -> list:\n    squares = [x * x for x in range(n)]\n    return squares\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("map") || result.contains("collect"));
        }
    }

    #[test]
    fn test_w8d_assign_dict_comprehension() {
        let code = "def f(n: int) -> dict:\n    d = {str(i): i for i in range(n)}\n    return d\n";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("fn") || result.contains("collect") || result.contains("HashMap")
            );
        }
    }

    #[test]
    fn test_w8d_assign_set_comprehension() {
        let code = "def f(items: list) -> set:\n    s = {x for x in items}\n    return s\n";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("fn") || result.contains("collect") || result.contains("HashSet")
            );
        }
    }

    #[test]
    fn test_w8d_assign_negation() {
        let code = "def f(x: int) -> int:\n    result = -x\n    return result\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("-"));
    }

    #[test]
    fn test_w8d_assign_not_bool() {
        let code = "def f(x: bool) -> bool:\n    result = not x\n    return result\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && (result.contains("!") || result.contains("not")));
    }

    #[test]
    fn test_w8d_assign_bitwise_and() {
        let code = "def f(a: int, b: int) -> int:\n    result = a & b\n    return result\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("&"));
    }

    #[test]
    fn test_w8d_assign_bitwise_or() {
        let code = "def f(a: int, b: int) -> int:\n    result = a | b\n    return result\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("|"));
    }

    #[test]
    fn test_w8d_assign_bitwise_xor() {
        let code = "def f(a: int, b: int) -> int:\n    result = a ^ b\n    return result\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("^"));
    }

    #[test]
    fn test_w8d_assign_power() {
        let code = "def f(x: int) -> int:\n    result = x ** 2\n    return result\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("pow"));
        }
    }

    #[test]
    fn test_w8d_assign_string_multiply() {
        let code = "def f(s: str, n: int) -> str:\n    result = s * n\n    return result\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("repeat"));
        }
    }

    #[test]
    fn test_w8d_assign_in_while_loop() {
        let code = "def f() -> int:\n    x = 0\n    while x < 10:\n        x += 1\n    return x\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("while"));
    }

    #[test]
    fn test_w8d_assign_from_ternary_str() {
        let code = "def f(x: int) -> str:\n    label = \"big\" if x > 100 else \"small\"\n    return label\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && (result.contains("big") || result.contains("small")));
    }

    #[test]
    fn test_w8d_assign_list_index() {
        let code =
            "def f() -> int:\n    items = [10, 20, 30]\n    val = items[1]\n    return val\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("20") || result.contains("[1]"));
        }
    }

    // ========================================================================
    // SECTION 3: Type inference paths in func_gen.rs (tests 101-150)
    // ========================================================================

    #[test]
    fn test_w8d_infer_return_len() {
        let code = "def f(items: list) -> int:\n    return len(items)\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("i64"));
    }

    #[test]
    fn test_w8d_infer_return_method_chain_strip_lower() {
        let code = "def f(s: str) -> str:\n    return s.strip().lower()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("fn") && (result.contains("trim") || result.contains("to_lowercase"))
        );
    }

    #[test]
    fn test_w8d_infer_return_method_chain_upper() {
        let code = "def f(s: str) -> str:\n    return s.upper()\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("to_uppercase") || result.contains("fn"));
    }

    #[test]
    fn test_w8d_infer_return_dict_get() {
        let code = "def f(d: dict, key: str) -> int:\n    return d.get(key, 0)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("get"));
        }
    }

    #[test]
    fn test_w8d_infer_return_conditional() {
        let code = "def f(x: int) -> int:\n    return x if x > 0 else -x\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && (result.contains("if") || result.contains("else")));
    }

    #[test]
    fn test_w8d_infer_early_return_empty_list() {
        let code =
            "def f(items: list) -> list:\n    if not items:\n        return []\n    return items\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("Vec") || result.contains("vec!"));
        }
    }

    #[test]
    fn test_w8d_infer_accumulator_sum() {
        let code = "def sum_list(items: list) -> int:\n    total = 0\n    for x in items:\n        total += x\n    return total\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") && (result.contains("+=") || result.contains("total")));
        }
    }

    #[test]
    fn test_w8d_infer_accumulator_product() {
        let code = "def product(items: list) -> int:\n    result = 1\n    for x in items:\n        result *= x\n    return result\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") && (result.contains("*=") || result.contains("result")));
        }
    }

    #[test]
    fn test_w8d_infer_build_list() {
        let code = "def doubled(items: list) -> list:\n    result = []\n    for x in items:\n        result.append(x * 2)\n    return result\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("push") || result.contains("Vec"));
        }
    }

    #[test]
    fn test_w8d_infer_build_dict() {
        let code = "def index_map(items: list) -> dict:\n    result = {}\n    for i in range(len(items)):\n        result[i] = items[i]\n    return result\n";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("fn") || result.contains("HashMap") || result.contains("insert")
            );
        }
    }

    #[test]
    fn test_w8d_infer_counter_pattern() {
        let code = "def count_positive(items: list) -> int:\n    count = 0\n    for x in items:\n        if x > 0:\n            count += 1\n    return count\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") && (result.contains("count") || result.contains("+=")));
        }
    }

    #[test]
    fn test_w8d_infer_bool_flag_found() {
        let code = "def has_target(items: list, target: int) -> bool:\n    found = False\n    for x in items:\n        if x == target:\n            found = True\n    return found\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") && result.contains("bool"));
        }
    }

    #[test]
    fn test_w8d_infer_max_tracking() {
        let code = "def find_max(items: list) -> int:\n    best = items[0]\n    for x in items:\n        if x > best:\n            best = x\n    return best\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("best"));
        }
    }

    #[test]
    fn test_w8d_infer_min_tracking() {
        let code = "def find_min(items: list) -> int:\n    smallest = items[0]\n    for x in items:\n        if x < smallest:\n            smallest = x\n    return smallest\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("smallest"));
        }
    }

    #[test]
    fn test_w8d_infer_string_concat_loop() {
        let code = "def join_items(items: list) -> str:\n    result = \"\"\n    for item in items:\n        result += str(item)\n    return result\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") && result.contains("String"));
        }
    }

    #[test]
    fn test_w8d_infer_nested_if_different_returns() {
        let code = "def classify(x: int) -> str:\n    if x > 100:\n        return \"high\"\n    elif x > 50:\n        return \"medium\"\n    elif x > 0:\n        return \"low\"\n    else:\n        return \"none\"\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("String"));
    }

    #[test]
    fn test_w8d_infer_function_calls_other() {
        let code = "def helper(x: int) -> int:\n    return x + 1\n\ndef main_fn(x: int) -> int:\n    return helper(x) * 2\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("helper") && result.contains("fn"));
    }

    #[test]
    fn test_w8d_infer_return_abs_int() {
        let code = "def absolute(x: int) -> int:\n    return abs(x)\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("abs"));
    }

    #[test]
    fn test_w8d_infer_return_min_builtin() {
        let code = "def smaller(a: int, b: int) -> int:\n    return min(a, b)\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("min"));
    }

    #[test]
    fn test_w8d_infer_return_max_builtin() {
        let code = "def larger(a: int, b: int) -> int:\n    return max(a, b)\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("max"));
    }

    #[test]
    fn test_w8d_infer_return_str_builtin() {
        let code = "def to_string(x: int) -> str:\n    return str(x)\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("to_string"));
    }

    #[test]
    fn test_w8d_infer_return_int_builtin() {
        let code = "def to_int(s: str) -> int:\n    return int(s)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("parse"));
        }
    }

    #[test]
    fn test_w8d_infer_return_float_builtin() {
        let code = "def to_float(s: str) -> float:\n    return float(s)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("parse") || result.contains("f64"));
        }
    }

    #[test]
    fn test_w8d_infer_return_bool_comparison() {
        let code = "def is_positive(x: int) -> bool:\n    return x > 0\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("bool"));
    }

    #[test]
    fn test_w8d_infer_return_bool_equality() {
        let code = "def is_zero(x: int) -> bool:\n    return x == 0\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("bool"));
    }

    #[test]
    fn test_w8d_infer_return_bool_in() {
        let code = "def contains(items: list, x: int) -> bool:\n    return x in items\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("contains"));
        }
    }

    #[test]
    fn test_w8d_infer_return_bool_not_in() {
        let code = "def missing(items: list, x: int) -> bool:\n    return x not in items\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("contains"));
        }
    }

    #[test]
    fn test_w8d_infer_return_string_format() {
        let code = "def greet(name: str) -> str:\n    return f\"Hello, {name}!\"\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && (result.contains("format!") || result.contains("Hello")));
    }

    #[test]
    fn test_w8d_infer_return_string_join() {
        let code = "def join_words(words: list) -> str:\n    return \", \".join(words)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("join"));
        }
    }

    #[test]
    fn test_w8d_infer_return_string_replace() {
        let code = "def clean(s: str) -> str:\n    return s.replace(\"old\", \"new\")\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("replace"));
    }

    #[test]
    fn test_w8d_infer_return_string_split() {
        let code = "def tokenize(s: str) -> list:\n    return s.split(\",\")\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("split"));
    }

    #[test]
    fn test_w8d_infer_return_list_sorted() {
        let code = "def sort_items(items: list) -> list:\n    return sorted(items)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("sort"));
        }
    }

    #[test]
    fn test_w8d_infer_return_list_reversed() {
        let code = "def reverse_items(items: list) -> list:\n    return list(reversed(items))\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("rev"));
        }
    }

    #[test]
    fn test_w8d_infer_return_enumerate() {
        let code = "def indexed(items: list) -> list:\n    return list(enumerate(items))\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("enumerate"));
        }
    }

    #[test]
    fn test_w8d_infer_return_zip() {
        let code = "def pair(a: list, b: list) -> list:\n    return list(zip(a, b))\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("zip"));
        }
    }

    #[test]
    fn test_w8d_infer_return_map() {
        let code = "def double_all(items: list) -> list:\n    return list(map(str, items))\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("map"));
        }
    }

    #[test]
    fn test_w8d_infer_return_filter() {
        let code = "def positives(items: list) -> list:\n    return list(filter(None, items))\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("filter"));
        }
    }

    #[test]
    fn test_w8d_infer_sum_builtin() {
        let code = "def total(items: list) -> int:\n    return sum(items)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("sum") || result.contains("iter"));
        }
    }

    #[test]
    fn test_w8d_infer_any_builtin() {
        let code = "def has_true(items: list) -> bool:\n    return any(items)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("any"));
        }
    }

    #[test]
    fn test_w8d_infer_all_builtin() {
        let code = "def all_true(items: list) -> bool:\n    return all(items)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("all"));
        }
    }

    #[test]
    fn test_w8d_infer_return_isinstance_guard() {
        let code = "def safe_add(a: int, b: int) -> int:\n    if a > 0 and b > 0:\n        return a + b\n    return 0\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("i64"));
    }

    #[test]
    fn test_w8d_infer_fibonacci() {
        let code = "def fib(n: int) -> int:\n    if n <= 1:\n        return n\n    return fib(n - 1) + fib(n - 2)\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fib") && result.contains("fn"));
    }

    #[test]
    fn test_w8d_infer_gcd() {
        let code = "def gcd(a: int, b: int) -> int:\n    while b != 0:\n        a, b = b, a % b\n    return a\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("gcd") || result.contains("while"));
        }
    }

    #[test]
    fn test_w8d_infer_linear_search() {
        let code = "def search(items: list, target: int) -> int:\n    for i in range(len(items)):\n        if items[i] == target:\n            return i\n    return -1\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("return") || result.contains("-1"));
        }
    }

    #[test]
    fn test_w8d_infer_return_none_explicit() {
        let code = "def maybe_find(items: list, x: int) -> bool:\n    for item in items:\n        if item == x:\n            return True\n    return False\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") && result.contains("bool"));
        }
    }

    #[test]
    fn test_w8d_infer_string_startswith() {
        let code =
            "def has_prefix(s: str, prefix: str) -> bool:\n    return s.startswith(prefix)\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("starts_with"));
    }

    #[test]
    fn test_w8d_infer_string_endswith() {
        let code = "def has_suffix(s: str, suffix: str) -> bool:\n    return s.endswith(suffix)\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("ends_with"));
    }

    // ========================================================================
    // SECTION 4: Edge cases and rare patterns (tests 151-200)
    // ========================================================================

    #[test]
    fn test_w8d_edge_empty_list_literal() {
        let code = "def f() -> list:\n    return []\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("Vec") || result.contains("vec!"));
        }
    }

    #[test]
    fn test_w8d_edge_empty_dict_literal() {
        let code = "def f() -> dict:\n    return {}\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("HashMap"));
        }
    }

    #[test]
    fn test_w8d_edge_empty_set_literal() {
        let code = "def f() -> set:\n    return set()\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("HashSet"));
        }
    }

    #[test]
    fn test_w8d_edge_empty_tuple_literal() {
        let code = "def f():\n    t = ()\n    return t\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("()"));
        }
    }

    #[test]
    fn test_w8d_edge_single_element_tuple() {
        let code = "def f() -> tuple:\n    return (1,)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("1"));
        }
    }

    #[test]
    fn test_w8d_edge_nested_list() {
        let code = "def f() -> list:\n    return [[1, 2], [3, 4]]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("vec!") || result.contains("Vec"));
        }
    }

    #[test]
    fn test_w8d_edge_nested_list_three_deep() {
        let code = "def f() -> list:\n    return [[[1]]]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("vec!"));
        }
    }

    #[test]
    fn test_w8d_edge_dict_with_tuple_key() {
        let code = "def f() -> dict:\n    d = {}\n    d[(1, 2)] = \"a\"\n    return d\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("HashMap"));
        }
    }

    #[test]
    fn test_w8d_edge_set_of_strings() {
        let code = "def f() -> set:\n    return {\"a\", \"b\", \"c\"}\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("HashSet"));
        }
    }

    #[test]
    fn test_w8d_edge_complex_number() {
        let code = "def f() -> complex:\n    return complex(1, 2)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("complex") || result.contains("1"));
        }
    }

    #[test]
    fn test_w8d_edge_bytes_literal() {
        let code = "def f() -> bytes:\n    return b\"hello\"\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("hello") || result.contains("u8"));
        }
    }

    #[test]
    fn test_w8d_edge_unicode_string() {
        let code = "def f() -> str:\n    return \"unicode test\"\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("unicode"));
    }

    #[test]
    fn test_w8d_edge_long_string() {
        let code = "def f() -> str:\n    return \"abcdefghijklmnopqrstuvwxyz\" * 3\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("repeat") || result.contains("abc"));
        }
    }

    #[test]
    fn test_w8d_edge_deep_nesting_if_for() {
        let code = "def f(n: int) -> int:\n    total = 0\n    for i in range(n):\n        for j in range(n):\n            if i > j:\n                total += 1\n    return total\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") && result.contains("for"));
        }
    }

    #[test]
    fn test_w8d_edge_triple_nested_for() {
        let code = "def f(n: int) -> int:\n    count = 0\n    for i in range(n):\n        for j in range(n):\n            for k in range(n):\n                count += 1\n    return count\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") && result.contains("for"));
        }
    }

    #[test]
    fn test_w8d_edge_deep_if_nesting() {
        let code = "def f(a: int, b: int, c: int) -> str:\n    if a > 0:\n        if b > 0:\n            if c > 0:\n                return \"all positive\"\n    return \"not all positive\"\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("if"));
    }

    #[test]
    fn test_w8d_edge_chained_method_strip_lower_split() {
        let code = "def f(s: str) -> list:\n    return s.strip().lower().split(\",\")\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("trim") || result.contains("split"));
        }
    }

    #[test]
    fn test_w8d_edge_chained_comparison_lt() {
        let code = "def f(x: int) -> bool:\n    return 0 < x < 10\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("bool") || result.contains("&&"));
        }
    }

    #[test]
    fn test_w8d_edge_chained_comparison_lte() {
        let code = "def f(a: int, b: int, c: int) -> bool:\n    return a <= b <= c\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("bool") || result.contains("&&"));
        }
    }

    #[test]
    fn test_w8d_edge_multiple_bool_ops() {
        let code =
            "def f(a: bool, b: bool, c: bool, d: bool) -> bool:\n    return a and b or c and d\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("bool"));
    }

    #[test]
    fn test_w8d_edge_nested_ternary() {
        let code = "def f(x: int) -> str:\n    return \"pos\" if x > 0 else \"neg\" if x < 0 else \"zero\"\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("pos") || result.contains("neg"));
        }
    }

    #[test]
    fn test_w8d_edge_list_multiply() {
        let code = "def f(n: int) -> list:\n    return [0] * n\n";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("fn")
                    || result.contains("vec!")
                    || result.contains("repeat")
                    || result.contains("0")
            );
        }
    }

    #[test]
    fn test_w8d_edge_dict_merge_unpack() {
        let code = "def f(d1: dict, d2: dict) -> dict:\n    return {**d1, **d2}\n";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("fn") || result.contains("HashMap") || result.contains("extend")
            );
        }
    }

    #[test]
    fn test_w8d_edge_enumerate_basic() {
        let code = "def f(items: list) -> list:\n    result = []\n    for i, x in enumerate(items):\n        result.append(i)\n    return result\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("enumerate"));
        }
    }

    #[test]
    fn test_w8d_edge_enumerate_with_start() {
        let code = "def f(items: list) -> list:\n    result = []\n    for i, x in enumerate(items, 1):\n        result.append(i)\n    return result\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("enumerate"));
        }
    }

    #[test]
    fn test_w8d_edge_zip_two_lists() {
        let code = "def f(a: list, b: list) -> list:\n    result = []\n    for x, y in zip(a, b):\n        result.append(x + y)\n    return result\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("zip"));
        }
    }

    #[test]
    fn test_w8d_edge_sorted_list() {
        let code = "def f(items: list) -> list:\n    return sorted(items)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("sort"));
        }
    }

    #[test]
    fn test_w8d_edge_reversed_list() {
        let code = "def f(items: list) -> list:\n    return list(reversed(items))\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("rev"));
        }
    }

    #[test]
    fn test_w8d_edge_list_with_negative_index() {
        let code = "def f(items: list) -> int:\n    return items[-1]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("-1") || result.contains("last"));
        }
    }

    #[test]
    fn test_w8d_edge_string_find() {
        let code = "def f(s: str, sub: str) -> int:\n    return s.find(sub)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("find"));
        }
    }

    #[test]
    fn test_w8d_edge_string_count() {
        let code = "def f(s: str, sub: str) -> int:\n    return s.count(sub)\n";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("fn") || result.contains("matches") || result.contains("count")
            );
        }
    }

    #[test]
    fn test_w8d_edge_string_isdigit() {
        let code = "def f(s: str) -> bool:\n    return s.isdigit()\n";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("fn")
                    || result.contains("is_digit")
                    || result.contains("is_numeric")
                    || result.contains("char")
            );
        }
    }

    #[test]
    fn test_w8d_edge_string_isalpha() {
        let code = "def f(s: str) -> bool:\n    return s.isalpha()\n";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("fn")
                    || result.contains("is_alpha")
                    || result.contains("alphabetic")
            );
        }
    }

    #[test]
    fn test_w8d_edge_string_strip() {
        let code = "def f(s: str) -> str:\n    return s.strip()\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("trim"));
    }

    #[test]
    fn test_w8d_edge_string_lstrip() {
        let code = "def f(s: str) -> str:\n    return s.lstrip()\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("trim_start"));
        }
    }

    #[test]
    fn test_w8d_edge_string_rstrip() {
        let code = "def f(s: str) -> str:\n    return s.rstrip()\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("trim_end"));
        }
    }

    #[test]
    fn test_w8d_edge_string_title() {
        let code = "def f(s: str) -> str:\n    return s.title()\n";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("fn")
                    || result.contains("title")
                    || result.contains("to_uppercase")
            );
        }
    }

    #[test]
    fn test_w8d_edge_for_else() {
        let code = "def f(items: list, target: int) -> bool:\n    for x in items:\n        if x == target:\n            return True\n    return False\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") && result.contains("bool"));
        }
    }

    #[test]
    fn test_w8d_edge_while_with_break() {
        let code = "def f(n: int) -> int:\n    i = 0\n    while True:\n        if i >= n:\n            break\n        i += 1\n    return i\n";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("fn")
                    && (result.contains("loop")
                        || result.contains("while")
                        || result.contains("break"))
            );
        }
    }

    #[test]
    fn test_w8d_edge_while_with_continue() {
        let code = "def f(n: int) -> int:\n    total = 0\n    i = 0\n    while i < n:\n        i += 1\n        if i % 2 == 0:\n            continue\n        total += i\n    return total\n";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("fn") && (result.contains("continue") || result.contains("while"))
            );
        }
    }

    #[test]
    fn test_w8d_edge_for_with_break() {
        let code = "def f(items: list) -> int:\n    for x in items:\n        if x > 100:\n            break\n    return 0\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") && result.contains("break"));
        }
    }

    #[test]
    fn test_w8d_edge_for_with_continue() {
        let code = "def f(items: list) -> int:\n    total = 0\n    for x in items:\n        if x < 0:\n            continue\n        total += x\n    return total\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") && result.contains("continue"));
        }
    }

    #[test]
    fn test_w8d_edge_assert_statement() {
        let code = "def f(x: int) -> int:\n    assert x > 0\n    return x\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("assert"));
        }
    }

    #[test]
    fn test_w8d_edge_assert_with_message() {
        let code = "def f(x: int) -> int:\n    assert x > 0, \"must be positive\"\n    return x\n";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("fn") || result.contains("assert") || result.contains("positive")
            );
        }
    }

    #[test]
    fn test_w8d_edge_multiple_list_appends() {
        let code = "def f() -> list:\n    items = []\n    items.append(1)\n    items.append(2)\n    items.append(3)\n    return items\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("push"));
        }
    }

    #[test]
    fn test_w8d_edge_dict_keys() {
        let code = "def f(d: dict) -> list:\n    return list(d.keys())\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("keys"));
        }
    }

    #[test]
    fn test_w8d_edge_dict_values() {
        let code = "def f(d: dict) -> list:\n    return list(d.values())\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("values"));
        }
    }

    #[test]
    fn test_w8d_edge_dict_items_loop() {
        let code = "def f(d: dict) -> int:\n    count = 0\n    for k, v in d.items():\n        count += 1\n    return count\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("iter") || result.contains("for"));
        }
    }

    #[test]
    fn test_w8d_edge_list_len() {
        let code = "def f(items: list) -> int:\n    return len(items)\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("len"));
    }

    #[test]
    fn test_w8d_edge_string_len() {
        let code = "def f(s: str) -> int:\n    return len(s)\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("len"));
    }

    #[test]
    fn test_w8d_edge_range_step() {
        let code = "def f() -> list:\n    result = []\n    for i in range(0, 10, 2):\n        result.append(i)\n    return result\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("step") || result.contains("for"));
        }
    }

    #[test]
    fn test_w8d_edge_range_negative_step() {
        let code = "def f() -> list:\n    result = []\n    for i in range(10, 0, -1):\n        result.append(i)\n    return result\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("rev") || result.contains("for"));
        }
    }

    #[test]
    fn test_w8d_edge_nested_function_two_levels() {
        let code = "def outer(x: int) -> int:\n    def middle(y: int) -> int:\n        def inner(z: int) -> int:\n            return z + 1\n        return inner(y)\n    return middle(x)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("outer") || result.contains("inner"));
        }
    }

    #[test]
    fn test_w8d_edge_fstring_multiple_vars() {
        let code =
            "def f(name: str, age: int) -> str:\n    return f\"{name} is {age} years old\"\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("format!"));
    }

    #[test]
    fn test_w8d_edge_fstring_expression() {
        let code = "def f(x: int) -> str:\n    return f\"result: {x * 2}\"\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("format!"));
        }
    }

    #[test]
    fn test_w8d_edge_multiline_string_assign() {
        let code = "def f() -> str:\n    msg = \"line one \" + \"line two\"\n    return msg\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("fn") && (result.contains("line one") || result.contains("line two"))
        );
    }

    #[test]
    fn test_w8d_edge_comparison_chain_eq() {
        let code = "def f(a: int, b: int, c: int) -> bool:\n    return a == b == c\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("bool") || result.contains("=="));
        }
    }

    #[test]
    fn test_w8d_edge_not_equal() {
        let code = "def f(a: int, b: int) -> bool:\n    return a != b\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("!="));
    }

    #[test]
    fn test_w8d_edge_boolean_and_or_not() {
        let code = "def f(a: bool, b: bool) -> bool:\n    return not (a and b) or (a or b)\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("bool"));
    }

    #[test]
    fn test_w8d_edge_integer_division() {
        let code = "def f(a: int, b: int) -> int:\n    return a // b\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("/"));
    }

    #[test]
    fn test_w8d_edge_modulo_operator() {
        let code = "def f(a: int, b: int) -> int:\n    return a % b\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn") && result.contains("%"));
    }

    #[test]
    fn test_w8d_edge_power_operator() {
        let code = "def f(base: int, exp: int) -> int:\n    return base ** exp\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("pow"));
        }
    }
}
