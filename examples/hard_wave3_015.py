"""Numerical methods: Sorting-based numerical algorithms.

Tests: comparison-based sorting, partition logic, selection algorithms,
merge patterns, stability considerations.
"""

from typing import List, Tuple


def insertion_sort(arr: List[int]) -> List[int]:
    """Sort array using insertion sort."""
    result: List[int] = []
    for x in arr:
        result.append(x)
    n: int = len(result)
    i: int = 1
    while i < n:
        val: int = result[i]
        j: int = i - 1
        while j >= 0 and result[j] > val:
            result[j + 1] = result[j]
            j -= 1
        result[j + 1] = val
        i += 1
    return result


def selection_sort(arr: List[int]) -> List[int]:
    """Sort array using selection sort."""
    result: List[int] = []
    for x in arr:
        result.append(x)
    n: int = len(result)
    i: int = 0
    while i < n:
        min_idx: int = i
        j: int = i + 1
        while j < n:
            if result[j] < result[min_idx]:
                min_idx = j
            j += 1
        temp: int = result[i]
        result[i] = result[min_idx]
        result[min_idx] = temp
        i += 1
    return result


def bubble_sort(arr: List[int]) -> List[int]:
    """Sort array using bubble sort with early exit."""
    result: List[int] = []
    for x in arr:
        result.append(x)
    n: int = len(result)
    i: int = 0
    while i < n:
        swapped: bool = False
        j: int = 0
        while j < n - i - 1:
            if result[j] > result[j + 1]:
                temp: int = result[j]
                result[j] = result[j + 1]
                result[j + 1] = temp
                swapped = True
            j += 1
        if not swapped:
            return result
        i += 1
    return result


def counting_sort(arr: List[int], max_val: int) -> List[int]:
    """Sort non-negative integers using counting sort."""
    counts: List[int] = []
    i: int = 0
    while i <= max_val:
        counts.append(0)
        i += 1
    for x in arr:
        if x >= 0 and x <= max_val:
            counts[x] = counts[x] + 1
    result: List[int] = []
    i = 0
    while i <= max_val:
        j: int = 0
        while j < counts[i]:
            result.append(i)
            j += 1
        i += 1
    return result


def find_kth_smallest(arr: List[int], k_val: int) -> int:
    """Find kth smallest element using sorting."""
    sorted_arr: List[int] = insertion_sort(arr)
    if k_val < 0 or k_val >= len(sorted_arr):
        return -1
    return sorted_arr[k_val]


def count_inversions(arr: List[int]) -> int:
    """Count number of inversions in array (brute force)."""
    n: int = len(arr)
    count: int = 0
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            if arr[i] > arr[j]:
                count += 1
            j += 1
        i += 1
    return count


def is_sorted(arr: List[int]) -> bool:
    """Check if array is sorted in ascending order."""
    n: int = len(arr)
    i: int = 1
    while i < n:
        if arr[i] < arr[i - 1]:
            return False
        i += 1
    return True


def test_sorting() -> bool:
    """Test sorting algorithms."""
    ok: bool = True
    s1: List[int] = insertion_sort([5, 3, 1, 4, 2])
    if not is_sorted(s1):
        ok = False
    s2: List[int] = selection_sort([5, 3, 1, 4, 2])
    if not is_sorted(s2):
        ok = False
    s3: List[int] = bubble_sort([5, 3, 1, 4, 2])
    if not is_sorted(s3):
        ok = False
    inv: int = count_inversions([3, 1, 2])
    if inv != 2:
        ok = False
    return ok
