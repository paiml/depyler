"""Stock buy/sell profit maximization."""


def max_profit_single(prices: list[int]) -> int:
    """Find maximum profit from a single buy/sell transaction."""
    length: int = len(prices)
    if length < 2:
        return 0
    min_price: int = prices[0]
    max_profit: int = 0
    i: int = 1
    while i < length:
        profit: int = prices[i] - min_price
        if profit > max_profit:
            max_profit = profit
        if prices[i] < min_price:
            min_price = prices[i]
        i = i + 1
    return max_profit


def max_profit_unlimited(prices: list[int]) -> int:
    """Find maximum profit with unlimited transactions."""
    length: int = len(prices)
    if length < 2:
        return 0
    total: int = 0
    i: int = 1
    while i < length:
        diff: int = prices[i] - prices[i - 1]
        if diff > 0:
            total = total + diff
        i = i + 1
    return total


def best_buy_day(prices: list[int]) -> int:
    """Find the best day to buy (for single transaction)."""
    length: int = len(prices)
    if length < 2:
        return 0
    min_price: int = prices[0]
    min_day: int = 0
    best_day: int = 0
    max_profit: int = 0
    i: int = 1
    while i < length:
        if prices[i] < min_price:
            min_price = prices[i]
            min_day = i
        profit: int = prices[i] - min_price
        if profit > max_profit:
            max_profit = profit
            best_day = min_day
        i = i + 1
    return best_day


def count_profitable_days(prices: list[int]) -> int:
    """Count days where price increased from previous day."""
    length: int = len(prices)
    if length < 2:
        return 0
    count: int = 0
    i: int = 1
    while i < length:
        if prices[i] > prices[i - 1]:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test stock profit operations."""
    passed: int = 0

    if max_profit_single([7, 1, 5, 3, 6, 4]) == 5:
        passed = passed + 1

    if max_profit_single([7, 6, 4, 3, 1]) == 0:
        passed = passed + 1

    if max_profit_unlimited([7, 1, 5, 3, 6, 4]) == 7:
        passed = passed + 1

    if max_profit_unlimited([1, 2, 3, 4, 5]) == 4:
        passed = passed + 1

    if best_buy_day([7, 1, 5, 3, 6, 4]) == 1:
        passed = passed + 1

    if count_profitable_days([7, 1, 5, 3, 6, 4]) == 3:
        passed = passed + 1

    if max_profit_single([]) == 0:
        passed = passed + 1

    if max_profit_single([5]) == 0:
        passed = passed + 1

    return passed
