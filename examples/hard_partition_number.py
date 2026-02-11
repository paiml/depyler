# Integer partitions (number of ways to sum to n)


def partition_count(n: int) -> int:
    # Number of ways to write n as sum of positive integers
    # Using dynamic programming
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


def partition_count_distinct(n: int) -> int:
    # Number of partitions with distinct parts
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


def partition_count_at_most_k(n: int, k: int) -> int:
    # Partitions where each part is at most k
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


def partition_into_k_parts(n: int, k: int) -> int:
    # Number of ways to partition n into exactly k parts
    # p(n, k) = p(n-1, k-1) + p(n-k, k)
    if n <= 0 or k <= 0:
        return 0
    # Build 2D dp as flat array
    cols: int = k + 1
    rows: int = n + 1
    dp: list[int] = []
    i = 0
    while i < rows * cols:
        dp.append(0)
        i = i + 1
    dp[0 * cols + 0] = 1  # p(0, 0) = 1
    r: int = 1
    while r <= n:
        c: int = 1
        while c <= k and c <= r:
            dp[r * cols + c] = dp[(r - 1) * cols + (c - 1)] + dp[(r - c) * cols + c]
            c = c + 1
        r = r + 1
    return dp[n * cols + k]


def test_module() -> int:
    passed: int = 0

    # Test 1: p(0) = 1
    if partition_count(0) == 1:
        passed = passed + 1

    # Test 2: p(4) = 5 (4, 3+1, 2+2, 2+1+1, 1+1+1+1)
    if partition_count(4) == 5:
        passed = passed + 1

    # Test 3: p(5) = 7
    if partition_count(5) == 7:
        passed = passed + 1

    # Test 4: distinct partitions of 5 = 3 (5, 4+1, 3+2)
    if partition_count_distinct(5) == 3:
        passed = passed + 1

    # Test 5: distinct partitions of 6 = 4 (6, 5+1, 4+2, 3+2+1)
    if partition_count_distinct(6) == 4:
        passed = passed + 1

    # Test 6: partitions of 5 with parts at most 2 = 3 (2+2+1, 2+1+1+1, 1+1+1+1+1)
    if partition_count_at_most_k(5, 2) == 3:
        passed = passed + 1

    # Test 7: partition 6 into exactly 2 parts = 3 (5+1, 4+2, 3+3)
    if partition_into_k_parts(6, 2) == 3:
        passed = passed + 1

    # Test 8: partition 7 into 3 parts = 4 (5+1+1, 4+2+1, 3+3+1, 3+2+2)
    if partition_into_k_parts(7, 3) == 4:
        passed = passed + 1

    return passed
