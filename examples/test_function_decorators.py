"""Test function decorators for v1.3.0"""

def timing_decorator(func):
    """A simple timing decorator"""
    def wrapper(*args, **kwargs):
        # In real code, would time the function
        result = func(*args, **kwargs)
        return result
    return wrapper

def logging_decorator(func):
    """A simple logging decorator"""
    def wrapper(*args, **kwargs):
        # In real code, would log the call
        result = func(*args, **kwargs) 
        return result
    return wrapper

@timing_decorator
def slow_function(n: int) -> int:
    """A function that would be slow"""
    total = 0
    for i in range(n):
        total += i
    return total

@logging_decorator
@timing_decorator
def important_calculation(x: int, y: int) -> int:
    """A function with stacked decorators"""
    return x * y + x + y

# Parameterized decorator
def repeat(times: int):
    """Decorator that repeats function call"""
    def decorator(func):
        def wrapper(*args, **kwargs):
            result = None
            for _ in range(times):
                result = func(*args, **kwargs)
            return result
        return wrapper
    return decorator

@repeat(3)
def greet(name: str) -> str:
    """Function that will be called 3 times"""
    return f"Hello, {name}!"

def test_decorators():
    """Test decorated functions"""
    result1 = slow_function(100)
    result2 = important_calculation(5, 10)
    result3 = greet("World")
    
    return result1 + result2