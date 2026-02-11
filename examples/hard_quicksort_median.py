"""Quicksort with median-of-three pivot selection."""


def median_of_three(arr: list[int], lo: int, hi: int) -> int:
    """Return index of median of arr[lo], arr[mid], arr[hi]."""
    mid: int = lo + (hi - lo) // 2
    a: int = arr[lo]
    b: int = arr[mid]
    c: int = arr[hi]
    if a <= b and b <= c:
        return mid
    if c <= b and b <= a:
        return mid
    if b <= a and a <= c:
        return lo
    if c <= a and a <= b:
        return lo
    return hi


def partition(arr: list[int], lo: int, hi: int) -> int:
    """Partition using median-of-three pivot. Returns pivot index."""
    pivot_idx: int = median_of_three(arr, lo, hi)
    tmp: int = arr[pivot_idx]
    arr[pivot_idx] = arr[hi]
    arr[hi] = tmp
    pivot: int = arr[hi]
    i: int = lo
    j: int = lo
    while j < hi:
        if arr[j] <= pivot:
            tmp2: int = arr[i]
            arr[i] = arr[j]
            arr[j] = tmp2
            i = i + 1
        j = j + 1
    tmp3: int = arr[i]
    arr[i] = arr[hi]
    arr[hi] = tmp3
    return i


def quicksort_range(arr: list[int], lo: int, hi: int) -> int:
    """Quicksort a range of the array. Returns 0."""
    if lo < hi:
        p: int = partition(arr, lo, hi)
        quicksort_range(arr, lo, p - 1)
        quicksort_range(arr, p + 1, hi)
    return 0


def quicksort(arr: list[int]) -> int:
    """Sort array using quicksort with median-of-three. Returns 0."""
    n: int = len(arr)
    if n <= 1:
        return 0
    quicksort_range(arr, 0, n - 1)
    return 0


def is_sorted(arr: list[int]) -> int:
    """Returns 1 if arr is sorted ascending."""
    n: int = len(arr)
    i: int = 0
    while i < n - 1:
        if arr[i] > arr[i + 1]:
            return 0
        i = i + 1
    return 1


def test_module() -> int:
    """Test quicksort with median-of-three."""
    passed: int = 0

    a1: list[int] = [9, 7, 5, 3, 1, 2, 4, 6, 8]
    quicksort(a1)
    if is_sorted(a1) == 1:
        passed = passed + 1

    a2: list[int] = [1, 2, 3, 4, 5]
    quicksort(a2)
    if is_sorted(a2) == 1:
        passed = passed + 1

    a3: list[int] = [5, 4, 3, 2, 1]
    quicksort(a3)
    if a3[0] == 1 and a3[4] == 5:
        passed = passed + 1

    a4: list[int] = [3, 3, 3, 1, 1, 2, 2]
    quicksort(a4)
    if is_sorted(a4) == 1:
        passed = passed + 1

    a5: list[int] = [42]
    quicksort(a5)
    if a5[0] == 42:
        passed = passed + 1

    a6: list[int] = []
    quicksort(a6)
    if len(a6) == 0:
        passed = passed + 1

    return passed
