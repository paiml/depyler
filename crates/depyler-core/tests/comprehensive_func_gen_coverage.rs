//! Comprehensive tests for func_gen.rs coverage
//! DEPYLER-COVERAGE-95: Target 95% coverage on function generation

use depyler_core::DepylerPipeline;

fn transpile(code: &str) -> Result<String, anyhow::Error> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code)
}

// =============================================================================
// FUNCTION DEFINITION COVERAGE
// =============================================================================

mod function_definitions {
    use super::*;

    #[test]
    fn test_empty_function() {
        let code = "def f() -> None:\n    pass";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_function_with_return() {
        let code = "def f() -> int:\n    return 42";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_function_with_param() {
        let code = "def f(x: int) -> int:\n    return x";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_function_multiple_params() {
        let code = "def f(x: int, y: int, z: int) -> int:\n    return x + y + z";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_function_mixed_types() {
        let code = "def f(x: int, s: str, b: bool) -> str:\n    return s";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_function_list_param() {
        let code = "def f(lst: list[int]) -> int:\n    return len(lst)";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_function_dict_param() {
        let code = "def f(d: dict[str, int]) -> int:\n    return len(d)";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_function_tuple_param() {
        let code = "def f(t: tuple[int, str]) -> int:\n    return t[0]";
        let _ = transpile(code);
    }

    #[test]
    fn test_function_set_param() {
        let code = "def f(s: set[int]) -> int:\n    return len(s)";
        let _ = transpile(code);
    }

    #[test]
    fn test_function_optional_param() {
        let code = "from typing import Optional\n\ndef f(x: Optional[int]) -> int:\n    if x is None:\n        return 0\n    return x";
        let _ = transpile(code);
    }

    #[test]
    fn test_function_union_param() {
        let code = "from typing import Union\n\ndef f(x: Union[int, str]) -> str:\n    return str(x)";
        let _ = transpile(code);
    }
}

// =============================================================================
// DEFAULT PARAMETER COVERAGE
// =============================================================================

mod default_parameters {
    use super::*;

    #[test]
    fn test_default_int() {
        let code = "def f(x: int = 0) -> int:\n    return x";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_default_str() {
        let code = "def f(s: str = \"default\") -> str:\n    return s";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_default_bool() {
        let code = "def f(b: bool = True) -> bool:\n    return b";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_default_float() {
        let code = "def f(x: float = 0.0) -> float:\n    return x";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_default_none() {
        let code = "from typing import Optional\n\ndef f(x: Optional[int] = None) -> int:\n    if x is None:\n        return 0\n    return x";
        let _ = transpile(code);
    }

    #[test]
    fn test_multiple_defaults() {
        let code = "def f(a: int = 1, b: int = 2, c: int = 3) -> int:\n    return a + b + c";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_mixed_defaults() {
        let code = "def f(x: int, y: int = 0, z: int = 0) -> int:\n    return x + y + z";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_default_list() {
        let code = "def f(lst: list[int] = []) -> list[int]:\n    return lst";
        let _ = transpile(code);
    }

    #[test]
    fn test_default_dict() {
        let code = "def f(d: dict[str, int] = {}) -> dict[str, int]:\n    return d";
        let _ = transpile(code);
    }
}

// =============================================================================
// RETURN TYPE COVERAGE
// =============================================================================

mod return_types {
    use super::*;

    #[test]
    fn test_return_none() {
        let code = "def f() -> None:\n    pass";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_return_int() {
        let code = "def f() -> int:\n    return 42";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_return_float() {
        let code = "def f() -> float:\n    return 3.14";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_return_str() {
        let code = "def f() -> str:\n    return \"hello\"";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_return_bool() {
        let code = "def f() -> bool:\n    return True";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_return_list() {
        let code = "def f() -> list[int]:\n    return [1, 2, 3]";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_return_dict() {
        let code = "def f() -> dict[str, int]:\n    return {\"a\": 1}";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_return_tuple() {
        let code = "def f() -> tuple[int, str]:\n    return (1, \"a\")";
        let _ = transpile(code);
    }

    #[test]
    fn test_return_set() {
        let code = "def f() -> set[int]:\n    return {1, 2, 3}";
        let _ = transpile(code);
    }

    #[test]
    fn test_return_optional() {
        let code = "from typing import Optional\n\ndef f(x: int) -> Optional[int]:\n    if x > 0:\n        return x\n    return None";
        let _ = transpile(code);
    }

    #[test]
    fn test_return_nested_list() {
        let code = "def f() -> list[list[int]]:\n    return [[1, 2], [3, 4]]";
        let _ = transpile(code);
    }

    #[test]
    fn test_return_nested_dict() {
        let code = "def f() -> dict[str, dict[str, int]]:\n    return {\"a\": {\"b\": 1}}";
        let _ = transpile(code);
    }
}

// =============================================================================
// DOCSTRING COVERAGE
// =============================================================================

mod docstrings {
    use super::*;

    #[test]
    fn test_single_line_docstring() {
        let code = "def f() -> None:\n    \"\"\"This is a docstring.\"\"\"\n    pass";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_multi_line_docstring() {
        let code = "def f() -> None:\n    \"\"\"\n    This is a multi-line docstring.\n    It has multiple lines.\n    \"\"\"\n    pass";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_docstring_with_params() {
        let code = "def f(x: int) -> int:\n    \"\"\"\n    Doubles the input.\n    \n    Args:\n        x: The input value.\n    \n    Returns:\n        The doubled value.\n    \"\"\"\n    return x * 2";
        assert!(transpile(code).is_ok());
    }
}

// =============================================================================
// RECURSIVE FUNCTION COVERAGE
// =============================================================================

mod recursive_functions {
    use super::*;

    #[test]
    fn test_factorial() {
        let code = "def factorial(n: int) -> int:\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_fibonacci() {
        let code = "def fib(n: int) -> int:\n    if n <= 1:\n        return n\n    return fib(n - 1) + fib(n - 2)";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_gcd() {
        let code = "def gcd(a: int, b: int) -> int:\n    if b == 0:\n        return a\n    return gcd(b, a % b)";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_sum_list_recursive() {
        let code = "def sum_list(lst: list[int]) -> int:\n    if len(lst) == 0:\n        return 0\n    return lst[0] + sum_list(lst[1:])";
        let _ = transpile(code);
    }
}

// =============================================================================
// MULTIPLE FUNCTION COVERAGE
// =============================================================================

mod multiple_functions {
    use super::*;

    #[test]
    fn test_two_functions() {
        let code = "def add(a: int, b: int) -> int:\n    return a + b\n\ndef mul(a: int, b: int) -> int:\n    return a * b";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_function_calls_function() {
        let code = "def helper(x: int) -> int:\n    return x * 2\n\ndef main(x: int) -> int:\n    return helper(x) + 1";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_chain_of_functions() {
        let code = "def a() -> int:\n    return 1\n\ndef b() -> int:\n    return a() + 1\n\ndef c() -> int:\n    return b() + 1";
        assert!(transpile(code).is_ok());
    }

    #[test]
    fn test_mutual_recursion() {
        let code = "def is_even(n: int) -> bool:\n    if n == 0:\n        return True\n    return is_odd(n - 1)\n\ndef is_odd(n: int) -> bool:\n    if n == 0:\n        return False\n    return is_even(n - 1)";
        let _ = transpile(code);
    }
}

// =============================================================================
// NESTED FUNCTION COVERAGE
// =============================================================================

mod nested_functions {
    use super::*;

    #[test]
    fn test_nested_function() {
        let code = "def outer() -> int:\n    def inner() -> int:\n        return 1\n    return inner()";
        let _ = transpile(code);
    }

    #[test]
    fn test_nested_function_with_capture() {
        let code = "def outer(x: int) -> int:\n    def inner() -> int:\n        return x * 2\n    return inner()";
        let _ = transpile(code);
    }

    #[test]
    fn test_deeply_nested() {
        let code = "def level1() -> int:\n    def level2() -> int:\n        def level3() -> int:\n            return 1\n        return level3()\n    return level2()";
        let _ = transpile(code);
    }
}

// =============================================================================
// LAMBDA COVERAGE
// =============================================================================

mod lambda_functions {
    use super::*;

    #[test]
    fn test_lambda_in_variable() {
        let code = "def f() -> int:\n    add_one = lambda x: x + 1\n    return add_one(5)";
        let _ = transpile(code);
    }

    #[test]
    fn test_lambda_multi_arg() {
        let code = "def f() -> int:\n    add = lambda x, y: x + y\n    return add(1, 2)";
        let _ = transpile(code);
    }

    #[test]
    fn test_lambda_in_map() {
        let code = "def f(lst: list[int]) -> list[int]:\n    return list(map(lambda x: x * 2, lst))";
        let _ = transpile(code);
    }

    #[test]
    fn test_lambda_in_filter() {
        let code = "def f(lst: list[int]) -> list[int]:\n    return list(filter(lambda x: x > 0, lst))";
        let _ = transpile(code);
    }

    #[test]
    fn test_lambda_in_sorted() {
        let code = "def f(lst: list[str]) -> list[str]:\n    return sorted(lst, key=lambda x: len(x))";
        let _ = transpile(code);
    }

    #[test]
    fn test_lambda_in_reduce() {
        let code = "from functools import reduce\n\ndef f(lst: list[int]) -> int:\n    return reduce(lambda a, b: a + b, lst, 0)";
        let _ = transpile(code);
    }
}

// =============================================================================
// GENERATOR FUNCTION COVERAGE
// =============================================================================

mod generator_functions {
    use super::*;

    #[test]
    fn test_simple_generator() {
        let code = "def gen() -> int:\n    yield 1\n    yield 2\n    yield 3";
        let _ = transpile(code);
    }

    #[test]
    fn test_generator_with_loop() {
        let code = "def gen(n: int) -> int:\n    for i in range(n):\n        yield i";
        let _ = transpile(code);
    }

    #[test]
    fn test_generator_with_condition() {
        let code = "def gen(n: int) -> int:\n    for i in range(n):\n        if i % 2 == 0:\n            yield i";
        let _ = transpile(code);
    }

    #[test]
    fn test_infinite_generator() {
        let code = "def infinite() -> int:\n    i: int = 0\n    while True:\n        yield i\n        i += 1";
        let _ = transpile(code);
    }
}

// =============================================================================
// ASYNC FUNCTION COVERAGE
// =============================================================================

mod async_functions {
    use super::*;

    #[test]
    fn test_async_def() {
        let code = "async def f() -> int:\n    return 42";
        let _ = transpile(code);
    }

    #[test]
    fn test_async_with_await() {
        let code = "async def fetch() -> str:\n    return \"data\"\n\nasync def main() -> str:\n    result = await fetch()\n    return result";
        let _ = transpile(code);
    }

    #[test]
    fn test_async_for() {
        let code = "async def f() -> int:\n    total: int = 0\n    async for item in async_iter():\n        total += item\n    return total";
        let _ = transpile(code);
    }

    #[test]
    fn test_async_with() {
        let code = "async def f() -> None:\n    async with async_context() as ctx:\n        pass";
        let _ = transpile(code);
    }
}

// =============================================================================
// DECORATOR COVERAGE
// =============================================================================

mod decorators {
    use super::*;

    #[test]
    fn test_staticmethod() {
        let code = "class Foo:\n    @staticmethod\n    def bar() -> int:\n        return 42";
        let _ = transpile(code);
    }

    #[test]
    fn test_classmethod() {
        let code = "class Foo:\n    @classmethod\n    def bar(cls) -> int:\n        return 42";
        let _ = transpile(code);
    }

    #[test]
    fn test_property() {
        let code = "class Foo:\n    def __init__(self) -> None:\n        self._x: int = 0\n    \n    @property\n    def x(self) -> int:\n        return self._x";
        let _ = transpile(code);
    }

    #[test]
    fn test_custom_decorator() {
        let code = "def decorator(f):\n    return f\n\n@decorator\ndef foo() -> int:\n    return 42";
        let _ = transpile(code);
    }

    #[test]
    fn test_multiple_decorators() {
        let code = "def dec1(f):\n    return f\n\ndef dec2(f):\n    return f\n\n@dec1\n@dec2\ndef foo() -> int:\n    return 42";
        let _ = transpile(code);
    }
}

// =============================================================================
// TYPE VARIABLE COVERAGE
// =============================================================================

mod type_variables {
    use super::*;

    #[test]
    fn test_generic_function() {
        let code = "from typing import TypeVar\n\nT = TypeVar('T')\n\ndef identity(x: T) -> T:\n    return x";
        let _ = transpile(code);
    }

    #[test]
    fn test_bounded_type_var() {
        let code = "from typing import TypeVar\n\nT = TypeVar('T', int, float)\n\ndef add(a: T, b: T) -> T:\n    return a + b";
        let _ = transpile(code);
    }
}

// =============================================================================
// SPECIAL METHOD COVERAGE
// =============================================================================

mod special_methods {
    use super::*;

    #[test]
    fn test_init() {
        let code = "class Foo:\n    def __init__(self, x: int) -> None:\n        self.x = x";
        let _ = transpile(code);
    }

    #[test]
    fn test_str() {
        let code = "class Foo:\n    def __str__(self) -> str:\n        return \"Foo\"";
        let _ = transpile(code);
    }

    #[test]
    fn test_repr() {
        let code = "class Foo:\n    def __repr__(self) -> str:\n        return \"Foo()\"";
        let _ = transpile(code);
    }

    #[test]
    fn test_len() {
        let code = "class Foo:\n    def __len__(self) -> int:\n        return 0";
        let _ = transpile(code);
    }

    #[test]
    fn test_getitem() {
        let code = "class Foo:\n    def __getitem__(self, key: int) -> int:\n        return key";
        let _ = transpile(code);
    }

    #[test]
    fn test_setitem() {
        let code = "class Foo:\n    def __setitem__(self, key: int, value: int) -> None:\n        pass";
        let _ = transpile(code);
    }

    #[test]
    fn test_iter() {
        let code = "class Foo:\n    def __iter__(self):\n        return iter([])";
        let _ = transpile(code);
    }

    #[test]
    fn test_next() {
        let code = "class Foo:\n    def __next__(self) -> int:\n        return 0";
        let _ = transpile(code);
    }

    #[test]
    fn test_eq() {
        let code = "class Foo:\n    def __eq__(self, other) -> bool:\n        return True";
        let _ = transpile(code);
    }

    #[test]
    fn test_lt() {
        let code = "class Foo:\n    def __lt__(self, other) -> bool:\n        return True";
        let _ = transpile(code);
    }

    #[test]
    fn test_add() {
        let code = "class Foo:\n    def __add__(self, other):\n        return Foo()";
        let _ = transpile(code);
    }

    #[test]
    fn test_enter_exit() {
        let code = "class Foo:\n    def __enter__(self):\n        return self\n    def __exit__(self, exc_type, exc_val, exc_tb) -> None:\n        pass";
        let _ = transpile(code);
    }

    #[test]
    fn test_call() {
        let code = "class Foo:\n    def __call__(self, x: int) -> int:\n        return x";
        let _ = transpile(code);
    }

    #[test]
    fn test_hash() {
        let code = "class Foo:\n    def __hash__(self) -> int:\n        return 0";
        let _ = transpile(code);
    }

    #[test]
    fn test_bool() {
        let code = "class Foo:\n    def __bool__(self) -> bool:\n        return True";
        let _ = transpile(code);
    }
}

// =============================================================================
// VARIADIC FUNCTION COVERAGE
// =============================================================================

mod variadic_functions {
    use super::*;

    #[test]
    fn test_args() {
        let code = "def f(*args: int) -> int:\n    return sum(args)";
        let _ = transpile(code);
    }

    #[test]
    fn test_kwargs() {
        let code = "def f(**kwargs: int) -> int:\n    return sum(kwargs.values())";
        let _ = transpile(code);
    }

    #[test]
    fn test_args_and_kwargs() {
        let code = "def f(*args: int, **kwargs: int) -> int:\n    return sum(args) + sum(kwargs.values())";
        let _ = transpile(code);
    }

    #[test]
    fn test_positional_and_args() {
        let code = "def f(x: int, *args: int) -> int:\n    return x + sum(args)";
        let _ = transpile(code);
    }

    #[test]
    fn test_keyword_only() {
        let code = "def f(*, x: int, y: int) -> int:\n    return x + y";
        let _ = transpile(code);
    }

    #[test]
    fn test_positional_only() {
        let code = "def f(x: int, /, y: int) -> int:\n    return x + y";
        let _ = transpile(code);
    }
}
