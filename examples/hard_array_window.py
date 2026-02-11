"""Sliding window operations on arrays: max, min, sum, and average."""


def sliding_window_sum(arr: list[int], window_size: int) -> list[int]:
    """Calculate sum for each sliding window position."""
    n: int = len(arr)
    result: list[int] = []
    if window_size <= 0 or window_size > n:
        return result
    current_sum: int = 0
    i: int = 0
    while i < window_size:
        current_sum = current_sum + arr[i]
        i = i + 1
    result.append(current_sum)
    j: int = window_size
    while j < n:
        old_idx: int = j - window_size
        current_sum = current_sum + arr[j] - arr[old_idx]
        result.append(current_sum)
        j = j + 1
    return result


def sliding_window_max(arr: list[int], window_size: int) -> list[int]:
    """Find maximum in each sliding window position."""
    n: int = len(arr)
    result: list[int] = []
    if window_size <= 0 or window_size > n:
        return result
    i: int = 0
    while i <= n - window_size:
        max_val: int = arr[i]
        j: int = i + 1
        limit: int = i + window_size
        while j < limit:
            if arr[j] > max_val:
                max_val = arr[j]
            j = j + 1
        result.append(max_val)
        i = i + 1
    return result


def sliding_window_min(arr: list[int], window_size: int) -> list[int]:
    """Find minimum in each sliding window position."""
    n: int = len(arr)
    result: list[int] = []
    if window_size <= 0 or window_size > n:
        return result
    i: int = 0
    while i <= n - window_size:
        min_val: int = arr[i]
        j: int = i + 1
        limit: int = i + window_size
        while j < limit:
            if arr[j] < min_val:
                min_val = arr[j]
            j = j + 1
        result.append(min_val)
        i = i + 1
    return result


def max_sum_window(arr: list[int], window_size: int) -> int:
    """Find the maximum sum among all windows of given size."""
    sums: list[int] = sliding_window_sum(arr, window_size)
    if len(sums) == 0:
        return 0
    best: int = sums[0]
    i: int = 1
    while i < len(sums):
        if sums[i] > best:
            best = sums[i]
        i = i + 1
    return best


def test_module() -> int:
    """Test sliding window operations."""
    ok: int = 0

    arr: list[int] = [1, 3, 2, 5, 1, 4]
    sums: list[int] = sliding_window_sum(arr, 3)
    if sums[0] == 6 and sums[1] == 10 and sums[2] == 8 and sums[3] == 10:
        ok = ok + 1

    maxes: list[int] = sliding_window_max(arr, 3)
    if maxes[0] == 3 and maxes[1] == 5 and maxes[2] == 5 and maxes[3] == 5:
        ok = ok + 1

    mins: list[int] = sliding_window_min(arr, 3)
    if mins[0] == 1 and mins[1] == 2 and mins[2] == 1 and mins[3] == 1:
        ok = ok + 1

    if max_sum_window(arr, 3) == 10:
        ok = ok + 1

    empty_result: list[int] = sliding_window_sum(arr, 0)
    if len(empty_result) == 0:
        ok = ok + 1

    single: list[int] = sliding_window_sum(arr, 1)
    if single[0] == 1 and single[1] == 3:
        ok = ok + 1

    return ok
