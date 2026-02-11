"""Find missing positive integers in arrays.

Tests: first missing positive, missing in range, count missing, kth missing.
"""


def first_missing_positive(arr: list[int]) -> int:
    """Find smallest positive integer not in array."""
    n: int = len(arr)
    candidate: int = 1
    while candidate <= n + 1:
        found: int = 0
        for v in arr:
            if v == candidate:
                found = 1
        if found == 0:
            return candidate
        candidate = candidate + 1
    return candidate


def count_missing_in_range(arr: list[int], lo: int, hi: int) -> int:
    """Count integers in [lo, hi] not present in arr."""
    count: int = 0
    val: int = lo
    while val <= hi:
        found: int = 0
        for v in arr:
            if v == val:
                found = 1
        if found == 0:
            count = count + 1
        val = val + 1
    return count


def kth_missing_positive(arr: list[int], k: int) -> int:
    """Find the kth missing positive integer."""
    missing_count: int = 0
    candidate: int = 1
    while missing_count < k:
        found: int = 0
        for v in arr:
            if v == candidate:
                found = 1
        if found == 0:
            missing_count = missing_count + 1
            if missing_count == k:
                return candidate
        candidate = candidate + 1
    return candidate - 1


def sum_missing_in_range(arr: list[int], lo: int, hi: int) -> int:
    """Sum of integers in [lo, hi] not present in arr."""
    total: int = 0
    val: int = lo
    while val <= hi:
        found: int = 0
        for v in arr:
            if v == val:
                found = 1
        if found == 0:
            total = total + val
        val = val + 1
    return total


def test_module() -> int:
    """Test missing positive."""
    ok: int = 0
    if first_missing_positive([3, 4, -1, 1]) == 2:
        ok = ok + 1
    if first_missing_positive([1, 2, 3]) == 4:
        ok = ok + 1
    if count_missing_in_range([2, 4, 6], 1, 6) == 3:
        ok = ok + 1
    if kth_missing_positive([2, 3, 4, 7, 11], 1) == 1:
        ok = ok + 1
    if kth_missing_positive([2, 3, 4, 7, 11], 5) == 9:
        ok = ok + 1
    if sum_missing_in_range([1, 3, 5], 1, 5) == 6:
        ok = ok + 1
    return ok
