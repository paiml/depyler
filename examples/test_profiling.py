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

    a: int = 0
    b: int = 1
    for _i in range(2, n + 1):
        c: int = a + b
        a = b
        b = c
    return b


def process_list(items: list[int]) -> int:
    """Process a list with nested loops - O(n squared) complexity."""
    total: int = 0
    n: int = len(items)
    i: int = 0
    while i < n:
        j: int = i
        while j < n:
            val_i: int = items[i]
            val_j: int = items[j]
            if val_i < val_j:
                total = total + val_i * val_j
            j = j + 1
        i = i + 1
    return total


def string_concatenation_in_loop(n: int) -> str:
    """String concatenation in loop - inefficient pattern."""
    result: str = ""
    for i in range(n):
        result = result + "Item " + str(i) + ", "
    return result


def allocate_flat_list(n: int) -> list[int]:
    """Function that allocates a flat list of computed values."""
    results: list[int] = []
    for i in range(n):
        for j in range(10):
            results.append(i * j)
    return results


def count_values(values: list[int]) -> int:
    """Count and sum values from a list."""
    count: int = 0
    for value in values:
        count = count + value
    return count


def compute_mat_index(row: int, col: int, stride: int) -> int:
    """Compute flat matrix index."""
    return row * stride + col


def matrix_multiply_flat(a_flat: list[float], b_flat: list[float],
                         rows_a: int, cols_a: int, cols_b: int) -> list[float]:
    """Matrix multiplication using flat arrays."""
    result: list[float] = []
    i: int = 0
    total_size: int = rows_a * cols_b
    while i < total_size:
        result.append(0.0)
        i = i + 1
    ri: int = 0
    while ri < rows_a:
        ci: int = 0
        while ci < cols_b:
            total: float = 0.0
            ki: int = 0
            while ki < cols_a:
                a_idx: int = compute_mat_index(ri, ki, cols_a)
                b_idx: int = compute_mat_index(ki, ci, cols_b)
                a_val: float = a_flat[a_idx]
                b_val: float = b_flat[b_idx]
                total = total + a_val * b_val
                ki = ki + 1
            r_idx: int = compute_mat_index(ri, ci, cols_b)
            result[r_idx] = total
            ci = ci + 1
        ri = ri + 1
    return result


def simple_function(x: int, y: int) -> int:
    """Simple function for baseline comparison."""
    return x + y


def run_profiling() -> int:
    """Main entry point with various function calls."""
    total: int = 0

    for i in range(5):
        total = total + fibonacci_recursive(i)

    total = total + fibonacci_iterative(30)

    test_list: list[int] = []
    li: int = 0
    while li < 20:
        test_list.append(li)
        li = li + 1
    total = total + process_list(test_list)

    msg: str = string_concatenation_in_loop(5)
    total = total + len(msg)

    flat: list[int] = allocate_flat_list(5)
    total = total + len(flat)

    vals: list[int] = [1, 2, 3, 42, 10]
    total = total + count_values(vals)

    a_mat: list[float] = [1.0, 2.0, 3.0, 4.0]
    b_mat: list[float] = [5.0, 6.0, 7.0, 8.0]
    c_mat: list[float] = matrix_multiply_flat(a_mat, b_mat, 2, 2, 2)
    total = total + len(c_mat)

    total = total + simple_function(10, 20)

    return total


if __name__ == "__main__":
    result: int = run_profiling()
    if result > 0:
        pass
