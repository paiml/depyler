# Type inference test: GCD/LCM computations with untyped params
# Strategy: Return types annotated, parameter types MISSING on some functions


def gcd_euclidean(a, b) -> int:
    """GCD using Euclidean algorithm - params inferred from modulo usage."""
    if a < 0:
        a = 0 - a
    if b < 0:
        b = 0 - b
    while b != 0:
        temp: int = b
        b = a % b
        a = temp
    return a


def lcm_compute(a, b) -> int:
    """LCM using GCD - params inferred from arithmetic."""
    if a == 0 or b == 0:
        return 0
    g: int = gcd_euclidean(a, b)
    if g == 0:
        return 0
    va: int = a
    vb: int = b
    if va < 0:
        va = 0 - va
    if vb < 0:
        vb = 0 - vb
    return (va // g) * vb


def gcd_binary(a, b) -> int:
    """Binary GCD algorithm (Stein's algorithm)."""
    if a < 0:
        a = 0 - a
    if b < 0:
        b = 0 - b
    if a == 0:
        return b
    if b == 0:
        return a
    shift: int = 0
    while ((a | b) & 1) == 0:
        a = a >> 1
        b = b >> 1
        shift = shift + 1
    while (a & 1) == 0:
        a = a >> 1
    while b != 0:
        while (b & 1) == 0:
            b = b >> 1
        if a > b:
            temp: int = a
            a = b
            b = temp
        b = b - a
    return a << shift


def coprime_check(a, b) -> int:
    """Check if two numbers are coprime. Returns 1 if yes, 0 if no."""
    g: int = gcd_euclidean(a, b)
    if g == 1:
        return 1
    return 0


def euler_totient_simple(n) -> int:
    """Euler's totient function using GCD."""
    if n <= 0:
        return 0
    if n == 1:
        return 1
    count: int = 0
    i: int = 1
    while i < n:
        if coprime_check(i, n) == 1:
            count = count + 1
        i = i + 1
    return count


def gcd_of_three(a, b, c) -> int:
    """GCD of three numbers."""
    return gcd_euclidean(gcd_euclidean(a, b), c)


def test_module() -> int:
    """Test all GCD/LCM inference functions."""
    total: int = 0

    # gcd_euclidean tests
    if gcd_euclidean(12, 8) == 4:
        total = total + 1
    if gcd_euclidean(0, 5) == 5:
        total = total + 1
    if gcd_euclidean(7, 0) == 7:
        total = total + 1
    if gcd_euclidean(17, 13) == 1:
        total = total + 1

    # lcm_compute tests
    if lcm_compute(4, 6) == 12:
        total = total + 1
    if lcm_compute(0, 5) == 0:
        total = total + 1

    # gcd_binary tests
    if gcd_binary(12, 8) == 4:
        total = total + 1
    if gcd_binary(100, 75) == 25:
        total = total + 1

    # coprime_check tests
    if coprime_check(7, 11) == 1:
        total = total + 1
    if coprime_check(6, 9) == 0:
        total = total + 1

    # euler_totient_simple tests
    if euler_totient_simple(1) == 1:
        total = total + 1
    if euler_totient_simple(12) == 4:
        total = total + 1

    # gcd_of_three tests
    if gcd_of_three(12, 18, 24) == 6:
        total = total + 1
    if gcd_of_three(7, 11, 13) == 1:
        total = total + 1

    return total
