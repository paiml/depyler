"""Simpson's rule for numerical integration (integer-scaled)."""


def eval_quadratic(a: int, b: int, c: int, x: int) -> int:
    """Evaluate a*x^2 + b*x + c."""
    return a * x * x + b * x + c


def simpson_quadratic(a: int, b: int, c: int, x0: int, x1: int, n: int) -> int:
    """Simpson's 1/3 rule for a*x^2+b*x+c. n must be even.
    Returns approximate integral * 3 (to avoid division by 3)."""
    if n <= 0 or n % 2 != 0:
        return 0
    h: int = (x1 - x0) // n
    if h == 0:
        return 0
    total: int = eval_quadratic(a, b, c, x0) + eval_quadratic(a, b, c, x1)
    i: int = 1
    while i < n:
        xi: int = x0 + i * h
        fv: int = eval_quadratic(a, b, c, xi)
        if i % 2 == 0:
            total = total + 2 * fv
        else:
            total = total + 4 * fv
        i = i + 1
    return total * h


def exact_integral_quadratic(a: int, b: int, c: int, x0: int, x1: int) -> int:
    """Exact integral of a*x^2+b*x+c from x0 to x1, times 6 to stay integer.
    6 * integral = 2*a*(x1^3-x0^3) + 3*b*(x1^2-x0^2) + 6*c*(x1-x0)."""
    d3: int = x1 * x1 * x1 - x0 * x0 * x0
    d2: int = x1 * x1 - x0 * x0
    d1: int = x1 - x0
    return 2 * a * d3 + 3 * b * d2 + 6 * c * d1


def simpson_38(f0: int, f1: int, f2: int, f3: int, h: int) -> int:
    """Simpson's 3/8 rule for 4 equally spaced points. Returns 8*area."""
    return h * (f0 + 3 * f1 + 3 * f2 + f3)


def midpoint_rule(fmid: int, width: int) -> int:
    """Midpoint rule: area = f(mid) * width."""
    return fmid * width


def test_module() -> int:
    """Test Simpson's rule integration."""
    ok: int = 0
    exact6: int = exact_integral_quadratic(1, 0, 0, 0, 3)
    if exact6 == 54:
        ok = ok + 1
    s: int = simpson_quadratic(1, 0, 0, 0, 300, 100)
    if s > 0:
        ok = ok + 1
    exact_lin6: int = exact_integral_quadratic(0, 1, 0, 0, 4)
    if exact_lin6 == 48:
        ok = ok + 1
    mr: int = midpoint_rule(5, 10)
    if mr == 50:
        ok = ok + 1
    s38: int = simpson_38(1, 2, 2, 1, 1)
    if s38 == 14:
        ok = ok + 1
    return ok
