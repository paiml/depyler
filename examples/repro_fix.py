"""Reproduction for (i + 1) * dx generating i32 * f64.

The issue: When i is from range(n) (Int) and dx is Float,
the expression (i + 1) * dx doesn't compile because (i + 1) is also Int.

Error: E0277: cannot multiply `i32` by `f64`
"""


def find_all_roots(a: float, b: float, n: int) -> list[float]:
    """Find roots in interval."""
    roots = []
    dx = (b - a) / n

    for i in range(n):
        x0 = a + i * dx
        x1 = a + (i + 1) * dx  # BUG: (i + 1) is i32, dx is f64
        roots.append(x0)
        roots.append(x1)

    return roots
