"""Real-world statistics library.

Mimics: statistics module, numpy basic stats, pandas describe().
Implements mean, median, variance, std dev approximation, percentiles.
"""


def stat_sum(data: list[int]) -> int:
    """Sum all values."""
    total: int = 0
    idx: int = 0
    while idx < len(data):
        total = total + data[idx]
        idx = idx + 1
    return total


def stat_mean_x100(data: list[int]) -> int:
    """Compute mean * 100 (to preserve precision with integers)."""
    if len(data) == 0:
        return 0
    return (stat_sum(data) * 100) // len(data)


def stat_min(data: list[int]) -> int:
    """Find minimum value."""
    if len(data) == 0:
        return 0
    result: int = data[0]
    idx: int = 1
    while idx < len(data):
        if data[idx] < result:
            result = data[idx]
        idx = idx + 1
    return result


def stat_max(data: list[int]) -> int:
    """Find maximum value."""
    if len(data) == 0:
        return 0
    result: int = data[0]
    idx: int = 1
    while idx < len(data):
        if data[idx] > result:
            result = data[idx]
        idx = idx + 1
    return result


def stat_sorted(data: list[int]) -> list[int]:
    """Return sorted copy of data (insertion sort)."""
    result: list[int] = []
    idx: int = 0
    while idx < len(data):
        result.append(data[idx])
        idx = idx + 1
    i: int = 1
    while i < len(result):
        val: int = result[i]
        j: int = i - 1
        while j >= 0 and result[j] > val:
            result[j + 1] = result[j]
            j = j - 1
        result[j + 1] = val
        i = i + 1
    return result


def stat_median(data: list[int]) -> int:
    """Compute median (integer). For even length, average of two middle values."""
    if len(data) == 0:
        return 0
    s: list[int] = stat_sorted(data)
    n: int = len(s)
    if n % 2 == 1:
        return s[n // 2]
    mid: int = n // 2
    return (s[mid - 1] + s[mid]) // 2


def stat_variance_x100(data: list[int]) -> int:
    """Compute variance * 100 for precision. Uses sum of squared differences from mean."""
    if len(data) <= 1:
        return 0
    mean100: int = stat_mean_x100(data)
    sum_sq: int = 0
    idx: int = 0
    while idx < len(data):
        diff: int = data[idx] * 100 - mean100
        sum_sq = sum_sq + diff * diff
        idx = idx + 1
    return sum_sq // (len(data) * 100)


def stat_range(data: list[int]) -> int:
    """Compute range (max - min)."""
    return stat_max(data) - stat_min(data)


def stat_percentile(data: list[int], pct: int) -> int:
    """Compute percentile value (nearest rank method). pct is 0-100."""
    if len(data) == 0:
        return 0
    s: list[int] = stat_sorted(data)
    rank: int = (pct * len(s)) // 100
    if rank >= len(s):
        rank = len(s) - 1
    if rank < 0:
        rank = 0
    return s[rank]


def stat_correlation_x1000(x_data: list[int], y_data: list[int]) -> int:
    """Compute Pearson correlation * 1000 (integer approximation).
    Returns value between -1000 and 1000."""
    n: int = len(x_data)
    if n != len(y_data) or n <= 1:
        return 0
    sum_x: int = stat_sum(x_data)
    sum_y: int = stat_sum(y_data)
    sum_xy: int = 0
    sum_xx: int = 0
    sum_yy: int = 0
    idx: int = 0
    while idx < n:
        sum_xy = sum_xy + x_data[idx] * y_data[idx]
        sum_xx = sum_xx + x_data[idx] * x_data[idx]
        sum_yy = sum_yy + y_data[idx] * y_data[idx]
        idx = idx + 1
    numerator: int = n * sum_xy - sum_x * sum_y
    denom_x: int = n * sum_xx - sum_x * sum_x
    denom_y: int = n * sum_yy - sum_y * sum_y
    if denom_x <= 0 or denom_y <= 0:
        return 0
    # Integer sqrt approximation
    product: int = denom_x * denom_y
    sqrt_approx: int = 1
    while sqrt_approx * sqrt_approx < product:
        sqrt_approx = sqrt_approx + 1
    return (numerator * 1000) // sqrt_approx


def test_module() -> int:
    """Test statistics module."""
    passed: int = 0

    data: list[int] = [4, 8, 15, 16, 23, 42]

    # Test 1: sum
    if stat_sum(data) == 108:
        passed = passed + 1

    # Test 2: mean * 100
    m: int = stat_mean_x100(data)
    if m == 1800:
        passed = passed + 1

    # Test 3: min and max
    if stat_min(data) == 4 and stat_max(data) == 42:
        passed = passed + 1

    # Test 4: median (even count)
    med: int = stat_median(data)
    if med == 15:
        passed = passed + 1

    # Test 5: median (odd count)
    odd_data: list[int] = [1, 3, 5, 7, 9]
    if stat_median(odd_data) == 5:
        passed = passed + 1

    # Test 6: range
    if stat_range(data) == 38:
        passed = passed + 1

    # Test 7: percentile
    p50: int = stat_percentile(data, 50)
    if p50 == 15 or p50 == 16:
        passed = passed + 1

    # Test 8: correlation (perfect positive)
    x: list[int] = [1, 2, 3, 4, 5]
    y: list[int] = [2, 4, 6, 8, 10]
    corr: int = stat_correlation_x1000(x, y)
    if corr >= 900:
        passed = passed + 1

    return passed
