"""Palindrome checks and longest palindromic substring."""


def is_palindrome(s: str) -> int:
    """Check if string is a palindrome. Returns 1 or 0."""
    left: int = 0
    right: int = len(s) - 1
    while left < right:
        if s[left] != s[right]:
            return 0
        left = left + 1
        right = right - 1
    return 1


def expand_around_center(s: str, left: int, right: int) -> int:
    """Expand from center and return length of palindrome."""
    while left >= 0 and right < len(s) and s[left] == s[right]:
        left = left - 1
        right = right + 1
    return right - left - 1


def longest_palindrome_length(s: str) -> int:
    """Find length of longest palindromic substring."""
    if len(s) == 0:
        return 0
    best: int = 1
    i: int = 0
    while i < len(s):
        odd_len: int = expand_around_center(s, i, i)
        even_len: int = expand_around_center(s, i, i + 1)
        curr: int = odd_len
        if even_len > curr:
            curr = even_len
        if curr > best:
            best = curr
        i = i + 1
    return best


def count_palindromic_substrings(s: str) -> int:
    """Count total number of palindromic substrings."""
    count: int = 0
    i: int = 0
    while i < len(s):
        # Odd length palindromes centered at i
        left: int = i
        right: int = i
        while left >= 0 and right < len(s) and s[left] == s[right]:
            count = count + 1
            left = left - 1
            right = right + 1
        # Even length palindromes centered between i and i+1
        left = i
        right = i + 1
        while left >= 0 and right < len(s) and s[left] == s[right]:
            count = count + 1
            left = left - 1
            right = right + 1
        i = i + 1
    return count


def test_module() -> int:
    passed: int = 0

    if is_palindrome("racecar") == 1:
        passed = passed + 1

    if is_palindrome("hello") == 0:
        passed = passed + 1

    if is_palindrome("") == 1:
        passed = passed + 1

    if longest_palindrome_length("babad") == 3:
        passed = passed + 1

    if longest_palindrome_length("cbbd") == 2:
        passed = passed + 1

    if count_palindromic_substrings("aaa") == 6:
        passed = passed + 1

    if count_palindromic_substrings("abc") == 3:
        passed = passed + 1

    return passed
