"""Verification that all recent fixes work.

This file tests the fixes from DEPYLER-0804 through DEPYLER-0808.
"""

from collections.abc import Callable


def newton(
    f: Callable[[float], float],
    df: Callable[[float], float],
    x0: float,
) -> float:
    """Tests DEPYLER-0804: x is Float, not numpy array."""
    x = x0
    fx = f(x)
    dfx = df(x)
    x_new = x - fx / dfx  # DEPYLER-0804: x is Float, generates scalar sub
    return x_new


def find_all_roots(a: float, b: float, n: int) -> list[float]:
    """Tests DEPYLER-0805: (i + 1) casts to float."""
    roots = []
    dx = (b - a) / n

    for i in range(n):
        x0 = a + i * dx
        x1 = a + (i + 1) * dx  # DEPYLER-0805: (i + 1) as f64
        roots.append(x0)
        roots.append(x1)

    # DEPYLER-0807: sorted() uses sort_by for floats
    return sorted(roots)


def bisection(a: float, b: float, max_iter: int = 100) -> float:
    """Tests DEPYLER-0806: default arg 100 as i32."""
    for _ in range(max_iter):
        a = (a + b) / 2
    return a


def negative_power():
    """Tests DEPYLER-0808: negative exponent produces float.

    Without type annotation, return type should be inferred as (f64, f64, f64).
    """
    a = 2 ** -1   # 0.5
    b = 10 ** -2  # 0.01
    c = 5 ** -3   # 0.008
    return a, b, c
