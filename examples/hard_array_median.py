"""Median calculations on arrays.

Implements median finding algorithms using sorting
and selection-based approaches.
"""


def bubble_sort_copy(arr: list[int], size: int) -> list[int]:
    """Create a sorted copy of the array using bubble sort."""
    sorted_arr: list[int] = []
    i: int = 0
    while i < size:
        sorted_arr.append(arr[i])
        i = i + 1

    si: int = 0
    while si < size - 1:
        sj: int = 0
        while sj < size - 1 - si:
            next_j: int = sj + 1
            if sorted_arr[sj] > sorted_arr[next_j]:
                tmp: int = sorted_arr[sj]
                sorted_arr[sj] = sorted_arr[next_j]
                sorted_arr[next_j] = tmp
            sj = sj + 1
        si = si + 1
    return sorted_arr


def find_median(arr: list[int], size: int) -> int:
    """Find the median of an array. For even size, returns lower median."""
    tmp_sorted: list[int] = bubble_sort_copy(arr, size)
    mid: int = size // 2
    if size % 2 == 1:
        return tmp_sorted[mid]
    lower: int = mid - 1
    result: int = (tmp_sorted[lower] + tmp_sorted[mid]) // 2
    return result


def running_median(arr: list[int], size: int, window: int) -> list[int]:
    """Compute running median with given window size."""
    result: list[int] = []
    i: int = 0
    limit: int = size - window + 1
    while i < limit:
        window_data: list[int] = []
        j: int = 0
        while j < window:
            idx: int = i + j
            window_data.append(arr[idx])
            j = j + 1
        med: int = find_median(window_data, window)
        result.append(med)
        i = i + 1
    return result


def median_absolute_deviation(arr: list[int], size: int) -> int:
    """Compute median absolute deviation from the median."""
    med: int = find_median(arr, size)
    deviations: list[int] = []
    i: int = 0
    while i < size:
        diff: int = arr[i] - med
        if diff < 0:
            diff = -diff
        deviations.append(diff)
        i = i + 1
    result: int = find_median(deviations, size)
    return result


def test_module() -> int:
    """Test median calculation operations."""
    ok: int = 0

    arr1: list[int] = [3, 1, 4, 1, 5]
    med1: int = find_median(arr1, 5)
    if med1 == 3:
        ok = ok + 1

    arr2: list[int] = [1, 2, 3, 4]
    med2: int = find_median(arr2, 4)
    if med2 == 2:
        ok = ok + 1

    arr3: list[int] = [1, 3, 5, 7, 9]
    tmp_running: list[int] = running_median(arr3, 5, 3)
    if tmp_running[0] == 3 and tmp_running[1] == 5 and tmp_running[2] == 7:
        ok = ok + 1

    arr4: list[int] = [1, 2, 3, 4, 5]
    mad: int = median_absolute_deviation(arr4, 5)
    if mad == 1:
        ok = ok + 1

    return ok
