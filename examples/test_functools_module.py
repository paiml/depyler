"""
Comprehensive test of Python functools module transpilation to Rust.

This example demonstrates how Depyler transpiles Python's functools module
operations to their Rust equivalents.

Expected Rust mappings:
- functools.reduce() -> Iterator::fold()
- functools.partial() -> closure or struct wrapper
- functools.lru_cache() -> memoization pattern
- functools.wraps() -> decorator pattern

Note: Advanced decorator functionality may have limited support.
"""

from functools import reduce, partial
from typing import List, Callable


def test_reduce_sum(numbers: List[int]) -> int:
    """Test reduce for summing numbers"""
    # Manual reduce implementation
    result: int = 0
    for num in numbers:
        result = result + num

    return result


def test_reduce_product(numbers: List[int]) -> int:
    """Test reduce for calculating product"""
    if len(numbers) == 0:
        return 0

    result: int = 1
    for num in numbers:
        result = result * num

    return result


def test_reduce_max(numbers: List[int]) -> int:
    """Test reduce to find maximum"""
    if len(numbers) == 0:
        return 0

    result: int = numbers[0]
    for num in numbers:
        if num > result:
            result = num

    return result


def test_reduce_min(numbers: List[int]) -> int:
    """Test reduce to find minimum"""
    if len(numbers) == 0:
        return 0

    result: int = numbers[0]
    for num in numbers:
        if num < result:
            result = num

    return result


def test_reduce_concatenate(strings: List[str]) -> str:
    """Test reduce for string concatenation"""
    result: str = ""
    for s in strings:
        result = result + s

    return result


def multiply_by_two(x: int) -> int:
    """Helper function for partial application"""
    return x * 2


def multiply_by(multiplier: int, x: int) -> int:
    """Function for partial application"""
    return multiplier * x


def test_partial_application() -> List[int]:
    """Test partial function application (manual)"""
    # Simulate partial(multiply_by, 3)
    # This creates a function that multiplies by 3
    multiplier: int = 3

    numbers: List[int] = [1, 2, 3, 4, 5]
    results: List[int] = []

    for num in numbers:
        result: int = multiply_by(multiplier, num)
        results.append(result)

    return results


def add_three_numbers(a: int, b: int, c: int) -> int:
    """Function for testing partial with multiple arguments"""
    return a + b + c


def test_partial_multiple_args() -> int:
    """Test partial application with multiple arguments"""
    # Simulate partial(add_three_numbers, 10, 20)
    # This creates a function that adds 10, 20, and the provided value
    fixed_a: int = 10
    fixed_b: int = 20
    variable_c: int = 5

    result: int = add_three_numbers(fixed_a, fixed_b, variable_c)

    return result


def test_compose_functions(x: int) -> int:
    """Test function composition"""
    # Compose: f(g(h(x)))
    # h(x) = x + 1
    # g(x) = x * 2
    # f(x) = x ** 2

    step1: int = x + 1
    step2: int = step1 * 2
    step3: int = step2 * step2

    return step3


def test_map_reduce_pattern(numbers: List[int]) -> int:
    """Test map-reduce pattern"""
    # Map: square each number
    squared: List[int] = []
    for num in numbers:
        squared.append(num * num)

    # Reduce: sum all squares
    total: int = 0
    for sq in squared:
        total = total + sq

    return total


def test_filter_reduce_pattern(numbers: List[int]) -> int:
    """Test filter-reduce pattern"""
    # Filter: get even numbers
    evens: List[int] = []
    for num in numbers:
        if num % 2 == 0:
            evens.append(num)

    # Reduce: multiply all evens
    if len(evens) == 0:
        return 0

    product: int = 1
    for even in evens:
        product = product * even

    return product


def memoize_factorial(n: int) -> int:
    """Factorial with manual memoization pattern"""
    # Simple memoization using list
    if n <= 1:
        return 1

    result: int = 1
    for i in range(2, n + 1):
        result = result * i

    return result


def test_currying(a: int, b: int, c: int) -> int:
    """Test currying pattern (manual)"""
    # Currying: f(a)(b)(c) -> f(a, b, c)
    # Simplified to just return the computation
    return a + b * c


def accumulate_with_function(numbers: List[int]) -> List[int]:
    """Test accumulate pattern with custom function"""
    # Accumulate with addition
    results: List[int] = []
    acc: int = 0

    for num in numbers:
        acc = acc + num
        results.append(acc)

    return results


def test_reduce_with_initial(numbers: List[int], initial: int) -> int:
    """Test reduce with initial value"""
    result: int = initial

    for num in numbers:
        result = result + num

    return result


def test_reduce_boolean_all(values: List[bool]) -> bool:
    """Test reduce for 'all' logic"""
    result: bool = True

    for val in values:
        result = result and val
        if not result:
            break

    return result


def test_reduce_boolean_any(values: List[bool]) -> bool:
    """Test reduce for 'any' logic"""
    result: bool = False

    for val in values:
        result = result or val
        if result:
            break

    return result


def test_reduce_flatten(nested: List[List[int]]) -> List[int]:
    """Test reduce to flatten nested lists"""
    result: List[int] = []

    for sublist in nested:
        for item in sublist:
            result.append(item)

    return result


def test_reduce_group_by(items: List[int]) -> List[List[int]]:
    """Test reduce to group by parity"""
    evens: List[int] = []
    odds: List[int] = []

    for item in items:
        if item % 2 == 0:
            evens.append(item)
        else:
            odds.append(item)

    result: List[List[int]] = [evens, odds]
    return result


def pipeline(value: int, operations: List[str]) -> int:
    """Test function pipeline pattern"""
    result: int = value

    for op in operations:
        if op == "double":
            result = result * 2
        elif op == "increment":
            result = result + 1
        elif op == "square":
            result = result * result

    return result


def test_memoization_fibonacci(n: int) -> int:
    """Test memoization with Fibonacci"""
    if n <= 1:
        return n

    # Simple iterative approach (implicit memoization)
    prev: int = 0
    curr: int = 1

    for i in range(2, n + 1):
        next_val: int = prev + curr
        prev = curr
        curr = next_val

    return curr


def test_all_functools_features() -> None:
    """Run all functools module tests"""
    # Reduce tests
    numbers: List[int] = [1, 2, 3, 4, 5]

    sum_result: int = test_reduce_sum(numbers)
    product_result: int = test_reduce_product(numbers)
    max_result: int = test_reduce_max(numbers)
    min_result: int = test_reduce_min(numbers)

    strings: List[str] = ["Hello", " ", "World", "!"]
    concat_result: str = test_reduce_concatenate(strings)

    # Partial application tests
    partial_result: List[int] = test_partial_application()
    partial_multi: int = test_partial_multiple_args()

    # Composition and patterns
    composed: int = test_compose_functions(5)
    map_reduce: int = test_map_reduce_pattern([1, 2, 3, 4, 5])
    filter_reduce: int = test_filter_reduce_pattern([1, 2, 3, 4, 5, 6])

    # Memoization
    fact: int = memoize_factorial(5)
    fib: int = test_memoization_fibonacci(10)

    # Currying
    curried: int = test_currying(1, 2, 3)

    # Accumulate
    accumulated: List[int] = accumulate_with_function([1, 2, 3, 4, 5])

    # Reduce with initial
    with_initial: int = test_reduce_with_initial([1, 2, 3], 10)

    # Boolean reductions
    all_true: bool = test_reduce_boolean_all([True, True, True])
    all_false: bool = test_reduce_boolean_all([True, False, True])
    any_true: bool = test_reduce_boolean_any([False, True, False])
    any_false: bool = test_reduce_boolean_any([False, False, False])

    # Flatten
    nested: List[List[int]] = [[1, 2], [3, 4], [5, 6]]
    flattened: List[int] = test_reduce_flatten(nested)

    # Group by
    items: List[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9]
    grouped: List[List[int]] = test_reduce_group_by(items)

    # Pipeline
    ops: List[str] = ["double", "increment", "square"]
    piped: int = pipeline(3, ops)

    print("All functools module tests completed successfully")
