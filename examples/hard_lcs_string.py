"""Longest common substring using DP."""


def longest_common_substring(a: str, b: str) -> int:
    """Find length of longest common substring between a and b."""
    m: int = len(a)
    n: int = len(b)
    if m == 0 or n == 0:
        return 0
    prev_row: list[int] = []
    j: int = 0
    while j <= n:
        prev_row.append(0)
        j = j + 1
    best: int = 0
    i: int = 1
    while i <= m:
        curr_row: list[int] = []
        k: int = 0
        while k <= n:
            curr_row.append(0)
            k = k + 1
        j2: int = 1
        while j2 <= n:
            if a[i - 1] == b[j2 - 1]:
                curr_row[j2] = prev_row[j2 - 1] + 1
                if curr_row[j2] > best:
                    best = curr_row[j2]
            else:
                curr_row[j2] = 0
            j2 = j2 + 1
        prev_row = curr_row
        i = i + 1
    return best


def longest_common_subsequence(a: str, b: str) -> int:
    """Find length of longest common subsequence (not necessarily contiguous)."""
    m: int = len(a)
    n: int = len(b)
    if m == 0 or n == 0:
        return 0
    prev_row: list[int] = []
    j: int = 0
    while j <= n:
        prev_row.append(0)
        j = j + 1
    i: int = 1
    while i <= m:
        curr_row: list[int] = []
        k: int = 0
        while k <= n:
            curr_row.append(0)
            k = k + 1
        j2: int = 1
        while j2 <= n:
            if a[i - 1] == b[j2 - 1]:
                curr_row[j2] = prev_row[j2 - 1] + 1
            else:
                val_a: int = prev_row[j2]
                val_b: int = curr_row[j2 - 1]
                if val_a > val_b:
                    curr_row[j2] = val_a
                else:
                    curr_row[j2] = val_b
            j2 = j2 + 1
        prev_row = curr_row
        i = i + 1
    return prev_row[n]


def edit_distance(a: str, b: str) -> int:
    """Compute Levenshtein edit distance between two strings."""
    m: int = len(a)
    n: int = len(b)
    prev_row: list[int] = []
    j: int = 0
    while j <= n:
        prev_row.append(j)
        j = j + 1
    i: int = 1
    while i <= m:
        curr_row: list[int] = []
        k: int = 0
        while k <= n:
            curr_row.append(0)
            k = k + 1
        curr_row[0] = i
        j2: int = 1
        while j2 <= n:
            if a[i - 1] == b[j2 - 1]:
                curr_row[j2] = prev_row[j2 - 1]
            else:
                op1: int = prev_row[j2 - 1] + 1
                op2: int = prev_row[j2] + 1
                op3: int = curr_row[j2 - 1] + 1
                best: int = op1
                if op2 < best:
                    best = op2
                if op3 < best:
                    best = op3
                curr_row[j2] = best
            j2 = j2 + 1
        prev_row = curr_row
        i = i + 1
    return prev_row[n]


def test_module() -> int:
    passed: int = 0

    if longest_common_substring("abcdef", "zbcdf") == 3:
        passed = passed + 1

    if longest_common_substring("abc", "xyz") == 0:
        passed = passed + 1

    if longest_common_subsequence("abcde", "ace") == 3:
        passed = passed + 1

    if longest_common_subsequence("abc", "abc") == 3:
        passed = passed + 1

    if edit_distance("kitten", "sitting") == 3:
        passed = passed + 1

    if edit_distance("", "abc") == 3:
        passed = passed + 1

    if edit_distance("abc", "abc") == 0:
        passed = passed + 1

    if longest_common_substring("abcdxyz", "xyzabcd") == 4:
        passed = passed + 1

    return passed
