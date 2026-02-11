"""Pancake sorting algorithm."""


def flip(arr: list[int], end: int) -> list[int]:
    """Reverse elements from index 0 to end (inclusive)."""
    start: int = 0
    while start < end:
        temp: int = arr[start]
        arr[start] = arr[end]
        arr[end] = temp
        start = start + 1
        end = end - 1
    return arr


def find_max_index(arr: list[int], limit: int) -> int:
    """Find index of maximum element up to limit (inclusive)."""
    max_idx: int = 0
    max_val: int = arr[0]
    i: int = 1
    while i <= limit:
        if arr[i] > max_val:
            max_val = arr[i]
            max_idx = i
        i = i + 1
    return max_idx


def pancake_sort(arr: list[int]) -> list[int]:
    """Sort array using pancake flips."""
    length: int = len(arr)
    current_size: int = length - 1
    while current_size > 0:
        max_idx: int = find_max_index(arr, current_size)
        if max_idx != current_size:
            if max_idx != 0:
                arr = flip(arr, max_idx)
            arr = flip(arr, current_size)
        current_size = current_size - 1
    return arr


def count_pancake_flips(arr: list[int]) -> int:
    """Count the number of flips needed to sort."""
    work: list[int] = []
    i: int = 0
    length: int = len(arr)
    while i < length:
        work.append(arr[i])
        i = i + 1
    flips: int = 0
    current_size: int = length - 1
    while current_size > 0:
        max_idx: int = find_max_index(work, current_size)
        if max_idx != current_size:
            if max_idx != 0:
                work = flip(work, max_idx)
                flips = flips + 1
            work = flip(work, current_size)
            flips = flips + 1
        current_size = current_size - 1
    return flips


def test_module() -> int:
    """Test pancake sorting operations."""
    passed: int = 0

    r1: list[int] = flip([1, 2, 3, 4], 3)
    if r1[0] == 4 and r1[3] == 1:
        passed = passed + 1

    r2: int = find_max_index([3, 1, 4, 2], 3)
    if r2 == 2:
        passed = passed + 1

    r3: list[int] = pancake_sort([3, 1, 4, 2])
    if r3[0] == 1 and r3[1] == 2 and r3[2] == 3 and r3[3] == 4:
        passed = passed + 1

    r4: list[int] = pancake_sort([5, 3, 1, 4, 2])
    if r4[0] == 1 and r4[4] == 5:
        passed = passed + 1

    r5: list[int] = pancake_sort([1])
    if r5[0] == 1:
        passed = passed + 1

    r6: int = count_pancake_flips([1, 2, 3])
    if r6 == 0:
        passed = passed + 1

    r7: int = count_pancake_flips([3, 2, 1])
    if r7 > 0:
        passed = passed + 1

    r8: list[int] = pancake_sort([2, 1])
    if r8[0] == 1 and r8[1] == 2:
        passed = passed + 1

    return passed
