"""Test augmented assignment in classes"""

class Counter:
    def __init__(self, initial: int = 0):
        self.value = initial
    
    def increment(self, amount: int):
        """Test augmented assignment"""
        self.value += amount
        return self.value

def test_counter():
    c = Counter(10)
    result = c.increment(5)
    return result