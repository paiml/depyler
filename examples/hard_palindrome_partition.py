"""Palindrome partitioning DP - minimum cuts to partition into palindromes."""


def is_palindrome_range(s: list[int], start: int, end: int) -> int:
    """Check if s[start..end] inclusive is a palindrome. Returns 1 if yes."""
    l_pos: int = start
    r_pos: int = end
    while l_pos < r_pos:
        if s[l_pos] != s[r_pos]:
            return 0
        l_pos = l_pos + 1
        r_pos = r_pos - 1
    return 1


def min_palindrome_cuts(s: list[int]) -> int:
    """Minimum cuts needed to partition s into palindromes."""
    n: int = len(s)
    if n <= 1:
        return 0
    pal: list[int] = []
    i: int = 0
    while i < n * n:
        pal.append(0)
        i = i + 1
    i = 0
    while i < n:
        pal[i * n + i] = 1
        i = i + 1
    gap: int = 1
    while gap < n:
        i = 0
        while i < n - gap:
            j: int = i + gap
            if s[i] == s[j]:
                if gap == 1:
                    pal[i * n + j] = 1
                elif pal[(i + 1) * n + (j - 1)] == 1:
                    pal[i * n + j] = 1
            i = i + 1
        gap = gap + 1
    cuts: list[int] = []
    i = 0
    while i < n:
        cuts.append(n)
        i = i + 1
    i = 0
    while i < n:
        if pal[0 * n + i] == 1:
            cuts[i] = 0
        else:
            j2: int = 0
            while j2 < i:
                if pal[(j2 + 1) * n + i] == 1:
                    candidate: int = cuts[j2] + 1
                    if candidate < cuts[i]:
                        cuts[i] = candidate
                j2 = j2 + 1
        i = i + 1
    return cuts[n - 1]


def count_palindrome_substrings(s: list[int]) -> int:
    """Count all palindromic substrings."""
    n: int = len(s)
    count: int = 0
    i: int = 0
    while i < n:
        j: int = i
        while j < n:
            if is_palindrome_range(s, i, j) == 1:
                count = count + 1
            j = j + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test palindrome partitioning."""
    passed: int = 0

    s1: list[int] = [1, 2, 1]
    if min_palindrome_cuts(s1) == 0:
        passed = passed + 1

    s2: list[int] = [1, 2, 3]
    if min_palindrome_cuts(s2) == 2:
        passed = passed + 1

    s3: list[int] = [1, 2, 2, 1]
    if min_palindrome_cuts(s3) == 0:
        passed = passed + 1

    s4: list[int] = [1, 2, 1, 3, 1]
    if min_palindrome_cuts(s4) == 1:
        passed = passed + 1

    if count_palindrome_substrings([1, 2, 1]) == 4:
        passed = passed + 1

    empty: list[int] = []
    if min_palindrome_cuts(empty) == 0:
        passed = passed + 1

    return passed
