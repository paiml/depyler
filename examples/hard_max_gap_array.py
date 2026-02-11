"""Maximum gap and spacing operations in arrays.

Tests: max gap, min gap, average gap, gap count above threshold.
"""


def max_gap(arr: list[int]) -> int:
    """Maximum difference between consecutive elements in sorted order."""
    n: int = len(arr)
    if n < 2:
        return 0
    sorted_arr: list[int] = []
    for v in arr:
        sorted_arr.append(v)
    i: int = 0
    while i < n - 1:
        j: int = 0
        while j < n - 1 - i:
            if sorted_arr[j] > sorted_arr[j + 1]:
                tmp: int = sorted_arr[j]
                sorted_arr[j] = sorted_arr[j + 1]
                sorted_arr[j + 1] = tmp
            j = j + 1
        i = i + 1
    best: int = 0
    i = 0
    while i < n - 1:
        gap: int = sorted_arr[i + 1] - sorted_arr[i]
        if gap > best:
            best = gap
        i = i + 1
    return best


def min_gap(arr: list[int]) -> int:
    """Minimum absolute difference between any two elements."""
    n: int = len(arr)
    if n < 2:
        return 0
    sorted_arr: list[int] = []
    for v in arr:
        sorted_arr.append(v)
    i: int = 0
    while i < n - 1:
        j: int = 0
        while j < n - 1 - i:
            if sorted_arr[j] > sorted_arr[j + 1]:
                tmp: int = sorted_arr[j]
                sorted_arr[j] = sorted_arr[j + 1]
                sorted_arr[j + 1] = tmp
            j = j + 1
        i = i + 1
    best: int = sorted_arr[1] - sorted_arr[0]
    i = 1
    while i < n - 1:
        gap: int = sorted_arr[i + 1] - sorted_arr[i]
        if gap < best:
            best = gap
        i = i + 1
    return best


def count_gaps_above(arr: list[int], threshold: int) -> int:
    """Count consecutive pairs with gap > threshold (in original order)."""
    n: int = len(arr)
    count: int = 0
    i: int = 0
    while i < n - 1:
        gap: int = arr[i + 1] - arr[i]
        if gap < 0:
            gap = -gap
        if gap > threshold:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test max gap operations."""
    ok: int = 0
    if max_gap([3, 6, 9, 1]) == 3:
        ok = ok + 1
    if max_gap([10]) == 0:
        ok = ok + 1
    if min_gap([1, 5, 3, 19, 18]) == 1:
        ok = ok + 1
    if count_gaps_above([1, 10, 2, 20], 5) == 3:
        ok = ok + 1
    if count_gaps_above([1, 2, 3], 5) == 0:
        ok = ok + 1
    return ok
