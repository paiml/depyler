"""Binary search variants: upper bound, lower bound, rotated array search."""


def lower_bound(arr: list[int], target: int) -> int:
    """Find first index where arr[index] >= target."""
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
    """Find first index where arr[index] > target."""
    lo: int = 0
    hi: int = len(arr)
    while lo < hi:
        mid: int = lo + (hi - lo) // 2
        if arr[mid] <= target:
            lo = mid + 1
        else:
            hi = mid
    return lo


def search_rotated(arr: list[int], target: int) -> int:
    """Search in a rotated sorted array. Returns index or -1."""
    lo: int = 0
    hi: int = len(arr) - 1
    while lo <= hi:
        mid: int = lo + (hi - lo) // 2
        if arr[mid] == target:
            return mid
        if arr[lo] <= arr[mid]:
            if arr[lo] <= target < arr[mid]:
                hi = mid - 1
            else:
                lo = mid + 1
        else:
            if arr[mid] < target <= arr[hi]:
                lo = mid + 1
            else:
                hi = mid - 1
    return -1


def count_occurrences(arr: list[int], target: int) -> int:
    """Count how many times target appears using bounds."""
    lb: int = lower_bound(arr, target)
    ub: int = upper_bound(arr, target)
    return ub - lb


def test_module() -> int:
    passed: int = 0

    arr1: list[int] = [1, 2, 3, 4, 5, 6, 7, 8]
    if lower_bound(arr1, 4) == 3:
        passed = passed + 1
    if upper_bound(arr1, 4) == 4:
        passed = passed + 1

    arr2: list[int] = [4, 5, 6, 7, 0, 1, 2]
    if search_rotated(arr2, 0) == 4:
        passed = passed + 1
    if search_rotated(arr2, 3) == -1:
        passed = passed + 1

    arr3: list[int] = [1, 2, 2, 2, 3, 4]
    if count_occurrences(arr3, 2) == 3:
        passed = passed + 1

    if lower_bound(arr1, 9) == 8:
        passed = passed + 1

    arr4: list[int] = [3, 4, 5, 1, 2]
    if search_rotated(arr4, 5) == 2:
        passed = passed + 1

    if count_occurrences(arr3, 5) == 0:
        passed = passed + 1

    return passed
