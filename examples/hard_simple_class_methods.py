"""Class with various method patterns.

Tests: getters, setters, computed properties, state mutation.
"""


class Account:
    """Simple bank account."""

    def __init__(self, owner: str, balance: int) -> None:
        self.owner: str = owner
        self.balance: int = balance
        self.transactions: int = 0

    def deposit(self, amount: int) -> None:
        self.balance += amount
        self.transactions += 1

    def withdraw(self, amount: int) -> bool:
        if amount > self.balance:
            return False
        self.balance -= amount
        self.transactions += 1
        return True

    def get_balance(self) -> int:
        return self.balance

    def get_transactions(self) -> int:
        return self.transactions

    def get_owner(self) -> str:
        return self.owner


def compute_interest(principal: int, rate_pct: int, years: int) -> int:
    """Simple integer interest: principal * rate * years / 100."""
    return principal * rate_pct * years // 100


def total_balance(balances: list[int]) -> int:
    """Sum all balances."""
    total: int = 0
    for b in balances:
        total += b
    return total


def test_module() -> int:
    """Test account operations."""
    ok: int = 0

    a = Account("alice", 1000)
    a.deposit(500)
    if a.get_balance() == 1500:
        ok += 1
    if a.get_transactions() == 1:
        ok += 1

    success: bool = a.withdraw(200)
    if success:
        ok += 1
    if a.get_balance() == 1300:
        ok += 1

    fail: bool = a.withdraw(5000)
    if not fail:
        ok += 1

    interest: int = compute_interest(1000, 5, 2)
    if interest == 100:
        ok += 1

    t: int = total_balance([100, 200, 300])
    if t == 600:
        ok += 1

    return ok
