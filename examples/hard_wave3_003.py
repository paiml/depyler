"""Numerical methods: Trapezoid and Simpson's rule integration.

Tests: summation loops, function evaluation, step size computation,
weighted accumulation, boundary handling.
"""

from typing import List, Tuple


def trapezoid_x_squared(a: float, b: float, n: int) -> float:
    """Integrate x^2 from a to b using trapezoid rule with n steps."""
    if n <= 0:
        return 0.0
    h: float = (b - a) / float(n)
    total: float = (a * a + b * b) / 2.0
    i: int = 1
    while i < n:
        x: float = a + float(i) * h
        total = total + x * x
        i += 1
    return total * h


def trapezoid_x_cubed(a: float, b: float, n: int) -> float:
    """Integrate x^3 from a to b using trapezoid rule."""
    if n <= 0:
        return 0.0
    h: float = (b - a) / float(n)
    fa: float = a * a * a
    fb: float = b * b * b
    total: float = (fa + fb) / 2.0
    i: int = 1
    while i < n:
        x: float = a + float(i) * h
        total = total + x * x * x
        i += 1
    return total * h


def simpson_x_squared(a: float, b: float, n: int) -> float:
    """Integrate x^2 using Simpson's 1/3 rule (n must be even)."""
    if n <= 0:
        return 0.0
    steps: int = n
    if steps % 2 != 0:
        steps = steps + 1
    h: float = (b - a) / float(steps)
    total: float = a * a + b * b
    i: int = 1
    while i < steps:
        x: float = a + float(i) * h
        if i % 2 == 0:
            total = total + 2.0 * x * x
        else:
            total = total + 4.0 * x * x
        i += 1
    return total * h / 3.0


def simpson_38_x_squared(a: float, b: float, n: int) -> float:
    """Integrate x^2 using Simpson's 3/8 rule."""
    if n <= 0:
        return 0.0
    steps: int = n
    if steps < 3:
        steps = 3
    h: float = (b - a) / float(steps)
    total: float = a * a + b * b
    i: int = 1
    while i < steps:
        x: float = a + float(i) * h
        if i % 3 == 0:
            total = total + 2.0 * x * x
        else:
            total = total + 3.0 * x * x
        i += 1
    return total * 3.0 * h / 8.0


def midpoint_rule(a: float, b: float, n: int) -> float:
    """Integrate x^2 using midpoint rectangle rule."""
    if n <= 0:
        return 0.0
    h: float = (b - a) / float(n)
    total: float = 0.0
    i: int = 0
    while i < n:
        mid: float = a + (float(i) + 0.5) * h
        total = total + mid * mid
        i += 1
    return total * h


def romberg_step(a: float, b: float, prev: float, k: int) -> float:
    """One Romberg refinement step for x^2 integration."""
    n_points: int = 1
    for i in range(k):
        n_points = n_points * 2
    h: float = (b - a) / float(n_points)
    total: float = 0.0
    i: int = 0
    while i < n_points:
        x: float = a + (float(i) + 0.5) * h
        total = total + x * x
        i += 1
    return (prev + h * total) / 2.0


def test_integration() -> bool:
    """Test integration methods against known values."""
    ok: bool = True
    trap: float = trapezoid_x_squared(0.0, 1.0, 1000)
    diff: float = trap - 0.333333
    if diff < 0.0:
        diff = -diff
    if diff > 0.01:
        ok = False
    simp: float = simpson_x_squared(0.0, 1.0, 100)
    diff2: float = simp - 0.333333
    if diff2 < 0.0:
        diff2 = -diff2
    if diff2 > 0.01:
        ok = False
    return ok
