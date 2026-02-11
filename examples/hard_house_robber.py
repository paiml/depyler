"""House robber DP problem variants.

Tests: basic robber, circular robber, max non-adjacent sum.
"""


def house_robber(houses: list[int]) -> int:
    """Max money robbing non-adjacent houses."""
    n: int = len(houses)
    if n == 0:
        return 0
    if n == 1:
        return houses[0]
    prev2: int = 0
    prev1: int = houses[0]
    i: int = 1
    while i < n:
        curr: int = prev1
        candidate: int = prev2 + houses[i]
        if candidate > curr:
            curr = candidate
        prev2 = prev1
        prev1 = curr
        i = i + 1
    return prev1


def max_non_adjacent_sum(values: list[int]) -> int:
    """Max sum of non-adjacent elements."""
    return house_robber(values)


def house_robber_range(houses: list[int], start: int, end: int) -> int:
    """Robber on subarray houses[start..end]."""
    if start > end:
        return 0
    if start == end:
        return houses[start]
    prev2: int = 0
    prev1: int = houses[start]
    i: int = start + 1
    while i <= end:
        curr: int = prev1
        candidate: int = prev2 + houses[i]
        if candidate > curr:
            curr = candidate
        prev2 = prev1
        prev1 = curr
        i = i + 1
    return prev1


def house_robber_circular(houses: list[int]) -> int:
    """Circular street: first and last are adjacent."""
    n: int = len(houses)
    if n == 0:
        return 0
    if n == 1:
        return houses[0]
    a: int = house_robber_range(houses, 0, n - 2)
    b: int = house_robber_range(houses, 1, n - 1)
    if a > b:
        return a
    return b


def test_module() -> int:
    """Test house robber."""
    ok: int = 0
    if house_robber([1, 2, 3, 1]) == 4:
        ok = ok + 1
    if house_robber([2, 7, 9, 3, 1]) == 12:
        ok = ok + 1
    if house_robber([]) == 0:
        ok = ok + 1
    if max_non_adjacent_sum([5, 1, 1, 5]) == 10:
        ok = ok + 1
    if house_robber_circular([2, 3, 2]) == 3:
        ok = ok + 1
    if house_robber_circular([1, 2, 3, 1]) == 4:
        ok = ok + 1
    return ok
