"""Unbounded and bounded 0/1 knapsack problems."""


def knapsack_01(weights: list[int], values: list[int], capacity: int) -> int:
    """0/1 knapsack: each item used at most once."""
    n: int = len(weights)
    dp: list[int] = []
    i: int = 0
    while i <= capacity:
        dp.append(0)
        i = i + 1
    item: int = 0
    while item < n:
        w: int = capacity
        while w >= weights[item]:
            new_val: int = dp[w - weights[item]] + values[item]
            if new_val > dp[w]:
                dp[w] = new_val
            w = w - 1
        item = item + 1
    return dp[capacity]


def knapsack_unbounded(weights: list[int], values: list[int], capacity: int) -> int:
    """Unbounded knapsack: each item can be used multiple times."""
    dp: list[int] = []
    i: int = 0
    while i <= capacity:
        dp.append(0)
        i = i + 1
    w: int = 1
    while w <= capacity:
        item: int = 0
        while item < len(weights):
            if weights[item] <= w:
                new_val: int = dp[w - weights[item]] + values[item]
                if new_val > dp[w]:
                    dp[w] = new_val
            item = item + 1
        w = w + 1
    return dp[capacity]


def can_fill_exactly(weights: list[int], target: int) -> int:
    """Check if we can fill knapsack to exact target weight. Returns 1 or 0."""
    dp: list[int] = []
    i: int = 0
    while i <= target:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    item: int = 0
    while item < len(weights):
        w: int = target
        while w >= weights[item]:
            if dp[w - weights[item]] == 1:
                dp[w] = 1
            w = w - 1
        item = item + 1
    return dp[target]


def test_module() -> int:
    passed: int = 0

    v1: int = knapsack_01([1, 3, 4, 5], [1, 4, 5, 7], 7)
    if v1 == 9:
        passed = passed + 1

    v2: int = knapsack_01([2, 3, 4], [3, 4, 5], 5)
    if v2 == 7:
        passed = passed + 1

    v3: int = knapsack_unbounded([1, 3, 4, 5], [1, 4, 5, 7], 7)
    if v3 == 9:
        passed = passed + 1

    v4: int = knapsack_unbounded([2, 3], [3, 4], 7)
    if v4 == 10:
        passed = passed + 1

    if can_fill_exactly([1, 5, 11], 12) == 1:
        passed = passed + 1

    if can_fill_exactly([3, 5], 4) == 0:
        passed = passed + 1

    v5: int = knapsack_01([], [], 10)
    if v5 == 0:
        passed = passed + 1

    return passed
