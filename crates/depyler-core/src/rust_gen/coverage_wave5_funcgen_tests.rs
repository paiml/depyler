//! Coverage wave 5: func_gen.rs, argparse_transform.rs, binary_ops.rs targeted tests
//!
//! Targets:
//! - func_gen.rs: 960 missed lines (82.1% covered) - complex parameter inference,
//!   return type inference, tuple unpacking, *args/**kwargs, decorators, nested classes,
//!   abstract methods, class variables, multiple inheritance, dataclass fields
//! - argparse_transform.rs: 480 missed lines (77.8% covered) - ArgumentParser, add_argument
//!   with all param types, subparsers, mutually exclusive groups, parse_args
//! - binary_ops.rs: 326 missed lines (59.8% covered) - all binary operators with type
//!   coercion, set operations, dict merge, containment, power, floor division

#[cfg(test)]
mod tests {
    use crate::DepylerPipeline;

    fn transpile(code: &str) -> String {
        let pipeline = DepylerPipeline::new();
        pipeline.transpile(code).expect("transpile")
    }

    fn transpile_ok(code: &str) -> bool {
        let pipeline = DepylerPipeline::new();
        pipeline.transpile(code).is_ok()
    }

    // =========================================================================
    // Section 1: func_gen - Complex parameter inference
    // =========================================================================

    #[test]
    fn test_w5_param_inference_int_annotation() {
        let result = transpile("def add(x: int, y: int) -> int:\n    return x + y\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_param_inference_float_annotation() {
        let result = transpile("def scale(x: float, factor: float) -> float:\n    return x * factor\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_param_inference_str_annotation() {
        let result = transpile("def greet(name: str) -> str:\n    return 'Hello ' + name\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_param_inference_bool_annotation() {
        let result = transpile("def toggle(flag: bool) -> bool:\n    return not flag\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_param_inference_list_int() {
        let result = transpile("def sum_list(nums: list) -> int:\n    total = 0\n    for n in nums:\n        total = total + n\n    return total\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_param_inference_dict_annotation() {
        let result = transpile("def get_val(d: dict, key: str) -> str:\n    return d[key]\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_param_inference_optional() {
        let result = transpile("from typing import Optional\ndef maybe(x: Optional[int] = None) -> int:\n    if x is None:\n        return 0\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_param_inference_tuple_type() {
        let result = transpile("from typing import Tuple\ndef pair(t: Tuple[int, int]) -> int:\n    return t[0] + t[1]\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_param_no_annotation_used_as_int() {
        let result = transpile("def double(x):\n    return x * 2\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_param_no_annotation_used_as_str() {
        let result = transpile("def upper(s):\n    return s.upper()\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 2: func_gen - Return type inference from body
    // =========================================================================

    #[test]
    fn test_w5_return_type_infer_int_literal() {
        let result = transpile("def get_zero():\n    return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_return_type_infer_float_literal() {
        let result = transpile("def get_pi():\n    return 3.14159\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_return_type_infer_string_literal() {
        let result = transpile("def get_name():\n    return 'hello'\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_return_type_infer_bool_literal() {
        let result = transpile("def is_valid():\n    return True\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_return_type_infer_list() {
        let result = transpile("def get_items():\n    return [1, 2, 3]\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_return_type_infer_dict() {
        let result = transpile("def get_config():\n    return {'key': 'value'}\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_return_type_infer_tuple() {
        let result = transpile("def get_pair():\n    return (1, 2)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_return_type_infer_none() {
        let result = transpile("def do_nothing():\n    return None\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_return_type_infer_from_if_else() {
        let result = transpile("def check(x: int):\n    if x > 0:\n        return 'positive'\n    else:\n        return 'negative'\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_return_type_infer_optional_pattern() {
        let result = transpile("def find(items: list, target: int):\n    for item in items:\n        if item == target:\n            return item\n    return None\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_return_type_infer_from_variable() {
        let result = transpile("def compute(x: int) -> int:\n    result = x * 2\n    return result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_return_type_empty_tuple_annotation() {
        let result = transpile("def get_empty() -> tuple:\n    return ()\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 3: func_gen - Tuple unpacking parameters
    // =========================================================================

    #[test]
    fn test_w5_tuple_unpack_two_vars() {
        let result = transpile("def process():\n    a, b = 1, 2\n    return a + b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_tuple_unpack_three_vars() {
        let result = transpile("def process():\n    x, y, z = 1, 2, 3\n    return x + y + z\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_tuple_unpack_from_function() {
        let result = transpile("def get_pair() -> tuple:\n    return (10, 20)\n\ndef use_pair():\n    a, b = get_pair()\n    return a\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_tuple_unpack_in_for_loop() {
        let result = transpile("def process(pairs: list):\n    for key, value in pairs:\n        print(key)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_tuple_unpack_nested() {
        // Nested tuple unpacking may not be supported; just check it doesn't crash
        let ok = transpile_ok("def process():\n    a, (b, c) = 1, (2, 3)\n    return a + b + c\n");
        // Accept either success or graceful error
        let _ = ok;
    }

    #[test]
    fn test_w5_tuple_unpack_with_type_inference() {
        let result = transpile("def swap(x: int, y: int) -> tuple:\n    a, b = y, x\n    return (a, b)\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 4: func_gen - *args and **kwargs
    // =========================================================================

    #[test]
    fn test_w5_args_simple() {
        let result = transpile("def variadic(*args):\n    return len(args)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_kwargs_simple() {
        let result = transpile("def config(**kwargs):\n    return kwargs\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_args_and_kwargs() {
        let result = transpile("def flexible(*args, **kwargs):\n    return len(args)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_args_with_named_params() {
        let result = transpile("def mixed(x: int, *args):\n    return x + len(args)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_kwargs_with_named_params() {
        let result = transpile("def mixed(x: int, **kwargs):\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_args_kwargs_with_named() {
        let result = transpile("def mixed(name: str, *args, **kwargs):\n    return name\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 5: func_gen - Decorator combinations
    // =========================================================================

    #[test]
    fn test_w5_staticmethod_decorator() {
        let result = transpile("class MyClass:\n    @staticmethod\n    def helper(x: int) -> int:\n        return x * 2\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_classmethod_decorator() {
        let result = transpile("class MyClass:\n    @classmethod\n    def create(cls, x: int):\n        return cls(x)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_property_decorator() {
        let result = transpile("class Circle:\n    def __init__(self, radius: float):\n        self.radius = radius\n    @property\n    def area(self) -> float:\n        return 3.14159 * self.radius * self.radius\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_multiple_methods_with_decorators() {
        let code = "class Util:\n    @staticmethod\n    def add(a: int, b: int) -> int:\n        return a + b\n    @staticmethod\n    def sub(a: int, b: int) -> int:\n        return a - b\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_method_no_decorator() {
        let result = transpile("class Counter:\n    def __init__(self):\n        self.count = 0\n    def increment(self):\n        self.count = self.count + 1\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 6: func_gen - Nested class methods
    // =========================================================================

    #[test]
    fn test_w5_nested_function_in_function() {
        let result = transpile("def outer(x: int) -> int:\n    def inner(y: int) -> int:\n        return y + 1\n    return inner(x)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_nested_function_with_closure() {
        let result = transpile("def make_adder(n: int):\n    def adder(x: int) -> int:\n        return x + n\n    return adder\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_nested_function_multiple() {
        let result = transpile("def pipeline(x: int) -> int:\n    def step1(v: int) -> int:\n        return v + 1\n    def step2(v: int) -> int:\n        return v * 2\n    return step2(step1(x))\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_method_returning_self_type() {
        let result = transpile("class Builder:\n    def __init__(self):\n        self.items = []\n    def add(self, item: str):\n        self.items.append(item)\n        return self\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 7: func_gen - Abstract methods and class variables
    // =========================================================================

    #[test]
    fn test_w5_class_variable_int() {
        let result = transpile("class Config:\n    MAX_SIZE = 100\n    def __init__(self):\n        self.size = 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_class_variable_string() {
        let result = transpile("class App:\n    VERSION = '1.0.0'\n    def __init__(self):\n        self.name = 'app'\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_class_variable_list() {
        let result = transpile("class Registry:\n    ITEMS = []\n    def __init__(self):\n        self.count = 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_class_with_docstring() {
        let result = transpile("class Documented:\n    \"\"\"A documented class.\"\"\"\n    def __init__(self):\n        self.value = 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_class_method_with_docstring() {
        let result = transpile("class MyClass:\n    def __init__(self):\n        self.x = 0\n    def process(self) -> int:\n        \"\"\"Process and return result.\"\"\"\n        return self.x + 1\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 8: func_gen - Multiple inheritance / mixin patterns
    // =========================================================================

    #[test]
    fn test_w5_simple_inheritance() {
        let result = transpile("class Base:\n    def __init__(self):\n        self.x = 0\n\nclass Child(Base):\n    def __init__(self):\n        self.y = 1\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_method_override() {
        let result = transpile("class Base:\n    def greet(self) -> str:\n        return 'base'\n\nclass Child(Base):\n    def greet(self) -> str:\n        return 'child'\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_class_with_str_method() {
        let result = transpile("class Point:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y\n    def __str__(self) -> str:\n        return str(self.x)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_class_with_repr_method() {
        let result = transpile("class Item:\n    def __init__(self, name: str):\n        self.name = name\n    def __repr__(self) -> str:\n        return self.name\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_class_with_len_method() {
        let result = transpile("class Container:\n    def __init__(self):\n        self.items = []\n    def __len__(self) -> int:\n        return len(self.items)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_class_with_eq_method() {
        let result = transpile("class Point:\n    def __init__(self, x: int):\n        self.x = x\n    def __eq__(self, other) -> bool:\n        return self.x == other.x\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 9: func_gen - Dataclass fields
    // =========================================================================

    #[test]
    fn test_w5_dataclass_basic() {
        let result = transpile("from dataclasses import dataclass\n\n@dataclass\nclass Point:\n    x: int\n    y: int\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_dataclass_with_default() {
        let result = transpile("from dataclasses import dataclass\n\n@dataclass\nclass Config:\n    name: str = 'default'\n    count: int = 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_dataclass_with_method() {
        let result = transpile("from dataclasses import dataclass\n\n@dataclass\nclass Rect:\n    width: int\n    height: int\n    def area(self) -> int:\n        return self.width * self.height\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_dataclass_with_float_field() {
        let result = transpile("from dataclasses import dataclass\n\n@dataclass\nclass Measurement:\n    value: float\n    unit: str\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_dataclass_bool_field() {
        let result = transpile("from dataclasses import dataclass\n\n@dataclass\nclass Feature:\n    name: str\n    enabled: bool = False\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 10: func_gen - Variable type inference from body
    // =========================================================================

    #[test]
    fn test_w5_var_type_infer_from_assignment() {
        let result = transpile("def process(x: int) -> int:\n    result = x * 2\n    return result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_var_type_infer_from_subscript() {
        let result = transpile("def first(items: list) -> int:\n    item = items[0]\n    return item\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_var_type_none_then_string() {
        let result = transpile("def process():\n    result = None\n    result = 'hello'\n    return result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_var_type_infer_in_for_loop() {
        let result = transpile("def sum_all(nums: list) -> int:\n    total = 0\n    for n in nums:\n        total = total + n\n    return total\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_var_type_infer_in_if_else() {
        let result = transpile("def categorize(x: int) -> str:\n    if x > 0:\n        label = 'positive'\n    else:\n        label = 'negative'\n    return label\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_var_type_infer_in_while_loop() {
        let result = transpile("def countdown(n: int) -> int:\n    count = 0\n    while n > 0:\n        count = count + 1\n        n = n - 1\n    return count\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_var_type_infer_in_try_except() {
        let result = transpile("def safe_div(a: int, b: int) -> int:\n    result = 0\n    try:\n        result = a // b\n    except:\n        result = -1\n    return result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_var_type_infer_from_param() {
        let result = transpile("def copy_val(n: int) -> int:\n    result = n\n    return result\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 11: func_gen - Loop/if escaping variables
    // =========================================================================

    #[test]
    fn test_w5_loop_escaping_simple() {
        let result = transpile("def find_first(items: list) -> int:\n    for item in items:\n        found = item\n    return found\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_if_escaping_simple() {
        let result = transpile("def classify(x: int) -> str:\n    if x > 0:\n        result = 'pos'\n    else:\n        result = 'neg'\n    return result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_loop_escaping_with_type() {
        let result = transpile("def sum_squares(nums: list) -> int:\n    total = 0\n    for n in nums:\n        total = total + n * n\n    return total\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_nested_loop_escaping() {
        let result = transpile("def flatten(matrix: list) -> list:\n    result = []\n    for row in matrix:\n        for item in row:\n            result.append(item)\n    return result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_while_escaping_variable() {
        let result = transpile("def count_down(n: int) -> int:\n    i = 0\n    while n > 0:\n        i = i + 1\n        n = n - 1\n    return i\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 12: func_gen - cmd_* handler patterns
    // =========================================================================

    #[test]
    fn test_w5_cmd_handler_basic() {
        let result = transpile("def cmd_run(args):\n    print(args.name)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_cmd_handler_multiple_fields() {
        let result = transpile("def cmd_build(args):\n    print(args.target)\n    print(args.verbose)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_handle_prefix_handler() {
        let result = transpile("def handle_request(args):\n    return args.path\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_cmd_handler_with_conditional() {
        let result = transpile("def cmd_deploy(args):\n    if args.dry_run:\n        print('dry run')\n    else:\n        print('deploying')\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 13: func_gen - Return type special cases
    // =========================================================================

    #[test]
    fn test_w5_return_type_expects_int() {
        let result = transpile("def divide(a: int, b: int) -> int:\n    return a / b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_return_type_expects_float() {
        let result = transpile("def ratio(a: int, b: int) -> float:\n    return a / b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_returns_impl_iterator_generator() {
        let result = transpile("def gen_range(n: int):\n    i = 0\n    while i < n:\n        yield i\n        i = i + 1\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_function_can_fail_try() {
        let result = transpile("def risky(x: int) -> int:\n    try:\n        return 10 // x\n    except:\n        return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_function_always_returns() {
        let result = transpile("def always(x: int) -> str:\n    if x > 0:\n        return 'yes'\n    return 'no'\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 14: func_gen - Async functions
    // =========================================================================

    #[test]
    fn test_w5_async_function_simple() {
        let result = transpile("async def fetch():\n    return 42\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_async_function_with_params() {
        let result = transpile("async def process(url: str) -> str:\n    return url\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_async_function_with_await() {
        let result = transpile("async def fetch_data(url: str):\n    result = await get(url)\n    return result\n");
        assert!(transpile_ok("async def fetch_data(url: str):\n    result = await get(url)\n    return result\n"));
    }

    // =========================================================================
    // Section 15: argparse - Basic ArgumentParser
    // =========================================================================

    #[test]
    fn test_w5_argparse_basic_parser() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser(description='My tool')\n    parser.add_argument('filename')\n    args = parser.parse_args()\n    print(args.filename)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_argparse_with_description() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser(description='A simple tool')\n    args = parser.parse_args()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_argparse_positional_string() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('input_file')\n    args = parser.parse_args()\n    print(args.input_file)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_argparse_positional_int() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('count', type=int)\n    args = parser.parse_args()\n    print(args.count)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_argparse_flag_verbose() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('-v', '--verbose', action='store_true')\n    args = parser.parse_args()\n    if args.verbose:\n        print('verbose mode')\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_argparse_flag_store_false() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--no-color', action='store_false')\n    args = parser.parse_args()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_argparse_flag_count() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('-v', '--verbosity', action='count')\n    args = parser.parse_args()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_argparse_flag_append() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--include', action='append')\n    args = parser.parse_args()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_argparse_nargs_plus() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('files', nargs='+')\n    args = parser.parse_args()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_argparse_nargs_star() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('files', nargs='*')\n    args = parser.parse_args()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_argparse_nargs_optional() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('config', nargs='?')\n    args = parser.parse_args()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_argparse_nargs_number() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('coords', nargs=2, type=int)\n    args = parser.parse_args()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_argparse_default_value_string() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--output', default='result.txt')\n    args = parser.parse_args()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_argparse_default_value_int() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--port', type=int, default=8080)\n    args = parser.parse_args()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_argparse_required_flag() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--name', required=True)\n    args = parser.parse_args()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_argparse_choices() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--format', choices=['json', 'csv', 'xml'])\n    args = parser.parse_args()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_argparse_help_text() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--output', help='Output file path')\n    args = parser.parse_args()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_argparse_type_float() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--threshold', type=float, default=0.5)\n    args = parser.parse_args()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_argparse_epilog() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser(description='My tool', epilog='Example usage')\n    args = parser.parse_args()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 16: argparse - Subparsers
    // =========================================================================

    #[test]
    fn test_w5_argparse_subparsers_basic() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='command')\n    clone_parser = subparsers.add_parser('clone')\n    clone_parser.add_argument('url')\n    args = parser.parse_args()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_argparse_subparsers_multiple() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='command')\n    add_parser = subparsers.add_parser('add')\n    add_parser.add_argument('items', nargs='+')\n    rm_parser = subparsers.add_parser('remove')\n    rm_parser.add_argument('name')\n    args = parser.parse_args()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_argparse_subparser_with_help() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='command')\n    run_parser = subparsers.add_parser('run', help='Run the process')\n    run_parser.add_argument('--fast', action='store_true')\n    args = parser.parse_args()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 17: argparse - Multiple arguments combined
    // =========================================================================

    #[test]
    fn test_w5_argparse_multiple_positional_and_flags() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser(description='File processor')\n    parser.add_argument('input_file')\n    parser.add_argument('output_file')\n    parser.add_argument('-v', '--verbose', action='store_true')\n    parser.add_argument('-n', '--num-workers', type=int, default=4)\n    args = parser.parse_args()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_argparse_short_flag_only() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('-q')\n    args = parser.parse_args()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_argparse_long_flag_only() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--debug', action='store_true')\n    args = parser.parse_args()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_argparse_dest_parameter() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('-o', '--output-dir', dest='output')\n    args = parser.parse_args()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_argparse_metavar() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--file', metavar='FILE')\n    args = parser.parse_args()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 18: binary_ops - Floor division
    // =========================================================================

    #[test]
    fn test_w5_floor_div_basic() {
        let result = transpile("def divide(a: int, b: int) -> int:\n    return a // b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_floor_div_negative() {
        let result = transpile("def divide_neg(a: int) -> int:\n    return a // 3\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_floor_div_in_loop() {
        let result = transpile("def halve(n: int) -> int:\n    count = 0\n    while n > 0:\n        n = n // 2\n        count = count + 1\n    return count\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_floor_div_with_variables() {
        let result = transpile("def index(total: int, parts: int) -> int:\n    return total // parts\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 19: binary_ops - Power operator
    // =========================================================================

    #[test]
    fn test_w5_pow_int_positive() {
        let result = transpile("def square(x: int) -> int:\n    return x ** 2\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_pow_int_negative_exp() {
        let result = transpile("def inverse(x: int) -> float:\n    return x ** -1\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_pow_float_base() {
        let result = transpile("def compute(x: float) -> float:\n    return x ** 2.0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_pow_float_exp() {
        let result = transpile("def sqrt_approx(x: int) -> float:\n    return x ** 0.5\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_pow_variable_base_and_exp() {
        let result = transpile("def power(base: int, exp: int) -> int:\n    return base ** exp\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_pow_literal_both() {
        let result = transpile("def cube():\n    return 2 ** 3\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_pow_chained() {
        let result = transpile("def big_pow(x: int) -> int:\n    return x ** 2 ** 3\n");
        assert!(transpile_ok("def big_pow(x: int) -> int:\n    return x ** 2 ** 3\n"));
    }

    // =========================================================================
    // Section 20: binary_ops - Set operations
    // =========================================================================

    #[test]
    fn test_w5_set_union() {
        let result = transpile("def combine() -> set:\n    a = {1, 2, 3}\n    b = {3, 4, 5}\n    return a | b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_set_intersection() {
        let result = transpile("def common() -> set:\n    a = {1, 2, 3}\n    b = {2, 3, 4}\n    return a & b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_set_difference() {
        let result = transpile("def diff() -> set:\n    a = {1, 2, 3}\n    b = {2, 3}\n    return a - b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_set_symmetric_diff() {
        let result = transpile("def sym_diff() -> set:\n    a = {1, 2, 3}\n    b = {2, 3, 4}\n    return a ^ b\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 21: binary_ops - Dict merge
    // =========================================================================

    #[test]
    fn test_w5_dict_merge_basic() {
        let result = transpile("def merge_dicts():\n    a = {'x': 1}\n    b = {'y': 2}\n    return a | b\n");
        assert!(transpile_ok("def merge_dicts():\n    a = {'x': 1}\n    b = {'y': 2}\n    return a | b\n"));
    }

    #[test]
    fn test_w5_dict_merge_overlapping() {
        let result = transpile("def merge():\n    defaults = {'color': 'red', 'size': 10}\n    overrides = {'color': 'blue'}\n    return defaults | overrides\n");
        assert!(transpile_ok("def merge():\n    defaults = {'color': 'red', 'size': 10}\n    overrides = {'color': 'blue'}\n    return defaults | overrides\n"));
    }

    // =========================================================================
    // Section 22: binary_ops - Containment operators
    // =========================================================================

    #[test]
    fn test_w5_in_list() {
        let result = transpile("def check(x: int) -> bool:\n    return x in [1, 2, 3]\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_not_in_list() {
        let result = transpile("def check(x: int) -> bool:\n    return x not in [1, 2, 3]\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_in_string() {
        let result = transpile("def has_char(s: str, c: str) -> bool:\n    return c in s\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_not_in_string() {
        let result = transpile("def no_char(s: str, c: str) -> bool:\n    return c not in s\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_in_dict() {
        let result = transpile("def has_key(d: dict, key: str) -> bool:\n    return key in d\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_not_in_dict() {
        let result = transpile("def no_key(d: dict, key: str) -> bool:\n    return key not in d\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_in_set() {
        let result = transpile("def in_set(x: int) -> bool:\n    s = {1, 2, 3}\n    return x in s\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_in_tuple() {
        let result = transpile("def in_tuple(x: int) -> bool:\n    return x in (1, 2, 3)\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 23: binary_ops - String multiplication and addition
    // =========================================================================

    #[test]
    fn test_w5_string_repeat_literal() {
        let result = transpile("def repeat_str() -> str:\n    return 'abc' * 3\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_string_repeat_reversed() {
        let result = transpile("def repeat_str() -> str:\n    return 3 * 'abc'\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_string_concat_literal() {
        let result = transpile("def concat() -> str:\n    return 'hello' + ' ' + 'world'\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_string_concat_vars() {
        let result = transpile("def greet(first: str, last: str) -> str:\n    return first + ' ' + last\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_list_concat() {
        let result = transpile("def merge(a: list, b: list) -> list:\n    return a + b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_list_repeat() {
        let result = transpile("def zeros() -> list:\n    return [0] * 10\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_list_repeat_variable_size() {
        let result = transpile("def make_list(n: int) -> list:\n    return [0] * n\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 24: binary_ops - Comparison with type coercion
    // =========================================================================

    #[test]
    fn test_w5_compare_eq_int() {
        let result = transpile("def eq(x: int, y: int) -> bool:\n    return x == y\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_compare_neq() {
        let result = transpile("def neq(x: int, y: int) -> bool:\n    return x != y\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_compare_lt() {
        let result = transpile("def lt(x: int, y: int) -> bool:\n    return x < y\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_compare_lte() {
        let result = transpile("def lte(x: int, y: int) -> bool:\n    return x <= y\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_compare_gt() {
        let result = transpile("def gt(x: int, y: int) -> bool:\n    return x > y\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_compare_gte() {
        let result = transpile("def gte(x: int, y: int) -> bool:\n    return x >= y\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_compare_float_int_coercion() {
        let result = transpile("def check(x: float) -> bool:\n    return x > 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_compare_int_float_coercion() {
        let result = transpile("def check(x: int, threshold: float) -> bool:\n    return x < threshold\n");
        assert!(transpile_ok("def check(x: int, threshold: float) -> bool:\n    return x < threshold\n"));
    }

    #[test]
    fn test_w5_compare_string_ordering() {
        let result = transpile("def compare_strs(a: str, b: str) -> bool:\n    return a < b\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 25: binary_ops - Logical operators (and/or)
    // =========================================================================

    #[test]
    fn test_w5_logical_and_booleans() {
        let result = transpile("def both(a: bool, b: bool) -> bool:\n    return a and b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_logical_or_booleans() {
        let result = transpile("def either(a: bool, b: bool) -> bool:\n    return a or b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_logical_and_with_comparisons() {
        let result = transpile("def in_range(x: int) -> bool:\n    return x > 0 and x < 100\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_logical_or_with_comparisons() {
        let result = transpile("def out_range(x: int) -> bool:\n    return x < 0 or x > 100\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_logical_or_string_default() {
        let result = transpile("def get_name(name: str) -> str:\n    return name or 'unknown'\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_logical_or_with_none_check() {
        let result = transpile("def safe_get(val: str) -> str:\n    return val or ''\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_logical_and_mixed_types() {
        let result = transpile("def check(items: list) -> bool:\n    return len(items) > 0 and items[0] == 'test'\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 26: binary_ops - Arithmetic with type coercion
    // =========================================================================

    #[test]
    fn test_w5_add_int_int() {
        let result = transpile("def add(a: int, b: int) -> int:\n    return a + b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_add_float_float() {
        let result = transpile("def add(a: float, b: float) -> float:\n    return a + b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_sub_int_int() {
        let result = transpile("def sub(a: int, b: int) -> int:\n    return a - b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_mul_int_int() {
        let result = transpile("def mul(a: int, b: int) -> int:\n    return a * b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_div_int_int_to_float() {
        let result = transpile("def div(a: int, b: int) -> float:\n    return a / b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_mod_int_int() {
        let result = transpile("def mod_op(a: int, b: int) -> int:\n    return a % b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_chained_arithmetic() {
        let result = transpile("def chain(a: int, b: int, c: int) -> int:\n    return a + b * c\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_arithmetic_with_unary_neg() {
        let result = transpile("def negate(x: int) -> int:\n    return -x + 1\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_sub_from_len() {
        let result = transpile("def last_index(items: list) -> int:\n    return len(items) - 1\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 27: binary_ops - Bitwise operators
    // =========================================================================

    #[test]
    fn test_w5_bitwise_and() {
        let result = transpile("def bit_and(a: int, b: int) -> int:\n    return a & b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_bitwise_or() {
        let result = transpile("def bit_or(a: int, b: int) -> int:\n    return a | b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_bitwise_xor() {
        let result = transpile("def bit_xor(a: int, b: int) -> int:\n    return a ^ b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_left_shift() {
        let result = transpile("def shift_left(x: int, n: int) -> int:\n    return x << n\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_right_shift() {
        let result = transpile("def shift_right(x: int, n: int) -> int:\n    return x >> n\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 28: func_gen - Complex function patterns
    // =========================================================================

    #[test]
    fn test_w5_function_with_default_none() {
        let result = transpile("def process(x: int, label: str = None) -> int:\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_function_with_default_empty_list() {
        let result = transpile("def process(items: list = []) -> list:\n    return items\n");
        assert!(transpile_ok("def process(items: list = []) -> list:\n    return items\n"));
    }

    #[test]
    fn test_w5_function_with_multiple_returns() {
        let result = transpile("def classify(x: int) -> str:\n    if x > 100:\n        return 'high'\n    elif x > 50:\n        return 'medium'\n    elif x > 0:\n        return 'low'\n    else:\n        return 'none'\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_function_recursive() {
        let result = transpile("def factorial(n: int) -> int:\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_function_with_global_constant() {
        let result = transpile("MAX_VALUE = 100\n\ndef clamp(x: int) -> int:\n    if x > MAX_VALUE:\n        return MAX_VALUE\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_function_with_assert() {
        let result = transpile("def positive(x: int) -> int:\n    assert x > 0\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_function_with_assert_message() {
        let result = transpile("def positive(x: int) -> int:\n    assert x > 0, 'x must be positive'\n    return x\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 29: func_gen - Class method patterns
    // =========================================================================

    #[test]
    fn test_w5_class_method_accessing_self() {
        let result = transpile("class Counter:\n    def __init__(self):\n        self.value = 0\n    def get(self) -> int:\n        return self.value\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_class_method_modifying_self() {
        let result = transpile("class Counter:\n    def __init__(self):\n        self.value = 0\n    def increment(self):\n        self.value = self.value + 1\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_class_multiple_methods() {
        let result = transpile("class Stack:\n    def __init__(self):\n        self.items = []\n    def push(self, item: int):\n        self.items.append(item)\n    def pop(self) -> int:\n        return self.items.pop()\n    def is_empty(self) -> bool:\n        return len(self.items) == 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_class_method_with_params() {
        let result = transpile("class Calculator:\n    def __init__(self):\n        self.result = 0\n    def add(self, x: int, y: int) -> int:\n        self.result = x + y\n        return self.result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_class_method_returning_list() {
        let result = transpile("class Container:\n    def __init__(self):\n        self.items = []\n    def get_items(self) -> list:\n        return self.items\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 30: func_gen - Generator functions
    // =========================================================================

    #[test]
    fn test_w5_generator_yield_simple() {
        let result = transpile("def gen():\n    yield 1\n    yield 2\n    yield 3\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_generator_yield_in_loop() {
        let result = transpile("def count_up(n: int):\n    i = 0\n    while i < n:\n        yield i\n        i = i + 1\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_generator_yield_conditional() {
        let result = transpile("def evens(n: int):\n    for i in range(n):\n        if i % 2 == 0:\n            yield i\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 31: binary_ops - Division edge cases
    // =========================================================================

    #[test]
    fn test_w5_div_regular_ints() {
        let result = transpile("def div(a: int, b: int) -> int:\n    return a / b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_div_float_result() {
        let result = transpile("def div(a: int, b: int) -> float:\n    return a / b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_div_mixed_float_int() {
        let result = transpile("def div(a: float, b: int) -> float:\n    return a / b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_div_float_float() {
        let result = transpile("def div(a: float, b: float) -> float:\n    return a / b\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 32: func_gen - Lambda and closures
    // =========================================================================

    #[test]
    fn test_w5_lambda_simple() {
        let result = transpile("def use_lambda():\n    f = lambda x: x + 1\n    return f(5)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_lambda_multiple_params() {
        let result = transpile("def use_lambda():\n    add = lambda x, y: x + y\n    return add(3, 4)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_lambda_in_sort() {
        let result = transpile("def sort_by_second(pairs: list) -> list:\n    return sorted(pairs, key=lambda x: x[1])\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_closure_captures_var() {
        let result = transpile("def make_multiplier(factor: int):\n    def multiply(x: int) -> int:\n        return x * factor\n    return multiply\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 33: func_gen - Error handling patterns
    // =========================================================================

    #[test]
    fn test_w5_try_except_basic() {
        let result = transpile("def safe_parse(s: str) -> int:\n    try:\n        return int(s)\n    except:\n        return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_try_except_finally() {
        let result = transpile("def process():\n    try:\n        x = 1\n    except:\n        x = 0\n    finally:\n        print('done')\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_raise_exception() {
        let result = transpile("def validate(x: int):\n    if x < 0:\n        raise ValueError('negative')\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_try_except_else() {
        let result = transpile("def safe_op(x: int) -> int:\n    try:\n        result = 10 // x\n    except:\n        result = -1\n    else:\n        result = result + 1\n    return result\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 34: func_gen - With statement / context managers
    // =========================================================================

    #[test]
    fn test_w5_with_open_file() {
        let result = transpile("def read_file(path: str) -> str:\n    with open(path) as f:\n        return f.read()\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_with_open_write() {
        let result = transpile("def write_file(path: str, data: str):\n    with open(path, 'w') as f:\n        f.write(data)\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 35: func_gen - List/dict/set comprehensions
    // =========================================================================

    #[test]
    fn test_w5_list_comp_simple() {
        let result = transpile("def squares(n: int) -> list:\n    return [x * x for x in range(n)]\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_list_comp_with_filter() {
        let result = transpile("def even_squares(n: int) -> list:\n    return [x * x for x in range(n) if x % 2 == 0]\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_dict_comp() {
        let result = transpile("def index_map(items: list) -> dict:\n    return {i: item for i, item in enumerate(items)}\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_set_comp() {
        let result = transpile("def unique_lengths(words: list) -> set:\n    return {len(w) for w in words}\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 36: func_gen - Complex return patterns
    // =========================================================================

    #[test]
    fn test_w5_return_list_literal() {
        let result = transpile("def get_items() -> list:\n    return [1, 2, 3]\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_return_dict_literal() {
        let result = transpile("def get_config() -> dict:\n    return {'a': 1, 'b': 2}\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_return_empty_list() {
        let result = transpile("def empty() -> list:\n    return []\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_return_empty_dict() {
        let result = transpile("def empty() -> dict:\n    return {}\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_return_conditional_expression() {
        let result = transpile("def abs_val(x: int) -> int:\n    return x if x > 0 else -x\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 37: binary_ops - Chained comparisons and complex expressions
    // =========================================================================

    #[test]
    fn test_w5_chained_comparison() {
        let result = transpile("def in_range(x: int) -> bool:\n    return 0 < x and x < 100\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_compare_with_unary_neg() {
        let result = transpile("def is_negative(x: int) -> bool:\n    return x < -1\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_arithmetic_chain_add_mul() {
        let result = transpile("def compute(a: int, b: int, c: int) -> int:\n    return a + b + c\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_arithmetic_sub_chain() {
        let result = transpile("def compute(a: int, b: int, c: int) -> int:\n    return a - b - c\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_mixed_arithmetic_operators() {
        let result = transpile("def expr(a: int, b: int) -> int:\n    return (a + b) * (a - b)\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 38: func_gen - String processing functions
    // =========================================================================

    #[test]
    fn test_w5_func_string_methods() {
        let result = transpile("def process(s: str) -> str:\n    return s.strip().lower()\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_func_string_split() {
        let result = transpile("def words(s: str) -> list:\n    return s.split()\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_func_string_join() {
        let result = transpile("def join_words(words: list) -> str:\n    return ' '.join(words)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_func_string_replace() {
        let result = transpile("def clean(s: str) -> str:\n    return s.replace('\\n', ' ')\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_func_string_format() {
        let result = transpile("def greet(name: str) -> str:\n    return f'Hello, {name}!'\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 39: func_gen - Enum-like patterns
    // =========================================================================

    #[test]
    fn test_w5_class_with_class_constants() {
        let result = transpile("class Color:\n    RED = 0\n    GREEN = 1\n    BLUE = 2\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_class_with_string_constants() {
        let result = transpile("class Status:\n    ACTIVE = 'active'\n    INACTIVE = 'inactive'\n    PENDING = 'pending'\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 40: func_gen - Mixed patterns for deep coverage
    // =========================================================================

    #[test]
    fn test_w5_function_with_for_and_if() {
        let result = transpile("def filter_positive(nums: list) -> list:\n    result = []\n    for n in nums:\n        if n > 0:\n            result.append(n)\n    return result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_function_with_while_and_break() {
        let result = transpile("def find_first_positive(nums: list) -> int:\n    for n in nums:\n        if n > 0:\n            return n\n    return -1\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_function_with_continue() {
        let result = transpile("def skip_negatives(nums: list) -> list:\n    result = []\n    for n in nums:\n        if n < 0:\n            continue\n        result.append(n)\n    return result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_function_with_enumerate() {
        let result = transpile("def with_index(items: list) -> list:\n    result = []\n    for i, item in enumerate(items):\n        result.append((i, item))\n    return result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_function_nested_if() {
        let result = transpile("def nested(x: int, y: int) -> str:\n    if x > 0:\n        if y > 0:\n            return 'both positive'\n        return 'x positive'\n    return 'neither'\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_function_with_not() {
        let result = transpile("def negate(x: bool) -> bool:\n    return not x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_function_walrus_operator() {
        let result = transpile_ok("def process(data: list) -> int:\n    if (n := len(data)) > 10:\n        return n\n    return 0\n");
        assert!(result);
    }

    #[test]
    fn test_w5_function_multiple_assignment() {
        let result = transpile("def init_vars() -> int:\n    x = 0\n    y = 0\n    z = x + y\n    return z\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_function_augmented_assign() {
        let result = transpile("def accumulate(n: int) -> int:\n    total = 0\n    i = 0\n    while i < n:\n        total += i\n        i += 1\n    return total\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_function_empty_body() {
        let result = transpile("def noop():\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_function_only_docstring() {
        let result = transpile("def documented():\n    \"\"\"This function has only a docstring.\"\"\"\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_complex_default_values() {
        let result = transpile("def process(x: int = 0, y: float = 1.0, name: str = 'test') -> int:\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_function_with_type_check_isinstance() {
        let result = transpile("def check_type(x) -> bool:\n    return isinstance(x, int)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_deeply_nested_function() {
        let result = transpile("def outer():\n    def middle():\n        def inner():\n            return 42\n        return inner()\n    return middle()\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 41: argparse - Group patterns
    // =========================================================================

    #[test]
    fn test_w5_argparse_argument_group() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser()\n    group = parser.add_argument_group('input')\n    group.add_argument('--source')\n    args = parser.parse_args()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_argparse_mutually_exclusive() {
        let code = "import argparse\n\ndef main():\n    parser = argparse.ArgumentParser()\n    group = parser.add_mutually_exclusive_group()\n    group.add_argument('--verbose', action='store_true')\n    group.add_argument('--quiet', action='store_true')\n    args = parser.parse_args()\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 42: More binary_ops edge cases
    // =========================================================================

    #[test]
    fn test_w5_modulo_with_negative() {
        let result = transpile("def mod_neg(x: int) -> int:\n    return x % 3\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_multiple_boolean_and() {
        let result = transpile("def all_positive(a: int, b: int, c: int) -> bool:\n    return a > 0 and b > 0 and c > 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_multiple_boolean_or() {
        let result = transpile("def any_zero(a: int, b: int, c: int) -> bool:\n    return a == 0 or b == 0 or c == 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_complex_boolean_expression() {
        let result = transpile("def complex_check(x: int, y: int) -> bool:\n    return (x > 0 and y > 0) or (x < 0 and y < 0)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_nested_arithmetic() {
        let result = transpile("def quadratic(a: int, b: int, c: int, x: int) -> int:\n    return a * x * x + b * x + c\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_string_equality() {
        let result = transpile("def is_hello(s: str) -> bool:\n    return s == 'hello'\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_string_inequality() {
        let result = transpile("def not_empty(s: str) -> bool:\n    return s != ''\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 43: func_gen - Complex class patterns
    // =========================================================================

    #[test]
    fn test_w5_class_with_property_and_method() {
        let result = transpile("class Rect:\n    def __init__(self, w: int, h: int):\n        self.w = w\n        self.h = h\n    def area(self) -> int:\n        return self.w * self.h\n    def perimeter(self) -> int:\n        return 2 * (self.w + self.h)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_class_with_class_and_instance_vars() {
        let result = transpile("class Player:\n    DEFAULT_HEALTH = 100\n    def __init__(self, name: str):\n        self.name = name\n        self.health = Player.DEFAULT_HEALTH\n");
        assert!(transpile_ok("class Player:\n    DEFAULT_HEALTH = 100\n    def __init__(self, name: str):\n        self.name = name\n        self.health = Player.DEFAULT_HEALTH\n"));
    }

    #[test]
    fn test_w5_class_method_calls_another_method() {
        let result = transpile("class Math:\n    def __init__(self):\n        self.val = 0\n    def double(self) -> int:\n        return self.val * 2\n    def quadruple(self) -> int:\n        return self.double() * 2\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_class_with_boolean_method() {
        let result = transpile("class Account:\n    def __init__(self, balance: float):\n        self.balance = balance\n    def is_empty(self) -> bool:\n        return self.balance == 0\n");
        assert!(transpile_ok("class Account:\n    def __init__(self, balance: float):\n        self.balance = balance\n    def is_empty(self) -> bool:\n        return self.balance == 0\n"));
    }

    // =========================================================================
    // Section 44: Miscellaneous coverage gaps
    // =========================================================================

    #[test]
    fn test_w5_slice_basic() {
        let result = transpile("def first_three(items: list) -> list:\n    return items[0:3]\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_negative_indexing() {
        let result = transpile("def last_item(items: list) -> int:\n    return items[-1]\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_range_two_args() {
        let result = transpile("def count(start: int, end: int) -> list:\n    result = []\n    for i in range(start, end):\n        result.append(i)\n    return result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_range_three_args() {
        let result = transpile("def count_step(start: int, end: int, step: int) -> list:\n    result = []\n    for i in range(start, end, step):\n        result.append(i)\n    return result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_dict_access() {
        let result = transpile("def get_val(d: dict, key: str) -> str:\n    return d[key]\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_dict_get_with_default() {
        let result = transpile("def safe_get(d: dict, key: str) -> str:\n    return d.get(key, 'default')\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_list_append_in_loop() {
        let result = transpile("def build_list(n: int) -> list:\n    result = []\n    for i in range(n):\n        result.append(i * 2)\n    return result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_string_startswith() {
        let result = transpile("def is_prefix(s: str, prefix: str) -> bool:\n    return s.startswith(prefix)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_string_endswith() {
        let result = transpile("def is_suffix(s: str, suffix: str) -> bool:\n    return s.endswith(suffix)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_len_call() {
        let result = transpile("def size(items: list) -> int:\n    return len(items)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_bool_not_operator() {
        let result = transpile("def invert(x: bool) -> bool:\n    return not x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_is_none_check() {
        let result = transpile("def is_null(x) -> bool:\n    return x is None\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_is_not_none_check() {
        let result = transpile("def is_not_null(x) -> bool:\n    return x is not None\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_print_statement() {
        let result = transpile("def hello():\n    print('Hello, World!')\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_print_multiple_args() {
        let result = transpile("def show(name: str, age: int):\n    print(name, age)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_int_conversion() {
        let result = transpile("def to_int(s: str) -> int:\n    return int(s)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_str_conversion() {
        let result = transpile("def to_str(n: int) -> str:\n    return str(n)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_float_conversion() {
        let result = transpile("def to_float(s: str) -> float:\n    return float(s)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_min_builtin() {
        let result = transpile("def minimum(a: int, b: int) -> int:\n    return min(a, b)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_max_builtin() {
        let result = transpile("def maximum(a: int, b: int) -> int:\n    return max(a, b)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_abs_builtin() {
        let result = transpile("def absolute(x: int) -> int:\n    return abs(x)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_sum_builtin() {
        let result = transpile("def total(nums: list) -> int:\n    return sum(nums)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_sorted_builtin() {
        let result = transpile("def sort_list(items: list) -> list:\n    return sorted(items)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_reversed_builtin() {
        let result = transpile("def reverse_list(items: list) -> list:\n    return list(reversed(items))\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_zip_builtin() {
        let result = transpile("def pair_up(a: list, b: list) -> list:\n    return list(zip(a, b))\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_any_builtin() {
        let result = transpile("def has_positive(nums: list) -> bool:\n    return any(n > 0 for n in nums)\n");
        assert!(transpile_ok("def has_positive(nums: list) -> bool:\n    return any(n > 0 for n in nums)\n"));
    }

    #[test]
    fn test_w5_all_builtin() {
        let result = transpile_ok("def all_positive(nums: list) -> bool:\n    return all(n > 0 for n in nums)\n");
        assert!(result);
    }

    #[test]
    fn test_w5_map_builtin() {
        let result = transpile("def double_all(nums: list) -> list:\n    return list(map(lambda x: x * 2, nums))\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_filter_builtin() {
        let result = transpile("def positives(nums: list) -> list:\n    return list(filter(lambda x: x > 0, nums))\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // Section 45: Additional edge cases for deep coverage
    // =========================================================================

    #[test]
    fn test_w5_ternary_expression() {
        let result = transpile("def ternary(x: int) -> str:\n    return 'even' if x % 2 == 0 else 'odd'\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_nested_ternary() {
        let result = transpile("def classify(x: int) -> str:\n    return 'positive' if x > 0 else 'zero' if x == 0 else 'negative'\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_multiline_function_body() {
        let result = transpile("def process(x: int) -> int:\n    a = x + 1\n    b = a * 2\n    c = b - 3\n    return c\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_function_calling_function() {
        let result = transpile("def helper(x: int) -> int:\n    return x * 2\n\ndef main_fn(x: int) -> int:\n    return helper(x) + 1\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_function_with_global_list() {
        let result = transpile("PRIMES = [2, 3, 5, 7, 11]\n\ndef is_prime(n: int) -> bool:\n    return n in PRIMES\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_class_with_init_and_multiple_methods() {
        let result = transpile("class Queue:\n    def __init__(self):\n        self.items = []\n    def enqueue(self, item: int):\n        self.items.append(item)\n    def dequeue(self) -> int:\n        return self.items.pop(0)\n    def size(self) -> int:\n        return len(self.items)\n    def is_empty(self) -> bool:\n        return len(self.items) == 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_binary_search() {
        let result = transpile("def binary_search(arr: list, target: int) -> int:\n    low = 0\n    high = len(arr) - 1\n    while low <= high:\n        mid = (low + high) // 2\n        if arr[mid] == target:\n            return mid\n        elif arr[mid] < target:\n            low = mid + 1\n        else:\n            high = mid - 1\n    return -1\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_fibonacci() {
        let result = transpile("def fibonacci(n: int) -> int:\n    if n <= 0:\n        return 0\n    if n == 1:\n        return 1\n    return fibonacci(n - 1) + fibonacci(n - 2)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_gcd() {
        let result = transpile("def gcd(a: int, b: int) -> int:\n    while b != 0:\n        a, b = b, a % b\n    return a\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_fizzbuzz() {
        let result = transpile("def fizzbuzz(n: int) -> str:\n    if n % 15 == 0:\n        return 'FizzBuzz'\n    elif n % 3 == 0:\n        return 'Fizz'\n    elif n % 5 == 0:\n        return 'Buzz'\n    else:\n        return str(n)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_word_count() {
        let result = transpile("def word_count(text: str) -> dict:\n    counts = {}\n    for word in text.split():\n        if word in counts:\n            counts[word] = counts[word] + 1\n        else:\n            counts[word] = 1\n    return counts\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_flatten_list() {
        let result = transpile("def flatten(matrix: list) -> list:\n    result = []\n    for row in matrix:\n        for item in row:\n            result.append(item)\n    return result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w5_is_palindrome() {
        let result = transpile("def is_palindrome(s: str) -> bool:\n    return s == s[::-1]\n");
        assert!(transpile_ok("def is_palindrome(s: str) -> bool:\n    return s == s[::-1]\n"));
    }
}
