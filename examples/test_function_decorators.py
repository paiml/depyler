"""Function composition patterns without decorators.

Tests the algorithmic intent of decorators (wrapping, chaining, repeating)
using plain function calls that the transpiler supports.
"""


def slow_function(n: int) -> int:
    """A function that sums integers up to n."""
    total: int = 0
    i: int = 0
    while i < n:
        total += i
        i = i + 1
    return total


def important_calculation(x: int, y: int) -> int:
    """A function performing a calculation."""
    return x * y + x + y


def timed_slow_function(n: int) -> int:
    """Simulate timing decorator by wrapping slow_function."""
    result: int = slow_function(n)
    return result


def logged_timed_calculation(x: int, y: int) -> int:
    """Simulate stacked logging and timing decorators."""
    result: int = important_calculation(x, y)
    return result


def repeat_greet(name: str, times: int) -> str:
    """Simulate repeat decorator by calling function multiple times."""
    result: str = ""
    i: int = 0
    while i < times:
        result = "Hello, " + name + "!"
        i = i + 1
    return result


def test_decorators() -> int:
    """Test decorator-simulated functions."""
    result1: int = timed_slow_function(100)
    result2: int = logged_timed_calculation(5, 10)
    result3: str = repeat_greet("World", 3)
    total: int = result1 + result2
    return total
