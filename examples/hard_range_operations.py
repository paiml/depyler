"""Range operations: prefix sum for range sum query, brute-force range min query.

Tests: build_prefix_sum, range_sum, range_min, range_max.
"""


def build_prefix_sum(arr: list[int]) -> list[int]:
    """Build prefix sum array. prefix[i] = sum of arr[0..i-1]."""
    prefix: list[int] = [0]
    i: int = 0
    total: int = 0
    while i < len(arr):
        total = total + arr[i]
        prefix.append(total)
        i = i + 1
    return prefix


def range_sum(prefix: list[int], left: int, right: int) -> int:
    """Get sum of elements in range [left, right] using prefix sums."""
    return prefix[right + 1] - prefix[left]


def range_min(arr: list[int], left: int, right: int) -> int:
    """Find minimum element in range [left, right] (brute force)."""
    min_val: int = arr[left]
    i: int = left + 1
    while i <= right:
        if arr[i] < min_val:
            min_val = arr[i]
        i = i + 1
    return min_val


def range_max(arr: list[int], left: int, right: int) -> int:
    """Find maximum element in range [left, right] (brute force)."""
    max_val: int = arr[left]
    i: int = left + 1
    while i <= right:
        if arr[i] > max_val:
            max_val = arr[i]
        i = i + 1
    return max_val


def range_count(arr: list[int], left: int, right: int, target: int) -> int:
    """Count occurrences of target in range [left, right]."""
    count: int = 0
    i: int = left
    while i <= right:
        if arr[i] == target:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test range operations."""
    ok: int = 0

    arr: list[int] = [1, 3, 5, 7, 9, 11]
    prefix: list[int] = build_prefix_sum(arr)

    # sum of [1, 3, 5] = 9
    if range_sum(prefix, 0, 2) == 9:
        ok = ok + 1

    # sum of [5, 7, 9] = 21
    if range_sum(prefix, 2, 4) == 21:
        ok = ok + 1

    # sum of all = 36
    if range_sum(prefix, 0, 5) == 36:
        ok = ok + 1

    if range_min(arr, 1, 4) == 3:
        ok = ok + 1

    if range_max(arr, 1, 4) == 9:
        ok = ok + 1

    arr2: list[int] = [1, 2, 1, 3, 1]
    if range_count(arr2, 0, 4, 1) == 3:
        ok = ok + 1

    if range_count(arr2, 2, 4, 1) == 2:
        ok = ok + 1

    return ok
