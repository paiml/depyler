"""Integer partition counting."""


def partition_count(n: int) -> int:
    """Count number of integer partitions of n using DP."""
    if n < 0:
        return 0
    if n == 0:
        return 1
    dp: list[int] = []
    i: int = 0
    while i <= n:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    k: int = 1
    while k <= n:
        j: int = k
        while j <= n:
            dp[j] = dp[j] + dp[j - k]
            j = j + 1
        k = k + 1
    return dp[n]


def partition_count_max(n: int, mx: int) -> int:
    """Count partitions of n with parts at most mx."""
    if n < 0:
        return 0
    if n == 0:
        return 1
    dp: list[int] = []
    i: int = 0
    while i <= n:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    k: int = 1
    while k <= mx:
        j: int = k
        while j <= n:
            dp[j] = dp[j] + dp[j - k]
            j = j + 1
        k = k + 1
    return dp[n]


def partition_count_distinct(n: int) -> int:
    """Count partitions of n into distinct parts."""
    if n < 0:
        return 0
    if n == 0:
        return 1
    dp: list[int] = []
    i: int = 0
    while i <= n:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    k: int = 1
    while k <= n:
        j: int = n
        while j >= k:
            dp[j] = dp[j] + dp[j - k]
            j = j - 1
        k = k + 1
    return dp[n]


def test_module() -> int:
    """Test partition functions."""
    ok: int = 0
    if partition_count(5) == 7:
        ok = ok + 1
    if partition_count(0) == 1:
        ok = ok + 1
    if partition_count_max(5, 3) == 5:
        ok = ok + 1
    if partition_count_distinct(5) == 3:
        ok = ok + 1
    if partition_count(10) == 42:
        ok = ok + 1
    return ok
