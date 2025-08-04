#!/usr/bin/env python3
"""Performance profiling demo for Depyler.

This example demonstrates various performance patterns that the profiler
can detect and analyze.
"""


def fibonacci_recursive(n: int) -> int:
    """Recursive Fibonacci - will be identified as hot path."""
    if n <= 1:
        return n
    return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)


def fibonacci_iterative(n: int) -> int:
    """Iterative Fibonacci - more efficient."""
    if n <= 1:
        return n
    
    a, b = 0, 1
    for _ in range(2, n + 1):
        a, b = b, a + b
    return b


def process_list(items: list[int]) -> int:
    """Process a list with nested loops - O(n²) complexity."""
    total = 0
    
    # Nested loop pattern that profiler will flag
    for i in range(len(items)):
        for j in range(i, len(items)):
            if items[i] < items[j]:
                total += items[i] * items[j]
    
    return total


def string_concatenation_in_loop(n: int) -> str:
    """String concatenation in loop - inefficient pattern."""
    result = ""
    for i in range(n):
        result += f"Item {i}, "  # O(n²) due to string immutability
    return result


def allocate_many_lists(n: int) -> list[list[int]]:
    """Function with many allocations."""
    results = []
    for i in range(n):
        # Each iteration allocates a new list
        inner_list = []
        for j in range(10):
            inner_list.append(i * j)
        results.append(inner_list)
    return results


def type_check_heavy(values: list[object]) -> int:
    """Function with many type checks that Rust can optimize away."""
    count = 0
    for value in values:
        if isinstance(value, int):
            count += value
        elif isinstance(value, str):
            count += len(value)
        elif isinstance(value, list):
            count += len(value)
    return count


def matrix_multiply(a: list[list[float]], b: list[list[float]]) -> list[list[float]]:
    """Matrix multiplication - triple nested loop."""
    rows_a = len(a)
    cols_a = len(a[0]) if a else 0
    cols_b = len(b[0]) if b else 0
    
    # Initialize result matrix
    result = [[0.0 for _ in range(cols_b)] for _ in range(rows_a)]
    
    # Triple nested loop - O(n³)
    for i in range(rows_a):
        for j in range(cols_b):
            for k in range(cols_a):
                result[i][j] += a[i][k] * b[k][j]
    
    return result


def simple_function(x: int, y: int) -> int:
    """Simple function for baseline comparison."""
    return x + y


def main():
    """Main entry point with various function calls."""
    # Hot path: recursive fibonacci
    for i in range(5):
        fibonacci_recursive(i)
    
    # More efficient version
    fibonacci_iterative(30)
    
    # Nested loops
    test_list = list(range(100))
    process_list(test_list)
    
    # String concatenation
    string_concatenation_in_loop(100)
    
    # Many allocations
    allocate_many_lists(50)
    
    # Type checks
    mixed_values = [1, "hello", [1, 2, 3], 42, "world"]
    type_check_heavy(mixed_values)
    
    # Matrix multiplication
    mat_a = [[1.0, 2.0], [3.0, 4.0]]
    mat_b = [[5.0, 6.0], [7.0, 8.0]]
    matrix_multiply(mat_a, mat_b)
    
    # Simple function
    simple_function(10, 20)


if __name__ == "__main__":
    main()