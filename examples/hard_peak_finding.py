"""Peak element finding in arrays.

Tests: find peak index, count peaks, find all peaks.
"""


def find_peak_index(arr: list[int]) -> int:
    """Find index of a peak element (greater than neighbors).
    Returns -1 if array is empty."""
    n: int = len(arr)
    if n == 0:
        return -1
    if n == 1:
        return 0
    if arr[0] >= arr[1]:
        return 0
    i: int = 1
    while i < n - 1:
        if arr[i] >= arr[i - 1]:
            if arr[i] >= arr[i + 1]:
                return i
        i = i + 1
    if arr[n - 1] >= arr[n - 2]:
        return n - 1
    return 0


def find_all_peak_indices(arr: list[int]) -> list[int]:
    """Find all peak indices."""
    peaks: list[int] = []
    n: int = len(arr)
    if n == 0:
        return peaks
    if n == 1:
        peaks.append(0)
        return peaks
    if arr[0] > arr[1]:
        peaks.append(0)
    i: int = 1
    while i < n - 1:
        if arr[i] > arr[i - 1]:
            if arr[i] > arr[i + 1]:
                peaks.append(i)
        i = i + 1
    if arr[n - 1] > arr[n - 2]:
        peaks.append(n - 1)
    return peaks


def find_plateau_length(arr: list[int]) -> int:
    """Find length of longest plateau (consecutive equal max elements)."""
    if len(arr) == 0:
        return 0
    max_val: int = arr[0]
    i: int = 1
    while i < len(arr):
        if arr[i] > max_val:
            max_val = arr[i]
        i = i + 1
    max_len: int = 0
    cur_len: int = 0
    j: int = 0
    while j < len(arr):
        if arr[j] == max_val:
            cur_len = cur_len + 1
            if cur_len > max_len:
                max_len = cur_len
        else:
            cur_len = 0
        j = j + 1
    return max_len


def test_module() -> int:
    """Test peak finding operations."""
    ok: int = 0
    arr: list[int] = [1, 3, 20, 4, 1, 0]
    idx: int = find_peak_index(arr)
    if idx == 2:
        ok = ok + 1
    peaks: list[int] = find_all_peak_indices(arr)
    if len(peaks) == 1:
        ok = ok + 1
    if peaks[0] == 2:
        ok = ok + 1
    multi: list[int] = [1, 5, 2, 7, 3]
    mp: list[int] = find_all_peak_indices(multi)
    if len(mp) == 2:
        ok = ok + 1
    if find_plateau_length([1, 3, 3, 3, 2]) == 3:
        ok = ok + 1
    return ok
