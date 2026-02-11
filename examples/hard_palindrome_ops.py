"""Palindrome checking and generation.

Tests: string reversal, two-pointer technique, numeric palindromes.
"""


def is_palindrome_str(s: str) -> bool:
    """Check if string is a palindrome."""
    n: int = len(s)
    i: int = 0
    while i < n // 2:
        j: int = n - 1 - i
        if s[i] != s[j]:
            return False
        i += 1
    return True


def is_palindrome_num(n: int) -> bool:
    """Check if number is a palindrome."""
    if n < 0:
        return False
    original: int = n
    reversed_n: int = 0
    while n > 0:
        reversed_n = reversed_n * 10 + n % 10
        n = n // 10
    return original == reversed_n


def longest_palindrome_length(s: str) -> int:
    """Find length of longest palindromic substring (brute force)."""
    n: int = len(s)
    if n == 0:
        return 0
    best: int = 1
    i: int = 0
    while i < n:
        j: int = i + 1
        while j <= n:
            sub: str = s[i:j]
            if is_palindrome_str(sub):
                length: int = j - i
                if length > best:
                    best = length
            j += 1
        i += 1
    return best


def count_palindromic_substrings(s: str) -> int:
    """Count number of palindromic substrings."""
    n: int = len(s)
    count: int = 0
    i: int = 0
    while i < n:
        j: int = i + 1
        while j <= n:
            sub: str = s[i:j]
            if is_palindrome_str(sub):
                count += 1
            j += 1
        i += 1
    return count


def reverse_string(s: str) -> str:
    """Reverse a string."""
    result: str = ""
    i: int = len(s) - 1
    while i >= 0:
        result = result + s[i]
        i -= 1
    return result


def test_module() -> int:
    """Test palindrome operations."""
    ok: int = 0

    if is_palindrome_str("racecar"):
        ok += 1
    if not is_palindrome_str("hello"):
        ok += 1

    if is_palindrome_num(121):
        ok += 1
    if not is_palindrome_num(123):
        ok += 1

    lp: int = longest_palindrome_length("babad")
    if lp == 3:
        ok += 1

    r: str = reverse_string("hello")
    if r == "olleh":
        ok += 1

    return ok
