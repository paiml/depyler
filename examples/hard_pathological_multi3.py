# Pathological multi-function: Helper functions called from multiple callers
# Tests: shared utility functions, DRY patterns, cross-function dependencies


def is_prime(n: int) -> bool:
    """Check if n is prime."""
    if n < 2:
        return False
    if n == 2:
        return True
    if n % 2 == 0:
        return False
    d: int = 3
    while d * d <= n:
        if n % d == 0:
            return False
        d = d + 2
    return True


def gcd(a: int, b: int) -> int:
    """Greatest common divisor."""
    x: int = a
    if x < 0:
        x = 0 - x
    y: int = b
    if y < 0:
        y = 0 - y
    while y != 0:
        temp: int = y
        y = x % y
        x = temp
    return x


def lcm(a: int, b: int) -> int:
    """Least common multiple using gcd."""
    g: int = gcd(a, b)
    if g == 0:
        return 0
    return (a // g) * b


def count_primes_in_range(start: int, end: int) -> int:
    """Count primes in [start, end) using is_prime helper."""
    count: int = 0
    i: int = start
    while i < end:
        if is_prime(i) == True:
            count = count + 1
        i = i + 1
    return count


def sum_of_prime_factors(n: int) -> int:
    """Sum all unique prime factors of n."""
    total: int = 0
    d: int = 2
    remaining: int = n
    while d * d <= remaining:
        if remaining % d == 0:
            if is_prime(d) == True:
                total = total + d
            while remaining % d == 0:
                remaining = remaining // d
        d = d + 1
    if remaining > 1:
        total = total + remaining
    return total


def are_coprime(a: int, b: int) -> bool:
    """Check if a and b are coprime (gcd == 1)."""
    # Workaround: avoid returning func_call() == literal directly.
    # Transpiler generates double ?? on the comparison.
    g: int = gcd(a, b)
    return g == 1


def simplify_fraction(numer: int, denom: int) -> list[int]:
    """Simplify fraction, return [numerator, denominator]."""
    if denom == 0:
        return [0, 0]
    g: int = gcd(numer, denom)
    result: list[int] = []
    result.append(numer // g)
    result.append(denom // g)
    return result


def lcm_of_list(nums: list[int]) -> int:
    """Compute LCM of a list of numbers."""
    if len(nums) == 0:
        return 0
    result: int = nums[0]
    i: int = 1
    while i < len(nums):
        result = lcm(result, nums[i])
        i = i + 1
    return result


def euler_totient(n: int) -> int:
    """Count integers 1..n that are coprime with n."""
    count: int = 0
    i: int = 1
    while i <= n:
        if are_coprime(i, n) == True:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    passed: int = 0
    # Test 1: is_prime
    if is_prime(17) == True:
        passed = passed + 1
    # Test 2: gcd
    if gcd(12, 8) == 4:
        passed = passed + 1
    # Test 3: lcm
    if lcm(4, 6) == 12:
        passed = passed + 1
    # Test 4: count primes in [1,20) = 8 (2,3,5,7,11,13,17,19)
    if count_primes_in_range(1, 20) == 8:
        passed = passed + 1
    # Test 5: prime factors of 12 = 2+3 = 5
    if sum_of_prime_factors(12) == 5:
        passed = passed + 1
    # Test 6: simplify 6/8 = 3/4
    frac: list[int] = simplify_fraction(6, 8)
    if frac[0] == 3 and frac[1] == 4:
        passed = passed + 1
    # Test 7: euler totient of 12 = 4 (1,5,7,11)
    if euler_totient(12) == 4:
        passed = passed + 1
    return passed
