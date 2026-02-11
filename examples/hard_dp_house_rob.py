"""House robber problem variants using DP."""


def rob_linear(houses: list[int]) -> int:
    """Max money robbing non-adjacent houses in a line."""
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


def rob_range(houses: list[int], start: int, end: int) -> int:
    """Rob houses in range [start, end) non-adjacent."""
    if start >= end:
        return 0
    if end - start == 1:
        return houses[start]
    prev2: int = 0
    prev1: int = houses[start]
    i: int = start + 1
    while i < end:
        curr: int = prev1
        candidate: int = prev2 + houses[i]
        if candidate > curr:
            curr = candidate
        prev2 = prev1
        prev1 = curr
        i = i + 1
    return prev1


def rob_circular(houses: list[int]) -> int:
    """Max money when houses are in a circle (first and last adjacent)."""
    n: int = len(houses)
    if n == 0:
        return 0
    if n == 1:
        return houses[0]
    option_a: int = rob_range(houses, 0, n - 1)
    option_b: int = rob_range(houses, 1, n)
    if option_a > option_b:
        return option_a
    return option_b


def rob_with_cooldown(houses: list[int]) -> int:
    """Rob with cooldown: must wait one house after robbing."""
    n: int = len(houses)
    if n == 0:
        return 0
    if n == 1:
        return houses[0]
    if n == 2:
        if houses[0] > houses[1]:
            return houses[0]
        return houses[1]
    dp: list[int] = []
    idx: int = 0
    while idx < n:
        dp.append(0)
        idx = idx + 1
    dp[0] = houses[0]
    if houses[1] > houses[0]:
        dp[1] = houses[1]
    else:
        dp[1] = houses[0]
    i: int = 2
    while i < n:
        skip: int = dp[i - 1]
        take: int = houses[i]
        if i >= 3:
            take = take + dp[i - 3]
        else:
            pass
        cooldown_take: int = houses[i] + dp[i - 2]
        best: int = skip
        if take > best:
            best = take
        if cooldown_take > best:
            best = cooldown_take
        dp[i] = best
        i = i + 1
    return dp[n - 1]


def test_module() -> int:
    passed: int = 0

    h1: list[int] = [1, 2, 3, 1]
    if rob_linear(h1) == 4:
        passed = passed + 1

    h2: list[int] = [2, 7, 9, 3, 1]
    if rob_linear(h2) == 12:
        passed = passed + 1

    h3: list[int] = [2, 3, 2]
    if rob_circular(h3) == 3:
        passed = passed + 1

    h4: list[int] = [1, 2, 3, 1]
    if rob_circular(h4) == 4:
        passed = passed + 1

    empty: list[int] = []
    if rob_linear(empty) == 0:
        passed = passed + 1

    h5: list[int] = [5]
    if rob_circular(h5) == 5:
        passed = passed + 1

    h6: list[int] = [2, 1, 1, 2]
    if rob_circular(h6) == 3:
        passed = passed + 1

    return passed
