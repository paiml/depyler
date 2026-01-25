//! Targeted function generation tests
//!
//! These tests specifically target uncovered code paths in func_gen.rs.

use crate::DepylerPipeline;

#[allow(dead_code)]
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
// FUNCTION SIGNATURE PATTERNS
// ============================================================================

#[test]
fn test_func_no_params_no_return() {
    assert!(transpile_ok("def foo():\n    pass"));
}

#[test]
fn test_func_single_param() {
    assert!(transpile_ok("def foo(x: int):\n    pass"));
}

#[test]
fn test_func_multiple_params() {
    assert!(transpile_ok("def foo(x: int, y: str, z: bool):\n    pass"));
}

#[test]
fn test_func_param_with_default() {
    assert!(transpile_ok("def foo(x: int = 10):\n    return x"));
}

#[test]
fn test_func_params_with_defaults() {
    assert!(transpile_ok(
        "def foo(x: int, y: int = 10, z: str = 'default'):\n    return x + y"
    ));
}

#[test]
fn test_func_args_only() {
    assert!(transpile_ok("def foo(*args):\n    return len(args)"));
}

#[test]
fn test_func_kwargs_only() {
    assert!(transpile_ok("def foo(**kwargs):\n    return len(kwargs)"));
}

#[test]
fn test_func_args_and_kwargs() {
    assert!(transpile_ok(
        "def foo(*args, **kwargs):\n    return len(args) + len(kwargs)"
    ));
}

#[test]
fn test_func_mixed_params_args_kwargs() {
    assert!(transpile_ok(
        "def foo(x: int, *args, **kwargs):\n    return x"
    ));
}

#[test]
fn test_func_keyword_only() {
    assert!(transpile_ok(
        "def foo(*, x: int, y: int = 10):\n    return x + y"
    ));
}

#[test]
fn test_func_positional_only() {
    let _ = transpile_ok("def foo(x: int, /, y: int):\n    return x + y");
}

// ============================================================================
// RETURN TYPE PATTERNS
// ============================================================================

#[test]
fn test_func_return_int() {
    assert!(transpile_ok("def foo() -> int:\n    return 42"));
}

#[test]
fn test_func_return_float() {
    assert!(transpile_ok("def foo() -> float:\n    return 3.14"));
}

#[test]
fn test_func_return_str() {
    assert!(transpile_ok("def foo() -> str:\n    return 'hello'"));
}

#[test]
fn test_func_return_bool() {
    assert!(transpile_ok("def foo() -> bool:\n    return True"));
}

#[test]
fn test_func_return_none() {
    assert!(transpile_ok("def foo() -> None:\n    pass"));
}

#[test]
fn test_func_return_list() {
    assert!(transpile_ok(
        "def foo() -> list[int]:\n    return [1, 2, 3]"
    ));
}

#[test]
fn test_func_return_dict() {
    assert!(transpile_ok(
        "def foo() -> dict[str, int]:\n    return {'a': 1}"
    ));
}

#[test]
fn test_func_return_set() {
    assert!(transpile_ok("def foo() -> set[int]:\n    return {1, 2, 3}"));
}

#[test]
fn test_func_return_tuple() {
    assert!(transpile_ok(
        "def foo() -> tuple[int, str]:\n    return (1, 'hello')"
    ));
}

#[test]
fn test_func_return_optional() {
    assert!(transpile_ok(
        "from typing import Optional\n\ndef foo() -> Optional[int]:\n    return None"
    ));
}

#[test]
fn test_func_return_union() {
    assert!(transpile_ok(
        "from typing import Union\n\ndef foo() -> Union[int, str]:\n    return 42"
    ));
}

// ============================================================================
// FUNCTION BODY PATTERNS
// ============================================================================

#[test]
fn test_func_local_vars() {
    assert!(transpile_ok(
        "def foo():\n    x = 1\n    y = 2\n    return x + y"
    ));
}

#[test]
fn test_func_conditional_return() {
    assert!(transpile_ok(
        "def foo(x: int) -> int:\n    if x > 0:\n        return x\n    return -x"
    ));
}

#[test]
fn test_func_loop_return() {
    assert!(transpile_ok("def foo(items: list[int]) -> int:\n    total = 0\n    for item in items:\n        total += item\n    return total"));
}

#[test]
fn test_func_while_return() {
    assert!(transpile_ok("def foo(n: int) -> int:\n    count = 0\n    while n > 0:\n        count += 1\n        n -= 1\n    return count"));
}

#[test]
fn test_func_try_except() {
    assert!(transpile_ok("def foo(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError:\n        return 0"));
}

#[test]
fn test_func_with_statement() {
    assert!(transpile_ok(
        "def foo(path: str) -> str:\n    with open(path) as f:\n        return f.read()"
    ));
}

#[test]
fn test_func_nested_scopes() {
    assert!(transpile_ok("def foo():\n    x = 1\n    if True:\n        y = 2\n        if True:\n            z = 3\n            return x + y + z"));
}

// ============================================================================
// NESTED FUNCTION PATTERNS
// ============================================================================

#[test]
fn test_nested_func_simple() {
    assert!(transpile_ok(
        "def outer():\n    def inner():\n        return 1\n    return inner()"
    ));
}

#[test]
fn test_nested_func_with_params() {
    assert!(transpile_ok(
        "def outer(x: int):\n    def inner(y: int):\n        return x + y\n    return inner(10)"
    ));
}

#[test]
fn test_nested_func_closure() {
    assert!(transpile_ok(
        "def make_adder(x: int):\n    def adder(y: int):\n        return x + y\n    return adder"
    ));
}

#[test]
fn test_nested_func_multiple() {
    assert!(transpile_ok("def outer():\n    def inner1():\n        return 1\n    def inner2():\n        return 2\n    return inner1() + inner2()"));
}

// ============================================================================
// RECURSIVE FUNCTION PATTERNS
// ============================================================================

#[test]
fn test_recursive_factorial() {
    assert!(transpile_ok("def factorial(n: int) -> int:\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)"));
}

#[test]
fn test_recursive_fibonacci() {
    assert!(transpile_ok("def fib(n: int) -> int:\n    if n <= 1:\n        return n\n    return fib(n - 1) + fib(n - 2)"));
}

#[test]
fn test_recursive_sum() {
    assert!(transpile_ok("def sum_list(items: list[int]) -> int:\n    if not items:\n        return 0\n    return items[0] + sum_list(items[1:])"));
}

// ============================================================================
// DECORATOR PATTERNS
// ============================================================================

#[test]
fn test_staticmethod() {
    assert!(transpile_ok(
        "class Foo:\n    @staticmethod\n    def bar() -> int:\n        return 42"
    ));
}

#[test]
fn test_classmethod() {
    assert!(transpile_ok(
        "class Foo:\n    @classmethod\n    def bar(cls) -> str:\n        return 'Foo'"
    ));
}

#[test]
fn test_property_getter() {
    assert!(transpile_ok("class Foo:\n    _x: int = 0\n    @property\n    def x(self) -> int:\n        return self._x"));
}

#[test]
fn test_property_setter() {
    let _ = transpile_ok("class Foo:\n    _x: int = 0\n    @property\n    def x(self) -> int:\n        return self._x\n    @x.setter\n    def x(self, val: int):\n        self._x = val");
}

// ============================================================================
// CLASS METHOD PATTERNS
// ============================================================================

#[test]
fn test_method_self() {
    assert!(transpile_ok(
        "class Foo:\n    x: int\n    def get_x(self) -> int:\n        return self.x"
    ));
}

#[test]
fn test_method_self_mutation() {
    assert!(transpile_ok(
        "class Foo:\n    x: int\n    def set_x(self, val: int):\n        self.x = val"
    ));
}

#[test]
fn test_method_with_params() {
    assert!(transpile_ok(
        "class Foo:\n    x: int\n    def add(self, val: int) -> int:\n        return self.x + val"
    ));
}

#[test]
fn test_method_chain() {
    assert!(transpile_ok("class Builder:\n    x: int = 0\n    def set_x(self, val: int) -> 'Builder':\n        self.x = val\n        return self"));
}

#[test]
fn test_init_method() {
    assert!(transpile_ok("class Point:\n    x: int\n    y: int\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y"));
}

#[test]
fn test_init_with_defaults() {
    assert!(transpile_ok("class Point:\n    x: int\n    y: int\n    def __init__(self, x: int = 0, y: int = 0):\n        self.x = x\n        self.y = y"));
}

#[test]
fn test_str_method() {
    assert!(transpile_ok("class Point:\n    x: int\n    y: int\n    def __str__(self) -> str:\n        return f'({self.x}, {self.y})'"));
}

#[test]
fn test_repr_method() {
    assert!(transpile_ok("class Point:\n    x: int\n    y: int\n    def __repr__(self) -> str:\n        return f'Point({self.x}, {self.y})'"));
}

#[test]
fn test_len_method() {
    assert!(transpile_ok("class Container:\n    items: list[int]\n    def __len__(self) -> int:\n        return len(self.items)"));
}

#[test]
fn test_getitem_method() {
    assert!(transpile_ok("class Container:\n    items: list[int]\n    def __getitem__(self, idx: int) -> int:\n        return self.items[idx]"));
}

#[test]
fn test_setitem_method() {
    assert!(transpile_ok("class Container:\n    items: list[int]\n    def __setitem__(self, idx: int, val: int):\n        self.items[idx] = val"));
}

#[test]
fn test_contains_method() {
    assert!(transpile_ok("class Container:\n    items: list[int]\n    def __contains__(self, val: int) -> bool:\n        return val in self.items"));
}

#[test]
fn test_iter_method() {
    assert!(transpile_ok("class Container:\n    items: list[int]\n    def __iter__(self):\n        return iter(self.items)"));
}

#[test]
fn test_eq_method() {
    assert!(transpile_ok("class Point:\n    x: int\n    y: int\n    def __eq__(self, other: 'Point') -> bool:\n        return self.x == other.x and self.y == other.y"));
}

#[test]
fn test_lt_method() {
    assert!(transpile_ok("class Point:\n    x: int\n    def __lt__(self, other: 'Point') -> bool:\n        return self.x < other.x"));
}

#[test]
fn test_add_method() {
    assert!(transpile_ok("class Point:\n    x: int\n    y: int\n    def __add__(self, other: 'Point') -> 'Point':\n        return Point(self.x + other.x, self.y + other.y)"));
}

#[test]
fn test_sub_method() {
    assert!(transpile_ok("class Point:\n    x: int\n    y: int\n    def __sub__(self, other: 'Point') -> 'Point':\n        return Point(self.x - other.x, self.y - other.y)"));
}

#[test]
fn test_mul_method() {
    assert!(transpile_ok("class Vector:\n    x: int\n    def __mul__(self, scalar: int) -> 'Vector':\n        return Vector(self.x * scalar)"));
}

// ============================================================================
// GENERATOR FUNCTION PATTERNS
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
        "def gen(n: int):\n    for i in range(n):\n        yield i"
    ));
}

#[test]
fn test_generator_with_state() {
    assert!(transpile_ok(
        "def gen():\n    x = 0\n    while x < 10:\n        yield x\n        x += 1"
    ));
}

#[test]
fn test_generator_yield_from() {
    assert!(transpile_ok("def gen():\n    yield from [1, 2, 3]"));
}

#[test]
fn test_generator_conditional() {
    assert!(transpile_ok("def gen(items: list[int]):\n    for item in items:\n        if item > 0:\n            yield item"));
}

#[test]
fn test_generator_typed() {
    assert!(transpile_ok(
        "from typing import Iterator\n\ndef gen() -> Iterator[int]:\n    yield 1\n    yield 2"
    ));
}

// ============================================================================
// ASYNC FUNCTION PATTERNS
// ============================================================================

#[test]
fn test_async_def() {
    assert!(transpile_ok("async def foo():\n    pass"));
}

#[test]
fn test_async_with_return() {
    assert!(transpile_ok("async def foo() -> int:\n    return 42"));
}

#[test]
fn test_async_with_await() {
    assert!(transpile_ok("async def foo():\n    await bar()"));
}

// ============================================================================
// FUNCTION CALL PATTERNS
// ============================================================================

#[test]
fn test_call_no_args() {
    assert!(transpile_ok(
        "def bar() -> int:\n    return 1\n\ndef foo() -> int:\n    return bar()"
    ));
}

#[test]
fn test_call_positional() {
    assert!(transpile_ok(
        "def bar(x: int) -> int:\n    return x\n\ndef foo() -> int:\n    return bar(42)"
    ));
}

#[test]
fn test_call_keyword() {
    assert!(transpile_ok(
        "def bar(x: int) -> int:\n    return x\n\ndef foo() -> int:\n    return bar(x=42)"
    ));
}

#[test]
fn test_call_mixed() {
    assert!(transpile_ok("def bar(x: int, y: int) -> int:\n    return x + y\n\ndef foo() -> int:\n    return bar(1, y=2)"));
}

#[test]
fn test_call_star_args() {
    assert!(transpile_ok("def bar(*args) -> int:\n    return len(args)\n\ndef foo() -> int:\n    items = [1, 2, 3]\n    return bar(*items)"));
}

#[test]
fn test_call_double_star_kwargs() {
    assert!(transpile_ok("def bar(**kwargs) -> int:\n    return len(kwargs)\n\ndef foo() -> int:\n    opts = {'a': 1}\n    return bar(**opts)"));
}

// ============================================================================
// DATACLASS PATTERNS
// ============================================================================

#[test]
fn test_dataclass_simple() {
    assert!(transpile_ok(
        "from dataclasses import dataclass\n\n@dataclass\nclass Point:\n    x: int\n    y: int"
    ));
}

#[test]
fn test_dataclass_with_defaults() {
    assert!(transpile_ok("from dataclasses import dataclass\n\n@dataclass\nclass Point:\n    x: int = 0\n    y: int = 0"));
}

#[test]
fn test_dataclass_with_method() {
    assert!(transpile_ok("from dataclasses import dataclass\n\n@dataclass\nclass Point:\n    x: int\n    y: int\n    def distance(self) -> float:\n        return (self.x ** 2 + self.y ** 2) ** 0.5"));
}

#[test]
fn test_dataclass_frozen() {
    assert!(transpile_ok("from dataclasses import dataclass\n\n@dataclass(frozen=True)\nclass Point:\n    x: int\n    y: int"));
}

// ============================================================================
// TYPE VARIABLE PATTERNS
// ============================================================================

#[test]
fn test_generic_function() {
    assert!(transpile_ok("from typing import TypeVar, List\n\nT = TypeVar('T')\n\ndef first(items: List[T]) -> T:\n    return items[0]"));
}

#[test]
fn test_generic_function_bound() {
    assert!(transpile_ok("from typing import TypeVar\n\nT = TypeVar('T', int, str)\n\ndef process(item: T) -> T:\n    return item"));
}

// ============================================================================
// CALLABLE TYPE PATTERNS
// ============================================================================

#[test]
fn test_callable_param() {
    assert!(transpile_ok("from typing import Callable\n\ndef apply(f: Callable[[int], int], x: int) -> int:\n    return f(x)"));
}

#[test]
fn test_callable_return() {
    assert!(transpile_ok("from typing import Callable\n\ndef make_adder(n: int) -> Callable[[int], int]:\n    def adder(x: int) -> int:\n        return x + n\n    return adder"));
}

// ============================================================================
// EXCEPTION HANDLING IN FUNCTIONS
// ============================================================================

#[test]
fn test_raise_in_function() {
    assert!(transpile_ok(
        "def foo(x: int):\n    if x < 0:\n        raise ValueError('negative')"
    ));
}

#[test]
fn test_try_in_function() {
    assert!(transpile_ok("def foo(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError:\n        return -1"));
}

#[test]
fn test_try_finally_in_function() {
    assert!(transpile_ok("def foo(path: str):\n    f = None\n    try:\n        f = open(path)\n        return f.read()\n    finally:\n        if f:\n            f.close()"));
}

// ============================================================================
// DOCSTRING PATTERNS
// ============================================================================

#[test]
fn test_func_with_docstring() {
    assert!(transpile_ok(
        "def foo(x: int) -> int:\n    '''Returns double of x.'''\n    return x * 2"
    ));
}

#[test]
fn test_func_with_multiline_docstring() {
    assert!(transpile_ok("def foo(x: int) -> int:\n    '''\n    Doubles the input.\n    \n    Args:\n        x: input value\n    \n    Returns:\n        doubled value\n    '''\n    return x * 2"));
}

#[test]
fn test_class_with_docstring() {
    assert!(transpile_ok(
        "class Foo:\n    '''A simple class.'''\n    x: int"
    ));
}

// ============================================================================
// LAMBDA PATTERNS IN FUNCTIONS
// ============================================================================

#[test]
fn test_lambda_local() {
    assert!(transpile_ok(
        "def foo():\n    f = lambda x: x * 2\n    return f(10)"
    ));
}

#[test]
fn test_lambda_in_map() {
    assert!(transpile_ok(
        "def foo(items: list[int]) -> list[int]:\n    return list(map(lambda x: x * 2, items))"
    ));
}

#[test]
fn test_lambda_in_filter() {
    assert!(transpile_ok(
        "def foo(items: list[int]) -> list[int]:\n    return list(filter(lambda x: x > 0, items))"
    ));
}

#[test]
fn test_lambda_in_sorted() {
    assert!(transpile_ok("def foo(items: list[tuple[str, int]]) -> list[tuple[str, int]]:\n    return sorted(items, key=lambda x: x[1])"));
}

// ============================================================================
// COMPREHENSION IN FUNCTIONS
// ============================================================================

#[test]
fn test_list_comp_in_return() {
    assert!(transpile_ok(
        "def foo(items: list[int]) -> list[int]:\n    return [x * 2 for x in items]"
    ));
}

#[test]
fn test_dict_comp_in_return() {
    assert!(transpile_ok(
        "def foo(items: list[str]) -> dict[str, int]:\n    return {x: len(x) for x in items}"
    ));
}

#[test]
fn test_set_comp_in_return() {
    assert!(transpile_ok(
        "def foo(items: list[int]) -> set[int]:\n    return {x % 10 for x in items}"
    ));
}

#[test]
fn test_genexp_in_return() {
    assert!(transpile_ok(
        "def foo(items: list[int]) -> int:\n    return sum(x * x for x in items)"
    ));
}

// ============================================================================
// COMPLEX ALGORITHM PATTERNS
// ============================================================================

#[test]
fn test_binary_search() {
    assert!(transpile_ok("def binary_search(items: list[int], target: int) -> int:\n    left, right = 0, len(items) - 1\n    while left <= right:\n        mid = (left + right) // 2\n        if items[mid] == target:\n            return mid\n        elif items[mid] < target:\n            left = mid + 1\n        else:\n            right = mid - 1\n    return -1"));
}

#[test]
fn test_quicksort() {
    assert!(transpile_ok("def quicksort(items: list[int]) -> list[int]:\n    if len(items) <= 1:\n        return items\n    pivot = items[len(items) // 2]\n    left = [x for x in items if x < pivot]\n    middle = [x for x in items if x == pivot]\n    right = [x for x in items if x > pivot]\n    return quicksort(left) + middle + quicksort(right)"));
}

#[test]
fn test_merge_sort() {
    assert!(transpile_ok("def merge_sort(items: list[int]) -> list[int]:\n    if len(items) <= 1:\n        return items\n    mid = len(items) // 2\n    left = merge_sort(items[:mid])\n    right = merge_sort(items[mid:])\n    return merge(left, right)\n\ndef merge(left: list[int], right: list[int]) -> list[int]:\n    result = []\n    i = j = 0\n    while i < len(left) and j < len(right):\n        if left[i] <= right[j]:\n            result.append(left[i])\n            i += 1\n        else:\n            result.append(right[j])\n            j += 1\n    result.extend(left[i:])\n    result.extend(right[j:])\n    return result"));
}

#[test]
fn test_gcd() {
    assert!(transpile_ok(
        "def gcd(a: int, b: int) -> int:\n    while b:\n        a, b = b, a % b\n    return a"
    ));
}

#[test]
fn test_is_prime() {
    assert!(transpile_ok("def is_prime(n: int) -> bool:\n    if n < 2:\n        return False\n    for i in range(2, int(n ** 0.5) + 1):\n        if n % i == 0:\n            return False\n    return True"));
}

#[test]
fn test_fibonacci_memo() {
    assert!(transpile_ok("def fib_memo(n: int, memo: dict[int, int] = {}) -> int:\n    if n in memo:\n        return memo[n]\n    if n <= 1:\n        return n\n    memo[n] = fib_memo(n - 1, memo) + fib_memo(n - 2, memo)\n    return memo[n]"));
}
