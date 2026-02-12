"""Numerical methods: Numerical linear systems and iterative solvers.

Tests: Gauss-Seidel, Jacobi iteration, back substitution,
condition number estimation, norm computation.
"""

from typing import List, Tuple


def jacobi_2x2(a11: float, a12: float, a21: float, a22: float,
               b1: float, b2: float, tol: float, max_iter: int) -> Tuple[float, float]:
    """Solve 2x2 system Ax=b using Jacobi iteration."""
    x1: float = 0.0
    x2: float = 0.0
    iterations: int = 0
    while iterations < max_iter:
        new_x1: float = (b1 - a12 * x2) / a11
        new_x2: float = (b2 - a21 * x1) / a22
        d1: float = new_x1 - x1
        d2: float = new_x2 - x2
        if d1 < 0.0:
            d1 = -d1
        if d2 < 0.0:
            d2 = -d2
        if d1 + d2 < tol:
            return (new_x1, new_x2)
        x1 = new_x1
        x2 = new_x2
        iterations += 1
    return (x1, x2)


def gauss_seidel_2x2(a11: float, a12: float, a21: float, a22: float,
                     b1: float, b2: float, tol: float, max_iter: int) -> Tuple[float, float]:
    """Solve 2x2 system Ax=b using Gauss-Seidel iteration."""
    x1: float = 0.0
    x2: float = 0.0
    iterations: int = 0
    while iterations < max_iter:
        new_x1: float = (b1 - a12 * x2) / a11
        new_x2: float = (b2 - a21 * new_x1) / a22
        d1: float = new_x1 - x1
        d2: float = new_x2 - x2
        if d1 < 0.0:
            d1 = -d1
        if d2 < 0.0:
            d2 = -d2
        if d1 + d2 < tol:
            return (new_x1, new_x2)
        x1 = new_x1
        x2 = new_x2
        iterations += 1
    return (x1, x2)


def back_substitute_upper(a00: float, a01: float, a11: float,
                          b0: float, b1: float) -> Tuple[float, float]:
    """Back substitution for 2x2 upper triangular system."""
    if a11 == 0.0:
        return (0.0, 0.0)
    x1: float = b1 / a11
    if a00 == 0.0:
        return (0.0, x1)
    x0: float = (b0 - a01 * x1) / a00
    return (x0, x1)


def vector_inf_norm(v: List[float]) -> float:
    """Compute infinity norm (max absolute value)."""
    result: float = 0.0
    for x in v:
        abs_x: float = x
        if abs_x < 0.0:
            abs_x = -abs_x
        if abs_x > result:
            result = abs_x
    return result


def vector_one_norm(v: List[float]) -> float:
    """Compute 1-norm (sum of absolute values)."""
    total: float = 0.0
    for x in v:
        abs_x: float = x
        if abs_x < 0.0:
            abs_x = -abs_x
        total = total + abs_x
    return total


def power_iteration_2x2(a00: float, a01: float, a10: float, a11: float,
                        max_iter: int) -> float:
    """Estimate dominant eigenvalue of 2x2 matrix using power iteration."""
    x0: float = 1.0
    x1: float = 0.0
    eigenvalue: float = 0.0
    iterations: int = 0
    while iterations < max_iter:
        y0: float = a00 * x0 + a01 * x1
        y1: float = a10 * x0 + a11 * x1
        mag: float = y0
        if mag < 0.0:
            mag = -mag
        mag2: float = y1
        if mag2 < 0.0:
            mag2 = -mag2
        if mag2 > mag:
            mag = mag2
        if mag == 0.0:
            return 0.0
        x0 = y0 / mag
        x1 = y1 / mag
        eigenvalue = mag
        iterations += 1
    return eigenvalue


def cramer_2x2(a11: float, a12: float, a21: float, a22: float,
               b1: float, b2: float) -> Tuple[float, float]:
    """Solve 2x2 system using Cramer's rule."""
    det: float = a11 * a22 - a12 * a21
    if det == 0.0:
        return (0.0, 0.0)
    x1: float = (b1 * a22 - a12 * b2) / det
    x2: float = (a11 * b2 - b1 * a21) / det
    return (x1, x2)


def test_linear_systems() -> bool:
    """Test linear system solvers."""
    ok: bool = True
    sol: Tuple[float, float] = cramer_2x2(2.0, 1.0, 1.0, 3.0, 5.0, 7.0)
    diff1: float = sol[0] - 1.6
    if diff1 < 0.0:
        diff1 = -diff1
    if diff1 > 0.01:
        ok = False
    jac: Tuple[float, float] = jacobi_2x2(4.0, 1.0, 1.0, 3.0, 5.0, 7.0, 0.001, 100)
    if jac[0] < 0.0:
        ok = False
    gs: Tuple[float, float] = gauss_seidel_2x2(4.0, 1.0, 1.0, 3.0, 5.0, 7.0, 0.001, 100)
    if gs[0] < 0.0:
        ok = False
    return ok
