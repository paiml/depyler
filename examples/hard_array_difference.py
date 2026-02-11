"""Array difference and delta operations.

Tests: element-wise diff, running diff, max delta, symmetric difference count.
"""


def elementwise_diff(a: list[int], b: list[int]) -> list[int]:
    """Element-wise difference a[i] - b[i]."""
    result: list[int] = []
    n: int = len(a)
    if len(b) < n:
        n = len(b)
    i: int = 0
    while i < n:
        result.append(a[i] - b[i])
        i = i + 1
    return result


def running_difference(arr: list[int]) -> list[int]:
    """Compute arr[i+1] - arr[i] for consecutive pairs."""
    result: list[int] = []
    n: int = len(arr)
    i: int = 0
    while i < n - 1:
        result.append(arr[i + 1] - arr[i])
        i = i + 1
    return result


def max_absolute_delta(arr: list[int]) -> int:
    """Maximum absolute difference between consecutive elements."""
    n: int = len(arr)
    if n < 2:
        return 0
    best: int = 0
    i: int = 0
    while i < n - 1:
        delta: int = arr[i + 1] - arr[i]
        if delta < 0:
            delta = -delta
        if delta > best:
            best = delta
        i = i + 1
    return best


def count_increasing_pairs(arr: list[int]) -> int:
    """Count consecutive pairs where arr[i+1] > arr[i]."""
    n: int = len(arr)
    count: int = 0
    i: int = 0
    while i < n - 1:
        if arr[i + 1] > arr[i]:
            count = count + 1
        i = i + 1
    return count


def sum_of_diffs(arr: list[int]) -> int:
    """Sum of absolute consecutive differences."""
    n: int = len(arr)
    total: int = 0
    i: int = 0
    while i < n - 1:
        d: int = arr[i + 1] - arr[i]
        if d < 0:
            d = -d
        total = total + d
        i = i + 1
    return total


def test_module() -> int:
    """Test array difference."""
    ok: int = 0
    ed: list[int] = elementwise_diff([5, 10, 15], [1, 2, 3])
    if ed[0] == 4 and ed[1] == 8 and ed[2] == 12:
        ok = ok + 1
    rd: list[int] = running_difference([1, 4, 9, 16])
    if rd[0] == 3 and rd[1] == 5 and rd[2] == 7:
        ok = ok + 1
    if max_absolute_delta([1, 10, 3, 20]) == 17:
        ok = ok + 1
    if count_increasing_pairs([1, 3, 2, 5, 4]) == 2:
        ok = ok + 1
    if sum_of_diffs([1, 5, 2, 8]) == 13:
        ok = ok + 1
    return ok
