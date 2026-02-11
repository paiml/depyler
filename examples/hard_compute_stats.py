"""Statistics: mean, median, mode, variance, std dev, percentile.

Tests: mean, median, mode, variance, std_dev, percentile, range.
"""


def stat_sum(arr: list[int]) -> int:
    """Sum of all elements."""
    total: int = 0
    i: int = 0
    n: int = len(arr)
    while i < n:
        total = total + arr[i]
        i = i + 1
    return total


def stat_mean_x100(arr: list[int]) -> int:
    """Mean * 100 (integer approximation)."""
    n: int = len(arr)
    if n == 0:
        return 0
    total: int = stat_sum(arr)
    return total * 100 // n


def stat_sorted(arr: list[int]) -> list[int]:
    """Return sorted copy of array."""
    result: list[int] = []
    i: int = 0
    n: int = len(arr)
    while i < n:
        result.append(arr[i])
        i = i + 1
    si: int = 0
    while si < n - 1:
        sj: int = si + 1
        while sj < n:
            if result[sj] < result[si]:
                tmp: int = result[si]
                result[si] = result[sj]
                result[sj] = tmp
            sj = sj + 1
        si = si + 1
    return result


def stat_median_x100(arr: list[int]) -> int:
    """Median * 100 (integer approximation)."""
    n: int = len(arr)
    if n == 0:
        return 0
    s: list[int] = stat_sorted(arr)
    if n % 2 == 1:
        return s[n // 2] * 100
    mid: int = n // 2
    return (s[mid - 1] + s[mid]) * 50


def stat_mode(arr: list[int]) -> int:
    """Mode (most frequent value). Returns first if tie."""
    n: int = len(arr)
    if n == 0:
        return 0
    best_val: int = arr[0]
    best_count: int = 0
    i: int = 0
    while i < n:
        count: int = 0
        j: int = 0
        while j < n:
            if arr[j] == arr[i]:
                count = count + 1
            j = j + 1
        if count > best_count:
            best_count = count
            best_val = arr[i]
        i = i + 1
    return best_val


def stat_variance_x100(arr: list[int]) -> int:
    """Population variance * 100 (integer approximation)."""
    n: int = len(arr)
    if n == 0:
        return 0
    mean_x100: int = stat_mean_x100(arr)
    sum_sq: int = 0
    i: int = 0
    while i < n:
        diff: int = arr[i] * 100 - mean_x100
        sum_sq = sum_sq + diff * diff
        i = i + 1
    return sum_sq // (n * 100)


def stat_range(arr: list[int]) -> int:
    """Range = max - min."""
    n: int = len(arr)
    if n == 0:
        return 0
    min_val: int = arr[0]
    max_val: int = arr[0]
    i: int = 1
    while i < n:
        if arr[i] < min_val:
            min_val = arr[i]
        if arr[i] > max_val:
            max_val = arr[i]
        i = i + 1
    return max_val - min_val


def stat_percentile(arr: list[int], p: int) -> int:
    """Percentile (0-100) using nearest rank method."""
    n: int = len(arr)
    if n == 0:
        return 0
    s: list[int] = stat_sorted(arr)
    rank: int = p * n // 100
    if rank >= n:
        rank = n - 1
    if rank < 0:
        rank = 0
    return s[rank]


def stat_min(arr: list[int]) -> int:
    """Minimum value."""
    n: int = len(arr)
    if n == 0:
        return 0
    result: int = arr[0]
    i: int = 1
    while i < n:
        if arr[i] < result:
            result = arr[i]
        i = i + 1
    return result


def stat_max(arr: list[int]) -> int:
    """Maximum value."""
    n: int = len(arr)
    if n == 0:
        return 0
    result: int = arr[0]
    i: int = 1
    while i < n:
        if arr[i] > result:
            result = arr[i]
        i = i + 1
    return result


def test_module() -> int:
    """Test statistics algorithms."""
    passed: int = 0

    data: list[int] = [4, 8, 6, 2, 10]

    if stat_sum(data) == 30:
        passed = passed + 1

    if stat_mean_x100(data) == 600:
        passed = passed + 1

    if stat_median_x100(data) == 600:
        passed = passed + 1

    data2: list[int] = [1, 2, 2, 3, 3, 3, 4]
    if stat_mode(data2) == 3:
        passed = passed + 1

    if stat_range(data) == 8:
        passed = passed + 1

    if stat_min(data) == 2:
        passed = passed + 1

    if stat_max(data) == 10:
        passed = passed + 1

    p50: int = stat_percentile(data, 50)
    if p50 == 6:
        passed = passed + 1

    return passed
