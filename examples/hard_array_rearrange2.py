"""Array rearrangement: segregate even/odd, move zeros, Dutch flag problem.

Tests: segregate_even_odd, move_zeros_end, dutch_flag_sort.
"""


def segregate_even_odd(arr: list[int]) -> list[int]:
    """Move all even numbers before odd numbers, preserving relative order."""
    evens: list[int] = []
    odds: list[int] = []
    i: int = 0
    while i < len(arr):
        if arr[i] % 2 == 0:
            evens.append(arr[i])
        else:
            odds.append(arr[i])
        i = i + 1
    result: list[int] = []
    j: int = 0
    while j < len(evens):
        result.append(evens[j])
        j = j + 1
    j = 0
    while j < len(odds):
        result.append(odds[j])
        j = j + 1
    return result


def move_zeros_end(arr: list[int]) -> list[int]:
    """Move all zeros to end while preserving order of non-zero elements."""
    result: list[int] = arr[:]
    write: int = 0
    read: int = 0
    while read < len(result):
        if result[read] != 0:
            result[write] = result[read]
            write = write + 1
        read = read + 1
    while write < len(result):
        result[write] = 0
        write = write + 1
    return result


def dutch_flag_sort(arr: list[int]) -> list[int]:
    """Sort array of 0s, 1s, and 2s (Dutch National Flag problem)."""
    result: list[int] = arr[:]
    low: int = 0
    mid: int = 0
    high: int = len(result) - 1
    while mid <= high:
        if result[mid] == 0:
            tmp: int = result[low]
            result[low] = result[mid]
            result[mid] = tmp
            low = low + 1
            mid = mid + 1
        elif result[mid] == 1:
            mid = mid + 1
        else:
            tmp2: int = result[mid]
            result[mid] = result[high]
            result[high] = tmp2
            high = high - 1
    return result


def count_value(arr: list[int], val: int) -> int:
    """Count occurrences of val in arr."""
    count: int = 0
    i: int = 0
    while i < len(arr):
        if arr[i] == val:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test array rearrangement operations."""
    ok: int = 0

    if segregate_even_odd([1, 2, 3, 4, 5, 6]) == [2, 4, 6, 1, 3, 5]:
        ok = ok + 1

    if segregate_even_odd([]) == []:
        ok = ok + 1

    if move_zeros_end([0, 1, 0, 3, 12]) == [1, 3, 12, 0, 0]:
        ok = ok + 1

    if move_zeros_end([0, 0, 0]) == [0, 0, 0]:
        ok = ok + 1

    if dutch_flag_sort([2, 0, 1, 2, 0, 1]) == [0, 0, 1, 1, 2, 2]:
        ok = ok + 1

    if dutch_flag_sort([1, 0, 2]) == [0, 1, 2]:
        ok = ok + 1

    if count_value([1, 2, 1, 3, 1], 1) == 3:
        ok = ok + 1

    return ok
