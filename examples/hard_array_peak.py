"""Peak finding operations on arrays.

Implements algorithms to find peaks, valleys, and
local extrema in integer arrays.
"""


def find_peak(arr: list[int], size: int) -> int:
    """Find index of first peak element (greater than both neighbors).

    Returns -1 if no peak found. First and last elements can be peaks
    if they are greater than their single neighbor.
    """
    if size == 0:
        return -1
    if size == 1:
        return 0
    if arr[0] > arr[1]:
        return 0
    i: int = 1
    limit: int = size - 1
    while i < limit:
        prev_idx: int = i - 1
        next_idx: int = i + 1
        if arr[i] > arr[prev_idx] and arr[i] > arr[next_idx]:
            return i
        i = i + 1
    last: int = size - 1
    second_last: int = size - 2
    if arr[last] > arr[second_last]:
        return last
    return -1


def count_peaks(arr: list[int], size: int) -> int:
    """Count all peak elements in the array."""
    if size <= 1:
        return 0
    count: int = 0
    if arr[0] > arr[1]:
        count = count + 1
    i: int = 1
    limit: int = size - 1
    while i < limit:
        prev_idx: int = i - 1
        next_idx: int = i + 1
        if arr[i] > arr[prev_idx] and arr[i] > arr[next_idx]:
            count = count + 1
        i = i + 1
    last: int = size - 1
    second_last: int = size - 2
    if arr[last] > arr[second_last]:
        count = count + 1
    return count


def find_valley(arr: list[int], size: int) -> int:
    """Find index of first valley element (less than both neighbors)."""
    if size <= 2:
        return -1
    i: int = 1
    limit: int = size - 1
    while i < limit:
        prev_idx: int = i - 1
        next_idx: int = i + 1
        if arr[i] < arr[prev_idx] and arr[i] < arr[next_idx]:
            return i
        i = i + 1
    return -1


def peak_to_valley_diff(arr: list[int], size: int) -> int:
    """Find maximum difference between a peak and the following valley."""
    max_diff: int = 0
    i: int = 1
    limit: int = size - 1
    while i < limit:
        prev_idx: int = i - 1
        next_idx: int = i + 1
        if arr[i] > arr[prev_idx] and arr[i] > arr[next_idx]:
            j: int = i + 1
            while j < limit:
                pj: int = j - 1
                nj: int = j + 1
                if arr[j] < arr[pj] and arr[j] < arr[nj]:
                    diff: int = arr[i] - arr[j]
                    if diff > max_diff:
                        max_diff = diff
                    j = size
                j = j + 1
        i = i + 1
    return max_diff


def test_module() -> int:
    """Test peak finding operations."""
    ok: int = 0

    arr1: list[int] = [1, 3, 2, 5, 1]
    peak: int = find_peak(arr1, 5)
    if peak == 1:
        ok = ok + 1

    peaks: int = count_peaks(arr1, 5)
    if peaks == 2:
        ok = ok + 1

    valley: int = find_valley(arr1, 5)
    if valley == 2:
        ok = ok + 1

    arr2: list[int] = [1, 8, 2, 7, 3]
    pvd: int = peak_to_valley_diff(arr2, 5)
    if pvd == 6:
        ok = ok + 1

    return ok
