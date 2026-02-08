//! Coverage boost deep tests - Wave 2
//!
//! Targets deeper uncovered branches in:
//! - expr_methods.rs (direct_rules_convert): obscure method calls
//! - type_helpers.rs: type inference from expressions
//! - lambda_generators.rs: f-string, closures with complex captures
//! - binary_ops.rs: deeper operator edge cases
//! - expr_advanced.rs: module constructors, comprehensions
//! - argparse_transform.rs: subcommand analysis
//! - codegen.rs: expression to tokens conversion
//! - call_dispatch.rs: stdlib type constructors
//! - var_analysis.rs: variable collection patterns

use crate::DepylerPipeline;

fn transpile(code: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline
        .transpile(code)
        .expect("transpilation should succeed")
}

fn transpile_ok(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_ok()
}

// =============================================================================
// Section 1: expr_methods.rs deeper branches - obscure string/list/dict methods
// =============================================================================

#[test]
fn test_string_expandtabs() {
    let code = transpile("def expand(s: str) -> str:\n    return s.expandtabs(4)");
    assert!(!code.is_empty(), "expandtabs: {}", code);
}

#[test]
fn test_string_partition() {
    let code = transpile("def split_at(s: str) -> tuple:\n    return s.partition(\":\")");
    assert!(!code.is_empty(), "partition: {}", code);
}

#[test]
fn test_string_rpartition() {
    let code = transpile("def rsplit_at(s: str) -> tuple:\n    return s.rpartition(\":\")");
    assert!(!code.is_empty(), "rpartition: {}", code);
}

#[test]
fn test_string_maketrans() {
    let code = transpile("def trans(s: str) -> str:\n    table = str.maketrans(\"abc\", \"xyz\")\n    return s.translate(table)");
    assert!(!code.is_empty(), "maketrans/translate: {}", code);
}

#[test]
fn test_string_format_method() {
    let code = transpile("def fmt(name: str) -> str:\n    return \"Hello, {}\".format(name)");
    assert!(!code.is_empty(), "format method: {}", code);
}

#[test]
fn test_list_sort_key() {
    let code = transpile("def sort_by_len(items: list) -> list:\n    items.sort(key=len)\n    return items");
    assert!(!code.is_empty(), "sort with key: {}", code);
}

#[test]
fn test_list_pop_index() {
    let code = transpile("def pop_first(items: list) -> int:\n    return items.pop(0)");
    assert!(!code.is_empty(), "pop(0): {}", code);
}

#[test]
fn test_dict_popitem() {
    let code = transpile("def pop_item(d: dict) -> tuple:\n    return d.popitem()");
    assert!(!code.is_empty(), "popitem: {}", code);
}

#[test]
fn test_set_add() {
    let code = transpile("def add_item(s: set, x: int):\n    s.add(x)");
    assert!(!code.is_empty(), "set add: {}", code);
}

#[test]
fn test_set_discard() {
    let code = transpile("def remove_item(s: set, x: int):\n    s.discard(x)");
    assert!(!code.is_empty(), "set discard: {}", code);
}

#[test]
fn test_set_union() {
    let code = transpile("def combine(a: set, b: set) -> set:\n    return a.union(b)");
    assert!(!code.is_empty(), "set union: {}", code);
}

#[test]
fn test_set_intersection() {
    let code = transpile("def common(a: set, b: set) -> set:\n    return a.intersection(b)");
    assert!(!code.is_empty(), "set intersection: {}", code);
}

#[test]
fn test_set_difference() {
    let code = transpile("def diff(a: set, b: set) -> set:\n    return a.difference(b)");
    assert!(!code.is_empty(), "set difference: {}", code);
}

// =============================================================================
// Section 2: f-string and lambda deep tests
// =============================================================================

#[test]
fn test_fstring_with_expression() {
    let code = transpile("def info(x: int) -> str:\n    return f\"value is {x + 1}\"");
    assert!(code.contains("format!"), "fstring expr: {}", code);
}

#[test]
fn test_fstring_multiple_vars() {
    let code = transpile("def fmt(a: str, b: int) -> str:\n    return f\"{a}: {b}\"");
    assert!(code.contains("format!"), "fstring multi: {}", code);
}

#[test]
fn test_fstring_with_method() {
    let code = transpile("def fmt(s: str) -> str:\n    return f\"{s.upper()}\"");
    assert!(!code.is_empty(), "fstring method: {}", code);
}

#[test]
fn test_fstring_nested_braces() {
    let code = transpile("def fmt(x: float) -> str:\n    return f\"{x:.2f}\"");
    assert!(!code.is_empty(), "fstring format spec: {}", code);
}

#[test]
fn test_lambda_no_args() {
    let code = transpile("def make():\n    return lambda: 42");
    assert!(!code.is_empty(), "lambda no args: {}", code);
}

#[test]
fn test_lambda_multiple_args() {
    let code = transpile("def make():\n    return lambda x, y: x + y");
    assert!(!code.is_empty(), "lambda multi args: {}", code);
}

#[test]
fn test_lambda_in_default() {
    let code = transpile("def process(items: list, key=lambda x: x) -> list:\n    return sorted(items, key=key)");
    assert!(!code.is_empty(), "lambda default: {}", code);
}

#[test]
fn test_lambda_conditional() {
    let code = transpile("def make():\n    return lambda x: \"even\" if x % 2 == 0 else \"odd\"");
    assert!(!code.is_empty(), "lambda conditional: {}", code);
}

// =============================================================================
// Section 3: Complex class patterns and protocol implementations
// =============================================================================

#[test]
fn test_class_with_property() {
    let code = transpile("class Circle:\n    def __init__(self, r: float):\n        self.r = r\n    @property\n    def area(self) -> float:\n        return 3.14159 * self.r * self.r");
    assert!(!code.is_empty(), "property: {}", code);
}

#[test]
fn test_class_with_str() {
    let code = transpile("class Point:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y\n    def __str__(self) -> str:\n        return f\"({self.x}, {self.y})\"");
    assert!(!code.is_empty(), "dunder str: {}", code);
}

#[test]
fn test_class_with_len() {
    let code = transpile("class Container:\n    def __init__(self):\n        self.items: list = []\n    def __len__(self) -> int:\n        return len(self.items)");
    assert!(!code.is_empty(), "dunder len: {}", code);
}

#[test]
fn test_class_with_eq() {
    let code = transpile("class Value:\n    def __init__(self, v: int):\n        self.v = v\n    def __eq__(self, other) -> bool:\n        return self.v == other.v");
    assert!(!code.is_empty(), "dunder eq: {}", code);
}

#[test]
fn test_class_with_repr() {
    let code = transpile("class Item:\n    def __init__(self, name: str):\n        self.name = name\n    def __repr__(self) -> str:\n        return f\"Item({self.name})\"");
    assert!(!code.is_empty(), "dunder repr: {}", code);
}

#[test]
fn test_class_with_iter() {
    let code = transpile("class Range:\n    def __init__(self, n: int):\n        self.n = n\n    def __iter__(self):\n        return iter(range(self.n))");
    assert!(!code.is_empty(), "dunder iter: {}", code);
}

#[test]
fn test_class_inheritance() {
    let code = transpile("class Animal:\n    def __init__(self, name: str):\n        self.name = name\n    def speak(self) -> str:\n        return self.name\n\nclass Dog(Animal):\n    def speak(self) -> str:\n        return f\"{self.name} barks\"");
    assert!(!code.is_empty(), "inheritance: {}", code);
}

#[test]
fn test_class_classmethod() {
    let code = transpile("class Config:\n    @classmethod\n    def from_file(cls, path: str):\n        return cls()");
    assert!(!code.is_empty(), "classmethod: {}", code);
}

#[test]
fn test_class_staticmethod() {
    let code = transpile("class Math:\n    @staticmethod\n    def add(a: int, b: int) -> int:\n        return a + b");
    assert!(!code.is_empty(), "staticmethod: {}", code);
}

// =============================================================================
// Section 4: Comprehension and generator patterns
// =============================================================================

#[test]
fn test_list_comprehension_with_condition() {
    let code = transpile("def evens(n: int) -> list:\n    return [x for x in range(n) if x % 2 == 0]");
    assert!(!code.is_empty(), "filtered listcomp: {}", code);
}

#[test]
fn test_dict_comprehension() {
    let code = transpile("def make_dict(keys: list) -> dict:\n    return {k: len(k) for k in keys}");
    assert!(!code.is_empty(), "dict comp: {}", code);
}

#[test]
fn test_set_comprehension() {
    let code = transpile("def unique_lengths(words: list) -> set:\n    return {len(w) for w in words}");
    assert!(!code.is_empty(), "set comp: {}", code);
}

#[test]
fn test_nested_list_comprehension() {
    let code = transpile("def flatten(matrix: list) -> list:\n    return [x for row in matrix for x in row]");
    assert!(!code.is_empty(), "nested comp: {}", code);
}

#[test]
fn test_generator_expression_sum() {
    let code = transpile("def total(items: list) -> int:\n    return sum(x * x for x in items)");
    assert!(!code.is_empty(), "gen expr sum: {}", code);
}

#[test]
fn test_generator_expression_any() {
    let code = transpile("def has_positive(items: list) -> bool:\n    return any(x > 0 for x in items)");
    assert!(!code.is_empty(), "gen expr any: {}", code);
}

#[test]
fn test_generator_expression_all() {
    let code = transpile("def all_positive(items: list) -> bool:\n    return all(x > 0 for x in items)");
    assert!(!code.is_empty(), "gen expr all: {}", code);
}

#[test]
fn test_generator_expression_min() {
    let code = transpile("def min_square(items: list) -> int:\n    return min(x * x for x in items)");
    assert!(!code.is_empty(), "gen expr min: {}", code);
}

#[test]
fn test_generator_expression_max() {
    let code = transpile("def max_square(items: list) -> int:\n    return max(x * x for x in items)");
    assert!(!code.is_empty(), "gen expr max: {}", code);
}

// =============================================================================
// Section 5: Complex assignment and variable patterns
// =============================================================================

#[test]
fn test_augmented_assign_add() {
    let code = transpile("def accumulate(items: list) -> int:\n    total = 0\n    for x in items:\n        total += x\n    return total");
    assert!(code.contains("+="), "aug add: {}", code);
}

#[test]
fn test_augmented_assign_mul() {
    let code = transpile("def product(items: list) -> int:\n    result = 1\n    for x in items:\n        result *= x\n    return result");
    assert!(!code.is_empty(), "aug mul: {}", code);
}

#[test]
fn test_augmented_assign_sub() {
    let code = transpile("def countdown(n: int) -> int:\n    total = 100\n    total -= n\n    return total");
    assert!(code.contains("-="), "aug sub: {}", code);
}

#[test]
fn test_augmented_assign_div() {
    let code = transpile("def halve(x: float) -> float:\n    x /= 2.0\n    return x");
    assert!(!code.is_empty(), "aug div: {}", code);
}

#[test]
fn test_string_concatenation_augmented() {
    let code = transpile("def build(parts: list) -> str:\n    result = \"\"\n    for p in parts:\n        result += p\n    return result");
    assert!(!code.is_empty(), "string aug concat: {}", code);
}

#[test]
fn test_walrus_operator() {
    let code = transpile_ok("def check(items: list) -> bool:\n    if (n := len(items)) > 0:\n        return n > 5\n    return False");
    assert!(code, "walrus operator");
}

#[test]
fn test_multiple_return_values() {
    let code = transpile("def min_max(items: list) -> tuple:\n    return (min(items), max(items))");
    assert!(!code.is_empty(), "multiple returns: {}", code);
}

#[test]
fn test_chained_comparison() {
    let code = transpile("def in_range(x: int) -> bool:\n    return 0 < x < 100");
    assert!(!code.is_empty(), "chained comparison: {}", code);
}

#[test]
fn test_ternary_nested() {
    let code = transpile("def classify(x: int) -> str:\n    return \"big\" if x > 100 else \"medium\" if x > 10 else \"small\"");
    assert!(!code.is_empty(), "nested ternary: {}", code);
}

#[test]
fn test_global_constant() {
    let code = transpile("MAX_SIZE = 100\n\ndef check(x: int) -> bool:\n    return x < MAX_SIZE");
    assert!(!code.is_empty(), "global constant: {}", code);
}

// =============================================================================
// Section 6: IO/stdlib patterns for deeper coverage
// =============================================================================

#[test]
fn test_os_listdir() {
    let code = transpile("import os\ndef list_files(path: str) -> list:\n    return os.listdir(path)");
    assert!(!code.is_empty(), "os.listdir: {}", code);
}

#[test]
fn test_os_makedirs() {
    let code = transpile("import os\ndef ensure_dir(path: str):\n    os.makedirs(path, exist_ok=True)");
    assert!(!code.is_empty(), "os.makedirs: {}", code);
}

#[test]
fn test_os_path_basename() {
    let code = transpile("import os\ndef name(path: str) -> str:\n    return os.path.basename(path)");
    assert!(!code.is_empty(), "basename: {}", code);
}

#[test]
fn test_os_path_dirname() {
    let code = transpile("import os\ndef parent(path: str) -> str:\n    return os.path.dirname(path)");
    assert!(!code.is_empty(), "dirname: {}", code);
}

#[test]
fn test_os_path_splitext() {
    let code = transpile("import os\ndef split_ext(path: str) -> tuple:\n    return os.path.splitext(path)");
    assert!(!code.is_empty(), "splitext: {}", code);
}

#[test]
fn test_os_path_isfile() {
    let code = transpile("import os\ndef is_file(path: str) -> bool:\n    return os.path.isfile(path)");
    assert!(!code.is_empty(), "isfile: {}", code);
}

#[test]
fn test_os_path_isdir() {
    let code = transpile("import os\ndef is_dir(path: str) -> bool:\n    return os.path.isdir(path)");
    assert!(!code.is_empty(), "isdir: {}", code);
}

#[test]
fn test_os_path_abspath() {
    let code = transpile("import os\ndef absolute(path: str) -> str:\n    return os.path.abspath(path)");
    assert!(!code.is_empty(), "abspath: {}", code);
}

#[test]
fn test_subprocess_run() {
    let code = transpile("import subprocess\ndef run_cmd(cmd: str) -> int:\n    result = subprocess.run([cmd], capture_output=True)\n    return result.returncode");
    assert!(!code.is_empty(), "subprocess.run: {}", code);
}

#[test]
fn test_sys_argv() {
    let code = transpile("import sys\ndef get_args() -> list:\n    return sys.argv[1:]");
    assert!(!code.is_empty(), "sys.argv: {}", code);
}

#[test]
fn test_sys_exit() {
    let code = transpile("import sys\ndef bail(code: int):\n    sys.exit(code)");
    assert!(!code.is_empty(), "sys.exit: {}", code);
}

#[test]
fn test_re_match() {
    let code = transpile("import re\ndef matches(pattern: str, text: str) -> bool:\n    return re.match(pattern, text) is not None");
    assert!(!code.is_empty(), "re.match: {}", code);
}

#[test]
fn test_re_findall() {
    let code = transpile("import re\ndef find_all(pattern: str, text: str) -> list:\n    return re.findall(pattern, text)");
    assert!(!code.is_empty(), "re.findall: {}", code);
}

#[test]
fn test_re_sub() {
    let code = transpile("import re\ndef replace(pattern: str, repl: str, text: str) -> str:\n    return re.sub(pattern, repl, text)");
    assert!(!code.is_empty(), "re.sub: {}", code);
}

#[test]
fn test_math_floor() {
    let code = transpile("import math\ndef floor_val(x: float) -> int:\n    return math.floor(x)");
    assert!(!code.is_empty(), "math.floor: {}", code);
}

#[test]
fn test_math_ceil() {
    let code = transpile("import math\ndef ceil_val(x: float) -> int:\n    return math.ceil(x)");
    assert!(!code.is_empty(), "math.ceil: {}", code);
}

#[test]
fn test_math_log() {
    let code = transpile("import math\ndef log_val(x: float) -> float:\n    return math.log(x)");
    assert!(!code.is_empty(), "math.log: {}", code);
}

#[test]
fn test_math_sqrt() {
    let code = transpile("import math\ndef sqrt_val(x: float) -> float:\n    return math.sqrt(x)");
    assert!(code.contains("sqrt"), "math.sqrt: {}", code);
}

#[test]
fn test_math_pow() {
    let code = transpile("import math\ndef power(x: float, y: float) -> float:\n    return math.pow(x, y)");
    assert!(!code.is_empty(), "math.pow: {}", code);
}

#[test]
fn test_math_pi() {
    let code = transpile("import math\ndef circle_area(r: float) -> float:\n    return math.pi * r * r");
    assert!(!code.is_empty(), "math.pi: {}", code);
}

// =============================================================================
// Section 7: Exception and error handling edge cases
// =============================================================================

#[test]
fn test_try_finally() {
    let code = transpile("def cleanup(path: str):\n    try:\n        print(path)\n    finally:\n        print(\"done\")");
    assert!(!code.is_empty(), "try-finally: {}", code);
}

#[test]
fn test_try_except_else() {
    let code = transpile("def safe(x: int) -> int:\n    try:\n        result = 100 // x\n    except ZeroDivisionError:\n        result = 0\n    else:\n        result += 1\n    return result");
    assert!(!code.is_empty(), "try-except-else: {}", code);
}

#[test]
fn test_multiple_except() {
    let code = transpile("def safe_parse(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError:\n        return -1\n    except TypeError:\n        return -2");
    assert!(!code.is_empty(), "multi except: {}", code);
}

#[test]
fn test_raise_from() {
    let code = transpile("def convert(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError as e:\n        raise RuntimeError(\"bad value\") from e");
    assert!(!code.is_empty(), "raise from: {}", code);
}

#[test]
fn test_custom_exception() {
    let code = transpile("class AppError(Exception):\n    pass\n\ndef fail():\n    raise AppError(\"something broke\")");
    assert!(!code.is_empty(), "custom exception: {}", code);
}

// =============================================================================
// Section 8: Type conversion patterns
// =============================================================================

#[test]
fn test_int_to_str() {
    let code = transpile("def to_str(x: int) -> str:\n    return str(x)");
    assert!(code.contains("to_string"), "int to str: {}", code);
}

#[test]
fn test_str_to_int() {
    let code = transpile("def to_int(s: str) -> int:\n    return int(s)");
    assert!(code.contains("parse"), "str to int: {}", code);
}

#[test]
fn test_str_to_float() {
    let code = transpile("def to_float(s: str) -> float:\n    return float(s)");
    assert!(code.contains("parse"), "str to float: {}", code);
}

#[test]
fn test_float_to_int() {
    let code = transpile("def truncate(x: float) -> int:\n    return int(x)");
    assert!(!code.is_empty(), "float to int: {}", code);
}

#[test]
fn test_bool_to_int() {
    let code = transpile("def flag_val(b: bool) -> int:\n    return int(b)");
    assert!(!code.is_empty(), "bool to int: {}", code);
}

#[test]
fn test_list_to_set() {
    let code = transpile("def unique(items: list) -> set:\n    return set(items)");
    assert!(!code.is_empty(), "list to set: {}", code);
}

#[test]
fn test_tuple_to_list() {
    let code = transpile("def to_list(t: tuple) -> list:\n    return list(t)");
    assert!(!code.is_empty(), "tuple to list: {}", code);
}

#[test]
fn test_dict_keys_to_list() {
    let code = transpile("def key_list(d: dict) -> list:\n    return list(d.keys())");
    assert!(!code.is_empty(), "keys to list: {}", code);
}

#[test]
fn test_sorted_with_key() {
    let code = transpile("def sort_by_len(words: list) -> list:\n    return sorted(words, key=len)");
    assert!(!code.is_empty(), "sorted with key: {}", code);
}

#[test]
fn test_sorted_reverse() {
    let code = transpile("def sort_desc(items: list) -> list:\n    return sorted(items, reverse=True)");
    assert!(!code.is_empty(), "sorted reverse: {}", code);
}

#[test]
fn test_enumerate_start() {
    let code = transpile("def indexed(items: list):\n    for i, val in enumerate(items, 1):\n        print(i, val)");
    assert!(!code.is_empty(), "enumerate start: {}", code);
}

#[test]
fn test_zip_two_lists() {
    let code = transpile("def pairs(a: list, b: list) -> list:\n    return list(zip(a, b))");
    assert!(!code.is_empty(), "zip: {}", code);
}

#[test]
fn test_reversed_builtin() {
    let code = transpile("def rev(items: list) -> list:\n    return list(reversed(items))");
    assert!(!code.is_empty(), "reversed: {}", code);
}

#[test]
fn test_abs_builtin() {
    let code = transpile("def magnitude(x: int) -> int:\n    return abs(x)");
    assert!(code.contains("abs"), "abs: {}", code);
}

#[test]
fn test_round_builtin() {
    let code = transpile("def round_it(x: float) -> int:\n    return round(x)");
    assert!(!code.is_empty(), "round: {}", code);
}

#[test]
fn test_chr_ord() {
    let code = transpile("def char_code(c: str) -> int:\n    return ord(c)");
    assert!(!code.is_empty(), "ord: {}", code);
}

#[test]
fn test_hex_builtin() {
    let code = transpile("def to_hex(n: int) -> str:\n    return hex(n)");
    assert!(!code.is_empty(), "hex: {}", code);
}

#[test]
fn test_bin_builtin() {
    let code = transpile("def to_bin(n: int) -> str:\n    return bin(n)");
    assert!(!code.is_empty(), "bin: {}", code);
}

#[test]
fn test_oct_builtin() {
    let code = transpile("def to_oct(n: int) -> str:\n    return oct(n)");
    assert!(!code.is_empty(), "oct: {}", code);
}

// =============================================================================
// Section 9: Multiple-file-like patterns (deeper codegen paths)
// =============================================================================

#[test]
fn test_global_list_constant() {
    let code = transpile("COLORS = [\"red\", \"green\", \"blue\"]\n\ndef first_color() -> str:\n    return COLORS[0]");
    assert!(!code.is_empty(), "global list: {}", code);
}

#[test]
fn test_global_dict_constant() {
    let code = transpile("CONFIG = {\"host\": \"localhost\", \"port\": 8080}\n\ndef get_host() -> str:\n    return CONFIG[\"host\"]");
    assert!(!code.is_empty(), "global dict: {}", code);
}

#[test]
fn test_multiline_string() {
    let code = transpile("def doc() -> str:\n    return \"\"\"line1\nline2\nline3\"\"\"");
    assert!(!code.is_empty(), "multiline string: {}", code);
}

#[test]
fn test_multiple_functions() {
    let code = transpile("def add(a: int, b: int) -> int:\n    return a + b\n\ndef sub(a: int, b: int) -> int:\n    return a - b\n\ndef mul(a: int, b: int) -> int:\n    return a * b");
    assert!(!code.is_empty(), "multi functions: {}", code);
}

#[test]
fn test_function_calling_function() {
    let code = transpile("def square(x: int) -> int:\n    return x * x\n\ndef sum_squares(n: int) -> int:\n    total = 0\n    for i in range(n):\n        total += square(i)\n    return total");
    assert!(!code.is_empty(), "func calling func: {}", code);
}

#[test]
fn test_recursive_function() {
    let code = transpile("def factorial(n: int) -> int:\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)");
    assert!(!code.is_empty(), "recursive: {}", code);
}

#[test]
fn test_default_arguments() {
    let code = transpile("def greet(name: str, greeting: str = \"Hello\") -> str:\n    return f\"{greeting}, {name}\"");
    assert!(!code.is_empty(), "default args: {}", code);
}

#[test]
fn test_varargs() {
    let code = transpile("def total(*args: int) -> int:\n    return sum(args)");
    assert!(!code.is_empty(), "varargs: {}", code);
}

#[test]
fn test_kwargs() {
    let code = transpile("def config(**kwargs: str) -> dict:\n    return kwargs");
    assert!(!code.is_empty(), "kwargs: {}", code);
}

#[test]
fn test_nested_function() {
    let code = transpile("def outer(x: int) -> int:\n    def inner(y: int) -> int:\n        return x + y\n    return inner(10)");
    assert!(!code.is_empty(), "nested function: {}", code);
}

#[test]
fn test_decorator_pattern() {
    let code = transpile("def log(func):\n    def wrapper(*args):\n        print(\"calling\")\n        return func(*args)\n    return wrapper");
    assert!(!code.is_empty(), "decorator: {}", code);
}

// =============================================================================
// Section 10: Data structure patterns
// =============================================================================

#[test]
fn test_list_of_tuples() {
    let code = transpile("def make() -> list:\n    return [(1, \"a\"), (2, \"b\"), (3, \"c\")]");
    assert!(!code.is_empty(), "list of tuples: {}", code);
}

#[test]
fn test_dict_of_lists() {
    let code = transpile("def make() -> dict:\n    return {\"a\": [1, 2], \"b\": [3, 4]}");
    assert!(!code.is_empty(), "dict of lists: {}", code);
}

#[test]
fn test_set_operations() {
    let code = transpile("def symmetric_diff(a: set, b: set) -> set:\n    return a ^ b");
    assert!(!code.is_empty(), "set xor: {}", code);
}

#[test]
fn test_frozen_set() {
    let code = transpile("def immutable() -> frozenset:\n    return frozenset([1, 2, 3])");
    assert!(!code.is_empty(), "frozenset: {}", code);
}

#[test]
fn test_string_multiplication_in_func() {
    let code = transpile("def banner(width: int) -> str:\n    line = \"=\" * width\n    return line");
    assert!(!code.is_empty(), "string mul: {}", code);
}

#[test]
fn test_list_multiplication() {
    let code = transpile("def zeros(n: int) -> list:\n    return [0] * n");
    assert!(!code.is_empty(), "list mul: {}", code);
}

#[test]
fn test_complex_dict_access() {
    let code = transpile("def get_nested(data: dict) -> str:\n    return data[\"users\"][0][\"name\"]");
    assert!(!code.is_empty(), "nested dict access: {}", code);
}

#[test]
fn test_conditional_dict_access() {
    let code = transpile("def safe_get(d: dict, key: str) -> str:\n    if key in d:\n        return d[key]\n    return \"\"");
    assert!(!code.is_empty(), "conditional dict: {}", code);
}
