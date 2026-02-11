"""Count derangements (permutations with no fixed point) using DP."""


def count_derangements(n: int) -> int:
    """Count derangements of n elements using DP."""
    if n == 0:
        return 1
    if n == 1:
        return 0
    if n == 2:
        return 1
    dp: list[int] = []
    i: int = 0
    while i <= n:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    dp[1] = 0
    dp[2] = 1
    i = 3
    while i <= n:
        dp[i] = (i - 1) * (dp[i - 1] + dp[i - 2])
        i = i + 1
    return dp[n]


def subfactorial(n: int) -> int:
    """Subfactorial (another name for derangements)."""
    return count_derangements(n)


def factorial(n: int) -> int:
    """Compute n!."""
    if n <= 1:
        return 1
    result: int = 1
    i: int = 2
    while i <= n:
        result = result * i
        i = i + 1
    return result


def derangement_probability_num(n: int) -> int:
    """Numerator of derangement probability D(n)/n!."""
    return count_derangements(n)


def derangement_probability_den(n: int) -> int:
    """Denominator of derangement probability D(n)/n!."""
    return factorial(n)


def test_module() -> int:
    """Test derangement counting."""
    ok: int = 0
    if count_derangements(0) == 1:
        ok = ok + 1
    if count_derangements(1) == 0:
        ok = ok + 1
    if count_derangements(2) == 1:
        ok = ok + 1
    if count_derangements(3) == 2:
        ok = ok + 1
    if count_derangements(4) == 9:
        ok = ok + 1
    if count_derangements(5) == 44:
        ok = ok + 1
    if subfactorial(5) == 44:
        ok = ok + 1
    if factorial(5) == 120:
        ok = ok + 1
    return ok
