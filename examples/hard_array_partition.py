"""Partition algorithms: Lomuto, Hoare, and 3-way partition."""


def lomuto_partition(arr: list[int], low: int, high: int) -> int:
    """Lomuto partition scheme, returns pivot index."""
    pivot: int = arr[high]
    i: int = low - 1
    j: int = low
    while j < high:
        if arr[j] <= pivot:
            i = i + 1
            tmp: int = arr[i]
            arr[i] = arr[j]
            arr[j] = tmp
        j = j + 1
    tmp2: int = arr[i + 1]
    arr[i + 1] = arr[high]
    arr[high] = tmp2
    return i + 1


def hoare_partition(arr: list[int], low: int, high: int) -> int:
    """Hoare partition scheme, returns partition index."""
    pivot: int = arr[low]
    i: int = low - 1
    j: int = high + 1
    while True:
        i = i + 1
        while arr[i] < pivot:
            i = i + 1
        j = j - 1
        while arr[j] > pivot:
            j = j - 1
        if i >= j:
            return j
        tmp: int = arr[i]
        arr[i] = arr[j]
        arr[j] = tmp


def three_way_partition(arr: list[int], pivot_val: int) -> list[int]:
    """Dutch national flag: partition into <pivot, ==pivot, >pivot."""
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        result.append(arr[i])
        i = i + 1
    low: int = 0
    mid: int = 0
    high: int = len(result) - 1
    while mid <= high:
        if result[mid] < pivot_val:
            tmp: int = result[low]
            result[low] = result[mid]
            result[mid] = tmp
            low = low + 1
            mid = mid + 1
        elif result[mid] == pivot_val:
            mid = mid + 1
        else:
            tmp2: int = result[mid]
            result[mid] = result[high]
            result[high] = tmp2
            high = high - 1
    return result


def is_sorted(arr: list[int]) -> int:
    """Check if array is sorted. Returns 1 for true, 0 for false."""
    i: int = 0
    while i < len(arr) - 1:
        if arr[i] > arr[i + 1]:
            return 0
        i = i + 1
    return 1


def test_module() -> int:
    passed: int = 0

    a1: list[int] = [3, 6, 8, 10, 1, 2, 1]
    a1_high: int = len(a1) - 1
    pi1: int = lomuto_partition(a1, 0, a1_high)
    if a1[pi1] == 1:
        passed = passed + 1

    a2: list[int] = [10, 7, 8, 9, 1, 5]
    a2_high: int = len(a2) - 1
    pi2: int = hoare_partition(a2, 0, a2_high)
    if pi2 >= 0 and pi2 < len(a2):
        passed = passed + 1

    t1: list[int] = three_way_partition([4, 2, 4, 1, 4, 3, 5], 4)
    all_less: int = 1
    idx: int = 0
    while idx < len(t1) and t1[idx] < 4:
        idx = idx + 1
    while idx < len(t1) and t1[idx] == 4:
        idx = idx + 1
    while idx < len(t1):
        if t1[idx] <= 4:
            all_less = 0
        idx = idx + 1
    if all_less == 1:
        passed = passed + 1

    t2: list[int] = three_way_partition([1, 1, 1], 1)
    if t2 == [1, 1, 1]:
        passed = passed + 1

    if is_sorted([1, 2, 3, 4, 5]) == 1:
        passed = passed + 1

    if is_sorted([5, 3, 1]) == 0:
        passed = passed + 1

    if is_sorted([]) == 1:
        passed = passed + 1

    return passed
