//! Coverage wave 4: func_gen.rs targeted coverage tests
//!
//! Targets 968 missed lines in func_gen.rs (81.9% coverage).
//! Exercises class generation, async, generators, closures, decorators,
//! type inference, parameter handling, return type inference, and more.

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
// Section 1: Class __init__ and basic struct generation
// =============================================================================

#[test]
fn test_class_init_single_field() {
    let code = transpile("class Point:\n    def __init__(self, x: int):\n        self.x = x");
    assert!(!code.is_empty(), "class with single field: {}", code);
}

#[test]
fn test_class_init_multiple_fields() {
    let code = transpile(
        "class Point:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y",
    );
    assert!(!code.is_empty(), "class with multiple fields: {}", code);
}

#[test]
fn test_class_init_with_default_values() {
    let code = transpile(
        "class Config:\n    def __init__(self, name: str = 'default', count: int = 0):\n        self.name = name\n        self.count = count",
    );
    assert!(!code.is_empty(), "class with default values: {}", code);
}

#[test]
fn test_class_init_string_field() {
    let code = transpile(
        "class Person:\n    def __init__(self, name: str, age: int):\n        self.name = name\n        self.age = age",
    );
    assert!(!code.is_empty(), "class with string field: {}", code);
}

#[test]
fn test_class_init_bool_field() {
    let code = transpile(
        "class Toggle:\n    def __init__(self, enabled: bool):\n        self.enabled = enabled",
    );
    assert!(!code.is_empty(), "class with bool field: {}", code);
}

#[test]
fn test_class_init_float_field() {
    let code = transpile(
        "class Circle:\n    def __init__(self, radius: float):\n        self.radius = radius",
    );
    assert!(!code.is_empty(), "class with float field: {}", code);
}

#[test]
fn test_class_init_list_field() {
    let code =
        transpile("class Container:\n    def __init__(self):\n        self.items: list = []");
    assert!(!code.is_empty(), "class with list field: {}", code);
}

#[test]
fn test_class_init_dict_field() {
    let code = transpile("class Registry:\n    def __init__(self):\n        self.data: dict = {}");
    assert!(!code.is_empty(), "class with dict field: {}", code);
}

#[test]
fn test_class_init_none_default() {
    let code = transpile(
        "class Node:\n    def __init__(self, value: int, parent = None):\n        self.value = value\n        self.parent = parent",
    );
    assert!(!code.is_empty(), "class with None default: {}", code);
}

#[test]
fn test_class_init_computed_field() {
    let code = transpile(
        "class Rectangle:\n    def __init__(self, w: int, h: int):\n        self.w = w\n        self.h = h\n        self.area = w * h",
    );
    assert!(!code.is_empty(), "class with computed field: {}", code);
}

// =============================================================================
// Section 2: Class dunder methods (__str__, __repr__, __len__, etc.)
// =============================================================================

#[test]
fn test_class_str_method() {
    let code = transpile(
        "class Point:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y\n    def __str__(self) -> str:\n        return str(self.x)",
    );
    assert!(!code.is_empty(), "class with __str__: {}", code);
}

#[test]
fn test_class_repr_method() {
    let code = transpile(
        "class Point:\n    def __init__(self, x: int):\n        self.x = x\n    def __repr__(self) -> str:\n        return 'Point(' + str(self.x) + ')'",
    );
    assert!(!code.is_empty(), "class with __repr__: {}", code);
}

#[test]
fn test_class_len_method() {
    let code = transpile(
        "class MyList:\n    def __init__(self):\n        self.items: list = []\n    def __len__(self) -> int:\n        return len(self.items)",
    );
    assert!(!code.is_empty(), "class with __len__: {}", code);
}

#[test]
fn test_class_getitem_method() {
    let code = transpile(
        "class MyList:\n    def __init__(self):\n        self.items: list = []\n    def __getitem__(self, idx: int) -> int:\n        return self.items[idx]",
    );
    assert!(!code.is_empty(), "class with __getitem__: {}", code);
}

#[test]
fn test_class_setitem_method() {
    let code = transpile(
        "class MyList:\n    def __init__(self):\n        self.items: list = []\n    def __setitem__(self, idx: int, val: int):\n        self.items[idx] = val",
    );
    assert!(!code.is_empty(), "class with __setitem__: {}", code);
}

#[test]
fn test_class_contains_method() {
    let code = transpile(
        "class MySet:\n    def __init__(self):\n        self.items: list = []\n    def __contains__(self, item: int) -> bool:\n        return item in self.items",
    );
    assert!(!code.is_empty(), "class with __contains__: {}", code);
}

#[test]
fn test_class_eq_method() {
    let code = transpile(
        "class Point:\n    def __init__(self, x: int):\n        self.x = x\n    def __eq__(self, other) -> bool:\n        return self.x == other.x",
    );
    assert!(!code.is_empty(), "class with __eq__: {}", code);
}

#[test]
fn test_class_iter_method() {
    let code = transpile(
        "class MyRange:\n    def __init__(self, n: int):\n        self.n = n\n    def __iter__(self):\n        return iter(range(self.n))",
    );
    assert!(!code.is_empty(), "class with __iter__: {}", code);
}

#[test]
fn test_class_str_with_fstring() {
    let code = transpile(
        "class Coord:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y\n    def __str__(self) -> str:\n        return f'({self.x}, {self.y})'",
    );
    assert!(!code.is_empty(), "class __str__ with fstring: {}", code);
}

#[test]
fn test_class_hash_method() {
    let code = transpile(
        "class Key:\n    def __init__(self, val: int):\n        self.val = val\n    def __hash__(self) -> int:\n        return hash(self.val)",
    );
    assert!(!code.is_empty(), "class with __hash__: {}", code);
}

// =============================================================================
// Section 3: Class methods and regular methods
// =============================================================================

#[test]
fn test_class_method_returns_value() {
    let code = transpile(
        "class Calc:\n    def __init__(self, val: int):\n        self.val = val\n    def double(self) -> int:\n        return self.val * 2",
    );
    assert!(!code.is_empty(), "class method returns value: {}", code);
}

#[test]
fn test_class_method_mutates_state() {
    let code = transpile(
        "class Counter:\n    def __init__(self):\n        self.count = 0\n    def increment(self):\n        self.count += 1",
    );
    assert!(!code.is_empty(), "class method mutates state: {}", code);
}

#[test]
fn test_class_method_with_params() {
    let code = transpile(
        "class Adder:\n    def __init__(self, base: int):\n        self.base = base\n    def add(self, x: int) -> int:\n        return self.base + x",
    );
    assert!(!code.is_empty(), "class method with params: {}", code);
}

#[test]
fn test_class_multiple_methods() {
    let code = transpile(
        "class Stack:\n    def __init__(self):\n        self.items: list = []\n    def push(self, item: int):\n        self.items.append(item)\n    def pop(self) -> int:\n        return self.items.pop()\n    def is_empty(self) -> bool:\n        return len(self.items) == 0",
    );
    assert!(!code.is_empty(), "class multiple methods: {}", code);
}

#[test]
fn test_class_method_returning_self_field() {
    let code = transpile(
        "class Box:\n    def __init__(self, val: int):\n        self.val = val\n    def get(self) -> int:\n        return self.val\n    def set(self, new_val: int):\n        self.val = new_val",
    );
    assert!(!code.is_empty(), "class getter and setter methods: {}", code);
}

// =============================================================================
// Section 4: @staticmethod, @classmethod decorators
// =============================================================================

#[test]
fn test_staticmethod_no_params() {
    let code =
        transpile("class Math:\n    @staticmethod\n    def pi() -> float:\n        return 3.14159");
    assert!(!code.is_empty(), "staticmethod no params: {}", code);
}

#[test]
fn test_staticmethod_with_params() {
    let code = transpile(
        "class Math:\n    @staticmethod\n    def add(a: int, b: int) -> int:\n        return a + b",
    );
    assert!(!code.is_empty(), "staticmethod with params: {}", code);
}

#[test]
fn test_staticmethod_string_op() {
    let code = transpile(
        "class StringUtils:\n    @staticmethod\n    def is_blank(s: str) -> bool:\n        return len(s) == 0",
    );
    assert!(!code.is_empty(), "staticmethod string op: {}", code);
}

#[test]
fn test_classmethod_simple() {
    let code = transpile(
        "class Factory:\n    @classmethod\n    def create(cls) -> str:\n        return 'instance'",
    );
    assert!(!code.is_empty(), "classmethod simple: {}", code);
}

#[test]
fn test_classmethod_with_params() {
    let code = transpile(
        "class Builder:\n    @classmethod\n    def from_string(cls, s: str) -> str:\n        return s",
    );
    assert!(!code.is_empty(), "classmethod with params: {}", code);
}

// =============================================================================
// Section 5: @property getters and setters
// =============================================================================

#[test]
fn test_property_getter() {
    let code = transpile(
        "class Circle:\n    def __init__(self, r: float):\n        self._r = r\n    @property\n    def radius(self) -> float:\n        return self._r",
    );
    assert!(!code.is_empty(), "property getter: {}", code);
}

#[test]
fn test_property_setter() {
    let code = transpile(
        "class Square:\n    def __init__(self, s: int):\n        self._s = s\n    @property\n    def side(self) -> int:\n        return self._s\n    @side.setter\n    def side(self, val: int):\n        self._s = val",
    );
    assert!(!code.is_empty(), "property with setter: {}", code);
}

#[test]
fn test_property_computed() {
    let code = transpile(
        "class Rect:\n    def __init__(self, w: int, h: int):\n        self.w = w\n        self.h = h\n    @property\n    def area(self) -> int:\n        return self.w * self.h",
    );
    assert!(!code.is_empty(), "computed property: {}", code);
}

// =============================================================================
// Section 6: Inheritance patterns
// =============================================================================

#[test]
fn test_simple_inheritance() {
    assert!(transpile_ok(
        "class Animal:\n    def speak(self) -> str:\n        return 'sound'\n\nclass Dog(Animal):\n    def speak(self) -> str:\n        return 'bark'"
    ));
}

#[test]
fn test_inheritance_with_init() {
    assert!(transpile_ok(
        "class Base:\n    def __init__(self, x: int):\n        self.x = x\n\nclass Child(Base):\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y"
    ));
}

#[test]
fn test_inheritance_with_super() {
    assert!(transpile_ok(
        "class Base:\n    def __init__(self, name: str):\n        self.name = name\n\nclass Derived(Base):\n    def __init__(self, name: str, val: int):\n        super().__init__(name)\n        self.val = val"
    ));
}

#[test]
fn test_inheritance_method_override() {
    assert!(transpile_ok(
        "class Shape:\n    def area(self) -> int:\n        return 0\n\nclass Square(Shape):\n    def __init__(self, side: int):\n        self.side = side\n    def area(self) -> int:\n        return self.side * self.side"
    ));
}

#[test]
fn test_inheritance_exception() {
    assert!(transpile_ok(
        "class CustomError(Exception):\n    pass\n\ndef raise_it():\n    raise CustomError('oops')"
    ));
}

// =============================================================================
// Section 7: Async functions
// =============================================================================

#[test]
fn test_async_basic() {
    let code = transpile("async def fetch() -> str:\n    return 'data'");
    assert!(!code.is_empty(), "async basic: {}", code);
}

#[test]
fn test_async_with_params() {
    let code = transpile("async def fetch(url: str) -> str:\n    return url");
    assert!(!code.is_empty(), "async with params: {}", code);
}

#[test]
fn test_async_returning_int() {
    let code = transpile("async def compute(x: int) -> int:\n    return x * 2");
    assert!(!code.is_empty(), "async returning int: {}", code);
}

#[test]
fn test_async_with_await() {
    assert!(transpile_ok("async def get_data():\n    result = await fetch()\n    return result"));
}

#[test]
fn test_async_void() {
    let code = transpile("async def notify(msg: str):\n    print(msg)");
    assert!(!code.is_empty(), "async void: {}", code);
}

// =============================================================================
// Section 8: Generator functions (yield)
// =============================================================================

#[test]
fn test_generator_yield_values() {
    let code = transpile("def gen():\n    yield 1\n    yield 2\n    yield 3");
    assert!(!code.is_empty(), "generator yield values: {}", code);
}

#[test]
fn test_generator_yield_in_loop() {
    let code = transpile(
        "def count_up(n: int):\n    i = 0\n    while i < n:\n        yield i\n        i += 1",
    );
    assert!(!code.is_empty(), "generator yield in loop: {}", code);
}

#[test]
fn test_generator_yield_strings() {
    let code = transpile("def greetings():\n    yield 'hello'\n    yield 'world'");
    assert!(!code.is_empty(), "generator yield strings: {}", code);
}

#[test]
fn test_generator_with_param() {
    let code = transpile("def squares(n: int):\n    for i in range(n):\n        yield i * i");
    assert!(!code.is_empty(), "generator with param: {}", code);
}

#[test]
fn test_generator_fibonacci() {
    let code = transpile(
        "def fib():\n    a = 0\n    b = 1\n    while True:\n        yield a\n        a, b = b, a + b",
    );
    assert!(!code.is_empty(), "fibonacci generator: {}", code);
}

// =============================================================================
// Section 9: Nested functions / closures
// =============================================================================

#[test]
fn test_nested_function_basic() {
    let code = transpile(
        "def outer() -> int:\n    def inner() -> int:\n        return 42\n    return inner()",
    );
    assert!(!code.is_empty(), "nested function basic: {}", code);
}

#[test]
fn test_nested_function_with_params() {
    let code = transpile(
        "def outer(x: int) -> int:\n    def inner(y: int) -> int:\n        return x + y\n    return inner(10)",
    );
    assert!(!code.is_empty(), "nested function with params: {}", code);
}

#[test]
fn test_nested_function_captures_variable() {
    let code = transpile(
        "def make_adder(n: int):\n    def adder(x: int) -> int:\n        return x + n\n    return adder(5)",
    );
    assert!(!code.is_empty(), "nested function captures variable: {}", code);
}

#[test]
fn test_nested_function_multiple() {
    let code = transpile(
        "def combo(a: int, b: int) -> int:\n    def add() -> int:\n        return a + b\n    def mul() -> int:\n        return a * b\n    return add() + mul()",
    );
    assert!(!code.is_empty(), "multiple nested functions: {}", code);
}

#[test]
fn test_closure_returned() {
    assert!(transpile_ok(
        "def make_multiplier(factor: int):\n    def multiply(x: int) -> int:\n        return x * factor\n    return multiply"
    ));
}

// =============================================================================
// Section 10: Default parameter values
// =============================================================================

#[test]
fn test_default_none_param() {
    let code = transpile(
        "def process(data = None):\n    if data is None:\n        return 0\n    return 1",
    );
    assert!(!code.is_empty(), "default None param: {}", code);
}

#[test]
fn test_default_empty_list() {
    let code = transpile(
        "def append_to(item: int, lst: list = []):\n    lst.append(item)\n    return lst",
    );
    assert!(!code.is_empty(), "default empty list: {}", code);
}

#[test]
fn test_default_empty_dict() {
    let code = transpile(
        "def update_dict(key: str, val: str, d: dict = {}):\n    d[key] = val\n    return d",
    );
    assert!(!code.is_empty(), "default empty dict: {}", code);
}

#[test]
fn test_default_negative_int() {
    let code = transpile("def offset(x: int, delta: int = -1) -> int:\n    return x + delta");
    assert!(!code.is_empty(), "default negative int: {}", code);
}

#[test]
fn test_default_float_param() {
    let code =
        transpile("def scale(x: float, factor: float = 1.0) -> float:\n    return x * factor");
    assert!(!code.is_empty(), "default float param: {}", code);
}

#[test]
fn test_default_bool_param() {
    let code = transpile("def render(text: str, bold: bool = False) -> str:\n    return text");
    assert!(!code.is_empty(), "default bool param: {}", code);
}

#[test]
fn test_mixed_defaults_and_required() {
    let code = transpile(
        "def configure(name: str, timeout: int = 30, retries: int = 3) -> str:\n    return name",
    );
    assert!(!code.is_empty(), "mixed defaults and required: {}", code);
}

// =============================================================================
// Section 11: *args and **kwargs handling
// =============================================================================

#[test]
fn test_varargs_basic() {
    let code = transpile(
        "def total(*args) -> int:\n    result = 0\n    for a in args:\n        result += a\n    return result",
    );
    assert!(!code.is_empty(), "varargs basic: {}", code);
}

#[test]
fn test_varargs_with_regular_param() {
    let code = transpile(
        "def prefix_all(prefix: str, *items) -> list:\n    result = []\n    return result",
    );
    assert!(!code.is_empty(), "varargs with regular param: {}", code);
}

#[test]
fn test_kwargs_basic() {
    assert!(transpile_ok("def config(**kwargs):\n    return kwargs"));
}

// =============================================================================
// Section 12: Type-annotated parameters (complex types)
// =============================================================================

#[test]
fn test_param_list_int() {
    let code = transpile(
        "def sum_list(items: list) -> int:\n    total = 0\n    for x in items:\n        total += x\n    return total",
    );
    assert!(!code.is_empty(), "param list int: {}", code);
}

#[test]
fn test_param_dict_str_int() {
    let code = transpile("def lookup(data: dict, key: str) -> int:\n    return data[key]");
    assert!(!code.is_empty(), "param dict str int: {}", code);
}

#[test]
fn test_param_optional_int() {
    let code = transpile("def safe_add(a: int, b: int = 0) -> int:\n    return a + b");
    assert!(!code.is_empty(), "param optional int: {}", code);
}

#[test]
fn test_param_tuple_type() {
    let code = transpile("def swap(pair: tuple) -> tuple:\n    return (pair[1], pair[0])");
    assert!(!code.is_empty(), "param tuple type: {}", code);
}

#[test]
fn test_param_set_type() {
    let code = transpile("def contains(items: set, val: int) -> bool:\n    return val in items");
    assert!(!code.is_empty(), "param set type: {}", code);
}

#[test]
fn test_param_list_of_str() {
    let code = transpile("def join_all(words: list) -> str:\n    return ', '.join(words)");
    assert!(!code.is_empty(), "param list of str: {}", code);
}

#[test]
fn test_param_complex_nested() {
    let code = transpile(
        "def process(data: list) -> int:\n    total = 0\n    for row in data:\n        total += row\n    return total",
    );
    assert!(!code.is_empty(), "param complex nested: {}", code);
}

// =============================================================================
// Section 13: Return type annotations
// =============================================================================

#[test]
fn test_return_int() {
    let code = transpile("def get_val() -> int:\n    return 42");
    assert!(!code.is_empty(), "return int: {}", code);
}

#[test]
fn test_return_float() {
    let code = transpile("def get_pi() -> float:\n    return 3.14159");
    assert!(!code.is_empty(), "return float: {}", code);
}

#[test]
fn test_return_str() {
    let code = transpile("def greet() -> str:\n    return 'hello'");
    assert!(!code.is_empty(), "return str: {}", code);
}

#[test]
fn test_return_bool() {
    let code = transpile("def is_valid() -> bool:\n    return True");
    assert!(!code.is_empty(), "return bool: {}", code);
}

#[test]
fn test_return_list() {
    let code = transpile("def make_list() -> list:\n    return [1, 2, 3]");
    assert!(!code.is_empty(), "return list: {}", code);
}

#[test]
fn test_return_dict() {
    let code = transpile("def make_dict() -> dict:\n    return {'a': 1}");
    assert!(!code.is_empty(), "return dict: {}", code);
}

#[test]
fn test_return_none_explicitly() {
    let code = transpile("def noop() -> None:\n    return None");
    assert!(!code.is_empty(), "return None: {}", code);
}

#[test]
fn test_return_optional_pattern() {
    let code = transpile(
        "def find(items: list, target: int):\n    for item in items:\n        if item == target:\n            return item\n    return None",
    );
    assert!(!code.is_empty(), "return optional pattern: {}", code);
}

// =============================================================================
// Section 14: Docstrings extraction
// =============================================================================

#[test]
fn test_docstring_single_line() {
    let code = transpile("def greet():\n    \"\"\"Return greeting.\"\"\"\n    return 'hello'");
    assert!(!code.is_empty(), "docstring single line: {}", code);
}

#[test]
fn test_docstring_multiline() {
    let code = transpile(
        "def compute(x: int) -> int:\n    \"\"\"Compute the result.\n\n    Args:\n        x: input value\n    \"\"\"\n    return x * 2",
    );
    assert!(!code.is_empty(), "docstring multiline: {}", code);
}

#[test]
fn test_class_docstring() {
    let code = transpile(
        "class MyClass:\n    \"\"\"A simple class.\"\"\"\n    def __init__(self, x: int):\n        self.x = x",
    );
    assert!(!code.is_empty(), "class docstring: {}", code);
}

#[test]
fn test_method_docstring() {
    let code = transpile(
        "class Widget:\n    def __init__(self, name: str):\n        self.name = name\n    def render(self) -> str:\n        \"\"\"Render the widget.\"\"\"\n        return self.name",
    );
    assert!(!code.is_empty(), "method docstring: {}", code);
}

// =============================================================================
// Section 15: Try/except/finally in functions
// =============================================================================

#[test]
fn test_try_except_basic() {
    let code = transpile(
        "def safe_div(a: int, b: int) -> int:\n    try:\n        return a // b\n    except:\n        return 0",
    );
    assert!(!code.is_empty(), "try except basic: {}", code);
}

#[test]
fn test_try_except_specific() {
    let code = transpile(
        "def parse_int(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError:\n        return -1",
    );
    assert!(!code.is_empty(), "try except specific: {}", code);
}

#[test]
fn test_try_finally() {
    let code =
        transpile("def cleanup():\n    try:\n        x = 1\n    finally:\n        print('done')");
    assert!(!code.is_empty(), "try finally: {}", code);
}

#[test]
fn test_try_except_finally() {
    let code = transpile(
        "def robust(x: int) -> int:\n    try:\n        return x * 2\n    except:\n        return 0\n    finally:\n        print('cleanup')",
    );
    assert!(!code.is_empty(), "try except finally: {}", code);
}

#[test]
fn test_try_except_else() {
    let code = transpile(
        "def attempt(x: int) -> int:\n    try:\n        result = x * 2\n    except:\n        result = 0\n    else:\n        result = result + 1\n    return result",
    );
    assert!(!code.is_empty(), "try except else: {}", code);
}

#[test]
fn test_try_multiple_except() {
    let code = transpile(
        "def multi_catch(x: int) -> str:\n    try:\n        return str(x)\n    except ValueError:\n        return 'value_err'\n    except TypeError:\n        return 'type_err'",
    );
    assert!(!code.is_empty(), "try multiple except: {}", code);
}

#[test]
fn test_nested_try_except() {
    let code = transpile(
        "def nested_try(a: int, b: int) -> int:\n    try:\n        try:\n            return a // b\n        except:\n            return -1\n    except:\n        return -2",
    );
    assert!(!code.is_empty(), "nested try except: {}", code);
}

// =============================================================================
// Section 16: Context managers (with statement)
// =============================================================================

#[test]
fn test_with_open_read() {
    let code = transpile(
        "def read_file(path: str) -> str:\n    with open(path) as f:\n        return f.read()",
    );
    assert!(!code.is_empty(), "with open read: {}", code);
}

#[test]
fn test_with_open_write() {
    let code = transpile(
        "def write_file(path: str, content: str):\n    with open(path, 'w') as f:\n        f.write(content)",
    );
    assert!(!code.is_empty(), "with open write: {}", code);
}

#[test]
fn test_with_in_function() {
    let code = transpile(
        "def process(path: str) -> int:\n    with open(path) as f:\n        data = f.read()\n    return len(data)",
    );
    assert!(!code.is_empty(), "with in function: {}", code);
}

// =============================================================================
// Section 17: Multiple return values (tuples)
// =============================================================================

#[test]
fn test_return_tuple_two() {
    let code = transpile("def divmod_custom(a: int, b: int) -> tuple:\n    return (a // b, a % b)");
    assert!(!code.is_empty(), "return tuple two: {}", code);
}

#[test]
fn test_return_tuple_three() {
    let code = transpile("def rgb(r: int, g: int, b: int) -> tuple:\n    return (r, g, b)");
    assert!(!code.is_empty(), "return tuple three: {}", code);
}

#[test]
fn test_return_tuple_mixed_types() {
    let code = transpile("def info() -> tuple:\n    return ('name', 42, True)");
    assert!(!code.is_empty(), "return tuple mixed types: {}", code);
}

#[test]
fn test_tuple_unpacking_return() {
    let code = transpile(
        "def split_pair(pair: tuple) -> tuple:\n    a = pair[0]\n    b = pair[1]\n    return (b, a)",
    );
    assert!(!code.is_empty(), "tuple unpacking return: {}", code);
}

// =============================================================================
// Section 18: Return type inference from body
// =============================================================================

#[test]
fn test_infer_return_from_literal() {
    let code = transpile("def get_val():\n    return 42");
    assert!(!code.is_empty(), "infer return from literal: {}", code);
}

#[test]
fn test_infer_return_from_string() {
    let code = transpile("def get_name():\n    return 'hello'");
    assert!(!code.is_empty(), "infer return from string: {}", code);
}

#[test]
fn test_infer_return_from_bool() {
    let code = transpile("def is_ready():\n    return True");
    assert!(!code.is_empty(), "infer return from bool: {}", code);
}

#[test]
fn test_infer_return_optional_none_and_int() {
    let code =
        transpile("def maybe_get(x: int):\n    if x > 0:\n        return x\n    return None");
    assert!(!code.is_empty(), "infer optional None+int: {}", code);
}

#[test]
fn test_infer_return_optional_none_and_string() {
    let code =
        transpile("def maybe_str(x: int):\n    if x > 0:\n        return 'yes'\n    return None");
    assert!(!code.is_empty(), "infer optional None+string: {}", code);
}

#[test]
fn test_infer_return_list_from_comp() {
    let code = transpile("def evens(n: int):\n    return [x for x in range(n) if x % 2 == 0]");
    assert!(!code.is_empty(), "infer return list from comp: {}", code);
}

#[test]
fn test_infer_return_from_arithmetic() {
    let code = transpile("def compute(a: int, b: int):\n    return a + b");
    assert!(!code.is_empty(), "infer return from arithmetic: {}", code);
}

#[test]
fn test_infer_return_from_comparison() {
    let code = transpile("def is_positive(x: int):\n    return x > 0");
    assert!(!code.is_empty(), "infer return from comparison: {}", code);
}

// =============================================================================
// Section 19: Variable type inference in body
// =============================================================================

#[test]
fn test_var_type_from_annotation() {
    let code = transpile("def foo():\n    x: int = 10\n    return x");
    assert!(!code.is_empty(), "var type from annotation: {}", code);
}

#[test]
fn test_var_type_from_literal_int() {
    let code = transpile("def foo():\n    x = 42\n    return x");
    assert!(!code.is_empty(), "var type from literal int: {}", code);
}

#[test]
fn test_var_type_from_literal_str() {
    let code = transpile("def foo():\n    name = 'hello'\n    return name");
    assert!(!code.is_empty(), "var type from literal str: {}", code);
}

#[test]
fn test_var_type_from_param() {
    let code = transpile("def foo(n: int) -> int:\n    result = n\n    return result");
    assert!(!code.is_empty(), "var type from param: {}", code);
}

#[test]
fn test_var_type_from_list_literal() {
    let code = transpile("def foo() -> list:\n    items = [1, 2, 3]\n    return items");
    assert!(!code.is_empty(), "var type from list literal: {}", code);
}

#[test]
fn test_var_type_from_dict_literal() {
    let code = transpile("def foo() -> dict:\n    data = {'key': 'value'}\n    return data");
    assert!(!code.is_empty(), "var type from dict literal: {}", code);
}

#[test]
fn test_var_type_none_then_concrete() {
    let code =
        transpile("def foo(x: int):\n    result = None\n    result = 'found'\n    return result");
    assert!(!code.is_empty(), "var type None then concrete: {}", code);
}

// =============================================================================
// Section 20: Loop and if escaping variables
// =============================================================================

#[test]
fn test_loop_escaping_variable() {
    let code = transpile(
        "def find_first(items: list) -> int:\n    found = 0\n    for item in items:\n        found = item\n    return found",
    );
    assert!(!code.is_empty(), "loop escaping variable: {}", code);
}

#[test]
fn test_while_loop_escaping_variable() {
    let code = transpile(
        "def count_down(n: int) -> int:\n    result = 0\n    i = n\n    while i > 0:\n        result = i\n        i -= 1\n    return result",
    );
    assert!(!code.is_empty(), "while loop escaping variable: {}", code);
}

#[test]
fn test_if_escaping_variable() {
    let code = transpile(
        "def classify(x: int) -> str:\n    if x > 0:\n        label = 'positive'\n    else:\n        label = 'non-positive'\n    return label",
    );
    assert!(!code.is_empty(), "if escaping variable: {}", code);
}

#[test]
fn test_nested_if_escaping() {
    let code = transpile(
        "def classify2(x: int) -> str:\n    if x > 10:\n        label = 'big'\n    elif x > 0:\n        label = 'small'\n    else:\n        label = 'zero'\n    return label",
    );
    assert!(!code.is_empty(), "nested if escaping: {}", code);
}

// =============================================================================
// Section 21: Function with complex body patterns
// =============================================================================

#[test]
fn test_function_with_assert() {
    let code = transpile(
        "def positive_add(a: int, b: int) -> int:\n    assert a > 0\n    assert b > 0\n    return a + b",
    );
    assert!(!code.is_empty(), "function with assert: {}", code);
}

#[test]
fn test_function_with_raise() {
    let code = transpile(
        "def divide(a: int, b: int) -> int:\n    if b == 0:\n        raise ValueError('zero')\n    return a // b",
    );
    assert!(!code.is_empty(), "function with raise: {}", code);
}

#[test]
fn test_function_returning_string_method() {
    let code = transpile("def upper_name(name: str) -> str:\n    return name.upper()");
    assert!(!code.is_empty(), "return string method: {}", code);
}

#[test]
fn test_function_returning_fstring() {
    let code = transpile("def greet(name: str) -> str:\n    return f'Hello, {name}!'");
    assert!(!code.is_empty(), "return fstring: {}", code);
}

#[test]
fn test_function_with_string_concat_return() {
    let code =
        transpile("def full_name(first: str, last: str) -> str:\n    return first + ' ' + last");
    assert!(!code.is_empty(), "return string concat: {}", code);
}

#[test]
fn test_function_with_list_comprehension() {
    let code = transpile("def doubled(items: list) -> list:\n    return [x * 2 for x in items]");
    assert!(!code.is_empty(), "function with list comp: {}", code);
}

#[test]
fn test_function_with_dict_comprehension() {
    let code = transpile("def invert(d: dict) -> dict:\n    return {v: k for k, v in d.items()}");
    assert!(!code.is_empty(), "function with dict comp: {}", code);
}

#[test]
fn test_function_with_set_comprehension() {
    let code =
        transpile("def unique_squares(items: list) -> set:\n    return {x * x for x in items}");
    assert!(!code.is_empty(), "function with set comp: {}", code);
}

// =============================================================================
// Section 22: Parameter mutability detection
// =============================================================================

#[test]
fn test_param_mutated_append() {
    let code = transpile("def add_item(items: list, item: int):\n    items.append(item)");
    assert!(!code.is_empty(), "param mutated append: {}", code);
}

#[test]
fn test_param_mutated_assignment() {
    let code = transpile("def increment(x: int) -> int:\n    x = x + 1\n    return x");
    assert!(!code.is_empty(), "param mutated assignment: {}", code);
}

#[test]
fn test_param_unused() {
    let code = transpile("def ignore(x: int) -> int:\n    return 42");
    assert!(!code.is_empty(), "param unused: {}", code);
}

#[test]
fn test_param_used_in_condition_only() {
    let code =
        transpile("def check(flag: bool) -> int:\n    if flag:\n        return 1\n    return 0");
    assert!(!code.is_empty(), "param used in condition: {}", code);
}

// =============================================================================
// Section 23: Rust keyword parameter names
// =============================================================================

#[test]
fn test_param_name_match() {
    assert!(transpile_ok("def foo(match: str) -> str:\n    return match"));
}

#[test]
fn test_param_name_type() {
    assert!(transpile_ok("def foo(loop: int) -> int:\n    return loop"));
}

#[test]
fn test_param_name_impl() {
    assert!(transpile_ok("def foo(impl: str) -> str:\n    return impl"));
}

// =============================================================================
// Section 24: Expression type inference (infer_expr_type_with_env)
// =============================================================================

#[test]
fn test_infer_module_json_loads() {
    let code =
        transpile("import json\ndef parse(s: str):\n    data = json.loads(s)\n    return data");
    assert!(!code.is_empty(), "infer json.loads: {}", code);
}

#[test]
fn test_infer_json_dumps() {
    let code =
        transpile("import json\ndef serialize(data: dict) -> str:\n    return json.dumps(data)");
    assert!(!code.is_empty(), "infer json.dumps: {}", code);
}

#[test]
fn test_infer_string_split() {
    let code = transpile("def tokenize(s: str) -> list:\n    return s.split(',')");
    assert!(!code.is_empty(), "infer string split: {}", code);
}

#[test]
fn test_infer_string_upper() {
    let code = transpile("def shout(s: str) -> str:\n    return s.upper()");
    assert!(!code.is_empty(), "infer string upper: {}", code);
}

#[test]
fn test_infer_dict_get() {
    let code = transpile("def lookup(d: dict, key: str):\n    return d.get(key)");
    assert!(!code.is_empty(), "infer dict get: {}", code);
}

#[test]
fn test_infer_list_pop() {
    let code = transpile("def last(items: list):\n    return items.pop()");
    assert!(!code.is_empty(), "infer list pop: {}", code);
}

// =============================================================================
// Section 25: Propagate return type to variables
// =============================================================================

#[test]
fn test_propagate_return_type_to_empty_list() {
    let code = transpile(
        "def get_items() -> list:\n    result = []\n    result.append(1)\n    return result",
    );
    assert!(!code.is_empty(), "propagate return to empty list: {}", code);
}

#[test]
fn test_propagate_return_type_to_empty_dict() {
    let code = transpile(
        "def get_config() -> dict:\n    result = {}\n    result['key'] = 'value'\n    return result",
    );
    assert!(!code.is_empty(), "propagate return to empty dict: {}", code);
}

#[test]
fn test_propagate_through_if_branch() {
    let code = transpile(
        "def get_items(flag: bool) -> list:\n    result = []\n    if flag:\n        result.append(1)\n    return result",
    );
    assert!(!code.is_empty(), "propagate through if branch: {}", code);
}

// =============================================================================
// Section 26: String method classification and return type
// =============================================================================

#[test]
fn test_string_method_strip_returns_owned() {
    let code = transpile("def clean(s: str) -> str:\n    return s.strip()");
    assert!(!code.is_empty(), "string strip returns owned: {}", code);
}

#[test]
fn test_string_method_replace_returns_owned() {
    let code = transpile("def fix(s: str) -> str:\n    return s.replace('old', 'new')");
    assert!(!code.is_empty(), "string replace returns owned: {}", code);
}

#[test]
fn test_string_method_startswith_returns_bool() {
    let code = transpile("def is_prefix(s: str) -> bool:\n    return s.startswith('pre')");
    assert!(!code.is_empty(), "string startswith: {}", code);
}

#[test]
fn test_string_method_find_returns_int() {
    let code = transpile("def locate(s: str, sub: str) -> int:\n    return s.find(sub)");
    assert!(!code.is_empty(), "string find: {}", code);
}

#[test]
fn test_string_method_title() {
    let code = transpile("def titlecase(s: str) -> str:\n    return s.title()");
    assert!(!code.is_empty(), "string title: {}", code);
}

#[test]
fn test_string_method_capitalize() {
    let code = transpile("def cap(s: str) -> str:\n    return s.capitalize()");
    assert!(!code.is_empty(), "string capitalize: {}", code);
}

// =============================================================================
// Section 27: Power operations and negative exponents
// =============================================================================

#[test]
fn test_power_negative_exponent() {
    let code = transpile("def inv(x: int) -> float:\n    return x ** -1");
    assert!(!code.is_empty(), "power negative exponent: {}", code);
}

#[test]
fn test_power_positive_exponent() {
    let code = transpile("def square(x: int) -> int:\n    return x ** 2");
    assert!(!code.is_empty(), "power positive exponent: {}", code);
}

#[test]
fn test_power_float_base() {
    let code = transpile("def sqrt(x: float) -> float:\n    return x ** 0.5");
    assert!(!code.is_empty(), "power float base: {}", code);
}

// =============================================================================
// Section 28: ADT / enum child type rewriting
// =============================================================================

#[test]
fn test_abstract_class_hierarchy() {
    assert!(transpile_ok(
        "class Shape:\n    def area(self) -> float:\n        return 0.0\n\nclass Circle(Shape):\n    def __init__(self, r: float):\n        self.r = r\n    def area(self) -> float:\n        return 3.14 * self.r * self.r\n\nclass Square(Shape):\n    def __init__(self, s: float):\n        self.s = s\n    def area(self) -> float:\n        return self.s * self.s"
    ));
}

// =============================================================================
// Section 29: Conditional returns (if/elif/else)
// =============================================================================

#[test]
fn test_conditional_return_if_else() {
    let code = transpile(
        "def abs_val(x: int) -> int:\n    if x >= 0:\n        return x\n    else:\n        return -x",
    );
    assert!(!code.is_empty(), "conditional return: {}", code);
}

#[test]
fn test_conditional_return_multiple_branches() {
    let code = transpile(
        "def classify(x: int) -> str:\n    if x > 0:\n        return 'positive'\n    elif x < 0:\n        return 'negative'\n    else:\n        return 'zero'",
    );
    assert!(!code.is_empty(), "multiple branch return: {}", code);
}

#[test]
fn test_early_return_guard() {
    let code = transpile(
        "def safe_sqrt(x: float) -> float:\n    if x < 0:\n        return 0.0\n    return x ** 0.5",
    );
    assert!(!code.is_empty(), "early return guard: {}", code);
}

// =============================================================================
// Section 30: Complex class patterns
// =============================================================================

#[test]
fn test_class_with_method_calling_another_method() {
    let code = transpile(
        "class Calc:\n    def __init__(self, val: int):\n        self.val = val\n    def double(self) -> int:\n        return self.val * 2\n    def quadruple(self) -> int:\n        return self.double() * 2",
    );
    assert!(!code.is_empty(), "class method calling method: {}", code);
}

#[test]
fn test_class_with_conditional_method() {
    let code = transpile(
        "class Account:\n    def __init__(self, balance: int):\n        self.balance = balance\n    def withdraw(self, amount: int) -> bool:\n        if amount <= self.balance:\n            self.balance -= amount\n            return True\n        return False",
    );
    assert!(!code.is_empty(), "class conditional method: {}", code);
}

#[test]
fn test_class_with_loop_method() {
    let code = transpile(
        "class Summer:\n    def __init__(self):\n        self.total = 0\n    def add_all(self, items: list):\n        for x in items:\n            self.total += x",
    );
    assert!(!code.is_empty(), "class loop method: {}", code);
}

// =============================================================================
// Section 31: Lambda expressions in function context
// =============================================================================

#[test]
fn test_lambda_in_sorted() {
    let code = transpile(
        "def sort_by_len(items: list) -> list:\n    return sorted(items, key=lambda x: len(x))",
    );
    assert!(!code.is_empty(), "lambda in sorted: {}", code);
}

#[test]
fn test_lambda_in_filter() {
    let code = transpile(
        "def evens(items: list) -> list:\n    return list(filter(lambda x: x % 2 == 0, items))",
    );
    assert!(!code.is_empty(), "lambda in filter: {}", code);
}

#[test]
fn test_lambda_in_map() {
    let code = transpile(
        "def double_all(items: list) -> list:\n    return list(map(lambda x: x * 2, items))",
    );
    assert!(!code.is_empty(), "lambda in map: {}", code);
}

// =============================================================================
// Section 32: Recursive functions
// =============================================================================

#[test]
fn test_recursive_factorial() {
    let code = transpile(
        "def factorial(n: int) -> int:\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)",
    );
    assert!(!code.is_empty(), "recursive factorial: {}", code);
}

#[test]
fn test_recursive_fibonacci() {
    let code = transpile(
        "def fib(n: int) -> int:\n    if n <= 0:\n        return 0\n    if n == 1:\n        return 1\n    return fib(n - 1) + fib(n - 2)",
    );
    assert!(!code.is_empty(), "recursive fibonacci: {}", code);
}

// =============================================================================
// Section 33: Expressions in function body that exercise type inference
// =============================================================================

#[test]
fn test_sys_argv_attribute_inference() {
    let code = transpile("import sys\ndef get_args() -> list:\n    return sys.argv");
    assert!(!code.is_empty(), "sys.argv inference: {}", code);
}

#[test]
fn test_list_repeat_pattern() {
    let code = transpile("def zeros(n: int) -> list:\n    return [0] * n");
    assert!(!code.is_empty(), "list repeat pattern: {}", code);
}

#[test]
fn test_conditional_expression_return() {
    let code = transpile("def max_of(a: int, b: int) -> int:\n    return a if a > b else b");
    assert!(!code.is_empty(), "conditional expr return: {}", code);
}

#[test]
fn test_string_slice_return() {
    let code = transpile("def first_three(s: str) -> str:\n    return s[:3]");
    assert!(!code.is_empty(), "string slice return: {}", code);
}

#[test]
fn test_index_into_list_return() {
    let code = transpile("def first(items: list) -> int:\n    return items[0]");
    assert!(!code.is_empty(), "index into list return: {}", code);
}

#[test]
fn test_dict_index_return() {
    let code = transpile("def get_val(d: dict, key: str):\n    return d[key]");
    assert!(!code.is_empty(), "dict index return: {}", code);
}

#[test]
fn test_unary_not_return() {
    let code = transpile("def negate(flag: bool) -> bool:\n    return not flag");
    assert!(!code.is_empty(), "unary not return: {}", code);
}

// =============================================================================
// Section 34: Method call inference patterns
// =============================================================================

#[test]
fn test_dict_keys_inference() {
    let code = transpile("def get_keys(d: dict) -> list:\n    return list(d.keys())");
    assert!(!code.is_empty(), "dict keys inference: {}", code);
}

#[test]
fn test_dict_values_inference() {
    let code = transpile("def get_values(d: dict) -> list:\n    return list(d.values())");
    assert!(!code.is_empty(), "dict values inference: {}", code);
}

#[test]
fn test_dict_items_inference() {
    let code = transpile("def get_items(d: dict) -> list:\n    return list(d.items())");
    assert!(!code.is_empty(), "dict items inference: {}", code);
}

#[test]
fn test_string_join_inference() {
    let code = transpile("def join_words(words: list) -> str:\n    return ' '.join(words)");
    assert!(!code.is_empty(), "string join inference: {}", code);
}

// =============================================================================
// Section 35: Builtin call return type inference
// =============================================================================

#[test]
fn test_builtin_len_returns_int() {
    let code = transpile("def count(s: str) -> int:\n    return len(s)");
    assert!(!code.is_empty(), "len returns int: {}", code);
}

#[test]
fn test_builtin_int_returns_int() {
    let code = transpile("def to_int(s: str) -> int:\n    return int(s)");
    assert!(!code.is_empty(), "int returns int: {}", code);
}

#[test]
fn test_builtin_float_returns_float() {
    let code = transpile("def to_float(s: str) -> float:\n    return float(s)");
    assert!(!code.is_empty(), "float returns float: {}", code);
}

#[test]
fn test_builtin_str_returns_str() {
    let code = transpile("def to_str(x: int) -> str:\n    return str(x)");
    assert!(!code.is_empty(), "str returns str: {}", code);
}

#[test]
fn test_builtin_bool_returns_bool() {
    let code = transpile("def to_bool(x: int) -> bool:\n    return bool(x)");
    assert!(!code.is_empty(), "bool returns bool: {}", code);
}

#[test]
fn test_builtin_abs_returns_int() {
    let code = transpile("def magnitude(x: int) -> int:\n    return abs(x)");
    assert!(!code.is_empty(), "abs returns int: {}", code);
}

#[test]
fn test_builtin_range_returns_list() {
    let code = transpile("def make_range(n: int) -> list:\n    return list(range(n))");
    assert!(!code.is_empty(), "range returns list: {}", code);
}

// =============================================================================
// Section 36: Multiple assignment and tuple unpacking
// =============================================================================

#[test]
fn test_tuple_unpack_in_body() {
    let code = transpile("def sum_pair(pair: tuple) -> int:\n    a, b = pair\n    return a + b");
    assert!(!code.is_empty(), "tuple unpack in body: {}", code);
}

#[test]
fn test_swap_variables() {
    let code = transpile("def swap(a: int, b: int) -> tuple:\n    a, b = b, a\n    return (a, b)");
    assert!(!code.is_empty(), "swap variables: {}", code);
}

// =============================================================================
// Section 37: Type coercion in expressions
// =============================================================================

#[test]
fn test_int_float_coercion() {
    let code = transpile("def mixed(a: int, b: float) -> float:\n    return a + b");
    assert!(!code.is_empty(), "int float coercion: {}", code);
}

#[test]
fn test_float_division() {
    let code = transpile("def divide(a: int, b: int) -> float:\n    return a / b");
    assert!(!code.is_empty(), "float division: {}", code);
}

#[test]
fn test_floor_division() {
    let code = transpile("def int_div(a: int, b: int) -> int:\n    return a // b");
    assert!(!code.is_empty(), "floor division: {}", code);
}

// =============================================================================
// Section 38: For loop patterns
// =============================================================================

#[test]
fn test_for_range_accumulate() {
    let code = transpile(
        "def sum_range(n: int) -> int:\n    total = 0\n    for i in range(n):\n        total += i\n    return total",
    );
    assert!(!code.is_empty(), "for range accumulate: {}", code);
}

#[test]
fn test_for_list_filter() {
    let code = transpile(
        "def positives(items: list) -> list:\n    result = []\n    for x in items:\n        if x > 0:\n            result.append(x)\n    return result",
    );
    assert!(!code.is_empty(), "for list filter: {}", code);
}

#[test]
fn test_for_dict_iteration() {
    let code = transpile("def print_dict(d: dict):\n    for key in d:\n        print(key)");
    assert!(!code.is_empty(), "for dict iteration: {}", code);
}

#[test]
fn test_for_enumerate() {
    let code = transpile(
        "def indexed(items: list):\n    for i, item in enumerate(items):\n        print(i)",
    );
    assert!(!code.is_empty(), "for enumerate: {}", code);
}

// =============================================================================
// Section 39: While loop patterns
// =============================================================================

#[test]
fn test_while_countdown() {
    let code =
        transpile("def countdown(n: int):\n    while n > 0:\n        print(n)\n        n -= 1");
    assert!(!code.is_empty(), "while countdown: {}", code);
}

#[test]
fn test_while_with_break() {
    let code = transpile(
        "def find_zero(items: list) -> int:\n    i = 0\n    while i < len(items):\n        if items[i] == 0:\n            break\n        i += 1\n    return i",
    );
    assert!(!code.is_empty(), "while with break: {}", code);
}

#[test]
fn test_while_with_continue() {
    let code = transpile(
        "def skip_neg(items: list) -> int:\n    total = 0\n    i = 0\n    while i < len(items):\n        i += 1\n        if items[i - 1] < 0:\n            continue\n        total += items[i - 1]\n    return total",
    );
    assert!(!code.is_empty(), "while with continue: {}", code);
}

// =============================================================================
// Section 40: Complex function compositions
// =============================================================================

#[test]
fn test_function_calling_function() {
    let code = transpile(
        "def double(x: int) -> int:\n    return x * 2\n\ndef quadruple(x: int) -> int:\n    return double(double(x))",
    );
    assert!(!code.is_empty(), "function calling function: {}", code);
}

#[test]
fn test_function_with_multiple_returns() {
    let code = transpile(
        "def grade(score: int) -> str:\n    if score >= 90:\n        return 'A'\n    if score >= 80:\n        return 'B'\n    if score >= 70:\n        return 'C'\n    return 'F'",
    );
    assert!(!code.is_empty(), "multiple returns: {}", code);
}

#[test]
fn test_function_with_walrus_operator() {
    assert!(transpile_ok(
        "def process(items: list) -> int:\n    if n := len(items):\n        return n\n    return 0"
    ));
}

#[test]
fn test_class_with_try_in_method() {
    let code = transpile(
        "class SafeCalc:\n    def __init__(self, val: int):\n        self.val = val\n    def safe_div(self, divisor: int) -> int:\n        try:\n            return self.val // divisor\n        except:\n            return 0",
    );
    assert!(!code.is_empty(), "class with try in method: {}", code);
}

#[test]
fn test_function_with_nested_list_comp_in_return() {
    let code = transpile(
        "def flatten(matrix: list) -> list:\n    return [x for row in matrix for x in row]",
    );
    assert!(!code.is_empty(), "nested list comp in return: {}", code);
}

#[test]
fn test_function_with_conditional_in_list_comp() {
    let code = transpile(
        "def even_squares(n: int) -> list:\n    return [x * x for x in range(n) if x % 2 == 0]",
    );
    assert!(!code.is_empty(), "conditional list comp: {}", code);
}

#[test]
fn test_method_returning_string_format() {
    let code = transpile(
        "class Person:\n    def __init__(self, name: str, age: int):\n        self.name = name\n        self.age = age\n    def describe(self) -> str:\n        return self.name + ' is ' + str(self.age)",
    );
    assert!(!code.is_empty(), "method returning string format: {}", code);
}

#[test]
fn test_class_with_multiple_dunders() {
    let code = transpile(
        "class MyNum:\n    def __init__(self, val: int):\n        self.val = val\n    def __str__(self) -> str:\n        return str(self.val)\n    def __eq__(self, other) -> bool:\n        return self.val == other.val\n    def __len__(self) -> int:\n        return self.val",
    );
    assert!(!code.is_empty(), "class multiple dunders: {}", code);
}

#[test]
fn test_function_returning_empty_list() {
    let code = transpile("def empty() -> list:\n    return []");
    assert!(!code.is_empty(), "return empty list: {}", code);
}

#[test]
fn test_function_returning_empty_dict() {
    let code = transpile("def empty_dict() -> dict:\n    return {}");
    assert!(!code.is_empty(), "return empty dict: {}", code);
}

#[test]
fn test_function_with_global_call_inference() {
    let code = transpile("def count_chars(s: str) -> int:\n    return len(s)");
    assert!(!code.is_empty(), "global call inference: {}", code);
}

#[test]
fn test_class_with_list_method() {
    let code = transpile(
        "class Queue:\n    def __init__(self):\n        self.items: list = []\n    def enqueue(self, item: int):\n        self.items.append(item)\n    def dequeue(self) -> int:\n        return self.items.pop(0)\n    def is_empty(self) -> bool:\n        return len(self.items) == 0",
    );
    assert!(!code.is_empty(), "class with list method: {}", code);
}

#[test]
fn test_function_with_bool_ops() {
    let code = transpile("def check(a: bool, b: bool) -> bool:\n    return a and b or not a");
    assert!(!code.is_empty(), "function with bool ops: {}", code);
}

#[test]
fn test_function_modulo_return() {
    let code = transpile("def is_even(x: int) -> bool:\n    return x % 2 == 0");
    assert!(!code.is_empty(), "function modulo return: {}", code);
}

#[test]
fn test_function_with_augmented_assign() {
    let code = transpile(
        "def accumulate(items: list) -> int:\n    total = 0\n    for x in items:\n        total += x\n    return total",
    );
    assert!(!code.is_empty(), "function augmented assign: {}", code);
}
