"""Compute Catalan numbers using dynamic programming."""


def catalan_dp(n: int) -> int:
    """Compute nth Catalan number using DP table."""
    if n <= 1:
        return 1
    dp: list[int] = []
    i: int = 0
    while i <= n:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    dp[1] = 1
    i = 2
    while i <= n:
        j: int = 0
        while j < i:
            dp[i] = dp[i] + dp[j] * dp[i - 1 - j]
            j = j + 1
        i = i + 1
    return dp[n]


def catalan_recursive_memo(n: int, memo: list[int]) -> int:
    """Catalan with memoization. memo[i] = -1 means not computed."""
    if n <= 1:
        return 1
    if memo[n] != 0 - 1:
        return memo[n]
    result: int = 0
    k: int = 0
    while k < n:
        left: int = catalan_recursive_memo(k, memo)
        right: int = catalan_recursive_memo(n - 1 - k, memo)
        result = result + left * right
        k = k + 1
    memo[n] = result
    return result


def catalan_with_memo(n: int) -> int:
    """Helper to compute Catalan with memo table."""
    memo: list[int] = []
    i: int = 0
    while i <= n:
        memo.append(0 - 1)
        i = i + 1
    return catalan_recursive_memo(n, memo)


def count_bst_trees(n: int) -> int:
    """Number of structurally unique BSTs with n nodes equals Catalan(n)."""
    return catalan_dp(n)


def test_module() -> int:
    """Test Catalan number computations."""
    ok: int = 0
    if catalan_dp(0) == 1:
        ok = ok + 1
    if catalan_dp(1) == 1:
        ok = ok + 1
    if catalan_dp(2) == 2:
        ok = ok + 1
    if catalan_dp(3) == 5:
        ok = ok + 1
    if catalan_dp(4) == 14:
        ok = ok + 1
    if catalan_dp(5) == 42:
        ok = ok + 1
    if catalan_with_memo(5) == 42:
        ok = ok + 1
    if count_bst_trees(4) == 14:
        ok = ok + 1
    return ok
