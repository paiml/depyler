"""Numerical methods: Polynomial arithmetic and evaluation.

Tests: coefficient manipulation, synthetic division, polynomial multiplication,
GCD of polynomials, evaluation at multiple points.
"""

from typing import List, Tuple


def poly_add(a: List[float], b: List[float]) -> List[float]:
    """Add two polynomials represented as coefficient lists."""
    na: int = len(a)
    nb: int = len(b)
    n: int = na
    if nb > n:
        n = nb
    result: List[float] = []
    i: int = 0
    while i < n:
        va: float = 0.0
        vb: float = 0.0
        if i < na:
            va = a[i]
        if i < nb:
            vb = b[i]
        result.append(va + vb)
        i += 1
    return result


def poly_multiply(a: List[float], b: List[float]) -> List[float]:
    """Multiply two polynomials."""
    na: int = len(a)
    nb: int = len(b)
    if na == 0 or nb == 0:
        return []
    total_len: int = na + nb
    total_len = total_len - 1
    result: List[float] = []
    i: int = 0
    while i < total_len:
        result.append(0.0)
        i += 1
    i = 0
    while i < na:
        j: int = 0
        while j < nb:
            result[i + j] = result[i + j] + a[i] * b[j]
            j += 1
        i += 1
    return result


def poly_eval(coeffs: List[float], x: float) -> float:
    """Evaluate polynomial at x using Horner's method (coeffs[0] = constant)."""
    n: int = len(coeffs)
    if n == 0:
        return 0.0
    result: float = coeffs[n - 1]
    i: int = n - 2
    while i >= 0:
        result = result * x + coeffs[i]
        i -= 1
    return result


def poly_derivative(coeffs: List[float]) -> List[float]:
    """Compute derivative of polynomial."""
    n: int = len(coeffs)
    if n <= 1:
        return [0.0]
    result: List[float] = []
    i: int = 1
    while i < n:
        result.append(coeffs[i] * float(i))
        i += 1
    return result


def poly_integral(coeffs: List[float]) -> List[float]:
    """Compute indefinite integral (constant = 0)."""
    result: List[float] = [0.0]
    i: int = 0
    while i < len(coeffs):
        result.append(coeffs[i] / float(i + 1))
        i += 1
    return result


def synthetic_division(coeffs: List[float], root: float) -> List[float]:
    """Divide polynomial by (x - root) using synthetic division."""
    n: int = len(coeffs)
    if n <= 1:
        return [0.0]
    result: List[float] = []
    result.append(coeffs[n - 1])
    i: int = n - 2
    while i >= 0:
        val: float = result[len(result) - 1] * root + coeffs[i]
        result.append(val)
        i -= 1
    out_len: int = len(result) - 1
    quotient: List[float] = []
    i = 0
    while i < out_len:
        quotient.append(result[i])
        i += 1
    quotient.reverse()
    return quotient


def poly_scale(coeffs: List[float], scalar: float) -> List[float]:
    """Scale all polynomial coefficients by scalar."""
    result: List[float] = []
    for c in coeffs:
        result.append(c * scalar)
    return result


def test_polynomial() -> bool:
    """Test polynomial operations."""
    ok: bool = True
    prod: List[float] = poly_multiply([1.0, 1.0], [1.0, 1.0])
    if len(prod) != 3:
        ok = False
    val: float = poly_eval([1.0, 2.0, 1.0], 2.0)
    diff: float = val - 9.0
    if diff < 0.0:
        diff = -diff
    if diff > 0.001:
        ok = False
    deriv: List[float] = poly_derivative([3.0, 0.0, 1.0])
    if len(deriv) != 2:
        ok = False
    return ok
