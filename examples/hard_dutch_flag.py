"""Dutch national flag problem variants.

Tests: three-way partition, count partitions, two-color sort, flag check.
"""


def dutch_flag_count(arr: list[int], pivot: int) -> list[int]:
    """Count elements less than, equal to, and greater than pivot."""
    less: int = 0
    equal: int = 0
    greater: int = 0
    i: int = 0
    while i < len(arr):
        if arr[i] < pivot:
            less = less + 1
        elif arr[i] == pivot:
            equal = equal + 1
        else:
            greater = greater + 1
        i = i + 1
    result: list[int] = [less, equal, greater]
    return result


def count_zeros_ones(arr: list[int]) -> list[int]:
    """Count zeros and ones in a binary array."""
    zeros: int = 0
    ones: int = 0
    i: int = 0
    while i < len(arr):
        if arr[i] == 0:
            zeros = zeros + 1
        else:
            ones = ones + 1
        i = i + 1
    return [zeros, ones]


def is_partitioned(arr: list[int], pivot: int) -> int:
    """Check if array is partitioned around pivot. Returns 1 if yes."""
    n: int = len(arr)
    if n <= 1:
        return 1
    phase: int = 0
    i: int = 0
    while i < n:
        if phase == 0:
            if arr[i] == pivot:
                phase = 1
            elif arr[i] > pivot:
                return 0
        elif phase == 1:
            if arr[i] > pivot:
                phase = 2
            elif arr[i] < pivot:
                return 0
        else:
            if arr[i] <= pivot:
                return 0
        i = i + 1
    return 1


def count_inversions_simple(arr: list[int]) -> int:
    """Count number of inversions (i < j but arr[i] > arr[j])."""
    n: int = len(arr)
    count: int = 0
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            if arr[i] > arr[j]:
                count = count + 1
            j = j + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test Dutch flag operations."""
    ok: int = 0
    counts: list[int] = dutch_flag_count([1, 3, 2, 3, 1, 2, 3], 2)
    if counts[0] == 2:
        ok = ok + 1
    if counts[1] == 2:
        ok = ok + 1
    if counts[2] == 3:
        ok = ok + 1
    zc: list[int] = count_zeros_ones([0, 1, 0, 1, 1])
    if zc[0] == 2:
        ok = ok + 1
    if zc[1] == 3:
        ok = ok + 1
    if is_partitioned([1, 1, 2, 3, 3], 2) == 1:
        ok = ok + 1
    if count_inversions_simple([2, 4, 1, 3, 5]) == 3:
        ok = ok + 1
    return ok
