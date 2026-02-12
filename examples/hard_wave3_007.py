"""Numerical methods: Interpolation and curve fitting.

Tests: polynomial evaluation, coefficient computation, Lagrange basis,
divided differences, Horner's method.
"""

from typing import List, Tuple


def horner_eval(coeffs: List[float], x: float) -> float:
    """Evaluate polynomial using Horner's method. coeffs[0] is highest degree."""
    result: float = 0.0
    for c in coeffs:
        result = result * x + c
    return result


def lagrange_interpolate(xs: List[float], ys: List[float], x: float) -> float:
    """Lagrange interpolation at point x given data points."""
    n: int = len(xs)
    result: float = 0.0
    i: int = 0
    while i < n:
        basis: float = 1.0
        j: int = 0
        while j < n:
            if i != j:
                denom: float = xs[i] - xs[j]
                if denom != 0.0:
                    basis = basis * (x - xs[j]) / denom
            j += 1
        result = result + ys[i] * basis
        i += 1
    return result


def divided_differences(xs: List[float], ys: List[float]) -> List[float]:
    """Compute Newton's divided differences for interpolation."""
    n: int = len(xs)
    dd: List[float] = []
    for y in ys:
        dd.append(y)
    j: int = 1
    while j < n:
        i: int = n - 1
        while i >= j:
            denom: float = xs[i] - xs[i - j]
            if denom != 0.0:
                dd[i] = (dd[i] - dd[i - 1]) / denom
            i -= 1
        j += 1
    return dd


def newton_interpolate(xs: List[float], dd: List[float], x: float) -> float:
    """Evaluate Newton interpolation polynomial at x."""
    n: int = len(dd)
    result: float = dd[n - 1]
    i: int = n - 2
    while i >= 0:
        result = result * (x - xs[i]) + dd[i]
        i -= 1
    return result


def linear_regression_slope(xs: List[float], ys: List[float]) -> float:
    """Compute slope of best-fit line using least squares."""
    n: int = len(xs)
    if n == 0:
        return 0.0
    sum_x: float = 0.0
    sum_y: float = 0.0
    sum_xy: float = 0.0
    sum_x2: float = 0.0
    i: int = 0
    while i < n:
        sum_x = sum_x + xs[i]
        sum_y = sum_y + ys[i]
        sum_xy = sum_xy + xs[i] * ys[i]
        sum_x2 = sum_x2 + xs[i] * xs[i]
        i += 1
    denom: float = float(n) * sum_x2 - sum_x * sum_x
    if denom == 0.0:
        return 0.0
    return (float(n) * sum_xy - sum_x * sum_y) / denom


def linear_regression_intercept(xs: List[float], ys: List[float]) -> float:
    """Compute intercept of best-fit line."""
    n: int = len(xs)
    if n == 0:
        return 0.0
    slope: float = linear_regression_slope(xs, ys)
    sum_x: float = 0.0
    sum_y: float = 0.0
    i: int = 0
    while i < n:
        sum_x = sum_x + xs[i]
        sum_y = sum_y + ys[i]
        i += 1
    return (sum_y - slope * sum_x) / float(n)


def polynomial_derivative(coeffs: List[float]) -> List[float]:
    """Compute derivative of polynomial. coeffs[0] is constant term."""
    result: List[float] = []
    i: int = 1
    while i < len(coeffs):
        result.append(coeffs[i] * float(i))
        i += 1
    return result


def test_interpolation() -> bool:
    """Test interpolation methods."""
    ok: bool = True
    val: float = horner_eval([1.0, 0.0, 0.0], 3.0)
    diff: float = val - 9.0
    if diff < 0.0:
        diff = -diff
    if diff > 0.001:
        ok = False
    slope: float = linear_regression_slope([1.0, 2.0, 3.0], [2.0, 4.0, 6.0])
    diff2: float = slope - 2.0
    if diff2 < 0.0:
        diff2 = -diff2
    if diff2 > 0.01:
        ok = False
    return ok
