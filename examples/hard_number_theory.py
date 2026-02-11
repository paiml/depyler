# Number theory (primes, factorials, permutations)
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def factorial(n: int) -> int:
    """Compute n factorial iteratively."""
    if n <= 1:
        return 1
    result: int = 1
    i: int = 2
    while i <= n:
        result = result * i
        i = i + 1
    return result


def is_prime(n: int) -> bool:
    """Check if n is prime."""
    if n < 2:
        return False
    if n == 2:
        return True
    if n % 2 == 0:
        return False
    i: int = 3
    while i * i <= n:
        if n % i == 0:
            return False
        i = i + 2
    return True


def count_primes_below(limit: int) -> int:
    """Count primes below the given limit."""
    count: int = 0
    n: int = 2
    while n < limit:
        if is_prime(n):
            count = count + 1
        n = n + 1
    return count


def gcd(a: int, b: int) -> int:
    """Compute greatest common divisor using Euclidean algorithm."""
    while b != 0:
        temp: int = b
        b = a % b
        a = temp
    return a


def lcm(a: int, b: int) -> int:
    """Compute least common multiple."""
    if a == 0 or b == 0:
        return 0
    return (a * b) // gcd(a, b)


def test_module() -> int:
    assert factorial(0) == 1
    assert factorial(1) == 1
    assert factorial(5) == 120
    assert factorial(10) == 3628800
    assert is_prime(2) == True
    assert is_prime(17) == True
    assert is_prime(4) == False
    assert is_prime(1) == False
    assert count_primes_below(10) == 4
    assert count_primes_below(20) == 8
    assert gcd(12, 8) == 4
    assert gcd(17, 13) == 1
    assert gcd(100, 75) == 25
    assert lcm(4, 6) == 12
    assert lcm(3, 5) == 15
    assert lcm(0, 5) == 0
    return 0


if __name__ == "__main__":
    test_module()
