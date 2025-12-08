"""Reproduction for integer literal suffix mismatch.

The issue: When calling a function with default int argument,
the literal 100 is generated as 100i64 but the parameter expects i32.

Error: E0308: mismatched types, expected `i32`, found `i64`
"""


def bisection(a: float, b: float, max_iter: int = 100) -> float:
    """Find root using bisection."""
    for _ in range(max_iter):
        a = (a + b) / 2
    return a


def find_roots(a: float, b: float) -> float:
    """Call bisection without explicit max_iter."""
    result = bisection(a, b)  # Uses default, generates 100i64 instead of 100
    return result
