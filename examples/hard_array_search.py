"""Binary search variants: first/last occurrence, floor, ceil."""


def binary_search(arr: list[int], target: int) -> int:
    """Standard binary search returning index or -1."""
    left: int = 0
    right: int = len(arr) - 1
    while left <= right:
        mid: int = left + (right - left) // 2
        if arr[mid] == target:
            return mid
        if arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    return -1


def first_occurrence(arr: list[int], target: int) -> int:
    """Find index of first occurrence of target."""
    left: int = 0
    right: int = len(arr) - 1
    result: int = -1
    while left <= right:
        mid: int = left + (right - left) // 2
        if arr[mid] == target:
            result = mid
            right = mid - 1
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    return result


def last_occurrence(arr: list[int], target: int) -> int:
    """Find index of last occurrence of target."""
    left: int = 0
    right: int = len(arr) - 1
    result: int = -1
    while left <= right:
        mid: int = left + (right - left) // 2
        if arr[mid] == target:
            result = mid
            left = mid + 1
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    return result


def floor_value(arr: list[int], target: int) -> int:
    """Find largest element <= target. Return -1 if none."""
    left: int = 0
    right: int = len(arr) - 1
    result: int = -1
    while left <= right:
        mid: int = left + (right - left) // 2
        if arr[mid] <= target:
            result = arr[mid]
            left = mid + 1
        else:
            right = mid - 1
    return result


def ceil_value(arr: list[int], target: int) -> int:
    """Find smallest element >= target. Return -1 if none."""
    left: int = 0
    right: int = len(arr) - 1
    result: int = -1
    while left <= right:
        mid: int = left + (right - left) // 2
        if arr[mid] >= target:
            result = arr[mid]
            right = mid - 1
        else:
            left = mid + 1
    return result


def test_module() -> int:
    passed: int = 0
    arr: list[int] = [1, 2, 4, 4, 4, 7, 9]

    if binary_search(arr, 4) >= 2:
        passed = passed + 1

    if first_occurrence(arr, 4) == 2:
        passed = passed + 1

    if last_occurrence(arr, 4) == 4:
        passed = passed + 1

    if floor_value(arr, 5) == 4:
        passed = passed + 1

    if ceil_value(arr, 5) == 7:
        passed = passed + 1

    if binary_search(arr, 10) == -1:
        passed = passed + 1

    if floor_value(arr, 0) == -1:
        passed = passed + 1

    if ceil_value(arr, 10) == -1:
        passed = passed + 1

    return passed
