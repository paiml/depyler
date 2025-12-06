# DEPYLER-0720: Integer literals in float comparisons
# Python pattern: self.amount > 0 where amount is float
# Problem: Rust expects 0.0 for f64 comparison, not 0
# Expected: Integer literals should become float when compared with floats

class Account:
    def __init__(self, balance: float) -> None:
        self.balance: float = balance

    def is_positive(self) -> bool:
        """Check if balance is positive."""
        return self.balance > 0
