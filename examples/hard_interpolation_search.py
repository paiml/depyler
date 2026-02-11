"""Interpolation search for uniformly distributed sorted data."""


def interpolation_search(arr: list[int], target: int) -> int:
    """Search using interpolation of value positions. Returns index or -1."""
    lo: int = 0
    hi: int = len(arr) - 1
    while lo <= hi and target >= arr[lo] and target <= arr[hi]:
        if lo == hi:
            if arr[lo] == target:
                return lo
            return -1
        denom: int = arr[hi] - arr[lo]
        if denom == 0:
            if arr[lo] == target:
                return lo
            return -1
        pos: int = lo + ((target - arr[lo]) * (hi - lo)) // denom
        if pos < lo:
            pos = lo
        if pos > hi:
            pos = hi
        if arr[pos] == target:
            return pos
        if arr[pos] < target:
            lo = pos + 1
        else:
            hi = pos - 1
    return -1


def interpolation_count(arr: list[int], target: int) -> int:
    """Count occurrences using interpolation search to find one, then expand."""
    idx: int = interpolation_search(arr, target)
    if idx == -1:
        return 0
    count: int = 1
    left: int = idx - 1
    while left >= 0 and arr[left] == target:
        count = count + 1
        left = left - 1
    right: int = idx + 1
    sz: int = len(arr)
    while right < sz and arr[right] == target:
        count = count + 1
        right = right + 1
    return count


def interpolation_range(arr: list[int], low_val: int, high_val: int) -> int:
    """Count elements in [low_val, high_val] range."""
    n: int = len(arr)
    start: int = 0
    while start < n and arr[start] < low_val:
        start = start + 1
    count: int = 0
    idx: int = start
    while idx < n and arr[idx] <= high_val:
        count = count + 1
        idx = idx + 1
    return count


def test_module() -> int:
    passed: int = 0

    arr1: list[int] = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]
    if interpolation_search(arr1, 50) == 4:
        passed = passed + 1

    if interpolation_search(arr1, 10) == 0:
        passed = passed + 1

    if interpolation_search(arr1, 55) == -1:
        passed = passed + 1

    arr2: list[int] = [1, 2, 2, 2, 3, 4, 5]
    cnt: int = interpolation_count(arr2, 2)
    if cnt == 3:
        passed = passed + 1

    if interpolation_count(arr2, 6) == 0:
        passed = passed + 1

    arr3: list[int] = [1, 3, 5, 7, 9, 11, 13, 15]
    rng: int = interpolation_range(arr3, 5, 11)
    if rng == 4:
        passed = passed + 1

    if interpolation_search(arr1, 100) == 9:
        passed = passed + 1

    if interpolation_range(arr3, 20, 30) == 0:
        passed = passed + 1

    return passed
