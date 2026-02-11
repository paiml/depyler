"""Auction and pricing algorithms.

Tests: dutch auction price, first-price sealed, second-price, reserve price.
"""


def dutch_auction_price(start_price: int, decrement: int, bids: list[int]) -> int:
    """Simulate Dutch auction: price drops until a bid matches or exceeds it."""
    price: int = start_price
    while price > 0:
        i: int = 0
        while i < len(bids):
            if bids[i] >= price:
                return price
            i = i + 1
        price = price - decrement
    return 0


def first_price_winner(bids: list[int]) -> int:
    """First-price sealed auction: highest bidder pays their bid."""
    if len(bids) == 0:
        return 0
    best: int = bids[0]
    i: int = 1
    while i < len(bids):
        if bids[i] > best:
            best = bids[i]
        i = i + 1
    return best


def second_price_value(bids: list[int]) -> int:
    """Second-price (Vickrey) auction: winner pays second-highest bid."""
    n: int = len(bids)
    if n < 2:
        return 0
    first: int = bids[0]
    second: int = 0
    if bids[1] > first:
        second = first
        first = bids[1]
    else:
        second = bids[1]
    i: int = 2
    while i < n:
        if bids[i] > first:
            second = first
            first = bids[i]
        elif bids[i] > second:
            second = bids[i]
        i = i + 1
    return second


def revenue_with_reserve(bids: list[int], reserve: int) -> int:
    """Calculate revenue with a reserve price (first-price above reserve)."""
    best: int = 0
    i: int = 0
    while i < len(bids):
        if bids[i] >= reserve and bids[i] > best:
            best = bids[i]
        i = i + 1
    return best


def test_module() -> int:
    """Test auction algorithms."""
    ok: int = 0
    if dutch_auction_price(100, 10, [30, 50, 70]) == 70:
        ok = ok + 1
    if dutch_auction_price(100, 10, [10, 20]) == 20:
        ok = ok + 1
    if first_price_winner([50, 80, 30, 90]) == 90:
        ok = ok + 1
    if first_price_winner([10]) == 10:
        ok = ok + 1
    if second_price_value([50, 80, 30, 90]) == 80:
        ok = ok + 1
    if second_price_value([10, 20]) == 10:
        ok = ok + 1
    if revenue_with_reserve([50, 30, 70, 20], 40) == 70:
        ok = ok + 1
    if revenue_with_reserve([10, 20], 50) == 0:
        ok = ok + 1
    return ok
