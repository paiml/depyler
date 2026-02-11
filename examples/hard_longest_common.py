"""Longest common subsequence and substring.

Tests: LCS length, subsequence check, longest increasing subsequence.
"""


def lcs_length(s1: str, s2: str) -> int:
    """Length of longest common subsequence."""
    m: int = len(s1)
    n: int = len(s2)
    prev: list[int] = [0] * (n + 1)
    i: int = 1
    while i <= m:
        curr: list[int] = [0] * (n + 1)
        j: int = 1
        while j <= n:
            if s1[i - 1] == s2[j - 1]:
                curr[j] = prev[j - 1] + 1
            else:
                a: int = prev[j]
                b: int = curr[j - 1]
                if a > b:
                    curr[j] = a
                else:
                    curr[j] = b
            j = j + 1
        prev = curr
        i = i + 1
    return prev[n]


def longest_common_substring_len(s1: str, s2: str) -> int:
    """Length of longest common substring."""
    m: int = len(s1)
    n: int = len(s2)
    best: int = 0
    prev: list[int] = [0] * (n + 1)
    i: int = 1
    while i <= m:
        curr: list[int] = [0] * (n + 1)
        j: int = 1
        while j <= n:
            if s1[i - 1] == s2[j - 1]:
                curr[j] = prev[j - 1] + 1
                if curr[j] > best:
                    best = curr[j]
            j = j + 1
        prev = curr
        i = i + 1
    return best


def is_subsequence_val(sub: str, main: str) -> int:
    """Check if sub is a subsequence of main. Returns 1 if yes, 0 if no."""
    si: int = 0
    mi: int = 0
    while si < len(sub) and mi < len(main):
        if sub[si] == main[mi]:
            si = si + 1
        mi = mi + 1
    if si == len(sub):
        return 1
    return 0


def longest_increasing_subseq_len(arr: list[int]) -> int:
    """Length of longest increasing subsequence using patience sorting."""
    n: int = len(arr)
    if n == 0:
        return 0
    tails: list[int] = []
    i: int = 0
    while i < n:
        pos: int = 0
        found: int = 0
        j: int = 0
        while j < len(tails):
            if tails[j] >= arr[i]:
                pos = j
                found = 1
                j = len(tails)
            else:
                j = j + 1
        if found == 1:
            tails[pos] = arr[i]
        else:
            tails.append(arr[i])
        i = i + 1
    return len(tails)


def test_module() -> None:
    assert lcs_length("abcde", "ace") == 3
    assert lcs_length("abc", "def") == 0
    assert lcs_length("abc", "abc") == 3
    assert longest_common_substring_len("abcdef", "zbcdf") == 3
    assert longest_common_substring_len("abc", "xyz") == 0
    assert is_subsequence_val("ace", "abcde") == 1
    assert is_subsequence_val("aec", "abcde") == 0
    assert is_subsequence_val("", "abc") == 1
    assert longest_increasing_subseq_len([10, 9, 2, 5, 3, 7, 101, 18]) == 4
    assert longest_increasing_subseq_len([0, 1, 0, 3, 2, 3]) == 4
    assert longest_increasing_subseq_len([]) == 0
