"""Range query operations using prefix sums.

Tests: prefix sum, range sum, range min, range max.
"""


def build_prefix_sum(arr: list[int]) -> list[int]:
    """Build prefix sum array."""
    n: int = len(arr)
    prefix: list[int] = [0] * (n + 1)
    i: int = 0
    while i < n:
        prefix[i + 1] = prefix[i] + arr[i]
        i = i + 1
    return prefix


def range_sum(prefix: list[int], left: int, right: int) -> int:
    """Sum of elements from left to right (inclusive) using prefix sums."""
    return prefix[right + 1] - prefix[left]


def range_min_brute(arr: list[int], left: int, right: int) -> int:
    """Find minimum in range [left, right] by brute force."""
    best: int = arr[left]
    i: int = left + 1
    while i <= right:
        if arr[i] < best:
            best = arr[i]
        i = i + 1
    return best


def range_max_brute(arr: list[int], left: int, right: int) -> int:
    """Find maximum in range [left, right] by brute force."""
    best: int = arr[left]
    i: int = left + 1
    while i <= right:
        if arr[i] > best:
            best = arr[i]
        i = i + 1
    return best


def count_in_range(arr: list[int], lo: int, hi: int) -> int:
    """Count elements in [lo, hi]."""
    count: int = 0
    i: int = 0
    while i < len(arr):
        if arr[i] >= lo and arr[i] <= hi:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test range query operations."""
    ok: int = 0
    arr: list[int] = [1, 3, 5, 7, 9, 2, 4]
    prefix: list[int] = build_prefix_sum(arr)
    if range_sum(prefix, 0, 2) == 9:
        ok = ok + 1
    if range_sum(prefix, 1, 4) == 24:
        ok = ok + 1
    if range_min_brute(arr, 0, 3) == 1:
        ok = ok + 1
    if range_min_brute(arr, 4, 6) == 2:
        ok = ok + 1
    if range_max_brute(arr, 0, 6) == 9:
        ok = ok + 1
    if count_in_range(arr, 3, 7) == 3:
        ok = ok + 1
    if count_in_range(arr, 10, 20) == 0:
        ok = ok + 1
    return ok
