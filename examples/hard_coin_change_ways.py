"""Number of ways to make change using dynamic programming."""


def coin_change_ways(coins: list[int], amount: int) -> int:
    """Count number of ways to make change for amount using given coins."""
    dp: list[int] = []
    i: int = 0
    while i <= amount:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    ci: int = 0
    while ci < len(coins):
        coin: int = coins[ci]
        j: int = coin
        while j <= amount:
            dp[j] = dp[j] + dp[j - coin]
            j = j + 1
        ci = ci + 1
    return dp[amount]


def min_coins(coins: list[int], amount: int) -> int:
    """Find minimum number of coins to make amount. Returns -1 if impossible."""
    big: int = amount + 1
    dp: list[int] = []
    i: int = 0
    while i <= amount:
        dp.append(big)
        i = i + 1
    dp[0] = 0
    ci: int = 0
    while ci < len(coins):
        coin: int = coins[ci]
        j: int = coin
        while j <= amount:
            candidate: int = dp[j - coin] + 1
            if candidate < dp[j]:
                dp[j] = candidate
            j = j + 1
        ci = ci + 1
    if dp[amount] > amount:
        return 0 - 1
    return dp[amount]


def can_make_change(coins: list[int], amount: int) -> int:
    """Check if we can make exact change. Returns 1 if yes."""
    result: int = min_coins(coins, amount)
    if result == 0 - 1:
        return 0
    return 1


def largest_amount_impossible(coins: list[int], limit: int) -> int:
    """Find largest amount up to limit that cannot be made. Returns 0 if all possible."""
    largest: int = 0
    amt: int = 1
    while amt <= limit:
        if can_make_change(coins, amt) == 0:
            largest = amt
        amt = amt + 1
    return largest


def test_module() -> int:
    """Test coin change ways."""
    passed: int = 0

    coins1: list[int] = [1, 2, 5]
    if coin_change_ways(coins1, 5) == 4:
        passed = passed + 1

    if coin_change_ways(coins1, 0) == 1:
        passed = passed + 1

    if min_coins(coins1, 11) == 3:
        passed = passed + 1

    coins2: list[int] = [2]
    if min_coins(coins2, 3) == 0 - 1:
        passed = passed + 1

    if can_make_change(coins1, 100) == 1:
        passed = passed + 1

    coins3: list[int] = [3, 5]
    if largest_amount_impossible(coins3, 20) == 7:
        passed = passed + 1

    return passed
