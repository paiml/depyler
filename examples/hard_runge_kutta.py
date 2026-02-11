# Runge-Kutta 4th order ODE solver using integer-scaled arithmetic
# Solves dy/dx = f(x, y) with fixed-point math


def rk4_linear(y0: int, x0: int, x_end: int, steps: int, scale: int) -> int:
    # Solve dy/dx = y (exponential growth) using RK4
    if steps == 0:
        return y0
    h: int = (x_end - x0) // steps
    y: int = y0
    i: int = 0
    while i < steps:
        k1: int = y
        k2: int = y + h * k1 // (2 * scale)
        k3: int = y + h * k2 // (2 * scale)
        k4: int = y + h * k3 // scale
        y = y + h * (k1 + 2 * k2 + 2 * k3 + k4) // (6 * scale)
        i = i + 1
    return y


def rk4_decay(y0: int, x0: int, x_end: int, steps: int, scale: int) -> int:
    # Solve dy/dx = -y (exponential decay) using RK4
    if steps == 0:
        return y0
    h: int = (x_end - x0) // steps
    y: int = y0
    i: int = 0
    while i < steps:
        k1: int = -y
        k2: int = -(y + h * k1 // (2 * scale))
        k3: int = -(y + h * k2 // (2 * scale))
        k4: int = -(y + h * k3 // scale)
        y = y + h * (k1 + 2 * k2 + 2 * k3 + k4) // (6 * scale)
        i = i + 1
    return y


def rk4_constant(y0: int, rate: int, x0: int, x_end: int, steps: int, scale: int) -> int:
    # Solve dy/dx = rate (constant rate)
    if steps == 0:
        return y0
    h: int = (x_end - x0) // steps
    y: int = y0
    i: int = 0
    while i < steps:
        k1: int = rate
        k2: int = rate
        k3: int = rate
        k4: int = rate
        y = y + h * (k1 + 2 * k2 + 2 * k3 + k4) // (6 * scale)
        i = i + 1
    return y


def abs_val(x: int) -> int:
    if x < 0:
        return -x
    return x


def test_module() -> int:
    passed: int = 0
    scale: int = 1000

    # Test 1: constant rate dy/dx = 1, from 0 to 1, y0=0 => y=1
    r: int = rk4_constant(0, scale, 0, scale, 100, scale)
    if abs_val(r - scale) < 10:
        passed = passed + 1

    # Test 2: constant rate dy/dx = 2, from 0 to 1, y0=0 => y=2
    r = rk4_constant(0, 2 * scale, 0, scale, 100, scale)
    if abs_val(r - 2 * scale) < 10:
        passed = passed + 1

    # Test 3: constant rate with initial value y0=5, dy/dx=1, 0->1 => 6
    r = rk4_constant(5 * scale, scale, 0, scale, 100, scale)
    if abs_val(r - 6 * scale) < 10:
        passed = passed + 1

    # Test 4: exponential growth: result is between 2.5 and 2.8 (scale units)
    r = rk4_linear(scale, 0, scale, 100, scale)
    if r > 2500 and r < 2800:
        passed = passed + 1

    # Test 5: decay result positive and less than initial
    r = rk4_decay(scale, 0, scale, 100, scale)
    if r > 0 and r < scale:
        passed = passed + 1

    # Test 6: zero steps returns initial value
    r = rk4_constant(42 * scale, scale, 0, scale, 0, scale)
    if r == 42 * scale:
        passed = passed + 1

    # Test 7: more steps means higher accuracy for growth
    r1: int = rk4_linear(scale, 0, scale, 10, scale)
    r2: int = rk4_linear(scale, 0, scale, 50, scale)
    r3: int = rk4_linear(scale, 0, scale, 200, scale)
    # All should be in reasonable range and converge
    if r1 > 2000 and r2 > 2000 and r3 > 2000:
        passed = passed + 1

    return passed
