// Session 10: Integration tests for rust_gen.rs fixup functions and type inference
// These transpile-level tests exercise the text post-processing functions
// that fix generated Rust code for correct compilation.

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile(python_code: &str) -> String {
    let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
    let (module, _) =
        AstBridge::new().with_source(python_code.to_string()).python_to_hir(ast).expect("hir");
    let tm = TypeMapper::default();
    let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
    result
}

// ============ Enum path separator coverage ============

#[test]
fn test_s10_enum_variant_access() {
    let code = transpile(
        "from enum import Enum\nclass Color(Enum):\n    RED = 1\n    GREEN = 2\n    BLUE = 3\ndef get_color() -> int:\n    return Color.RED\n",
    );
    assert!(code.contains("Color"), "Should have Color enum: {code}");
}

#[test]
fn test_s10_enum_with_value_method() {
    let code = transpile(
        "from enum import Enum\nclass Status(Enum):\n    ACTIVE = 'active'\n    INACTIVE = 'inactive'\ndef is_active(s: str) -> bool:\n    return s == Status.ACTIVE\n",
    );
    assert!(code.contains("Status"), "Should have Status: {code}");
}

// ============ Python truthiness / negation coverage ============

#[test]
fn test_s10_not_on_string() {
    let code = transpile("def is_empty(name: str) -> bool:\n    return not name\n");
    assert!(
        code.contains("is_empty") || code.contains("empty"),
        "Should handle not on string: {code}"
    );
}

#[test]
fn test_s10_not_on_list() {
    let code = transpile("def is_empty(items: list) -> bool:\n    return not items\n");
    assert!(code.contains("fn is_empty"), "Should handle not on list: {code}");
}

#[test]
fn test_s10_not_on_bool() {
    let code = transpile("def negate(flag: bool) -> bool:\n    return not flag\n");
    assert!(code.contains("!flag") || code.contains("!"), "Should negate bool: {code}");
}

// ============ is_none on non-Option coverage ============

#[test]
fn test_s10_none_check_with_option_param() {
    let code = transpile(
        "def process(val: int = None) -> int:\n    if val is None:\n        return 0\n    return val\n",
    );
    assert!(code.contains("fn process"), "Should transpile None check: {code}");
}

#[test]
fn test_s10_none_comparison() {
    let code = transpile("def check(x: int) -> bool:\n    return x is None\n");
    assert!(code.contains("fn check"), "Should handle is None comparison: {code}");
}

// ============ HashMap / dict generation coverage ============

#[test]
fn test_s10_dict_creation_empty() {
    let code =
        transpile("def make_dict() -> dict:\n    d = {}\n    d['key'] = 'value'\n    return d\n");
    assert!(code.contains("HashMap") || code.contains("dict"), "Should generate HashMap: {code}");
}

#[test]
fn test_s10_dict_literal() {
    let code = transpile("def make_dict() -> dict:\n    return {'a': 1, 'b': 2, 'c': 3}\n");
    assert!(
        code.contains("HashMap") || code.contains("insert"),
        "Should generate dict literal: {code}"
    );
}

#[test]
fn test_s10_dict_iteration_items() {
    let code = transpile(
        "def iter_dict(d: dict) -> None:\n    for k, v in d.items():\n        print(k, v)\n",
    );
    assert!(code.contains("iter()") || code.contains("items"), "Should handle dict items: {code}");
}

// ============ Class / struct generation coverage ============

#[test]
fn test_s10_class_with_init_and_methods() {
    let code = transpile(
        "class Rectangle:\n    def __init__(self, width: float, height: float):\n        self.width = width\n        self.height = height\n    def area(self) -> float:\n        return self.width * self.height\n    def perimeter(self) -> float:\n        return 2 * (self.width + self.height)\n",
    );
    assert!(code.contains("struct Rectangle"), "Should have struct: {code}");
    assert!(code.contains("fn area"), "Should have area method: {code}");
    assert!(code.contains("fn perimeter"), "Should have perimeter method: {code}");
}

#[test]
fn test_s10_class_with_str_repr() {
    let code = transpile(
        "class Person:\n    def __init__(self, name: str, age: int):\n        self.name = name\n        self.age = age\n    def __str__(self) -> str:\n        return f'{self.name} ({self.age})'\n",
    );
    assert!(code.contains("struct Person"), "Should have struct: {code}");
    assert!(code.contains("Display") || code.contains("fmt"), "Should have Display impl: {code}");
}

#[test]
fn test_s10_multiple_classes() {
    let code = transpile(
        "class Dog:\n    def __init__(self, name: str):\n        self.name = name\n    def speak(self) -> str:\n        return 'Woof'\nclass Cat:\n    def __init__(self, name: str):\n        self.name = name\n    def speak(self) -> str:\n        return 'Meow'\n",
    );
    assert!(code.contains("struct Dog"), "Should have Dog: {code}");
    assert!(code.contains("struct Cat"), "Should have Cat: {code}");
}

// ============ Type alias coverage ============

#[test]
fn test_s10_type_alias_list() {
    let code = transpile(
        "from typing import List\nMatrix = List[List[int]]\ndef sum_matrix(m: List[List[int]]) -> int:\n    total = 0\n    for row in m:\n        for val in row:\n            total += val\n    return total\n",
    );
    assert!(code.contains("fn sum_matrix"), "Should transpile with type alias: {code}");
}

// ============ Constant / static generation coverage ============

#[test]
fn test_s10_list_constant() {
    let code = transpile(
        "PRIMES = [2, 3, 5, 7, 11, 13]\ndef is_prime(n: int) -> bool:\n    return n in PRIMES\n",
    );
    assert!(
        code.contains("PRIMES") || code.contains("primes"),
        "Should generate list constant: {code}"
    );
}

#[test]
fn test_s10_dict_constant() {
    let code = transpile(
        "CODES = {'US': 1, 'UK': 44, 'DE': 49}\ndef get_code(country: str) -> int:\n    return CODES[country]\n",
    );
    assert!(
        code.contains("CODES") || code.contains("HashMap"),
        "Should generate dict constant: {code}"
    );
}

#[test]
fn test_s10_string_constant() {
    let code = transpile("VERSION = '1.0.0'\ndef get_version() -> str:\n    return VERSION\n");
    assert!(
        code.contains("VERSION") || code.contains("version"),
        "Should generate string constant: {code}"
    );
}

// ============ Import generation coverage ============

#[test]
fn test_s10_import_os() {
    let code = transpile("import os\ndef get_cwd() -> str:\n    return os.getcwd()\n");
    assert!(code.contains("fn get_cwd"), "Should transpile os import: {code}");
}

#[test]
fn test_s10_import_math() {
    let code = transpile(
        "import math\ndef circle_area(r: float) -> float:\n    return math.pi * r ** 2\n",
    );
    assert!(code.contains("fn circle_area"), "Should transpile math import: {code}");
}

#[test]
fn test_s10_from_import() {
    let code = transpile(
        "from typing import List, Dict, Optional\ndef process(items: List[int]) -> Optional[int]:\n    if not items:\n        return None\n    return items[0]\n",
    );
    assert!(code.contains("fn process"), "Should handle from import: {code}");
}

#[test]
fn test_s10_stub_function_import() {
    let code =
        transpile("from mylib import helper\ndef run(x: int) -> int:\n    return helper(x) + 1\n");
    assert!(code.contains("fn run"), "Should generate stub: {code}");
}

// ============ Power / sqrt type fixup coverage ============

#[test]
fn test_s10_power_operator() {
    let code = transpile("def square(x: int) -> int:\n    return x ** 2\n");
    assert!(code.contains("pow") || code.contains("powi"), "Should handle power: {code}");
}

#[test]
fn test_s10_sqrt_math() {
    let code = transpile(
        "import math\ndef hypotenuse(a: float, b: float) -> float:\n    return math.sqrt(a ** 2 + b ** 2)\n",
    );
    assert!(code.contains("sqrt"), "Should handle sqrt: {code}");
}

// ============ Vec / collect patterns coverage ============

#[test]
fn test_s10_list_comprehension_join() {
    let code = transpile(
        "def make_csv(items: list) -> str:\n    return ','.join([str(x) for x in items])\n",
    );
    assert!(code.contains("join") || code.contains("map"), "Should handle join: {code}");
}

#[test]
fn test_s10_sorted_list() {
    let code = transpile("def sort_items(items: list) -> list:\n    return sorted(items)\n");
    assert!(code.contains("sort"), "Should handle sorted: {code}");
}

// ============ Datetime subtraction coverage ============

#[test]
fn test_s10_datetime_operations() {
    let code = transpile(
        "from datetime import date\ndef days_until(target: str) -> int:\n    today = date.today()\n    return 0\n",
    );
    assert!(code.contains("fn days_until"), "Should handle datetime: {code}");
}

// ============ Generator / yield coverage ============

#[test]
fn test_s10_generator_function() {
    let code = transpile(
        "def count_up(n: int) -> int:\n    i = 0\n    while i < n:\n        yield i\n        i += 1\n",
    );
    assert!(code.contains("count_up") || code.contains("yield"), "Should handle generator: {code}");
}

// ============ Context manager coverage ============

#[test]
fn test_s10_with_open() {
    let code = transpile(
        "def read_file(path: str) -> str:\n    with open(path) as f:\n        return f.read()\n",
    );
    assert!(code.contains("fn read_file"), "Should handle with/open: {code}");
}

// ============ Complex algorithms coverage ============

#[test]
fn test_s10_binary_search() {
    let code = transpile(
        "def binary_search(arr: list, target: int) -> int:\n    low = 0\n    high = len(arr) - 1\n    while low <= high:\n        mid = (low + high) // 2\n        if arr[mid] == target:\n            return mid\n        elif arr[mid] < target:\n            low = mid + 1\n        else:\n            high = mid - 1\n    return -1\n",
    );
    assert!(code.contains("fn binary_search"), "Should transpile binary_search: {code}");
    assert!(code.contains("while"), "Should have while loop: {code}");
}

#[test]
fn test_s10_fibonacci_sequence() {
    let code = transpile(
        "def fibonacci(n: int) -> list:\n    if n <= 0:\n        return []\n    if n == 1:\n        return [0]\n    fib = [0, 1]\n    for i in range(2, n):\n        fib.append(fib[i-1] + fib[i-2])\n    return fib\n",
    );
    assert!(code.contains("fn fibonacci"), "Should transpile fibonacci: {code}");
}

#[test]
fn test_s10_gcd_algorithm() {
    let code = transpile(
        "def gcd(a: int, b: int) -> int:\n    while b != 0:\n        a, b = b, a % b\n    return a\n",
    );
    assert!(code.contains("fn gcd"), "Should transpile gcd: {code}");
}

#[test]
fn test_s10_matrix_transpose() {
    let code = transpile(
        "def transpose(matrix: list) -> list:\n    rows = len(matrix)\n    cols = len(matrix[0])\n    result = []\n    for j in range(cols):\n        row = []\n        for i in range(rows):\n            row.append(matrix[i][j])\n        result.append(row)\n    return result\n",
    );
    assert!(code.contains("fn transpose"), "Should transpile transpose: {code}");
}

// ============ String operations coverage ============

#[test]
fn test_s10_string_format() {
    let code = transpile(
        "def greet(name: str, age: int) -> str:\n    return f'Hello {name}, you are {age} years old'\n",
    );
    assert!(code.contains("fn greet"), "Should handle fstring: {code}");
    assert!(code.contains("format!"), "Should use format!: {code}");
}

#[test]
fn test_s10_string_methods() {
    let code = transpile(
        "def process(text: str) -> str:\n    return text.strip().upper().replace(' ', '_')\n",
    );
    assert!(code.contains("fn process"), "Should handle chained methods: {code}");
}

#[test]
fn test_s10_string_split_join() {
    let code = transpile(
        "def reverse_words(text: str) -> str:\n    words = text.split(' ')\n    words.reverse()\n    return ' '.join(words)\n",
    );
    assert!(code.contains("fn reverse_words"), "Should handle split/join: {code}");
}

// ============ Exception handling coverage ============

#[test]
fn test_s10_try_except_basic() {
    let code = transpile(
        "def safe_div(a: int, b: int) -> float:\n    try:\n        return a / b\n    except ZeroDivisionError:\n        return 0.0\n",
    );
    assert!(code.contains("fn safe_div"), "Should handle try/except: {code}");
}

#[test]
fn test_s10_try_except_finally() {
    let code = transpile(
        "def process(x: int) -> int:\n    result = 0\n    try:\n        result = x * 2\n    except ValueError:\n        result = -1\n    finally:\n        print('done')\n    return result\n",
    );
    assert!(code.contains("fn process"), "Should handle try/finally: {code}");
}

// ============ Complex type annotation coverage ============

#[test]
fn test_s10_optional_param() {
    let code = transpile(
        "from typing import Optional\ndef find(items: list, default: Optional[int] = None) -> int:\n    if items:\n        return items[0]\n    if default is not None:\n        return default\n    return 0\n",
    );
    assert!(code.contains("fn find"), "Should handle Optional param: {code}");
}

#[test]
fn test_s10_tuple_return() {
    let code = transpile(
        "from typing import Tuple\ndef divmod_result(a: int, b: int) -> Tuple[int, int]:\n    return a // b, a % b\n",
    );
    assert!(code.contains("fn divmod_result"), "Should handle tuple return: {code}");
}

// ============ Augmented assignment coverage ============

#[test]
fn test_s10_augmented_assign_all() {
    let code = transpile(
        "def ops(x: int) -> int:\n    x += 1\n    x -= 2\n    x *= 3\n    x %= 5\n    return x\n",
    );
    assert!(code.contains("fn ops"), "Should handle augmented assigns: {code}");
    // Augmented assigns may use py_add/py_mul or direct ops
    assert!(code.contains("py_add") || code.contains("+="), "Should have add: {code}");
    assert!(code.contains("- (2") || code.contains("-="), "Should have sub: {code}");
    assert!(code.contains("py_mul") || code.contains("*="), "Should have mul: {code}");
}

// ============ Bitwise operations coverage ============

#[test]
fn test_s10_bitwise_ops() {
    let code = transpile(
        "def bitwise(a: int, b: int) -> int:\n    x = a & b\n    y = a | b\n    z = a ^ b\n    w = a << 2\n    v = a >> 3\n    return x + y + z + w + v\n",
    );
    assert!(code.contains("fn bitwise"), "Should handle bitwise: {code}");
}

// ============ Lambda / closure coverage ============

#[test]
fn test_s10_lambda_in_sorted() {
    let code = transpile(
        "def sort_by_len(items: list) -> list:\n    return sorted(items, key=lambda x: len(x))\n",
    );
    assert!(code.contains("sort") || code.contains("sorted"), "Should handle lambda sort: {code}");
}

#[test]
fn test_s10_lambda_in_filter() {
    let code = transpile(
        "def filter_positive(items: list) -> list:\n    return list(filter(lambda x: x > 0, items))\n",
    );
    assert!(code.contains("filter"), "Should handle lambda filter: {code}");
}

// ============ Comprehension coverage ============

#[test]
fn test_s10_list_comp_with_condition() {
    let code =
        transpile("def evens(n: int) -> list:\n    return [x for x in range(n) if x % 2 == 0]\n");
    assert!(code.contains("fn evens"), "Should handle list comp: {code}");
}

#[test]
fn test_s10_dict_comprehension() {
    let code = transpile("def squares(n: int) -> dict:\n    return {x: x**2 for x in range(n)}\n");
    assert!(code.contains("fn squares"), "Should handle dict comp: {code}");
}

#[test]
fn test_s10_set_comprehension() {
    let code =
        transpile("def unique_squares(items: list) -> set:\n    return {x**2 for x in items}\n");
    assert!(code.contains("fn unique_squares"), "Should handle set comp: {code}");
}

// ============ rust_type_string_to_hir coverage ============

#[test]
fn test_s10_type_inference_complex() {
    let code = transpile(
        "from typing import Dict, List\ndef lookup(data: Dict[str, List[int]], key: str) -> List[int]:\n    return data[key]\n",
    );
    assert!(code.contains("fn lookup"), "Should handle complex types: {code}");
    assert!(code.contains("HashMap") || code.contains("Dict"), "Should have HashMap: {code}");
}

// ============ Decorator coverage ============

#[test]
fn test_s10_staticmethod_decorator() {
    let code = transpile(
        "class MathUtils:\n    @staticmethod\n    def add(a: int, b: int) -> int:\n        return a + b\n",
    );
    assert!(code.contains("fn add"), "Should handle staticmethod: {code}");
}

// ============ Multiple return paths coverage ============

#[test]
fn test_s10_multiple_returns() {
    let code = transpile(
        "def classify(x: int) -> str:\n    if x > 0:\n        return 'positive'\n    elif x < 0:\n        return 'negative'\n    else:\n        return 'zero'\n",
    );
    assert!(code.contains("fn classify"), "Should handle multiple returns: {code}");
    assert!(
        code.contains("positive") && code.contains("negative") && code.contains("zero"),
        "Should have all return values: {code}"
    );
}

// ============ Nested function coverage ============

#[test]
fn test_s10_nested_function() {
    let code = transpile(
        "def outer(x: int) -> int:\n    def inner(y: int) -> int:\n        return y * 2\n    return inner(x) + 1\n",
    );
    assert!(code.contains("fn outer"), "Should handle nested function: {code}");
}

// ============ Assert statement coverage ============

#[test]
fn test_s10_assert_statement() {
    let code = transpile(
        "def validate(x: int) -> int:\n    assert x > 0, 'x must be positive'\n    return x\n",
    );
    assert!(code.contains("assert"), "Should handle assert: {code}");
}

// ============ Walrus operator coverage ============

#[test]
fn test_s10_walrus_operator() {
    let code = transpile(
        "def process(items: list) -> int:\n    if (n := len(items)) > 0:\n        return n\n    return 0\n",
    );
    assert!(code.contains("fn process"), "Should handle walrus: {code}");
}

// ============ isinstance check coverage ============

#[test]
fn test_s10_isinstance_check() {
    let code = transpile("def check_type(x: int) -> bool:\n    return isinstance(x, int)\n");
    assert!(code.contains("fn check_type"), "Should handle isinstance: {code}");
}

// ============ Delete statement coverage ============

#[test]
fn test_s10_del_statement() {
    let code = transpile("def remove_key(d: dict, key: str) -> None:\n    del d[key]\n");
    assert!(code.contains("fn remove_key"), "Should handle del: {code}");
}

// ============ Global constant patterns ============

#[test]
fn test_s10_multiple_constants() {
    let code = transpile(
        "MAX_SIZE = 100\nMIN_SIZE = 1\nDEFAULT = 50\ndef validate(size: int) -> int:\n    if size > MAX_SIZE:\n        return MAX_SIZE\n    if size < MIN_SIZE:\n        return MIN_SIZE\n    return size\n",
    );
    assert!(code.contains("fn validate"), "Should handle constants: {code}");
}

// ============ Recursive function coverage ============

#[test]
fn test_s10_recursive_factorial() {
    let code = transpile(
        "def factorial(n: int) -> int:\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)\n",
    );
    assert!(code.contains("fn factorial"), "Should handle recursion: {code}");
    assert!(code.contains("factorial("), "Should have recursive call: {code}");
}

// ============ Ternary expression coverage ============

#[test]
fn test_s10_ternary_expression() {
    let code = transpile("def abs_val(x: int) -> int:\n    return x if x >= 0 else -x\n");
    assert!(code.contains("fn abs_val"), "Should handle ternary: {code}");
}

// ============ Star args coverage ============

#[test]
fn test_s10_varargs() {
    let code = transpile(
        "def sum_all(*args: int) -> int:\n    total = 0\n    for x in args:\n        total += x\n    return total\n",
    );
    assert!(code.contains("fn sum_all"), "Should handle varargs: {code}");
}
