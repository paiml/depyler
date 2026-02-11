"""Inversion counting in arrays.

Tests: count inversions, sorted check, sortedness measure.
"""


def count_inversions(arr: list[int]) -> int:
    """Count inversions: pairs (i,j) where i < j but arr[i] > arr[j]."""
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


def count_strict_inversions(arr: list[int]) -> int:
    """Count strict inversions (no ties)."""
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


def max_inversions(n: int) -> int:
    """Maximum possible inversions for array of size n."""
    return n * (n - 1) // 2


def sortedness_score(arr: list[int]) -> int:
    """Measure how sorted the array is. 100 = fully sorted, 0 = reversed.
    Returns percentage."""
    n: int = len(arr)
    if n <= 1:
        return 100
    max_inv: int = max_inversions(n)
    if max_inv == 0:
        return 100
    inv: int = count_inversions(arr)
    score: int = 100 - (inv * 100 // max_inv)
    return score


def test_module() -> int:
    """Test inversion counting operations."""
    ok: int = 0
    if count_inversions([2, 4, 1, 3, 5]) == 3:
        ok = ok + 1
    if count_inversions([1, 2, 3]) == 0:
        ok = ok + 1
    if count_inversions([3, 2, 1]) == 3:
        ok = ok + 1
    if max_inversions(4) == 6:
        ok = ok + 1
    if sortedness_score([1, 2, 3, 4]) == 100:
        ok = ok + 1
    if sortedness_score([4, 3, 2, 1]) == 0:
        ok = ok + 1
    return ok
