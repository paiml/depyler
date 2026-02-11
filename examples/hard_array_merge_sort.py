"""Merge sort with inversion count."""


def merge_and_count(arr: list[int], left: int, mid: int, right: int) -> int:
    """Merge two sorted halves and count inversions."""
    left_part: list[int] = []
    right_part: list[int] = []
    i: int = left
    while i <= mid:
        left_part.append(arr[i])
        i = i + 1
    j: int = mid + 1
    while j <= right:
        right_part.append(arr[j])
        j = j + 1
    li: int = 0
    ri: int = 0
    ki: int = left
    inversions: int = 0
    while li < len(left_part) and ri < len(right_part):
        if left_part[li] <= right_part[ri]:
            arr[ki] = left_part[li]
            li = li + 1
        else:
            arr[ki] = right_part[ri]
            inversions = inversions + (len(left_part) - li)
            ri = ri + 1
        ki = ki + 1
    while li < len(left_part):
        arr[ki] = left_part[li]
        li = li + 1
        ki = ki + 1
    while ri < len(right_part):
        arr[ki] = right_part[ri]
        ri = ri + 1
        ki = ki + 1
    return inversions


def merge_sort_helper(arr: list[int], left: int, right: int) -> int:
    """Recursive merge sort returning inversion count."""
    count: int = 0
    if left < right:
        mid: int = left + (right - left) // 2
        count = count + merge_sort_helper(arr, left, mid)
        count = count + merge_sort_helper(arr, mid + 1, right)
        count = count + merge_and_count(arr, left, mid, right)
    return count


def merge_sort(arr: list[int]) -> list[int]:
    """Sort array using merge sort, return sorted copy."""
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        result.append(arr[i])
        i = i + 1
    if len(result) > 1:
        merge_sort_helper(result, 0, len(result) - 1)
    return result


def count_inversions(arr: list[int]) -> int:
    """Count number of inversions in the array."""
    copy: list[int] = []
    i: int = 0
    while i < len(arr):
        copy.append(arr[i])
        i = i + 1
    if len(copy) <= 1:
        return 0
    return merge_sort_helper(copy, 0, len(copy) - 1)


def test_module() -> int:
    passed: int = 0

    s1: list[int] = merge_sort([5, 3, 1, 4, 2])
    if s1 == [1, 2, 3, 4, 5]:
        passed = passed + 1

    s2: list[int] = merge_sort([])
    if s2 == []:
        passed = passed + 1

    s3: list[int] = merge_sort([1])
    if s3 == [1]:
        passed = passed + 1

    inv1: int = count_inversions([1, 2, 3, 4, 5])
    if inv1 == 0:
        passed = passed + 1

    inv2: int = count_inversions([5, 4, 3, 2, 1])
    if inv2 == 10:
        passed = passed + 1

    inv3: int = count_inversions([2, 4, 1, 3, 5])
    if inv3 == 3:
        passed = passed + 1

    s4: list[int] = merge_sort([3, 3, 1, 1, 2])
    if s4 == [1, 1, 2, 3, 3]:
        passed = passed + 1

    return passed
