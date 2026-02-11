"""Trapezoidal rule for numerical integration (integer-scaled)."""


def eval_quadratic(a: int, b: int, c: int, x: int) -> int:
    """Evaluate a*x^2 + b*x + c."""
    return a * x * x + b * x + c


def trapezoid_quadratic(a: int, b: int, c: int, x0: int, x1: int, n: int) -> int:
    """Trapezoidal integration of a*x^2 + b*x + c from x0 to x1 with n steps.
    All values scaled by 1000. Returns area * 1000."""
    if n <= 0:
        return 0
    h: int = (x1 - x0) // n
    if h == 0:
        return 0
    total: int = eval_quadratic(a, b, c, x0) + eval_quadratic(a, b, c, x1)
    i: int = 1
    while i < n:
        xi: int = x0 + i * h
        total = total + 2 * eval_quadratic(a, b, c, xi)
        i = i + 1
    return (total * h) // 2


def trapezoid_linear(slope: int, intercept: int, x0: int, x1: int) -> int:
    """Exact trapezoidal integration for linear function (slope*x + intercept).
    Returns 2*area (to avoid division)."""
    y0: int = slope * x0 + intercept
    y1: int = slope * x1 + intercept
    return (y0 + y1) * (x1 - x0)


def sum_of_squares(n: int) -> int:
    """Sum of i^2 for i = 1..n. Exact formula: n*(n+1)*(2n+1)/6."""
    return n * (n + 1) * (2 * n + 1) // 6


def sum_of_cubes(n: int) -> int:
    """Sum of i^3 for i = 1..n. Exact formula: (n*(n+1)/2)^2."""
    half: int = n * (n + 1) // 2
    return half * half


def test_module() -> int:
    """Test trapezoidal integration."""
    ok: int = 0
    area: int = trapezoid_linear(2, 0, 0, 10)
    if area == 200:
        ok = ok + 1
    area2: int = trapezoid_linear(0, 5, 0, 10)
    if area2 == 100:
        ok = ok + 1
    if sum_of_squares(3) == 14:
        ok = ok + 1
    if sum_of_cubes(3) == 36:
        ok = ok + 1
    t: int = trapezoid_quadratic(1, 0, 0, 0, 100, 100)
    if t > 0:
        ok = ok + 1
    return ok
