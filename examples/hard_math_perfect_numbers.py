"""Perfect, abundant, and deficient number classification."""


def sum_proper_divisors(n: int) -> int:
    """Sum of proper divisors of n."""
    if n <= 1:
        return 0
    total: int = 1
    i: int = 2
    while i * i <= n:
        if n % i == 0:
            total = total + i
            if i != n // i:
                total = total + n // i
        i = i + 1
    return total


def is_perfect(n: int) -> int:
    """Returns 1 if n is a perfect number."""
    if n <= 1:
        return 0
    if sum_proper_divisors(n) == n:
        return 1
    return 0


def classify_number(n: int) -> int:
    """Classify: 0=deficient, 1=perfect, 2=abundant."""
    s: int = sum_proper_divisors(n)
    if s == n:
        return 1
    if s > n:
        return 2
    return 0


def are_amicable(a: int, b: int) -> int:
    """Returns 1 if a and b are amicable numbers."""
    if a == b:
        return 0
    sa: int = sum_proper_divisors(a)
    sb: int = sum_proper_divisors(b)
    if sa == b and sb == a:
        return 1
    return 0


def test_module() -> int:
    """Test perfect number functions."""
    ok: int = 0
    if is_perfect(6) == 1:
        ok = ok + 1
    if is_perfect(28) == 1:
        ok = ok + 1
    if classify_number(8) == 0:
        ok = ok + 1
    if classify_number(12) == 2:
        ok = ok + 1
    if are_amicable(220, 284) == 1:
        ok = ok + 1
    return ok
