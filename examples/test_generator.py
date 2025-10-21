"""Test generator functions for v1.3.0"""

def simple_generator(n: int) -> int:
    """A simple generator that yields numbers"""
    i = 0
    while i < n:
        yield i
        i += 1

def fibonacci_generator(n: int) -> int:
    """Generate Fibonacci numbers"""
    a, b = 0, 1
    count = 0
    while count < n:
        yield a
        a, b = b, a + b
        count += 1

def test_generator() -> int:
    """Test generator usage"""
    # For now, we can't use generators directly
    # This would need yield support
    return 42