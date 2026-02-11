# Type inference test: Math computations
# Strategy: Only test_module() -> int annotated, everything else inferred


def absolute(x):
    """Absolute value - type inferred from comparison."""
    if x < 0:
        return 0 - x
    return x


def power_int(num, exp):
    """Integer power computation."""
    if exp < 0:
        return 0
    if exp == 0:
        return 1
    result = 1
    i = 0
    while i < exp:
        result = result * num
        i = i + 1
    return result


def isqrt_newton(n):
    """Integer square root using Newton's method."""
    if n < 0:
        return 0
    if n == 0:
        return 0
    x = n
    y = (x + 1) // 2
    while y < x:
        x = y
        y = (x + n // x) // 2
    return x


def is_prime(n):
    """Primality test. Returns 1 if prime, 0 if not."""
    if n < 2:
        return 0
    if n == 2:
        return 1
    if n % 2 == 0:
        return 0
    d = 3
    while d * d <= n:
        if n % d == 0:
            return 0
        d = d + 2
    return 1


def next_prime(n):
    """Find the smallest prime greater than n."""
    candidate = n + 1
    if candidate < 2:
        candidate = 2
    while is_prime(candidate) == 0:
        candidate = candidate + 1
    return candidate


def modular_exp(num, exp, mod):
    """Modular exponentiation: (num^exp) % mod."""
    if mod <= 0:
        return 0
    if mod == 1:
        return 0
    result = 1
    b = num % mod
    if b < 0:
        b = b + mod
    e = exp
    while e > 0:
        if e & 1 == 1:
            result = (result * b) % mod
        e = e >> 1
        b = (b * b) % mod
    return result


def sum_divisors(n):
    """Sum of all proper divisors of n."""
    if n <= 1:
        return 0
    total = 1
    d = 2
    while d * d <= n:
        if n % d == 0:
            total = total + d
            if d != n // d:
                total = total + n // d
        d = d + 1
    return total


def is_perfect_number(n):
    """Check if n is a perfect number. Returns 1 if yes, 0 if no."""
    if n <= 1:
        return 0
    if sum_divisors(n) == n:
        return 1
    return 0


def test_module() -> int:
    """Test all math inference functions."""
    total: int = 0

    # absolute tests
    if absolute(5) == 5:
        total = total + 1
    if absolute(0 - 7) == 7:
        total = total + 1
    if absolute(0) == 0:
        total = total + 1

    # power_int tests
    if power_int(2, 10) == 1024:
        total = total + 1
    if power_int(3, 0) == 1:
        total = total + 1

    # isqrt_newton tests
    if isqrt_newton(0) == 0:
        total = total + 1
    if isqrt_newton(4) == 2:
        total = total + 1
    if isqrt_newton(8) == 2:
        total = total + 1

    # is_prime tests
    if is_prime(2) == 1:
        total = total + 1
    if is_prime(17) == 1:
        total = total + 1
    if is_prime(4) == 0:
        total = total + 1

    # next_prime tests
    if next_prime(10) == 11:
        total = total + 1
    if next_prime(1) == 2:
        total = total + 1

    # modular_exp tests
    if modular_exp(2, 10, 1000) == 24:
        total = total + 1

    # is_perfect_number tests
    if is_perfect_number(6) == 1:
        total = total + 1
    if is_perfect_number(28) == 1:
        total = total + 1
    if is_perfect_number(12) == 0:
        total = total + 1

    return total
