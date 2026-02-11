"""Integer partition counting using dynamic programming."""


def count_partitions(n: int) -> int:
    """Count number of ways to partition integer n."""
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


def count_partitions_max_k(n: int, k: int) -> int:
    """Count partitions of n using parts at most k."""
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
    part: int = 1
    while part <= k:
        j: int = part
        while j <= n:
            dp[j] = dp[j] + dp[j - part]
            j = j + 1
        part = part + 1
    return dp[n]


def count_partitions_exact_k(n: int, k: int) -> int:
    """Count partitions of n into exactly k parts."""
    if k <= 0 or n <= 0:
        return 0
    if k > n:
        return 0
    if k == n:
        return 1
    dp: list[int] = []
    i: int = 0
    total: int = (n + 1) * (k + 1)
    while i < total:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    i = 1
    while i <= n:
        j: int = 1
        while j <= k and j <= i:
            dp[i * (k + 1) + j] = dp[(i - 1) * (k + 1) + j - 1] + dp[(i - j) * (k + 1) + j]
            j = j + 1
        i = i + 1
    return dp[n * (k + 1) + k]


def test_module() -> int:
    """Test partition counting."""
    ok: int = 0
    if count_partitions(0) == 1:
        ok = ok + 1
    if count_partitions(1) == 1:
        ok = ok + 1
    if count_partitions(4) == 5:
        ok = ok + 1
    if count_partitions(5) == 7:
        ok = ok + 1
    if count_partitions_max_k(5, 2) == 3:
        ok = ok + 1
    if count_partitions_max_k(5, 3) == 5:
        ok = ok + 1
    if count_partitions_exact_k(5, 2) == 2:
        ok = ok + 1
    return ok
