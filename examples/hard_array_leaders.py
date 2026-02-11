"""Array leaders, peaks, and valleys operations.

Tests: find leaders, count peaks, count valleys.
"""


def find_leaders(arr: list[int]) -> list[int]:
    """Find leaders: elements greater than all to their right."""
    n: int = len(arr)
    if n == 0:
        return []
    leaders: list[int] = []
    max_from_right: int = arr[n - 1]
    leaders.append(arr[n - 1])
    i: int = n - 2
    while i >= 0:
        if arr[i] > max_from_right:
            leaders.append(arr[i])
            max_from_right = arr[i]
        i = i - 1
    result: list[int] = []
    j: int = len(leaders) - 1
    while j >= 0:
        result.append(leaders[j])
        j = j - 1
    return result


def count_peaks(arr: list[int]) -> int:
    """Count peaks: elements greater than both neighbors."""
    n: int = len(arr)
    if n < 3:
        return 0
    count: int = 0
    i: int = 1
    while i < n - 1:
        if arr[i] > arr[i - 1]:
            if arr[i] > arr[i + 1]:
                count = count + 1
        i = i + 1
    return count


def count_valleys(arr: list[int]) -> int:
    """Count valleys: elements less than both neighbors."""
    n: int = len(arr)
    if n < 3:
        return 0
    count: int = 0
    i: int = 1
    while i < n - 1:
        if arr[i] < arr[i - 1]:
            if arr[i] < arr[i + 1]:
                count = count + 1
        i = i + 1
    return count


def max_difference(arr: list[int]) -> int:
    """Find maximum difference arr[j] - arr[i] where j > i."""
    n: int = len(arr)
    if n < 2:
        return 0
    min_val: int = arr[0]
    max_diff: int = arr[1] - arr[0]
    i: int = 1
    while i < n:
        diff: int = arr[i] - min_val
        if diff > max_diff:
            max_diff = diff
        if arr[i] < min_val:
            min_val = arr[i]
        i = i + 1
    return max_diff


def test_module() -> int:
    """Test array leader operations."""
    ok: int = 0
    arr: list[int] = [16, 17, 4, 3, 5, 2]
    leaders: list[int] = find_leaders(arr)
    if leaders[0] == 17:
        ok = ok + 1
    if leaders[1] == 5:
        ok = ok + 1
    if leaders[2] == 2:
        ok = ok + 1
    wave: list[int] = [1, 3, 2, 4, 1, 5, 2]
    if count_peaks(wave) == 2:
        ok = ok + 1
    if count_valleys(wave) == 2:
        ok = ok + 1
    if max_difference([2, 3, 10, 6, 4, 8, 1]) == 8:
        ok = ok + 1
    return ok
