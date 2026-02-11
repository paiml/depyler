# Simple linear regression (least squares) using integer-scaled arithmetic


def sum_list(arr: list[int]) -> int:
    total: int = 0
    i: int = 0
    while i < len(arr):
        total = total + arr[i]
        i = i + 1
    return total


def sum_product(xs: list[int], ys: list[int]) -> int:
    total: int = 0
    i: int = 0
    while i < len(xs):
        total = total + xs[i] * ys[i]
        i = i + 1
    return total


def sum_squares(arr: list[int]) -> int:
    total: int = 0
    i: int = 0
    while i < len(arr):
        total = total + arr[i] * arr[i]
        i = i + 1
    return total


def regression_slope(xs: list[int], ys: list[int], scale: int) -> int:
    # Returns slope * scale
    n: int = len(xs)
    if n == 0:
        return 0
    sx: int = sum_list(xs)
    sy: int = sum_list(ys)
    sxy: int = sum_product(xs, ys)
    sxx: int = sum_squares(xs)
    numerator: int = n * sxy - sx * sy
    denominator: int = n * sxx - sx * sx
    if denominator == 0:
        return 0
    return numerator * scale // denominator


def regression_intercept(xs: list[int], ys: list[int], slope_scaled: int, scale: int) -> int:
    # Returns intercept * scale
    n: int = len(xs)
    if n == 0:
        return 0
    sx: int = sum_list(xs)
    sy: int = sum_list(ys)
    return (sy * scale - slope_scaled * sx) // n


def predict(slope_scaled: int, intercept_scaled: int, x: int, scale: int) -> int:
    # Returns predicted y * scale
    return slope_scaled * x // scale + intercept_scaled


def residual_sum_squares(xs: list[int], ys: list[int], slope_scaled: int, intercept_scaled: int, scale: int) -> int:
    total: int = 0
    i: int = 0
    while i < len(xs):
        pred: int = predict(slope_scaled, intercept_scaled, xs[i], scale)
        diff: int = ys[i] * scale - pred
        total = total + diff * diff // (scale * scale)
        i = i + 1
    return total


def abs_val(x: int) -> int:
    if x < 0:
        return -x
    return x


def test_module() -> int:
    passed: int = 0
    scale: int = 1000

    # Test 1: perfect linear y = 2x + 1
    # Points: (0,1), (1,3), (2,5), (3,7)
    xs: list[int] = [0, 1, 2, 3]
    ys: list[int] = [1, 3, 5, 7]
    slope: int = regression_slope(xs, ys, scale)
    if abs_val(slope - 2000) < 10:
        passed = passed + 1

    # Test 2: intercept
    intercept: int = regression_intercept(xs, ys, slope, scale)
    if abs_val(intercept - 1000) < 10:
        passed = passed + 1

    # Test 3: prediction
    pred: int = predict(slope, intercept, 4, scale)
    if abs_val(pred - 9000) < 50:
        passed = passed + 1

    # Test 4: residuals near zero for perfect fit
    rss: int = residual_sum_squares(xs, ys, slope, intercept, scale)
    if rss < 10:
        passed = passed + 1

    # Test 5: constant function y = 5
    ys2: list[int] = [5, 5, 5, 5]
    slope2: int = regression_slope(xs, ys2, scale)
    if abs_val(slope2) < 10:
        passed = passed + 1

    # Test 6: negative slope y = -x + 10
    ys3: list[int] = [10, 9, 8, 7]
    slope3: int = regression_slope(xs, ys3, scale)
    if abs_val(slope3 + 1000) < 10:
        passed = passed + 1

    # Test 7: sum helpers
    if sum_list(xs) == 6 and sum_squares(xs) == 14:
        passed = passed + 1

    return passed
