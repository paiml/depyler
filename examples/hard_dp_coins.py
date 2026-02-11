"""Coin change variants: min coins, count ways, greedy check."""


def min_coins(coins: list[int], amount: int) -> int:
    """Find minimum number of coins to make amount. Returns -1 if impossible."""
    dp: list[int] = []
    i: int = 0
    while i <= amount:
        dp.append(999999)
        i = i + 1
    dp[0] = 0
    a: int = 1
    while a <= amount:
        ci: int = 0
        while ci < len(coins):
            if coins[ci] <= a and dp[a - coins[ci]] != 999999:
                new_val: int = dp[a - coins[ci]] + 1
                if new_val < dp[a]:
                    dp[a] = new_val
            ci = ci + 1
        a = a + 1
    if dp[amount] == 999999:
        return -1
    return dp[amount]


def count_ways(coins: list[int], amount: int) -> int:
    """Count number of ways to make amount using coins (order doesn't matter)."""
    dp: list[int] = []
    i: int = 0
    while i <= amount:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    ci: int = 0
    while ci < len(coins):
        a: int = coins[ci]
        while a <= amount:
            dp[a] = dp[a] + dp[a - coins[ci]]
            a = a + 1
        ci = ci + 1
    return dp[amount]


def greedy_coins(coins: list[int], amount: int) -> int:
    """Greedy coin count (may not be optimal). Coins must be sorted descending."""
    count: int = 0
    remaining: int = amount
    ci: int = 0
    while ci < len(coins) and remaining > 0:
        while remaining >= coins[ci]:
            remaining = remaining - coins[ci]
            count = count + 1
        ci = ci + 1
    if remaining > 0:
        return -1
    return count


def is_greedy_optimal(coins: list[int], amount: int) -> int:
    """Check if greedy gives optimal result. Returns 1 or 0."""
    greedy_result: int = greedy_coins(coins, amount)
    optimal_result: int = min_coins(coins, amount)
    if greedy_result == optimal_result:
        return 1
    return 0


def test_module() -> int:
    passed: int = 0

    if min_coins([1, 5, 10, 25], 30) == 2:
        passed = passed + 1

    if min_coins([2], 3) == -1:
        passed = passed + 1

    if count_ways([1, 2, 5], 5) == 4:
        passed = passed + 1

    if count_ways([1], 0) == 1:
        passed = passed + 1

    if greedy_coins([25, 10, 5, 1], 41) == 4:
        passed = passed + 1

    if min_coins([1, 5, 10, 25], 0) == 0:
        passed = passed + 1

    if is_greedy_optimal([25, 10, 5, 1], 30) == 1:
        passed = passed + 1

    return passed
