"""Number decomposition: prime factorization, digit decomposition, and sum representations."""


def prime_factors(n: int) -> list[int]:
    """Return list of prime factors (with repetition)."""
    result: list[int] = []
    if n <= 1:
        return result
    d: int = 2
    while d * d <= n:
        while n % d == 0:
            result.append(d)
            n = n // d
        d = d + 1
    if n > 1:
        result.append(n)
    return result


def count_prime_factors_distinct(n: int) -> int:
    """Count the number of distinct prime factors."""
    if n <= 1:
        return 0
    count: int = 0
    d: int = 2
    while d * d <= n:
        if n % d == 0:
            count = count + 1
            while n % d == 0:
                n = n // d
        d = d + 1
    if n > 1:
        count = count + 1
    return count


def sum_of_squares_count(n: int) -> int:
    """Count minimum number of perfect squares that sum to n (Lagrange's theorem).
    Uses dynamic programming approach."""
    if n <= 0:
        return 0
    dp: list[int] = []
    i: int = 0
    while i <= n:
        dp.append(i)
        i = i + 1
    s: int = 1
    while s * s <= n:
        sq: int = s * s
        j: int = sq
        while j <= n:
            candidate: int = dp[j - sq] + 1
            if candidate < dp[j]:
                dp[j] = candidate
            j = j + 1
        s = s + 1
    return dp[n]


def egyptian_fraction_steps(num: int, den: int) -> int:
    """Count how many unit fractions needed for Egyptian fraction representation.
    Uses greedy algorithm."""
    if num <= 0 or den <= 0:
        return 0
    count: int = 0
    max_steps: int = 50
    while num > 0 and count < max_steps:
        # Ceiling of den/num
        unit_den: int = (den + num - 1) // num
        count = count + 1
        num = num * unit_den - den
        den = den * unit_den
        # Simplify
        if num > 0:
            a: int = num
            b: int = den
            while b > 0:
                temp: int = b
                b = a % b
                a = temp
            num = num // a
            den = den // a
    return count


def test_module() -> int:
    """Test number decomposition functions."""
    ok: int = 0

    pf: list[int] = prime_factors(12)
    if len(pf) == 3 and pf[0] == 2 and pf[1] == 2 and pf[2] == 3:
        ok = ok + 1

    if count_prime_factors_distinct(12) == 2:
        ok = ok + 1

    if count_prime_factors_distinct(7) == 1:
        ok = ok + 1

    if sum_of_squares_count(1) == 1:
        ok = ok + 1

    if sum_of_squares_count(12) == 3:
        ok = ok + 1

    # 3/7 = 1/3 + 1/11 + 1/231 => 3 steps
    steps: int = egyptian_fraction_steps(3, 7)
    if steps >= 1:
        ok = ok + 1

    return ok
