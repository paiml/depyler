# Type inference test: Binary search with untyped params
# Strategy: Return types annotated, parameter types MISSING on some functions


def binary_search(arr: list[int], target) -> int:
    """Binary search returning index or -1. Target type inferred."""
    low: int = 0
    high: int = len(arr) - 1
    while low <= high:
        mid: int = (low + high) // 2
        if arr[mid] == target:
            return mid
        if arr[mid] < target:
            low = mid + 1
        else:
            high = mid - 1
    return 0 - 1


def linear_search(arr: list[int], target) -> int:
    """Linear search returning index or -1."""
    i: int = 0
    while i < len(arr):
        if arr[i] == target:
            return i
        i = i + 1
    return 0 - 1


def count_occurrences(arr: list[int], target) -> int:
    """Count how many times target appears in array."""
    count: int = 0
    i: int = 0
    while i < len(arr):
        if arr[i] == target:
            count = count + 1
        i = i + 1
    return count


def find_first_ge(arr: list[int], threshold) -> int:
    """Find index of first element >= threshold in sorted array."""
    low: int = 0
    high: int = len(arr)
    while low < high:
        mid: int = (low + high) // 2
        if arr[mid] < threshold:
            low = mid + 1
        else:
            high = mid
    if low < len(arr):
        return low
    return 0 - 1


def find_peak(arr: list[int]) -> int:
    """Find a peak element index (element greater than neighbors)."""
    n: int = len(arr)
    if n == 0:
        return 0 - 1
    if n == 1:
        return 0
    if arr[0] >= arr[1]:
        return 0
    if arr[n - 1] >= arr[n - 2]:
        return n - 1
    i: int = 1
    while i < n - 1:
        if arr[i] >= arr[i - 1] and arr[i] >= arr[i + 1]:
            return i
        i = i + 1
    return 0


def search_range_count(arr: list[int], low_val, high_val) -> int:
    """Count elements in range [low_val, high_val] inclusive."""
    count: int = 0
    i: int = 0
    while i < len(arr):
        if arr[i] >= low_val and arr[i] <= high_val:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test all search inference functions."""
    total: int = 0

    sorted_arr: list[int] = [1, 3, 5, 7, 9, 11, 13]

    # binary_search tests
    if binary_search(sorted_arr, 7) == 3:
        total = total + 1
    if binary_search(sorted_arr, 1) == 0:
        total = total + 1
    if binary_search(sorted_arr, 13) == 6:
        total = total + 1
    if binary_search(sorted_arr, 4) == 0 - 1:
        total = total + 1

    # linear_search tests
    if linear_search(sorted_arr, 5) == 2:
        total = total + 1
    if linear_search(sorted_arr, 100) == 0 - 1:
        total = total + 1

    # count_occurrences tests
    repeated: list[int] = [1, 2, 2, 3, 2, 4]
    if count_occurrences(repeated, 2) == 3:
        total = total + 1
    if count_occurrences(repeated, 5) == 0:
        total = total + 1

    # find_first_ge tests
    if find_first_ge(sorted_arr, 6) == 3:
        total = total + 1
    if find_first_ge(sorted_arr, 1) == 0:
        total = total + 1
    if find_first_ge(sorted_arr, 100) == 0 - 1:
        total = total + 1

    # find_peak tests
    peak_arr: list[int] = [1, 3, 5, 4, 2]
    peak_idx: int = find_peak(peak_arr)
    if peak_idx == 2:
        total = total + 1

    # search_range_count tests
    if search_range_count(sorted_arr, 3, 9) == 4:
        total = total + 1

    return total
