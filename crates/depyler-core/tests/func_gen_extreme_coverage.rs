//! EXTREME TDD Tests for func_gen.rs
//!
//! Comprehensive coverage for function generation including:
//! - Function signatures
//! - Parameter handling
//! - Decorators
//! - Return types
//! - Edge cases

use depyler_core::DepylerPipeline;

fn transpile(code: &str) -> Result<String, String> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).map_err(|e| e.to_string())
}

fn transpile_ok(code: &str) -> bool {
    transpile(code).is_ok()
}

#[allow(dead_code)]
fn transpile_contains(code: &str, expected: &str) -> bool {
    match transpile(code) {
        Ok(result) => result.contains(expected),
        Err(_) => false,
    }
}

// ============================================================================
// FUNCTION SIGNATURES - Comprehensive Coverage
// ============================================================================

mod signatures {
    use super::*;

    #[test]
    fn test_no_params_no_return() {
        assert!(transpile_ok("def f():\n    pass"));
    }

    #[test]
    fn test_no_params_with_return() {
        assert!(transpile_ok("def f() -> int:\n    return 42"));
    }

    #[test]
    fn test_one_param_no_type() {
        assert!(transpile_ok("def f(x):\n    return x"));
    }

    #[test]
    fn test_one_param_with_type() {
        assert!(transpile_ok("def f(x: int) -> int:\n    return x"));
    }

    #[test]
    fn test_multiple_params_typed() {
        assert!(transpile_ok("def f(a: int, b: str, c: float) -> str:\n    return str(a) + b + str(c)"));
    }

    #[test]
    fn test_default_param_int() {
        assert!(transpile_ok("def f(x: int = 0) -> int:\n    return x"));
    }

    #[test]
    fn test_default_param_str() {
        assert!(transpile_ok("def f(s: str = 'default') -> str:\n    return s"));
    }

    #[test]
    fn test_default_param_none() {
        assert!(transpile_ok("def f(x = None):\n    return x"));
    }

    #[test]
    fn test_default_param_bool() {
        assert!(transpile_ok("def f(flag: bool = True) -> bool:\n    return flag"));
    }

    #[test]
    fn test_default_param_float() {
        assert!(transpile_ok("def f(x: float = 3.14) -> float:\n    return x"));
    }

    #[test]
    fn test_default_param_list() {
        assert!(transpile_ok("def f(lst: list = None) -> list:\n    if lst is None:\n        return []\n    return lst"));
    }

    #[test]
    fn test_mixed_default_nondefault() {
        assert!(transpile_ok("def f(a: int, b: int = 0, c: str = '') -> str:\n    return str(a + b) + c"));
    }

    #[test]
    fn test_many_params() {
        assert!(transpile_ok("def f(a: int, b: int, c: int, d: int, e: int, f: int) -> int:\n    return a + b + c + d + e + f"));
    }

    #[test]
    fn test_many_defaults() {
        assert!(transpile_ok("def f(a: int = 1, b: int = 2, c: int = 3, d: int = 4) -> int:\n    return a + b + c + d"));
    }
}

// ============================================================================
// PARAMETER TYPES - Comprehensive Coverage
// ============================================================================

mod param_types {
    use super::*;

    #[test]
    fn test_param_int() {
        assert!(transpile_ok("def f(x: int) -> int:\n    return x"));
    }

    #[test]
    fn test_param_float() {
        assert!(transpile_ok("def f(x: float) -> float:\n    return x"));
    }

    #[test]
    fn test_param_str() {
        assert!(transpile_ok("def f(x: str) -> str:\n    return x"));
    }

    #[test]
    fn test_param_bool() {
        assert!(transpile_ok("def f(x: bool) -> bool:\n    return x"));
    }

    #[test]
    fn test_param_list() {
        assert!(transpile_ok("def f(x: list) -> list:\n    return x"));
    }

    #[test]
    fn test_param_dict() {
        assert!(transpile_ok("def f(x: dict) -> dict:\n    return x"));
    }

    #[test]
    fn test_param_set() {
        assert!(transpile_ok("def f(x: set) -> set:\n    return x"));
    }

    #[test]
    fn test_param_tuple() {
        assert!(transpile_ok("def f(x: tuple) -> tuple:\n    return x"));
    }

    #[test]
    fn test_param_bytes() {
        assert!(transpile_ok("def f(x: bytes) -> bytes:\n    return x"));
    }

    #[test]
    fn test_param_list_int() {
        assert!(transpile_ok("def f(x: list[int]) -> list[int]:\n    return x"));
    }

    #[test]
    fn test_param_list_str() {
        assert!(transpile_ok("def f(x: list[str]) -> list[str]:\n    return x"));
    }

    #[test]
    fn test_param_dict_str_int() {
        assert!(transpile_ok("def f(x: dict[str, int]) -> dict[str, int]:\n    return x"));
    }

    #[test]
    fn test_param_optional() {
        assert!(transpile_ok("from typing import Optional\ndef f(x: Optional[int]) -> int:\n    return x if x else 0"));
    }

    #[test]
    fn test_param_union() {
        assert!(transpile_ok("from typing import Union\ndef f(x: Union[int, str]) -> str:\n    return str(x)"));
    }

    #[test]
    fn test_param_callable() {
        assert!(transpile_ok("from typing import Callable\ndef f(func: Callable[[int], int]) -> int:\n    return func(5)"));
    }

    #[test]
    fn test_param_any() {
        assert!(transpile_ok("from typing import Any\ndef f(x: Any) -> Any:\n    return x"));
    }
}

// ============================================================================
// RETURN TYPES - Comprehensive Coverage
// ============================================================================

mod return_types {
    use super::*;

    #[test]
    fn test_return_none_implicit() {
        assert!(transpile_ok("def f():\n    pass"));
    }

    #[test]
    fn test_return_none_explicit() {
        assert!(transpile_ok("def f() -> None:\n    pass"));
    }

    #[test]
    fn test_return_int() {
        assert!(transpile_ok("def f() -> int:\n    return 42"));
    }

    #[test]
    fn test_return_float() {
        assert!(transpile_ok("def f() -> float:\n    return 3.14"));
    }

    #[test]
    fn test_return_str() {
        assert!(transpile_ok("def f() -> str:\n    return 'hello'"));
    }

    #[test]
    fn test_return_bool() {
        assert!(transpile_ok("def f() -> bool:\n    return True"));
    }

    #[test]
    fn test_return_list() {
        assert!(transpile_ok("def f() -> list:\n    return [1, 2, 3]"));
    }

    #[test]
    fn test_return_dict() {
        assert!(transpile_ok("def f() -> dict:\n    return {'a': 1}"));
    }

    #[test]
    fn test_return_tuple_typed() {
        assert!(transpile_ok("def f() -> tuple[int, str]:\n    return (1, 'a')"));
    }

    #[test]
    fn test_return_optional() {
        assert!(transpile_ok("from typing import Optional\ndef f(x: int) -> Optional[int]:\n    if x > 0:\n        return x\n    return None"));
    }

    #[test]
    fn test_return_list_typed() {
        assert!(transpile_ok("def f() -> list[int]:\n    return [1, 2, 3]"));
    }

    #[test]
    fn test_return_dict_typed() {
        assert!(transpile_ok("def f() -> dict[str, int]:\n    return {'a': 1}"));
    }

    #[test]
    fn test_return_self_reference() {
        assert!(transpile_ok("class Foo:\n    def clone(self) -> 'Foo':\n        return Foo()"));
    }
}

// ============================================================================
// VARARGS AND KWARGS - Comprehensive Coverage
// ============================================================================

mod varargs_kwargs {
    use super::*;

    #[test]
    fn test_varargs_only() {
        assert!(transpile_ok("def f(*args):\n    for arg in args:\n        print(arg)"));
    }

    #[test]
    fn test_varargs_typed() {
        assert!(transpile_ok("def f(*args: int) -> int:\n    return sum(args)"));
    }

    #[test]
    fn test_kwargs_only() {
        assert!(transpile_ok("def f(**kwargs):\n    for k, v in kwargs.items():\n        print(k, v)"));
    }

    #[test]
    fn test_kwargs_typed() {
        assert!(transpile_ok("def f(**kwargs: str):\n    for k, v in kwargs.items():\n        print(k, v)"));
    }

    #[test]
    fn test_args_and_kwargs() {
        assert!(transpile_ok("def f(*args, **kwargs):\n    pass"));
    }

    #[test]
    fn test_regular_and_varargs() {
        assert!(transpile_ok("def f(first: int, *rest):\n    return first + sum(rest)"));
    }

    #[test]
    fn test_regular_and_kwargs() {
        assert!(transpile_ok("def f(first: int, **kwargs):\n    return first"));
    }

    #[test]
    fn test_all_param_types() {
        assert!(transpile_ok("def f(a: int, b: int = 0, *args, **kwargs):\n    pass"));
    }

    #[test]
    fn test_keyword_only_params() {
        assert!(transpile_ok("def f(*, x: int, y: int) -> int:\n    return x + y"));
    }

    #[test]
    fn test_keyword_only_after_varargs() {
        assert!(transpile_ok("def f(*args, sep: str = ' ') -> str:\n    return sep.join(str(a) for a in args)"));
    }

    #[test]
    fn test_positional_only_params() {
        assert!(transpile_ok("def f(x: int, y: int, /) -> int:\n    return x + y"));
    }
}

// ============================================================================
// DECORATORS - Comprehensive Coverage
// ============================================================================

mod decorators {
    use super::*;

    #[test]
    fn test_staticmethod() {
        assert!(transpile_ok("class Foo:\n    @staticmethod\n    def bar() -> int:\n        return 42"));
    }

    #[test]
    fn test_classmethod() {
        assert!(transpile_ok("class Foo:\n    @classmethod\n    def create(cls) -> 'Foo':\n        return cls()"));
    }

    #[test]
    fn test_property() {
        assert!(transpile_ok("class Foo:\n    def __init__(self):\n        self._x = 0\n    @property\n    def x(self) -> int:\n        return self._x"));
    }

    #[test]
    fn test_property_setter() {
        assert!(transpile_ok("class Foo:\n    def __init__(self):\n        self._x = 0\n    @property\n    def x(self) -> int:\n        return self._x\n    @x.setter\n    def x(self, val: int):\n        self._x = val"));
    }

    #[test]
    fn test_custom_decorator() {
        assert!(transpile_ok("def my_decorator(func):\n    return func\n@my_decorator\ndef f() -> int:\n    return 42"));
    }

    #[test]
    fn test_decorator_with_args() {
        assert!(transpile_ok("def repeat(n):\n    def decorator(func):\n        return func\n    return decorator\n@repeat(3)\ndef f():\n    pass"));
    }

    #[test]
    fn test_multiple_decorators() {
        assert!(transpile_ok("def dec1(f):\n    return f\ndef dec2(f):\n    return f\n@dec1\n@dec2\ndef f():\n    pass"));
    }

    #[test]
    fn test_dataclass_decorator() {
        assert!(transpile_ok("from dataclasses import dataclass\n@dataclass\nclass Point:\n    x: int\n    y: int"));
    }

    #[test]
    fn test_dataclass_frozen() {
        assert!(transpile_ok("from dataclasses import dataclass\n@dataclass(frozen=True)\nclass ImmutablePoint:\n    x: int\n    y: int"));
    }
}

// ============================================================================
// NESTED FUNCTIONS - Comprehensive Coverage
// ============================================================================

mod nested_functions {
    use super::*;

    #[test]
    fn test_simple_nested() {
        assert!(transpile_ok("def outer():\n    def inner():\n        return 42\n    return inner()"));
    }

    #[test]
    fn test_nested_with_params() {
        assert!(transpile_ok("def outer(x: int):\n    def inner(y: int) -> int:\n        return x + y\n    return inner(10)"));
    }

    #[test]
    fn test_closure() {
        assert!(transpile_ok("def make_adder(n: int):\n    def add(x: int) -> int:\n        return x + n\n    return add"));
    }

    #[test]
    fn test_deeply_nested() {
        assert!(transpile_ok("def level1():\n    def level2():\n        def level3():\n            return 3\n        return level3()\n    return level2()"));
    }

    #[test]
    fn test_nested_with_nonlocal() {
        assert!(transpile_ok("def counter():\n    count = 0\n    def increment():\n        nonlocal count\n        count += 1\n        return count\n    return increment"));
    }

    #[test]
    fn test_nested_lambda() {
        assert!(transpile_ok("def make_multiplier(n: int):\n    return lambda x: x * n"));
    }
}

// ============================================================================
// RECURSIVE FUNCTIONS - Comprehensive Coverage
// ============================================================================

mod recursion {
    use super::*;

    #[test]
    fn test_simple_recursion() {
        assert!(transpile_ok("def factorial(n: int) -> int:\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)"));
    }

    #[test]
    fn test_tail_recursion() {
        assert!(transpile_ok("def factorial_tail(n: int, acc: int = 1) -> int:\n    if n <= 1:\n        return acc\n    return factorial_tail(n - 1, n * acc)"));
    }

    #[test]
    fn test_fibonacci() {
        assert!(transpile_ok("def fib(n: int) -> int:\n    if n <= 1:\n        return n\n    return fib(n - 1) + fib(n - 2)"));
    }

    #[test]
    fn test_mutual_recursion() {
        assert!(transpile_ok("def is_even(n: int) -> bool:\n    if n == 0:\n        return True\n    return is_odd(n - 1)\ndef is_odd(n: int) -> bool:\n    if n == 0:\n        return False\n    return is_even(n - 1)"));
    }

    #[test]
    fn test_tree_traversal() {
        assert!(transpile_ok("def sum_tree(node):\n    if node is None:\n        return 0\n    return node['value'] + sum_tree(node.get('left')) + sum_tree(node.get('right'))"));
    }
}

// ============================================================================
// GENERATOR FUNCTIONS - Comprehensive Coverage
// ============================================================================

mod generators {
    use super::*;

    #[test]
    fn test_simple_generator() {
        assert!(transpile_ok("def gen():\n    yield 1\n    yield 2\n    yield 3"));
    }

    #[test]
    fn test_generator_with_loop() {
        assert!(transpile_ok("def count_up(n: int):\n    i = 0\n    while i < n:\n        yield i\n        i += 1"));
    }

    #[test]
    fn test_generator_with_return() {
        assert!(transpile_ok("def gen_with_return():\n    yield 1\n    yield 2\n    return 'done'"));
    }

    #[test]
    fn test_generator_expression_in_func() {
        assert!(transpile_ok("def squares(n: int):\n    return (x * x for x in range(n))"));
    }

    #[test]
    fn test_yield_from() {
        assert!(transpile_ok("def chain(*iterables):\n    for it in iterables:\n        yield from it"));
    }
}

// ============================================================================
// ASYNC FUNCTIONS - Comprehensive Coverage
// ============================================================================

mod async_functions {
    use super::*;

    #[test]
    fn test_async_def() {
        let _ = transpile("async def f():\n    pass");
    }

    #[test]
    fn test_async_with_await() {
        let _ = transpile("async def f():\n    result = await some_async_call()\n    return result");
    }

    #[test]
    fn test_async_generator() {
        let _ = transpile("async def gen():\n    for i in range(10):\n        yield i");
    }
}

// ============================================================================
// SPECIAL METHODS - Comprehensive Coverage
// ============================================================================

mod special_methods {
    use super::*;

    #[test]
    fn test_init() {
        assert!(transpile_ok("class Foo:\n    def __init__(self):\n        self.x = 0"));
    }

    #[test]
    fn test_init_with_params() {
        assert!(transpile_ok("class Foo:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y"));
    }

    #[test]
    fn test_str() {
        assert!(transpile_ok("class Foo:\n    def __str__(self) -> str:\n        return 'Foo'"));
    }

    #[test]
    fn test_repr() {
        assert!(transpile_ok("class Foo:\n    def __repr__(self) -> str:\n        return 'Foo()'"));
    }

    #[test]
    fn test_eq() {
        assert!(transpile_ok("class Foo:\n    def __init__(self, x: int):\n        self.x = x\n    def __eq__(self, other) -> bool:\n        return self.x == other.x"));
    }

    #[test]
    fn test_lt() {
        assert!(transpile_ok("class Foo:\n    def __init__(self, x: int):\n        self.x = x\n    def __lt__(self, other) -> bool:\n        return self.x < other.x"));
    }

    #[test]
    fn test_len() {
        assert!(transpile_ok("class Container:\n    def __init__(self, items: list):\n        self.items = items\n    def __len__(self) -> int:\n        return len(self.items)"));
    }

    #[test]
    fn test_getitem() {
        assert!(transpile_ok("class Container:\n    def __init__(self, items: list):\n        self.items = items\n    def __getitem__(self, idx: int):\n        return self.items[idx]"));
    }

    #[test]
    fn test_setitem() {
        assert!(transpile_ok("class Container:\n    def __init__(self, items: list):\n        self.items = items\n    def __setitem__(self, idx: int, val):\n        self.items[idx] = val"));
    }

    #[test]
    fn test_iter() {
        assert!(transpile_ok("class Container:\n    def __init__(self, items: list):\n        self.items = items\n    def __iter__(self):\n        return iter(self.items)"));
    }

    #[test]
    fn test_contains() {
        assert!(transpile_ok("class Container:\n    def __init__(self, items: list):\n        self.items = items\n    def __contains__(self, item) -> bool:\n        return item in self.items"));
    }

    #[test]
    fn test_add() {
        assert!(transpile_ok("class Number:\n    def __init__(self, x: int):\n        self.x = x\n    def __add__(self, other):\n        return Number(self.x + other.x)"));
    }

    #[test]
    fn test_hash() {
        assert!(transpile_ok("class Foo:\n    def __init__(self, x: int):\n        self.x = x\n    def __hash__(self) -> int:\n        return hash(self.x)"));
    }

    #[test]
    fn test_enter_exit() {
        assert!(transpile_ok("class Context:\n    def __enter__(self):\n        return self\n    def __exit__(self, exc_type, exc_val, exc_tb):\n        pass"));
    }

    #[test]
    fn test_call() {
        assert!(transpile_ok("class Callable:\n    def __call__(self, x: int) -> int:\n        return x * 2"));
    }
}

// ============================================================================
// EDGE CASES - Comprehensive Coverage
// ============================================================================

mod edge_cases {
    use super::*;

    #[test]
    fn test_single_line_function() {
        assert!(transpile_ok("def f(): return 42"));
    }

    #[test]
    fn test_function_with_only_docstring() {
        assert!(transpile_ok("def f():\n    '''Docstring only'''\n    pass"));
    }

    #[test]
    fn test_function_with_multiline_docstring() {
        assert!(transpile_ok("def f():\n    '''\n    Multi\n    line\n    docstring\n    '''\n    return 42"));
    }

    #[test]
    fn test_function_returning_function() {
        assert!(transpile_ok("def make_func():\n    def inner():\n        return 42\n    return inner"));
    }

    #[test]
    fn test_function_with_many_statements() {
        let mut code = "def f():\n".to_string();
        for i in 0..50 {
            code.push_str(&format!("    x{} = {}\n", i, i));
        }
        code.push_str("    return x0");
        assert!(transpile_ok(&code));
    }

    #[test]
    fn test_function_with_complex_default() {
        // Default values that are expressions
        assert!(transpile_ok("def f(x: int = 1 + 2) -> int:\n    return x"));
    }

    #[test]
    fn test_function_overload_simulation() {
        // Python's typing.overload pattern
        assert!(transpile_ok("from typing import overload\n@overload\ndef f(x: int) -> int: ...\n@overload\ndef f(x: str) -> str: ...\ndef f(x):\n    return x"));
    }

    #[test]
    fn test_lambda_in_default() {
        // Lambda as default value
        assert!(transpile_ok("def f(func = lambda x: x):\n    return func(5)"));
    }

    #[test]
    fn test_empty_function_with_type_hints() {
        assert!(transpile_ok("def f(x: int, y: str) -> None:\n    pass"));
    }

    #[test]
    fn test_function_name_with_underscore() {
        assert!(transpile_ok("def _private_func():\n    pass"));
    }

    #[test]
    fn test_function_name_with_double_underscore() {
        assert!(transpile_ok("def __dunder_func__():\n    pass"));
    }

    #[test]
    fn test_very_long_function_name() {
        let name = "a".repeat(100);
        let code = format!("def {}():\n    pass", name);
        assert!(transpile_ok(&code));
    }

    #[test]
    fn test_many_parameters() {
        let params: Vec<String> = (0..20).map(|i| format!("p{}: int", i)).collect();
        let code = format!("def f({}):\n    pass", params.join(", "));
        assert!(transpile_ok(&code));
    }

    #[test]
    fn test_function_with_complex_return() {
        assert!(transpile_ok("def f(x: int) -> dict[str, list[int]]:\n    return {'data': [x, x*2, x*3]}"));
    }
}

// ============================================================================
// FALSIFICATION TESTS - Try to BREAK the code
// ============================================================================

mod falsification {
    use super::*;

    #[test]
    fn test_missing_colon_rejected() {
        assert!(!transpile_ok("def f()\n    pass"));
    }

    #[test]
    fn test_invalid_param_syntax_rejected() {
        assert!(!transpile_ok("def f(x y):\n    pass"));
    }

    #[test]
    fn test_duplicate_param_names() {
        // Python allows this but shouldn't - our transpiler might handle it
        let _ = transpile("def f(x, x):\n    return x");
    }

    #[test]
    fn test_default_before_nondefault() {
        // This should be a syntax error in Python
        assert!(!transpile_ok("def f(x = 1, y):\n    pass"));
    }

    #[test]
    fn test_invalid_return_type() {
        // Invalid type annotation
        let _ = transpile("def f() -> invalid_type:\n    pass");
    }

    #[test]
    fn test_very_deep_nesting() {
        let mut code = "def f():\n".to_string();
        for i in 0..20 {
            code.push_str(&"    ".repeat(i + 1));
            code.push_str(&format!("def nested{}():\n", i));
        }
        code.push_str(&"    ".repeat(21));
        code.push_str("return 42\n");
        // Should handle without stack overflow
        let _ = transpile(&code);
    }

    #[test]
    fn test_circular_type_reference() {
        // Type that references itself
        let _ = transpile("def f(x: 'Node') -> 'Node':\n    return x");
    }
}
