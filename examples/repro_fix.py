"""Reproduction for x - fx / dfx generating Vector code.

The issue: When x, fx, dfx are all floats, the expression x - fx / dfx
should generate simple scalar arithmetic, not Vector::from_vec nonsense.

Error: E0433: failed to resolve: use of undeclared type `Vector`
"""

from collections.abc import Callable


def newton(
    f: Callable[[float], float],
    df: Callable[[float], float],
    x0: float,
) -> float:
    x = x0
    fx = f(x)
    dfx = df(x)
    x_new = x - fx / dfx  # BUG: generates Vector::from_vec(x.as_slice()...)
    return x_new
