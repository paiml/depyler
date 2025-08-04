# @depyler: optimization_level = "aggressive"
# @depyler: bounds_checking = "none"
from typing import Dict

def fibonacci_recursive(n: int) -> int:
    """Classic recursive fibonacci - demonstrates recursion"""
    if n <= 1:
        return n
    return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)

def fibonacci_memoized(n: int, memo: Dict[int, int] = None) -> int:
    """Memoized fibonacci - demonstrates optimization patterns"""
    if memo is None:
        memo = {}
    
    if n in memo:
        return memo[n]
    
    if n <= 1:
        result = n
    else:
        result = fibonacci_memoized(n - 1, memo) + fibonacci_memoized(n - 2, memo)
    
    memo[n] = result
    return result

def fibonacci_iterative(n: int) -> int:
    """Iterative fibonacci - demonstrates loops and efficiency"""
    if n <= 1:
        return n
    
    a, b = 0, 1
    for _ in range(2, n + 1):
        a, b = b, a + b
    
    return b