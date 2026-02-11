"""Next permutation and permutation operations.

Tests: next permutation, is sorted, permutation rank estimate.
"""


def next_permutation(arr: list[int]) -> list[int]:
    """Compute next lexicographic permutation. Returns same if last."""
    n: int = len(arr)
    result: list[int] = []
    i: int = 0
    while i < n:
        result.append(arr[i])
        i = i + 1
    k: int = n - 2
    while k >= 0:
        if result[k] < result[k + 1]:
            break
        k = k - 1
    if k < 0:
        return result
    l: int = n - 1
    while l > k:
        if result[l] > result[k]:
            break
        l = l - 1
    tmp: int = result[k]
    result[k] = result[l]
    result[l] = tmp
    lo: int = k + 1
    hi: int = n - 1
    while lo < hi:
        tmp2: int = result[lo]
        result[lo] = result[hi]
        result[hi] = tmp2
        lo = lo + 1
        hi = hi - 1
    return result


def is_sorted_ascending(arr: list[int]) -> int:
    """Check if array is sorted in ascending order. Returns 1 if yes."""
    i: int = 1
    while i < len(arr):
        if arr[i] < arr[i - 1]:
            return 0
        i = i + 1
    return 1


def is_sorted_descending(arr: list[int]) -> int:
    """Check if array is sorted in descending order. Returns 1 if yes."""
    i: int = 1
    while i < len(arr):
        if arr[i] > arr[i - 1]:
            return 0
        i = i + 1
    return 1


def count_ascending_runs(arr: list[int]) -> int:
    """Count number of ascending runs in array."""
    if len(arr) == 0:
        return 0
    runs: int = 1
    i: int = 1
    while i < len(arr):
        if arr[i] < arr[i - 1]:
            runs = runs + 1
        i = i + 1
    return runs


def test_module() -> int:
    """Test permutation operations."""
    ok: int = 0
    arr: list[int] = [1, 2, 3]
    nxt: list[int] = next_permutation(arr)
    if nxt[0] == 1:
        ok = ok + 1
    if nxt[1] == 3:
        ok = ok + 1
    if nxt[2] == 2:
        ok = ok + 1
    if is_sorted_ascending([1, 2, 3, 4]) == 1:
        ok = ok + 1
    if is_sorted_descending([4, 3, 2, 1]) == 1:
        ok = ok + 1
    if count_ascending_runs([1, 3, 2, 4, 1]) == 3:
        ok = ok + 1
    return ok
