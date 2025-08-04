# @depyler: optimization_level = "aggressive"
from typing import List

def factorial(n: int) -> int:
    """Calculate factorial using iteration"""
    if n <= 1:
        return 1
    
    result = 1
    for i in range(2, n + 1):
        result = result * i
    
    return result

def gcd(a: int, b: int) -> int:
    """Greatest common divisor using Euclidean algorithm"""
    while b != 0:
        temp = b
        b = a % b
        a = temp
    
    return a

def is_prime(n: int) -> bool:
    """Check if number is prime"""
    if n < 2:
        return False
    
    if n == 2:
        return True
    
    if n % 2 == 0:
        return False
    
    i = 3
    while i * i <= n:
        if n % i == 0:
            return False
        i = i + 2
    
    return True

def sum_of_squares(numbers: List[int]) -> int:
    """Calculate sum of squares"""
    total = 0
    for num in numbers:
        total = total + num * num
    
    return total

def power(base: int, exponent: int) -> int:
    """Calculate power using exponentiation by squaring"""
    if exponent == 0:
        return 1
    
    if exponent < 0:
        return 0  # Simplified for integer division
    
    result = 1
    while exponent > 0:
        if exponent % 2 == 1:
            result = result * base
        base = base * base
        exponent = exponent // 2
    
    return result