# Pearson correlation coefficient using integer-scaled arithmetic


def sum_list(arr: list[int]) -> int:
    total: int = 0
    i: int = 0
    while i < len(arr):
        total = total + arr[i]
        i = i + 1
    return total


def mean_scaled(arr: list[int], scale: int) -> int:
    if len(arr) == 0:
        return 0
    return sum_list(arr) * scale // len(arr)


def covariance_scaled(xs: list[int], ys: list[int], scale: int) -> int:
    n: int = len(xs)
    if n == 0:
        return 0
    mx: int = mean_scaled(xs, scale)
    my: int = mean_scaled(ys, scale)
    total: int = 0
    i: int = 0
    while i < n:
        dx: int = xs[i] * scale - mx
        dy: int = ys[i] * scale - my
        total = total + dx * dy // scale
        i = i + 1
    return total // n


def variance_scaled(arr: list[int], scale: int) -> int:
    n: int = len(arr)
    if n == 0:
        return 0
    m: int = mean_scaled(arr, scale)
    total: int = 0
    i: int = 0
    while i < n:
        d: int = arr[i] * scale - m
        total = total + d * d // scale
        i = i + 1
    return total // n


def isqrt(n: int) -> int:
    if n <= 0:
        return 0
    x: int = n
    y: int = (x + 1) // 2
    while y < x:
        x = y
        y = (x + n // x) // 2
    return x


def pearson_correlation(xs: list[int], ys: list[int], scale: int) -> int:
    # Returns correlation * scale
    cov: int = covariance_scaled(xs, ys, scale)
    vx: int = variance_scaled(xs, scale)
    vy: int = variance_scaled(ys, scale)
    if vx == 0 or vy == 0:
        return 0
    denom: int = isqrt(vx * vy)
    if denom == 0:
        return 0
    return cov * scale // denom


def abs_val(x: int) -> int:
    if x < 0:
        return -x
    return x


def test_module() -> int:
    passed: int = 0
    scale: int = 1000

    # Test 1: perfect positive correlation
    xs: list[int] = [1, 2, 3, 4, 5]
    ys: list[int] = [2, 4, 6, 8, 10]
    r: int = pearson_correlation(xs, ys, scale)
    if abs_val(r - 1000) < 50:
        passed = passed + 1

    # Test 2: perfect negative correlation
    ys_neg: list[int] = [10, 8, 6, 4, 2]
    r = pearson_correlation(xs, ys_neg, scale)
    if abs_val(r + 1000) < 50:
        passed = passed + 1

    # Test 3: zero correlation (constant y)
    ys_const: list[int] = [5, 5, 5, 5, 5]
    r = pearson_correlation(xs, ys_const, scale)
    if r == 0:
        passed = passed + 1

    # Test 4: mean scaled
    m: int = mean_scaled(xs, scale)
    if m == 3000:
        passed = passed + 1

    # Test 5: covariance of identical = variance
    cov: int = covariance_scaled(xs, xs, scale)
    var: int = variance_scaled(xs, scale)
    if abs_val(cov - var) < 50:
        passed = passed + 1

    # Test 6: variance of constant = 0
    if variance_scaled(ys_const, scale) == 0:
        passed = passed + 1

    # Test 7: isqrt correctness
    if isqrt(100) == 10 and isqrt(81) == 9:
        passed = passed + 1

    return passed
