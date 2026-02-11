"""Binomial coefficient computation using Pascal's triangle.

Tests: nCr values, edge cases, symmetry property, sum of row, Pascal identity.
"""


def binomial(n: int, r: int) -> int:
    """Return C(n, r) using dynamic programming (Pascal's triangle)."""
    if r > n:
        return 0
    if r == 0 or r == n:
        return 1
    dp: list[list[int]] = []
    i: int = 0
    while i <= n:
        row: list[int] = []
        j: int = 0
        while j <= r:
            row.append(0)
            j = j + 1
        dp.append(row)
        i = i + 1
    i = 0
    while i <= n:
        dp[i][0] = 1
        i = i + 1
    i = 1
    while i <= n:
        limit: int = i
        if r < limit:
            limit = r
        j: int = 1
        while j <= limit:
            dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j]
            j = j + 1
        i = i + 1
    return dp[n][r]


def pascal_row(n: int) -> list[int]:
    """Return the nth row of Pascal's triangle (0-indexed)."""
    row: list[int] = []
    row.append(1)
    k: int = 1
    while k <= n:
        val: int = row[k - 1] * (n - k + 1) // k
        row.append(val)
        k = k + 1
    return row


def catalan_number(n: int) -> int:
    """Return the nth Catalan number using binomial coefficients."""
    return binomial(2 * n, n) // (n + 1)


def sum_of_pascal_row(n: int) -> int:
    """Return sum of nth row of Pascal's triangle (should be 2^n)."""
    row: list[int] = pascal_row(n)
    total: int = 0
    i: int = 0
    while i < len(row):
        total = total + row[i]
        i = i + 1
    return total


def test_module() -> int:
    """Test binomial coefficient computations."""
    ok: int = 0

    if binomial(5, 2) == 10:
        ok = ok + 1
    if binomial(10, 3) == 120:
        ok = ok + 1
    if binomial(0, 0) == 1:
        ok = ok + 1
    if binomial(5, 0) == 1:
        ok = ok + 1
    if binomial(5, 5) == 1:
        ok = ok + 1
    if binomial(3, 5) == 0:
        ok = ok + 1

    row4: list[int] = pascal_row(4)
    if len(row4) == 5:
        ok = ok + 1
    if row4[2] == 6:
        ok = ok + 1

    if catalan_number(4) == 14:
        ok = ok + 1

    if sum_of_pascal_row(5) == 32:
        ok = ok + 1

    return ok
