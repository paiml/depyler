#!/usr/bin/env python3
"""Example module for documentation generation.

This module demonstrates various Python features that Depyler
can document, including functions and type annotations.
"""

from typing import List


def fibonacci(n: int) -> int:
    """Calculate the n-th Fibonacci number.

    This function uses an iterative approach for efficiency.
    """
    if n <= 1:
        return n

    a: int = 0
    b: int = 1
    i: int = 2
    while i <= n:
        temp: int = b
        b = a + b
        a = temp
        i = i + 1
    return b


def process_data_count(items: List[int]) -> int:
    """Count items in the list."""
    return len(items)


def process_data_sum(items: List[int]) -> int:
    """Sum items in the list."""
    total: int = 0
    for x in items:
        total = total + x
    return total


def process_data_max(items: List[int]) -> int:
    """Find max in the list or 0 if empty."""
    if len(items) == 0:
        return 0
    best: int = items[0]
    for x in items:
        if x > best:
            best = x
    return best


def process_data_min(items: List[int]) -> int:
    """Find min in the list or 0 if empty."""
    if len(items) == 0:
        return 0
    best: int = items[0]
    for x in items:
        if x < best:
            best = x
    return best


def count_above_threshold(items: List[int], threshold: int) -> int:
    """Count items above threshold."""
    count: int = 0
    for x in items:
        if x > threshold:
            count = count + 1
    return count


def test_module() -> int:
    """Main entry point demonstrating usage."""
    passed: int = 0

    # Test fibonacci
    if fibonacci(0) == 0:
        passed = passed + 1
    if fibonacci(1) == 1:
        passed = passed + 1
    if fibonacci(10) == 55:
        passed = passed + 1

    # Test process_data functions
    data: List[int] = [1, 2, 3, 4, 5]
    if process_data_count(data) == 5:
        passed = passed + 1
    if process_data_sum(data) == 15:
        passed = passed + 1
    if process_data_max(data) == 5:
        passed = passed + 1
    if process_data_min(data) == 1:
        passed = passed + 1

    # Test count above threshold
    if count_above_threshold(data, 3) == 2:
        passed = passed + 1

    return passed
