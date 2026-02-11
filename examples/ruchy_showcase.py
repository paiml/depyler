#!/usr/bin/env python3
"""
Showcase of Python features that transpile to Ruchy script format.
This demonstrates the Pythonic to functional transformation capabilities.
"""

from typing import List


def fibonacci(n: int) -> int:
    """Calculate fibonacci number recursively."""
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)


def process_data(numbers: List[int]) -> List[int]:
    """Process data using functional pipeline style."""
    result = [x * 2 for x in numbers if x > 0]
    return result


def greet(name: str) -> str:
    """Create a greeting."""
    return "Hello, " + name + "!"


def greet_with_title(name: str, title: str) -> str:
    """Create a greeting with title."""
    return "Hello, " + title + " " + name + "!"


def process_text(text: str) -> str:
    """Process text data."""
    return text.upper()


def test_module() -> int:
    """Main entry point."""
    passed: int = 0

    # Test fibonacci
    if fibonacci(10) == 55:
        passed = passed + 1

    # Test data processing
    numbers: List[int] = [1, -2, 3, -4, 5]
    processed: List[int] = process_data(numbers)
    if len(processed) == 3:
        passed = passed + 1

    # Test greeting
    g1: str = greet("Alice")
    if len(g1) > 0:
        passed = passed + 1

    g2: str = greet_with_title("Bob", "Dr.")
    if len(g2) > 0:
        passed = passed + 1

    # Test process_text
    t: str = process_text("hello")
    if t == "HELLO":
        passed = passed + 1

    return passed
