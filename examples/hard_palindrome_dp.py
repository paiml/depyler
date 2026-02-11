"""Longest palindromic subsequence using dynamic programming.

Tests: palindrome length, single char, all same chars, no palindrome, general cases.
"""


def longest_palindrome_subseq(s: str) -> int:
    """Return length of longest palindromic subsequence in s."""
    n: int = len(s)
    if n == 0:
        return 0
    dp: list[list[int]] = []
    i: int = 0
    while i < n:
        row: list[int] = []
        j: int = 0
        while j < n:
            row.append(0)
            j = j + 1
        dp.append(row)
        i = i + 1
    i = 0
    while i < n:
        dp[i][i] = 1
        i = i + 1
    cl: int = 2
    while cl <= n:
        i = 0
        while i <= n - cl:
            j: int = i + cl - 1
            if s[i] == s[j] and cl == 2:
                dp[i][j] = 2
            elif s[i] == s[j]:
                dp[i][j] = dp[i + 1][j - 1] + 2
            else:
                a: int = dp[i + 1][j]
                b: int = dp[i][j - 1]
                if a > b:
                    dp[i][j] = a
                else:
                    dp[i][j] = b
            i = i + 1
        cl = cl + 1
    return dp[0][n - 1]


def min_insertions_palindrome(s: str) -> int:
    """Return minimum insertions to make s a palindrome."""
    return len(s) - longest_palindrome_subseq(s)


def is_palindrome(s: str) -> int:
    """Return 1 if s is a palindrome, 0 otherwise."""
    left: int = 0
    right: int = len(s) - 1
    while left < right:
        if s[left] != s[right]:
            return 0
        left = left + 1
        right = right - 1
    return 1


def count_palindrome_substrings(s: str) -> int:
    """Return count of palindromic substrings."""
    n: int = len(s)
    count: int = 0
    i: int = 0
    while i < n:
        left: int = i
        right: int = i
        while left >= 0 and right < n and s[left] == s[right]:
            count = count + 1
            left = left - 1
            right = right + 1
        left = i
        right = i + 1
        while left >= 0 and right < n and s[left] == s[right]:
            count = count + 1
            left = left - 1
            right = right + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test palindrome DP algorithms."""
    ok: int = 0

    if longest_palindrome_subseq("bbbab") == 4:
        ok = ok + 1
    if longest_palindrome_subseq("cbbd") == 2:
        ok = ok + 1
    if longest_palindrome_subseq("a") == 1:
        ok = ok + 1
    if longest_palindrome_subseq("") == 0:
        ok = ok + 1
    if longest_palindrome_subseq("aaaa") == 4:
        ok = ok + 1

    if min_insertions_palindrome("abc") == 2:
        ok = ok + 1
    if min_insertions_palindrome("aba") == 0:
        ok = ok + 1

    if is_palindrome("racecar") == 1:
        ok = ok + 1
    if is_palindrome("hello") == 0:
        ok = ok + 1

    if count_palindrome_substrings("aab") == 4:
        ok = ok + 1

    return ok
