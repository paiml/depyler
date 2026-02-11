def max_profit_single(prices: list[int]) -> int:
    n: int = len(prices)
    if n < 2:
        return 0
    min_price: int = prices[0]
    best: int = 0
    i: int = 1
    while i < n:
        p: int = prices[i]
        diff: int = p - min_price
        if diff > best:
            best = diff
        if p < min_price:
            min_price = p
        i = i + 1
    return best

def max_profit_multiple(prices: list[int]) -> int:
    n: int = len(prices)
    if n < 2:
        return 0
    total: int = 0
    i: int = 1
    while i < n:
        prev: int = prices[i - 1]
        curr: int = prices[i]
        if curr > prev:
            total = total + curr - prev
        i = i + 1
    return total

def max_profit_with_fee(prices: list[int], fee: int) -> int:
    n: int = len(prices)
    if n < 2:
        return 0
    cash: int = 0
    hold: int = 0 - prices[0]
    i: int = 1
    while i < n:
        p: int = prices[i]
        old_cash: int = cash
        new_cash: int = hold + p - fee
        if new_cash > cash:
            cash = new_cash
        new_hold: int = old_cash - p
        if new_hold > hold:
            hold = new_hold
        i = i + 1
    return cash

def test_module() -> int:
    passed: int = 0
    r1: int = max_profit_single([7, 1, 5, 3, 6, 4])
    if r1 == 5:
        passed = passed + 1
    r2: int = max_profit_single([7, 6, 4, 3, 1])
    if r2 == 0:
        passed = passed + 1
    r3: int = max_profit_multiple([7, 1, 5, 3, 6, 4])
    if r3 == 7:
        passed = passed + 1
    r4: int = max_profit_multiple([1, 2, 3, 4, 5])
    if r4 == 4:
        passed = passed + 1
    r5: int = max_profit_with_fee([1, 3, 2, 8, 4, 9], 2)
    if r5 == 8:
        passed = passed + 1
    r6: int = max_profit_single([2, 4, 1])
    if r6 == 2:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
