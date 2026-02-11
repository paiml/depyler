"""Hard numeric algorithm patterns for transpiler stress-testing.

Tests: GCD/LCM, prime sieve, fibonacci with matrix-like approach,
modular exponentiation, integer square root, digit manipulation,
and combinatorial math -- all pure integer arithmetic.
"""


def gcd(a: int, b: int) -> int:
    """Compute greatest common divisor using Euclidean algorithm."""
    if a < 0:
        a = -a
    if b < 0:
        b = -b
    while b != 0:
        temp: int = b
        b = a % b
        a = temp
    return a


def lcm(a: int, b: int) -> int:
    """Compute least common multiple using GCD."""
    if a == 0 or b == 0:
        return 0
    abs_a: int = a
    abs_b: int = b
    if abs_a < 0:
        abs_a = -abs_a
    if abs_b < 0:
        abs_b = -abs_b
    return (abs_a // gcd(abs_a, abs_b)) * abs_b


def extended_gcd(a: int, b: int) -> list[int]:
    """Extended Euclidean algorithm returning [gcd, x, y] where ax + by = gcd."""
    if a == 0:
        return [b, 0, 1]
    remainder: int = b % a
    rec: list[int] = extended_gcd(remainder, a)
    g: int = rec[0]
    x1: int = rec[1]
    y1: int = rec[2]
    x: int = y1 - (b // a) * x1
    y: int = x1
    return [g, x, y]


def sieve_of_eratosthenes(limit: int) -> list[int]:
    """Return all prime numbers up to limit using the Sieve of Eratosthenes."""
    if limit < 2:
        return []
    is_prime: list[bool] = []
    for _ in range(limit + 1):
        is_prime.append(True)
    is_prime[0] = False
    is_prime[1] = False
    i: int = 2
    while i * i <= limit:
        if is_prime[i]:
            j: int = i * i
            while j <= limit:
                is_prime[j] = False
                j += i
        i += 1
    primes: list[int] = []
    for n in range(2, limit + 1):
        if is_prime[n]:
            primes.append(n)
    return primes


def is_prime(n: int) -> bool:
    """Check if a number is prime using trial division."""
    if n < 2:
        return False
    if n < 4:
        return True
    if n % 2 == 0 or n % 3 == 0:
        return False
    i: int = 5
    while i * i <= n:
        if n % i == 0 or n % (i + 2) == 0:
            return False
        i += 6
    return True


def fibonacci_iterative(n: int) -> int:
    """Compute the nth Fibonacci number iteratively.

    fib(0) = 0, fib(1) = 1, fib(n) = fib(n-1) + fib(n-2).
    """
    if n <= 0:
        return 0
    if n == 1:
        return 1
    prev: int = 0
    curr: int = 1
    for _ in range(2, n + 1):
        next_val: int = prev + curr
        prev = curr
        curr = next_val
    return curr


def fibonacci_matrix(n: int) -> int:
    """Compute nth Fibonacci using matrix exponentiation approach.

    Uses the identity: [[F(n+1), F(n)], [F(n), F(n-1)]] = [[1,1],[1,0]]^n.
    Implemented with fast doubling derived from the matrix form.
    """
    if n <= 0:
        return 0
    if n == 1:
        return 1
    # Fast doubling: F(2k) = F(k)[2*F(k+1) - F(k)]
    #                F(2k+1) = F(k)^2 + F(k+1)^2
    a: int = 0
    b: int = 1
    bit: int = highest_bit(n)
    while bit > 0:
        # Doubling step
        c: int = a * (2 * b - a)
        d: int = a * a + b * b
        if n & bit:
            a = d
            b = c + d
        else:
            a = c
            b = d
        bit = bit >> 1
    return a


def highest_bit(n: int) -> int:
    """Find the highest set bit position as a power of 2."""
    if n <= 0:
        return 0
    bit: int = 1
    while bit <= n:
        bit = bit << 1
    return bit >> 1


def mod_pow(base: int, exp: int, modulus: int) -> int:
    """Compute (base ^ exp) % modulus using fast modular exponentiation."""
    if modulus == 1:
        return 0
    result: int = 1
    base = base % modulus
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % modulus
        exp = exp >> 1
        base = (base * base) % modulus
    return result


def integer_sqrt(n: int) -> int:
    """Compute the integer square root using Newton's method."""
    if n < 0:
        return 0
    if n == 0:
        return 0
    x: int = n
    y: int = (x + 1) // 2
    while y < x:
        x = y
        y = (x + n // x) // 2
    return x


def sum_of_divisors(n: int) -> int:
    """Compute the sum of all divisors of n."""
    if n <= 0:
        return 0
    total: int = 0
    i: int = 1
    while i * i <= n:
        if n % i == 0:
            total += i
            if i != n // i:
                total += n // i
        i += 1
    return total


def count_digits(n: int) -> int:
    """Count the number of digits in an integer."""
    if n < 0:
        n = -n
    if n == 0:
        return 1
    count: int = 0
    while n > 0:
        count += 1
        n = n // 10
    return count


def digit_sum(n: int) -> int:
    """Compute the sum of digits of a non-negative integer."""
    if n < 0:
        n = -n
    total: int = 0
    while n > 0:
        total += n % 10
        n = n // 10
    return total


def reverse_number(n: int) -> int:
    """Reverse the digits of a non-negative integer."""
    if n < 0:
        return -reverse_number(-n)
    result: int = 0
    while n > 0:
        result = result * 10 + n % 10
        n = n // 10
    return result


def is_palindrome_number(n: int) -> bool:
    """Check if a non-negative integer is a palindrome."""
    if n < 0:
        return False
    return n == reverse_number(n)


def n_choose_k(n: int, k: int) -> int:
    """Compute binomial coefficient C(n, k) using iterative method."""
    if k < 0 or k > n:
        return 0
    if k == 0 or k == n:
        return 1
    # Use symmetry to minimize iterations
    if k > n - k:
        k = n - k
    result: int = 1
    for i in range(k):
        result = result * (n - i) // (i + 1)
    return result


def test_all() -> bool:
    """Comprehensive test exercising all numeric algorithm functions."""
    # Test gcd
    assert gcd(12, 8) == 4
    assert gcd(17, 13) == 1
    assert gcd(0, 5) == 5
    assert gcd(100, 75) == 25
    assert gcd(-12, 8) == 4

    # Test lcm
    assert lcm(4, 6) == 12
    assert lcm(3, 7) == 21
    assert lcm(0, 5) == 0
    assert lcm(12, 18) == 36

    # Test extended_gcd
    eg: list[int] = extended_gcd(35, 15)
    assert eg[0] == 5
    # Verify: 35*x + 15*y == 5
    assert 35 * eg[1] + 15 * eg[2] == 5

    # Test sieve_of_eratosthenes
    primes: list[int] = sieve_of_eratosthenes(30)
    assert primes == [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
    assert sieve_of_eratosthenes(1) == []
    assert sieve_of_eratosthenes(2) == [2]

    # Test is_prime
    assert is_prime(2) == True
    assert is_prime(3) == True
    assert is_prime(4) == False
    assert is_prime(17) == True
    assert is_prime(1) == False
    assert is_prime(97) == True
    assert is_prime(100) == False

    # Test fibonacci_iterative
    assert fibonacci_iterative(0) == 0
    assert fibonacci_iterative(1) == 1
    assert fibonacci_iterative(10) == 55
    assert fibonacci_iterative(20) == 6765

    # Test fibonacci_matrix (fast doubling)
    assert fibonacci_matrix(0) == 0
    assert fibonacci_matrix(1) == 1
    assert fibonacci_matrix(10) == 55
    assert fibonacci_matrix(20) == 6765
    assert fibonacci_matrix(30) == 832040

    # Test mod_pow
    assert mod_pow(2, 10, 1000) == 24
    assert mod_pow(3, 5, 13) == 9
    assert mod_pow(2, 0, 7) == 1
    assert mod_pow(5, 3, 1) == 0

    # Test integer_sqrt
    assert integer_sqrt(0) == 0
    assert integer_sqrt(1) == 1
    assert integer_sqrt(4) == 2
    assert integer_sqrt(9) == 3
    assert integer_sqrt(10) == 3
    assert integer_sqrt(100) == 10
    assert integer_sqrt(144) == 12

    # Test sum_of_divisors
    assert sum_of_divisors(1) == 1
    assert sum_of_divisors(6) == 12  # 1+2+3+6
    assert sum_of_divisors(28) == 56  # perfect number: 1+2+4+7+14+28
    assert sum_of_divisors(12) == 28  # 1+2+3+4+6+12

    # Test count_digits
    assert count_digits(0) == 1
    assert count_digits(5) == 1
    assert count_digits(99) == 2
    assert count_digits(1000) == 4
    assert count_digits(-42) == 2

    # Test digit_sum
    assert digit_sum(0) == 0
    assert digit_sum(123) == 6
    assert digit_sum(9999) == 36

    # Test reverse_number
    assert reverse_number(123) == 321
    assert reverse_number(1000) == 1
    assert reverse_number(0) == 0
    assert reverse_number(-123) == -321

    # Test is_palindrome_number
    assert is_palindrome_number(121) == True
    assert is_palindrome_number(12321) == True
    assert is_palindrome_number(123) == False
    assert is_palindrome_number(0) == True

    # Test n_choose_k
    assert n_choose_k(5, 2) == 10
    assert n_choose_k(10, 3) == 120
    assert n_choose_k(0, 0) == 1
    assert n_choose_k(5, 0) == 1
    assert n_choose_k(5, 5) == 1
    assert n_choose_k(5, 6) == 0

    # 2^10 mod 1000 = 1024 mod 1000 = 24, verify
    assert mod_pow(2, 10, 1000) == 24

    return True


def main() -> None:
    """Run all tests and report results."""
    result: bool = test_all()
    if result:
        print("All numeric algorithm tests passed!")


if __name__ == "__main__":
    main()
