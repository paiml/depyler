"""Longest common subsequence of three strings (as int arrays)."""


def lcs_three(a: list[int], b: list[int], c: list[int]) -> int:
    """LCS length of three sequences represented as int arrays."""
    la: int = len(a)
    lb: int = len(b)
    lc: int = len(c)
    dp: list[int] = []
    total: int = (la + 1) * (lb + 1) * (lc + 1)
    idx: int = 0
    while idx < total:
        dp.append(0)
        idx = idx + 1
    i: int = 1
    while i <= la:
        j: int = 1
        while j <= lb:
            k: int = 1
            while k <= lc:
                flat: int = i * (lb + 1) * (lc + 1) + j * (lc + 1) + k
                if a[i - 1] == b[j - 1] and b[j - 1] == c[k - 1]:
                    prev: int = (i - 1) * (lb + 1) * (lc + 1) + (j - 1) * (lc + 1) + (k - 1)
                    dp[flat] = dp[prev] + 1
                else:
                    v1: int = (i - 1) * (lb + 1) * (lc + 1) + j * (lc + 1) + k
                    v2: int = i * (lb + 1) * (lc + 1) + (j - 1) * (lc + 1) + k
                    v3: int = i * (lb + 1) * (lc + 1) + j * (lc + 1) + (k - 1)
                    best: int = dp[v1]
                    if dp[v2] > best:
                        best = dp[v2]
                    if dp[v3] > best:
                        best = dp[v3]
                    dp[flat] = best
                k = k + 1
            j = j + 1
        i = i + 1
    return dp[la * (lb + 1) * (lc + 1) + lb * (lc + 1) + lc]


def lcs_two(a: list[int], b: list[int]) -> int:
    """LCS length of two sequences."""
    la: int = len(a)
    lb: int = len(b)
    dp: list[int] = []
    total: int = (la + 1) * (lb + 1)
    idx: int = 0
    while idx < total:
        dp.append(0)
        idx = idx + 1
    i: int = 1
    while i <= la:
        j: int = 1
        while j <= lb:
            flat: int = i * (lb + 1) + j
            if a[i - 1] == b[j - 1]:
                dp[flat] = dp[(i - 1) * (lb + 1) + (j - 1)] + 1
            else:
                v1: int = dp[(i - 1) * (lb + 1) + j]
                v2: int = dp[i * (lb + 1) + (j - 1)]
                if v1 > v2:
                    dp[flat] = v1
                else:
                    dp[flat] = v2
            j = j + 1
        i = i + 1
    return dp[la * (lb + 1) + lb]


def test_module() -> int:
    """Test LCS computations."""
    ok: int = 0
    a1: list[int] = [1, 2, 3, 4]
    b1: list[int] = [1, 3, 4, 5]
    c1: list[int] = [1, 2, 4, 6]
    if lcs_three(a1, b1, c1) == 2:
        ok = ok + 1
    a2: list[int] = [1, 2, 3]
    b2: list[int] = [1, 2, 3]
    c2: list[int] = [1, 2, 3]
    if lcs_three(a2, b2, c2) == 3:
        ok = ok + 1
    empty: list[int] = []
    if lcs_three(a2, b2, empty) == 0:
        ok = ok + 1
    if lcs_two(a1, b1) == 3:
        ok = ok + 1
    d1: list[int] = [1, 3, 5]
    d2: list[int] = [2, 4, 6]
    if lcs_two(d1, d2) == 0:
        ok = ok + 1
    if lcs_two(a2, b2) == 3:
        ok = ok + 1
    s1: list[int] = [1]
    s2: list[int] = [1]
    s3: list[int] = [1]
    if lcs_three(s1, s2, s3) == 1:
        ok = ok + 1
    return ok
