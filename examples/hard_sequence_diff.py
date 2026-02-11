"""Sequence differencing operations.

Tests: first difference, second difference, cumulative sum, difference restored.
"""


def first_difference(arr: list[int]) -> list[int]:
    """Compute first differences: d[i] = arr[i+1] - arr[i]."""
    result: list[int] = []
    i: int = 0
    n: int = len(arr)
    while i < n - 1:
        result.append(arr[i + 1] - arr[i])
        i = i + 1
    return result


def second_difference(arr: list[int]) -> list[int]:
    """Compute second differences."""
    d1: list[int] = first_difference(arr)
    return first_difference(d1)


def cumulative_sum(arr: list[int]) -> list[int]:
    """Compute cumulative sum (prefix sum)."""
    result: list[int] = []
    total: int = 0
    i: int = 0
    while i < len(arr):
        total = total + arr[i]
        result.append(total)
        i = i + 1
    return result


def restore_from_diff(first_val: int, diffs: list[int]) -> list[int]:
    """Restore original sequence from first value and differences."""
    result: list[int] = [first_val]
    i: int = 0
    while i < len(diffs):
        result.append(result[i] + diffs[i])
        i = i + 1
    return result


def is_arithmetic_sequence(arr: list[int]) -> int:
    """Check if sequence is arithmetic (constant first difference). Returns 1 or 0."""
    n: int = len(arr)
    if n <= 2:
        return 1
    d: list[int] = first_difference(arr)
    i: int = 1
    while i < len(d):
        if d[i] != d[0]:
            return 0
        i = i + 1
    return 1


def test_module() -> int:
    """Test sequence differencing operations."""
    ok: int = 0
    d1: list[int] = first_difference([1, 4, 9, 16, 25])
    if d1[0] == 3 and d1[1] == 5 and d1[2] == 7:
        ok = ok + 1
    d2: list[int] = second_difference([1, 4, 9, 16, 25])
    if d2[0] == 2 and d2[1] == 2 and d2[2] == 2:
        ok = ok + 1
    cs: list[int] = cumulative_sum([1, 2, 3, 4, 5])
    if cs[0] == 1 and cs[4] == 15:
        ok = ok + 1
    restored: list[int] = restore_from_diff(1, [3, 5, 7, 9])
    if restored[0] == 1 and restored[4] == 25:
        ok = ok + 1
    if is_arithmetic_sequence([2, 5, 8, 11]) == 1:
        ok = ok + 1
    if is_arithmetic_sequence([1, 4, 9, 16]) == 0:
        ok = ok + 1
    return ok
