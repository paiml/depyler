"""Longest common subsequence with backtracking."""


def lcs_length(s1: str, s2: str) -> int:
    """Find length of longest common subsequence."""
    m: int = len(s1)
    n: int = len(s2)
    # dp is flattened (m+1) x (n+1)
    dp: list[int] = []
    total: int = (m + 1) * (n + 1)
    k: int = 0
    while k < total:
        dp.append(0)
        k = k + 1
    i: int = 1
    while i <= m:
        j: int = 1
        while j <= n:
            if s1[i - 1] == s2[j - 1]:
                dp[i * (n + 1) + j] = dp[(i - 1) * (n + 1) + (j - 1)] + 1
            else:
                val1: int = dp[(i - 1) * (n + 1) + j]
                val2: int = dp[i * (n + 1) + (j - 1)]
                if val1 > val2:
                    dp[i * (n + 1) + j] = val1
                else:
                    dp[i * (n + 1) + j] = val2
            j = j + 1
        i = i + 1
    return dp[m * (n + 1) + n]


def lcs_string(s1: str, s2: str) -> str:
    """Find the actual LCS string via backtracking."""
    m: int = len(s1)
    n: int = len(s2)
    dp: list[int] = []
    total: int = (m + 1) * (n + 1)
    k: int = 0
    while k < total:
        dp.append(0)
        k = k + 1
    i: int = 1
    while i <= m:
        j: int = 1
        while j <= n:
            if s1[i - 1] == s2[j - 1]:
                dp[i * (n + 1) + j] = dp[(i - 1) * (n + 1) + (j - 1)] + 1
            else:
                val1: int = dp[(i - 1) * (n + 1) + j]
                val2: int = dp[i * (n + 1) + (j - 1)]
                if val1 > val2:
                    dp[i * (n + 1) + j] = val1
                else:
                    dp[i * (n + 1) + j] = val2
            j = j + 1
        i = i + 1
    # Backtrack
    result: str = ""
    bi: int = m
    bj: int = n
    while bi > 0 and bj > 0:
        if s1[bi - 1] == s2[bj - 1]:
            result = s1[bi - 1] + result
            bi = bi - 1
            bj = bj - 1
        elif dp[(bi - 1) * (n + 1) + bj] > dp[bi * (n + 1) + (bj - 1)]:
            bi = bi - 1
        else:
            bj = bj - 1
    return result


def edit_distance(s1: str, s2: str) -> int:
    """Compute edit distance (Levenshtein) between two strings."""
    m: int = len(s1)
    n: int = len(s2)
    dp: list[int] = []
    total: int = (m + 1) * (n + 1)
    k: int = 0
    while k < total:
        dp.append(0)
        k = k + 1
    i: int = 0
    while i <= m:
        dp[i * (n + 1)] = i
        i = i + 1
    j: int = 0
    while j <= n:
        dp[j] = j
        j = j + 1
    i = 1
    while i <= m:
        j = 1
        while j <= n:
            if s1[i - 1] == s2[j - 1]:
                dp[i * (n + 1) + j] = dp[(i - 1) * (n + 1) + (j - 1)]
            else:
                v1: int = dp[(i - 1) * (n + 1) + j] + 1
                v2: int = dp[i * (n + 1) + (j - 1)] + 1
                v3: int = dp[(i - 1) * (n + 1) + (j - 1)] + 1
                best: int = v1
                if v2 < best:
                    best = v2
                if v3 < best:
                    best = v3
                dp[i * (n + 1) + j] = best
            j = j + 1
        i = i + 1
    return dp[m * (n + 1) + n]


def test_module() -> int:
    passed: int = 0

    if lcs_length("abcde", "ace") == 3:
        passed = passed + 1

    if lcs_length("abc", "def") == 0:
        passed = passed + 1

    if lcs_length("", "abc") == 0:
        passed = passed + 1

    lcs1: str = lcs_string("abcde", "ace")
    if lcs1 == "ace":
        passed = passed + 1

    if edit_distance("kitten", "sitting") == 3:
        passed = passed + 1

    if edit_distance("", "abc") == 3:
        passed = passed + 1

    if edit_distance("abc", "abc") == 0:
        passed = passed + 1

    return passed
