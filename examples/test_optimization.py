def compute_constants() -> int:
    """Demonstrates constant propagation and dead code elimination"""
    x = 5
    y = 10
    z = x + y  # Should be optimized to 15
    unused = 42  # Should be eliminated
    result = z * 2  # Should be optimized to 30
    return result

def fibonacci(n: int) -> int:
    """Example that won't be fully optimized (recursive)"""
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

def simple_math() -> float:
    """More constant folding examples"""
    a = 3.14
    b = 2.0
    c = a * b  # Should be optimized to 6.28
    d = c / 2.0  # Should be optimized to 3.14
    return d