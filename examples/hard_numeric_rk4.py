"""Runge-Kutta 4th order method using integer-scaled arithmetic."""


def rk4_linear(slope: int, y0: int, steps: int, h: int) -> int:
    """RK4 for dy/dx = slope (constant). Exact for any h.
    Returns y after 'steps' steps of size h."""
    y: int = y0
    i: int = 0
    while i < steps:
        k1: int = slope * h
        k2: int = slope * h
        k3: int = slope * h
        k4: int = slope * h
        y = y + (k1 + 2 * k2 + 2 * k3 + k4) // 6
        i = i + 1
    return y


def rk4_quadratic(y0: int, x0: int, steps: int, h: int) -> int:
    """RK4 for dy/dx = 2*x, y(x0)=y0. Step size h.
    Returns y * 1 (integer)."""
    y: int = y0
    x: int = x0
    i: int = 0
    while i < steps:
        k1: int = 2 * x * h
        k2: int = (2 * x + h) * h
        k3: int = (2 * x + h) * h
        k4: int = (2 * (x + h)) * h
        dy: int = (k1 + 2 * k2 + 2 * k3 + k4) // 6
        y = y + dy
        x = x + h
        i = i + 1
    return y


def rk4_step_count(total_interval: int, step_size: int) -> int:
    """Calculate number of steps needed."""
    if step_size <= 0:
        return 0
    return total_interval // step_size


def error_estimate(euler_val: int, rk4_val: int) -> int:
    """Absolute difference between Euler and RK4 results."""
    diff: int = euler_val - rk4_val
    if diff < 0:
        diff = 0 - diff
    return diff


def adaptive_step_needed(err: int, tol: int) -> int:
    """Returns 1 if error exceeds tolerance."""
    if err > tol:
        return 1
    return 0


def test_module() -> int:
    """Test RK4 method."""
    ok: int = 0
    r1: int = rk4_linear(5, 0, 10, 1)
    if r1 == 50:
        ok = ok + 1
    r2: int = rk4_linear(0, 100, 50, 1)
    if r2 == 100:
        ok = ok + 1
    sc: int = rk4_step_count(100, 5)
    if sc == 20:
        ok = ok + 1
    e: int = error_estimate(105, 100)
    if e == 5:
        ok = ok + 1
    if adaptive_step_needed(10, 5) == 1:
        ok = ok + 1
    return ok
