def prime_factors(n: int) -> list[int]:
    factors: list[int] = []
    val: int = n
    d: int = 2
    while d * d <= val:
        while val % d == 0:
            factors.append(d)
            val = val // d
        d = d + 1
    if val > 1:
        factors.append(val)
    return factors


def count_factors(n: int) -> int:
    count: int = 0
    d: int = 1
    while d * d <= n:
        if n % d == 0:
            count = count + 1
            if d != n // d:
                count = count + 1
        d = d + 1
    return count


def is_square_free(n: int) -> int:
    if n <= 1:
        return 1
    d: int = 2
    while d * d <= n:
        if n % (d * d) == 0:
            return 0
        d = d + 1
    return 1


def is_prime(n: int) -> int:
    if n < 2:
        return 0
    d: int = 2
    while d * d <= n:
        if n % d == 0:
            return 0
        d = d + 1
    return 1


def test_module() -> int:
    passed: int = 0
    if prime_factors(12) == [2, 2, 3]:
        passed = passed + 1
    if prime_factors(7) == [7]:
        passed = passed + 1
    if prime_factors(1) == []:
        passed = passed + 1
    if count_factors(12) == 6:
        passed = passed + 1
    if count_factors(7) == 2:
        passed = passed + 1
    if is_square_free(30) == 1:
        passed = passed + 1
    if is_square_free(12) == 0:
        passed = passed + 1
    if is_prime(17) == 1:
        passed = passed + 1
    if is_prime(1) == 0:
        passed = passed + 1
    return passed
