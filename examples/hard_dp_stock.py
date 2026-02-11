"""Stock trading with DP: max profit with at most 2 transactions."""


def max_profit_one(prices: list[int]) -> int:
    """Max profit with at most 1 transaction."""
    n: int = len(prices)
    if n < 2:
        return 0
    min_price: int = prices[0]
    best: int = 0
    i: int = 1
    while i < n:
        profit: int = prices[i] - min_price
        if profit > best:
            best = profit
        if prices[i] < min_price:
            min_price = prices[i]
        i = i + 1
    return best


def max_profit_unlimited(prices: list[int]) -> int:
    """Max profit with unlimited transactions (buy/sell same day ok)."""
    n: int = len(prices)
    total: int = 0
    i: int = 1
    while i < n:
        diff: int = prices[i] - prices[i - 1]
        if diff > 0:
            total = total + diff
        i = i + 1
    return total


def max_profit_two(prices: list[int]) -> int:
    """Max profit with at most 2 transactions."""
    n: int = len(prices)
    if n < 2:
        return 0
    first_price: int = prices[0]
    buy1: int = 0 - first_price
    sell1: int = 0
    buy2: int = 0 - first_price
    sell2: int = 0
    i: int = 1
    while i < n:
        neg_pi: int = 0 - prices[i]
        new_buy1: int = buy1
        if neg_pi > new_buy1:
            new_buy1 = neg_pi
        new_sell1: int = sell1
        candidate1: int = buy1 + prices[i]
        if candidate1 > new_sell1:
            new_sell1 = candidate1
        new_buy2: int = buy2
        candidate2: int = sell1 - prices[i]
        if candidate2 > new_buy2:
            new_buy2 = candidate2
        new_sell2: int = sell2
        candidate3: int = buy2 + prices[i]
        if candidate3 > new_sell2:
            new_sell2 = candidate3
        buy1 = new_buy1
        sell1 = new_sell1
        buy2 = new_buy2
        sell2 = new_sell2
        i = i + 1
    return sell2


def max_profit_cooldown(prices: list[int]) -> int:
    """Max profit with cooldown (must wait 1 day after selling)."""
    n: int = len(prices)
    if n < 2:
        return 0
    first_p: int = prices[0]
    held: int = 0 - first_p
    sold: int = 0
    rest: int = 0
    i: int = 1
    while i < n:
        new_held: int = held
        rest_minus_pi: int = rest - prices[i]
        if rest_minus_pi > new_held:
            new_held = rest_minus_pi
        new_sold: int = held + prices[i]
        new_rest: int = rest
        if sold > new_rest:
            new_rest = sold
        held = new_held
        sold = new_sold
        rest = new_rest
        i = i + 1
    if sold > rest:
        return sold
    return rest


def test_module() -> int:
    passed: int = 0

    p1: list[int] = [7, 1, 5, 3, 6, 4]
    if max_profit_one(p1) == 5:
        passed = passed + 1

    if max_profit_unlimited(p1) == 7:
        passed = passed + 1

    p2: list[int] = [3, 3, 5, 0, 0, 3, 1, 4]
    if max_profit_two(p2) == 6:
        passed = passed + 1

    p3: list[int] = [1, 2, 3, 0, 2]
    if max_profit_cooldown(p3) == 3:
        passed = passed + 1

    p4: list[int] = [7, 6, 4, 3, 1]
    if max_profit_one(p4) == 0:
        passed = passed + 1

    p5: list[int] = [1, 2, 4, 2, 5, 7, 2, 4, 9, 0]
    if max_profit_two(p5) == 13:
        passed = passed + 1

    empty: list[int] = []
    if max_profit_one(empty) == 0:
        passed = passed + 1

    return passed
