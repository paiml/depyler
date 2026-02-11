"""Binary search variants.

Tests: lower bound, upper bound, first occurrence, last occurrence, count.
"""


def lower_bound(arr: list[int], target: int) -> int:
    """Find first index where arr[i] >= target."""
    lo: int = 0
    hi: int = len(arr)
    while lo < hi:
        mid: int = lo + (hi - lo) // 2
        if arr[mid] < target:
            lo = mid + 1
        else:
            hi = mid
    return lo


def upper_bound(arr: list[int], target: int) -> int:
    """Find first index where arr[i] > target."""
    lo: int = 0
    hi: int = len(arr)
    while lo < hi:
        mid: int = lo + (hi - lo) // 2
        if arr[mid] <= target:
            lo = mid + 1
        else:
            hi = mid
    return lo


def count_occurrences(arr: list[int], target: int) -> int:
    """Count occurrences of target in sorted array."""
    return upper_bound(arr, target) - lower_bound(arr, target)


def first_occurrence(arr: list[int], target: int) -> int:
    """Find first occurrence of target. Returns -1 if not found."""
    idx: int = lower_bound(arr, target)
    if idx < len(arr) and arr[idx] == target:
        return idx
    return -1


def search_insert_position(arr: list[int], target: int) -> int:
    """Find position where target should be inserted to keep sorted order."""
    return lower_bound(arr, target)


def test_module() -> int:
    """Test binary search variants."""
    ok: int = 0
    arr: list[int] = [1, 2, 2, 3, 3, 3, 4, 5]
    if lower_bound(arr, 3) == 3:
        ok = ok + 1
    if upper_bound(arr, 3) == 6:
        ok = ok + 1
    if count_occurrences(arr, 3) == 3:
        ok = ok + 1
    if count_occurrences(arr, 2) == 2:
        ok = ok + 1
    if first_occurrence(arr, 3) == 3:
        ok = ok + 1
    if first_occurrence(arr, 6) == -1:
        ok = ok + 1
    if search_insert_position(arr, 0) == 0:
        ok = ok + 1
    if search_insert_position(arr, 6) == 8:
        ok = ok + 1
    return ok
