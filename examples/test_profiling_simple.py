#!/usr/bin/env python3
"""Simple performance profiling demo for Depyler."""


def fibonacci_recursive(n: int) -> int:
    """Recursive Fibonacci - will be identified as hot path."""
    if n <= 1:
        return n
    return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)


def process_list(items: list[int]) -> int:
    """Process a list with nested loops - O(n²) complexity."""
    total = 0
    
    # Nested loop pattern that profiler will flag
    for i in items:
        for j in items:
            if i < j:
                total = total + (i * j)
    
    return total


def string_concatenation_in_loop(n: int) -> str:
    """String concatenation in loop - inefficient pattern."""
    result = ""
    for i in range(n):
        # O(n²) due to string immutability
        result = result + str(i)
        result = result + ", "
    return result


def type_check_heavy(values: list[object]) -> int:
    """Function with many type checks that Rust can optimize away."""
    count = 0
    for value in values:
        if isinstance(value, int):
            count = count + value
        elif isinstance(value, str):
            count = count + len(value)
    return count


def simple_function(x: int, y: int) -> int:
    """Simple function for baseline comparison."""
    return x + y


def hot_loop():
    """Function with hot nested loops."""
    total = 0
    for i in range(100):
        for j in range(100):
            total = total + (i * j)
    return total