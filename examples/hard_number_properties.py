"""Number property checks: harshad, automorphic, kaprekar, and more."""


def is_harshad(n: int) -> int:
    """Check if n is a Harshad number (divisible by sum of its digits).
    Returns 1 if true, 0 otherwise."""
    if n <= 0:
        return 0
    digit_sum: int = 0
    temp: int = n
    while temp > 0:
        digit_sum = digit_sum + temp % 10
        temp = temp // 10
    if digit_sum == 0:
        return 0
    if n % digit_sum == 0:
        return 1
    return 0


def is_automorphic(n: int) -> int:
    """Check if n squared ends with n. Returns 1 if true, 0 otherwise."""
    if n < 0:
        return 0
    sq: int = n * n
    temp: int = n
    divisor: int = 1
    while temp > 0:
        divisor = divisor * 10
        temp = temp // 10
    if sq % divisor == n:
        return 1
    return 0


def is_spy_number(n: int) -> int:
    """Check if sum of digits equals product of digits.
    Returns 1 if true, 0 otherwise."""
    if n <= 0:
        return 0
    digit_sum: int = 0
    digit_product: int = 1
    temp: int = n
    while temp > 0:
        d: int = temp % 10
        digit_sum = digit_sum + d
        digit_product = digit_product * d
        temp = temp // 10
    if digit_sum == digit_product:
        return 1
    return 0


def count_divisors(n: int) -> int:
    """Count the number of divisors of n."""
    if n <= 0:
        return 0
    count: int = 0
    i: int = 1
    while i * i <= n:
        if n % i == 0:
            count = count + 1
            if i != n // i:
                count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test number property functions."""
    ok: int = 0

    if is_harshad(18) == 1:
        ok = ok + 1

    if is_harshad(19) == 0:
        ok = ok + 1

    if is_automorphic(5) == 1:
        ok = ok + 1

    if is_automorphic(76) == 1:
        ok = ok + 1

    if is_automorphic(7) == 0:
        ok = ok + 1

    if is_spy_number(123) == 1:
        ok = ok + 1

    if count_divisors(12) == 6:
        ok = ok + 1

    return ok
