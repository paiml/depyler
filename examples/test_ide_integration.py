"""
Example demonstrating IDE integration features.
Pure-function approach: no classes, no decorators, full type annotations.
"""


def calculate_fibonacci(n: int) -> int:
    """Calculate the nth Fibonacci number iteratively."""
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


def is_prime(n: int) -> int:
    """Check if a number is prime. Returns 1 for prime, 0 for not."""
    if n < 2:
        return 0
    if n == 2:
        return 1
    if n % 2 == 0:
        return 0
    divisor: int = 3
    while divisor * divisor <= n:
        if n % divisor == 0:
            return 0
        divisor = divisor + 2
    return 1


def count_primes_in_list(items: list[int]) -> int:
    """Count how many primes are in the list."""
    count: int = 0
    for item in items:
        check: int = is_prime(item)
        if check == 1:
            count = count + 1
    return count


def sum_list(items: list[int]) -> int:
    """Sum all elements in a list."""
    total: int = 0
    for item in items:
        total = total + item
    return total


def process_data(items: list[int]) -> list[int]:
    """Process a list of integers and return statistics as a list.

    Returns [count, total_sum, prime_count].
    """
    count: int = len(items)
    total: int = sum_list(items)
    primes: int = count_primes_in_list(items)
    stats: list[int] = [count, total, primes]
    return stats


def round_to_int(val: int, precision: int) -> int:
    """Simulate rounding by returning truncated integer value.

    For integer inputs precision is unused, returns val itself.
    """
    return val


def test_fibonacci() -> int:
    """Test fibonacci calculations."""
    passed: int = 0
    r0: int = calculate_fibonacci(0)
    if r0 == 0:
        passed = passed + 1
    r1: int = calculate_fibonacci(1)
    if r1 == 1:
        passed = passed + 1
    r10: int = calculate_fibonacci(10)
    if r10 == 55:
        passed = passed + 1
    return passed


def test_prime_check() -> int:
    """Test prime checking."""
    passed: int = 0
    p2: int = is_prime(2)
    if p2 == 1:
        passed = passed + 1
    p4: int = is_prime(4)
    if p4 == 0:
        passed = passed + 1
    p17: int = is_prime(17)
    if p17 == 1:
        passed = passed + 1
    p1: int = is_prime(1)
    if p1 == 0:
        passed = passed + 1
    return passed


def test_process_data() -> int:
    """Test data processing."""
    passed: int = 0
    items: list[int] = [2, 3, 4, 5, 6, 7, 8, 9, 10]
    stats: list[int] = process_data(items)
    if stats[0] == 9:
        passed = passed + 1
    if stats[1] == 54:
        passed = passed + 1
    if stats[2] == 4:
        passed = passed + 1
    return passed


def test_sum_list() -> int:
    """Test list summation."""
    passed: int = 0
    items: list[int] = [1, 2, 3, 4, 5]
    total: int = sum_list(items)
    if total == 15:
        passed = passed + 1
    empty: list[int] = []
    total2: int = sum_list(empty)
    if total2 == 0:
        passed = passed + 1
    return passed


def test_module() -> int:
    """Run all IDE integration tests. Returns count of passed tests."""
    passed: int = 0

    r1: int = test_fibonacci()
    passed = passed + r1

    r2: int = test_prime_check()
    passed = passed + r2

    r3: int = test_process_data()
    passed = passed + r3

    r4: int = test_sum_list()
    passed = passed + r4

    return passed
