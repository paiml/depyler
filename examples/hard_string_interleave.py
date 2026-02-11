"""Check if a string is an interleaving of two other strings.

Tests: valid interleaves, invalid interleaves, empty strings, single chars.
"""


def is_interleave(s1: str, s2: str, s3: str) -> int:
    """Return 1 if s3 is an interleaving of s1 and s2, 0 otherwise."""
    m: int = len(s1)
    n: int = len(s2)
    if m + n != len(s3):
        return 0
    dp: list[list[int]] = []
    i: int = 0
    while i <= m:
        row: list[int] = []
        j: int = 0
        while j <= n:
            row.append(0)
            j = j + 1
        dp.append(row)
        i = i + 1
    dp[0][0] = 1
    i = 1
    while i <= m:
        if s1[i - 1] == s3[i - 1] and dp[i - 1][0] == 1:
            dp[i][0] = 1
        i = i + 1
    j: int = 1
    while j <= n:
        if s2[j - 1] == s3[j - 1] and dp[0][j - 1] == 1:
            dp[0][j] = 1
        j = j + 1
    i = 1
    while i <= m:
        j = 1
        while j <= n:
            c: str = s3[i + j - 1]
            if s1[i - 1] == c and dp[i - 1][j] == 1:
                dp[i][j] = 1
            if s2[j - 1] == c and dp[i][j - 1] == 1:
                dp[i][j] = 1
            j = j + 1
        i = i + 1
    return dp[m][n]


def count_interleaves(s1: str, s2: str, s3: str) -> int:
    """Return number of ways s3 can be formed as interleave of s1 and s2."""
    m: int = len(s1)
    n: int = len(s2)
    if m + n != len(s3):
        return 0
    dp: list[list[int]] = []
    i: int = 0
    while i <= m:
        row: list[int] = []
        j: int = 0
        while j <= n:
            row.append(0)
            j = j + 1
        dp.append(row)
        i = i + 1
    dp[0][0] = 1
    i = 1
    while i <= m:
        if s1[i - 1] == s3[i - 1] and dp[i - 1][0] > 0:
            dp[i][0] = dp[i - 1][0]
        i = i + 1
    j: int = 1
    while j <= n:
        if s2[j - 1] == s3[j - 1] and dp[0][j - 1] > 0:
            dp[0][j] = dp[0][j - 1]
        j = j + 1
    i = 1
    while i <= m:
        j = 1
        while j <= n:
            c: str = s3[i + j - 1]
            if s1[i - 1] == c:
                dp[i][j] = dp[i][j] + dp[i - 1][j]
            if s2[j - 1] == c:
                dp[i][j] = dp[i][j] + dp[i][j - 1]
            j = j + 1
        i = i + 1
    return dp[m][n]


def test_module() -> int:
    """Test string interleaving."""
    ok: int = 0

    if is_interleave("aab", "axy", "aaxaby") == 1:
        ok = ok + 1
    if is_interleave("aab", "axy", "abaaxy") == 0:
        ok = ok + 1
    if is_interleave("", "", "") == 1:
        ok = ok + 1
    if is_interleave("a", "", "a") == 1:
        ok = ok + 1
    if is_interleave("", "b", "b") == 1:
        ok = ok + 1
    if is_interleave("abc", "def", "adbcef") == 1:
        ok = ok + 1
    if is_interleave("abc", "def", "abcdef") == 1:
        ok = ok + 1

    if is_interleave("xy", "ab", "xayb") == 1:
        ok = ok + 1

    if count_interleaves("a", "a", "aa") == 2:
        ok = ok + 1

    if is_interleave("ab", "cd", "acbd") == 1:
        ok = ok + 1

    return ok
