"""Numerical methods: Bisection method for root finding.

Tests: interval halving, sign detection, convergence criteria,
midpoint computation, bracket tracking.
"""

from typing import List, Tuple


def bisection_sqrt(x: float, tol: float) -> float:
    """Find square root via bisection on f(g) = g*g - x."""
    if x < 0.0:
        return -1.0
    if x == 0.0:
        return 0.0
    lo: float = 0.0
    hi: float = x
    if x < 1.0:
        hi = 1.0
    iterations: int = 0
    while iterations < 1000:
        mid: float = lo + (hi - lo) / 2.0
        val: float = mid * mid - x
        if val < 0.0:
            val = -val
        if val < tol:
            return mid
        if mid * mid < x:
            lo = mid
        else:
            hi = mid
        iterations += 1
    return lo + (hi - lo) / 2.0


def bisection_poly_cubic(a: float, b: float, c: float, d: float,
                         lo: float, hi: float, tol: float) -> float:
    """Find root of ax^3 + bx^2 + cx + d in [lo, hi] via bisection."""
    iterations: int = 0
    while iterations < 1000:
        mid: float = lo + (hi - lo) / 2.0
        f_mid: float = a * mid * mid * mid + b * mid * mid + c * mid + d
        if f_mid < 0.0:
            f_mid_abs: float = -f_mid
        else:
            f_mid_abs = f_mid
        if f_mid_abs < tol:
            return mid
        f_lo: float = a * lo * lo * lo + b * lo * lo + c * lo + d
        if f_lo * f_mid < 0.0:
            hi = mid
        else:
            lo = mid
        iterations += 1
    return lo + (hi - lo) / 2.0


def bisection_exp_approx(target: float, tol: float) -> float:
    """Approximate ln(target) using bisection with exp approximation."""
    lo: float = -10.0
    hi: float = 10.0
    iterations: int = 0
    while iterations < 1000:
        mid: float = lo + (hi - lo) / 2.0
        exp_mid: float = 1.0
        term: float = 1.0
        for i in range(1, 20):
            term = term * mid / float(i)
            exp_mid = exp_mid + term
        diff: float = exp_mid - target
        if diff < 0.0:
            diff = -diff
        if diff < tol:
            return mid
        if exp_mid < target:
            lo = mid
        else:
            hi = mid
        iterations += 1
    return lo + (hi - lo) / 2.0


def bisection_count_iterations(x: float, tol: float) -> int:
    """Count how many iterations bisection needs for sqrt(x)."""
    if x <= 0.0:
        return 0
    lo: float = 0.0
    hi: float = x
    if x < 1.0:
        hi = 1.0
    count: int = 0
    while count < 10000:
        mid: float = lo + (hi - lo) / 2.0
        val: float = mid * mid - x
        if val < 0.0:
            val = -val
        if val < tol:
            return count
        if mid * mid < x:
            lo = mid
        else:
            hi = mid
        count += 1
    return count


def bisection_quadratic(a: float, b: float, c: float,
                        lo: float, hi: float, tol: float) -> float:
    """Find root of ax^2 + bx + c via bisection."""
    iterations: int = 0
    while iterations < 1000:
        mid: float = lo + (hi - lo) / 2.0
        f_mid: float = a * mid * mid + b * mid + c
        if f_mid < 0.0:
            f_mid_abs: float = -f_mid
        else:
            f_mid_abs = f_mid
        if f_mid_abs < tol:
            return mid
        f_lo: float = a * lo * lo + b * lo + c
        if f_lo * f_mid < 0.0:
            hi = mid
        else:
            lo = mid
        iterations += 1
    return lo + (hi - lo) / 2.0


def test_bisection() -> bool:
    """Test bisection methods."""
    ok: bool = True
    sq: float = bisection_sqrt(25.0, 0.0001)
    diff: float = sq - 5.0
    if diff < 0.0:
        diff = -diff
    if diff > 0.01:
        ok = False
    iters: int = bisection_count_iterations(100.0, 0.001)
    if iters < 1:
        ok = False
    return ok
