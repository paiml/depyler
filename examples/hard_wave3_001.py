"""Numerical methods: Newton's method variants for root finding.

Tests: floating point arithmetic, convergence loops, absolute value,
conditional termination, function evaluation patterns.
"""

from typing import List, Tuple


def newton_sqrt(x: float, tol: float) -> float:
    """Compute square root of x using Newton's method."""
    if x < 0.0:
        return -1.0
    if x == 0.0:
        return 0.0
    guess: float = x / 2.0
    iterations: int = 0
    while iterations < 1000:
        new_guess: float = (guess + x / guess) / 2.0
        diff: float = new_guess - guess
        if diff < 0.0:
            diff = -diff
        if diff < tol:
            return new_guess
        guess = new_guess
        iterations += 1
    return guess


def newton_cube_root(x: float, tol: float) -> float:
    """Compute cube root using Newton's method."""
    if x == 0.0:
        return 0.0
    sign: float = 1.0
    val: float = x
    if x < 0.0:
        sign = -1.0
        val = -x
    guess: float = val / 3.0
    iterations: int = 0
    while iterations < 1000:
        new_guess: float = (2.0 * guess + val / (guess * guess)) / 3.0
        diff: float = new_guess - guess
        if diff < 0.0:
            diff = -diff
        if diff < tol:
            return sign * new_guess
        guess = new_guess
        iterations += 1
    return sign * guess


def newton_reciprocal(a: float, tol: float) -> float:
    """Compute 1/a without division using Newton's method."""
    if a == 0.0:
        return 0.0
    sign: float = 1.0
    val: float = a
    if a < 0.0:
        sign = -1.0
        val = -a
    guess: float = 0.1
    iterations: int = 0
    while iterations < 1000:
        new_guess: float = guess * (2.0 - val * guess)
        diff: float = new_guess - guess
        if diff < 0.0:
            diff = -diff
        if diff < tol:
            return sign * new_guess
        guess = new_guess
        iterations += 1
    return sign * guess


def newton_nth_root(x: float, n: int, tol: float) -> float:
    """Compute nth root of x using generalized Newton's method."""
    if x == 0.0:
        return 0.0
    if n == 0:
        return 1.0
    guess: float = x / float(n)
    iterations: int = 0
    while iterations < 1000:
        power: float = 1.0
        for i in range(n - 1):
            power = power * guess
        new_guess: float = ((float(n) - 1.0) * guess + x / power) / float(n)
        diff: float = new_guess - guess
        if diff < 0.0:
            diff = -diff
        if diff < tol:
            return new_guess
        guess = new_guess
        iterations += 1
    return guess


def halley_cube_root(x: float, tol: float) -> float:
    """Halley's method for cube root (cubic convergence)."""
    if x == 0.0:
        return 0.0
    guess: float = x / 3.0
    iterations: int = 0
    while iterations < 500:
        g3: float = guess * guess * guess
        num: float = g3 - x
        denom: float = 3.0 * guess * guess
        second: float = 6.0 * guess
        correction: float = num / (denom - (num * second) / (2.0 * denom))
        guess = guess - correction
        diff: float = correction
        if diff < 0.0:
            diff = -diff
        if diff < tol:
            return guess
        iterations += 1
    return guess


def test_newton_methods() -> bool:
    """Test all Newton's method variants."""
    ok: bool = True
    sq: float = newton_sqrt(16.0, 0.0001)
    diff1: float = sq - 4.0
    if diff1 < 0.0:
        diff1 = -diff1
    if diff1 > 0.01:
        ok = False
    cb: float = newton_cube_root(27.0, 0.0001)
    diff2: float = cb - 3.0
    if diff2 < 0.0:
        diff2 = -diff2
    if diff2 > 0.01:
        ok = False
    rec: float = newton_reciprocal(4.0, 0.0001)
    diff3: float = rec - 0.25
    if diff3 < 0.0:
        diff3 = -diff3
    if diff3 > 0.01:
        ok = False
    return ok
