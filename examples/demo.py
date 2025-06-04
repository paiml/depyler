from typing import List

def fibonacci(n: int) -> int:
    """Calculate fibonacci number recursively"""
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

def factorial(n: int) -> int:
    """Calculate factorial iteratively"""
    result = 1
    for i in range(1, n + 1):
        result = result * i
    return result

def is_prime(n: int) -> bool:
    """Check if a number is prime"""
    if n < 2:
        return False
    for i in range(2, n):
        if n % i == 0:
            return False
    return True

def process_list(numbers: List[int]) -> int:
    """Sum all numbers in a list"""
    total = 0
    for num in numbers:
        total = total + num
    return total