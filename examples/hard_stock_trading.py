"""Stock trading algorithms.

Tests: max profit single, max profit multiple, max profit with fee.
"""


def max_profit_single(prices: list[int]) -> int:
    """Maximum profit from one buy-sell transaction."""
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


def max_profit_multiple(prices: list[int]) -> int:
    """Maximum profit from unlimited buy-sell transactions."""
    n: int = len(prices)
    total: int = 0
    i: int = 1
    while i < n:
        if prices[i] > prices[i - 1]:
            total = total + prices[i] - prices[i - 1]
        i = i + 1
    return total


def max_profit_with_fee(prices: list[int], fee: int) -> int:
    """Maximum profit with transaction fee per trade."""
    n: int = len(prices)
    if n < 2:
        return 0
    first_price: int = prices[0]
    cash: int = 0
    hold: int = 0 - first_price
    i: int = 1
    while i < n:
        new_cash: int = cash
        candidate_sell: int = hold + prices[i] - fee
        if candidate_sell > cash:
            new_cash = candidate_sell
        new_hold: int = hold
        candidate_buy: int = cash - prices[i]
        if candidate_buy > hold:
            new_hold = candidate_buy
        cash = new_cash
        hold = new_hold
        i = i + 1
    return cash


def max_profit_two_txn(prices: list[int]) -> int:
    """Maximum profit from at most two buy-sell transactions."""
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
        neg_price: int = 0 - prices[i]
        if neg_price > buy1:
            buy1 = neg_price
        cand1: int = buy1 + prices[i]
        if cand1 > sell1:
            sell1 = cand1
        cand2: int = sell1 - prices[i]
        if cand2 > buy2:
            buy2 = cand2
        cand3: int = buy2 + prices[i]
        if cand3 > sell2:
            sell2 = cand3
        i = i + 1
    return sell2


def test_module() -> int:
    """Test stock trading algorithms."""
    ok: int = 0
    if max_profit_single([7, 1, 5, 3, 6, 4]) == 5:
        ok = ok + 1
    if max_profit_single([7, 6, 4, 3, 1]) == 0:
        ok = ok + 1
    if max_profit_multiple([7, 1, 5, 3, 6, 4]) == 7:
        ok = ok + 1
    if max_profit_multiple([1, 2, 3, 4, 5]) == 4:
        ok = ok + 1
    if max_profit_with_fee([1, 3, 2, 8, 4, 9], 2) == 8:
        ok = ok + 1
    if max_profit_two_txn([3, 3, 5, 0, 0, 3, 1, 4]) == 6:
        ok = ok + 1
    if max_profit_two_txn([1, 2, 3, 4, 5]) == 4:
        ok = ok + 1
    return ok
