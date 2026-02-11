"""Count unique BSTs (Catalan number variant).

Tests: unique BSTs, catalan, binomial coefficient, ballot problem.
"""


def count_unique_bst(n: int) -> int:
    """Number of structurally unique BSTs with n nodes."""
    dp: list[int] = []
    i: int = 0
    while i <= n:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    if n >= 1:
        dp[1] = 1
    i = 2
    while i <= n:
        j: int = 0
        while j < i:
            dp[i] = dp[i] + dp[j] * dp[i - 1 - j]
            j = j + 1
        i = i + 1
    return dp[n]


def catalan_number(n: int) -> int:
    """Nth Catalan number using DP."""
    return count_unique_bst(n)


def binomial_coeff(n: int, k: int) -> int:
    """Binomial coefficient C(n, k)."""
    if k > n:
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


def ballot_sequences(n: int) -> int:
    """Number of valid ballot sequences of length 2n (Catalan)."""
    return catalan_number(n)


def test_module() -> int:
    """Test unique BST count."""
    ok: int = 0
    if count_unique_bst(3) == 5:
        ok = ok + 1
    if count_unique_bst(4) == 14:
        ok = ok + 1
    if count_unique_bst(0) == 1:
        ok = ok + 1
    if catalan_number(5) == 42:
        ok = ok + 1
    if binomial_coeff(5, 2) == 10:
        ok = ok + 1
    if binomial_coeff(10, 3) == 120:
        ok = ok + 1
    if ballot_sequences(3) == 5:
        ok = ok + 1
    return ok
