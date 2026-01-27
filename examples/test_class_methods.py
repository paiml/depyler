"""Test class with various method types for v1.2.0"""

class Calculator:
    """A simple calculator class demonstrating method types"""
    
    def __init__(self, initial_value: int = 0):
        """Initialize calculator with a value"""
        self.value = initial_value
        self.history: list[str] = []
    
    def add(self, x: int) -> int:
        """Add to current value (instance method)"""
        self.value += x
        self.history.append(f"add({x})")
        return self.value
    
    def multiply(self, x: int) -> int:
        """Multiply current value (instance method)"""
        self.value *= x
        self.history.append(f"multiply({x})")
        return self.value
    
    @staticmethod
    def square(x: int) -> int:
        """Square a number (static method)"""
        return x * x
    
    @classmethod
    def from_string(cls, s: str):
        """Create calculator from string (class method)"""
        return cls(int(s))
    
    @property
    def current(self) -> int:
        """Get current value (property)"""
        return self.value
    
    def get_history(self) -> list[str]:
        """Get operation history"""
        return self.history.copy()

# Test usage
def test_calculator():
    # Create instance
    calc = Calculator(10)
    
    # Instance methods
    result1 = calc.add(5)
    result2 = calc.multiply(2)
    
    # Static method
    squared = Calculator.square(4)
    
    # Property access
    current = calc.current
    
    # Get history
    history = calc.get_history()
    
    return result1, result2, squared, current, history