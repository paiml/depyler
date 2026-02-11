"""Exponential search combining boundary doubling with binary search."""


def binary_search_range(arr: list[int], target: int, lo: int, hi: int) -> int:
    """Binary search within [lo, hi]. Returns index or -1."""
    while lo <= hi:
        mid: int = lo + (hi - lo) // 2
        if arr[mid] == target:
            return mid
        if arr[mid] < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return -1


def exponential_search(arr: list[int], target: int) -> int:
    """Search by doubling range then binary search. Returns index or -1."""
    n: int = len(arr)
    if n == 0:
        return -1
    if arr[0] == target:
        return 0
    bound: int = 1
    while bound < n and arr[bound] <= target:
        bound = bound * 2
    lo: int = bound // 2
    hi: int = bound
    if hi >= n:
        hi = n - 1
    result: int = binary_search_range(arr, target, lo, hi)
    return result


def exponential_lower_bound(arr: list[int], target: int) -> int:
    """Find first position >= target using exponential + binary approach."""
    n: int = len(arr)
    if n == 0:
        return 0
    if arr[0] >= target:
        return 0
    bound: int = 1
    while bound < n and arr[bound] < target:
        bound = bound * 2
    lo: int = bound // 2
    hi: int = bound
    if hi >= n:
        hi = n - 1
    while lo < hi:
        mid: int = lo + (hi - lo) // 2
        if arr[mid] < target:
            lo = mid + 1
        else:
            hi = mid
    if lo < n and arr[lo] >= target:
        return lo
    return n


def unbounded_search(arr: list[int], target: int) -> int:
    """Search in a conceptually unbounded sorted array (use actual length)."""
    n: int = len(arr)
    if n == 0:
        return -1
    idx: int = 0
    step: int = 1
    while idx < n and arr[idx] < target:
        idx = idx + step
        step = step * 2
    lo: int = idx // 2
    if lo == 0 and idx > 0:
        lo = 0
    hi: int = idx
    if hi >= n:
        hi = n - 1
    result: int = binary_search_range(arr, target, lo, hi)
    return result


def test_module() -> int:
    passed: int = 0

    arr1: list[int] = [2, 4, 6, 8, 10, 12, 14, 16, 18, 20]
    if exponential_search(arr1, 14) == 6:
        passed = passed + 1

    if exponential_search(arr1, 2) == 0:
        passed = passed + 1

    if exponential_search(arr1, 5) == -1:
        passed = passed + 1

    lb: int = exponential_lower_bound(arr1, 9)
    if lb == 4:
        passed = passed + 1

    if exponential_lower_bound(arr1, 1) == 0:
        passed = passed + 1

    if unbounded_search(arr1, 18) == 8:
        passed = passed + 1

    empty: list[int] = []
    if exponential_search(empty, 5) == -1:
        passed = passed + 1

    if exponential_lower_bound(arr1, 25) == 10:
        passed = passed + 1

    return passed
