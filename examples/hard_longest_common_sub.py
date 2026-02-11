"""Longest common subsequence using dynamic programming."""


def lcs_length(a: list[int], b: list[int]) -> int:
    """Compute length of longest common subsequence."""
    m: int = len(a)
    n: int = len(b)
    dp: list[int] = []
    i: int = 0
    while i < (m + 1) * (n + 1):
        dp.append(0)
        i = i + 1
    i = 1
    while i <= m:
        j: int = 1
        while j <= n:
            if a[i - 1] == b[j - 1]:
                dp[i * (n + 1) + j] = dp[(i - 1) * (n + 1) + (j - 1)] + 1
            else:
                val_up: int = dp[(i - 1) * (n + 1) + j]
                val_left: int = dp[i * (n + 1) + (j - 1)]
                if val_up > val_left:
                    dp[i * (n + 1) + j] = val_up
                else:
                    dp[i * (n + 1) + j] = val_left
            j = j + 1
        i = i + 1
    return dp[m * (n + 1) + n]


def lcs_reconstruct(a: list[int], b: list[int]) -> list[int]:
    """Reconstruct the LCS itself."""
    m: int = len(a)
    n: int = len(b)
    dp: list[int] = []
    i: int = 0
    while i < (m + 1) * (n + 1):
        dp.append(0)
        i = i + 1
    i = 1
    while i <= m:
        j: int = 1
        while j <= n:
            if a[i - 1] == b[j - 1]:
                dp[i * (n + 1) + j] = dp[(i - 1) * (n + 1) + (j - 1)] + 1
            else:
                val_up: int = dp[(i - 1) * (n + 1) + j]
                val_left: int = dp[i * (n + 1) + (j - 1)]
                if val_up > val_left:
                    dp[i * (n + 1) + j] = val_up
                else:
                    dp[i * (n + 1) + j] = val_left
            j = j + 1
        i = i + 1
    result: list[int] = []
    i = m
    j2: int = n
    while i > 0 and j2 > 0:
        if a[i - 1] == b[j2 - 1]:
            result.append(a[i - 1])
            i = i - 1
            j2 = j2 - 1
        elif dp[(i - 1) * (n + 1) + j2] > dp[i * (n + 1) + (j2 - 1)]:
            i = i - 1
        else:
            j2 = j2 - 1
    rev: list[int] = []
    k: int = len(result) - 1
    while k >= 0:
        rev.append(result[k])
        k = k - 1
    return rev


def test_module() -> int:
    """Test LCS."""
    passed: int = 0

    a1: list[int] = [1, 3, 4, 1, 2]
    b1: list[int] = [3, 4, 1, 2, 1, 3]
    if lcs_length(a1, b1) == 4:
        passed = passed + 1

    a2: list[int] = [1, 2, 3]
    b2: list[int] = [4, 5, 6]
    if lcs_length(a2, b2) == 0:
        passed = passed + 1

    a3: list[int] = [1, 2, 3]
    b3: list[int] = [1, 2, 3]
    if lcs_length(a3, b3) == 3:
        passed = passed + 1

    lcs: list[int] = lcs_reconstruct(a3, b3)
    if len(lcs) == 3 and lcs[0] == 1 and lcs[1] == 2:
        passed = passed + 1

    empty: list[int] = []
    if lcs_length(empty, a3) == 0:
        passed = passed + 1

    a4: list[int] = [1, 2, 1, 2]
    b4: list[int] = [2, 1, 2, 1]
    if lcs_length(a4, b4) == 3:
        passed = passed + 1

    return passed
