"""Greedy coin change: minimize number of coins for given denominations."""


def greedy_coin_count(denominations: list[int], amount: int) -> int:
    """Count minimum coins using greedy (assumes sorted descending works optimally).
    Denominations must be sorted descending."""
    remaining: int = amount
    count: int = 0
    idx: int = 0
    length: int = len(denominations)
    while idx < length:
        while remaining >= denominations[idx]:
            remaining = remaining - denominations[idx]
            count = count + 1
        idx = idx + 1
    if remaining > 0:
        return -1
    return count


def greedy_coin_list(denominations: list[int], amount: int) -> list[int]:
    """Return list of coins used (greedy). Denominations sorted descending."""
    remaining: int = amount
    coins_used: list[int] = []
    idx: int = 0
    length: int = len(denominations)
    while idx < length:
        while remaining >= denominations[idx]:
            remaining = remaining - denominations[idx]
            coins_used.append(denominations[idx])
        idx = idx + 1
    return coins_used


def count_each_denomination(denominations: list[int], amount: int) -> list[int]:
    """Return how many of each denomination used. Same order as input."""
    remaining: int = amount
    result: list[int] = []
    idx: int = 0
    length: int = len(denominations)
    while idx < length:
        coin_count: int = 0
        while remaining >= denominations[idx]:
            remaining = remaining - denominations[idx]
            coin_count = coin_count + 1
        result.append(coin_count)
        idx = idx + 1
    return result


def test_module() -> int:
    passed: int = 0

    denoms: list[int] = [25, 10, 5, 1]
    if greedy_coin_count(denoms, 41) == 4:
        passed = passed + 1
    if greedy_coin_count(denoms, 25) == 1:
        passed = passed + 1

    coins: list[int] = greedy_coin_list(denoms, 30)
    if coins[0] == 25:
        passed = passed + 1
    if coins[1] == 5:
        passed = passed + 1

    each: list[int] = count_each_denomination(denoms, 41)
    if each[0] == 1:
        passed = passed + 1
    if each[1] == 1:
        passed = passed + 1

    if greedy_coin_count([10, 5, 1], 0) == 0:
        passed = passed + 1

    return passed
