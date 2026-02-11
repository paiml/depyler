# Type inference test: Fibonacci with untyped params
# Strategy: Return types annotated, parameter types MISSING on some functions
# Tests transpiler's ability to infer int params from arithmetic usage


def fib_iterative(n) -> int:
    """Fibonacci via iteration - param type inferred from comparison."""
    if n <= 0:
        return 0
    if n == 1:
        return 1
    prev: int = 0
    curr: int = 1
    i: int = 2
    while i <= n:
        temp: int = curr
        curr = prev + curr
        prev = temp
        i = i + 1
    return curr


def fib_sum(limit) -> int:
    """Sum of fibonacci numbers up to limit."""
    total: int = 0
    prev: int = 0
    curr: int = 1
    while curr <= limit:
        total = total + curr
        temp: int = curr
        curr = prev + curr
        prev = temp
    return total


def fib_count_even(n) -> int:
    """Count even fibonacci numbers in first n terms."""
    count: int = 0
    prev: int = 0
    curr: int = 1
    idx: int = 1
    while idx <= n:
        if curr % 2 == 0:
            count = count + 1
        temp: int = curr
        curr = prev + curr
        prev = temp
        idx = idx + 1
    return count


def fib_last_digit(n) -> int:
    """Get last digit of n-th fibonacci number."""
    if n <= 0:
        return 0
    if n == 1:
        return 1
    prev: int = 0
    curr: int = 1
    i: int = 2
    while i <= n:
        temp: int = curr
        curr = (prev + curr) % 10
        prev = temp
        i = i + 1
    return curr


def fib_is_fib(n) -> int:
    """Check if n is a fibonacci number. Returns 1 if yes, 0 if no."""
    if n < 0:
        return 0
    if n == 0 or n == 1:
        return 1
    prev: int = 0
    curr: int = 1
    while curr < n:
        temp: int = curr
        curr = prev + curr
        prev = temp
    if curr == n:
        return 1
    return 0


def fib_tribonacci(n) -> int:
    """Tribonacci: each term is sum of previous 3."""
    if n <= 0:
        return 0
    if n == 1 or n == 2:
        return 1
    a: int = 0
    b: int = 1
    c: int = 1
    i: int = 3
    while i <= n:
        temp: int = a + b + c
        a = b
        b = c
        c = temp
        i = i + 1
    return c


def test_module() -> int:
    """Test all fibonacci inference functions."""
    total: int = 0

    # fib_iterative tests
    if fib_iterative(0) == 0:
        total = total + 1
    if fib_iterative(1) == 1:
        total = total + 1
    if fib_iterative(10) == 55:
        total = total + 1
    if fib_iterative(20) == 6765:
        total = total + 1

    # fib_sum tests
    if fib_sum(10) == 20:
        total = total + 1
    if fib_sum(0) == 0:
        total = total + 1

    # fib_count_even tests
    if fib_count_even(10) == 3:
        total = total + 1
    if fib_count_even(0) == 0:
        total = total + 1

    # fib_last_digit tests
    if fib_last_digit(10) == 5:
        total = total + 1
    if fib_last_digit(0) == 0:
        total = total + 1

    # fib_is_fib tests
    if fib_is_fib(8) == 1:
        total = total + 1
    if fib_is_fib(9) == 0:
        total = total + 1
    if fib_is_fib(0) == 1:
        total = total + 1

    # fib_tribonacci tests
    if fib_tribonacci(4) == 4:
        total = total + 1
    if fib_tribonacci(0) == 0:
        total = total + 1

    return total
