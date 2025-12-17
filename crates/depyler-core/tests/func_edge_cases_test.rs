//! Edge case function tests for increasing code coverage
//! Targets less common function patterns in func_gen.rs

use depyler_core::DepylerPipeline;

fn transpiles(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// =============================================================================
// Function Definition Tests
// =============================================================================

#[test]
fn test_func_no_params_no_return() {
    assert!(transpiles("def f():\n    pass"));
}

#[test]
fn test_func_one_param() {
    assert!(transpiles("def f(x):\n    return x"));
}

#[test]
fn test_func_multiple_params() {
    assert!(transpiles("def f(a, b, c):\n    return a + b + c"));
}

#[test]
fn test_func_typed_params() {
    assert!(transpiles("def f(x: int, y: float) -> float:\n    return x + y"));
}

#[test]
fn test_func_default_param() {
    assert!(transpiles("def f(x, y=10):\n    return x + y"));
}

#[test]
fn test_func_multiple_defaults() {
    assert!(transpiles("def f(a, b=1, c=2):\n    return a + b + c"));
}

#[test]
fn test_func_varargs() {
    assert!(transpiles("def f(*args):\n    return sum(args)"));
}

#[test]
fn test_func_kwargs() {
    assert!(transpiles("def f(**kwargs):\n    return kwargs"));
}

#[test]
fn test_func_varargs_and_kwargs() {
    assert!(transpiles("def f(*args, **kwargs):\n    return (args, kwargs)"));
}

#[test]
fn test_func_keyword_only() {
    assert!(transpiles("def f(a, *, b):\n    return a + b"));
}

#[test]
fn test_func_positional_only() {
    assert!(transpiles("def f(a, /, b):\n    return a + b"));
}

// =============================================================================
// Return Type Annotation Tests
// =============================================================================

#[test]
fn test_func_return_int() {
    assert!(transpiles("def f() -> int:\n    return 42"));
}

#[test]
fn test_func_return_str() {
    assert!(transpiles("def f() -> str:\n    return 'hello'"));
}

#[test]
fn test_func_return_float() {
    assert!(transpiles("def f() -> float:\n    return 3.14"));
}

#[test]
fn test_func_return_bool() {
    assert!(transpiles("def f() -> bool:\n    return True"));
}

#[test]
fn test_func_return_none() {
    assert!(transpiles("def f() -> None:\n    pass"));
}

#[test]
fn test_func_return_list() {
    assert!(transpiles("def f() -> list[int]:\n    return [1, 2, 3]"));
}

#[test]
fn test_func_return_dict() {
    assert!(transpiles("def f() -> dict[str, int]:\n    return {'a': 1}"));
}

#[test]
fn test_func_return_tuple() {
    assert!(transpiles("def f() -> tuple[int, str]:\n    return (1, 'a')"));
}

#[test]
fn test_func_return_set() {
    assert!(transpiles("def f() -> set[int]:\n    return {1, 2, 3}"));
}

#[test]
fn test_func_return_optional() {
    assert!(transpiles("from typing import Optional\ndef f() -> Optional[int]:\n    return None"));
}

#[test]
fn test_func_return_union() {
    assert!(transpiles("from typing import Union\ndef f() -> Union[int, str]:\n    return 'hello'"));
}

// =============================================================================
// Docstring Tests
// =============================================================================

#[test]
fn test_func_with_docstring() {
    assert!(transpiles("def f():\n    \"\"\"This is a docstring.\"\"\"\n    pass"));
}

#[test]
fn test_func_with_multiline_docstring() {
    assert!(transpiles("def f():\n    \"\"\"\n    Multi-line\n    docstring.\n    \"\"\"\n    pass"));
}

// =============================================================================
// Decorator Tests
// =============================================================================

#[test]
fn test_func_staticmethod() {
    assert!(transpiles("class C:\n    @staticmethod\n    def f():\n        return 1"));
}

#[test]
fn test_func_classmethod() {
    assert!(transpiles("class C:\n    @classmethod\n    def f(cls):\n        return cls"));
}

#[test]
fn test_func_property() {
    assert!(transpiles("class C:\n    @property\n    def value(self):\n        return self._value"));
}

#[test]
fn test_func_property_setter() {
    assert!(transpiles("class C:\n    @property\n    def value(self):\n        return self._value\n    @value.setter\n    def value(self, v):\n        self._value = v"));
}

#[test]
fn test_func_custom_decorator() {
    assert!(transpiles("def deco(f):\n    return f\n\n@deco\ndef func():\n    pass"));
}

#[test]
fn test_func_decorator_with_args() {
    assert!(transpiles("def deco(arg):\n    def wrapper(f):\n        return f\n    return wrapper\n\n@deco('test')\ndef func():\n    pass"));
}

#[test]
fn test_func_multiple_decorators() {
    assert!(transpiles("def deco1(f):\n    return f\ndef deco2(f):\n    return f\n\n@deco1\n@deco2\ndef func():\n    pass"));
}

// =============================================================================
// Async Function Tests
// =============================================================================

#[test]
fn test_async_func_simple() {
    assert!(transpiles("async def f():\n    return 1"));
}

#[test]
fn test_async_func_await() {
    assert!(transpiles("async def f():\n    result = await other()\n    return result"));
}

#[test]
fn test_async_func_typed() {
    assert!(transpiles("async def f() -> int:\n    return 1"));
}

#[test]
fn test_async_func_multiple_awaits() {
    assert!(transpiles("async def f():\n    a = await get_a()\n    b = await get_b()\n    return a + b"));
}

// =============================================================================
// Generator Function Tests
// =============================================================================

#[test]
fn test_generator_simple() {
    assert!(transpiles("def gen():\n    yield 1"));
}

#[test]
fn test_generator_loop() {
    assert!(transpiles("def gen(n):\n    for i in range(n):\n        yield i"));
}

#[test]
fn test_generator_conditional() {
    assert!(transpiles("def gen(n):\n    for i in range(n):\n        if i % 2 == 0:\n            yield i"));
}

#[test]
fn test_generator_with_return() {
    assert!(transpiles("def gen():\n    yield 1\n    yield 2\n    return"));
}

// =============================================================================
// Recursive Function Tests
// =============================================================================

#[test]
fn test_recursive_simple() {
    assert!(transpiles("def factorial(n):\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)"));
}

#[test]
fn test_recursive_tail() {
    assert!(transpiles("def fac_tail(n, acc=1):\n    if n <= 1:\n        return acc\n    return fac_tail(n - 1, n * acc)"));
}

#[test]
fn test_mutual_recursion() {
    assert!(transpiles("def even(n):\n    if n == 0:\n        return True\n    return odd(n - 1)\n\ndef odd(n):\n    if n == 0:\n        return False\n    return even(n - 1)"));
}

// =============================================================================
// Lambda in Function Tests
// =============================================================================

#[test]
fn test_lambda_as_return() {
    assert!(transpiles("def f():\n    return lambda x: x * 2"));
}

#[test]
fn test_lambda_in_map() {
    assert!(transpiles("def f(lst):\n    return list(map(lambda x: x * 2, lst))"));
}

#[test]
fn test_lambda_in_filter() {
    assert!(transpiles("def f(lst):\n    return list(filter(lambda x: x > 0, lst))"));
}

#[test]
fn test_lambda_in_sorted() {
    assert!(transpiles("def f(lst):\n    return sorted(lst, key=lambda x: x[1])"));
}

// =============================================================================
// Nested Function Tests
// =============================================================================

#[test]
fn test_nested_func_simple() {
    assert!(transpiles("def outer():\n    def inner():\n        return 1\n    return inner()"));
}

#[test]
fn test_nested_func_closure() {
    assert!(transpiles("def outer(x):\n    def inner():\n        return x * 2\n    return inner()"));
}

#[test]
fn test_nested_func_multiple_levels() {
    assert!(transpiles("def outer():\n    def middle():\n        def inner():\n            return 1\n        return inner()\n    return middle()"));
}

#[test]
fn test_nested_func_with_args() {
    assert!(transpiles("def outer(x):\n    def inner(y):\n        return x + y\n    return inner"));
}

// =============================================================================
// Error Handling Function Tests
// =============================================================================

#[test]
fn test_func_with_try() {
    assert!(transpiles("def f():\n    try:\n        return 1\n    except:\n        return 0"));
}

#[test]
fn test_func_raises() {
    assert!(transpiles("def f():\n    raise ValueError('error')"));
}

#[test]
fn test_func_catches_specific() {
    assert!(transpiles("def f():\n    try:\n        x = 1 / 0\n    except ZeroDivisionError:\n        return 0\n    return 1"));
}

// =============================================================================
// Complex Parameter Patterns
// =============================================================================

#[test]
fn test_func_all_param_types() {
    assert!(transpiles("def f(a, b=1, *args, c, d=2, **kwargs):\n    pass"));
}

#[test]
fn test_func_typed_defaults() {
    assert!(transpiles("def f(x: int = 0, y: str = '') -> tuple:\n    return (x, y)"));
}

#[test]
fn test_func_complex_default() {
    assert!(transpiles("def f(lst: list = None):\n    if lst is None:\n        lst = []\n    return lst"));
}

// =============================================================================
// Method Tests
// =============================================================================

#[test]
fn test_method_self() {
    assert!(transpiles("class C:\n    def method(self):\n        return self"));
}

#[test]
fn test_method_with_params() {
    assert!(transpiles("class C:\n    def method(self, x, y):\n        return x + y"));
}

#[test]
fn test_method_modifies_self() {
    assert!(transpiles("class C:\n    def __init__(self):\n        self.value = 0\n    def increment(self):\n        self.value += 1"));
}

#[test]
fn test_method_returns_self() {
    assert!(transpiles("class C:\n    def method(self):\n        self.value = 1\n        return self"));
}

// =============================================================================
// Dunder Method Tests
// =============================================================================

#[test]
fn test_dunder_init() {
    assert!(transpiles("class C:\n    def __init__(self, x):\n        self.x = x"));
}

#[test]
fn test_dunder_str() {
    assert!(transpiles("class C:\n    def __str__(self):\n        return 'C'"));
}

#[test]
fn test_dunder_repr() {
    assert!(transpiles("class C:\n    def __repr__(self):\n        return 'C()'"));
}

#[test]
fn test_dunder_len() {
    assert!(transpiles("class C:\n    def __len__(self):\n        return 0"));
}

#[test]
fn test_dunder_getitem() {
    assert!(transpiles("class C:\n    def __getitem__(self, key):\n        return key"));
}

#[test]
fn test_dunder_setitem() {
    assert!(transpiles("class C:\n    def __setitem__(self, key, value):\n        pass"));
}

#[test]
fn test_dunder_eq() {
    assert!(transpiles("class C:\n    def __eq__(self, other):\n        return True"));
}

#[test]
fn test_dunder_hash() {
    assert!(transpiles("class C:\n    def __hash__(self):\n        return 0"));
}

#[test]
fn test_dunder_add() {
    assert!(transpiles("class C:\n    def __add__(self, other):\n        return self"));
}

#[test]
fn test_dunder_sub() {
    assert!(transpiles("class C:\n    def __sub__(self, other):\n        return self"));
}

#[test]
fn test_dunder_mul() {
    assert!(transpiles("class C:\n    def __mul__(self, other):\n        return self"));
}

#[test]
fn test_dunder_call() {
    assert!(transpiles("class C:\n    def __call__(self, x):\n        return x"));
}

#[test]
fn test_dunder_iter() {
    assert!(transpiles("class C:\n    def __iter__(self):\n        return iter([])"));
}

#[test]
fn test_dunder_next() {
    assert!(transpiles("class C:\n    def __next__(self):\n        raise StopIteration"));
}

#[test]
fn test_dunder_enter_exit() {
    assert!(transpiles("class C:\n    def __enter__(self):\n        return self\n    def __exit__(self, exc_type, exc_val, exc_tb):\n        pass"));
}

// =============================================================================
// Annotation Tests
// =============================================================================

#[test]
fn test_annotation_list_generic() {
    assert!(transpiles("def f(lst: list[int]) -> list[int]:\n    return lst"));
}

#[test]
fn test_annotation_dict_generic() {
    assert!(transpiles("def f(d: dict[str, int]) -> dict[str, int]:\n    return d"));
}

#[test]
fn test_annotation_tuple_generic() {
    assert!(transpiles("def f(t: tuple[int, str, bool]) -> tuple[int, str, bool]:\n    return t"));
}

#[test]
fn test_annotation_optional() {
    assert!(transpiles("from typing import Optional\ndef f(x: Optional[int]) -> Optional[int]:\n    return x"));
}

#[test]
fn test_annotation_callable() {
    assert!(transpiles("from typing import Callable\ndef f(func: Callable[[int], int]) -> int:\n    return func(1)"));
}

#[test]
fn test_annotation_typevar() {
    assert!(transpiles("from typing import TypeVar\nT = TypeVar('T')\ndef f(x: T) -> T:\n    return x"));
}

// =============================================================================
// Edge Cases
// =============================================================================

#[test]
fn test_empty_function() {
    assert!(transpiles("def f():\n    pass"));
}

#[test]
fn test_function_single_stmt() {
    assert!(transpiles("def f():\n    return 1"));
}

#[test]
fn test_function_many_stmts() {
    assert!(transpiles("def f():\n    a = 1\n    b = 2\n    c = 3\n    d = 4\n    e = 5\n    return a + b + c + d + e"));
}

#[test]
fn test_function_early_return() {
    assert!(transpiles("def f(x):\n    if x < 0:\n        return None\n    return x"));
}

#[test]
fn test_function_multiple_returns() {
    assert!(transpiles("def f(x):\n    if x < 0:\n        return -1\n    elif x > 0:\n        return 1\n    else:\n        return 0"));
}
