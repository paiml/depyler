"""Numerical methods: Matrix operations and linear algebra primitives.

Tests: nested loop patterns, index arithmetic, accumulator patterns,
swap operations, determinant recursion.
"""

from typing import List, Tuple


def matrix_multiply_2x2(a00: float, a01: float, a10: float, a11: float,
                        b00: float, b01: float, b10: float, b11: float) -> Tuple[float, float, float, float]:
    """Multiply two 2x2 matrices represented as individual elements."""
    c00: float = a00 * b00 + a01 * b10
    c01: float = a00 * b01 + a01 * b11
    c10: float = a10 * b00 + a11 * b10
    c11: float = a10 * b01 + a11 * b11
    return (c00, c01, c10, c11)


def determinant_2x2(a: float, b: float, c: float, d: float) -> float:
    """Compute determinant of 2x2 matrix [[a,b],[c,d]]."""
    return a * d - b * c


def determinant_3x3(a: float, b: float, c: float,
                    d: float, e: float, f: float,
                    g: float, h: float, i: float) -> float:
    """Compute determinant of 3x3 matrix by cofactor expansion."""
    return a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)


def dot_product(a: List[float], b: List[float]) -> float:
    """Compute dot product of two vectors."""
    result: float = 0.0
    n: int = len(a)
    if n > len(b):
        n = len(b)
    i: int = 0
    while i < n:
        result = result + a[i] * b[i]
        i += 1
    return result


def vector_norm(v: List[float]) -> float:
    """Compute L2 norm of a vector."""
    sum_sq: float = 0.0
    for x in v:
        sum_sq = sum_sq + x * x
    guess: float = sum_sq / 2.0
    if sum_sq == 0.0:
        return 0.0
    iterations: int = 0
    while iterations < 100:
        new_guess: float = (guess + sum_sq / guess) / 2.0
        diff: float = new_guess - guess
        if diff < 0.0:
            diff = -diff
        if diff < 0.00001:
            return new_guess
        guess = new_guess
        iterations += 1
    return guess


def vector_add(a: List[float], b: List[float]) -> List[float]:
    """Add two vectors element-wise."""
    result: List[float] = []
    n: int = len(a)
    if n > len(b):
        n = len(b)
    i: int = 0
    while i < n:
        result.append(a[i] + b[i])
        i += 1
    return result


def vector_scale(v: List[float], scalar: float) -> List[float]:
    """Scale a vector by a scalar."""
    result: List[float] = []
    for x in v:
        result.append(x * scalar)
    return result


def cross_product_3d(a: List[float], b: List[float]) -> List[float]:
    """Compute 3D cross product."""
    result: List[float] = []
    result.append(a[1] * b[2] - a[2] * b[1])
    result.append(a[2] * b[0] - a[0] * b[2])
    result.append(a[0] * b[1] - a[1] * b[0])
    return result


def test_matrix_ops() -> bool:
    """Test matrix and vector operations."""
    ok: bool = True
    det: float = determinant_2x2(1.0, 2.0, 3.0, 4.0)
    diff: float = det - (-2.0)
    if diff < 0.0:
        diff = -diff
    if diff > 0.001:
        ok = False
    d: float = dot_product([1.0, 2.0, 3.0], [4.0, 5.0, 6.0])
    diff2: float = d - 32.0
    if diff2 < 0.0:
        diff2 = -diff2
    if diff2 > 0.001:
        ok = False
    return ok
