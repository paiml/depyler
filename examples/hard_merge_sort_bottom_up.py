"""Bottom-up merge sort implementation."""


def merge(arr: list[int], temp: list[int], left: int, mid: int, right: int) -> int:
    """Merge two sorted halves. Returns number of comparisons."""
    i: int = left
    j: int = mid + 1
    k: int = left
    comps: int = 0
    while i <= mid and j <= right:
        comps = comps + 1
        if arr[i] <= arr[j]:
            temp[k] = arr[i]
            i = i + 1
        else:
            temp[k] = arr[j]
            j = j + 1
        k = k + 1
    while i <= mid:
        temp[k] = arr[i]
        i = i + 1
        k = k + 1
    while j <= right:
        temp[k] = arr[j]
        j = j + 1
        k = k + 1
    i = left
    while i <= right:
        arr[i] = temp[i]
        i = i + 1
    return comps


def merge_sort_bottom_up(arr: list[int]) -> int:
    """Sort array using bottom-up merge sort. Returns total comparisons."""
    n: int = len(arr)
    if n <= 1:
        return 0
    temp: list[int] = []
    i: int = 0
    while i < n:
        temp.append(0)
        i = i + 1
    total_comps: int = 0
    width: int = 1
    while width < n:
        left: int = 0
        while left < n:
            mid: int = left + width - 1
            right: int = left + 2 * width - 1
            if mid >= n:
                mid = n - 1
            if right >= n:
                right = n - 1
            if mid < right:
                c: int = merge(arr, temp, left, mid, right)
                total_comps = total_comps + c
            left = left + 2 * width
        width = width * 2
    return total_comps


def is_sorted(arr: list[int]) -> int:
    """Check if array is sorted. Returns 1 if sorted."""
    n: int = len(arr)
    i: int = 0
    while i < n - 1:
        if arr[i] > arr[i + 1]:
            return 0
        i = i + 1
    return 1


def test_module() -> int:
    """Test bottom-up merge sort."""
    passed: int = 0

    a1: list[int] = [5, 3, 8, 1, 2, 7, 4, 6]
    merge_sort_bottom_up(a1)
    if is_sorted(a1) == 1:
        passed = passed + 1

    if a1[0] == 1 and a1[7] == 8:
        passed = passed + 1

    a2: list[int] = [1, 2, 3, 4]
    merge_sort_bottom_up(a2)
    if is_sorted(a2) == 1:
        passed = passed + 1

    a3: list[int] = [4, 3, 2, 1]
    merge_sort_bottom_up(a3)
    if a3[0] == 1 and a3[3] == 4:
        passed = passed + 1

    a4: list[int] = [1]
    merge_sort_bottom_up(a4)
    if a4[0] == 1:
        passed = passed + 1

    a5: list[int] = [3, 3, 3, 1, 1]
    merge_sort_bottom_up(a5)
    if is_sorted(a5) == 1:
        passed = passed + 1

    return passed
