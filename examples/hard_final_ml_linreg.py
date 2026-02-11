"""Simple linear regression using ordinary least squares.

Computes slope and intercept for y = mx + b using closed-form formulas.
"""


def mean_val(arr: list[float]) -> float:
    """Compute mean of a float array."""
    total: float = 0.0
    i: int = 0
    while i < len(arr):
        v: float = arr[i]
        total = total + v
        i = i + 1
    n: int = len(arr)
    if n == 0:
        return 0.0
    return total / (n * 1.0)


def variance_val(arr: list[float]) -> float:
    """Compute variance of a float array."""
    mu: float = mean_val(arr)
    total: float = 0.0
    i: int = 0
    while i < len(arr):
        v: float = arr[i]
        diff: float = v - mu
        total = total + diff * diff
        i = i + 1
    n: int = len(arr)
    if n == 0:
        return 0.0
    return total / (n * 1.0)


def covariance_val(xs: list[float], ys: list[float]) -> float:
    """Compute covariance of two arrays."""
    mx: float = mean_val(xs)
    my: float = mean_val(ys)
    total: float = 0.0
    i: int = 0
    while i < len(xs):
        vx: float = xs[i]
        vy: float = ys[i]
        total = total + (vx - mx) * (vy - my)
        i = i + 1
    n: int = len(xs)
    if n == 0:
        return 0.0
    return total / (n * 1.0)


def lin_reg_slope(xs: list[float], ys: list[float]) -> float:
    """Compute slope of linear regression."""
    var_x: float = variance_val(xs)
    if var_x < 0.0001:
        return 0.0
    return covariance_val(xs, ys) / var_x


def lin_reg_intercept(xs: list[float], ys: list[float]) -> float:
    """Compute intercept of linear regression."""
    slope: float = lin_reg_slope(xs, ys)
    return mean_val(ys) - slope * mean_val(xs)


def predict_val(slope: float, intercept: float, x: float) -> float:
    """Predict y given slope, intercept, and x."""
    return slope * x + intercept


def r_squared(xs: list[float], ys: list[float]) -> float:
    """Compute R-squared goodness of fit."""
    slope: float = lin_reg_slope(xs, ys)
    intercept: float = lin_reg_intercept(xs, ys)
    my: float = mean_val(ys)
    ss_res: float = 0.0
    ss_tot: float = 0.0
    i: int = 0
    while i < len(xs):
        vx: float = xs[i]
        vy: float = ys[i]
        pred: float = predict_val(slope, intercept, vx)
        diff_res: float = vy - pred
        ss_res = ss_res + diff_res * diff_res
        diff_tot: float = vy - my
        ss_tot = ss_tot + diff_tot * diff_tot
        i = i + 1
    if ss_tot < 0.0001:
        return 1.0
    return 1.0 - ss_res / ss_tot


def approx(a: float, b: float, tol: float) -> int:
    """Check if two floats are approximately equal."""
    diff: float = a - b
    if diff < 0.0:
        diff = 0.0 - diff
    if diff < tol:
        return 1
    return 0


def test_module() -> int:
    """Test linear regression."""
    ok: int = 0
    xs: list[float] = [1.0, 2.0, 3.0, 4.0, 5.0]
    ys: list[float] = [2.0, 4.0, 6.0, 8.0, 10.0]
    slope: float = lin_reg_slope(xs, ys)
    if approx(slope, 2.0, 0.01) == 1:
        ok = ok + 1
    intercept: float = lin_reg_intercept(xs, ys)
    if approx(intercept, 0.0, 0.01) == 1:
        ok = ok + 1
    pred: float = predict_val(slope, intercept, 6.0)
    if approx(pred, 12.0, 0.01) == 1:
        ok = ok + 1
    r2: float = r_squared(xs, ys)
    if approx(r2, 1.0, 0.01) == 1:
        ok = ok + 1
    m: float = mean_val(xs)
    if approx(m, 3.0, 0.01) == 1:
        ok = ok + 1
    return ok
