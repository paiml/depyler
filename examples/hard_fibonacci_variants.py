"""Fibonacci sequence variants.

Tests: iterative fib, fib sum, fib last digit, fib modular.
"""


def fibonacci_iterative(n: int) -> int:
    """Compute nth Fibonacci number iteratively."""
    if n <= 0:
        return 0
    if n == 1:
        return 1
    a: int = 0
    b: int = 1
    i: int = 2
    while i <= n:
        temp: int = a + b
        a = b
        b = temp
        i = i + 1
    return b


def fibonacci_sum(n: int) -> int:
    """Sum of first n Fibonacci numbers."""
    if n <= 0:
        return 0
    total: int = 0
    a: int = 0
    b: int = 1
    i: int = 0
    while i < n:
        total = total + a
        temp: int = a + b
        a = b
        b = temp
        i = i + 1
    return total


def fibonacci_last_digit(n: int) -> int:
    """Last digit of nth Fibonacci number."""
    if n <= 0:
        return 0
    if n == 1:
        return 1
    a: int = 0
    b: int = 1
    i: int = 2
    while i <= n:
        temp: int = (a + b) % 10
        a = b
        b = temp
        i = i + 1
    return b


def fibonacci_mod(n: int, m: int) -> int:
    """Nth Fibonacci number modulo m."""
    if n <= 0:
        return 0
    if n == 1:
        return 1 % m
    a: int = 0
    b: int = 1 % m
    i: int = 2
    while i <= n:
        temp: int = (a + b) % m
        a = b
        b = temp
        i = i + 1
    return b


def test_module() -> int:
    """Test Fibonacci variants."""
    ok: int = 0
    if fibonacci_iterative(0) == 0:
        ok = ok + 1
    if fibonacci_iterative(1) == 1:
        ok = ok + 1
    if fibonacci_iterative(10) == 55:
        ok = ok + 1
    if fibonacci_iterative(20) == 6765:
        ok = ok + 1
    if fibonacci_sum(6) == 12:
        ok = ok + 1
    if fibonacci_last_digit(10) == 5:
        ok = ok + 1
    if fibonacci_last_digit(20) == 5:
        ok = ok + 1
    if fibonacci_mod(10, 7) == 6:
        ok = ok + 1
    return ok
