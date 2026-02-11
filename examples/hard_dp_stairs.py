"""Stair climbing variants: 1/2/3 steps with DP."""


def climb_two_steps(n: int) -> int:
    """Count ways to climb n stairs using 1 or 2 steps."""
    if n <= 1:
        return 1
    prev2: int = 1
    prev1: int = 1
    i: int = 2
    while i <= n:
        curr: int = prev1 + prev2
        prev2 = prev1
        prev1 = curr
        i = i + 1
    return prev1


def climb_three_steps(n: int) -> int:
    """Count ways to climb n stairs using 1, 2, or 3 steps."""
    if n == 0:
        return 1
    if n == 1:
        return 1
    if n == 2:
        return 2
    a: int = 1
    b: int = 1
    c: int = 2
    i: int = 3
    while i <= n:
        curr: int = a + b + c
        a = b
        b = c
        c = curr
        i = i + 1
    return c


def min_cost_climb(cost: list[int]) -> int:
    """Minimum cost to reach top, can step 1 or 2 at a time."""
    n: int = len(cost)
    if n == 0:
        return 0
    if n == 1:
        return cost[0]
    prev2: int = cost[0]
    prev1: int = cost[1]
    i: int = 2
    while i < n:
        curr: int = cost[i]
        if prev2 < prev1:
            curr = curr + prev2
        else:
            curr = curr + prev1
        prev2 = prev1
        prev1 = curr
        i = i + 1
    if prev2 < prev1:
        return prev2
    return prev1


def climb_with_max_step(n: int, max_step: int) -> int:
    """Count ways using steps from 1 to max_step."""
    dp: list[int] = []
    idx: int = 0
    while idx <= n:
        dp.append(0)
        idx = idx + 1
    dp[0] = 1
    i: int = 1
    while i <= n:
        s: int = 1
        while s <= max_step and s <= i:
            dp[i] = dp[i] + dp[i - s]
            s = s + 1
        i = i + 1
    return dp[n]


def test_module() -> int:
    passed: int = 0

    if climb_two_steps(5) == 8:
        passed = passed + 1

    if climb_two_steps(0) == 1:
        passed = passed + 1

    if climb_three_steps(4) == 7:
        passed = passed + 1

    if climb_three_steps(3) == 4:
        passed = passed + 1

    cost1: list[int] = [10, 15, 20]
    if min_cost_climb(cost1) == 15:
        passed = passed + 1

    cost2: list[int] = [1, 100, 1, 1, 1, 100, 1, 1, 100, 1]
    if min_cost_climb(cost2) == 6:
        passed = passed + 1

    if climb_with_max_step(4, 3) == 7:
        passed = passed + 1

    if climb_with_max_step(3, 2) == 3:
        passed = passed + 1

    return passed
