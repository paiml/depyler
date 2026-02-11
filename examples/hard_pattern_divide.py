"""Divide and conquer: merge sort, count inversions, closest pair distance.

Tests: merge_sort, count_inversions, max_crossing_subarray, power.
"""


def merge_sort(arr: list[int]) -> list[int]:
    """Merge sort returning new sorted list."""
    n: int = len(arr)
    if n <= 1:
        return arr
    mid: int = n // 2
    left: list[int] = []
    i: int = 0
    while i < mid:
        left.append(arr[i])
        i = i + 1
    right: list[int] = []
    j: int = mid
    while j < n:
        right.append(arr[j])
        j = j + 1
    sorted_left: list[int] = merge_sort(left)
    sorted_right: list[int] = merge_sort(right)
    return merge_two(sorted_left, sorted_right)


def merge_two(a: list[int], b: list[int]) -> list[int]:
    """Merge two sorted lists."""
    result: list[int] = []
    i: int = 0
    j: int = 0
    a_len: int = len(a)
    b_len: int = len(b)
    while i < a_len:
        while j < b_len:
            if b[j] <= a[i]:
                result.append(b[j])
                j = j + 1
            else:
                break
        result.append(a[i])
        i = i + 1
    while j < b_len:
        result.append(b[j])
        j = j + 1
    return result


def count_inversions(arr: list[int]) -> int:
    """Count inversions using O(n^2) direct comparison."""
    n: int = len(arr)
    if n <= 1:
        return 0
    count: int = 0
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            if arr[i] > arr[j]:
                count = count + 1
            j = j + 1
        i = i + 1
    return count


def max_crossing_sum(arr: list[int], lo: int, mid: int, hi: int) -> int:
    """Max subarray sum crossing the midpoint."""
    left_sum: int = -999999999
    total: int = 0
    i: int = mid
    while i >= lo:
        total = total + arr[i]
        if total > left_sum:
            left_sum = total
        i = i - 1
    right_sum: int = -999999999
    total = 0
    j: int = mid + 1
    while j <= hi:
        total = total + arr[j]
        if total > right_sum:
            right_sum = total
        j = j + 1
    return left_sum + right_sum


def fast_power(x: int, n: int) -> int:
    """Compute x^n using divide and conquer (binary exponentiation)."""
    if n == 0:
        return 1
    if n == 1:
        return x
    half: int = fast_power(x, n // 2)
    if n % 2 == 0:
        return half * half
    return half * half * x


def test_module() -> int:
    """Test divide and conquer algorithms."""
    passed: int = 0

    sorted_arr: list[int] = merge_sort([5, 2, 8, 1, 9, 3])
    if sorted_arr == [1, 2, 3, 5, 8, 9]:
        passed = passed + 1

    empty_sort: list[int] = merge_sort([])
    if empty_sort == []:
        passed = passed + 1

    inv: int = count_inversions([2, 4, 1, 3, 5])
    if inv == 3:
        passed = passed + 1

    inv2: int = count_inversions([1, 2, 3])
    if inv2 == 0:
        passed = passed + 1

    arr: list[int] = [-2, 1, -3, 4, -1, 2, 1, -5, 4]
    cs: int = max_crossing_sum(arr, 0, 4, 8)
    if cs == 5:
        passed = passed + 1

    if fast_power(2, 10) == 1024:
        passed = passed + 1

    if fast_power(3, 0) == 1:
        passed = passed + 1

    return passed
