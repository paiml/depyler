"""Combinatorial math operations.

Tests: choose, permutation count, derangement, Stirling numbers.
"""


def choose(n: int, k: int) -> int:
    """Compute binomial coefficient C(n, k)."""
    if k < 0 or k > n:
        return 0
    if k == 0 or k == n:
        return 1
    kk: int = k
    if kk > n - kk:
        kk = n - kk
    result: int = 1
    i: int = 0
    while i < kk:
        result = result * (n - i)
        result = result // (i + 1)
        i = i + 1
    return result


def permutation_count(n: int, r: int) -> int:
    """Compute P(n, r) = n! / (n-r)!"""
    if r < 0 or r > n:
        return 0
    result: int = 1
    i: int = 0
    while i < r:
        result = result * (n - i)
        i = i + 1
    return result


def derangement(n: int) -> int:
    """Count derangements (permutations with no fixed points)."""
    if n == 0:
        return 1
    if n == 1:
        return 0
    a: int = 1
    b: int = 0
    i: int = 2
    while i <= n:
        temp: int = (i - 1) * (a + b)
        a = b
        b = temp
        i = i + 1
    return b


def multichoose(n: int, k: int) -> int:
    """Multiset coefficient: ways to choose k from n with repetition."""
    return choose(n + k - 1, k)


def test_module() -> int:
    """Test combinatorial operations."""
    ok: int = 0
    if choose(5, 2) == 10:
        ok = ok + 1
    if choose(10, 3) == 120:
        ok = ok + 1
    if choose(0, 0) == 1:
        ok = ok + 1
    if permutation_count(5, 3) == 60:
        ok = ok + 1
    if permutation_count(4, 4) == 24:
        ok = ok + 1
    if derangement(3) == 2:
        ok = ok + 1
    if derangement(4) == 9:
        ok = ok + 1
    if multichoose(3, 2) == 6:
        ok = ok + 1
    return ok
