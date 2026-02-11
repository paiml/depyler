"""Stair climbing DP problems.

Tests: ways to climb 1/2 steps, 1/2/3 steps, min cost, max reach.
"""


def climb_stairs_two(n: int) -> int:
    """Number of ways to climb n stairs taking 1 or 2 steps."""
    if n <= 1:
        return 1
    a: int = 1
    b: int = 1
    i: int = 2
    while i <= n:
        c: int = a + b
        a = b
        b = c
        i = i + 1
    return b


def climb_stairs_three(n: int) -> int:
    """Number of ways to climb n stairs taking 1, 2, or 3 steps."""
    if n <= 1:
        return 1
    if n == 2:
        return 2
    a: int = 1
    b: int = 1
    c: int = 2
    i: int = 3
    while i <= n:
        d: int = a + b + c
        a = b
        b = c
        c = d
        i = i + 1
    return c


def min_cost_stairs(costs: list[int]) -> int:
    """Minimum cost to climb stairs with cost at each step."""
    n: int = len(costs)
    if n == 0:
        return 0
    if n == 1:
        return costs[0]
    a: int = costs[0]
    b: int = costs[1]
    i: int = 2
    while i < n:
        c: int = costs[i]
        if a < b:
            c = c + a
        else:
            c = c + b
        a = b
        b = c
        i = i + 1
    if a < b:
        return a
    return b


def max_reachable_stair(energy: int, step_cost: int) -> int:
    """Max stair reachable with given energy and cost per step."""
    stair: int = 0
    remaining: int = energy
    while remaining >= step_cost:
        remaining = remaining - step_cost
        stair = stair + 1
    return stair


def test_module() -> int:
    """Test stair climbing."""
    ok: int = 0
    if climb_stairs_two(5) == 8:
        ok = ok + 1
    if climb_stairs_two(1) == 1:
        ok = ok + 1
    if climb_stairs_three(4) == 7:
        ok = ok + 1
    if min_cost_stairs([10, 15, 20]) == 15:
        ok = ok + 1
    if min_cost_stairs([1, 100, 1, 1, 1, 100, 1, 1, 100, 1]) == 6:
        ok = ok + 1
    if max_reachable_stair(100, 15) == 6:
        ok = ok + 1
    return ok
