# Type inference test: Bubble sort with untyped array
# Strategy: Return types annotated, parameter types MISSING on some functions


def bubble_sort_pass(arr: list[int], size) -> int:
    """One pass of bubble sort. Returns number of swaps made."""
    swaps: int = 0
    i: int = 0
    while i < size - 1:
        if arr[i] > arr[i + 1]:
            temp: int = arr[i]
            arr[i] = arr[i + 1]
            arr[i + 1] = temp
            swaps = swaps + 1
        i = i + 1
    return swaps


def bubble_sort_count(arr: list[int]) -> int:
    """Sort array and return total number of swaps."""
    total_swaps: int = 0
    n: int = len(arr)
    done: int = 0
    while done == 0:
        swaps: int = bubble_sort_pass(arr, n)
        total_swaps = total_swaps + swaps
        n = n - 1
        if swaps == 0 or n <= 1:
            done = 1
    return total_swaps


def is_sorted(arr: list[int]) -> int:
    """Check if array is sorted ascending. Returns 1 if yes, 0 if no."""
    i: int = 0
    while i < len(arr) - 1:
        if arr[i] > arr[i + 1]:
            return 0
        i = i + 1
    return 1


def insertion_position(arr: list[int], val) -> int:
    """Find where val should be inserted in sorted array."""
    i: int = 0
    while i < len(arr):
        if arr[i] >= val:
            return i
        i = i + 1
    return len(arr)


def count_inversions(arr: list[int]) -> int:
    """Count number of inversions (pairs where arr[i] > arr[j] for i < j)."""
    count: int = 0
    i: int = 0
    while i < len(arr):
        j: int = i + 1
        while j < len(arr):
            if arr[i] > arr[j]:
                count = count + 1
            j = j + 1
        i = i + 1
    return count


def selection_sort_swaps(arr: list[int]) -> int:
    """Selection sort returning number of swaps."""
    swaps: int = 0
    n: int = len(arr)
    i: int = 0
    while i < n - 1:
        min_idx: int = i
        j: int = i + 1
        while j < n:
            if arr[j] < arr[min_idx]:
                min_idx = j
            j = j + 1
        if min_idx != i:
            temp: int = arr[i]
            arr[i] = arr[min_idx]
            arr[min_idx] = temp
            swaps = swaps + 1
        i = i + 1
    return swaps


def test_module() -> int:
    """Test all sorting inference functions."""
    total: int = 0

    # bubble_sort_count tests
    arr1: list[int] = [3, 1, 4, 1, 5]
    swaps1: int = bubble_sort_count(arr1)
    if swaps1 > 0:
        total = total + 1
    if is_sorted(arr1) == 1:
        total = total + 1

    # already sorted
    arr2: list[int] = [1, 2, 3, 4, 5]
    swaps2: int = bubble_sort_count(arr2)
    if swaps2 == 0:
        total = total + 1

    # is_sorted tests
    if is_sorted([1, 2, 3]) == 1:
        total = total + 1
    if is_sorted([3, 1, 2]) == 0:
        total = total + 1

    # insertion_position tests
    if insertion_position([1, 3, 5, 7], 4) == 2:
        total = total + 1
    if insertion_position([1, 3, 5], 0) == 0:
        total = total + 1
    if insertion_position([1, 3, 5], 6) == 3:
        total = total + 1

    # count_inversions tests
    if count_inversions([1, 2, 3]) == 0:
        total = total + 1
    if count_inversions([3, 2, 1]) == 3:
        total = total + 1
    if count_inversions([2, 1, 3]) == 1:
        total = total + 1

    # selection_sort_swaps tests
    arr3: list[int] = [5, 3, 1, 4, 2]
    sw3: int = selection_sort_swaps(arr3)
    if sw3 > 0:
        total = total + 1
    if is_sorted(arr3) == 1:
        total = total + 1

    return total
