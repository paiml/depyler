"""Longest palindromic substring using Manacher-like expansion."""


def expand_around_center(s: list[int], left: int, right: int) -> int:
    """Expand around center and return length of palindrome."""
    n: int = len(s)
    l_pos: int = left
    r_pos: int = right
    while l_pos >= 0 and r_pos < n and s[l_pos] == s[r_pos]:
        l_pos = l_pos - 1
        r_pos = r_pos + 1
    return r_pos - l_pos - 1


def longest_palindrome_length(s: list[int]) -> int:
    """Find length of longest palindromic substring."""
    n: int = len(s)
    if n == 0:
        return 0
    best: int = 1
    i: int = 0
    while i < n:
        odd: int = expand_around_center(s, i, i)
        if odd > best:
            best = odd
        even: int = expand_around_center(s, i, i + 1)
        if even > best:
            best = even
        i = i + 1
    return best


def is_palindrome(s: list[int]) -> int:
    """Check if entire list is palindrome. Returns 1 if yes."""
    n: int = len(s)
    i: int = 0
    while i < n // 2:
        if s[i] != s[n - 1 - i]:
            return 0
        i = i + 1
    return 1


def count_palindromic_substrings(s: list[int]) -> int:
    """Count total number of palindromic substrings."""
    n: int = len(s)
    count: int = 0
    i: int = 0
    while i < n:
        odd: int = 0
        k: int = 0
        while i - k >= 0 and i + k < n and s[i - k] == s[i + k]:
            odd = odd + 1
            k = k + 1
        count = count + odd
        even: int = 0
        k = 0
        while i - k >= 0 and i + 1 + k < n and s[i - k] == s[i + 1 + k]:
            even = even + 1
            k = k + 1
        count = count + even
        i = i + 1
    return count


def longest_palindrome_start(s: list[int]) -> int:
    """Find starting index of longest palindromic substring."""
    n: int = len(s)
    if n == 0:
        return 0
    best_len: int = 1
    best_start: int = 0
    i: int = 0
    while i < n:
        odd: int = expand_around_center(s, i, i)
        if odd > best_len:
            best_len = odd
            best_start = i - (odd - 1) // 2
        even: int = expand_around_center(s, i, i + 1)
        if even > best_len:
            best_len = even
            best_start = i - (even - 2) // 2
        i = i + 1
    return best_start


def test_module() -> int:
    """Test palindrome operations."""
    passed: int = 0

    s1: list[int] = [1, 2, 3, 2, 1]
    if longest_palindrome_length(s1) == 5:
        passed = passed + 1

    if is_palindrome(s1) == 1:
        passed = passed + 1

    s2: list[int] = [1, 2, 2, 1, 3]
    if longest_palindrome_length(s2) == 4:
        passed = passed + 1

    s3: list[int] = [1, 2, 3, 4]
    if longest_palindrome_length(s3) == 1:
        passed = passed + 1

    s4: list[int] = [1, 1, 1]
    if count_palindromic_substrings(s4) == 6:
        passed = passed + 1

    empty: list[int] = []
    if longest_palindrome_length(empty) == 0:
        passed = passed + 1

    return passed
