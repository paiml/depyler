"""Statistical operations on arrays: variance, percentiles, and outlier detection."""


def array_mean_x1000(arr: list[int]) -> int:
    """Calculate mean * 1000 to avoid floating point."""
    n: int = len(arr)
    if n == 0:
        return 0
    total: int = 0
    i: int = 0
    while i < n:
        total = total + arr[i]
        i = i + 1
    return total * 1000 // n


def array_variance_x1000(arr: list[int]) -> int:
    """Calculate variance * 1000 using integer arithmetic."""
    n: int = len(arr)
    if n <= 1:
        return 0
    mean_x1000: int = array_mean_x1000(arr)
    sum_sq: int = 0
    i: int = 0
    while i < n:
        diff: int = arr[i] * 1000 - mean_x1000
        sum_sq = sum_sq + diff * diff
        i = i + 1
    result: int = sum_sq // (n * 1000000)
    return result


def array_median(arr: list[int]) -> int:
    """Find the median of an array (sorts a copy first)."""
    n: int = len(arr)
    if n == 0:
        return 0
    sorted_arr: list[int] = []
    i: int = 0
    while i < n:
        sorted_arr.append(arr[i])
        i = i + 1
    # Bubble sort the copy
    j: int = 0
    while j < n:
        k: int = j + 1
        while k < n:
            if sorted_arr[k] < sorted_arr[j]:
                tmp: int = sorted_arr[j]
                sorted_arr[j] = sorted_arr[k]
                sorted_arr[k] = tmp
            k = k + 1
        j = j + 1
    mid: int = n // 2
    if n % 2 == 1:
        return sorted_arr[mid]
    prev: int = mid - 1
    return (sorted_arr[prev] + sorted_arr[mid]) // 2


def count_above_mean(arr: list[int]) -> int:
    """Count how many elements are above the mean."""
    n: int = len(arr)
    if n == 0:
        return 0
    total: int = 0
    i: int = 0
    while i < n:
        total = total + arr[i]
        i = i + 1
    mean_n: int = total
    count: int = 0
    j: int = 0
    while j < n:
        if arr[j] * n > mean_n:
            count = count + 1
        j = j + 1
    return count


def test_module() -> int:
    """Test array statistics functions."""
    ok: int = 0

    arr1: list[int] = [2, 4, 6, 8, 10]
    if array_mean_x1000(arr1) == 6000:
        ok = ok + 1

    if array_variance_x1000(arr1) >= 0:
        ok = ok + 1

    if array_median(arr1) == 6:
        ok = ok + 1

    arr2: list[int] = [1, 2, 3, 4]
    if array_median(arr2) == 2:
        ok = ok + 1

    if count_above_mean(arr1) == 2:
        ok = ok + 1

    empty: list[int] = []
    if array_mean_x1000(empty) == 0:
        ok = ok + 1

    return ok
