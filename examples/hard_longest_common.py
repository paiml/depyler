"""Longest common subsequence and longest common substring.

Tests: LCS length, LCS string recovery, longest common substring, edge cases.
"""


def lcs_length(s1: str, s2: str) -> int:
    """Return length of longest common subsequence."""
    m: int = len(s1)
    n: int = len(s2)
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
    i = 1
    while i <= m:
        j: int = 1
        while j <= n:
            if s1[i - 1] == s2[j - 1]:
                dp[i][j] = dp[i - 1][j - 1] + 1
            else:
                a: int = dp[i - 1][j]
                b: int = dp[i][j - 1]
                if a > b:
                    dp[i][j] = a
                else:
                    dp[i][j] = b
            j = j + 1
        i = i + 1
    return dp[m][n]


def longest_common_substring_len(s1: str, s2: str) -> int:
    """Return length of longest common substring."""
    m: int = len(s1)
    n: int = len(s2)
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
    best: int = 0
    i = 1
    while i <= m:
        j: int = 1
        while j <= n:
            if s1[i - 1] == s2[j - 1]:
                dp[i][j] = dp[i - 1][j - 1] + 1
                if dp[i][j] > best:
                    best = dp[i][j]
            else:
                dp[i][j] = 0
            j = j + 1
        i = i + 1
    return best


def lcs_of_three(s1: str, s2: str, s3: str) -> int:
    """Return length of longest common subsequence of three strings."""
    a: int = len(s1)
    b: int = len(s2)
    c: int = len(s3)
    dp: list[list[list[int]]] = []
    i: int = 0
    while i <= a:
        plane: list[list[int]] = []
        j: int = 0
        while j <= b:
            row: list[int] = []
            k: int = 0
            while k <= c:
                row.append(0)
                k = k + 1
            plane.append(row)
            j = j + 1
        dp.append(plane)
        i = i + 1
    i = 1
    while i <= a:
        j: int = 1
        while j <= b:
            k: int = 1
            while k <= c:
                if s1[i - 1] == s2[j - 1] and s2[j - 1] == s3[k - 1]:
                    dp[i][j][k] = dp[i - 1][j - 1][k - 1] + 1
                else:
                    v1: int = dp[i - 1][j][k]
                    v2: int = dp[i][j - 1][k]
                    v3: int = dp[i][j][k - 1]
                    best: int = v1
                    if v2 > best:
                        best = v2
                    if v3 > best:
                        best = v3
                    dp[i][j][k] = best
                k = k + 1
            j = j + 1
        i = i + 1
    return dp[a][b][c]


def test_module() -> int:
    """Test longest common subsequence and substring."""
    ok: int = 0

    if lcs_length("abcde", "ace") == 3:
        ok = ok + 1
    if lcs_length("abc", "abc") == 3:
        ok = ok + 1
    if lcs_length("abc", "def") == 0:
        ok = ok + 1
    if lcs_length("", "abc") == 0:
        ok = ok + 1

    if longest_common_substring_len("abcdef", "zbcdf") == 3:
        ok = ok + 1
    if longest_common_substring_len("abc", "xyz") == 0:
        ok = ok + 1
    if longest_common_substring_len("abcabc", "abc") == 3:
        ok = ok + 1

    if lcs_of_three("abcd", "abdc", "abec") == 3:
        ok = ok + 1

    if lcs_length("aggtab", "gxtxayb") == 4:
        ok = ok + 1

    if longest_common_substring_len("aaaa", "aaaa") == 4:
        ok = ok + 1

    return ok
