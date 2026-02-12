"""Numerical methods: Euler method and ODE solvers.

Tests: step-by-step iteration, state accumulation, derivative evaluation,
multi-variable updates, trajectory recording.
"""

from typing import List, Tuple


def euler_exponential(x0: float, dt: float, steps: int) -> float:
    """Solve dy/dx = y with y(0)=x0 using Euler method (approximates e^t)."""
    y: float = x0
    i: int = 0
    while i < steps:
        y = y + y * dt
        i += 1
    return y


def euler_decay(x0: float, rate: float, dt: float, steps: int) -> float:
    """Solve dy/dt = -rate*y with y(0)=x0 (exponential decay)."""
    y: float = x0
    i: int = 0
    while i < steps:
        y = y - rate * y * dt
        i += 1
    return y


def euler_harmonic_position(x0: float, v0: float, dt: float, steps: int) -> float:
    """Euler method for simple harmonic oscillator, return final position."""
    x: float = x0
    v: float = v0
    i: int = 0
    while i < steps:
        new_x: float = x + v * dt
        new_v: float = v - x * dt
        x = new_x
        v = new_v
        i += 1
    return x


def euler_harmonic_velocity(x0: float, v0: float, dt: float, steps: int) -> float:
    """Euler method for simple harmonic oscillator, return final velocity."""
    x: float = x0
    v: float = v0
    i: int = 0
    while i < steps:
        new_x: float = x + v * dt
        new_v: float = v - x * dt
        x = new_x
        v = new_v
        i += 1
    return v


def euler_logistic(pop: float, rate: float, capacity: float,
                   dt: float, steps: int) -> float:
    """Logistic growth: dp/dt = r*p*(1-p/K)."""
    p: float = pop
    i: int = 0
    while i < steps:
        growth: float = rate * p * (1.0 - p / capacity)
        p = p + growth * dt
        if p < 0.0:
            p = 0.0
        i += 1
    return p


def rk2_midpoint(y0: float, dt: float, steps: int) -> float:
    """RK2 midpoint method for dy/dx = y."""
    y: float = y0
    i: int = 0
    while i < steps:
        k1: float = y * dt
        k2: float = (y + k1 / 2.0) * dt
        y = y + k2
        i += 1
    return y


def rk4_exponential(y0: float, dt: float, steps: int) -> float:
    """RK4 method for dy/dx = y (should approximate e^(steps*dt)*y0)."""
    y: float = y0
    i: int = 0
    while i < steps:
        k1: float = y * dt
        k2: float = (y + k1 / 2.0) * dt
        k3: float = (y + k2 / 2.0) * dt
        k4: float = (y + k3) * dt
        y = y + (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0
        i += 1
    return y


def euler_trajectory_sum(x0: float, v0: float, dt: float, steps: int) -> float:
    """Sum of all positions in Euler trajectory (energy proxy)."""
    x: float = x0
    v: float = v0
    total: float = 0.0
    i: int = 0
    while i < steps:
        total = total + x
        new_x: float = x + v * dt
        new_v: float = v - x * dt
        x = new_x
        v = new_v
        i += 1
    return total


def test_euler_methods() -> bool:
    """Test Euler and RK methods."""
    ok: bool = True
    exp1: float = euler_exponential(1.0, 0.001, 1000)
    diff: float = exp1 - 2.718
    if diff < 0.0:
        diff = -diff
    if diff > 0.1:
        ok = False
    rk4: float = rk4_exponential(1.0, 0.001, 1000)
    diff2: float = rk4 - 2.718
    if diff2 < 0.0:
        diff2 = -diff2
    if diff2 > 0.01:
        ok = False
    decay: float = euler_decay(100.0, 0.1, 0.01, 100)
    if decay > 100.0:
        ok = False
    if decay < 0.0:
        ok = False
    return ok
