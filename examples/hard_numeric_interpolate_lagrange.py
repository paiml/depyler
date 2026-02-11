"""Lagrange interpolation using integer-scaled arithmetic."""


def lagrange_basis_num(xs: list[int], idx: int, x: int) -> int:
    """Numerator of Lagrange basis polynomial L_idx(x).
    Product of (x - xs[j]) for j != idx."""
    n: int = len(xs)
    result: int = 1
    j: int = 0
    while j < n:
        if j != idx:
            result = result * (x - xs[j])
        j = j + 1
    return result


def lagrange_basis_den(xs: list[int], idx: int) -> int:
    """Denominator of Lagrange basis: product of (xs[idx]-xs[j]) for j!=idx."""
    n: int = len(xs)
    result: int = 1
    j: int = 0
    xi: int = xs[idx]
    while j < n:
        if j != idx:
            result = result * (xi - xs[j])
        j = j + 1
    return result


def lagrange_interp_scaled(xs: list[int], ys: list[int], x: int) -> list[int]:
    """Lagrange interpolation. Returns [numerator, denominator].
    P(x) = sum(ys[i] * L_i(x))."""
    n: int = len(xs)
    common_den: int = 1
    i: int = 0
    while i < n:
        bd: int = lagrange_basis_den(xs, i)
        common_den = common_den * bd
        i = i + 1
    num: int = 0
    i2: int = 0
    while i2 < n:
        bn: int = lagrange_basis_num(xs, i2, x)
        bd2: int = lagrange_basis_den(xs, i2)
        term: int = ys[i2] * bn * (common_den // bd2)
        num = num + term
        i2 = i2 + 1
    return [num, common_den]


def linear_interp(x0: int, y0: int, x1: int, y1: int, x: int) -> list[int]:
    """Linear interpolation. Returns [numerator, denominator]."""
    num: int = y0 * (x1 - x) + y1 * (x - x0)
    den: int = x1 - x0
    return [num, den]


def test_module() -> int:
    """Test Lagrange interpolation."""
    ok: int = 0
    xs: list[int] = [0, 1, 2]
    ys: list[int] = [0, 1, 4]
    r: list[int] = lagrange_interp_scaled(xs, ys, 0)
    if r[0] == 0:
        ok = ok + 1
    r1: list[int] = lagrange_interp_scaled(xs, ys, 1)
    nr: int = r1[0]
    dr: int = r1[1]
    if dr != 0 and nr // dr == 1:
        ok = ok + 1
    li: list[int] = linear_interp(0, 0, 10, 100, 5)
    if li[0] == 500:
        ok = ok + 1
    if li[1] == 10:
        ok = ok + 1
    li2: list[int] = linear_interp(0, 0, 10, 100, 0)
    if li2[0] == 0:
        ok = ok + 1
    return ok
