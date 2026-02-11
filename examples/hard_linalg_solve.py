"""Solve Ax=b via Gaussian elimination (integer-scaled)."""


def mat_get(m: list[int], cols: int, r: int, c: int) -> int:
    """Get element at row r, col c."""
    idx: int = r * cols + c
    return m[idx]


def solve_2x2(a: list[int], b: list[int]) -> list[int]:
    """Solve 2x2 system Ax=b using Cramer's rule (scaled by det).
    Returns [det*x1, det*x2, det]."""
    d: int = a[0] * a[3] - a[1] * a[2]
    if d == 0:
        return [0, 0, 0]
    x0: int = b[0] * a[3] - a[1] * b[1]
    x1: int = a[0] * b[1] - b[0] * a[2]
    return [x0, x1, d]


def forward_substitution(lower: list[int], b: list[int], n: int) -> list[int]:
    """Solve Ly=b where L is lower triangular (diagonal 1s)."""
    y: list[int] = []
    i: int = 0
    while i < n:
        y.append(0)
        i = i + 1
    i2: int = 0
    while i2 < n:
        s: int = 0
        j: int = 0
        while j < i2:
            lv: int = mat_get(lower, n, i2, j)
            s = s + lv * y[j]
            j = j + 1
        y[i2] = b[i2] - s
        i2 = i2 + 1
    return y


def back_substitution_scaled(upper: list[int], b: list[int], n: int) -> list[int]:
    """Solve Ux=b for upper triangular. Returns x*product_of_pivots."""
    x: list[int] = []
    i: int = 0
    while i < n:
        x.append(0)
        i = i + 1
    i2: int = n - 1
    while i2 >= 0:
        s: int = 0
        j: int = i2 + 1
        while j < n:
            uv: int = mat_get(upper, n, i2, j)
            s = s + uv * x[j]
            j = j + 1
        piv: int = mat_get(upper, n, i2, i2)
        if piv != 0:
            x[i2] = (b[i2] - s) // piv
        i2 = i2 - 1
    return x


def test_module() -> int:
    """Test solve functions."""
    ok: int = 0
    a: list[int] = [2, 1, 5, 3]
    b: list[int] = [4, 7]
    sol: list[int] = solve_2x2(a, b)
    if sol[2] == 1:
        ok = ok + 1
    if sol[0] == 5:
        ok = ok + 1
    if sol[1] == 0 - 6:
        ok = ok + 1
    low: list[int] = [1, 0, 2, 1]
    bv: list[int] = [3, 8]
    y: list[int] = forward_substitution(low, bv, 2)
    if y[0] == 3:
        ok = ok + 1
    if y[1] == 2:
        ok = ok + 1
    return ok
