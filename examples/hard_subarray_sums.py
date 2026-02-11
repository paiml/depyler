"""Subarray sum problems.

Tests: max subarray sum, subarray with given sum, count subarrays.
"""


def max_subarray_sum(arr: list[int]) -> int:
    """Kadane's algorithm for maximum subarray sum."""
    if len(arr) == 0:
        return 0
    max_sum: int = arr[0]
    current: int = arr[0]
    i: int = 1
    while i < len(arr):
        if current + arr[i] > arr[i]:
            current = current + arr[i]
        else:
            current = arr[i]
        if current > max_sum:
            max_sum = current
        i = i + 1
    return max_sum


def subarray_sum_exists(arr: list[int], target: int) -> int:
    """Check if contiguous subarray with given sum exists. Returns 1 if yes.
    Works for non-negative arrays."""
    n: int = len(arr)
    i: int = 0
    while i < n:
        current_sum: int = 0
        j: int = i
        while j < n:
            current_sum = current_sum + arr[j]
            if current_sum == target:
                return 1
            j = j + 1
        i = i + 1
    return 0


def count_subarrays_with_sum(arr: list[int], target: int) -> int:
    """Count subarrays with exact sum equal to target."""
    n: int = len(arr)
    count: int = 0
    i: int = 0
    while i < n:
        current_sum: int = 0
        j: int = i
        while j < n:
            current_sum = current_sum + arr[j]
            if current_sum == target:
                count = count + 1
            j = j + 1
        i = i + 1
    return count


def max_subarray_length(arr: list[int], target: int) -> int:
    """Find length of longest subarray with sum equal to target."""
    n: int = len(arr)
    max_len: int = 0
    i: int = 0
    while i < n:
        current_sum: int = 0
        j: int = i
        while j < n:
            current_sum = current_sum + arr[j]
            if current_sum == target:
                length: int = j - i + 1
                if length > max_len:
                    max_len = length
            j = j + 1
        i = i + 1
    return max_len


def test_module() -> int:
    """Test subarray sum operations."""
    ok: int = 0
    if max_subarray_sum([-2, 1, -3, 4, -1, 2, 1, -5, 4]) == 6:
        ok = ok + 1
    if subarray_sum_exists([1, 4, 20, 3, 10, 5], 33) == 1:
        ok = ok + 1
    if subarray_sum_exists([1, 4, 0, 0, 3, 10, 5], 100) == 0:
        ok = ok + 1
    if count_subarrays_with_sum([1, 1, 1], 2) == 2:
        ok = ok + 1
    if max_subarray_length([1, 1, 1, 1], 2) == 2:
        ok = ok + 1
    return ok
