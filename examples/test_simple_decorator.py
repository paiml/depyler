"""Test simple decorator support"""

def simple_function(x: int) -> int:
    """A simple function without decorators"""
    return x * 2

def test_no_decorator():
    result = simple_function(5)
    return result