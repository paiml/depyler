"""Currency converter with integer cent-based arithmetic.

Tests: convert, round-trip, markup, discount, total.
"""


class CurrencyConverter:
    """Converts between currencies using integer cent rates."""

    def __init__(self, rate_num: int, rate_den: int) -> None:
        self.rate_num: int = rate_num
        self.rate_den: int = rate_den

    def convert(self, amount: int) -> int:
        return amount * self.rate_num // self.rate_den

    def reverse(self, amount: int) -> int:
        return amount * self.rate_den // self.rate_num

    def get_rate_num(self) -> int:
        return self.rate_num

    def get_rate_den(self) -> int:
        return self.rate_den


def apply_markup(amount: int, markup_pct: int) -> int:
    """Apply markup percentage (integer arithmetic)."""
    return amount + amount * markup_pct // 100


def apply_discount(amount: int, discount_pct: int) -> int:
    """Apply discount percentage."""
    return amount - amount * discount_pct // 100


def total_with_tax(prices: list[int], tax_pct: int) -> int:
    """Sum prices and apply tax."""
    total: int = 0
    for p in prices:
        total = total + p
    return total + total * tax_pct // 100


def test_module() -> int:
    """Test currency converter."""
    ok: int = 0
    c = CurrencyConverter(120, 100)
    if c.convert(1000) == 1200:
        ok = ok + 1
    if c.reverse(1200) == 1000:
        ok = ok + 1
    if apply_markup(1000, 10) == 1100:
        ok = ok + 1
    if apply_discount(1000, 25) == 750:
        ok = ok + 1
    if total_with_tax([100, 200, 300], 10) == 660:
        ok = ok + 1
    if c.get_rate_num() == 120:
        ok = ok + 1
    return ok
