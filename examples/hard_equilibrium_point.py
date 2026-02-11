"""Equilibrium index finding in arrays.

Tests: equilibrium index, balance point, prefix-suffix equality.
"""


def find_equilibrium_index(arr: list[int]) -> int:
    """Find index where left sum equals right sum. Returns -1 if none."""
    n: int = len(arr)
    if n == 0:
        return -1
    total: int = 0
    i: int = 0
    while i < n:
        total = total + arr[i]
        i = i + 1
    left_sum: int = 0
    j: int = 0
    while j < n:
        right_sum: int = total - left_sum - arr[j]
        if left_sum == right_sum:
            return j
        left_sum = left_sum + arr[j]
        j = j + 1
    return -1


def count_equilibrium_points(arr: list[int]) -> int:
    """Count all equilibrium indices."""
    n: int = len(arr)
    total: int = 0
    i: int = 0
    while i < n:
        total = total + arr[i]
        i = i + 1
    count: int = 0
    left_sum: int = 0
    j: int = 0
    while j < n:
        right_sum: int = total - left_sum - arr[j]
        if left_sum == right_sum:
            count = count + 1
        left_sum = left_sum + arr[j]
        j = j + 1
    return count


def prefix_sums(arr: list[int]) -> list[int]:
    """Compute prefix sum array."""
    result: list[int] = []
    running: int = 0
    i: int = 0
    while i < len(arr):
        running = running + arr[i]
        result.append(running)
        i = i + 1
    return result


def test_module() -> int:
    """Test equilibrium operations."""
    ok: int = 0
    arr: list[int] = [-7, 1, 5, 2, -4, 3, 0]
    idx: int = find_equilibrium_index(arr)
    if idx == 3:
        ok = ok + 1
    if count_equilibrium_points(arr) == 1:
        ok = ok + 1
    arr2: list[int] = [1, 2, 3]
    if find_equilibrium_index(arr2) == -1:
        ok = ok + 1
    ps: list[int] = prefix_sums([1, 2, 3, 4])
    if ps[0] == 1:
        ok = ok + 1
    if ps[3] == 10:
        ok = ok + 1
    return ok
