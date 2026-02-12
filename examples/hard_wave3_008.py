"""Numerical methods: Fixed-point iteration and convergence analysis.

Tests: iterative refinement, convergence detection, error tracking,
relaxation parameters, orbit computation.
"""

from typing import List, Tuple


def fixed_point_cos(x0: float, tol: float, max_iter: int) -> float:
    """Find fixed point of cos(x) = x using Taylor approx for cos."""
    x: float = x0
    iterations: int = 0
    while iterations < max_iter:
        cos_x: float = 1.0 - x * x / 2.0 + x * x * x * x / 24.0
        diff: float = cos_x - x
        if diff < 0.0:
            diff = -diff
        if diff < tol:
            return cos_x
        x = cos_x
        iterations += 1
    return x


def successive_overrelaxation(x0: float, omega: float, tol: float, max_iter: int) -> float:
    """SOR iteration for x = cos(x) with relaxation parameter omega."""
    x: float = x0
    iterations: int = 0
    while iterations < max_iter:
        cos_x: float = 1.0 - x * x / 2.0 + x * x * x * x / 24.0
        new_x: float = (1.0 - omega) * x + omega * cos_x
        diff: float = new_x - x
        if diff < 0.0:
            diff = -diff
        if diff < tol:
            return new_x
        x = new_x
        iterations += 1
    return x


def aitken_accelerate(x0: float, x1: float, x2: float) -> float:
    """Aitken delta-squared acceleration for convergence speedup."""
    denom: float = x2 - 2.0 * x1 + x0
    if denom == 0.0:
        return x2
    return x0 - (x1 - x0) * (x1 - x0) / denom


def convergence_rate(errors: List[float]) -> float:
    """Estimate convergence rate from sequence of errors."""
    n: int = len(errors)
    if n < 3:
        return 0.0
    rates: List[float] = []
    i: int = 2
    while i < n:
        if errors[i - 1] != 0.0 and errors[i - 2] != 0.0:
            r: float = errors[i] / errors[i - 1]
            rates.append(r)
        i += 1
    if len(rates) == 0:
        return 0.0
    total: float = 0.0
    for r in rates:
        total = total + r
    return total / float(len(rates))


def babylonian_sqrt(x: float, iterations: int) -> float:
    """Babylonian method (same as Newton) with fixed iteration count."""
    if x <= 0.0:
        return 0.0
    guess: float = x
    i: int = 0
    while i < iterations:
        guess = (guess + x / guess) / 2.0
        i += 1
    return guess


def secant_method(x0: float, x1: float, tol: float, max_iter: int) -> float:
    """Secant method for finding root of f(x) = x^2 - 2."""
    iterations: int = 0
    while iterations < max_iter:
        f0: float = x0 * x0 - 2.0
        f1: float = x1 * x1 - 2.0
        denom: float = f1 - f0
        if denom == 0.0:
            return x1
        x2: float = x1 - f1 * (x1 - x0) / denom
        diff: float = x2 - x1
        if diff < 0.0:
            diff = -diff
        if diff < tol:
            return x2
        x0 = x1
        x1 = x2
        iterations += 1
    return x1


def regula_falsi(lo: float, hi: float, tol: float, max_iter: int) -> float:
    """Regula falsi (false position) for x^2 - 2 = 0."""
    iterations: int = 0
    while iterations < max_iter:
        f_lo: float = lo * lo - 2.0
        f_hi: float = hi * hi - 2.0
        denom: float = f_hi - f_lo
        if denom == 0.0:
            return lo
        mid: float = lo - f_lo * (hi - lo) / denom
        f_mid: float = mid * mid - 2.0
        if f_mid < 0.0:
            f_mid_abs: float = -f_mid
        else:
            f_mid_abs = f_mid
        if f_mid_abs < tol:
            return mid
        if f_lo * f_mid < 0.0:
            hi = mid
        else:
            lo = mid
        iterations += 1
    return (lo + hi) / 2.0


def test_fixed_point() -> bool:
    """Test fixed-point and convergence methods."""
    ok: bool = True
    sq2: float = secant_method(1.0, 2.0, 0.0001, 100)
    diff: float = sq2 - 1.4142
    if diff < 0.0:
        diff = -diff
    if diff > 0.01:
        ok = False
    bab: float = babylonian_sqrt(9.0, 20)
    diff2: float = bab - 3.0
    if diff2 < 0.0:
        diff2 = -diff2
    if diff2 > 0.001:
        ok = False
    return ok
