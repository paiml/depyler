"""Euler method for solving ODEs using integer-scaled arithmetic."""


def euler_linear(slope: int, y0: int, steps: int, h: int) -> int:
    """Euler method for dy/dx = slope (constant). y(0)=y0.
    h is step size. Returns y after 'steps' steps."""
    y: int = y0
    i: int = 0
    while i < steps:
        y = y + slope * h
        i = i + 1
    return y


def euler_proportional(y0: int, steps: int, h_num: int, h_den: int) -> int:
    """Euler method for dy/dx = y (exponential growth).
    Step size = h_num/h_den. All scaled by h_den^steps.
    Returns y * h_den^steps."""
    y: int = y0
    i: int = 0
    scale: int = 1
    while i < steps:
        y = y * h_den + y * h_num
        scale = scale * h_den
        i = i + 1
    return y


def euler_decay(y0: int, rate: int, steps: int) -> int:
    """Euler method for dy/dx = -rate*y. Step h=1. Integer approx.
    Each step: y = y - rate*y = y*(1-rate). For rate < 1 use scaled."""
    y: int = y0 * 1000
    i: int = 0
    while i < steps:
        y = y - (rate * y) // 1000
        i = i + 1
    return y // 1000


def euler_quadratic(y0: int, steps: int) -> int:
    """Euler method for dy/dx = 2*x, y(0)=y0, h=1.
    Exact: y = x^2 + y0."""
    y: int = y0
    x: int = 0
    i: int = 0
    while i < steps:
        y = y + 2 * x
        x = x + 1
        i = i + 1
    return y


def exact_linear(slope: int, y0: int, x: int) -> int:
    """Exact solution of dy/dx = slope: y = slope*x + y0."""
    return slope * x + y0


def test_module() -> int:
    """Test Euler method for ODEs."""
    ok: int = 0
    r1: int = euler_linear(3, 0, 10, 1)
    if r1 == 30:
        ok = ok + 1
    r2: int = exact_linear(3, 0, 10)
    if r1 == r2:
        ok = ok + 1
    r3: int = euler_quadratic(0, 5)
    if r3 == 20:
        ok = ok + 1
    r4: int = euler_decay(1000, 100, 1)
    if r4 == 900:
        ok = ok + 1
    r5: int = euler_linear(0, 42, 100, 1)
    if r5 == 42:
        ok = ok + 1
    return ok
