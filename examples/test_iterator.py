"""Test iterator protocol for v1.3.0"""

class Range:
    """A simple range iterator"""
    
    def __init__(self, start: int, stop: int):
        self.start = start
        self.stop = stop
        self.current = start
    
    def __iter__(self):
        """Return self as iterator"""
        return self
    
    def __next__(self) -> int:
        """Get next value"""
        if self.current < self.stop:
            value = self.current
            self.current += 1
            return value
        else:
            # In Python, would raise StopIteration
            return -1  # Simplified for now

def test_custom_iterator():
    """Test custom iterator"""
    r = Range(0, 5)
    total = 0
    
    # Manual iteration for now
    value = r.__next__()
    while value != -1:
        total += value
        value = r.__next__()
    
    return total

def test_for_with_iterator():
    """Test for loop with iterator"""
    r = Range(0, 5)
    total = 0
    
    # This should work once we have proper iterator support
    for i in r:
        total += i
    
    return total