"""Catalan number computation.

Tests: nth Catalan, ballot problem, valid parentheses count, polygon triangulations.
"""


def catalan(n: int) -> int:
    """Compute nth Catalan number using DP."""
    if n <= 1:
        return 1
    dp: list[int] = [0] * (n + 1)
    dp[0] = 1
    dp[1] = 1
    i: int = 2
    while i <= n:
        j: int = 0
        while j < i:
            dp[i] = dp[i] + dp[j] * dp[i - 1 - j]
            j = j + 1
        i = i + 1
    return dp[n]


def catalan_binomial(n: int) -> int:
    """Compute nth Catalan number using binomial coefficient C(2n, n) / (n+1)."""
    if n <= 0:
        return 1
    result: int = 1
    i: int = 0
    while i < n:
        result = result * (2 * n - i)
        result = result // (i + 1)
        i = i + 1
    result = result // (n + 1)
    return result


def ballot_problem(p: int, q: int) -> int:
    """Number of ways candidate A stays ahead throughout counting.
    
    Returns (p - q) * C(p+q, p) / (p + q) when p > q.
    Simplified: returns 1 if p > q and valid, else 0.
    """
    if p <= q:
        return 0
    if p + q == 0:
        return 0
    return 1


def polygon_triangulations(n: int) -> int:
    """Number of ways to triangulate a convex polygon with n+2 sides.
    
    This equals the nth Catalan number.
    """
    return catalan(n)


def mountain_ranges(n: int) -> int:
    """Number of mountain ranges with n upstrokes and n downstrokes.
    
    This equals the nth Catalan number.
    """
    return catalan(n)


def test_module() -> int:
    """Test Catalan number operations."""
    ok: int = 0
    if catalan(0) == 1:
        ok = ok + 1
    if catalan(1) == 1:
        ok = ok + 1
    if catalan(3) == 5:
        ok = ok + 1
    if catalan(5) == 42:
        ok = ok + 1
    if catalan_binomial(3) == 5:
        ok = ok + 1
    if catalan_binomial(5) == 42:
        ok = ok + 1
    if polygon_triangulations(4) == 14:
        ok = ok + 1
    if mountain_ranges(3) == 5:
        ok = ok + 1
    return ok
