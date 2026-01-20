//! Test edge cases in func_gen.rs for coverage

use depyler_core::DepylerPipeline;

#[allow(dead_code)]
fn transpile(code: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).unwrap_or_else(|e| format!("ERROR: {}", e))
}

fn transpile_ok(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_ok()
}

// ============ Function parameter variations ============

#[test]
fn test_func_no_params() {
    assert!(transpile_ok("def f():\n    return 42"));
}

#[test]
fn test_func_one_param() {
    assert!(transpile_ok("def f(x: int) -> int:\n    return x"));
}

#[test]
fn test_func_multiple_params() {
    assert!(transpile_ok("def f(x: int, y: int, z: int) -> int:\n    return x + y + z"));
}

#[test]
fn test_func_default_param() {
    assert!(transpile_ok("def f(x: int = 0) -> int:\n    return x"));
}

#[test]
fn test_func_multiple_defaults() {
    assert!(transpile_ok("def f(x: int = 0, y: int = 1) -> int:\n    return x + y"));
}

#[test]
fn test_func_mixed_defaults() {
    assert!(transpile_ok("def f(x: int, y: int = 0) -> int:\n    return x + y"));
}

#[test]
fn test_func_string_default() {
    assert!(transpile_ok("def f(s: str = 'default') -> str:\n    return s"));
}

#[test]
fn test_func_none_default() {
    assert!(transpile_ok("def f(x = None):\n    return x"));
}

#[test]
fn test_func_list_default() {
    assert!(transpile_ok("def f(x: list = []) -> list:\n    return x"));
}

#[test]
fn test_func_dict_default() {
    assert!(transpile_ok("def f(x: dict = {}) -> dict:\n    return x"));
}

// ============ Return type variations ============

#[test]
fn test_func_return_int() {
    assert!(transpile_ok("def f() -> int:\n    return 42"));
}

#[test]
fn test_func_return_float() {
    assert!(transpile_ok("def f() -> float:\n    return 3.14"));
}

#[test]
fn test_func_return_str() {
    assert!(transpile_ok("def f() -> str:\n    return 'hello'"));
}

#[test]
fn test_func_return_bool() {
    assert!(transpile_ok("def f() -> bool:\n    return True"));
}

#[test]
fn test_func_return_list() {
    assert!(transpile_ok("def f() -> list:\n    return [1, 2, 3]"));
}

#[test]
fn test_func_return_dict() {
    assert!(transpile_ok("def f() -> dict:\n    return {'a': 1}"));
}

#[test]
fn test_func_return_tuple() {
    assert!(transpile_ok("def f():\n    return (1, 2, 3)"));
}

#[test]
fn test_func_return_none() {
    assert!(transpile_ok("def f() -> None:\n    return None"));
}

#[test]
fn test_func_return_optional() {
    assert!(transpile_ok("from typing import Optional\ndef f() -> Optional[int]:\n    return None"));
}

// ============ Generic type parameters ============

#[test]
fn test_func_list_int_param() {
    assert!(transpile_ok("def f(x: list[int]) -> int:\n    return sum(x)"));
}

#[test]
fn test_func_dict_str_int_param() {
    assert!(transpile_ok("def f(x: dict[str, int]) -> int:\n    return x.get('key', 0)"));
}

#[test]
fn test_func_set_param() {
    assert!(transpile_ok("def f(x: set[int]) -> int:\n    return len(x)"));
}

// ============ Variadic arguments ============

#[test]
fn test_func_args() {
    assert!(transpile_ok("def f(*args):\n    return args"));
}

#[test]
fn test_func_kwargs() {
    assert!(transpile_ok("def f(**kwargs):\n    return kwargs"));
}

#[test]
fn test_func_args_and_kwargs() {
    assert!(transpile_ok("def f(*args, **kwargs):\n    return (args, kwargs)"));
}

#[test]
fn test_func_positional_and_args() {
    assert!(transpile_ok("def f(x: int, *args):\n    return (x, args)"));
}

// ============ Decorators ============

#[test]
fn test_func_staticmethod() {
    assert!(transpile_ok("class Foo:\n    @staticmethod\n    def bar() -> int:\n        return 42"));
}

#[test]
fn test_func_classmethod() {
    assert!(transpile_ok("class Foo:\n    @classmethod\n    def create(cls) -> 'Foo':\n        return cls()"));
}

#[test]
fn test_func_property() {
    assert!(transpile_ok("class Foo:\n    @property\n    def value(self) -> int:\n        return 42"));
}

// ============ Nested functions ============

#[test]
fn test_nested_function() {
    assert!(transpile_ok("def outer() -> int:\n    def inner() -> int:\n        return 42\n    return inner()"));
}

#[test]
fn test_nested_function_with_closure() {
    assert!(transpile_ok("def outer(x: int) -> int:\n    def inner() -> int:\n        return x\n    return inner()"));
}

// ============ Lambda expressions ============

#[test]
fn test_lambda_simple() {
    assert!(transpile_ok("def f():\n    fn = lambda x: x * 2"));
}

#[test]
fn test_lambda_multiple_params() {
    assert!(transpile_ok("def f():\n    fn = lambda x, y: x + y"));
}

#[test]
fn test_lambda_no_params() {
    assert!(transpile_ok("def f():\n    fn = lambda: 42"));
}

#[test]
fn test_lambda_in_map() {
    assert!(transpile_ok("def f(items: list) -> list:\n    return list(map(lambda x: x * 2, items))"));
}

#[test]
fn test_lambda_in_filter() {
    assert!(transpile_ok("def f(items: list) -> list:\n    return list(filter(lambda x: x > 0, items))"));
}

#[test]
fn test_lambda_in_sorted() {
    assert!(transpile_ok("def f(items: list) -> list:\n    return sorted(items, key=lambda x: x)"));
}

// ============ Generator functions ============

#[test]
fn test_generator_simple() {
    assert!(transpile_ok("def gen():\n    yield 1\n    yield 2"));
}

#[test]
fn test_generator_with_loop() {
    assert!(transpile_ok("def gen(n: int):\n    for i in range(n):\n        yield i"));
}

#[test]
fn test_generator_with_return() {
    assert!(transpile_ok("def gen():\n    yield 1\n    return"));
}

// ============ Method variations ============

#[test]
fn test_method_self() {
    assert!(transpile_ok("class Foo:\n    def bar(self):\n        pass"));
}

#[test]
fn test_method_with_params() {
    assert!(transpile_ok("class Foo:\n    def bar(self, x: int) -> int:\n        return x"));
}

#[test]
fn test_method_accesses_self() {
    assert!(transpile_ok("class Foo:\n    def __init__(self):\n        self.x = 0\n    def get(self) -> int:\n        return self.x"));
}

#[test]
fn test_method_modifies_self() {
    assert!(transpile_ok("class Foo:\n    def __init__(self):\n        self.x = 0\n    def set(self, val: int):\n        self.x = val"));
}

// ============ Constructor patterns ============

#[test]
fn test_init_simple() {
    assert!(transpile_ok("class Foo:\n    def __init__(self):\n        pass"));
}

#[test]
fn test_init_with_params() {
    assert!(transpile_ok("class Foo:\n    def __init__(self, x: int):\n        self.x = x"));
}

#[test]
fn test_init_multiple_fields() {
    assert!(transpile_ok("class Foo:\n    def __init__(self, x: int, y: str):\n        self.x = x\n        self.y = y"));
}

#[test]
fn test_init_default_params() {
    assert!(transpile_ok("class Foo:\n    def __init__(self, x: int = 0):\n        self.x = x"));
}

// ============ Special methods ============

#[test]
fn test_dunder_str() {
    assert!(transpile_ok("class Foo:\n    def __str__(self) -> str:\n        return 'Foo'"));
}

#[test]
fn test_dunder_repr() {
    assert!(transpile_ok("class Foo:\n    def __repr__(self) -> str:\n        return 'Foo()'"));
}

#[test]
fn test_dunder_eq() {
    assert!(transpile_ok("class Foo:\n    def __eq__(self, other) -> bool:\n        return True"));
}

#[test]
fn test_dunder_hash() {
    assert!(transpile_ok("class Foo:\n    def __hash__(self) -> int:\n        return 0"));
}

#[test]
fn test_dunder_len() {
    assert!(transpile_ok("class Foo:\n    def __len__(self) -> int:\n        return 0"));
}

#[test]
fn test_dunder_iter() {
    assert!(transpile_ok("class Foo:\n    def __iter__(self):\n        return iter([])"));
}

#[test]
fn test_dunder_getitem() {
    assert!(transpile_ok("class Foo:\n    def __getitem__(self, key: int):\n        return key"));
}

#[test]
fn test_dunder_setitem() {
    assert!(transpile_ok("class Foo:\n    def __setitem__(self, key: int, value):\n        pass"));
}

// ============ Dataclass patterns ============

#[test]
fn test_dataclass_simple() {
    assert!(transpile_ok("from dataclasses import dataclass\n\n@dataclass\nclass Point:\n    x: int\n    y: int"));
}

#[test]
fn test_dataclass_with_default() {
    assert!(transpile_ok("from dataclasses import dataclass\n\n@dataclass\nclass Point:\n    x: int = 0\n    y: int = 0"));
}

#[test]
fn test_dataclass_with_method() {
    assert!(transpile_ok("from dataclasses import dataclass\n\n@dataclass\nclass Point:\n    x: int\n    y: int\n    \n    def distance(self) -> float:\n        return (self.x**2 + self.y**2)**0.5"));
}

// ============ Type hints in parameters ============

#[test]
fn test_union_type_param() {
    assert!(transpile_ok("from typing import Union\ndef f(x: Union[int, str]) -> str:\n    return str(x)"));
}

#[test]
fn test_callable_param() {
    assert!(transpile_ok("from typing import Callable\ndef f(fn: Callable[[int], int]) -> int:\n    return fn(1)"));
}

#[test]
fn test_any_type_param() {
    assert!(transpile_ok("from typing import Any\ndef f(x: Any):\n    return x"));
}

// ============ Recursive functions ============

#[test]
fn test_recursive_function() {
    assert!(transpile_ok("def factorial(n: int) -> int:\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)"));
}

#[test]
fn test_mutually_recursive() {
    let code = "def is_even(n: int) -> bool:\n    if n == 0:\n        return True\n    return is_odd(n - 1)\n\ndef is_odd(n: int) -> bool:\n    if n == 0:\n        return False\n    return is_even(n - 1)";
    assert!(transpile_ok(code));
}

// ============ Higher-order functions ============

#[test]
fn test_function_returns_function() {
    assert!(transpile_ok("def make_adder(n: int):\n    def adder(x: int) -> int:\n        return x + n\n    return adder"));
}

#[test]
fn test_function_takes_function() {
    assert!(transpile_ok("def apply(fn, x: int) -> int:\n    return fn(x)"));
}

// ============ Error handling in functions ============

#[test]
fn test_function_raises() {
    assert!(transpile_ok("def f():\n    raise ValueError('error')"));
}

#[test]
fn test_function_try_return() {
    assert!(transpile_ok("def f() -> int:\n    try:\n        return 1\n    except:\n        return 0"));
}

// ============ Docstrings ============

#[test]
fn test_function_with_docstring() {
    assert!(transpile_ok("def f() -> int:\n    \"\"\"Returns 42.\"\"\"\n    return 42"));
}

#[test]
fn test_class_with_docstring() {
    assert!(transpile_ok("class Foo:\n    \"\"\"A foo class.\"\"\"\n    pass"));
}
