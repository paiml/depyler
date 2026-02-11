"""
Comprehensive showcase of Depyler's type inference capabilities.
This example demonstrates how Depyler handles typed functions.
"""

from typing import List


def numeric_operations(x: int, y: int) -> int:
    """Numeric operations with typed parameters."""
    sum_val: int = x + y
    product: int = x * y

    if x > y:
        return sum_val
    else:
        return product


def string_manipulation(text: str) -> str:
    """String type operations."""
    upper_text: str = text.upper()

    if text.startswith("Hello"):
        return text.replace("Hello", "Hi")

    return text.strip()


def list_sum(items: List[int]) -> int:
    """Sum all items in a list."""
    total: int = 0
    for item in items:
        total = total + item
    return total


def mixed_inference(data: List[int], multiplier: int) -> int:
    """Multiple typed parameters for computation."""
    total: int = 0
    for value in data:
        total = total + value * multiplier
    if len(data) == 0:
        return 0
    return total // len(data)


def boolean_logic(a: int, b: int, c: int) -> int:
    """Boolean operations returning int codes."""
    if a > 0 and b > 0:
        return 1
    if b > 0 or c > 0:
        return 2
    if c <= 0:
        return 3
    return 0


def confidence_levels_demo(certain_str: str, probable_num: int, container_size: int) -> int:
    """Demonstrates typed parameters at different levels."""
    processed: str = certain_str.upper()
    doubled: int = probable_num * 2
    result: int = len(processed) + doubled + container_size
    return result


def test_module() -> int:
    """Run all tests."""
    passed: int = 0

    # Test numeric_operations
    if numeric_operations(5, 3) == 8:
        passed = passed + 1
    if numeric_operations(3, 5) == 15:
        passed = passed + 1

    # Test string_manipulation
    if string_manipulation("Hello World") == "Hi World":
        passed = passed + 1
    if string_manipulation("  test  ") == "test":
        passed = passed + 1

    # Test list_sum
    if list_sum([1, 2, 3, 4, 5]) == 15:
        passed = passed + 1
    if list_sum([]) == 0:
        passed = passed + 1

    # Test mixed_inference
    if mixed_inference([10, 20, 30], 2) == 40:
        passed = passed + 1

    # Test boolean_logic
    if boolean_logic(1, 1, 0) == 1:
        passed = passed + 1
    if boolean_logic(0, 1, 0) == 2:
        passed = passed + 1
    if boolean_logic(0, 0, 0) == 3:
        passed = passed + 1

    # Test confidence_levels_demo
    r: int = confidence_levels_demo("hi", 5, 3)
    if r == 15:
        passed = passed + 1

    return passed
