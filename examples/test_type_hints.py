"""
Example demonstrating type inference hints in Depyler.
Functions with type annotations for proper transpilation.
"""

from typing import List


def process_numbers(data: List[int]) -> int:
    """Process a list of numbers - return sum."""
    total: int = 0
    for num in data:
        total = total + num
    return total


def manipulate_text(text: str) -> str:
    """Various string operations."""
    result: str = text.upper()
    if result.startswith("HELLO"):
        result = result.replace("HELLO", "HI")
    return result.strip()


def mixed_operations(x: int, y: int) -> int:
    """Mixed numeric operations."""
    sum_val: int = x + y
    product: int = x * y

    if x > y:
        return sum_val
    else:
        return product


def container_first(items: List[int]) -> int:
    """Return first element or -1 for empty list."""
    if len(items) == 0:
        return -1
    first: int = items[0]
    return first


def inferred_return_types() -> int:
    """Function with inferable return type."""
    x: int = 10
    y: int = 20
    return x + y


def string_formatting(name: str, age: int) -> str:
    """String formatting with mixed types."""
    formatted_name: str = name.upper()
    next_age: int = age + 1
    return formatted_name + " will be " + str(next_age) + " next year"


def type_conversions(value: int) -> int:
    """Type conversion to int."""
    number: int = value
    return number


def partial_annotations_sum(data: List[int], multiplier: int) -> int:
    """Sum of items times multiplier."""
    result: int = 0
    for item in data:
        result = result + item * multiplier
    return result


def test_module() -> int:
    """Run all tests."""
    passed: int = 0

    # Test process_numbers
    nums: List[int] = [1, 2, 3, 4, 5]
    if process_numbers(nums) == 15:
        passed = passed + 1

    # Test manipulate_text
    if manipulate_text("hello") == "HELLO":
        passed = passed + 1

    # Test mixed_operations
    if mixed_operations(5, 3) == 8:
        passed = passed + 1
    if mixed_operations(3, 5) == 15:
        passed = passed + 1

    # Test container_first
    if container_first([10, 20, 30]) == 10:
        passed = passed + 1
    if container_first([]) == -1:
        passed = passed + 1

    # Test inferred_return_types
    if inferred_return_types() == 30:
        passed = passed + 1

    # Test type_conversions
    if type_conversions(42) == 42:
        passed = passed + 1

    # Test partial_annotations_sum
    if partial_annotations_sum([1, 2, 3], 2) == 12:
        passed = passed + 1

    return passed
