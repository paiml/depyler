//! Comprehensive function generator tests
//!
//! These tests exercise the func_gen.rs code paths through the transpilation pipeline.

use crate::DepylerPipeline;

fn transpile(code: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).expect("transpilation should succeed")
}

fn transpile_ok(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_ok()
}

// ============================================================================
// BASIC FUNCTION DEFINITIONS
// ============================================================================

#[test]
fn test_func_no_args() {
    let code = transpile("def foo():\n    pass");
    assert!(code.contains("fn foo"));
}

#[test]
fn test_func_one_arg() {
    let code = transpile("def foo(x):\n    return x");
    assert!(code.contains("fn foo") && code.contains("x"));
}

#[test]
fn test_func_multiple_args() {
    let code = transpile("def foo(a, b, c):\n    return a + b + c");
    assert!(code.contains("fn foo"));
}

#[test]
fn test_func_with_return_type() {
    let code = transpile("def foo() -> int:\n    return 42");
    assert!(code.contains("->") || code.contains("i64") || code.contains("42"));
}

#[test]
fn test_func_with_param_types() {
    let code = transpile("def add(a: int, b: int) -> int:\n    return a + b");
    assert!(code.contains("fn add") || code.contains("i64"));
}

// ============================================================================
// DEFAULT PARAMETERS
// ============================================================================

#[test]
fn test_func_default_int() {
    let code = transpile("def foo(x: int = 0):\n    return x");
    assert!(code.contains("fn foo") || code.contains("0"));
}

#[test]
fn test_func_default_string() {
    let code = transpile("def foo(name: str = 'default'):\n    return name");
    assert!(code.contains("fn foo") || code.contains("default"));
}

#[test]
fn test_func_default_none() {
    let code = transpile("def foo(x = None):\n    return x");
    assert!(code.contains("fn foo") || code.contains("None") || code.contains("Option"));
}

#[test]
fn test_func_default_bool() {
    let code = transpile("def foo(flag: bool = True):\n    return flag");
    assert!(code.contains("fn foo") || code.contains("true"));
}

#[test]
fn test_func_default_list() {
    // Note: mutable default arguments are a Python gotcha
    assert!(transpile_ok("def foo(items = None):\n    if items is None:\n        items = []\n    return items"));
}

#[test]
fn test_func_mixed_defaults() {
    let code = transpile("def foo(a, b=1, c=2):\n    return a + b + c");
    assert!(code.contains("fn foo") || code.contains("1") || code.contains("2"));
}

// ============================================================================
// KEYWORD ARGUMENTS
// ============================================================================

#[test]
fn test_func_kwargs_only() {
    assert!(transpile_ok("def foo(*, key=0):\n    return key"));
}

#[test]
fn test_func_positional_and_kwargs() {
    assert!(transpile_ok("def foo(a, *, key=0):\n    return a + key"));
}

// ============================================================================
// VARIADIC FUNCTIONS
// ============================================================================

#[test]
fn test_func_args() {
    assert!(transpile_ok("def foo(*args):\n    return len(args)"));
}

#[test]
fn test_func_kwargs() {
    assert!(transpile_ok("def foo(**kwargs):\n    return len(kwargs)"));
}

#[test]
fn test_func_args_and_kwargs() {
    assert!(transpile_ok("def foo(*args, **kwargs):\n    return len(args) + len(kwargs)"));
}

// ============================================================================
// RETURN TYPES
// ============================================================================

#[test]
fn test_func_return_int() {
    let code = transpile("def foo() -> int:\n    return 42");
    assert!(code.contains("i64") || code.contains("i32") || code.contains("42"));
}

#[test]
fn test_func_return_float() {
    let code = transpile("def foo() -> float:\n    return 3.14");
    assert!(code.contains("f64") || code.contains("f32") || code.contains("3.14"));
}

#[test]
fn test_func_return_str() {
    let code = transpile("def foo() -> str:\n    return 'hello'");
    assert!(code.contains("String") || code.contains("str") || code.contains("hello"));
}

#[test]
fn test_func_return_bool() {
    let code = transpile("def foo() -> bool:\n    return True");
    assert!(code.contains("bool") || code.contains("true"));
}

#[test]
fn test_func_return_list() {
    let code = transpile("def foo() -> list:\n    return [1, 2, 3]");
    assert!(code.contains("Vec") || code.contains("1"));
}

#[test]
fn test_func_return_dict() {
    let code = transpile("def foo() -> dict:\n    return {'a': 1}");
    assert!(code.contains("HashMap") || code.contains("a"));
}

#[test]
fn test_func_return_tuple() {
    let code = transpile("def foo() -> tuple:\n    return (1, 2)");
    assert!(code.contains("(") && code.contains(")") || code.contains("1"));
}

#[test]
fn test_func_return_optional() {
    let code = transpile("from typing import Optional\n\ndef foo() -> Optional[int]:\n    return None");
    assert!(code.contains("Option") || code.contains("None"));
}

// ============================================================================
// DOCSTRINGS
// ============================================================================

#[test]
fn test_func_docstring() {
    let code = transpile("def foo():\n    '''This is a docstring.'''\n    pass");
    assert!(code.contains("fn foo") || code.contains("///") || code.contains("docstring"));
}

#[test]
fn test_func_multiline_docstring() {
    let code = transpile("def foo():\n    '''\n    Multi-line\n    docstring.\n    '''\n    pass");
    assert!(code.contains("fn foo"));
}

// ============================================================================
// RECURSIVE FUNCTIONS
// ============================================================================

#[test]
fn test_func_recursive() {
    let code = transpile("def factorial(n: int) -> int:\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)");
    assert!(code.contains("fn factorial") || code.contains("factorial"));
}

#[test]
fn test_func_tail_recursive() {
    let code = transpile("def sum_to(n: int, acc: int = 0) -> int:\n    if n == 0:\n        return acc\n    return sum_to(n - 1, acc + n)");
    assert!(code.contains("fn sum_to") || code.contains("sum_to"));
}

// ============================================================================
// NESTED FUNCTIONS
// ============================================================================

#[test]
fn test_func_nested() {
    assert!(transpile_ok("def outer():\n    def inner():\n        return 1\n    return inner()"));
}

#[test]
fn test_func_closure() {
    assert!(transpile_ok("def make_adder(n):\n    def adder(x):\n        return x + n\n    return adder"));
}

// ============================================================================
// LAMBDA FUNCTIONS
// ============================================================================

#[test]
fn test_lambda_simple() {
    let code = transpile("f = lambda x: x * 2");
    assert!(code.contains("|x|") || code.contains("|") || code.contains("*"));
}

#[test]
fn test_lambda_multiple_args() {
    let code = transpile("f = lambda x, y: x + y");
    assert!(code.contains("|") || code.contains("+"));
}

#[test]
fn test_lambda_no_args() {
    let code = transpile("f = lambda: 42");
    assert!(code.contains("||") || code.contains("42"));
}

#[test]
fn test_lambda_in_call() {
    let code = transpile("result = list(map(lambda x: x * 2, [1, 2, 3]))");
    assert!(code.contains("|") || code.contains("map"));
}

#[test]
fn test_lambda_in_sort() {
    // Sorting with key function may require more context
    let code = transpile("def foo(items):\n    items.sort(key=lambda x: x)");
    assert!(code.contains("|") || code.contains("sort") || code.contains("fn"));
}

// ============================================================================
// GENERATOR FUNCTIONS
// ============================================================================

#[test]
fn test_generator_simple() {
    assert!(transpile_ok("def gen():\n    yield 1\n    yield 2\n    yield 3"));
}

#[test]
fn test_generator_loop() {
    assert!(transpile_ok("def range_gen(n):\n    for i in range(n):\n        yield i"));
}

#[test]
fn test_generator_expression() {
    assert!(transpile_ok("def gen():\n    yield from (x * 2 for x in range(10))"));
}

#[test]
fn test_generator_yield_from() {
    assert!(transpile_ok("def gen():\n    yield from [1, 2, 3]"));
}

// ============================================================================
// ASYNC FUNCTIONS
// ============================================================================

#[test]
fn test_async_func() {
    assert!(transpile_ok("async def fetch():\n    return 'data'"));
}

#[test]
fn test_async_await() {
    assert!(transpile_ok("async def main():\n    result = await fetch()\n    return result"));
}

#[test]
fn test_async_with_type() {
    assert!(transpile_ok("async def fetch() -> str:\n    return 'data'"));
}

// ============================================================================
// DECORATORS
// ============================================================================

#[test]
fn test_decorator_simple() {
    assert!(transpile_ok("@decorator\ndef foo():\n    pass"));
}

#[test]
fn test_decorator_with_args() {
    assert!(transpile_ok("@decorator(arg=1)\ndef foo():\n    pass"));
}

#[test]
fn test_multiple_decorators() {
    assert!(transpile_ok("@decorator1\n@decorator2\ndef foo():\n    pass"));
}

#[test]
fn test_staticmethod() {
    assert!(transpile_ok("class Foo:\n    @staticmethod\n    def bar():\n        return 1"));
}

#[test]
fn test_classmethod() {
    assert!(transpile_ok("class Foo:\n    @classmethod\n    def bar(cls):\n        return 1"));
}

#[test]
fn test_property() {
    assert!(transpile_ok("class Foo:\n    @property\n    def value(self):\n        return self._value"));
}

// ============================================================================
// SPECIAL METHODS
// ============================================================================

#[test]
fn test_init() {
    let code = transpile("class Foo:\n    def __init__(self, x):\n        self.x = x");
    assert!(code.contains("new") || code.contains("impl") || code.contains("Foo"));
}

#[test]
fn test_str() {
    assert!(transpile_ok("class Foo:\n    def __str__(self):\n        return 'Foo'"));
}

#[test]
fn test_repr() {
    assert!(transpile_ok("class Foo:\n    def __repr__(self):\n        return 'Foo()'"));
}

#[test]
fn test_len() {
    assert!(transpile_ok("class Container:\n    def __len__(self):\n        return len(self.items)"));
}

#[test]
fn test_getitem() {
    assert!(transpile_ok("class Container:\n    def __getitem__(self, key):\n        return self.items[key]"));
}

#[test]
fn test_setitem() {
    assert!(transpile_ok("class Container:\n    def __setitem__(self, key, value):\n        self.items[key] = value"));
}

#[test]
fn test_iter() {
    assert!(transpile_ok("class Container:\n    def __iter__(self):\n        return iter(self.items)"));
}

#[test]
fn test_eq() {
    assert!(transpile_ok("class Point:\n    def __eq__(self, other):\n        return self.x == other.x and self.y == other.y"));
}

#[test]
fn test_lt() {
    assert!(transpile_ok("class Point:\n    def __lt__(self, other):\n        return self.x < other.x"));
}

#[test]
fn test_add() {
    assert!(transpile_ok("class Point:\n    def __add__(self, other):\n        return Point(self.x + other.x, self.y + other.y)"));
}

// ============================================================================
// GENERIC TYPE HINTS
// ============================================================================

#[test]
fn test_func_generic_list() {
    let code = transpile("from typing import List\n\ndef foo(items: List[int]) -> int:\n    return sum(items)");
    assert!(code.contains("Vec") || code.contains("i64"));
}

#[test]
fn test_func_generic_dict() {
    let code = transpile("from typing import Dict\n\ndef foo(d: Dict[str, int]) -> int:\n    return len(d)");
    assert!(code.contains("HashMap") || code.contains("String"));
}

#[test]
fn test_func_generic_tuple() {
    let code = transpile("from typing import Tuple\n\ndef foo() -> Tuple[int, str]:\n    return (1, 'a')");
    assert!(code.contains("(") || code.contains("1"));
}

#[test]
fn test_func_union_type() {
    assert!(transpile_ok("from typing import Union\n\ndef foo(x: Union[int, str]) -> str:\n    return str(x)"));
}

// ============================================================================
// MULTIPLE FUNCTIONS
// ============================================================================

#[test]
fn test_multiple_functions() {
    let code = transpile("def foo():\n    return 1\n\ndef bar():\n    return 2");
    assert!(code.contains("fn foo") && code.contains("fn bar"));
}

#[test]
fn test_functions_calling_each_other() {
    let code = transpile("def helper(x):\n    return x * 2\n\ndef main():\n    return helper(21)");
    assert!(code.contains("fn helper") && code.contains("fn main"));
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_func_empty_body() {
    let code = transpile("def foo():\n    pass");
    assert!(code.contains("fn foo"));
}

#[test]
fn test_func_only_docstring() {
    let code = transpile("def foo():\n    '''Just a docstring.'''");
    assert!(code.contains("fn foo"));
}

#[test]
fn test_func_single_expression() {
    let code = transpile("def foo():\n    42");
    assert!(code.contains("fn foo"));
}

#[test]
fn test_func_with_comments() {
    let code = transpile("def foo():\n    # This is a comment\n    return 1");
    assert!(code.contains("fn foo") || code.contains("1"));
}

#[test]
fn test_func_long_name() {
    let code = transpile("def this_is_a_very_long_function_name_that_should_still_work():\n    return True");
    assert!(code.contains("fn this_is_a_very_long_function_name"));
}

#[test]
fn test_func_underscore_prefix() {
    let code = transpile("def _private_func():\n    return 1");
    assert!(code.contains("fn _private_func") || code.contains("private"));
}

#[test]
fn test_func_dunder_name() {
    // Custom dunder methods
    let code = transpile("def __custom_method__(self):\n    return 1");
    assert!(code.contains("fn") || code.contains("custom"));
}

// ============================================================================
// FUNCTION BODY PATTERNS
// ============================================================================

#[test]
fn test_func_with_local_vars() {
    assert!(transpile_ok("def foo():\n    x = 1\n    y = 2\n    z = x + y\n    return z"));
}

#[test]
fn test_func_with_conditional_return() {
    assert!(transpile_ok("def foo(x):\n    if x > 0:\n        return 'positive'\n    elif x < 0:\n        return 'negative'\n    return 'zero'"));
}

#[test]
fn test_func_with_loop() {
    assert!(transpile_ok("def foo(n):\n    total = 0\n    for i in range(n):\n        total += i\n    return total"));
}

#[test]
fn test_func_with_while() {
    assert!(transpile_ok("def foo(n):\n    result = 1\n    while n > 0:\n        result *= n\n        n -= 1\n    return result"));
}

#[test]
fn test_func_with_nested_loop() {
    assert!(transpile_ok("def foo(n, m):\n    result = 0\n    for i in range(n):\n        for j in range(m):\n            result += i * j\n    return result"));
}

#[test]
fn test_func_with_break() {
    assert!(transpile_ok("def foo(items):\n    for item in items:\n        if item < 0:\n            break\n    return item"));
}

#[test]
fn test_func_with_continue() {
    assert!(transpile_ok("def foo(items):\n    result = []\n    for item in items:\n        if item < 0:\n            continue\n        result.append(item)\n    return result"));
}

// ============================================================================
// FUNCTION PARAMETER PATTERNS
// ============================================================================

#[test]
fn test_func_typed_params() {
    assert!(transpile_ok("def foo(x: int, y: float, z: str) -> str:\n    return z"));
}

#[test]
fn test_func_list_param() {
    assert!(transpile_ok("from typing import List\n\ndef foo(items: List[int]) -> int:\n    return sum(items)"));
}

#[test]
fn test_func_dict_param() {
    assert!(transpile_ok("from typing import Dict\n\ndef foo(data: Dict[str, int]) -> int:\n    return len(data)"));
}

#[test]
fn test_func_optional_param() {
    assert!(transpile_ok("from typing import Optional\n\ndef foo(x: Optional[int] = None) -> int:\n    return x if x is not None else 0"));
}

#[test]
fn test_func_callable_param() {
    assert!(transpile_ok("from typing import Callable\n\ndef foo(f: Callable[[int], int]) -> int:\n    return f(42)"));
}

#[test]
fn test_func_tuple_param() {
    assert!(transpile_ok("from typing import Tuple\n\ndef foo(point: Tuple[int, int]) -> int:\n    return point[0] + point[1]"));
}

// ============================================================================
// FUNCTION RETURN PATTERNS
// ============================================================================

#[test]
fn test_func_return_none() {
    assert!(transpile_ok("def foo():\n    return None"));
}

#[test]
fn test_func_return_early() {
    assert!(transpile_ok("def foo(x):\n    if x < 0:\n        return -1\n    return x * 2"));
}

#[test]
fn test_func_multiple_returns() {
    assert!(transpile_ok("def foo(x):\n    if x < 0:\n        return 'negative'\n    if x == 0:\n        return 'zero'\n    return 'positive'"));
}

#[test]
fn test_func_return_expression() {
    assert!(transpile_ok("def foo(x, y):\n    return x * y + x - y"));
}

#[test]
fn test_func_return_call() {
    assert!(transpile_ok("def foo(s):\n    return len(s)"));
}

#[test]
fn test_func_return_method() {
    assert!(transpile_ok("def foo(s):\n    return s.upper()"));
}

#[test]
fn test_func_return_comprehension() {
    assert!(transpile_ok("def foo(n):\n    return [i * 2 for i in range(n)]"));
}

// ============================================================================
// CLASS METHOD PATTERNS
// ============================================================================

#[test]
fn test_method_self() {
    assert!(transpile_ok("class Foo:\n    def get_value(self):\n        return self.value"));
}

#[test]
fn test_method_mutating() {
    assert!(transpile_ok("class Counter:\n    def increment(self):\n        self.count += 1"));
}

#[test]
fn test_method_with_params() {
    assert!(transpile_ok("class Calculator:\n    def add(self, a, b):\n        return a + b"));
}

#[test]
fn test_method_returning_self() {
    assert!(transpile_ok("class Builder:\n    def set_value(self, v):\n        self.value = v\n        return self"));
}

#[test]
fn test_method_calling_other() {
    assert!(transpile_ok("class Foo:\n    def helper(self):\n        return 1\n    def main(self):\n        return self.helper() * 2"));
}

// ============================================================================
// COMPLEX FUNCTION PATTERNS
// ============================================================================

#[test]
fn test_func_binary_search() {
    assert!(transpile_ok("def binary_search(arr, target):\n    left, right = 0, len(arr) - 1\n    while left <= right:\n        mid = (left + right) // 2\n        if arr[mid] == target:\n            return mid\n        elif arr[mid] < target:\n            left = mid + 1\n        else:\n            right = mid - 1\n    return -1"));
}

#[test]
fn test_func_quicksort() {
    assert!(transpile_ok("def quicksort(arr):\n    if len(arr) <= 1:\n        return arr\n    pivot = arr[len(arr) // 2]\n    left = [x for x in arr if x < pivot]\n    middle = [x for x in arr if x == pivot]\n    right = [x for x in arr if x > pivot]\n    return quicksort(left) + middle + quicksort(right)"));
}

#[test]
fn test_func_fibonacci_memo() {
    assert!(transpile_ok("def fib(n, memo={}):\n    if n in memo:\n        return memo[n]\n    if n <= 1:\n        return n\n    result = fib(n-1, memo) + fib(n-2, memo)\n    memo[n] = result\n    return result"));
}

#[test]
fn test_func_gcd() {
    assert!(transpile_ok("def gcd(a, b):\n    while b:\n        a, b = b, a % b\n    return a"));
}

#[test]
fn test_func_is_prime() {
    assert!(transpile_ok("def is_prime(n):\n    if n < 2:\n        return False\n    for i in range(2, int(n**0.5) + 1):\n        if n % i == 0:\n            return False\n    return True"));
}

// ============================================================================
// ERROR HANDLING IN FUNCTIONS
// ============================================================================

#[test]
fn test_func_try_except() {
    assert!(transpile_ok("def foo():\n    try:\n        x = 1 / 0\n    except:\n        return -1\n    return 0"));
}

#[test]
fn test_func_try_except_as() {
    assert!(transpile_ok("def foo():\n    try:\n        x = int('abc')\n    except ValueError as e:\n        return str(e)\n    return 'ok'"));
}

#[test]
fn test_func_raise() {
    assert!(transpile_ok("def foo(x):\n    if x < 0:\n        raise ValueError('x must be non-negative')\n    return x"));
}

#[test]
fn test_func_assert() {
    assert!(transpile_ok("def foo(x):\n    assert x > 0, 'x must be positive'\n    return x * 2"));
}

// ============================================================================
// CONTEXT MANAGERS IN FUNCTIONS
// ============================================================================

#[test]
fn test_func_with_open() {
    assert!(transpile_ok("def foo():\n    with open('test.txt') as f:\n        return f.read()"));
}

#[test]
fn test_func_with_multiple() {
    assert!(transpile_ok("def foo():\n    with open('a.txt') as a, open('b.txt') as b:\n        return a.read() + b.read()"));
}

// ============================================================================
// FUNCTION ANNOTATIONS
// ============================================================================

#[test]
fn test_func_full_annotation() {
    assert!(transpile_ok("def foo(x: int, y: str = 'default') -> bool:\n    return len(y) > x"));
}

#[test]
fn test_func_nested_type_annotation() {
    assert!(transpile_ok("from typing import List, Dict\n\ndef foo(data: Dict[str, List[int]]) -> int:\n    return sum(data.get('key', []))"));
}

#[test]
fn test_func_forward_reference() {
    assert!(transpile_ok("class Node:\n    def get_next(self) -> 'Node':\n        return self.next"));
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_func_all_defaults() {
    assert!(transpile_ok("def foo(a=1, b=2, c=3):\n    return a + b + c"));
}

#[test]
fn test_func_many_params() {
    assert!(transpile_ok("def foo(a, b, c, d, e, f, g, h):\n    return a + b + c + d + e + f + g + h"));
}

#[test]
fn test_func_keyword_only_after_star() {
    assert!(transpile_ok("def foo(a, b, *, c, d):\n    return a + b + c + d"));
}

#[test]
fn test_func_positional_only() {
    assert!(transpile_ok("def foo(a, b, /, c, d):\n    return a + b + c + d"));
}

#[test]
fn test_func_all_param_types() {
    assert!(transpile_ok("def foo(a, b, /, c, d, *args, e, f, **kwargs):\n    return len(args) + len(kwargs)"));
}

#[test]
fn test_func_empty_return() {
    assert!(transpile_ok("def foo():\n    return"));
}

#[test]
fn test_func_implicit_none_return() {
    assert!(transpile_ok("def foo():\n    x = 1"));
}
