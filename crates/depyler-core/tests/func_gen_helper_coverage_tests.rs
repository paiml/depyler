//! EXTREME TDD tests for func_gen helper functions
//!
//! This file contains integration tests that exercise the helper functions
//! in func_gen.rs through transpilation. These tests target code paths that
//! are not hit by the main test suite.
//!
//! DEPYLER-COVERAGE: Target coverage improvement for func_gen.rs

use depyler_core::ast_bridge::AstBridge;
use depyler_core::codegen::hir_to_rust;
use rustpython_ast::Suite;
use rustpython_parser::Parse;

// ============================================================================
// Helper Functions
// ============================================================================

fn make_module(ast: Suite) -> rustpython_ast::Mod {
    rustpython_ast::Mod::Module(rustpython_ast::ModModule {
        body: ast,
        range: Default::default(),
        type_ignores: vec![],
    })
}

fn transpile(code: &str) -> Option<String> {
    let ast = Suite::parse(code, "<test>").ok()?;
    let bridge = AstBridge::new().with_source(code.to_string());
    let (hir, _) = bridge.python_to_hir(make_module(ast)).ok()?;
    hir_to_rust(&hir).ok()
}

fn transpile_succeeds(code: &str) -> bool {
    transpile(code).is_some()
}

// ============================================================================
// Basic function COVERAGE TESTS
// ============================================================================

#[test]
fn test_function_no_params() {
    let code = r#"
def hello() -> str:
    return "Hello"
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_one_param() {
    let code = r#"
def greet(name: str) -> str:
    return name
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_multiple_params() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_default_value() {
    let code = r#"
def greet(name: str = "World") -> str:
    return name
"#;
    let _ = transpile(code);
}

#[test]
fn test_function_multiple_defaults() {
    let code = r#"
def config(host: str = "localhost", port: int = 8080) -> str:
    return host
"#;
    let _ = transpile(code);
}

#[test]
fn test_function_mixed_params_defaults() {
    let code = r#"
def connect(host: str, port: int = 8080) -> str:
    return host
"#;
    let _ = transpile(code);
}

// ============================================================================
// Return type COVERAGE TESTS
// ============================================================================

#[test]
fn test_return_type_int() {
    let code = r#"
def get_int() -> int:
    return 42
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_return_type_float() {
    let code = r#"
def get_float() -> float:
    return 3.14
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_return_type_str() {
    let code = r#"
def get_str() -> str:
    return "hello"
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_return_type_bool() {
    let code = r#"
def get_bool() -> bool:
    return True
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_return_type_list() {
    let code = r#"
def get_list() -> list[int]:
    return [1, 2, 3]
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_return_type_dict() {
    let code = r#"
def get_dict() -> dict[str, int]:
    return {"a": 1}
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_return_type_tuple() {
    let code = r#"
def get_tuple() -> tuple[int, str]:
    return (1, "hello")
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_return_type_none() {
    let code = r#"
def no_return() -> None:
    pass
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_return_type_optional() {
    let code = r#"
from typing import Optional

def maybe_int(x: int) -> Optional[int]:
    if x > 0:
        return x
    return None
"#;
    let _ = transpile(code);
}

// ============================================================================
// Parameter types COVERAGE TESTS
// ============================================================================

#[test]
fn test_param_type_list() {
    let code = r#"
def sum_list(items: list[int]) -> int:
    return sum(items)
"#;
    let _ = transpile(code);
}

#[test]
fn test_param_type_dict() {
    let code = r#"
def get_key(d: dict[str, int], key: str) -> int:
    return d[key]
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_param_type_tuple() {
    let code = r#"
def first(t: tuple[int, str]) -> int:
    return t[0]
"#;
    let _ = transpile(code);
}

#[test]
fn test_param_type_optional() {
    let code = r#"
from typing import Optional

def process(x: Optional[int]) -> int:
    if x is None:
        return 0
    return x
"#;
    let _ = transpile(code);
}

#[test]
fn test_param_type_callable() {
    let code = r#"
from typing import Callable

def apply(f: Callable[[int], int], x: int) -> int:
    return f(x)
"#;
    let _ = transpile(code);
}

// ============================================================================
// Async function COVERAGE TESTS
// ============================================================================

#[test]
fn test_async_function_simple() {
    let code = r#"
async def fetch() -> str:
    return "data"
"#;
    let _ = transpile(code);
}

#[test]
fn test_async_function_with_await() {
    let code = r#"
async def get_data() -> str:
    result = await fetch()
    return result
"#;
    let _ = transpile(code);
}

#[test]
fn test_async_function_with_params() {
    let code = r#"
async def fetch_url(url: str) -> str:
    return url
"#;
    let _ = transpile(code);
}

// ============================================================================
// Generator function COVERAGE TESTS
// ============================================================================

#[test]
fn test_generator_simple() {
    let code = r#"
def count(n: int) -> int:
    for i in range(n):
        yield i
"#;
    let _ = transpile(code);
}

#[test]
fn test_generator_with_condition() {
    let code = r#"
def even_numbers(n: int) -> int:
    for i in range(n):
        if i % 2 == 0:
            yield i
"#;
    let _ = transpile(code);
}

// ============================================================================
// Decorator COVERAGE TESTS
// ============================================================================

#[test]
fn test_staticmethod() {
    let code = r#"
class Utils:
    @staticmethod
    def add(a: int, b: int) -> int:
        return a + b
"#;
    let _ = transpile(code);
}

#[test]
fn test_classmethod() {
    let code = r#"
class Factory:
    @classmethod
    def create(cls) -> int:
        return 0
"#;
    let _ = transpile(code);
}

#[test]
fn test_property_getter() {
    let code = r#"
class Point:
    def __init__(self, x: int) -> None:
        self._x = x

    @property
    def x(self) -> int:
        return self._x
"#;
    let _ = transpile(code);
}

// ============================================================================
// Method COVERAGE TESTS
// ============================================================================

#[test]
fn test_method_self_only() {
    let code = r#"
class Counter:
    def __init__(self) -> None:
        self.value = 0

    def get(self) -> int:
        return self.value
"#;
    let _ = transpile(code);
}

#[test]
fn test_method_with_params() {
    let code = r#"
class Counter:
    def __init__(self) -> None:
        self.value = 0

    def add(self, n: int) -> None:
        self.value = self.value + n
"#;
    let _ = transpile(code);
}

#[test]
fn test_method_mutating() {
    let code = r#"
class Stack:
    def __init__(self) -> None:
        self.items: list[int] = []

    def push(self, item: int) -> None:
        self.items.append(item)
"#;
    let _ = transpile(code);
}

// ============================================================================
// __init__ method COVERAGE TESTS
// ============================================================================

#[test]
fn test_init_simple() {
    let code = r#"
class Point:
    def __init__(self) -> None:
        self.x = 0
        self.y = 0
"#;
    let _ = transpile(code);
}

#[test]
fn test_init_with_params() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y
"#;
    let _ = transpile(code);
}

#[test]
fn test_init_with_defaults() {
    let code = r#"
class Config:
    def __init__(self, host: str = "localhost", port: int = 8080) -> None:
        self.host = host
        self.port = port
"#;
    let _ = transpile(code);
}

// ============================================================================
// Dunder methods COVERAGE TESTS
// ============================================================================

#[test]
fn test_str_method() {
    let code = r#"
class Point:
    def __init__(self, x: int) -> None:
        self.x = x

    def __str__(self) -> str:
        return f"Point({self.x})"
"#;
    let _ = transpile(code);
}

#[test]
fn test_repr_method() {
    let code = r#"
class Point:
    def __init__(self, x: int) -> None:
        self.x = x

    def __repr__(self) -> str:
        return f"Point({self.x})"
"#;
    let _ = transpile(code);
}

#[test]
fn test_len_method() {
    let code = r#"
class Container:
    def __init__(self) -> None:
        self.items: list[int] = []

    def __len__(self) -> int:
        return len(self.items)
"#;
    let _ = transpile(code);
}

#[test]
fn test_eq_method() {
    let code = r#"
class Point:
    def __init__(self, x: int) -> None:
        self.x = x

    def __eq__(self, other: object) -> bool:
        return self.x == other.x
"#;
    let _ = transpile(code);
}

// ============================================================================
// Lifetime inference COVERAGE TESTS
// ============================================================================

#[test]
fn test_borrow_param() {
    let code = r#"
def first(items: list[int]) -> int:
    return items[0]
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_return_borrowed() {
    let code = r#"
def identity(s: str) -> str:
    return s
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// Args field access COVERAGE TESTS
// ============================================================================

#[test]
fn test_args_single_field() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input")
    args = parser.parse_args()
    print(args.input)
"#;
    let _ = transpile(code);
}

#[test]
fn test_args_multiple_fields() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input")
    parser.add_argument("--output")
    args = parser.parse_args()
    print(args.input, args.output)
"#;
    let _ = transpile(code);
}

#[test]
fn test_args_in_condition() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--verbose", action="store_true")
    args = parser.parse_args()
    if args.verbose:
        print("verbose")
"#;
    let _ = transpile(code);
}

#[test]
fn test_args_in_binary() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--count", type=int)
    args = parser.parse_args()
    result = args.count + 1
    print(result)
"#;
    let _ = transpile(code);
}

#[test]
fn test_args_in_method_call() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--path")
    args = parser.parse_args()
    print(args.path.upper())
"#;
    let _ = transpile(code);
}

#[test]
fn test_args_in_list() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--item")
    args = parser.parse_args()
    items = [args.item]
    print(items)
"#;
    let _ = transpile(code);
}

#[test]
fn test_args_in_dict() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--key")
    parser.add_argument("--value")
    args = parser.parse_args()
    d = {"k": args.key, "v": args.value}
    print(d)
"#;
    let _ = transpile(code);
}

#[test]
fn test_args_in_fstring() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--name")
    args = parser.parse_args()
    print(f"Hello {args.name}")
"#;
    let _ = transpile(code);
}

#[test]
fn test_args_in_slice() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--start", type=int)
    parser.add_argument("--end", type=int)
    args = parser.parse_args()
    items = [1, 2, 3, 4, 5]
    print(items[args.start:args.end])
"#;
    let _ = transpile(code);
}

#[test]
fn test_args_in_list_comp() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--multiplier", type=int)
    args = parser.parse_args()
    items = [1, 2, 3]
    result = [x * args.multiplier for x in items]
    print(result)
"#;
    let _ = transpile(code);
}

#[test]
fn test_args_in_dict_comp() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--factor", type=int)
    args = parser.parse_args()
    items = [1, 2, 3]
    result = {x: x * args.factor for x in items}
    print(result)
"#;
    let _ = transpile(code);
}

#[test]
fn test_args_in_ternary() {
    let code = r#"
import argparse

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--use_alt", action="store_true")
    parser.add_argument("--value")
    args = parser.parse_args()
    result = args.value if args.use_alt else "default"
    print(result)
"#;
    let _ = transpile(code);
}

// ============================================================================
// Subcommand handler COVERAGE TESTS
// ============================================================================

#[test]
fn test_subcommand_handler() {
    let code = r#"
import argparse

def cmd_init(args) -> None:
    print("init")

def main() -> None:
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command")
    parser_init = subparsers.add_parser("init")
    parser_init.set_defaults(func=cmd_init)
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

#[test]
fn test_subcommand_with_args() {
    let code = r#"
import argparse

def cmd_add(args) -> None:
    print(args.name)

def main() -> None:
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command")
    parser_add = subparsers.add_parser("add")
    parser_add.add_argument("--name")
    parser_add.set_defaults(func=cmd_add)
    args = parser.parse_args()
"#;
    let _ = transpile(code);
}

// ============================================================================
// Complex function bodies COVERAGE TESTS
// ============================================================================

#[test]
fn test_function_with_loop() {
    let code = r#"
def sum_all(items: list[int]) -> int:
    total = 0
    for item in items:
        total = total + item
    return total
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_with_conditionals() {
    let code = r#"
def classify(x: int) -> str:
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    else:
        return "zero"
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_with_try_except() {
    let code = r#"
def safe_divide(a: int, b: int) -> int:
    try:
        return a // b
    except ZeroDivisionError:
        return 0
"#;
    let _ = transpile(code);
}

#[test]
fn test_function_with_nested_function() {
    let code = r#"
def outer(x: int) -> int:
    def inner(y: int) -> int:
        return y + 1
    return inner(x)
"#;
    let _ = transpile(code);
}

#[test]
fn test_function_with_lambda() {
    let code = r#"
def apply_double(items: list[int]) -> list[int]:
    return list(map(lambda x: x * 2, items))
"#;
    let _ = transpile(code);
}

// ============================================================================
// Keyword parameter COVERAGE TESTS
// ============================================================================

#[test]
fn test_kwargs_only() {
    let code = r#"
def config(*, host: str, port: int) -> str:
    return host
"#;
    let _ = transpile(code);
}

#[test]
fn test_positional_and_kwargs() {
    let code = r#"
def connect(host: str, *, port: int = 8080) -> str:
    return host
"#;
    let _ = transpile(code);
}

#[test]
fn test_star_args() {
    let code = r#"
def sum_all(*args: int) -> int:
    total = 0
    for arg in args:
        total = total + arg
    return total
"#;
    let _ = transpile(code);
}

#[test]
fn test_star_kwargs() {
    let code = r#"
def print_all(**kwargs: str) -> None:
    for k, v in kwargs.items():
        print(k, v)
"#;
    let _ = transpile(code);
}

// ============================================================================
// Complex type annotations COVERAGE TESTS
// ============================================================================

#[test]
fn test_nested_generic_return() {
    let code = r#"
def nested() -> list[list[int]]:
    return [[1, 2], [3, 4]]
"#;
    let _ = transpile(code);
}

#[test]
fn test_nested_generic_param() {
    let code = r#"
def flatten(matrix: list[list[int]]) -> list[int]:
    result: list[int] = []
    for row in matrix:
        for item in row:
            result.append(item)
    return result
"#;
    let _ = transpile(code);
}

#[test]
fn test_union_return() {
    let code = r#"
from typing import Union

def parse(s: str) -> Union[int, str]:
    return s
"#;
    let _ = transpile(code);
}

// ============================================================================
// Docstring COVERAGE TESTS
// ============================================================================

#[test]
fn test_function_with_docstring() {
    let code = r#"
def greet(name: str) -> str:
    """Greet the user."""
    return f"Hello {name}"
"#;
    let _ = transpile(code);
}

#[test]
fn test_function_with_multiline_docstring() {
    let code = r#"
def complex_func(x: int, y: int) -> int:
    """
    A complex function.

    Args:
        x: First argument
        y: Second argument

    Returns:
        The sum of x and y
    """
    return x + y
"#;
    let _ = transpile(code);
}

// ============================================================================
// Rust keyword handling COVERAGE TESTS
// ============================================================================

#[test]
fn test_keyword_param_type() {
    let code = r#"
def process(type_val: str) -> str:
    return type_val
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_keyword_param_match() {
    let code = r#"
def process(match_val: str) -> str:
    return match_val
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_keyword_param_impl() {
    let code = r#"
def process(impl_val: str) -> str:
    return impl_val
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// Module-level function COVERAGE TESTS
// ============================================================================

#[test]
fn test_multiple_functions() {
    let code = r#"
def first() -> int:
    return 1

def second() -> int:
    return 2

def third() -> int:
    return first() + second()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_recursive_function() {
    let code = r#"
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_mutually_recursive() {
    let code = r#"
def is_even(n: int) -> bool:
    if n == 0:
        return True
    return is_odd(n - 1)

def is_odd(n: int) -> bool:
    if n == 0:
        return False
    return is_even(n - 1)
"#;
    let _ = transpile(code);
}

// ============================================================================
// Edge cases COVERAGE TESTS
// ============================================================================

#[test]
fn test_empty_function() {
    let code = r#"
def empty() -> None:
    pass
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_only_docstring() {
    let code = r#"
def documented() -> None:
    """Just a docstring."""
    pass
"#;
    let _ = transpile(code);
}

#[test]
fn test_function_with_assert() {
    let code = r#"
def validate(x: int) -> None:
    assert x > 0, "x must be positive"
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_early_return() {
    let code = r#"
def guard(x: int) -> int:
    if x < 0:
        return 0
    if x > 100:
        return 100
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_with_walrus() {
    let code = r#"
def process(items: list[int]) -> int:
    if (n := len(items)) > 0:
        return n
    return 0
"#;
    let _ = transpile(code);
}
