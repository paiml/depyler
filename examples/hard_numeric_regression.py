"""Linear regression using least squares (integer-scaled arithmetic)."""


def sum_list(vals: list[int]) -> int:
    """Sum all elements."""
    total: int = 0
    i: int = 0
    n: int = len(vals)
    while i < n:
        total = total + vals[i]
        i = i + 1
    return total


def sum_products(xs: list[int], ys: list[int]) -> int:
    """Sum of xi*yi."""
    total: int = 0
    i: int = 0
    n: int = len(xs)
    while i < n:
        total = total + xs[i] * ys[i]
        i = i + 1
    return total


def sum_squares(vals: list[int]) -> int:
    """Sum of val^2."""
    total: int = 0
    i: int = 0
    n: int = len(vals)
    while i < n:
        total = total + vals[i] * vals[i]
        i = i + 1
    return total


def linreg_slope_num(xs: list[int], ys: list[int]) -> int:
    """Numerator of slope: n*sum(xy) - sum(x)*sum(y)."""
    n: int = len(xs)
    return n * sum_products(xs, ys) - sum_list(xs) * sum_list(ys)


def linreg_slope_den(xs: list[int]) -> int:
    """Denominator of slope: n*sum(x^2) - (sum(x))^2."""
    n: int = len(xs)
    sx: int = sum_list(xs)
    return n * sum_squares(xs) - sx * sx


def linreg_intercept_num(xs: list[int], ys: list[int]) -> int:
    """Numerator of intercept: sum(y)*sum(x^2) - sum(x)*sum(xy)."""
    return sum_list(ys) * sum_squares(xs) - sum_list(xs) * sum_products(xs, ys)


def test_module() -> int:
    """Test linear regression functions."""
    ok: int = 0
    xs: list[int] = [1, 2, 3, 4, 5]
    ys: list[int] = [2, 4, 6, 8, 10]
    sn: int = linreg_slope_num(xs, ys)
    sd: int = linreg_slope_den(xs)
    if sd != 0 and sn // sd == 2:
        ok = ok + 1
    inum: int = linreg_intercept_num(xs, ys)
    if inum == 0:
        ok = ok + 1
    if sum_list(xs) == 15:
        ok = ok + 1
    if sum_squares(xs) == 55:
        ok = ok + 1
    if sum_products(xs, ys) == 110:
        ok = ok + 1
    return ok
