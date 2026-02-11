"""Count derangements (subfactorial): permutations where no element is in its original position.

Tests: small values, known derangement counts, relation to factorial, base cases.
"""


def count_derangements(n: int) -> int:
    """Return number of derangements of n elements. D(n) = (n-1)*(D(n-1) + D(n-2))."""
    if n == 0:
        return 1
    if n == 1:
        return 0
    dp: list[int] = []
    i: int = 0
    while i <= n:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    dp[1] = 0
    i = 2
    while i <= n:
        dp[i] = (i - 1) * (dp[i - 1] + dp[i - 2])
        i = i + 1
    return dp[n]


def factorial(n: int) -> int:
    """Return n factorial."""
    result: int = 1
    i: int = 2
    while i <= n:
        result = result * i
        i = i + 1
    return result


def derangement_probability_numerator(n: int) -> int:
    """Return numerator for D(n)/n! probability (D(n) itself)."""
    return count_derangements(n)


def derangement_probability_denominator(n: int) -> int:
    """Return denominator for D(n)/n! probability (n! itself)."""
    return factorial(n)


def partial_derangements(n: int, k: int) -> int:
    """Return number of permutations of n with exactly k fixed points.

    Formula: C(n, k) * D(n - k)
    """
    if k > n:
        return 0
    c: int = 1
    i: int = 0
    while i < k:
        c = c * (n - i) // (i + 1)
        i = i + 1
    d: int = count_derangements(n - k)
    return c * d


def test_module() -> int:
    """Test derangement computations."""
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

    if factorial(5) == 120:
        ok = ok + 1

    if partial_derangements(4, 0) == 9:
        ok = ok + 1

    if partial_derangements(4, 4) == 1:
        ok = ok + 1

    num: int = derangement_probability_numerator(5)
    den: int = derangement_probability_denominator(5)
    if num == 44 and den == 120:
        ok = ok + 1

    return ok
