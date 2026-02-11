"""Pancake sorting implementation."""


def flip(arr: list[int], k: int) -> int:
    """Reverse first k+1 elements of arr in place. Returns 0."""
    left: int = 0
    right: int = k
    while left < right:
        tmp: int = arr[left]
        arr[left] = arr[right]
        arr[right] = tmp
        left = left + 1
        right = right - 1
    return 0


def find_max_idx(arr: list[int], end: int) -> int:
    """Find index of maximum element in arr[0..end]."""
    max_idx: int = 0
    i: int = 1
    while i <= end:
        if arr[i] > arr[max_idx]:
            max_idx = i
        i = i + 1
    return max_idx


def pancake_sort(arr: list[int]) -> int:
    """Sort array using pancake flips. Returns number of flips."""
    n: int = len(arr)
    flips: int = 0
    cur_size: int = n - 1
    while cur_size > 0:
        mi: int = find_max_idx(arr, cur_size)
        if mi != cur_size:
            if mi != 0:
                flip(arr, mi)
                flips = flips + 1
            flip(arr, cur_size)
            flips = flips + 1
        cur_size = cur_size - 1
    return flips


def is_sorted(arr: list[int]) -> int:
    """Returns 1 if array is sorted ascending."""
    i: int = 1
    while i < len(arr):
        if arr[i] < arr[i - 1]:
            return 0
        i = i + 1
    return 1


def copy_array(arr: list[int]) -> list[int]:
    """Create a copy of array."""
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        result.append(arr[i])
        i = i + 1
    return result


def test_module() -> int:
    """Test pancake sorting."""
    ok: int = 0
    a1: list[int] = [3, 1, 2]
    c1: list[int] = copy_array(a1)
    pancake_sort(c1)
    if is_sorted(c1) == 1:
        ok = ok + 1
    a2: list[int] = [5, 3, 1, 4, 2]
    c2: list[int] = copy_array(a2)
    pancake_sort(c2)
    if is_sorted(c2) == 1:
        ok = ok + 1
    a3: list[int] = [1, 2, 3]
    c3: list[int] = copy_array(a3)
    f3: int = pancake_sort(c3)
    if f3 == 0:
        ok = ok + 1
    a4: list[int] = [2, 1]
    c4: list[int] = copy_array(a4)
    pancake_sort(c4)
    if c4[0] == 1 and c4[1] == 2:
        ok = ok + 1
    a5: list[int] = [1]
    if is_sorted(a5) == 1:
        ok = ok + 1
    a6: list[int] = [4, 3, 2, 1]
    c6: list[int] = copy_array(a6)
    pancake_sort(c6)
    if is_sorted(c6) == 1:
        ok = ok + 1
    empty: list[int] = []
    if is_sorted(empty) == 1:
        ok = ok + 1
    return ok
