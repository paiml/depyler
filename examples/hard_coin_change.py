"""Coin change problem variants.

Tests: minimum coins, number of ways, reachable amounts.
"""


def min_coins(coins: list[int], amount: int) -> int:
    """Minimum number of coins to make amount. Returns -1 if impossible."""
    big: int = amount + 1
    dp: list[int] = [big] * (amount + 1)
    dp[0] = 0
    i: int = 1
    while i <= amount:
        j: int = 0
        while j < len(coins):
            if coins[j] <= i:
                candidate: int = dp[i - coins[j]] + 1
                if candidate < dp[i]:
                    dp[i] = candidate
            j = j + 1
        i = i + 1
    if dp[amount] > amount:
        return -1
    return dp[amount]


def count_ways(coins: list[int], amount: int) -> int:
    """Count number of ways to make amount (order doesn't matter)."""
    dp: list[int] = [0] * (amount + 1)
    dp[0] = 1
    i: int = 0
    while i < len(coins):
        j: int = coins[i]
        while j <= amount:
            dp[j] = dp[j] + dp[j - coins[i]]
            j = j + 1
        i = i + 1
    return dp[amount]


def count_permutations(coins: list[int], amount: int) -> int:
    """Count number of ways to make amount (order matters)."""
    dp: list[int] = [0] * (amount + 1)
    dp[0] = 1
    i: int = 1
    while i <= amount:
        j: int = 0
        while j < len(coins):
            if coins[j] <= i:
                dp[i] = dp[i] + dp[i - coins[j]]
            j = j + 1
        i = i + 1
    return dp[amount]


def max_reachable(coins: list[int], amount: int) -> int:
    """Find the largest amount <= given amount that can be made."""
    dp: list[bool] = [False] * (amount + 1)
    dp[0] = True
    i: int = 0
    while i < len(coins):
        j: int = coins[i]
        while j <= amount:
            if dp[j - coins[i]]:
                dp[j] = True
            j = j + 1
        i = i + 1
    best: int = 0
    k: int = amount
    while k >= 0:
        if dp[k]:
            best = k
            k = -1
        else:
            k = k - 1
    return best


def test_module() -> None:
    coins: list[int] = [1, 5, 10, 25]
    assert min_coins(coins, 30) == 2
    assert min_coins(coins, 11) == 2
    assert min_coins([2], 3) == -1
    assert count_ways(coins, 10) == 4
    assert count_ways([1, 2], 4) == 3
    assert count_permutations([1, 2], 4) == 5
    assert max_reachable([3, 7], 10) == 10
    assert max_reachable([3, 7], 8) == 7
