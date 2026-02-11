# Stirling numbers of second kind


def stirling2(n: int, k: int) -> int:
    # S(n, k): number of ways to partition n elements into k non-empty subsets
    if n == 0 and k == 0:
        return 1
    if n == 0 or k == 0:
        return 0
    if k > n:
        return 0
    cols: int = k + 1
    rows: int = n + 1
    dp: list[int] = []
    i: int = 0
    while i < rows * cols:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    r: int = 1
    while r <= n:
        c: int = 1
        while c <= k and c <= r:
            dp[r * cols + c] = c * dp[(r - 1) * cols + c] + dp[(r - 1) * cols + (c - 1)]
            c = c + 1
        r = r + 1
    return dp[n * cols + k]


def stirling2_row(n: int) -> list[int]:
    # Return [S(n,0), S(n,1), ..., S(n,n)]
    result: list[int] = []
    k: int = 0
    while k <= n:
        result.append(stirling2(n, k))
        k = k + 1
    return result


def stirling1_unsigned(n: int, k: int) -> int:
    # Unsigned Stirling numbers of first kind
    # |s(n,k)| = (n-1) * |s(n-1,k)| + |s(n-1,k-1)|
    if n == 0 and k == 0:
        return 1
    if n == 0 or k == 0:
        return 0
    if k > n:
        return 0
    cols: int = k + 1
    rows: int = n + 1
    dp: list[int] = []
    i: int = 0
    while i < rows * cols:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    r: int = 1
    while r <= n:
        c: int = 1
        while c <= k and c <= r:
            dp[r * cols + c] = (r - 1) * dp[(r - 1) * cols + c] + dp[(r - 1) * cols + (c - 1)]
            c = c + 1
        r = r + 1
    return dp[n * cols + k]


def bell_from_stirling(n: int) -> int:
    total: int = 0
    k: int = 0
    while k <= n:
        total = total + stirling2(n, k)
        k = k + 1
    return total


def test_module() -> int:
    passed: int = 0

    # Test 1: S(0,0) = 1
    if stirling2(0, 0) == 1:
        passed = passed + 1

    # Test 2: S(n,1) = 1 for all n >= 1
    if stirling2(5, 1) == 1:
        passed = passed + 1

    # Test 3: S(n,n) = 1
    if stirling2(5, 5) == 1:
        passed = passed + 1

    # Test 4: S(4,2) = 7
    if stirling2(4, 2) == 7:
        passed = passed + 1

    # Test 5: S(5,3) = 25
    if stirling2(5, 3) == 25:
        passed = passed + 1

    # Test 6: stirling2 row for n=4
    row: list[int] = stirling2_row(4)
    if row[0] == 0 and row[1] == 1 and row[2] == 7 and row[3] == 6 and row[4] == 1:
        passed = passed + 1

    # Test 7: unsigned Stirling first kind |s(4,2)| = 11
    if stirling1_unsigned(4, 2) == 11:
        passed = passed + 1

    # Test 8: Bell from Stirling B(4) = 15
    if bell_from_stirling(4) == 15:
        passed = passed + 1

    return passed
