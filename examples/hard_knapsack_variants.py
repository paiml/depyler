"""0/1 Knapsack problem variants.

Tests: basic knapsack, bounded knapsack value, item count.
"""


def knapsack_01(weights: list[int], values: list[int], capacity: int) -> int:
    """Classic 0/1 knapsack - maximum value within capacity."""
    n: int = len(weights)
    dp: list[int] = [0] * (capacity + 1)
    i: int = 0
    while i < n:
        w: int = capacity
        while w >= weights[i]:
            candidate: int = dp[w - weights[i]] + values[i]
            if candidate > dp[w]:
                dp[w] = candidate
            w = w - 1
        i = i + 1
    return dp[capacity]


def knapsack_count(weights: list[int], capacity: int) -> int:
    """Count number of ways to exactly fill the knapsack."""
    dp: list[int] = [0] * (capacity + 1)
    dp[0] = 1
    i: int = 0
    while i < len(weights):
        w: int = capacity
        while w >= weights[i]:
            dp[w] = dp[w] + dp[w - weights[i]]
            w = w - 1
        i = i + 1
    return dp[capacity]


def knapsack_max_items(weights: list[int], capacity: int) -> int:
    """Maximum number of items that fit in knapsack (each used once)."""
    big: int = capacity + 1
    dp: list[int] = [big] * (capacity + 1)
    dp[0] = 0
    items: list[int] = [0] * (capacity + 1)
    i: int = 0
    while i < len(weights):
        w: int = capacity
        while w >= weights[i]:
            if dp[w - weights[i]] + weights[i] <= w:
                new_items: int = items[w - weights[i]] + 1
                new_weight: int = dp[w - weights[i]] + weights[i]
                if new_items > items[w] or (new_items == items[w] and new_weight < dp[w]):
                    dp[w] = new_weight
                    items[w] = new_items
            w = w - 1
        i = i + 1
    return items[capacity]


def can_fill_exactly_val(weights: list[int], target: int) -> int:
    """Check if any subset of weights sums to exactly target. Returns 1 if yes, 0 if no."""
    dp: list[int] = [0] * (target + 1)
    dp[0] = 1
    i: int = 0
    while i < len(weights):
        w: int = target
        while w >= weights[i]:
            if dp[w - weights[i]] == 1:
                dp[w] = 1
            w = w - 1
        i = i + 1
    return dp[target]


def test_module() -> None:
    w: list[int] = [2, 3, 4, 5]
    v: list[int] = [3, 4, 5, 6]
    assert knapsack_01(w, v, 5) == 7
    assert knapsack_01(w, v, 8) == 10
    assert knapsack_count([1, 2, 3], 3) == 2
    assert knapsack_count([1, 2, 3], 4) == 1
    assert can_fill_exactly_val([1, 5, 11, 5], 11) == 1
    assert can_fill_exactly_val([1, 5, 3], 10) == 0
    assert knapsack_max_items([1, 2, 3, 4], 6) == 3
