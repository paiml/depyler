"""Coin change problem: minimum coins needed and number of ways to make change.

Tests: min coins for various amounts, number of ways, edge cases with zero and impossible amounts.
"""


def min_coins(coins: list[int], amount: int) -> int:
    """Return minimum number of coins needed to make amount, or -1 if impossible."""
    dp: list[int] = []
    i: int = 0
    while i <= amount:
        dp.append(amount + 1)
        i = i + 1
    dp[0] = 0
    a: int = 1
    while a <= amount:
        j: int = 0
        while j < len(coins):
            if coins[j] <= a:
                prev: int = dp[a - coins[j]] + 1
                if prev < dp[a]:
                    dp[a] = prev
            j = j + 1
        a = a + 1
    if dp[amount] > amount:
        return -1
    return dp[amount]


def count_ways(coins: list[int], amount: int) -> int:
    """Return the number of distinct ways to make change for amount."""
    dp: list[int] = []
    i: int = 0
    while i <= amount:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    j: int = 0
    while j < len(coins):
        a: int = coins[j]
        while a <= amount:
            dp[a] = dp[a] + dp[a - coins[j]]
            a = a + 1
        j = j + 1
    return dp[amount]


def min_coins_single(coin: int, amount: int) -> int:
    """Return min coins when only one denomination is available."""
    if amount % coin != 0:
        return -1
    return amount // coin


def test_module() -> int:
    """Test coin change algorithms."""
    ok: int = 0

    coins1: list[int] = [1, 5, 10, 25]
    if min_coins(coins1, 30) == 2:
        ok = ok + 1
    if min_coins(coins1, 11) == 2:
        ok = ok + 1
    if min_coins(coins1, 0) == 0:
        ok = ok + 1

    coins2: list[int] = [2]
    if min_coins(coins2, 3) == -1:
        ok = ok + 1

    coins3: list[int] = [1, 2, 5]
    if min_coins(coins3, 11) == 3:
        ok = ok + 1

    if count_ways(coins1, 10) == 4:
        ok = ok + 1
    if count_ways(coins1, 0) == 1:
        ok = ok + 1

    coins4: list[int] = [1, 2, 3]
    if count_ways(coins4, 4) == 4:
        ok = ok + 1

    if min_coins_single(5, 25) == 5:
        ok = ok + 1
    if min_coins_single(3, 7) == -1:
        ok = ok + 1

    return ok
