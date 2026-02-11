"""Compact and remove elements from arrays.

Implements in-place and copy-based array compaction operations
that remove unwanted elements while preserving order.
"""


def compact_zeros(arr: list[int], size: int) -> list[int]:
    """Remove all zero elements from array, preserving order of non-zeros."""
    result: list[int] = []
    i: int = 0
    while i < size:
        if arr[i] != 0:
            result.append(arr[i])
        i = i + 1
    return result


def compact_duplicates(arr: list[int], size: int) -> list[int]:
    """Remove consecutive duplicates from a sorted array."""
    if size == 0:
        return []
    result: list[int] = [arr[0]]
    i: int = 1
    while i < size:
        last_idx: int = len(result) - 1
        if arr[i] != result[last_idx]:
            result.append(arr[i])
        i = i + 1
    return result


def compact_below_threshold(arr: list[int], size: int, threshold: int) -> list[int]:
    """Remove all elements below threshold."""
    result: list[int] = []
    i: int = 0
    while i < size:
        if arr[i] >= threshold:
            result.append(arr[i])
        i = i + 1
    return result


def compact_at_indices(arr: list[int], size: int, remove_flags: list[int], flag_size: int) -> list[int]:
    """Remove elements at positions marked with 1 in remove_flags."""
    result: list[int] = []
    i: int = 0
    while i < size:
        if i < flag_size and remove_flags[i] == 0:
            result.append(arr[i])
        i = i + 1
    return result


def test_module() -> int:
    """Test array compaction operations."""
    ok: int = 0

    arr1: list[int] = [0, 1, 0, 2, 0, 3]
    tmp1: list[int] = compact_zeros(arr1, 6)
    if len(tmp1) == 3 and tmp1[0] == 1 and tmp1[1] == 2 and tmp1[2] == 3:
        ok = ok + 1

    arr2: list[int] = [1, 1, 2, 2, 3, 3, 3]
    tmp2: list[int] = compact_duplicates(arr2, 7)
    if len(tmp2) == 3 and tmp2[0] == 1 and tmp2[1] == 2 and tmp2[2] == 3:
        ok = ok + 1

    arr3: list[int] = [5, 2, 8, 1, 9, 3]
    tmp3: list[int] = compact_below_threshold(arr3, 6, 5)
    if len(tmp3) == 3 and tmp3[0] == 5 and tmp3[1] == 8 and tmp3[2] == 9:
        ok = ok + 1

    arr4: list[int] = [10, 20, 30, 40, 50]
    flags: list[int] = [0, 1, 0, 1, 0]
    tmp4: list[int] = compact_at_indices(arr4, 5, flags, 5)
    if len(tmp4) == 3 and tmp4[0] == 10 and tmp4[1] == 30 and tmp4[2] == 50:
        ok = ok + 1

    empty: list[int] = []
    tmp5: list[int] = compact_duplicates(empty, 0)
    if len(tmp5) == 0:
        ok = ok + 1

    return ok
