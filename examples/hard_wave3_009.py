"""Numerical methods: Numerical differentiation.

Tests: finite difference formulas, step size handling, higher-order
derivatives, Richardson extrapolation, error estimation.
"""

from typing import List, Tuple


def forward_diff_x_squared(x: float, h: float) -> float:
    """Forward difference approximation to d/dx(x^2) = 2x."""
    f_x: float = x * x
    f_xh: float = (x + h) * (x + h)
    return (f_xh - f_x) / h


def central_diff_x_squared(x: float, h: float) -> float:
    """Central difference approximation to d/dx(x^2) = 2x."""
    f_plus: float = (x + h) * (x + h)
    f_minus: float = (x - h) * (x - h)
    return (f_plus - f_minus) / (2.0 * h)


def backward_diff_x_squared(x: float, h: float) -> float:
    """Backward difference approximation."""
    f_x: float = x * x
    f_xh: float = (x - h) * (x - h)
    return (f_x - f_xh) / h


def second_derivative_x_cubed(x: float, h: float) -> float:
    """Second derivative of x^3 using central difference."""
    f_plus: float = (x + h) * (x + h) * (x + h)
    f_x: float = x * x * x
    f_minus: float = (x - h) * (x - h) * (x - h)
    return (f_plus - 2.0 * f_x + f_minus) / (h * h)


def richardson_extrapolation(x: float, h: float) -> float:
    """Richardson extrapolation for derivative of x^2."""
    d_h: float = central_diff_x_squared(x, h)
    d_h2: float = central_diff_x_squared(x, h / 2.0)
    return (4.0 * d_h2 - d_h) / 3.0


def five_point_stencil(x: float, h: float) -> float:
    """Five-point stencil for derivative of x^3."""
    f_m2: float = (x - 2.0 * h) * (x - 2.0 * h) * (x - 2.0 * h)
    f_m1: float = (x - h) * (x - h) * (x - h)
    f_p1: float = (x + h) * (x + h) * (x + h)
    f_p2: float = (x + 2.0 * h) * (x + 2.0 * h) * (x + 2.0 * h)
    return (-f_p2 + 8.0 * f_p1 - 8.0 * f_m1 + f_m2) / (12.0 * h)


def derivative_error_table(x: float) -> List[float]:
    """Compute derivative errors for decreasing step sizes of x^2."""
    errors: List[float] = []
    exact: float = 2.0 * x
    h: float = 1.0
    i: int = 0
    while i < 10:
        approx: float = central_diff_x_squared(x, h)
        err: float = approx - exact
        if err < 0.0:
            err = -err
        errors.append(err)
        h = h / 10.0
        i += 1
    return errors


def gradient_2d(x: float, y: float, h: float) -> Tuple[float, float]:
    """Gradient of f(x,y) = x^2 + y^2 using central differences."""
    df_dx: float = ((x + h) * (x + h) + y * y - (x - h) * (x - h) - y * y) / (2.0 * h)
    df_dy: float = (x * x + (y + h) * (y + h) - x * x - (y - h) * (y - h)) / (2.0 * h)
    return (df_dx, df_dy)


def test_derivatives() -> bool:
    """Test numerical differentiation methods."""
    ok: bool = True
    fd: float = forward_diff_x_squared(3.0, 0.001)
    diff: float = fd - 6.0
    if diff < 0.0:
        diff = -diff
    if diff > 0.01:
        ok = False
    cd: float = central_diff_x_squared(3.0, 0.001)
    diff2: float = cd - 6.0
    if diff2 < 0.0:
        diff2 = -diff2
    if diff2 > 0.0001:
        ok = False
    grad: Tuple[float, float] = gradient_2d(1.0, 1.0, 0.001)
    diff3: float = grad[0] - 2.0
    if diff3 < 0.0:
        diff3 = -diff3
    if diff3 > 0.01:
        ok = False
    return ok
