"""Test simple method call"""

class Counter:
    def __init__(self, n: int):
        self.count = n
    
    def increment(self):
        self.count += 1

def test_counter():
    c = Counter(0)
    c.increment()
    return c.count