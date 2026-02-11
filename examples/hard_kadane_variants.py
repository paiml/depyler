"""Kadane's algorithm variants for maximum subarray problems.

Tests: running sum, max tracking, circular subarray.
"""


def max_subarray_sum(arr: list[int]) -> int:
    """Classic Kadane's algorithm for maximum subarray sum."""
    if len(arr) == 0:
        return 0
    max_ending: int = arr[0]
    max_so_far: int = arr[0]
    i: int = 1
    while i < len(arr):
        if max_ending + arr[i] > arr[i]:
            max_ending = max_ending + arr[i]
        else:
            max_ending = arr[i]
        if max_ending > max_so_far:
            max_so_far = max_ending
        i += 1
    return max_so_far


def max_subarray_length(arr: list[int]) -> int:
    """Find length of maximum sum subarray."""
    if len(arr) == 0:
        return 0
    max_ending: int = arr[0]
    max_so_far: int = arr[0]
    curr_len: int = 1
    best_len: int = 1
    i: int = 1
    while i < len(arr):
        if max_ending + arr[i] > arr[i]:
            max_ending = max_ending + arr[i]
            curr_len += 1
        else:
            max_ending = arr[i]
            curr_len = 1
        if max_ending > max_so_far:
            max_so_far = max_ending
            best_len = curr_len
        i += 1
    return best_len


def min_subarray_sum(arr: list[int]) -> int:
    """Find minimum subarray sum (inverted Kadane)."""
    if len(arr) == 0:
        return 0
    min_ending: int = arr[0]
    min_so_far: int = arr[0]
    i: int = 1
    while i < len(arr):
        if min_ending + arr[i] < arr[i]:
            min_ending = min_ending + arr[i]
        else:
            min_ending = arr[i]
        if min_ending < min_so_far:
            min_so_far = min_ending
        i += 1
    return min_so_far


def max_circular_subarray(arr: list[int]) -> int:
    """Maximum subarray sum in circular array."""
    if len(arr) == 0:
        return 0
    total: int = 0
    for v in arr:
        total += v
    max_sum: int = max_subarray_sum(arr)
    min_sum: int = min_subarray_sum(arr)
    if total == min_sum:
        return max_sum
    circular: int = total - min_sum
    if circular > max_sum:
        return circular
    return max_sum


def max_crossing_sum(arr: list[int], lo: int, mid: int, hi: int) -> int:
    """Maximum crossing subarray sum."""
    left_sum: int = arr[mid]
    total: int = arr[mid]
    i: int = mid - 1
    while i >= lo:
        total += arr[i]
        if total > left_sum:
            left_sum = total
        i -= 1
    right_sum: int = arr[mid + 1]
    total = arr[mid + 1]
    i = mid + 2
    while i <= hi:
        total += arr[i]
        if total > right_sum:
            right_sum = total
        i += 1
    return left_sum + right_sum


def test_module() -> int:
    """Test Kadane's algorithm variants."""
    ok: int = 0

    s: int = max_subarray_sum([-2, 1, -3, 4, -1, 2, 1, -5, 4])
    if s == 6:
        ok += 1

    ml: int = max_subarray_length([-2, 1, -3, 4, -1, 2, 1, -5, 4])
    if ml == 4:
        ok += 1

    ms: int = min_subarray_sum([3, -4, 2, -3, -1, 7, -5])
    if ms == -6:
        ok += 1

    c: int = max_circular_subarray([5, -3, 5])
    if c == 10:
        ok += 1

    cs: int = max_crossing_sum([2, -5, 6, -2, 3, 1, 5, -6], 0, 3, 7)
    if cs == 13:
        ok += 1

    return ok
