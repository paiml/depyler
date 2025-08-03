"""Basic class example for v1.2.0"""
from dataclasses import dataclass

@dataclass
class Counter:
    """A simple counter"""
    value: int = 0
    
    def increment(self):
        """Increment counter"""
        self.value = self.value + 1
    
    def get_value(self) -> int:
        """Get current value"""
        return self.value
    
    @staticmethod
    def create_with_value(val: int):
        """Create counter with initial value"""
        return Counter(val)

# Test function
def test_counter():
    # Create counter
    c = Counter()
    
    # Use instance method
    c.increment()
    c.increment()
    
    # Get value
    val = c.get_value()
    
    # Use static method
    c2 = Counter.create_with_value(10)
    
    return val, c2.value