"""Levenshtein edit distance between two strings.

Tests: identical strings, empty strings, single char ops, general cases, symmetry.
"""


def edit_distance(s1: str, s2: str) -> int:
    """Return minimum edit distance (insert, delete, replace) between s1 and s2."""
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
    i = 0
    while i <= m:
        dp[i][0] = i
        i = i + 1
    j: int = 0
    while j <= n:
        dp[0][j] = j
        j = j + 1
    i = 1
    while i <= m:
        j = 1
        while j <= n:
            if s1[i - 1] == s2[j - 1]:
                dp[i][j] = dp[i - 1][j - 1]
            else:
                del_cost: int = dp[i - 1][j] + 1
                ins_cost: int = dp[i][j - 1] + 1
                rep_cost: int = dp[i - 1][j - 1] + 1
                best: int = del_cost
                if ins_cost < best:
                    best = ins_cost
                if rep_cost < best:
                    best = rep_cost
                dp[i][j] = best
            j = j + 1
        i = i + 1
    return dp[m][n]


def is_one_edit_away(s1: str, s2: str) -> int:
    """Return 1 if strings are at most one edit apart, 0 otherwise."""
    d: int = edit_distance(s1, s2)
    if d <= 1:
        return 1
    return 0


def hamming_distance(s1: str, s2: str) -> int:
    """Return number of positions where characters differ (same length strings)."""
    if len(s1) != len(s2):
        return -1
    count: int = 0
    i: int = 0
    while i < len(s1):
        if s1[i] != s2[i]:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test edit distance algorithms."""
    ok: int = 0

    if edit_distance("kitten", "sitting") == 3:
        ok = ok + 1
    if edit_distance("", "") == 0:
        ok = ok + 1
    if edit_distance("abc", "") == 3:
        ok = ok + 1
    if edit_distance("", "xyz") == 3:
        ok = ok + 1
    if edit_distance("abc", "abc") == 0:
        ok = ok + 1
    if edit_distance("ab", "ba") == 2:
        ok = ok + 1

    if edit_distance("sunday", "saturday") == 3:
        ok = ok + 1

    if is_one_edit_away("cat", "cats") == 1:
        ok = ok + 1
    if is_one_edit_away("cat", "dog") == 0:
        ok = ok + 1

    if hamming_distance("karolin", "kathrin") == 3:
        ok = ok + 1

    return ok
