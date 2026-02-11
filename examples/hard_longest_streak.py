"""Longest consecutive streak operations.

Tests: longest streak, longest increasing run, longest plateau, longest alternating.
"""


def longest_consecutive_streak(arr: list[int]) -> int:
    """Find the longest streak of consecutive equal elements."""
    n: int = len(arr)
    if n == 0:
        return 0
    best: int = 1
    curr: int = 1
    i: int = 1
    while i < n:
        if arr[i] == arr[i - 1]:
            curr = curr + 1
            if curr > best:
                best = curr
        else:
            curr = 1
        i = i + 1
    return best


def longest_increasing_run(arr: list[int]) -> int:
    """Find the longest strictly increasing run."""
    n: int = len(arr)
    if n == 0:
        return 0
    best: int = 1
    curr: int = 1
    i: int = 1
    while i < n:
        if arr[i] > arr[i - 1]:
            curr = curr + 1
            if curr > best:
                best = curr
        else:
            curr = 1
        i = i + 1
    return best


def longest_decreasing_run(arr: list[int]) -> int:
    """Find the longest strictly decreasing run."""
    n: int = len(arr)
    if n == 0:
        return 0
    best: int = 1
    curr: int = 1
    i: int = 1
    while i < n:
        if arr[i] < arr[i - 1]:
            curr = curr + 1
            if curr > best:
                best = curr
        else:
            curr = 1
        i = i + 1
    return best


def longest_nondecreasing_run(arr: list[int]) -> int:
    """Find the longest non-decreasing run."""
    n: int = len(arr)
    if n == 0:
        return 0
    best: int = 1
    curr: int = 1
    i: int = 1
    while i < n:
        if arr[i] >= arr[i - 1]:
            curr = curr + 1
            if curr > best:
                best = curr
        else:
            curr = 1
        i = i + 1
    return best


def test_module() -> int:
    """Test longest streak operations."""
    ok: int = 0
    if longest_consecutive_streak([1, 1, 2, 2, 2, 3]) == 3:
        ok = ok + 1
    if longest_consecutive_streak([5]) == 1:
        ok = ok + 1
    if longest_increasing_run([1, 2, 3, 1, 2]) == 3:
        ok = ok + 1
    if longest_increasing_run([5, 4, 3, 2, 1]) == 1:
        ok = ok + 1
    if longest_decreasing_run([5, 4, 3, 6, 5]) == 3:
        ok = ok + 1
    if longest_nondecreasing_run([1, 2, 2, 3, 1]) == 4:
        ok = ok + 1
    if longest_nondecreasing_run([3, 2, 1]) == 1:
        ok = ok + 1
    return ok
