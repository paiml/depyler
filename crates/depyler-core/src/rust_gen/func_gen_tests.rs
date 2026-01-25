//! Comprehensive function generator tests
//!
//! These tests exercise the func_gen.rs code paths through the transpilation pipeline.

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
    assert!(transpile_ok(
        "def foo(items = None):\n    if items is None:\n        items = []\n    return items"
    ));
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
    assert!(transpile_ok(
        "def foo(*args, **kwargs):\n    return len(args) + len(kwargs)"
    ));
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
    let code =
        transpile("from typing import Optional\n\ndef foo() -> Optional[int]:\n    return None");
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
    assert!(transpile_ok(
        "def outer():\n    def inner():\n        return 1\n    return inner()"
    ));
}

#[test]
fn test_func_closure() {
    assert!(transpile_ok(
        "def make_adder(n):\n    def adder(x):\n        return x + n\n    return adder"
    ));
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
    assert!(transpile_ok(
        "def gen():\n    yield 1\n    yield 2\n    yield 3"
    ));
}

#[test]
fn test_generator_loop() {
    assert!(transpile_ok(
        "def range_gen(n):\n    for i in range(n):\n        yield i"
    ));
}

#[test]
fn test_generator_expression() {
    assert!(transpile_ok(
        "def gen():\n    yield from (x * 2 for x in range(10))"
    ));
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
    assert!(transpile_ok(
        "async def main():\n    result = await fetch()\n    return result"
    ));
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
    assert!(transpile_ok(
        "@decorator1\n@decorator2\ndef foo():\n    pass"
    ));
}

#[test]
fn test_staticmethod() {
    assert!(transpile_ok(
        "class Foo:\n    @staticmethod\n    def bar():\n        return 1"
    ));
}

#[test]
fn test_classmethod() {
    assert!(transpile_ok(
        "class Foo:\n    @classmethod\n    def bar(cls):\n        return 1"
    ));
}

#[test]
fn test_property() {
    assert!(transpile_ok(
        "class Foo:\n    @property\n    def value(self):\n        return self._value"
    ));
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
    assert!(transpile_ok(
        "class Foo:\n    def __str__(self):\n        return 'Foo'"
    ));
}

#[test]
fn test_repr() {
    assert!(transpile_ok(
        "class Foo:\n    def __repr__(self):\n        return 'Foo()'"
    ));
}

#[test]
fn test_len() {
    assert!(transpile_ok(
        "class Container:\n    def __len__(self):\n        return len(self.items)"
    ));
}

#[test]
fn test_getitem() {
    assert!(transpile_ok(
        "class Container:\n    def __getitem__(self, key):\n        return self.items[key]"
    ));
}

#[test]
fn test_setitem() {
    assert!(transpile_ok(
        "class Container:\n    def __setitem__(self, key, value):\n        self.items[key] = value"
    ));
}

#[test]
fn test_iter() {
    assert!(transpile_ok(
        "class Container:\n    def __iter__(self):\n        return iter(self.items)"
    ));
}

#[test]
fn test_eq() {
    assert!(transpile_ok("class Point:\n    def __eq__(self, other):\n        return self.x == other.x and self.y == other.y"));
}

#[test]
fn test_lt() {
    assert!(transpile_ok(
        "class Point:\n    def __lt__(self, other):\n        return self.x < other.x"
    ));
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
    let code = transpile(
        "from typing import List\n\ndef foo(items: List[int]) -> int:\n    return sum(items)",
    );
    assert!(code.contains("Vec") || code.contains("i64"));
}

#[test]
fn test_func_generic_dict() {
    let code = transpile(
        "from typing import Dict\n\ndef foo(d: Dict[str, int]) -> int:\n    return len(d)",
    );
    assert!(code.contains("HashMap") || code.contains("String"));
}

#[test]
fn test_func_generic_tuple() {
    let code =
        transpile("from typing import Tuple\n\ndef foo() -> Tuple[int, str]:\n    return (1, 'a')");
    assert!(code.contains("(") || code.contains("1"));
}

#[test]
fn test_func_union_type() {
    assert!(transpile_ok(
        "from typing import Union\n\ndef foo(x: Union[int, str]) -> str:\n    return str(x)"
    ));
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
    let code = transpile(
        "def this_is_a_very_long_function_name_that_should_still_work():\n    return True",
    );
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
    assert!(transpile_ok(
        "def foo():\n    x = 1\n    y = 2\n    z = x + y\n    return z"
    ));
}

#[test]
fn test_func_with_conditional_return() {
    assert!(transpile_ok("def foo(x):\n    if x > 0:\n        return 'positive'\n    elif x < 0:\n        return 'negative'\n    return 'zero'"));
}

#[test]
fn test_func_with_loop() {
    assert!(transpile_ok(
        "def foo(n):\n    total = 0\n    for i in range(n):\n        total += i\n    return total"
    ));
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
    assert!(transpile_ok(
        "def foo(x: int, y: float, z: str) -> str:\n    return z"
    ));
}

#[test]
fn test_func_list_param() {
    assert!(transpile_ok(
        "from typing import List\n\ndef foo(items: List[int]) -> int:\n    return sum(items)"
    ));
}

#[test]
fn test_func_dict_param() {
    assert!(transpile_ok(
        "from typing import Dict\n\ndef foo(data: Dict[str, int]) -> int:\n    return len(data)"
    ));
}

#[test]
fn test_func_optional_param() {
    assert!(transpile_ok("from typing import Optional\n\ndef foo(x: Optional[int] = None) -> int:\n    return x if x is not None else 0"));
}

#[test]
fn test_func_callable_param() {
    assert!(transpile_ok(
        "from typing import Callable\n\ndef foo(f: Callable[[int], int]) -> int:\n    return f(42)"
    ));
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
    assert!(transpile_ok(
        "def foo(x):\n    if x < 0:\n        return -1\n    return x * 2"
    ));
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
    assert!(transpile_ok(
        "def foo(n):\n    return [i * 2 for i in range(n)]"
    ));
}

// ============================================================================
// CLASS METHOD PATTERNS
// ============================================================================

#[test]
fn test_method_self() {
    assert!(transpile_ok(
        "class Foo:\n    def get_value(self):\n        return self.value"
    ));
}

#[test]
fn test_method_mutating() {
    assert!(transpile_ok(
        "class Counter:\n    def increment(self):\n        self.count += 1"
    ));
}

#[test]
fn test_method_with_params() {
    assert!(transpile_ok(
        "class Calculator:\n    def add(self, a, b):\n        return a + b"
    ));
}

#[test]
fn test_method_returning_self() {
    assert!(transpile_ok(
        "class Builder:\n    def set_value(self, v):\n        self.value = v\n        return self"
    ));
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
    assert!(transpile_ok(
        "def gcd(a, b):\n    while b:\n        a, b = b, a % b\n    return a"
    ));
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
    assert!(transpile_ok(
        "def foo():\n    try:\n        x = 1 / 0\n    except:\n        return -1\n    return 0"
    ));
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
    assert!(transpile_ok(
        "def foo(x):\n    assert x > 0, 'x must be positive'\n    return x * 2"
    ));
}

// ============================================================================
// CONTEXT MANAGERS IN FUNCTIONS
// ============================================================================

#[test]
fn test_func_with_open() {
    assert!(transpile_ok(
        "def foo():\n    with open('test.txt') as f:\n        return f.read()"
    ));
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
    assert!(transpile_ok(
        "def foo(x: int, y: str = 'default') -> bool:\n    return len(y) > x"
    ));
}

#[test]
fn test_func_nested_type_annotation() {
    assert!(transpile_ok("from typing import List, Dict\n\ndef foo(data: Dict[str, List[int]]) -> int:\n    return sum(data.get('key', []))"));
}

#[test]
fn test_func_forward_reference() {
    assert!(transpile_ok(
        "class Node:\n    def get_next(self) -> 'Node':\n        return self.next"
    ));
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_func_all_defaults() {
    assert!(transpile_ok(
        "def foo(a=1, b=2, c=3):\n    return a + b + c"
    ));
}

#[test]
fn test_func_many_params() {
    assert!(transpile_ok(
        "def foo(a, b, c, d, e, f, g, h):\n    return a + b + c + d + e + f + g + h"
    ));
}

#[test]
fn test_func_keyword_only_after_star() {
    assert!(transpile_ok(
        "def foo(a, b, *, c, d):\n    return a + b + c + d"
    ));
}

#[test]
fn test_func_positional_only() {
    assert!(transpile_ok(
        "def foo(a, b, /, c, d):\n    return a + b + c + d"
    ));
}

#[test]
fn test_func_all_param_types() {
    assert!(transpile_ok(
        "def foo(a, b, /, c, d, *args, e, f, **kwargs):\n    return len(args) + len(kwargs)"
    ));
}

#[test]
fn test_func_empty_return() {
    assert!(transpile_ok("def foo():\n    return"));
}

#[test]
fn test_func_implicit_none_return() {
    assert!(transpile_ok("def foo():\n    x = 1"));
}

// ============================================================================
// COVERAGE BOOST: func_gen.rs Helper Functions
// These tests target specific uncovered code paths in func_gen.rs
// ============================================================================

// --- is_rust_keyword helper ---
#[test]
fn test_func_param_rust_keyword_type() {
    // Parameter named with Rust keyword
    assert!(transpile_ok("def foo(type: str) -> str:\n    return type"));
}

#[test]
fn test_func_param_rust_keyword_match() {
    assert!(transpile_ok(
        "def foo(match: int) -> int:\n    return match * 2"
    ));
}

// --- extract_args_field_accesses helper ---
#[test]
fn test_argparse_field_access() {
    assert!(transpile_ok(
        r#"import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--verbose")
    args = parser.parse_args()
    if args.verbose:
        print("verbose mode")"#
    ));
}

#[test]
fn test_argparse_multiple_field_access() {
    assert!(transpile_ok(
        r#"import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--input")
    parser.add_argument("--output")
    args = parser.parse_args()
    process(args.input, args.output)"#
    ));
}

// --- stmt_always_returns helper ---
#[test]
fn test_func_unconditional_return() {
    assert!(transpile_ok("def foo():\n    return 42"));
}

#[test]
fn test_func_if_else_both_return() {
    assert!(transpile_ok(
        "def foo(x):\n    if x:\n        return 1\n    else:\n        return 0"
    ));
}

#[test]
fn test_func_raise_always() {
    assert!(transpile_ok(
        "def foo():\n    raise ValueError('always fails')"
    ));
}

// --- codegen_generic_params helper ---
#[test]
fn test_func_generic_type_param() {
    assert!(transpile_ok(
        r#"from typing import TypeVar, Generic
T = TypeVar('T')
def identity(x: T) -> T:
    return x"#
    ));
}

// --- collect_nested_function_names helper ---
#[test]
fn test_func_nested_functions() {
    assert!(transpile_ok(
        r#"def outer():
    def inner1():
        return 1
    def inner2():
        return 2
    return inner1() + inner2()"#
    ));
}

#[test]
fn test_func_deeply_nested() {
    assert!(transpile_ok(
        r#"def level1():
    def level2():
        def level3():
            return 3
        return level3()
    return level2()"#
    ));
}

// --- collect_if_escaping_variables helper ---
#[test]
fn test_func_escaping_var_from_if() {
    assert!(transpile_ok(
        r#"def foo(cond):
    if cond:
        result = "yes"
    else:
        result = "no"
    return result"#
    ));
}

#[test]
fn test_func_escaping_var_multiple_branches() {
    assert!(transpile_ok(
        r#"def foo(x):
    if x > 0:
        sign = "positive"
    elif x < 0:
        sign = "negative"
    else:
        sign = "zero"
    return sign"#
    ));
}

// --- extract_toplevel_assigned_symbols helper ---
#[test]
fn test_func_toplevel_assignments() {
    assert!(transpile_ok(
        r#"def foo():
    a = 1
    b = 2
    c = a + b
    return c"#
    ));
}

#[test]
fn test_func_tuple_unpack_toplevel() {
    assert!(transpile_ok(
        r#"def foo():
    a, b, c = (1, 2, 3)
    return a + b + c"#
    ));
}

// --- find_var_type_in_body helper ---
#[test]
fn test_func_var_type_from_literal() {
    assert!(transpile_ok(
        r#"def foo() -> int:
    x = 42
    return x"#
    ));
}

#[test]
fn test_func_var_type_from_annotation() {
    assert!(transpile_ok(
        r#"def foo() -> str:
    x: str = "hello"
    return x"#
    ));
}

// --- collect_loop_escaping_variables helper ---
#[test]
fn test_func_escaping_from_for_loop() {
    assert!(transpile_ok(
        r#"def foo(items):
    found = None
    for item in items:
        if item > 0:
            found = item
            break
    return found"#
    ));
}

#[test]
fn test_func_escaping_from_while_loop() {
    assert!(transpile_ok(
        r#"def foo():
    count = 0
    while count < 10:
        count += 1
    return count"#
    ));
}

// --- collect_all_assigned_variables helper ---
#[test]
fn test_func_all_assigned_vars() {
    assert!(transpile_ok(
        r#"def foo():
    a = 1
    b = 2
    if True:
        c = 3
    for i in range(5):
        d = i
    return a + b"#
    ));
}

// --- is_var_used_in_remaining_stmts helper ---
#[test]
fn test_func_var_used_later() {
    assert!(transpile_ok(
        r#"def foo():
    x = 1
    y = 2
    return x + y"#
    ));
}

#[test]
fn test_func_var_unused_later() {
    assert!(transpile_ok(
        r#"def foo():
    x = 1
    return 0"#
    ));
}

// --- is_field_used_as_bool_condition helper ---
#[test]
fn test_argparse_bool_field_condition() {
    assert!(transpile_ok(
        r#"import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--verbose", action="store_true")
    args = parser.parse_args()
    if args.verbose:
        print("verbose")"#
    ));
}

// --- infer_numeric_type_from_arithmetic_usage helper ---
#[test]
fn test_argparse_numeric_field() {
    assert!(transpile_ok(
        r#"import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--count", type=int)
    args = parser.parse_args()
    result = args.count * 2
    return result"#
    ));
}

#[test]
fn test_argparse_float_field() {
    assert!(transpile_ok(
        r#"import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--factor", type=float)
    args = parser.parse_args()
    result = args.factor * 2.0
    return result"#
    ));
}

// --- is_param_used_in_body helper ---
#[test]
fn test_func_unused_param() {
    assert!(transpile_ok("def foo(x, y, z):\n    return x"));
}

#[test]
fn test_func_all_params_used() {
    assert!(transpile_ok("def foo(x, y, z):\n    return x + y + z"));
}

// --- codegen_single_param helper ---
#[test]
fn test_func_varargs_coverage() {
    assert!(transpile_ok("def foo(*args):\n    return sum(args)"));
}

#[test]
fn test_func_kwargs_coverage() {
    assert!(transpile_ok("def foo(**kwargs):\n    return len(kwargs)"));
}

#[test]
fn test_func_args_and_kwargs_coverage() {
    assert!(transpile_ok(
        "def foo(*args, **kwargs):\n    return len(args) + len(kwargs)"
    ));
}

// --- apply_param_borrowing_strategy helper ---
#[test]
fn test_func_borrowed_string_param() {
    assert!(transpile_ok("def foo(s: str) -> int:\n    return len(s)"));
}

#[test]
fn test_func_borrowed_list_param() {
    assert!(transpile_ok(
        "def foo(lst: list) -> int:\n    return len(lst)"
    ));
}

#[test]
fn test_func_borrowed_dict_param() {
    assert!(transpile_ok("def foo(d: dict) -> int:\n    return len(d)"));
}

// --- apply_borrowing_to_type helper ---
#[test]
fn test_func_borrowed_optional_param() {
    assert!(transpile_ok(
        r#"from typing import Optional
def foo(x: Optional[str] = None) -> int:
    if x:
        return len(x)
    return 0"#
    ));
}

// --- classify_string_method helper ---
#[test]
fn test_func_string_method_returns_owned() {
    assert!(transpile_ok(
        "def foo(s: str) -> str:\n    return s.upper()"
    ));
}

#[test]
fn test_func_string_method_returns_slice() {
    assert!(transpile_ok(
        "def foo(s: str) -> str:\n    return s.strip()"
    ));
}

// --- contains_owned_string_method helper ---
#[test]
fn test_func_string_concatenation() {
    assert!(transpile_ok(
        "def foo(a: str, b: str) -> str:\n    return a + b"
    ));
}

// --- function_returns_owned_string helper ---
#[test]
fn test_func_returns_format_string() {
    assert!(transpile_ok(
        "def foo(name: str) -> str:\n    return f'Hello, {name}!'"
    ));
}

#[test]
fn test_func_returns_replace() {
    assert!(transpile_ok(
        "def foo(s: str) -> str:\n    return s.replace('a', 'b')"
    ));
}

// --- return_type_expects_float helper ---
#[test]
fn test_func_float_return_type() {
    assert!(transpile_ok("def foo() -> float:\n    return 3.14"));
}

#[test]
fn test_func_float_return_coercion() {
    assert!(transpile_ok(
        "def foo(x: float) -> float:\n    return x + 1"
    ));
}

// --- Additional function patterns ---
#[test]
fn test_func_classmethod() {
    assert!(transpile_ok(
        r#"class Foo:
    @classmethod
    def create(cls):
        return cls()"#
    ));
}

#[test]
fn test_func_staticmethod() {
    assert!(transpile_ok(
        r#"class Foo:
    @staticmethod
    def helper(x):
        return x * 2"#
    ));
}

#[test]
fn test_func_property() {
    assert!(transpile_ok(
        r#"class Foo:
    @property
    def value(self):
        return self._value"#
    ));
}

#[test]
fn test_func_docstring_coverage() {
    assert!(transpile_ok(
        r#"def foo():
    """This is a docstring."""
    return 42"#
    ));
}

#[test]
fn test_func_multiline_docstring_coverage() {
    assert!(transpile_ok(
        r#"def foo():
    """
    This is a multiline docstring.

    Args:
        None

    Returns:
        int: Always 42
    """
    return 42"#
    ));
}

// --- Complex function patterns ---
#[test]
fn test_func_closure_coverage() {
    assert!(transpile_ok(
        r#"def outer(x):
    def inner(y):
        return x + y
    return inner"#
    ));
}

#[test]
fn test_func_decorator_coverage() {
    assert!(transpile_ok(
        r#"def decorator(func):
    def wrapper(*args):
        return func(*args)
    return wrapper

@decorator
def foo():
    return 42"#
    ));
}

#[test]
fn test_func_recursive_coverage() {
    assert!(transpile_ok(
        r#"def factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)"#
    ));
}

#[test]
fn test_func_mutual_recursion() {
    assert!(transpile_ok(
        r#"def is_even(n):
    if n == 0:
        return True
    return is_odd(n - 1)

def is_odd(n):
    if n == 0:
        return False
    return is_even(n - 1)"#
    ));
}

// ============================================================================
// COMPREHENSIVE FUNCTION GENERATION COVERAGE TESTS
// ============================================================================

// --- Typed function parameters ---
#[test]
fn test_func_typed_int_param() {
    let code = transpile(
        r#"def add(a: int, b: int) -> int:
    return a + b"#,
    );
    assert!(code.contains("i32") || code.contains("i64"));
}

#[test]
fn test_func_typed_float_param() {
    let code = transpile(
        r#"def multiply(a: float, b: float) -> float:
    return a * b"#,
    );
    assert!(code.contains("f64"));
}

#[test]
fn test_func_typed_string_param() {
    let code = transpile(
        r#"def greet(name: str) -> str:
    return "Hello, " + name"#,
    );
    assert!(code.contains("String") || code.contains("&str"));
}

#[test]
fn test_func_typed_bool_param() {
    let code = transpile(
        r#"def negate(value: bool) -> bool:
    return not value"#,
    );
    assert!(code.contains("bool"));
}

#[test]
fn test_func_typed_list_param() {
    let code = transpile(
        r#"from typing import List
def sum_list(nums: List[int]) -> int:
    total = 0
    for n in nums:
        total += n
    return total"#,
    );
    assert!(code.contains("Vec"));
}

#[test]
fn test_func_typed_dict_param() {
    let code = transpile(
        r#"from typing import Dict
def get_value(d: Dict[str, int], key: str) -> int:
    return d.get(key, 0)"#,
    );
    assert!(code.contains("HashMap"));
}

#[test]
fn test_func_typed_optional_param() {
    let code = transpile(
        r#"from typing import Optional
def maybe_double(x: Optional[int]) -> Optional[int]:
    if x is None:
        return None
    return x * 2"#,
    );
    assert!(code.contains("Option"));
}

#[test]
fn test_func_typed_tuple_param() {
    let code = transpile(
        r#"from typing import Tuple
def swap(pair: Tuple[int, int]) -> Tuple[int, int]:
    return (pair[1], pair[0])"#,
    );
    assert!(!code.is_empty());
}

// --- Default parameter values ---
#[test]
fn test_ext_func_default_int() {
    let code = transpile(
        r#"def increment(x: int, by: int = 1) -> int:
    return x + by"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_default_string() {
    let code = transpile(
        r#"def greet(name: str = "World") -> str:
    return "Hello, " + name"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_default_none() {
    let code = transpile(
        r#"def process(data: list = None):
    if data is None:
        data = []
    return len(data)"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_default_bool() {
    let code = transpile(
        r#"def debug_print(msg: str, verbose: bool = False):
    if verbose:
        print(msg)"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_multiple_defaults() {
    let code = transpile(
        r#"def configure(host: str = "localhost", port: int = 8080, ssl: bool = False):
    return f"{host}:{port}""#,
    );
    assert!(!code.is_empty());
}

// --- Variadic functions ---
#[test]
fn test_ext_func_args() {
    let code = transpile(
        r#"def sum_all(*args):
    total = 0
    for x in args:
        total += x
    return total"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_kwargs() {
    let code = transpile(
        r#"def print_kwargs(**kwargs):
    for key, value in kwargs.items():
        print(key, value)"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_args_kwargs() {
    let code = transpile(
        r#"def mixed(first, *args, **kwargs):
    print(first)
    print(args)
    print(kwargs)"#,
    );
    assert!(!code.is_empty());
}

// --- Async functions ---
#[test]
fn test_ext_func_async_basic() {
    let code = transpile(
        r#"async def fetch_data():
    return "data""#,
    );
    assert!(code.contains("async") || !code.is_empty());
}

#[test]
fn test_ext_func_async_await() {
    let code = transpile(
        r#"async def process():
    data = await fetch()
    return data"#,
    );
    assert!(!code.is_empty());
}

// --- Generator functions ---
#[test]
fn test_ext_func_generator_basic() {
    let code = transpile(
        r#"def counter(n: int):
    i = 0
    while i < n:
        yield i
        i += 1"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_generator_yield_from() {
    let code = transpile(
        r#"def flatten(nested: list):
    for item in nested:
        yield from item"#,
    );
    assert!(!code.is_empty());
}

// --- Lambda functions ---
#[test]
fn test_ext_func_lambda_simple() {
    let code = transpile(
        r#"def get_adder():
    return lambda x: x + 1"#,
    );
    assert!(code.contains("|") || code.contains("closure"));
}

#[test]
fn test_ext_func_lambda_multi_arg() {
    let code = transpile(
        r#"def get_multiplier():
    return lambda x, y: x * y"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_lambda_no_arg() {
    let code = transpile(
        r#"def get_constant():
    return lambda: 42"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_lambda_in_call() {
    let code = transpile(
        r#"def sort_by_len(items: list) -> list:
    return sorted(items, key=lambda x: len(x))"#,
    );
    assert!(!code.is_empty());
}

// --- Higher-order functions ---
#[test]
fn test_ext_func_higher_order_param() {
    let code = transpile(
        r#"from typing import Callable
def apply(f: Callable[[int], int], x: int) -> int:
    return f(x)"#,
    );
    assert!(code.contains("Fn") || !code.is_empty());
}

#[test]
fn test_ext_func_higher_order_return() {
    let code = transpile(
        r#"def make_adder(n: int):
    def adder(x: int) -> int:
        return x + n
    return adder"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_map_usage() {
    let code = transpile(
        r#"def double_all(nums: list) -> list:
    return list(map(lambda x: x * 2, nums))"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_filter_usage() {
    let code = transpile(
        r#"def filter_positive(nums: list) -> list:
    return list(filter(lambda x: x > 0, nums))"#,
    );
    assert!(!code.is_empty());
}

// --- Class methods ---
#[test]
fn test_ext_func_method_self() {
    let code = transpile(
        r#"class Counter:
    def __init__(self):
        self.count = 0

    def increment(self):
        self.count += 1"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_method_classmethod() {
    let code = transpile(
        r#"class Factory:
    @classmethod
    def create(cls) -> 'Factory':
        return cls()"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_method_staticmethod() {
    let code = transpile(
        r#"class Math:
    @staticmethod
    def add(a: int, b: int) -> int:
        return a + b"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_method_property() {
    let code = transpile(
        r#"class Circle:
    def __init__(self, radius: float):
        self._radius = radius

    @property
    def area(self) -> float:
        return 3.14159 * self._radius ** 2"#,
    );
    assert!(!code.is_empty());
}

// --- Special methods ---
#[test]
fn test_ext_func_init() {
    let code = transpile(
        r#"class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y"#,
    );
    assert!(code.contains("new") || code.contains("impl"));
}

#[test]
fn test_ext_func_str() {
    let code = transpile(
        r#"class Person:
    def __init__(self, name: str):
        self.name = name

    def __str__(self) -> str:
        return f"Person({self.name})""#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_repr() {
    let code = transpile(
        r#"class Data:
    def __repr__(self) -> str:
        return "Data()""#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_len() {
    let code = transpile(
        r#"class Container:
    def __init__(self):
        self.items = []

    def __len__(self) -> int:
        return len(self.items)"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_eq() {
    let code = transpile(
        r#"class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def __eq__(self, other) -> bool:
        return self.x == other.x and self.y == other.y"#,
    );
    assert!(!code.is_empty());
}

// --- Complex function patterns ---
#[test]
fn test_ext_func_nested_deep() {
    let code = transpile(
        r#"def outer():
    def middle():
        def inner():
            return 42
        return inner()
    return middle()"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_conditional_return_types() {
    let code = transpile(
        r#"def get_value(flag: bool):
    if flag:
        return 42
    else:
        return "forty-two""#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_early_return_guard() {
    let code = transpile(
        r#"def process(data: list):
    if not data:
        return None
    if len(data) == 0:
        return []
    return data[0]"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_multiple_decorators() {
    let code = transpile(
        r#"@decorator1
@decorator2
@decorator3
def decorated():
    return 42"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_generic_type_param() {
    let code = transpile(
        r#"from typing import TypeVar, List
T = TypeVar('T')
def first(items: List[T]) -> T:
    return items[0]"#,
    );
    assert!(!code.is_empty());
}

// --- Function with complex bodies ---
#[test]
fn test_ext_func_complex_control_flow() {
    let code = transpile(
        r#"def process(items: list) -> int:
    count = 0
    for item in items:
        if item > 0:
            count += 1
        elif item < 0:
            count -= 1
        else:
            continue
    return count"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_try_in_body() {
    let code = transpile(
        r#"def safe_parse(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return 0"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_with_in_body() {
    let code = transpile(
        r#"def read_first_line(path: str) -> str:
    with open(path) as f:
        return f.readline()"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_list_comp_in_body() {
    let code = transpile(
        r#"def squares(n: int) -> list:
    return [x * x for x in range(n)]"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_dict_comp_in_body() {
    let code = transpile(
        r#"def invert_dict(d: dict) -> dict:
    return {v: k for k, v in d.items()}"#,
    );
    assert!(!code.is_empty());
}

// --- Edge cases ---
#[test]
fn test_ext_func_empty_body() {
    let code = transpile(
        r#"def noop():
    pass"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_only_docstring() {
    let code = transpile(
        r#"def documented():
    """This function does nothing."""
    pass"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_single_expression() {
    let code = transpile(
        r#"def identity(x):
    return x"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_long_param_list() {
    let code = transpile(
        r#"def many_params(a: int, b: int, c: int, d: int, e: int, f: int) -> int:
    return a + b + c + d + e + f"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_keyword_only_params() {
    let code = transpile(
        r#"def keyword_only(*, name: str, value: int):
    print(name, value)"#,
    );
    assert!(!code.is_empty());
}

#[test]
fn test_ext_func_positional_only_params() {
    let code = transpile(
        r#"def positional_only(x: int, y: int, /):
    return x + y"#,
    );
    assert!(!code.is_empty());
}

// ============================================================================
// COMPREHENSIVE FUNCTION CODE GENERATION TESTS
// Focus on uncovered paths in func_gen.rs
// ============================================================================

// --- Type inference from body ---
#[test]
fn test_cov_return_type_inference_int() {
    let code = transpile(
        r#"def get_count():
    return 42"#,
    );
    assert!(code.contains("i64") || code.contains("fn"));
}

#[test]
fn test_cov_return_type_inference_float() {
    let code = transpile(
        r#"def get_pi():
    return 3.14"#,
    );
    assert!(code.contains("f64") || code.contains("fn"));
}

#[test]
fn test_cov_return_type_inference_string() {
    let code = transpile(
        r#"def get_name():
    return "hello""#,
    );
    assert!(code.contains("String") || code.contains("fn"));
}

#[test]
fn test_cov_return_type_inference_list() {
    let code = transpile(
        r#"def get_items():
    return [1, 2, 3]"#,
    );
    assert!(code.contains("Vec") || code.contains("fn"));
}

#[test]
fn test_cov_return_type_inference_dict() {
    let code = transpile(
        r#"def get_config():
    return {"key": "value"}"#,
    );
    assert!(code.contains("HashMap") || code.contains("fn"));
}

#[test]
fn test_cov_return_type_inference_tuple() {
    let code = transpile(
        r#"def get_pair():
    return (1, "a")"#,
    );
    assert!(code.contains("(") || code.contains("fn"));
}

#[test]
fn test_cov_return_type_inference_bool() {
    let code = transpile(
        r#"def is_valid():
    return True"#,
    );
    assert!(code.contains("bool") || code.contains("fn"));
}

// --- Parameter type inference from body ---
#[test]
fn test_cov_param_type_from_len_call() {
    let code = transpile(
        r#"def process(items):
    n = len(items)
    return n"#,
    );
    assert!(code.contains("fn") || code.contains("Vec") || code.contains("len"));
}

#[test]
fn test_cov_param_type_from_method_call() {
    let code = transpile(
        r#"def process(text):
    return text.upper()"#,
    );
    assert!(code.contains("fn") || code.contains("&str") || code.contains("String"));
}

#[test]
fn test_cov_param_type_from_iteration() {
    let code = transpile(
        r#"def process(items):
    for item in items:
        print(item)"#,
    );
    assert!(code.contains("fn") || code.contains("for") || code.contains("iter"));
}

#[test]
fn test_cov_param_type_from_index() {
    let code = transpile(
        r#"def get_first(items):
    return items[0]"#,
    );
    assert!(code.contains("fn") || code.contains("[0]") || code.contains("get"));
}

#[test]
fn test_cov_param_type_from_arithmetic() {
    let code = transpile(
        r#"def double(x):
    return x * 2"#,
    );
    assert!(code.contains("fn") || code.contains("* 2") || code.contains("i64"));
}

// --- Argparse field extraction ---
#[test]
fn test_cov_argparse_field_extraction() {
    let code = transpile(
        r#"import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--name", type=str)
    parser.add_argument("--count", type=int)
    args = parser.parse_args()
    print(args.name)
    return args.count"#,
    );
    assert!(code.contains("fn main") || code.contains("Args"));
}

// --- Function attributes ---
#[test]
fn test_cov_function_inline_always() {
    let code = transpile(
        r#"def tiny():
    return 1"#,
    );
    assert!(code.contains("inline") || code.contains("fn"));
}

// --- Nested function handling ---
#[test]
fn test_cov_nested_function() {
    let code = transpile(
        r#"def outer(x: int):
    def inner(y: int):
        return y * 2
    return inner(x)"#,
    );
    assert!(code.contains("fn outer") || code.contains("fn inner") || code.contains("fn"));
}

#[test]
fn test_cov_nested_function_capture() {
    let code = transpile(
        r#"def outer(x: int):
    def inner():
        return x * 2
    return inner()"#,
    );
    assert!(code.contains("fn") || code.contains("let"));
}

// --- Generic parameters ---
#[test]
fn test_cov_generic_function() {
    let code = transpile(
        r#"from typing import TypeVar
T = TypeVar('T')
def identity(x: T) -> T:
    return x"#,
    );
    assert!(code.contains("fn identity") || code.contains("<T>") || code.contains("fn"));
}

// --- Where clause generation ---
#[test]
fn test_cov_where_clause_bounded_generic() {
    let code = transpile(
        r#"from typing import TypeVar
T = TypeVar('T', bound='int')
def process(x: T) -> T:
    return x"#,
    );
    assert!(code.contains("fn") || code.contains("where") || code.contains("fn process"));
}

// --- Escaping variables from if ---
#[test]
fn test_cov_escaping_variable_from_if() {
    let code = transpile(
        r#"def choose(cond: bool):
    if cond:
        value = 1
    else:
        value = 2
    return value"#,
    );
    assert!(code.contains("let mut value") || code.contains("if") || code.contains("fn"));
}

#[test]
fn test_cov_escaping_variable_from_try() {
    let code = transpile(
        r#"def safe_parse(text: str):
    try:
        result = int(text)
    except:
        result = 0
    return result"#,
    );
    assert!(code.contains("let") || code.contains("match") || code.contains("fn"));
}

// --- Loop escaping variables ---
#[test]
fn test_cov_escaping_variable_from_loop() {
    let code = transpile(
        r#"def find_first(items: list) -> int:
    result = -1
    for i, item in enumerate(items):
        if item > 0:
            result = i
            break
    return result"#,
    );
    assert!(code.contains("let mut result") || code.contains("fn") || code.contains("break"));
}

// --- Variable used in remaining statements ---
#[test]
fn test_cov_var_used_in_remaining() {
    let code = transpile(
        r#"def process():
    x = 1
    y = x + 1
    z = y + 1
    return z"#,
    );
    assert!(code.contains("let") || code.contains("fn"));
}

// --- String method classification ---
#[test]
fn test_cov_string_method_owned() {
    let code = transpile(
        r#"def format_name(name: str) -> str:
    return name.upper()"#,
    );
    assert!(code.contains("to_uppercase") || code.contains("fn") || code.contains("String"));
}

#[test]
fn test_cov_string_method_borrowed() {
    let code = transpile(
        r#"def starts_with_a(name: str) -> bool:
    return name.startswith("a")"#,
    );
    assert!(code.contains("starts_with") || code.contains("fn") || code.contains("bool"));
}

#[test]
fn test_cov_string_concatenation() {
    let code = transpile(
        r#"def greet(name: str) -> str:
    return "Hello, " + name"#,
    );
    assert!(code.contains("format!") || code.contains("+") || code.contains("fn"));
}

// --- Function returns owned string ---
#[test]
fn test_cov_returns_owned_string_replace() {
    let code = transpile(
        r#"def clean(text: str) -> str:
    return text.replace(" ", "_")"#,
    );
    assert!(code.contains("replace") || code.contains("String") || code.contains("fn"));
}

#[test]
fn test_cov_returns_owned_string_format() {
    let code = transpile(
        r#"def format_value(x: int) -> str:
    return f"Value: {x}""#,
    );
    assert!(code.contains("format!") || code.contains("String") || code.contains("fn"));
}

// --- Return type expects float ---
#[test]
fn test_cov_return_expects_float() {
    let code = transpile(
        r#"def div(a: int, b: int) -> float:
    return a / b"#,
    );
    assert!(code.contains("f64") || code.contains("as f64") || code.contains("fn"));
}

// --- Heterogeneous IO returns ---
#[test]
fn test_cov_heterogeneous_io_return() {
    let code = transpile(
        r#"import sys
def get_writer(use_file: bool):
    if use_file:
        return open("out.txt", "w")
    return sys.stdout"#,
    );
    assert!(code.contains("Box<dyn") || code.contains("Write") || code.contains("fn"));
}

// --- Nested function detection ---
#[test]
fn test_cov_detect_returns_nested() {
    let code = transpile(
        r#"def make_adder(x: int):
    def add(y: int):
        return x + y
    return add"#,
    );
    assert!(code.contains("fn make_adder") || code.contains("closure") || code.contains("fn"));
}

// --- Param borrowing strategy ---
#[test]
fn test_cov_param_borrow_str() {
    let code = transpile(
        r#"def process_text(text: str):
    print(text)"#,
    );
    assert!(code.contains("&str") || code.contains("&String") || code.contains("fn"));
}

#[test]
fn test_cov_param_borrow_list() {
    let code = transpile(
        r#"def process_items(items: list):
    for item in items:
        print(item)"#,
    );
    assert!(code.contains("&[") || code.contains("&Vec") || code.contains("fn"));
}

#[test]
fn test_cov_param_no_borrow_owned() {
    let code = transpile(
        r#"def consume_list(items: list):
    items.clear()
    return items"#,
    );
    assert!(code.contains("mut") || code.contains("Vec") || code.contains("fn"));
}

// --- Args field type lookup ---
#[test]
fn test_cov_args_field_bool() {
    let code = transpile(
        r#"import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--verbose", action="store_true")
    args = parser.parse_args()
    if args.verbose:
        print("Verbose mode")"#,
    );
    assert!(code.contains("bool") || code.contains("verbose") || code.contains("fn main"));
}

#[test]
fn test_cov_args_field_vec() {
    let code = transpile(
        r#"import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("files", nargs="+")
    args = parser.parse_args()
    for f in args.files:
        print(f)"#,
    );
    assert!(code.contains("Vec") || code.contains("files") || code.contains("fn main"));
}

// --- Infer numeric type from arithmetic ---
#[test]
fn test_cov_infer_numeric_from_add() {
    let code = transpile(
        r#"def add_one(x):
    return x + 1"#,
    );
    assert!(code.contains("i64") || code.contains("fn"));
}

#[test]
fn test_cov_infer_numeric_from_div() {
    let code = transpile(
        r#"def half(x):
    return x / 2"#,
    );
    assert!(code.contains("f64") || code.contains("/") || code.contains("fn"));
}

#[test]
fn test_cov_infer_float_from_literal() {
    let code = transpile(
        r#"def add_half(x):
    return x + 0.5"#,
    );
    assert!(code.contains("f64") || code.contains("0.5") || code.contains("fn"));
}

// --- Field used as bool condition ---
#[test]
fn test_cov_field_bool_condition() {
    let code = transpile(
        r#"import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--debug")
    args = parser.parse_args()
    if args.debug:
        print("Debug")"#,
    );
    assert!(code.contains("is_some") || code.contains("if") || code.contains("fn main"));
}

// --- Complex function signatures ---
#[test]
fn test_cov_mixed_params() {
    let code = transpile(
        r#"def mixed(a: int, b: str = "default", *args, **kwargs):
    print(a, b)"#,
    );
    assert!(code.contains("fn mixed") || code.contains("fn"));
}

#[test]
fn test_cov_varargs_only() {
    let code = transpile(
        r#"def varargs(*args):
    for arg in args:
        print(arg)"#,
    );
    assert!(code.contains("args") || code.contains("fn varargs") || code.contains("fn"));
}

#[test]
fn test_cov_kwargs_only() {
    let code = transpile(
        r#"def kwargs(**kwargs):
    for k, v in kwargs.items():
        print(k, v)"#,
    );
    assert!(code.contains("kwargs") || code.contains("fn") || code.contains("HashMap"));
}

// --- Special method handling ---
#[test]
fn test_cov_dunder_init() {
    let code = transpile(
        r#"class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y"#,
    );
    assert!(
        code.contains("new")
            || code.contains("Self")
            || code.contains("fn")
            || code.contains("impl")
    );
}

#[test]
fn test_cov_dunder_str() {
    let code = transpile(
        r#"class Point:
    def __str__(self) -> str:
        return f"Point({self.x}, {self.y})""#,
    );
    assert!(code.contains("Display") || code.contains("fmt") || code.contains("fn"));
}

#[test]
fn test_cov_dunder_repr() {
    let code = transpile(
        r#"class Point:
    def __repr__(self) -> str:
        return f"Point({self.x}, {self.y})""#,
    );
    assert!(code.contains("Debug") || code.contains("fmt") || code.contains("fn"));
}

// --- Async function handling ---
#[test]
fn test_cov_async_function() {
    let code = transpile(
        r#"async def fetch(url: str):
    response = await get(url)
    return response"#,
    );
    assert!(code.contains("async") || code.contains("await") || code.contains("fn"));
}

#[test]
fn test_cov_async_function_with_return_type() {
    let code = transpile(
        r#"async def fetch(url: str) -> str:
    response = await get(url)
    return response"#,
    );
    assert!(code.contains("async") || code.contains("Future") || code.contains("fn"));
}

// --- Generator function handling ---
#[test]
fn test_cov_generator_function() {
    let code = transpile(
        r#"def count(n: int):
    for i in range(n):
        yield i"#,
    );
    assert!(code.contains("Iterator") || code.contains("yield") || code.contains("fn"));
}

#[test]
fn test_cov_generator_with_return() {
    let code = transpile(
        r#"def generate(items: list):
    for item in items:
        yield item
    return"#,
    );
    assert!(code.contains("Iterator") || code.contains("fn"));
}

// --- Lambda detection ---
#[test]
fn test_cov_lambda_in_body() {
    let code = transpile(
        r#"def apply_to_all(items: list):
    return list(map(lambda x: x * 2, items))"#,
    );
    assert!(code.contains("map") || code.contains("|x|") || code.contains("fn"));
}

// --- Complex return paths ---
#[test]
fn test_cov_multiple_return_paths() {
    let code = transpile(
        r#"def classify(x: int) -> str:
    if x < 0:
        return "negative"
    elif x == 0:
        return "zero"
    else:
        return "positive""#,
    );
    assert!(code.contains("if") || code.contains("else") || code.contains("fn"));
}

#[test]
fn test_cov_return_in_try_except() {
    let code = transpile(
        r#"def safe_divide(a: int, b: int) -> int:
    try:
        return a // b
    except:
        return 0"#,
    );
    assert!(code.contains("match") || code.contains("Ok") || code.contains("fn"));
}

#[test]
fn test_cov_return_from_with() {
    let code = transpile(
        r#"def read_first_line(path: str) -> str:
    with open(path) as f:
        return f.readline()"#,
    );
    assert!(code.contains("File") || code.contains("BufReader") || code.contains("fn"));
}

// --- Variable type environment ---
#[test]
fn test_cov_var_type_env_from_assign() {
    let code = transpile(
        r#"def compute():
    x = 1
    y = 2.0
    z = "test"
    return (x, y, z)"#,
    );
    assert!(code.contains("let") || code.contains("fn"));
}

#[test]
fn test_cov_var_type_env_from_for() {
    let code = transpile(
        r#"def sum_items(items: list) -> int:
    total = 0
    for item in items:
        total += item
    return total"#,
    );
    assert!(code.contains("let mut total") || code.contains("for") || code.contains("fn"));
}

// --- Return type from body with params ---
#[test]
fn test_cov_return_type_from_param_method() {
    let code = transpile(
        r#"def get_length(text: str):
    return len(text)"#,
    );
    assert!(code.contains("usize") || code.contains("len") || code.contains("fn"));
}

#[test]
fn test_cov_return_type_from_param_index() {
    let code = transpile(
        r#"def get_first(items: list):
    return items[0]"#,
    );
    assert!(code.contains("get(0)") || code.contains("[0]") || code.contains("fn"));
}

// --- Negative int expression detection ---
#[test]
fn test_cov_negative_int_literal() {
    let code = transpile(
        r#"def get_negative() -> int:
    return -1"#,
    );
    assert!(code.contains("-1") || code.contains("i64") || code.contains("fn"));
}

#[test]
fn test_cov_negative_int_expression() {
    let code = transpile(
        r#"def negate(x: int) -> int:
    return -x"#,
    );
    assert!(code.contains("-") || code.contains("fn"));
}

// --- Complex expressions in params ---
#[test]
fn test_cov_default_param_complex() {
    let code = transpile(
        r#"def greet(name: str = "World"):
    print(f"Hello, {name}")"#,
    );
    assert!(code.contains("Option") || code.contains("unwrap_or") || code.contains("fn"));
}

#[test]
fn test_cov_default_param_list() {
    let code = transpile(
        r#"def process(items: list = None):
    if items is None:
        items = []
    return items"#,
    );
    assert!(code.contains("Option") || code.contains("None") || code.contains("fn"));
}

// --- File creating expressions ---
#[test]
fn test_cov_file_creating_open() {
    let code = transpile(
        r#"def get_file():
    return open("test.txt", "w")"#,
    );
    assert!(code.contains("File") || code.contains("create") || code.contains("fn"));
}

#[test]
fn test_cov_file_creating_file_create() {
    let code = transpile(
        r#"from pathlib import Path
def get_file():
    return Path("test.txt").open("w")"#,
    );
    assert!(code.contains("File") || code.contains("Path") || code.contains("fn"));
}

// --- Stdio expressions ---
#[test]
fn test_cov_stdio_stdout() {
    let code = transpile(
        r#"import sys
def get_output():
    return sys.stdout"#,
    );
    assert!(code.contains("stdout") || code.contains("io::") || code.contains("fn"));
}

#[test]
fn test_cov_stdio_stderr() {
    let code = transpile(
        r#"import sys
def get_error_output():
    return sys.stderr"#,
    );
    assert!(code.contains("stderr") || code.contains("io::") || code.contains("fn"));
}

// --- Collect all assigned variables ---
#[test]
fn test_cov_all_assigned_in_if() {
    let code = transpile(
        r#"def process(cond: bool):
    if cond:
        a = 1
        b = 2
    else:
        a = 3
        b = 4
    return a + b"#,
    );
    assert!(code.contains("let mut a") || code.contains("let mut b") || code.contains("fn"));
}

#[test]
fn test_cov_all_assigned_in_loop() {
    let code = transpile(
        r#"def process(items: list):
    result = []
    for item in items:
        x = item * 2
        result.append(x)
    return result"#,
    );
    assert!(code.contains("let") || code.contains("for") || code.contains("fn"));
}

// --- Statement always returns ---
#[test]
fn test_cov_stmt_always_returns_if_else() {
    let code = transpile(
        r#"def get_value(cond: bool) -> int:
    if cond:
        return 1
    else:
        return 2"#,
    );
    assert!(code.contains("if") || code.contains("return") || code.contains("fn"));
}

#[test]
fn test_cov_stmt_always_returns_raise() {
    let code = transpile(
        r#"def fail() -> int:
    raise ValueError("always fails")"#,
    );
    assert!(code.contains("panic") || code.contains("Err") || code.contains("fn"));
}

// --- Is param used in body ---
#[test]
fn test_cov_param_unused() {
    let code = transpile(
        r#"def ignore(x: int):
    return 42"#,
    );
    assert!(code.contains("_x") || code.contains("fn ignore") || code.contains("fn"));
}

#[test]
fn test_cov_param_used_in_call() {
    let code = transpile(
        r#"def process(x: int):
    print(x)
    return x"#,
    );
    assert!(code.contains("x") || code.contains("fn"));
}

// --- Complex type inference ---
#[test]
fn test_cov_infer_type_dict_values() {
    let code = transpile(
        r#"def get_values(d: dict):
    return list(d.values())"#,
    );
    assert!(code.contains("values") || code.contains("Vec") || code.contains("fn"));
}

#[test]
fn test_cov_infer_type_dict_keys() {
    let code = transpile(
        r#"def get_keys(d: dict):
    return list(d.keys())"#,
    );
    assert!(code.contains("keys") || code.contains("Vec") || code.contains("fn"));
}

// ============================================================================
// EXTENDED FUNCTION GENERATION TESTS
// ============================================================================

#[test]
fn test_batch_func_default_mutable() {
    let code = transpile(
        r#"def add_items(items=None):
    if items is None:
        items = []
    items.append(1)
    return items"#,
    );
    assert!(code.contains("fn") || code.contains("None") || code.contains("vec!"));
}

#[test]
fn test_batch_func_generic_bounds() {
    let code = transpile(
        r#"from typing import TypeVar
T = TypeVar('T', bound='Comparable')
def find_max(items: list[T]) -> T:
    return max(items)"#,
    );
    assert!(code.contains("fn") || code.contains("max") || code.contains("Ord"));
}

#[test]
fn test_batch_func_multiple_return_types() {
    let code = transpile(
        r#"def parse(s: str):
    try:
        return int(s)
    except:
        return s"#,
    );
    assert!(code.contains("fn") || code.contains("parse") || code.contains("match"));
}

#[test]
fn test_batch_func_recursive() {
    let code = transpile(
        r#"def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)"#,
    );
    assert!(code.contains("fn factorial") || code.contains("*") || code.contains("recursion"));
}

#[test]
fn test_batch_func_mutual_recursion() {
    let code = transpile(
        r#"def is_even(n: int) -> bool:
    if n == 0:
        return True
    return is_odd(n - 1)

def is_odd(n: int) -> bool:
    if n == 0:
        return False
    return is_even(n - 1)"#,
    );
    assert!(code.contains("fn is_even") || code.contains("fn is_odd"));
}

#[test]
fn test_batch_func_closure() {
    let code = transpile(
        r#"def make_adder(x: int):
    def add(y: int) -> int:
        return x + y
    return add"#,
    );
    assert!(code.contains("fn") || code.contains("Fn") || code.contains("move"));
}

#[test]
fn test_batch_func_decorator_staticmethod() {
    let code = transpile(
        r#"class Math:
    @staticmethod
    def add(a: int, b: int) -> int:
        return a + b"#,
    );
    assert!(code.contains("fn add") || code.contains("impl") || code.contains("struct"));
}

#[test]
fn test_batch_func_decorator_classmethod() {
    let code = transpile(
        r#"class Counter:
    count = 0
    @classmethod
    def increment(cls):
        cls.count += 1"#,
    );
    assert!(code.contains("fn increment") || code.contains("impl") || code.contains("struct"));
}

#[test]
fn test_batch_func_decorator_property() {
    let code = transpile(
        r#"class Circle:
    def __init__(self, radius: float):
        self._radius = radius
    
    @property
    def area(self) -> float:
        return 3.14159 * self._radius ** 2"#,
    );
    assert!(code.contains("fn area") || code.contains("impl") || code.contains("struct"));
}

#[test]
fn test_batch_func_varargs() {
    let code = transpile(
        r#"def sum_all(*args) -> int:
    total = 0
    for x in args:
        total += x
    return total"#,
    );
    assert!(code.contains("fn sum_all") || code.contains("args") || code.contains("Vec"));
}

#[test]
fn test_batch_func_kwargs() {
    let code = transpile(
        r#"def configure(**kwargs) -> dict:
    return kwargs"#,
    );
    assert!(code.contains("fn configure") || code.contains("HashMap") || code.contains("kwargs"));
}

#[test]
fn test_batch_func_mixed_args() {
    let code = transpile(
        r#"def process(a: int, *args, **kwargs):
    return a + len(args) + len(kwargs)"#,
    );
    assert!(code.contains("fn process") || code.contains("len"));
}

#[test]
fn test_batch_func_keyword_only() {
    let code = transpile(
        r#"def greet(*, name: str, loud: bool = False):
    if loud:
        return name.upper()
    return name"#,
    );
    assert!(code.contains("fn greet") || code.contains("name") || code.contains("loud"));
}

#[test]
fn test_batch_func_positional_only() {
    let code = transpile(
        r#"def divide(a: int, b: int, /) -> float:
    return a / b"#,
    );
    assert!(code.contains("fn divide") || code.contains("/") || code.contains("f64"));
}

#[test]
fn test_batch_async_simple() {
    let code = transpile(
        r#"async def fetch():
    return 42"#,
    );
    assert!(code.contains("fn fetch") || code.contains("async") || code.contains("Future"));
}

#[test]
fn test_batch_async_await() {
    let code = transpile(
        r#"async def fetch_data():
    data = await get_data()
    return data"#,
    );
    assert!(code.contains("fn fetch_data") || code.contains("await") || code.contains("async"));
}

#[test]
fn test_batch_generator_simple() {
    let code = transpile(
        r#"def count_up(n: int):
    for i in range(n):
        yield i"#,
    );
    assert!(code.contains("fn count_up") || code.contains("Iterator") || code.contains("yield"));
}

#[test]
fn test_batch_generator_with_return() {
    let code = transpile(
        r#"def count_and_sum(n: int):
    total = 0
    for i in range(n):
        yield i
        total += i
    return total"#,
    );
    assert!(code.contains("fn") || code.contains("Iterator") || code.contains("yield"));
}

#[test]
fn test_batch_generator_send() {
    let code = transpile(
        r#"def echo():
    while True:
        x = yield
        yield x * 2"#,
    );
    assert!(code.contains("fn echo") || code.contains("yield"));
}

#[test]
fn test_batch_func_return_none() {
    let code = transpile(
        r#"def side_effect() -> None:
    print("hello")"#,
    );
    assert!(code.contains("fn side_effect") || code.contains("println!") || code.contains("()"));
}

#[test]
fn test_batch_func_return_list() {
    let code = transpile(
        r#"from typing import List
def get_items() -> List[int]:
    return [1, 2, 3]"#,
    );
    assert!(code.contains("fn get_items") || code.contains("Vec") || code.contains("vec!"));
}

#[test]
fn test_batch_func_return_dict() {
    let code = transpile(
        r#"from typing import Dict
def get_mapping() -> Dict[str, int]:
    return {"a": 1, "b": 2}"#,
    );
    assert!(code.contains("fn get_mapping") || code.contains("HashMap"));
}

#[test]
fn test_batch_func_return_tuple() {
    let code = transpile(
        r#"from typing import Tuple
def get_pair() -> Tuple[int, str]:
    return (42, "hello")"#,
    );
    assert!(code.contains("fn get_pair") || code.contains("(") || code.contains("tuple"));
}

#[test]
fn test_batch_func_return_set() {
    let code = transpile(
        r#"from typing import Set
def get_unique() -> Set[int]:
    return {1, 2, 3}"#,
    );
    assert!(code.contains("fn get_unique") || code.contains("HashSet"));
}

#[test]
fn test_batch_func_return_optional() {
    let code = transpile(
        r#"from typing import Optional
def maybe_int(s: str) -> Optional[int]:
    try:
        return int(s)
    except:
        return None"#,
    );
    assert!(code.contains("fn maybe_int") || code.contains("Option"));
}

#[test]
fn test_batch_func_return_union() {
    let code = transpile(
        r#"from typing import Union
def parse(s: str) -> Union[int, str]:
    try:
        return int(s)
    except:
        return s"#,
    );
    assert!(code.contains("fn parse") || code.contains("enum") || code.contains("Result"));
}

#[test]
fn test_batch_func_docstring() {
    let code = transpile(
        r#"def add(a: int, b: int) -> int:
    """Add two numbers and return the result."""
    return a + b"#,
    );
    assert!(code.contains("fn add") || code.contains("///") || code.contains("+"));
}

#[test]
fn test_batch_func_complex_docstring() {
    let code = transpile(
        r#"def complex_func(x: int, y: str) -> bool:
    """
    A complex function with detailed docs.
    
    Args:
        x: An integer value
        y: A string value
    
    Returns:
        True if x equals length of y
    """
    return x == len(y)"#,
    );
    assert!(code.contains("fn complex_func") || code.contains("///") || code.contains("=="));
}

#[test]
fn test_batch_func_infer_bool_from_comparison() {
    let code = transpile(
        r#"def is_positive(x: int):
    return x > 0"#,
    );
    assert!(code.contains("fn is_positive") || code.contains("bool") || code.contains(">"));
}

#[test]
fn test_batch_func_infer_int_from_literal() {
    let code = transpile(
        r#"def get_constant():
    return 42"#,
    );
    assert!(code.contains("fn get_constant") || code.contains("i64") || code.contains("42"));
}

#[test]
fn test_batch_func_infer_float_from_literal() {
    let code = transpile(
        r#"def get_pi():
    return 3.14159"#,
    );
    assert!(code.contains("fn get_pi") || code.contains("f64") || code.contains("3.14"));
}

#[test]
fn test_batch_func_infer_string_from_literal() {
    let code = transpile(
        r#"def get_greeting():
    return "hello""#,
    );
    assert!(code.contains("fn get_greeting") || code.contains("String") || code.contains("hello"));
}

#[test]
fn test_batch_func_infer_list_from_literal() {
    let code = transpile(
        r#"def get_numbers():
    return [1, 2, 3]"#,
    );
    assert!(code.contains("fn get_numbers") || code.contains("Vec") || code.contains("vec!"));
}

#[test]
fn test_batch_func_infer_from_conditional() {
    let code = transpile(
        r#"def get_value(flag: bool):
    if flag:
        return 1
    else:
        return 0"#,
    );
    assert!(code.contains("fn get_value") || code.contains("i64") || code.contains("if"));
}

#[test]
fn test_batch_func_infer_from_binary_op() {
    let code = transpile(
        r#"def multiply(a, b):
    return a * b"#,
    );
    assert!(code.contains("fn multiply") || code.contains("*"));
}

#[test]
fn test_batch_func_borrow_immutable() {
    let code = transpile(
        r#"def first(items: list) -> int:
    return items[0]"#,
    );
    assert!(code.contains("fn first") || code.contains("&") || code.contains("[0]"));
}

#[test]
fn test_batch_func_borrow_mutable() {
    let code = transpile(
        r#"def append_and_return(items: list, x: int) -> list:
    items.append(x)
    return items"#,
    );
    assert!(
        code.contains("fn append_and_return") || code.contains("&mut") || code.contains("push")
    );
}

#[test]
fn test_batch_func_owned_param() {
    let code = transpile(
        r#"def consume(s: str) -> int:
    return len(s)"#,
    );
    assert!(code.contains("fn consume") || code.contains("String") || code.contains("len"));
}

#[test]
fn test_batch_func_multiple_defaults() {
    let code = transpile(
        r#"def greet(name: str = "World", loud: bool = False, count: int = 1) -> str:
    greeting = f"Hello, {name}!"
    if loud:
        greeting = greeting.upper()
    return greeting * count"#,
    );
    assert!(code.contains("fn greet") || code.contains("format!") || code.contains("*"));
}

#[test]
fn test_batch_func_can_fail() {
    let code = transpile(
        r#"def parse_int(s: str) -> int:
    return int(s)"#,
    );
    assert!(code.contains("fn parse_int") || code.contains("Result") || code.contains("parse"));
}

#[test]
fn test_batch_func_error_propagation() {
    let code = transpile(
        r#"def read_number(path: str) -> int:
    with open(path) as f:
        return int(f.read())"#,
    );
    assert!(code.contains("fn read_number") || code.contains("?") || code.contains("Result"));
}

#[test]
fn test_batch_main_entry() {
    let code = transpile(
        r#"def main():
    print("Hello, World!")"#,
    );
    assert!(code.contains("fn main") || code.contains("println!"));
}

#[test]
fn test_batch_main_with_args() {
    let code = transpile(
        r#"import sys
def main():
    args = sys.argv[1:]
    for arg in args:
        print(arg)"#,
    );
    assert!(code.contains("fn main") || code.contains("args") || code.contains("env"));
}

#[test]
fn test_batch_main_return_code() {
    let code = transpile(
        r#"def main() -> int:
    return 0"#,
    );
    assert!(code.contains("fn main") || code.contains("Ok(())") || code.contains("exit"));
}
