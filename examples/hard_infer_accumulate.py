# Type inference test: Running sums and products
# Strategy: Only test_module() -> int annotated, everything else inferred


def running_sum(n):
    """Sum of 1 + 2 + ... + n."""
    total = 0
    i = 1
    while i <= n:
        total = total + i
        i = i + 1
    return total


def running_product(n):
    """Product of 1 * 2 * ... * n (factorial)."""
    if n <= 0:
        return 1
    result = 1
    i = 1
    while i <= n:
        result = result * i
        i = i + 1
    return result


def sum_of_squares(n):
    """Sum of 1^2 + 2^2 + ... + n^2."""
    total = 0
    i = 1
    while i <= n:
        total = total + i * i
        i = i + 1
    return total


def sum_of_cubes(n):
    """Sum of 1^3 + 2^3 + ... + n^3."""
    total = 0
    i = 1
    while i <= n:
        total = total + i * i * i
        i = i + 1
    return total


def alternating_sum(n):
    """1 - 2 + 3 - 4 + ... +/- n."""
    total = 0
    i = 1
    sign = 1
    while i <= n:
        total = total + sign * i
        sign = 0 - sign
        i = i + 1
    return total


def geometric_sum(first, ratio, terms):
    """Sum of geometric series: first + first*ratio + first*ratio^2 + ..."""
    if terms <= 0:
        return 0
    total = 0
    current = first
    i = 0
    while i < terms:
        total = total + current
        current = current * ratio
        i = i + 1
    return total


def harmonic_sum_approx(n, scale):
    """Approximate harmonic sum H(n) = 1/1 + 1/2 + ... + 1/n
    using fixed-point with given scale factor."""
    if n <= 0:
        return 0
    total = 0
    i = 1
    while i <= n:
        total = total + scale // i
        i = i + 1
    return total


def power_sum(n, exp):
    """Sum of i^exp for i from 1 to n."""
    total = 0
    i = 1
    while i <= n:
        power = 1
        j = 0
        while j < exp:
            power = power * i
            j = j + 1
        total = total + power
        i = i + 1
    return total


def test_module() -> int:
    """Test all accumulation inference functions."""
    total: int = 0

    # running_sum tests
    if running_sum(10) == 55:
        total = total + 1
    if running_sum(0) == 0:
        total = total + 1
    if running_sum(100) == 5050:
        total = total + 1

    # running_product tests
    if running_product(5) == 120:
        total = total + 1
    if running_product(0) == 1:
        total = total + 1

    # sum_of_squares tests
    if sum_of_squares(3) == 14:
        total = total + 1
    if sum_of_squares(10) == 385:
        total = total + 1

    # sum_of_cubes tests
    if sum_of_cubes(3) == 36:
        total = total + 1

    # alternating_sum tests
    if alternating_sum(4) == 0 - 2:
        total = total + 1
    if alternating_sum(5) == 3:
        total = total + 1

    # geometric_sum tests
    if geometric_sum(1, 2, 5) == 31:
        total = total + 1
    if geometric_sum(3, 1, 4) == 12:
        total = total + 1
    if geometric_sum(1, 3, 0) == 0:
        total = total + 1

    # harmonic_sum_approx tests
    h10: int = harmonic_sum_approx(10, 10000)
    if h10 > 28000 and h10 < 30000:
        total = total + 1

    # power_sum tests
    if power_sum(3, 2) == 14:
        total = total + 1

    return total
