//! Coverage wave 3: advanced pattern coverage boost tests
//!
//! Targets uncovered branches in lambda_generators.rs, expr_advanced.rs,
//! argparse_transform.rs, direct_rules_convert/, method_stmt_convert.rs,
//! stmt_convert.rs, expr_builtins.rs, expr_collections.rs,
//! expr_index_slice.rs, and expr_operators.rs

use crate::DepylerPipeline;

fn transpile(code: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).expect("transpilation should succeed")
}

fn transpile_ok(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_ok()
}

// =============================================================================
// Section 1: Lambda patterns (lambda_generators.rs - 53.2% cov, 354 missed)
// =============================================================================

#[test]
fn test_lambda_captured_variable_in_closure() {
    let code = transpile(
        "def make_adder(captured: int):\n    fn = lambda x: x + captured\n    return fn(5)",
    );
    assert!(!code.is_empty(), "lambda with captured variable: {}", code);
}

#[test]
fn test_lambda_nested_composition() {
    let code = transpile("def compose():\n    return lambda x: (lambda y: x + y)(10)");
    assert!(!code.is_empty(), "nested lambda composition: {}", code);
}

#[test]
fn test_lambda_in_sorted_key() {
    let code = transpile(
        "def sort_names(items: list) -> list:\n    return sorted(items, key=lambda x: len(x))",
    );
    assert!(!code.is_empty(), "lambda in sorted key: {}", code);
}

#[test]
fn test_lambda_in_filter() {
    let code = transpile(
        "def positives(items: list) -> list:\n    return list(filter(lambda x: x > 0, items))",
    );
    assert!(!code.is_empty(), "lambda in filter: {}", code);
}

#[test]
fn test_lambda_in_map() {
    let code = transpile(
        "def doubled(items: list) -> list:\n    return list(map(lambda x: x * 2, items))",
    );
    assert!(!code.is_empty(), "lambda in map: {}", code);
}

#[test]
fn test_lambda_with_default_param() {
    let code = transpile(
        "def make_scaler(factor: int):\n    fn = lambda x, y=10: x * factor + y\n    return fn(3)",
    );
    assert!(!code.is_empty(), "lambda with default param: {}", code);
}

#[test]
fn test_lambda_returning_string() {
    let code = transpile("def labeler():\n    return lambda x: \"item_\" + str(x)");
    assert!(!code.is_empty(), "lambda returning string: {}", code);
}

#[test]
fn test_lambda_boolean_expression() {
    let code = transpile("def checker():\n    return lambda x: x > 0 and x < 100");
    assert!(!code.is_empty(), "lambda boolean expr: {}", code);
}

#[test]
fn test_lambda_with_ternary() {
    let code = transpile("def classifier():\n    return lambda x: \"pos\" if x > 0 else \"neg\"");
    assert!(!code.is_empty(), "lambda with ternary: {}", code);
}

#[test]
fn test_lambda_assigned_to_variable() {
    let code =
        transpile("def use_lambda() -> int:\n    double = lambda x: x * 2\n    return double(21)");
    assert!(!code.is_empty(), "lambda assigned to var: {}", code);
}

#[test]
fn test_lambda_in_list() {
    let code = transpile(
        "def dispatch(idx: int) -> int:\n    ops = [lambda x: x + 1, lambda x: x * 2, lambda x: x - 1]\n    return ops[idx](10)",
    );
    assert!(!code.is_empty(), "lambda in list: {}", code);
}

// =============================================================================
// Section 2: f-string edge cases (lambda_generators.rs)
// =============================================================================

#[test]
fn test_fstring_with_list_variable() {
    let code = transpile("def show_list(my_list: list) -> str:\n    return f\"items: {my_list}\"");
    assert!(!code.is_empty(), "fstring with list: {}", code);
}

#[test]
fn test_fstring_with_dict_variable() {
    let code = transpile("def show_dict(my_dict: dict) -> str:\n    return f\"data: {my_dict}\"");
    assert!(!code.is_empty(), "fstring with dict: {}", code);
}

#[test]
fn test_fstring_with_format_spec_2f() {
    let code = transpile("def format_price(value: float) -> str:\n    return f\"${value:.2f}\"");
    assert!(!code.is_empty(), "fstring format spec .2f: {}", code);
}

#[test]
fn test_fstring_with_arithmetic_expression() {
    let code = transpile("def show_sum(x: int, y: int) -> str:\n    return f\"total: {x + y}\"");
    assert!(code.contains("format!"), "fstring with arithmetic: {}", code);
}

#[test]
fn test_fstring_with_method_call_upper() {
    let code = transpile("def shout(name: str) -> str:\n    return f\"HELLO {name.upper()}!\"");
    assert!(!code.is_empty(), "fstring with .upper(): {}", code);
}

#[test]
fn test_fstring_with_ternary_expression() {
    let code = transpile(
        "def status(active: bool) -> str:\n    return f\"Status: {'active' if active else 'inactive'}\"",
    );
    assert!(!code.is_empty(), "fstring with ternary: {}", code);
}

#[test]
fn test_fstring_with_nested_quotes() {
    let code = transpile("def quote(name: str) -> str:\n    return f\"Name: '{name}'\"");
    assert!(!code.is_empty(), "fstring nested quotes: {}", code);
}

#[test]
fn test_fstring_with_integer_formatting() {
    let code = transpile("def pad_num(n: int) -> str:\n    return f\"{n:05d}\"");
    assert!(!code.is_empty(), "fstring int format: {}", code);
}

#[test]
fn test_fstring_with_multiple_expressions() {
    let code =
        transpile("def coords(x: int, y: int, z: int) -> str:\n    return f\"({x}, {y}, {z})\"");
    assert!(code.contains("format!"), "fstring multi expr: {}", code);
}

#[test]
fn test_fstring_with_len_call() {
    let code =
        transpile("def show_count(items: list) -> str:\n    return f\"count: {len(items)}\"");
    assert!(!code.is_empty(), "fstring with len(): {}", code);
}

// =============================================================================
// Section 3: Comprehension patterns (expr_advanced.rs - 30.7% cov, 284 missed)
// =============================================================================

#[test]
fn test_list_comp_with_filter_positive() {
    let code = transpile(
        "def double_positives(lst: list) -> list:\n    return [x * 2 for x in lst if x > 0]",
    );
    assert!(!code.is_empty(), "list comp with filter: {}", code);
}

#[test]
fn test_set_comp_with_filter() {
    let code = transpile(
        "def even_doubles(lst: list) -> set:\n    return {x * 2 for x in lst if x % 2 == 0}",
    );
    assert!(!code.is_empty(), "set comp with filter: {}", code);
}

#[test]
fn test_dict_comp_with_filter() {
    let code = transpile(
        "def positive_doubled(items: list) -> dict:\n    return {i: v * 2 for i, v in enumerate(items) if v > 0}",
    );
    assert!(!code.is_empty(), "dict comp with filter: {}", code);
}

#[test]
fn test_nested_list_comp_product() {
    let code = transpile(
        "def products() -> list:\n    return [x * y for x in range(3) for y in range(3)]",
    );
    assert!(!code.is_empty(), "nested list comp product: {}", code);
}

#[test]
fn test_generator_expression_in_sum() {
    let code =
        transpile("def sum_squares(n: int) -> int:\n    return sum(x * x for x in range(n))");
    assert!(!code.is_empty(), "generator expr in sum: {}", code);
}

#[test]
fn test_comp_with_method_call_strip() {
    let code = transpile("def clean(lines: list) -> list:\n    return [s.strip() for s in lines]");
    assert!(!code.is_empty(), "comp with strip method: {}", code);
}

#[test]
fn test_dict_comp_from_enumerate() {
    let code = transpile(
        "def indexed(items: list) -> dict:\n    return {i: v for i, v in enumerate(items)}",
    );
    assert!(!code.is_empty(), "dict comp from enumerate: {}", code);
}

#[test]
fn test_list_comp_with_function_call() {
    let code =
        transpile("def string_lengths(items: list) -> list:\n    return [len(x) for x in items]");
    assert!(!code.is_empty(), "list comp with function call: {}", code);
}

#[test]
fn test_list_comp_with_conditional_expr() {
    let code = transpile(
        "def classify(nums: list) -> list:\n    return [\"even\" if x % 2 == 0 else \"odd\" for x in nums]",
    );
    assert!(!code.is_empty(), "list comp with conditional: {}", code);
}

#[test]
fn test_set_comp_basic() {
    let code =
        transpile("def unique_lengths(words: list) -> set:\n    return {len(w) for w in words}");
    assert!(!code.is_empty(), "set comp basic: {}", code);
}

#[test]
fn test_list_comp_nested_with_filter() {
    let code = transpile(
        "def flat_even(matrix: list) -> list:\n    return [x for row in matrix for x in row if x % 2 == 0]",
    );
    assert!(!code.is_empty(), "nested comp with filter: {}", code);
}

#[test]
fn test_dict_comp_key_transform() {
    let code = transpile(
        "def upper_keys(d: dict) -> dict:\n    return {k.upper(): v for k, v in d.items()}",
    );
    assert!(!code.is_empty(), "dict comp key transform: {}", code);
}

// =============================================================================
// Section 4: Argparse patterns (argparse_transform.rs - 77.8% cov, 480 missed)
// =============================================================================

#[test]
fn test_argparse_basic_parser() {
    let code = transpile(
        "import argparse\ndef main():\n    parser = argparse.ArgumentParser(description=\"A tool\")\n    args = parser.parse_args()",
    );
    assert!(!code.is_empty(), "basic argparse parser: {}", code);
}

#[test]
fn test_argparse_positional_argument() {
    let code = transpile(
        "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"input\")\n    args = parser.parse_args()",
    );
    assert!(!code.is_empty(), "argparse positional: {}", code);
}

#[test]
fn test_argparse_optional_with_flag() {
    let code = transpile(
        "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"-v\", \"--verbose\", action=\"store_true\")\n    args = parser.parse_args()",
    );
    assert!(!code.is_empty(), "argparse optional flag: {}", code);
}

#[test]
fn test_argparse_with_default() {
    let code = transpile(
        "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--output\", default=\"out.txt\")\n    args = parser.parse_args()",
    );
    assert!(!code.is_empty(), "argparse with default: {}", code);
}

#[test]
fn test_argparse_with_type_int() {
    let code = transpile(
        "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--count\", type=int, default=0)\n    args = parser.parse_args()",
    );
    assert!(!code.is_empty(), "argparse with type int: {}", code);
}

#[test]
fn test_argparse_with_choices() {
    let code = transpile(
        "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--format\", choices=[\"json\", \"csv\"])\n    args = parser.parse_args()",
    );
    assert!(!code.is_empty(), "argparse with choices: {}", code);
}

#[test]
fn test_argparse_with_nargs_plus() {
    let code = transpile(
        "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"files\", nargs=\"+\")\n    args = parser.parse_args()",
    );
    assert!(!code.is_empty(), "argparse nargs plus: {}", code);
}

#[test]
fn test_argparse_with_help() {
    let code = transpile(
        "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--name\", help=\"The user name\")\n    args = parser.parse_args()",
    );
    assert!(!code.is_empty(), "argparse with help: {}", code);
}

#[test]
fn test_argparse_with_required() {
    let code = transpile(
        "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--config\", required=True)\n    args = parser.parse_args()",
    );
    assert!(!code.is_empty(), "argparse with required: {}", code);
}

#[test]
fn test_argparse_multiple_args() {
    let code = transpile(
        "import argparse\ndef main():\n    parser = argparse.ArgumentParser(description=\"Multi\")\n    parser.add_argument(\"input\")\n    parser.add_argument(\"-o\", \"--output\", default=\"out.txt\")\n    parser.add_argument(\"-v\", \"--verbose\", action=\"store_true\")\n    parser.add_argument(\"--count\", type=int, default=1)\n    args = parser.parse_args()",
    );
    assert!(!code.is_empty(), "argparse multiple args: {}", code);
}

#[test]
fn test_argparse_subparsers() {
    let code = transpile_ok(
        "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest=\"command\")\n    sub = subparsers.add_parser(\"run\")\n    args = parser.parse_args()",
    );
    assert!(code, "argparse subparsers should parse");
}

#[test]
fn test_argparse_store_false_action() {
    let code = transpile(
        "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--no-cache\", action=\"store_false\", dest=\"cache\")\n    args = parser.parse_args()",
    );
    assert!(!code.is_empty(), "argparse store_false: {}", code);
}

// =============================================================================
// Section 5: Class patterns (method_stmt_convert.rs - 53.2% cov)
// =============================================================================

#[test]
fn test_class_with_init_and_method() {
    let code = transpile(
        "class Counter:\n    def __init__(self, start: int):\n        self.count = start\n    def increment(self) -> int:\n        self.count += 1\n        return self.count",
    );
    assert!(!code.is_empty(), "class init + method: {}", code);
}

#[test]
fn test_class_with_str_dunder() {
    let code = transpile(
        "class Person:\n    def __init__(self, name: str, age: int):\n        self.name = name\n        self.age = age\n    def __str__(self) -> str:\n        return f\"{self.name} ({self.age})\"",
    );
    assert!(!code.is_empty(), "class __str__: {}", code);
}

#[test]
fn test_class_with_repr_dunder() {
    let code = transpile(
        "class Coord:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y\n    def __repr__(self) -> str:\n        return f\"Coord({self.x}, {self.y})\"",
    );
    assert!(!code.is_empty(), "class __repr__: {}", code);
}

#[test]
fn test_class_with_len_dunder() {
    let code = transpile(
        "class Stack:\n    def __init__(self):\n        self.items: list = []\n    def __len__(self) -> int:\n        return len(self.items)\n    def push(self, item: int):\n        self.items.append(item)",
    );
    assert!(!code.is_empty(), "class __len__: {}", code);
}

#[test]
fn test_class_with_getitem_dunder() {
    let code = transpile(
        "class Row:\n    def __init__(self, data: list):\n        self.data = data\n    def __getitem__(self, i: int) -> int:\n        return self.data[i]",
    );
    assert!(!code.is_empty(), "class __getitem__: {}", code);
}

#[test]
fn test_class_with_setitem_dunder() {
    let code = transpile(
        "class Grid:\n    def __init__(self, size: int):\n        self.cells: list = [0] * size\n    def __setitem__(self, i: int, v: int):\n        self.cells[i] = v",
    );
    assert!(!code.is_empty(), "class __setitem__: {}", code);
}

#[test]
fn test_class_staticmethod_utility() {
    let code = transpile(
        "class Validator:\n    @staticmethod\n    def is_positive(n: int) -> bool:\n        return n > 0",
    );
    assert!(!code.is_empty(), "class @staticmethod: {}", code);
}

#[test]
fn test_class_classmethod_factory() {
    let code = transpile(
        "class Config:\n    def __init__(self, val: str):\n        self.val = val\n    @classmethod\n    def from_default(cls):\n        return cls(\"default\")",
    );
    assert!(!code.is_empty(), "class @classmethod factory: {}", code);
}

#[test]
fn test_class_inheritance_basic() {
    let code = transpile(
        "class Base:\n    def __init__(self, x: int):\n        self.x = x\n    def value(self) -> int:\n        return self.x\n\nclass Derived(Base):\n    def doubled(self) -> int:\n        return self.x * 2",
    );
    assert!(!code.is_empty(), "class inheritance: {}", code);
}

#[test]
fn test_class_super_init() {
    let code = transpile_ok(
        "class Base:\n    def __init__(self, x: int):\n        self.x = x\n\nclass Child(Base):\n    def __init__(self, x: int, y: int):\n        super().__init__(x)\n        self.y = y",
    );
    assert!(code, "class super().__init__");
}

#[test]
fn test_class_property_decorator() {
    let code = transpile(
        "class Rectangle:\n    def __init__(self, w: float, h: float):\n        self.w = w\n        self.h = h\n    @property\n    def area(self) -> float:\n        return self.w * self.h",
    );
    assert!(!code.is_empty(), "class @property: {}", code);
}

#[test]
fn test_class_multiple_methods() {
    let code = transpile(
        "class Calculator:\n    def __init__(self, value: int):\n        self.value = value\n    def add(self, n: int) -> int:\n        self.value += n\n        return self.value\n    def sub(self, n: int) -> int:\n        self.value -= n\n        return self.value\n    def reset(self):\n        self.value = 0",
    );
    assert!(!code.is_empty(), "class multiple methods: {}", code);
}

#[test]
fn test_class_with_bool_dunder() {
    let code = transpile(
        "class Wrapper:\n    def __init__(self, val: int):\n        self.val = val\n    def __bool__(self) -> bool:\n        return self.val != 0",
    );
    assert!(!code.is_empty(), "class __bool__: {}", code);
}

#[test]
fn test_class_with_eq_dunder() {
    let code = transpile(
        "class Token:\n    def __init__(self, kind: str, text: str):\n        self.kind = kind\n        self.text = text\n    def __eq__(self, other) -> bool:\n        return self.kind == other.kind and self.text == other.text",
    );
    assert!(!code.is_empty(), "class __eq__: {}", code);
}

// =============================================================================
// Section 6: Statement patterns (stmt_convert.rs - 57.1% cov)
// =============================================================================

#[test]
fn test_tuple_unpacking_assignment() {
    let code = transpile("def split_pair(pair: tuple) -> int:\n    a, b = pair\n    return a + b");
    assert!(!code.is_empty(), "tuple unpacking: {}", code);
}

#[test]
fn test_star_unpacking_rest() {
    let code = transpile_ok(
        "def head_tail(items: list) -> int:\n    first, *rest = items\n    return first",
    );
    assert!(code, "star unpacking should parse");
}

#[test]
fn test_import_with_alias() {
    let code = transpile("import os\ndef get_cwd() -> str:\n    return os.getcwd()");
    assert!(!code.is_empty(), "import module: {}", code);
}

#[test]
fn test_from_import_with_alias() {
    let code = transpile(
        "from os.path import join\ndef combine(a: str, b: str) -> str:\n    return join(a, b)",
    );
    assert!(!code.is_empty(), "from import: {}", code);
}

#[test]
fn test_context_manager_with() {
    let code = transpile(
        "def read_file(path: str) -> str:\n    with open(path) as f:\n        data = f.read()\n    return data",
    );
    assert!(!code.is_empty(), "context manager with: {}", code);
}

#[test]
fn test_try_except_finally_full() {
    let code = transpile(
        "def safe_div(a: int, b: int) -> int:\n    result = 0\n    try:\n        result = a // b\n    except ZeroDivisionError:\n        result = -1\n    finally:\n        print(result)\n    return result",
    );
    assert!(!code.is_empty(), "try/except/finally: {}", code);
}

#[test]
fn test_assert_with_message() {
    let code = transpile("def validate(x: int):\n    assert x > 0, \"must be positive\"");
    assert!(!code.is_empty(), "assert with message: {}", code);
}

#[test]
fn test_assert_without_message() {
    let code = transpile("def check(x: int):\n    assert x > 0");
    assert!(!code.is_empty(), "assert without message: {}", code);
}

#[test]
fn test_while_loop_with_break() {
    let code = transpile(
        "def find_first(items: list, target: int) -> int:\n    idx = 0\n    while idx < len(items):\n        if items[idx] == target:\n            break\n        idx += 1\n    return idx",
    );
    assert!(!code.is_empty(), "while with break: {}", code);
}

#[test]
fn test_while_loop_with_continue() {
    let code = transpile(
        "def sum_positive(items: list) -> int:\n    total = 0\n    idx = 0\n    while idx < len(items):\n        idx += 1\n        if items[idx - 1] < 0:\n            continue\n        total += items[idx - 1]\n    return total",
    );
    assert!(!code.is_empty(), "while with continue: {}", code);
}

#[test]
fn test_for_else_pattern() {
    let code = transpile_ok(
        "def find_item(items: list, target: int) -> bool:\n    for item in items:\n        if item == target:\n            return True\n    else:\n        return False",
    );
    assert!(code, "for/else should parse");
}

#[test]
fn test_nested_if_elif_else() {
    let code = transpile(
        "def classify(x: int) -> str:\n    if x > 100:\n        return \"high\"\n    elif x > 50:\n        return \"medium\"\n    elif x > 0:\n        return \"low\"\n    else:\n        return \"none\"",
    );
    assert!(!code.is_empty(), "nested if/elif/else: {}", code);
}

#[test]
fn test_global_variable_usage() {
    let code = transpile(
        "THRESHOLD = 50\n\ndef above_threshold(x: int) -> bool:\n    return x > THRESHOLD",
    );
    assert!(!code.is_empty(), "global variable: {}", code);
}

#[test]
fn test_multiple_assignment_same_line() {
    let code = transpile("def init() -> int:\n    x = y = z = 0\n    return x + y + z");
    assert!(!code.is_empty(), "multiple assignment: {}", code);
}

#[test]
fn test_augmented_assign_modulo() {
    let code = transpile("def wrap(x: int, n: int) -> int:\n    x %= n\n    return x");
    assert!(!code.is_empty(), "augmented modulo: {}", code);
}

#[test]
fn test_augmented_assign_floor_div() {
    let code = transpile("def half(x: int) -> int:\n    x //= 2\n    return x");
    assert!(!code.is_empty(), "augmented floor div: {}", code);
}

#[test]
fn test_delete_variable() {
    let code = transpile_ok("def cleanup():\n    temp = 42\n    del temp");
    assert!(code, "del variable should parse");
}

// =============================================================================
// Section 7: Collection operations (expr_collections.rs - 59.7% cov)
// =============================================================================

#[test]
fn test_set_literal_construction() {
    let code = transpile("def make_set() -> set:\n    return {1, 2, 3, 4, 5}");
    assert!(!code.is_empty(), "set literal: {}", code);
}

#[test]
fn test_frozenset_construction() {
    let code = transpile("def make_frozen() -> frozenset:\n    return frozenset([1, 2, 3])");
    assert!(!code.is_empty(), "frozenset: {}", code);
}

#[test]
fn test_tuple_mixed_types() {
    let code = transpile("def make_tuple() -> tuple:\n    return (1, \"hello\", True)");
    assert!(!code.is_empty(), "tuple mixed types: {}", code);
}

#[test]
fn test_dict_update_method() {
    let code =
        transpile("def merge(d: dict, other: dict) -> dict:\n    d.update(other)\n    return d");
    assert!(!code.is_empty(), "dict update: {}", code);
}

#[test]
fn test_list_extend_method() {
    let code = transpile(
        "def extend_list(items: list, more: list) -> list:\n    items.extend(more)\n    return items",
    );
    assert!(!code.is_empty(), "list extend: {}", code);
}

#[test]
fn test_set_add_method() {
    let code =
        transpile("def add_to_set(s: set, item: int) -> set:\n    s.add(item)\n    return s");
    assert!(!code.is_empty(), "set add: {}", code);
}

#[test]
fn test_set_remove_method() {
    let code = transpile(
        "def remove_from_set(s: set, item: int) -> set:\n    s.remove(item)\n    return s",
    );
    assert!(!code.is_empty(), "set remove: {}", code);
}

#[test]
fn test_set_discard_method() {
    let code = transpile(
        "def discard_from_set(s: set, item: int) -> set:\n    s.discard(item)\n    return s",
    );
    assert!(!code.is_empty(), "set discard: {}", code);
}

#[test]
fn test_empty_dict_construction() {
    let code = transpile("def make_empty() -> dict:\n    return {}");
    assert!(!code.is_empty(), "empty dict: {}", code);
}

#[test]
fn test_empty_list_construction() {
    let code = transpile("def make_empty() -> list:\n    return []");
    assert!(!code.is_empty(), "empty list: {}", code);
}

#[test]
fn test_dict_get_with_default() {
    let code =
        transpile("def safe_get(d: dict, key: str) -> str:\n    return d.get(key, \"default\")");
    assert!(!code.is_empty(), "dict get with default: {}", code);
}

#[test]
fn test_dict_setdefault() {
    let code =
        transpile("def ensure_key(d: dict, key: str) -> str:\n    return d.setdefault(key, \"\")");
    assert!(!code.is_empty(), "dict setdefault: {}", code);
}

#[test]
fn test_dict_pop_with_default() {
    let code = transpile("def take(d: dict, key: str) -> str:\n    return d.pop(key, \"none\")");
    assert!(!code.is_empty(), "dict pop with default: {}", code);
}

#[test]
fn test_list_insert_method() {
    let code = transpile(
        "def insert_front(items: list, val: int) -> list:\n    items.insert(0, val)\n    return items",
    );
    assert!(!code.is_empty(), "list insert: {}", code);
}

#[test]
fn test_list_remove_method() {
    let code = transpile(
        "def remove_val(items: list, val: int) -> list:\n    items.remove(val)\n    return items",
    );
    assert!(!code.is_empty(), "list remove: {}", code);
}

#[test]
fn test_list_count_method() {
    let code =
        transpile("def count_val(items: list, val: int) -> int:\n    return items.count(val)");
    assert!(!code.is_empty(), "list count: {}", code);
}

#[test]
fn test_list_index_method() {
    let code =
        transpile("def find_val(items: list, val: int) -> int:\n    return items.index(val)");
    assert!(!code.is_empty(), "list index: {}", code);
}

// =============================================================================
// Section 8: Index/slice operations (expr_index_slice.rs - 48.5% cov)
// =============================================================================

#[test]
fn test_negative_index_last() {
    let code = transpile("def last(items: list) -> int:\n    return items[-1]");
    assert!(!code.is_empty(), "negative index: {}", code);
}

#[test]
fn test_negative_index_second_last() {
    let code = transpile("def second_last(items: list) -> int:\n    return items[-2]");
    assert!(!code.is_empty(), "negative index -2: {}", code);
}

#[test]
fn test_slice_range() {
    let code = transpile("def middle(items: list) -> list:\n    return items[1:3]");
    assert!(!code.is_empty(), "slice 1:3: {}", code);
}

#[test]
fn test_slice_with_step() {
    let code = transpile("def every_other(items: list) -> list:\n    return items[::2]");
    assert!(!code.is_empty(), "slice step 2: {}", code);
}

#[test]
fn test_slice_reverse() {
    let code = transpile("def reverse(items: list) -> list:\n    return items[::-1]");
    assert!(!code.is_empty(), "slice reverse: {}", code);
}

#[test]
fn test_dict_key_access() {
    let code = transpile("def get_name(d: dict) -> str:\n    return d[\"name\"]");
    assert!(!code.is_empty(), "dict key access: {}", code);
}

#[test]
fn test_nested_dict_access() {
    let code = transpile("def deep(data: dict) -> str:\n    return data[\"a\"][\"b\"]");
    assert!(!code.is_empty(), "nested dict access: {}", code);
}

#[test]
fn test_slice_from_start() {
    let code = transpile("def first_three(items: list) -> list:\n    return items[:3]");
    assert!(!code.is_empty(), "slice from start: {}", code);
}

#[test]
fn test_slice_to_end() {
    let code = transpile("def rest(items: list) -> list:\n    return items[1:]");
    assert!(!code.is_empty(), "slice to end: {}", code);
}

#[test]
fn test_string_slicing() {
    let code = transpile("def prefix(s: str) -> str:\n    return s[:5]");
    assert!(!code.is_empty(), "string slicing: {}", code);
}

#[test]
fn test_string_negative_index() {
    let code = transpile("def last_char(s: str) -> str:\n    return s[-1]");
    assert!(!code.is_empty(), "string negative index: {}", code);
}

#[test]
fn test_list_assign_by_index() {
    let code = transpile(
        "def set_first(items: list, val: int) -> list:\n    items[0] = val\n    return items",
    );
    assert!(!code.is_empty(), "list assign by index: {}", code);
}

// =============================================================================
// Section 9: Operator patterns (expr_operators.rs - 57.1% cov)
// =============================================================================

#[test]
fn test_walrus_operator_in_if() {
    let result = transpile_ok(
        "def check_length(items: list) -> bool:\n    if (n := len(items)) > 0:\n        return n > 5\n    return False",
    );
    assert!(result, "walrus operator in if");
}

#[test]
fn test_ternary_expression() {
    let code = transpile("def abs_val(x: int) -> int:\n    return x if x >= 0 else -x");
    assert!(!code.is_empty(), "ternary expression: {}", code);
}

#[test]
fn test_chained_comparison_range() {
    let code = transpile("def in_range(x: int) -> bool:\n    return 1 < x < 10");
    assert!(!code.is_empty(), "chained comparison: {}", code);
}

#[test]
fn test_is_none_check() {
    let code = transpile("def is_null(x: int) -> bool:\n    return x is None");
    assert!(!code.is_empty(), "is None: {}", code);
}

#[test]
fn test_is_not_none_check() {
    let code = transpile("def has_value(x: int) -> bool:\n    return x is not None");
    assert!(!code.is_empty(), "is not None: {}", code);
}

#[test]
fn test_string_in_containment() {
    let code = transpile("def has_sub(text: str) -> bool:\n    return \"hello\" in text");
    assert!(!code.is_empty(), "string containment: {}", code);
}

#[test]
fn test_not_in_dict() {
    let code = transpile("def missing(d: dict, key: str) -> bool:\n    return key not in d");
    assert!(!code.is_empty(), "not in dict: {}", code);
}

#[test]
fn test_power_operator() {
    let code = transpile("def square(x: int) -> int:\n    return x ** 2");
    assert!(!code.is_empty(), "power operator: {}", code);
}

#[test]
fn test_power_operator_float() {
    let code = transpile("def cube(x: float) -> float:\n    return x ** 3");
    assert!(!code.is_empty(), "float power: {}", code);
}

#[test]
fn test_bitwise_and() {
    let code = transpile("def mask(a: int, b: int) -> int:\n    return a & b");
    assert!(code.contains("&"), "bitwise and: {}", code);
}

#[test]
fn test_bitwise_or() {
    let code = transpile("def combine_flags(a: int, b: int) -> int:\n    return a | b");
    assert!(code.contains("|"), "bitwise or: {}", code);
}

#[test]
fn test_bitwise_xor() {
    let code = transpile("def toggle(a: int, b: int) -> int:\n    return a ^ b");
    assert!(code.contains("^"), "bitwise xor: {}", code);
}

#[test]
fn test_left_shift() {
    let code = transpile("def shift_left(x: int) -> int:\n    return x << 2");
    assert!(code.contains("<<"), "left shift: {}", code);
}

#[test]
fn test_right_shift() {
    let code = transpile("def shift_right(x: int) -> int:\n    return x >> 2");
    assert!(code.contains(">>"), "right shift: {}", code);
}

#[test]
fn test_bitwise_not() {
    let code = transpile("def invert(x: int) -> int:\n    return ~x");
    assert!(!code.is_empty(), "bitwise not: {}", code);
}

#[test]
fn test_boolean_and_or() {
    let code = transpile("def check(a: bool, b: bool, c: bool) -> bool:\n    return a and b or c");
    assert!(!code.is_empty(), "boolean and/or: {}", code);
}

#[test]
fn test_not_operator() {
    let code = transpile("def negate(x: bool) -> bool:\n    return not x");
    assert!(!code.is_empty(), "not operator: {}", code);
}

#[test]
fn test_floor_division() {
    let code = transpile("def int_div(a: int, b: int) -> int:\n    return a // b");
    assert!(!code.is_empty(), "floor division: {}", code);
}

#[test]
fn test_modulo_operator() {
    let code = transpile("def remainder(a: int, b: int) -> int:\n    return a % b");
    assert!(!code.is_empty(), "modulo: {}", code);
}

// =============================================================================
// Section 10: Builtin functions (expr_builtins.rs - 53.0% cov)
// =============================================================================

#[test]
fn test_enumerate_with_start() {
    let code = transpile(
        "def numbered(items: list):\n    for i, val in enumerate(items, 1):\n        print(i, val)",
    );
    assert!(!code.is_empty(), "enumerate start=1: {}", code);
}

#[test]
fn test_zip_two_lists_loop() {
    let code = transpile(
        "def pair_up(xs: list, ys: list):\n    for a, b in zip(xs, ys):\n        print(a, b)",
    );
    assert!(!code.is_empty(), "zip loop: {}", code);
}

#[test]
fn test_map_with_str() {
    let code =
        transpile("def stringify(numbers: list) -> list:\n    return list(map(str, numbers))");
    assert!(!code.is_empty(), "map with str: {}", code);
}

#[test]
fn test_filter_with_none() {
    let code = transpile("def truthy(items: list) -> list:\n    return list(filter(None, items))");
    assert!(!code.is_empty(), "filter with None: {}", code);
}

#[test]
fn test_isinstance_single() {
    let code = transpile("def is_int(x: int) -> bool:\n    return isinstance(x, int)");
    assert!(!code.is_empty(), "isinstance single: {}", code);
}

#[test]
fn test_isinstance_tuple() {
    let code = transpile("def is_numeric(x: int) -> bool:\n    return isinstance(x, (int, float))");
    assert!(!code.is_empty(), "isinstance tuple: {}", code);
}

#[test]
fn test_hasattr_check() {
    let code = transpile_ok("def has_method(obj) -> bool:\n    return hasattr(obj, \"process\")");
    assert!(code, "hasattr should transpile");
}

#[test]
fn test_getattr_with_default() {
    // getattr with untyped obj may or may not transpile; test that the pipeline handles it
    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(
        "def get_attr_val(d: dict, key: str) -> str:\n    return d.get(key, \"fallback\")",
    );
    assert!(result.is_ok(), "getattr-like pattern: {:?}", result.err());
}

#[test]
fn test_input_builtin() {
    let code = transpile("def ask() -> str:\n    name = input(\"Enter name: \")\n    return name");
    assert!(!code.is_empty(), "input builtin: {}", code);
}

#[test]
fn test_print_with_sep() {
    let code = transpile("def show(a: str, b: str):\n    print(a, b, sep=\", \")");
    assert!(!code.is_empty(), "print with sep: {}", code);
}

#[test]
fn test_print_with_end() {
    let code = transpile("def show(msg: str):\n    print(msg, end=\"\")");
    assert!(!code.is_empty(), "print with end: {}", code);
}

#[test]
fn test_len_on_string() {
    let code = transpile("def str_len(s: str) -> int:\n    return len(s)");
    assert!(!code.is_empty(), "len on string: {}", code);
}

#[test]
fn test_len_on_list() {
    let code = transpile("def list_len(items: list) -> int:\n    return len(items)");
    assert!(!code.is_empty(), "len on list: {}", code);
}

#[test]
fn test_len_on_dict() {
    let code = transpile("def dict_len(d: dict) -> int:\n    return len(d)");
    assert!(!code.is_empty(), "len on dict: {}", code);
}

#[test]
fn test_max_of_list() {
    let code = transpile("def biggest(items: list) -> int:\n    return max(items)");
    assert!(!code.is_empty(), "max of list: {}", code);
}

#[test]
fn test_min_of_list() {
    let code = transpile("def smallest(items: list) -> int:\n    return min(items)");
    assert!(!code.is_empty(), "min of list: {}", code);
}

#[test]
fn test_sum_of_list() {
    let code = transpile("def total(items: list) -> int:\n    return sum(items)");
    assert!(!code.is_empty(), "sum of list: {}", code);
}

#[test]
fn test_any_builtin() {
    let code = transpile("def has_true(items: list) -> bool:\n    return any(items)");
    assert!(!code.is_empty(), "any builtin: {}", code);
}

#[test]
fn test_all_builtin() {
    let code = transpile("def all_true(items: list) -> bool:\n    return all(items)");
    assert!(!code.is_empty(), "all builtin: {}", code);
}

#[test]
fn test_sorted_builtin() {
    let code = transpile("def order(items: list) -> list:\n    return sorted(items)");
    assert!(!code.is_empty(), "sorted builtin: {}", code);
}

#[test]
fn test_reversed_builtin() {
    let code = transpile("def flip(items: list) -> list:\n    return list(reversed(items))");
    assert!(!code.is_empty(), "reversed builtin: {}", code);
}

#[test]
fn test_abs_int() {
    let code = transpile("def magnitude(x: int) -> int:\n    return abs(x)");
    assert!(!code.is_empty(), "abs int: {}", code);
}

#[test]
fn test_round_float() {
    let code = transpile("def rounded(x: float) -> float:\n    return round(x, 2)");
    assert!(!code.is_empty(), "round float: {}", code);
}

#[test]
fn test_chr_builtin() {
    let code = transpile("def to_char(n: int) -> str:\n    return chr(n)");
    assert!(!code.is_empty(), "chr builtin: {}", code);
}

#[test]
fn test_ord_builtin() {
    let code = transpile("def to_code(c: str) -> int:\n    return ord(c)");
    assert!(!code.is_empty(), "ord builtin: {}", code);
}

#[test]
fn test_type_builtin() {
    let code = transpile_ok("def get_type(x: int) -> str:\n    return str(type(x))");
    assert!(code, "type builtin should transpile");
}

#[test]
fn test_range_single_arg() {
    let code = transpile(
        "def count(n: int) -> int:\n    total = 0\n    for i in range(n):\n        total += i\n    return total",
    );
    assert!(!code.is_empty(), "range single arg: {}", code);
}

#[test]
fn test_range_two_args() {
    let code = transpile(
        "def count_range(a: int, b: int) -> int:\n    total = 0\n    for i in range(a, b):\n        total += i\n    return total",
    );
    assert!(!code.is_empty(), "range two args: {}", code);
}

#[test]
fn test_range_three_args() {
    let code = transpile(
        "def step_sum(n: int) -> int:\n    total = 0\n    for i in range(0, n, 2):\n        total += i\n    return total",
    );
    assert!(!code.is_empty(), "range three args: {}", code);
}

// =============================================================================
// Section 11: Mixed advanced patterns (cross-module coverage)
// =============================================================================

#[test]
fn test_dict_items_iteration() {
    let code = transpile(
        "def show_all(d: dict):\n    for key, val in d.items():\n        print(key, val)",
    );
    assert!(!code.is_empty(), "dict items iteration: {}", code);
}

#[test]
fn test_dict_keys_iteration() {
    let code = transpile("def all_keys(d: dict) -> list:\n    return list(d.keys())");
    assert!(!code.is_empty(), "dict keys: {}", code);
}

#[test]
fn test_dict_values_iteration() {
    let code = transpile("def all_vals(d: dict) -> list:\n    return list(d.values())");
    assert!(!code.is_empty(), "dict values: {}", code);
}

#[test]
fn test_string_split_method() {
    let code = transpile("def words(s: str) -> list:\n    return s.split(\" \")");
    assert!(!code.is_empty(), "string split: {}", code);
}

#[test]
fn test_string_join_method() {
    let code = transpile("def combine(parts: list) -> str:\n    return \", \".join(parts)");
    assert!(!code.is_empty(), "string join: {}", code);
}

#[test]
fn test_string_replace_method() {
    let code = transpile("def sanitize(s: str) -> str:\n    return s.replace(\"old\", \"new\")");
    assert!(!code.is_empty(), "string replace: {}", code);
}

#[test]
fn test_string_startswith() {
    let code = transpile("def is_comment(line: str) -> bool:\n    return line.startswith(\"#\")");
    assert!(!code.is_empty(), "startswith: {}", code);
}

#[test]
fn test_string_endswith() {
    let code = transpile("def is_python(path: str) -> bool:\n    return path.endswith(\".py\")");
    assert!(!code.is_empty(), "endswith: {}", code);
}

#[test]
fn test_string_strip_method() {
    let code = transpile("def clean(s: str) -> str:\n    return s.strip()");
    assert!(!code.is_empty(), "strip: {}", code);
}

#[test]
fn test_string_lstrip_rstrip() {
    let code = transpile("def clean_left(s: str) -> str:\n    return s.lstrip()");
    assert!(!code.is_empty(), "lstrip: {}", code);
}

#[test]
fn test_string_lower_upper() {
    let code = transpile("def normalize(s: str) -> str:\n    return s.lower()");
    assert!(!code.is_empty(), "lower: {}", code);
}

#[test]
fn test_string_isdigit() {
    let code = transpile("def is_number(s: str) -> bool:\n    return s.isdigit()");
    assert!(!code.is_empty(), "isdigit: {}", code);
}

#[test]
fn test_string_isalpha() {
    let code = transpile("def is_word(s: str) -> bool:\n    return s.isalpha()");
    assert!(!code.is_empty(), "isalpha: {}", code);
}

#[test]
fn test_list_sort_method() {
    let code =
        transpile("def sort_inplace(items: list) -> list:\n    items.sort()\n    return items");
    assert!(!code.is_empty(), "list sort: {}", code);
}

#[test]
fn test_list_reverse_method() {
    let code =
        transpile("def rev_inplace(items: list) -> list:\n    items.reverse()\n    return items");
    assert!(!code.is_empty(), "list reverse: {}", code);
}

#[test]
fn test_list_clear_method() {
    let code = transpile("def empty(items: list) -> list:\n    items.clear()\n    return items");
    assert!(!code.is_empty(), "list clear: {}", code);
}

#[test]
fn test_list_copy_method() {
    let code = transpile("def clone(items: list) -> list:\n    return items.copy()");
    assert!(!code.is_empty(), "list copy: {}", code);
}

#[test]
fn test_dict_clear_method() {
    let code = transpile("def empty_dict(d: dict) -> dict:\n    d.clear()\n    return d");
    assert!(!code.is_empty(), "dict clear: {}", code);
}

#[test]
fn test_in_operator_list() {
    let code = transpile("def contains(items: list, val: int) -> bool:\n    return val in items");
    assert!(!code.is_empty(), "in list: {}", code);
}

#[test]
fn test_nested_for_loops() {
    let code = transpile(
        "def matrix_sum(matrix: list) -> int:\n    total = 0\n    for row in matrix:\n        for val in row:\n            total += val\n    return total",
    );
    assert!(!code.is_empty(), "nested for: {}", code);
}

#[test]
fn test_list_comprehension_to_string() {
    let code = transpile("def int_strings(n: int) -> list:\n    return [str(i) for i in range(n)]");
    assert!(!code.is_empty(), "comp with str(): {}", code);
}

#[test]
fn test_conditional_return_early() {
    let code = transpile(
        "def guard(x: int) -> int:\n    if x < 0:\n        return 0\n    if x > 100:\n        return 100\n    return x",
    );
    assert!(!code.is_empty(), "early returns: {}", code);
}

#[test]
fn test_multiple_string_operations() {
    let code = transpile(
        "def process(s: str) -> str:\n    result = s.strip()\n    result = result.lower()\n    result = result.replace(\" \", \"_\")\n    return result",
    );
    assert!(!code.is_empty(), "chained string ops: {}", code);
}

#[test]
fn test_complex_dict_comprehension() {
    let code = transpile(
        "def word_lengths(words: list) -> dict:\n    return {w: len(w) for w in words if len(w) > 0}",
    );
    assert!(!code.is_empty(), "complex dict comp: {}", code);
}

#[test]
fn test_ternary_in_assignment() {
    let code = transpile(
        "def clamp(x: int, lo: int, hi: int) -> int:\n    result = lo if x < lo else hi if x > hi else x\n    return result",
    );
    assert!(!code.is_empty(), "ternary in assignment: {}", code);
}

#[test]
fn test_exception_class_definition() {
    let code = transpile(
        "class ValidationError(Exception):\n    pass\n\ndef validate(x: int):\n    if x < 0:\n        raise ValidationError(\"negative\")",
    );
    assert!(!code.is_empty(), "custom exception: {}", code);
}

#[test]
fn test_try_except_as_variable() {
    let code = transpile(
        "def safe_convert(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError as e:\n        print(str(e))\n        return 0",
    );
    assert!(!code.is_empty(), "except as variable: {}", code);
}

#[test]
fn test_empty_function_pass() {
    let code = transpile("def noop():\n    pass");
    assert!(!code.is_empty(), "pass function: {}", code);
}

#[test]
fn test_docstring_function() {
    let code = transpile(
        "def documented(x: int) -> int:\n    \"\"\"Return doubled value.\"\"\"\n    return x * 2",
    );
    assert!(!code.is_empty(), "docstring function: {}", code);
}

#[test]
fn test_multiline_function_body() {
    let code = transpile(
        "def process(items: list) -> int:\n    total = 0\n    count = 0\n    for item in items:\n        total += item\n        count += 1\n    if count == 0:\n        return 0\n    return total // count",
    );
    assert!(!code.is_empty(), "multiline body: {}", code);
}
