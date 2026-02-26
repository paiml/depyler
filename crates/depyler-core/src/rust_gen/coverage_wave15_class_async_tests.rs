//! Wave 15: Coverage tests for class generation, async/await patterns, error handling,
//! and advanced function signatures.
//!
//! Tests target UNCOVERED code paths in:
//! - Class generation: __init__, __str__, __repr__, __eq__, __len__, __getitem__, __contains__,
//!   __iter__, @classmethod, @staticmethod, @property, inheritance, class variables, docstrings
//! - Async/await: async def, await expressions, async with params, async with try/except,
//!   async returning collections, async with loops
//! - Error handling: try/except, try/except/else, try/except/finally, multiple except,
//!   except as, raise, raise from, re-raise, custom exceptions, nested try, assert
//! - Advanced functions: default args, *args, **kwargs, mixed params, keyword-only,
//!   nested functions, closures, decorators, recursion, generators
//!
//! Status: 200 tests (test_w15ca_class_001..070, test_w15ca_async_001..040,
//!         test_w15ca_error_001..050, test_w15ca_func_001..040)

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
    // CLASS PATTERNS (70 tests: test_w15ca_class_001 through test_w15ca_class_070)
    // =========================================================================

    #[test]
    fn test_w15ca_class_001_basic_init_single_field() {
        let result = transpile("class Foo:\n    def __init__(self, x: int):\n        self.x = x\n");
        assert!(!result.is_empty());
        assert!(result.contains("struct") || result.contains("fn"));
    }

    #[test]
    fn test_w15ca_class_002_init_two_fields() {
        let result = transpile("class Point:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_003_init_string_field() {
        let result = transpile(
            "class Person:\n    def __init__(self, name: str):\n        self.name = name\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_004_init_bool_field() {
        let result = transpile(
            "class Toggle:\n    def __init__(self, active: bool):\n        self.active = active\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_005_init_float_field() {
        let result = transpile("class Measurement:\n    def __init__(self, value: float):\n        self.value = value\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_006_init_three_fields_mixed() {
        let result = transpile("class Record:\n    def __init__(self, name: str, age: int, score: float):\n        self.name = name\n        self.age = age\n        self.score = score\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_007_init_empty_list() {
        let result = transpile("class Bag:\n    def __init__(self):\n        self.items = []\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_008_init_zero_defaults() {
        let result = transpile("class Counter:\n    def __init__(self):\n        self.count = 0\n        self.total = 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_009_init_with_str_method() {
        let result = transpile("class Label:\n    def __init__(self, text: str):\n        self.text = text\n    def __str__(self) -> str:\n        return self.text\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_010_init_with_repr_method() {
        let result = transpile("class Tag:\n    def __init__(self, value: str):\n        self.value = value\n    def __repr__(self) -> str:\n        return self.value\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_011_str_and_repr_both() {
        let result = transpile("class Item:\n    def __init__(self, name: str):\n        self.name = name\n    def __str__(self) -> str:\n        return self.name\n    def __repr__(self) -> str:\n        return self.name\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_012_custom_method_no_params() {
        let result = transpile("class Timer:\n    def __init__(self):\n        self.elapsed = 0\n    def reset(self):\n        self.elapsed = 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_013_custom_method_with_params() {
        let result = transpile("class Accumulator:\n    def __init__(self):\n        self.total = 0\n    def add(self, value: int):\n        self.total = self.total + value\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_014_method_returns_int() {
        let result = transpile("class Wallet:\n    def __init__(self):\n        self.balance = 0\n    def get_balance(self) -> int:\n        return self.balance\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_015_method_returns_str() {
        let result = transpile("class Greeter:\n    def __init__(self, name: str):\n        self.name = name\n    def greet(self) -> str:\n        return self.name\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_016_method_returns_bool() {
        let result = transpile("class Gate:\n    def __init__(self):\n        self.open = False\n    def is_open(self) -> bool:\n        return self.open\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_017_multiple_custom_methods() {
        let result = transpile("class Bank:\n    def __init__(self):\n        self.balance = 0\n    def deposit(self, amount: int):\n        self.balance = self.balance + amount\n    def withdraw(self, amount: int):\n        self.balance = self.balance - amount\n    def get_balance(self) -> int:\n        return self.balance\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_018_inheritance_basic() {
        let result = transpile("class Animal:\n    def __init__(self):\n        self.alive = True\n\nclass Dog(Animal):\n    def __init__(self):\n        self.breed = \"unknown\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_019_inheritance_method_override() {
        let result = transpile("class Shape:\n    def area(self) -> float:\n        return 0.0\n\nclass Square(Shape):\n    def __init__(self, side: float):\n        self.side = side\n    def area(self) -> float:\n        return self.side * self.side\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_020_inheritance_with_new_method() {
        let result = transpile("class Vehicle:\n    def __init__(self):\n        self.speed = 0\n\nclass Car(Vehicle):\n    def __init__(self):\n        self.doors = 4\n    def honk(self) -> str:\n        return \"beep\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_021_classmethod_create() {
        let result = transpile("class Widget:\n    def __init__(self, name: str):\n        self.name = name\n    @classmethod\n    def create(cls, name: str):\n        return cls(name)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_022_classmethod_no_params() {
        let result = transpile("class Factory:\n    @classmethod\n    def build(cls):\n        return cls()\n    def __init__(self):\n        self.ready = True\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_023_staticmethod_utility() {
        let result = transpile("class MathUtil:\n    @staticmethod\n    def add(a: int, b: int) -> int:\n        return a + b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_024_staticmethod_no_args() {
        let result = transpile(
            "class Constants:\n    @staticmethod\n    def pi() -> float:\n        return 3.14159\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_025_staticmethod_string_util() {
        let result = transpile("class StringUtil:\n    @staticmethod\n    def to_upper(s: str) -> str:\n        return s.upper()\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_026_property_getter() {
        let result = transpile("class Circle:\n    def __init__(self, radius: float):\n        self.radius = radius\n    @property\n    def diameter(self) -> float:\n        return self.radius * 2.0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_027_property_from_field() {
        let result = transpile("class User:\n    def __init__(self, first: str, last: str):\n        self.first = first\n        self.last = last\n    @property\n    def full_name(self) -> str:\n        return self.first\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_028_eq_method_int() {
        let result = transpile("class Coord:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y\n    def __eq__(self, other) -> bool:\n        return self.x == other.x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_029_eq_method_string() {
        let result = transpile("class Token:\n    def __init__(self, value: str):\n        self.value = value\n    def __eq__(self, other) -> bool:\n        return self.value == other.value\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_030_len_method() {
        let result = transpile("class Shelf:\n    def __init__(self):\n        self.books = []\n    def __len__(self) -> int:\n        return len(self.books)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_031_getitem_method() {
        let result = transpile("class Registry:\n    def __init__(self):\n        self.data = {}\n    def __getitem__(self, key: str) -> str:\n        return self.data[key]\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_032_contains_method() {
        let result = transpile("class Collection:\n    def __init__(self):\n        self.items = []\n    def __contains__(self, item: int) -> bool:\n        return item in self.items\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_033_iter_method() {
        let result = transpile("class Sequence:\n    def __init__(self):\n        self.items = []\n    def __iter__(self):\n        return iter(self.items)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_034_dataclass_like_typed_init() {
        let result = transpile("class Config:\n    def __init__(self, host: str, port: int, debug: bool):\n        self.host = host\n        self.port = port\n        self.debug = debug\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_035_default_value_int() {
        let result = transpile("class Position:\n    def __init__(self, x: int = 0, y: int = 0):\n        self.x = x\n        self.y = y\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_036_default_value_string() {
        let result = transpile("class Logger:\n    def __init__(self, name: str = \"default\"):\n        self.name = name\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_037_default_value_bool() {
        let result = transpile("class Feature:\n    def __init__(self, enabled: bool = False):\n        self.enabled = enabled\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_038_method_calls_other_method() {
        let result = transpile("class Validator:\n    def __init__(self):\n        self.errors = 0\n    def check(self, value: int) -> bool:\n        if value < 0:\n            self.errors = self.errors + 1\n            return False\n        return True\n    def is_valid(self) -> bool:\n        return self.errors == 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_039_class_variable_int() {
        let result = transpile(
            "class Limits:\n    MAX = 100\n    def __init__(self):\n        self.current = 0\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_040_class_variable_string() {
        let result = transpile(
            "class Version:\n    NAME = \"v1\"\n    def __init__(self):\n        self.build = 0\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_041_class_docstring() {
        let result = transpile("class Helper:\n    \"\"\"A helper class.\"\"\"\n    def __init__(self):\n        self.active = True\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_042_method_docstring() {
        let result = transpile("class Processor:\n    def __init__(self):\n        self.count = 0\n    def run(self) -> int:\n        \"\"\"Run the processor.\"\"\"\n        return self.count\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_043_init_with_computation() {
        let result = transpile("class Doubler:\n    def __init__(self, value: int):\n        self.value = value\n        self.doubled = value * 2\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_044_method_with_conditional() {
        let result = transpile("class Clamp:\n    def __init__(self, lo: int, hi: int):\n        self.lo = lo\n        self.hi = hi\n    def apply(self, x: int) -> int:\n        if x < self.lo:\n            return self.lo\n        if x > self.hi:\n            return self.hi\n        return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_045_method_with_loop() {
        let result = transpile("class Summer:\n    def __init__(self):\n        self.values = []\n    def total(self) -> int:\n        s: int = 0\n        for v in self.values:\n            s = s + v\n        return s\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_046_two_classes_same_module() {
        let result = transpile("class First:\n    def __init__(self):\n        self.a = 1\n\nclass Second:\n    def __init__(self):\n        self.b = 2\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_047_class_with_free_function() {
        let result = transpile("class Data:\n    def __init__(self, x: int):\n        self.x = x\n\ndef create_data(v: int) -> int:\n    return v\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_048_class_method_modifies_field() {
        let result = transpile("class Stack:\n    def __init__(self):\n        self.items = []\n    def push(self, item: int):\n        self.items.append(item)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_049_class_method_returns_field_len() {
        let result = transpile("class Queue:\n    def __init__(self):\n        self.items = []\n    def size(self) -> int:\n        return len(self.items)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_050_class_with_pop_method() {
        let result = transpile("class Stack:\n    def __init__(self):\n        self.items = []\n    def pop(self) -> int:\n        return self.items.pop()\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_051_class_init_and_len_and_push() {
        let result = transpile("class Bag:\n    def __init__(self):\n        self.items = []\n    def __len__(self) -> int:\n        return len(self.items)\n    def add(self, item: int):\n        self.items.append(item)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_052_class_multiple_dunder_methods() {
        let result = transpile("class Val:\n    def __init__(self, v: int):\n        self.v = v\n    def __str__(self) -> str:\n        return str(self.v)\n    def __eq__(self, other) -> bool:\n        return self.v == other.v\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_053_staticmethod_multiple() {
        let result = transpile("class Math:\n    @staticmethod\n    def add(a: int, b: int) -> int:\n        return a + b\n    @staticmethod\n    def mul(a: int, b: int) -> int:\n        return a * b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_054_classmethod_with_typed_return() {
        let result = transpile("class Box:\n    def __init__(self, size: int):\n        self.size = size\n    @classmethod\n    def small(cls):\n        return cls(1)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_055_property_computed_value() {
        let result = transpile("class Rect:\n    def __init__(self, w: int, h: int):\n        self.w = w\n        self.h = h\n    @property\n    def area(self) -> int:\n        return self.w * self.h\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_056_property_and_regular_method() {
        let result = transpile("class Temp:\n    def __init__(self, celsius: float):\n        self.celsius = celsius\n    @property\n    def fahrenheit(self) -> float:\n        return self.celsius * 1.8 + 32.0\n    def reset(self):\n        self.celsius = 0.0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_057_inheritance_three_levels() {
        let result = transpile("class A:\n    def __init__(self):\n        self.a = 1\n\nclass B(A):\n    def __init__(self):\n        self.b = 2\n\nclass C(B):\n    def __init__(self):\n        self.c = 3\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_058_class_with_boolean_method() {
        let result = transpile("class Checker:\n    def __init__(self, threshold: int):\n        self.threshold = threshold\n    def passes(self, value: int) -> bool:\n        return value >= self.threshold\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_059_class_method_with_string_return() {
        let result = transpile("class Formatter:\n    def __init__(self, prefix: str):\n        self.prefix = prefix\n    def format(self, text: str) -> str:\n        return self.prefix + text\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_060_class_with_increment_decrement() {
        let result = transpile("class Counter:\n    def __init__(self):\n        self.val = 0\n    def inc(self):\n        self.val = self.val + 1\n    def dec(self):\n        self.val = self.val - 1\n    def get(self) -> int:\n        return self.val\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_061_class_with_multiply_method() {
        let result = transpile("class Scaler:\n    def __init__(self, factor: int):\n        self.factor = factor\n    def scale(self, x: int) -> int:\n        return x * self.factor\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_062_empty_class_with_pass() {
        let result = transpile("class Empty:\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_063_class_with_none_init() {
        let result = transpile("class Lazy:\n    def __init__(self):\n        self.data = None\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_064_class_with_negative_init() {
        let result = transpile(
            "class Offset:\n    def __init__(self):\n        self.x = -1\n        self.y = -1\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_065_class_with_string_concat_method() {
        let result = transpile("class Builder:\n    def __init__(self):\n        self.parts = \"\"\n    def append(self, s: str):\n        self.parts = self.parts + s\n    def build(self) -> str:\n        return self.parts\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_066_inheritance_with_str() {
        let result = transpile("class Base:\n    def __str__(self) -> str:\n        return \"base\"\n\nclass Derived(Base):\n    def __str__(self) -> str:\n        return \"derived\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_067_class_four_fields() {
        let result = transpile("class Connection:\n    def __init__(self, host: str, port: int, timeout: int, secure: bool):\n        self.host = host\n        self.port = port\n        self.timeout = timeout\n        self.secure = secure\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_068_class_method_returns_list() {
        let result = transpile("class Collector:\n    def __init__(self):\n        self.items = []\n    def get_items(self) -> list:\n        return self.items\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_069_class_method_with_two_params() {
        let result = transpile("class Calculator:\n    def __init__(self):\n        self.result = 0\n    def add(self, a: int, b: int) -> int:\n        self.result = a + b\n        return self.result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_class_070_class_with_staticmethod_and_init() {
        let result = transpile("class Util:\n    def __init__(self, name: str):\n        self.name = name\n    @staticmethod\n    def version() -> str:\n        return \"1.0\"\n    def get_name(self) -> str:\n        return self.name\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // ASYNC PATTERNS (40 tests: test_w15ca_async_001 through test_w15ca_async_040)
    // =========================================================================

    #[test]
    fn test_w15ca_async_001_basic_return_int() {
        let result = transpile("async def fetch() -> int:\n    return 1\n");
        assert!(!result.is_empty());
        assert!(result.contains("async") || result.contains("fn"));
    }

    #[test]
    fn test_w15ca_async_002_basic_return_string() {
        let result = transpile("async def greet() -> str:\n    return \"hello\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_003_basic_no_return() {
        let result = transpile("async def noop():\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_004_with_await() {
        let result = transpile("async def get_data():\n    x = await fetch()\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_005_await_in_assignment() {
        let result =
            transpile("async def load():\n    result = await read_file()\n    return result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_006_with_str_param() {
        let result = transpile("async def fetch_url(url: str) -> str:\n    return url\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_007_with_int_param() {
        let result = transpile("async def compute(n: int) -> int:\n    return n * 2\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_008_with_two_params() {
        let result = transpile("async def combine(a: str, b: str) -> str:\n    return a + b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_009_with_bool_return() {
        let result = transpile("async def check() -> bool:\n    return True\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_010_with_float_return() {
        let result = transpile("async def measure() -> float:\n    return 3.14\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_011_with_conditional() {
        let result = transpile("async def decide(x: int) -> str:\n    if x > 0:\n        return \"positive\"\n    return \"non-positive\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_012_await_with_param() {
        let result = transpile(
            "async def request(url: str):\n    data = await fetch(url)\n    return data\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_013_multiple_awaits() {
        let result = transpile(
            "async def pipeline():\n    a = await step1()\n    b = await step2()\n    return a\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_014_await_in_conditional() {
        let result = transpile("async def conditional_fetch(flag: bool):\n    if flag:\n        result = await fetch_a()\n    else:\n        result = await fetch_b()\n    return result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_015_async_with_print() {
        let result =
            transpile("async def log_data():\n    data = await get_data()\n    print(data)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_016_async_return_computed() {
        let result = transpile("async def double(x: int) -> int:\n    return x * 2\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_017_async_with_local_var() {
        let result = transpile(
            "async def process():\n    count: int = 0\n    count = count + 1\n    return count\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_018_async_with_string_ops() {
        let result = transpile("async def transform(s: str) -> str:\n    return s.upper()\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_019_async_with_comparison() {
        let result = transpile("async def is_positive(n: int) -> bool:\n    return n > 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_020_async_void() {
        let result = transpile("async def fire_and_forget(msg: str):\n    print(msg)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_021_async_for_loop_body() {
        let result = transpile(
            "async def process_items(items: list):\n    for item in items:\n        print(item)\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_022_async_with_range_loop() {
        let result = transpile("async def count_up(n: int) -> int:\n    total: int = 0\n    for i in range(n):\n        total = total + i\n    return total\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_023_async_while_loop() {
        let result = transpile("async def wait_loop():\n    count: int = 0\n    while count < 10:\n        count = count + 1\n    return count\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_024_async_nested_if() {
        let result = transpile("async def categorize(x: int) -> str:\n    if x > 100:\n        return \"high\"\n    elif x > 50:\n        return \"medium\"\n    else:\n        return \"low\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_025_async_multiple_params_types() {
        let result = transpile(
            "async def handle(name: str, count: int, flag: bool) -> str:\n    return name\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_026_async_return_none() {
        let result = transpile("async def do_nothing():\n    x: int = 1\n    return None\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_027_async_with_assert() {
        let result =
            transpile("async def validate(x: int) -> int:\n    assert x > 0\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_028_async_return_bool_expr() {
        let result = transpile(
            "async def both_positive(a: int, b: int) -> bool:\n    return a > 0 and b > 0\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_029_async_with_string_concat() {
        let result = transpile(
            "async def build_msg(prefix: str, body: str) -> str:\n    return prefix + body\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_030_async_with_multiplication() {
        let result = transpile("async def area(w: int, h: int) -> int:\n    return w * h\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_031_async_with_division() {
        let result = transpile("async def half(x: float) -> float:\n    return x / 2.0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_032_async_with_negation() {
        let result = transpile("async def negate(x: int) -> int:\n    return -x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_033_async_with_modulo() {
        let result = transpile("async def is_even(n: int) -> bool:\n    return n % 2 == 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_034_two_async_functions() {
        let result = transpile(
            "async def first() -> int:\n    return 1\n\nasync def second() -> int:\n    return 2\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_035_async_and_sync_together() {
        let result = transpile(
            "def sync_fn() -> int:\n    return 1\n\nasync def async_fn() -> int:\n    return 2\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_036_async_with_early_return() {
        let result = transpile(
            "async def guard(x: int) -> int:\n    if x < 0:\n        return 0\n    return x\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_037_async_arithmetic_chain() {
        let result =
            transpile("async def calc(a: int, b: int, c: int) -> int:\n    return a + b * c\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_038_async_return_list_literal() {
        let result = transpile("async def items() -> list:\n    return [1, 2, 3]\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_039_async_return_empty_string() {
        let result = transpile("async def blank() -> str:\n    return \"\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_async_040_async_return_zero() {
        let result = transpile("async def zero() -> int:\n    return 0\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // ERROR HANDLING (50 tests: test_w15ca_error_001 through test_w15ca_error_050)
    // =========================================================================

    #[test]
    fn test_w15ca_error_001_basic_try_except() {
        let result = transpile("def safe_div(a: int, b: int) -> int:\n    try:\n        return a // b\n    except:\n        return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_002_try_except_zero_division() {
        let result = transpile("def divide(a: int, b: int) -> int:\n    try:\n        return a // b\n    except ZeroDivisionError:\n        return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_003_try_except_value_error() {
        let result = transpile("def parse_int(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError:\n        return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_004_try_except_type_error() {
        let result = transpile("def safe_add(a: int, b: int) -> int:\n    try:\n        return a + b\n    except TypeError:\n        return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_005_try_except_key_error() {
        let result = transpile("def safe_get(d: dict, k: str) -> str:\n    try:\n        return d[k]\n    except KeyError:\n        return \"\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_006_try_except_index_error() {
        let result = transpile("def safe_index(items: list, i: int) -> int:\n    try:\n        return items[i]\n    except IndexError:\n        return -1\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_007_try_except_else() {
        let result = transpile("def try_else(x: int) -> int:\n    try:\n        result = x * 2\n    except:\n        result = 0\n    else:\n        result = result + 1\n    return result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_008_try_except_finally() {
        let result = transpile("def with_cleanup() -> int:\n    x: int = 0\n    try:\n        x = 1\n    except:\n        x = -1\n    finally:\n        print(x)\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_009_try_except_else_finally() {
        let result = transpile("def full_try(x: int) -> int:\n    result: int = 0\n    try:\n        result = x * 2\n    except:\n        result = -1\n    else:\n        result = result + 1\n    finally:\n        print(result)\n    return result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_010_multiple_except_types() {
        let result = transpile("def safe_op(x: int) -> int:\n    try:\n        return x * 2\n    except ValueError:\n        return -1\n    except TypeError:\n        return -2\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_011_except_as_variable() {
        let result = transpile("def handle_error(x: int) -> str:\n    try:\n        return str(x)\n    except ValueError as e:\n        return \"error\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_012_raise_value_error() {
        let result = transpile("def validate(x: int) -> int:\n    if x < 0:\n        raise ValueError(\"negative\")\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_013_raise_type_error() {
        let result = transpile("def check_type(x: str) -> str:\n    if not x:\n        raise TypeError(\"empty\")\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_014_raise_runtime_error() {
        let result =
            transpile("def fail() -> int:\n    raise RuntimeError(\"failed\")\n    return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_015_raise_key_error() {
        let result = transpile("def lookup(d: dict, k: str) -> str:\n    if k not in d:\n        raise KeyError(\"missing\")\n    return d[k]\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_016_raise_index_error() {
        let result = transpile("def get_item(items: list, idx: int) -> int:\n    if idx >= len(items):\n        raise IndexError(\"out of bounds\")\n    return items[idx]\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_017_raise_no_message() {
        let result = transpile("def boom():\n    raise ValueError(\"boom\")\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_018_bare_raise() {
        let result =
            transpile("def reraise():\n    try:\n        x = 1\n    except:\n        raise\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_019_custom_exception_class() {
        let result = transpile("class CustomError(Exception):\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_020_custom_exception_with_init() {
        let result = transpile("class AppError(Exception):\n    def __init__(self, code: int):\n        self.code = code\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_021_nested_try_inner() {
        let result = transpile("def nested() -> int:\n    try:\n        try:\n            return 1\n        except:\n            return 2\n    except:\n        return 3\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_022_assert_simple() {
        let result = transpile("def check(x: int) -> int:\n    assert x > 0\n    return x\n");
        assert!(!result.is_empty());
        assert!(result.contains("assert") || result.contains("panic"));
    }

    #[test]
    fn test_w15ca_error_023_assert_with_message() {
        let result = transpile(
            "def check_msg(x: int) -> int:\n    assert x > 0, \"must be positive\"\n    return x\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_024_try_with_return_in_body() {
        let result = transpile("def safe_compute(x: int) -> int:\n    try:\n        return x * x\n    except:\n        return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_025_try_with_assignment() {
        let result = transpile("def safe_parse(s: str) -> int:\n    result: int = 0\n    try:\n        result = int(s)\n    except:\n        result = -1\n    return result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_026_try_with_function_call() {
        let result = transpile("def safe_call() -> int:\n    try:\n        x = int(\"42\")\n        return x\n    except:\n        return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_027_except_runtime_error() {
        let result = transpile("def recover() -> str:\n    try:\n        return \"ok\"\n    except RuntimeError:\n        return \"failed\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_028_except_attribute_error() {
        let result = transpile("def safe_attr() -> str:\n    try:\n        return \"value\"\n    except AttributeError:\n        return \"missing\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_029_except_io_error() {
        let result = transpile("def safe_read() -> str:\n    try:\n        return \"data\"\n    except IOError:\n        return \"\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_030_try_in_loop() {
        let result = transpile("def process_items(items: list) -> int:\n    count: int = 0\n    for item in items:\n        try:\n            count = count + 1\n        except:\n            pass\n    return count\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_031_try_with_while() {
        let result = transpile("def retry() -> int:\n    attempts: int = 0\n    while attempts < 3:\n        try:\n            return 1\n        except:\n            attempts = attempts + 1\n    return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_032_raise_in_if() {
        let result = transpile("def guard(x: int) -> int:\n    if x == 0:\n        raise ValueError(\"zero not allowed\")\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_033_raise_in_elif() {
        let result = transpile("def classify(x: int) -> str:\n    if x > 0:\n        return \"positive\"\n    elif x == 0:\n        raise ValueError(\"zero\")\n    return \"negative\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_034_raise_in_else() {
        let result = transpile("def require_positive(x: int) -> int:\n    if x > 0:\n        return x\n    else:\n        raise ValueError(\"not positive\")\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_035_multiple_raises() {
        let result = transpile("def strict_validate(x: int) -> int:\n    if x < 0:\n        raise ValueError(\"negative\")\n    if x > 100:\n        raise ValueError(\"too large\")\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_036_raise_file_not_found() {
        let result = transpile("def open_file(path: str) -> str:\n    raise FileNotFoundError(\"not found\")\n    return \"\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_037_raise_stop_iteration() {
        let result =
            transpile("def done() -> int:\n    raise StopIteration(\"end\")\n    return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_038_try_except_with_print() {
        let result = transpile("def log_error() -> int:\n    try:\n        return 1\n    except:\n        print(\"error\")\n        return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_039_finally_cleanup() {
        let result = transpile("def cleanup() -> int:\n    x: int = 0\n    try:\n        x = 42\n    finally:\n        print(\"done\")\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_040_try_multiple_statements() {
        let result = transpile("def multi_step() -> int:\n    try:\n        a: int = 1\n        b: int = 2\n        return a + b\n    except:\n        return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_041_except_with_pass() {
        let result = transpile("def suppress() -> int:\n    try:\n        return 1\n    except:\n        pass\n    return 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_042_try_assign_and_use() {
        let result = transpile("def compute() -> int:\n    result: int = 0\n    try:\n        result = 10 * 2\n    except:\n        result = -1\n    return result\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_043_try_string_operation() {
        let result = transpile("def safe_upper(s: str) -> str:\n    try:\n        return s.upper()\n    except:\n        return \"\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_044_raise_overflow_error() {
        let result = transpile("def check_overflow(x: int) -> int:\n    if x > 1000000:\n        raise OverflowError(\"too big\")\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_045_raise_not_implemented() {
        let result = transpile(
            "def stub() -> int:\n    raise NotImplementedError(\"not done\")\n    return 0\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_046_assert_equality() {
        let result = transpile("def check_eq(a: int, b: int):\n    assert a == b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_047_assert_not_none() {
        let result = transpile("def check_exists(x: str):\n    assert len(x) > 0\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_048_try_bool_return() {
        let result = transpile("def safe_check(x: int) -> bool:\n    try:\n        return x > 0\n    except:\n        return False\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_049_except_then_raise() {
        let result = transpile("def rethrow(x: int) -> int:\n    try:\n        return x\n    except ValueError:\n        raise RuntimeError(\"converted\")\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_error_050_try_with_local_vars() {
        let result = transpile("def multi_try() -> int:\n    a: int = 0\n    b: int = 0\n    try:\n        a = 10\n        b = 20\n    except:\n        a = -1\n        b = -1\n    return a + b\n");
        assert!(!result.is_empty());
    }

    // =========================================================================
    // ADVANCED FUNCTIONS (40 tests: test_w15ca_func_001 through test_w15ca_func_040)
    // =========================================================================

    #[test]
    fn test_w15ca_func_001_default_int_arg() {
        let result = transpile("def greet(times: int = 1) -> int:\n    return times\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_002_default_str_arg() {
        let result = transpile("def hello(name: str = \"world\") -> str:\n    return name\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_003_default_bool_arg() {
        let result = transpile("def toggle(flag: bool = False) -> bool:\n    return not flag\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_004_default_float_arg() {
        let result = transpile("def scale(factor: float = 1.0) -> float:\n    return factor\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_005_two_default_args() {
        let result = transpile("def point(x: int = 0, y: int = 0) -> int:\n    return x + y\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_006_mixed_required_and_default() {
        let result = transpile("def tag(name: str, value: int = 0) -> str:\n    return name\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_007_three_defaults() {
        let result = transpile(
            "def config(a: int = 1, b: int = 2, c: int = 3) -> int:\n    return a + b + c\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_008_args_variadic() {
        let result = transpile("def variadic(*args) -> int:\n    return len(args)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_009_kwargs_dict() {
        let result = transpile("def options(**kwargs) -> int:\n    return len(kwargs)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_010_args_and_kwargs() {
        let result = transpile("def mixed(*args, **kwargs) -> int:\n    return len(args)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_011_positional_and_args() {
        let result = transpile("def prefix(first: int, *args) -> int:\n    return first\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_012_nested_function_basic() {
        let result = transpile(
            "def outer() -> int:\n    def inner() -> int:\n        return 42\n    return inner()\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_013_nested_function_with_params() {
        let result = transpile("def outer(x: int) -> int:\n    def inner(y: int) -> int:\n        return y + 1\n    return inner(x)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_014_nested_function_closure() {
        let result = transpile("def make_adder(n: int):\n    def adder(x: int) -> int:\n        return x + n\n    return adder\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_015_nested_function_two_inner() {
        let result = transpile("def pipeline(x: int) -> int:\n    def step1(v: int) -> int:\n        return v + 1\n    def step2(v: int) -> int:\n        return v * 2\n    return step2(step1(x))\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_016_recursive_factorial() {
        let result = transpile("def factorial(n: int) -> int:\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)\n");
        assert!(!result.is_empty());
        assert!(result.contains("factorial"));
    }

    #[test]
    fn test_w15ca_func_017_recursive_fibonacci() {
        let result = transpile("def fib(n: int) -> int:\n    if n <= 1:\n        return n\n    return fib(n - 1) + fib(n - 2)\n");
        assert!(!result.is_empty());
        assert!(result.contains("fib"));
    }

    #[test]
    fn test_w15ca_func_018_recursive_sum() {
        let result = transpile("def sum_to(n: int) -> int:\n    if n <= 0:\n        return 0\n    return n + sum_to(n - 1)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_019_generator_simple() {
        let result = transpile("def gen():\n    yield 1\n    yield 2\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_020_generator_with_loop() {
        let result = transpile("def count_gen(n: int):\n    for i in range(n):\n        yield i\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_021_generator_three_yields() {
        let result = transpile("def triple():\n    yield 1\n    yield 2\n    yield 3\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_022_generator_yield_computed() {
        let result =
            transpile("def doubled(n: int):\n    for i in range(n):\n        yield i * 2\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_023_lambda_in_assignment() {
        let result = transpile(
            "def use_lambda() -> int:\n    double = lambda x: x * 2\n    return double(5)\n",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_024_function_with_multiple_returns() {
        let result = transpile("def classify(x: int) -> str:\n    if x > 0:\n        return \"positive\"\n    elif x < 0:\n        return \"negative\"\n    else:\n        return \"zero\"\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_025_function_void_with_print() {
        let result = transpile("def say_hello(name: str):\n    print(name)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_026_function_no_params_no_return() {
        let result = transpile("def noop():\n    pass\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_027_function_single_expression() {
        let result = transpile("def identity(x: int) -> int:\n    return x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_028_function_complex_expression() {
        let result = transpile("def quadratic(a: int, b: int, c: int, x: int) -> int:\n    return a * x * x + b * x + c\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_029_function_with_list_param() {
        let result = transpile("def sum_list(items: list) -> int:\n    total: int = 0\n    for item in items:\n        total = total + item\n    return total\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_030_function_with_dict_param() {
        let result = transpile("def dict_size(d: dict) -> int:\n    return len(d)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_031_function_returning_tuple() {
        let result = transpile("def min_max(a: int, b: int) -> tuple:\n    if a < b:\n        return (a, b)\n    return (b, a)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_032_function_with_string_ops() {
        let result = transpile("def shout(s: str) -> str:\n    return s.upper()\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_033_function_with_len() {
        let result = transpile("def length(s: str) -> int:\n    return len(s)\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_034_function_many_params() {
        let result = transpile("def many(a: int, b: int, c: int, d: int, e: int) -> int:\n    return a + b + c + d + e\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_035_function_with_boolean_ops() {
        let result = transpile("def both(a: bool, b: bool) -> bool:\n    return a and b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_036_function_or_op() {
        let result = transpile("def either(a: bool, b: bool) -> bool:\n    return a or b\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_037_function_not_op() {
        let result = transpile("def negate(a: bool) -> bool:\n    return not a\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_038_function_chained_comparison() {
        let result = transpile("def in_range(x: int) -> bool:\n    return 0 < x and x < 100\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_039_function_ternary_return() {
        let result = transpile("def abs_val(x: int) -> int:\n    return x if x >= 0 else -x\n");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w15ca_func_040_function_with_augmented_assign() {
        let result = transpile("def accumulate(items: list) -> int:\n    total: int = 0\n    for x in items:\n        total += x\n    return total\n");
        assert!(!result.is_empty());
    }
}
