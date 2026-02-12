"""Numerical methods: Numerical optimization.

Tests: gradient descent, line search, golden section search,
function minimization, parameter tuning loops.
"""

from typing import List, Tuple


def gradient_descent_1d(x0: float, lr: float, steps: int) -> float:
    """Minimize f(x) = x^2 using gradient descent."""
    x: float = x0
    i: int = 0
    while i < steps:
        grad: float = 2.0 * x
        x = x - lr * grad
        i += 1
    return x


def gradient_descent_2d(x0: float, y0: float, lr: float, steps: int) -> Tuple[float, float]:
    """Minimize f(x,y) = x^2 + y^2 using gradient descent."""
    x: float = x0
    y: float = y0
    i: int = 0
    while i < steps:
        gx: float = 2.0 * x
        gy: float = 2.0 * y
        x = x - lr * gx
        y = y - lr * gy
        i += 1
    return (x, y)


def golden_section_search(a: float, b: float, tol: float) -> float:
    """Minimize f(x) = (x-2)^2 on [a,b] using golden section search."""
    phi: float = 1.618034
    resphi: float = 2.0 - phi
    x1: float = a + resphi * (b - a)
    x2: float = b - resphi * (b - a)
    f1: float = (x1 - 2.0) * (x1 - 2.0)
    f2: float = (x2 - 2.0) * (x2 - 2.0)
    iterations: int = 0
    while iterations < 1000:
        diff: float = b - a
        if diff < 0.0:
            diff = -diff
        if diff < tol:
            return (a + b) / 2.0
        if f1 < f2:
            b = x2
            x2 = x1
            f2 = f1
            x1 = a + resphi * (b - a)
            f1 = (x1 - 2.0) * (x1 - 2.0)
        else:
            a = x1
            x1 = x2
            f1 = f2
            x2 = b - resphi * (b - a)
            f2 = (x2 - 2.0) * (x2 - 2.0)
        iterations += 1
    return (a + b) / 2.0


def ternary_search_min(a: float, b: float, tol: float) -> float:
    """Ternary search for minimum of f(x) = (x-3)^2 on [a,b]."""
    iterations: int = 0
    lo: float = a
    hi: float = b
    while iterations < 1000:
        diff: float = hi - lo
        if diff < 0.0:
            diff = -diff
        if diff < tol:
            return (lo + hi) / 2.0
        m1: float = lo + (hi - lo) / 3.0
        m2: float = hi - (hi - lo) / 3.0
        f1: float = (m1 - 3.0) * (m1 - 3.0)
        f2: float = (m2 - 3.0) * (m2 - 3.0)
        if f1 < f2:
            hi = m2
        else:
            lo = m1
        iterations += 1
    return (lo + hi) / 2.0


def momentum_gd(x0: float, lr: float, momentum: float, steps: int) -> float:
    """Gradient descent with momentum for f(x) = x^4 - 2x^2."""
    x: float = x0
    vel: float = 0.0
    i: int = 0
    while i < steps:
        grad: float = 4.0 * x * x * x - 4.0 * x
        vel = momentum * vel - lr * grad
        x = x + vel
        i += 1
    return x


def simulated_annealing_1d(x0: float, temp: float, cooling: float, steps: int) -> float:
    """Simple simulated annealing for f(x) = x^2 using deterministic perturbation."""
    x: float = x0
    best: float = x0
    best_val: float = x0 * x0
    t: float = temp
    i: int = 0
    while i < steps:
        step: float = t * 0.1
        if i % 2 == 0:
            step = -step
        candidate: float = x + step
        cand_val: float = candidate * candidate
        if cand_val < best_val:
            best = candidate
            best_val = cand_val
            x = candidate
        elif t > 0.01:
            x = candidate
        t = t * cooling
        i += 1
    return best


def test_optimization() -> bool:
    """Test optimization methods."""
    ok: bool = True
    min1: float = gradient_descent_1d(10.0, 0.1, 100)
    if min1 > 0.01:
        ok = False
    if min1 < -0.01:
        ok = False
    golden: float = golden_section_search(0.0, 5.0, 0.0001)
    diff: float = golden - 2.0
    if diff < 0.0:
        diff = -diff
    if diff > 0.01:
        ok = False
    ternary: float = ternary_search_min(0.0, 6.0, 0.0001)
    diff2: float = ternary - 3.0
    if diff2 < 0.0:
        diff2 = -diff2
    if diff2 > 0.01:
        ok = False
    return ok
