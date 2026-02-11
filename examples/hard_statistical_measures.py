"""Statistical measures: variance, standard deviation approx, percentile.

Tests: mean, variance, std_dev_approx, percentile (using integer arithmetic).
"""


def mean_times_100(arr: list[int]) -> int:
    """Compute mean * 100 for integer precision."""
    if len(arr) == 0:
        return 0
    total: int = 0
    i: int = 0
    while i < len(arr):
        total = total + arr[i]
        i = i + 1
    return (total * 100) // len(arr)


def variance_times_100(arr: list[int]) -> int:
    """Compute variance * 100 using integer arithmetic.
    
    variance = sum((x - mean)^2) / n, scaled by 100.
    """
    if len(arr) == 0:
        return 0
    n: int = len(arr)
    total: int = 0
    i: int = 0
    while i < n:
        total = total + arr[i]
        i = i + 1
    # Use sum of squares formula: var = E[X^2] - (E[X])^2
    sum_sq: int = 0
    i = 0
    while i < n:
        sum_sq = sum_sq + arr[i] * arr[i]
        i = i + 1
    # var * n^2 = n * sum_sq - total^2
    # var * 100 = (n * sum_sq - total^2) * 100 / n^2
    numerator: int = (n * sum_sq - total * total) * 100
    return numerator // (n * n)


def isqrt(n: int) -> int:
    """Integer square root using Newton's method."""
    if n < 0:
        return 0
    if n == 0:
        return 0
    x: int = n
    y: int = (x + 1) // 2
    while y < x:
        x = y
        y = (x + n // x) // 2
    return x


def std_dev_approx(arr: list[int]) -> int:
    """Approximate standard deviation as integer (floor of actual)."""
    v: int = variance_times_100(arr)
    # v is variance * 100, so sqrt(v/100) = sqrt(v)/10
    return isqrt(v * 100) // 100


def percentile(arr: list[int], p: int) -> int:
    """Compute p-th percentile using nearest-rank method. Requires sorted input."""
    if len(arr) == 0:
        return 0
    # Sort using insertion sort
    sorted_arr: list[int] = arr[:]
    i: int = 1
    while i < len(sorted_arr):
        key: int = sorted_arr[i]
        j: int = i - 1
        while j >= 0 and sorted_arr[j] > key:
            sorted_arr[j + 1] = sorted_arr[j]
            j = j - 1
        sorted_arr[j + 1] = key
        i = i + 1
    rank: int = (p * len(sorted_arr) + 99) // 100
    if rank < 1:
        rank = 1
    if rank > len(sorted_arr):
        rank = len(sorted_arr)
    return sorted_arr[rank - 1]


def median(arr: list[int]) -> int:
    """Compute median (integer, lower median for even length)."""
    sorted_arr: list[int] = arr[:]
    i: int = 1
    while i < len(sorted_arr):
        key: int = sorted_arr[i]
        j: int = i - 1
        while j >= 0 and sorted_arr[j] > key:
            sorted_arr[j + 1] = sorted_arr[j]
            j = j - 1
        sorted_arr[j + 1] = key
        i = i + 1
    n: int = len(sorted_arr)
    if n == 0:
        return 0
    return sorted_arr[(n - 1) // 2]


def test_module() -> int:
    """Test statistical measures."""
    ok: int = 0

    # mean of [2, 4, 4, 4, 5, 5, 7, 9] = 5.0 => 500
    if mean_times_100([2, 4, 4, 4, 5, 5, 7, 9]) == 500:
        ok = ok + 1

    if mean_times_100([]) == 0:
        ok = ok + 1

    # variance of [2, 4, 4, 4, 5, 5, 7, 9] = 4.0 => 400
    if variance_times_100([2, 4, 4, 4, 5, 5, 7, 9]) == 400:
        ok = ok + 1

    if isqrt(16) == 4:
        ok = ok + 1

    if isqrt(0) == 0:
        ok = ok + 1

    if median([3, 1, 2]) == 2:
        ok = ok + 1

    if percentile([15, 20, 35, 40, 50], 50) == 35:
        ok = ok + 1

    return ok
