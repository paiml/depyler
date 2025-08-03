"""Depyler v1.2.0 OOP Features Showcase

This example demonstrates the new object-oriented programming features
added in v1.2.0, including classes, methods, and attributes.
"""
from dataclasses import dataclass
from typing import List

class BankAccount:
    """A simple bank account with instance methods and attributes"""
    
    def __init__(self, owner: str, initial_balance: float = 0.0):
        """Initialize account with owner name and optional balance"""
        self.owner = owner
        self.balance = initial_balance
        self.transaction_count = 0
    
    def deposit(self, amount: float) -> float:
        """Deposit money and return new balance"""
        self.balance += amount
        self.transaction_count += 1
        return self.balance
    
    def withdraw(self, amount: float) -> bool:
        """Withdraw money if sufficient funds, return success status"""
        if amount <= self.balance:
            self.balance -= amount
            self.transaction_count += 1
            return True
        return False
    
    @staticmethod
    def calculate_interest(principal: float, rate: float, years: int) -> float:
        """Calculate simple interest (static method)"""
        return principal * rate * years
    
    @property
    def summary(self) -> str:
        """Get account summary (property decorator)"""
        return f"{self.owner}: ${self.balance:.2f}"

@dataclass
class Transaction:
    """A transaction record using dataclass decorator"""
    from_account: str
    to_account: str
    amount: float
    
    def is_valid(self) -> bool:
        """Check if transaction is valid"""
        return self.amount > 0.0

class SavingsAccount(BankAccount):
    """Savings account with interest (inheritance example)"""
    
    def __init__(self, owner: str, interest_rate: float = 0.02):
        super().__init__(owner, 0.0)
        self.interest_rate = interest_rate
    
    def apply_interest(self) -> float:
        """Apply interest to current balance"""
        interest = self.balance * self.interest_rate
        self.balance += interest
        return self.balance

def demo_instance_methods():
    """Demonstrate instance methods and attribute access"""
    account = BankAccount("Alice", 1000.0)
    
    # Instance method calls
    new_balance = account.deposit(500.0)
    success = account.withdraw(200.0)
    
    # Attribute access
    owner = account.owner
    balance = account.balance
    
    return balance

def demo_static_methods():
    """Demonstrate static method calls"""
    # Static method call on class
    interest = BankAccount.calculate_interest(1000.0, 0.05, 3)
    return interest

def demo_dataclass():
    """Demonstrate dataclass functionality"""
    # Create dataclass instance
    tx = Transaction("Alice", "Bob", 100.0)
    
    # Access fields
    amount = tx.amount
    
    # Call method
    valid = tx.is_valid()
    
    return valid

def demo_all_features():
    """Comprehensive demo of all v1.2.0 OOP features"""
    # Create and use regular class
    checking = BankAccount("Charlie", 5000.0)
    checking.deposit(1000.0)
    
    # Create and use dataclass
    transaction = Transaction("Charlie", "Dana", 500.0)
    if transaction.is_valid():
        checking.withdraw(transaction.amount)
    
    # Use static method
    future_value = BankAccount.calculate_interest(
        checking.balance, 0.03, 5
    )
    
    # Access property (once properties are fully supported)
    # summary = checking.summary
    
    return checking.balance + future_value

# Entry point for testing
def main():
    """Run all demonstrations"""
    balance = demo_instance_methods()
    interest = demo_static_methods()
    valid = demo_dataclass()
    final = demo_all_features()
    
    return final