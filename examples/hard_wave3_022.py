"""Text processing: String validators and checkers.

Tests: character range checking, pattern matching, escape handling,
bracket balancing, format validation.
"""

from typing import Dict, List, Tuple


def is_palindrome(s: str) -> bool:
    """Check if string is a palindrome."""
    n: int = len(s)
    i: int = 0
    while i < n // 2:
        if s[i] != s[n - 1 - i]:
            return False
        i += 1
    return True


def is_balanced_parens(s: str) -> bool:
    """Check if parentheses are balanced."""
    depth: int = 0
    for ch in s:
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
        if depth < 0:
            return False
    return depth == 0


def is_all_digits(s: str) -> bool:
    """Check if string contains only digit characters."""
    if len(s) == 0:
        return False
    for ch in s:
        if ch < "0" or ch > "9":
            return False
    return True


def is_all_alpha(s: str) -> bool:
    """Check if string contains only alphabetic characters."""
    if len(s) == 0:
        return False
    for ch in s:
        is_lower: bool = ch >= "a" and ch <= "z"
        is_upper: bool = ch >= "A" and ch <= "Z"
        if not is_lower and not is_upper:
            return False
    return True


def is_valid_identifier(s: str) -> bool:
    """Check if string is a valid Python-style identifier."""
    if len(s) == 0:
        return False
    first: str = s[0]
    if first != "_":
        is_lower: bool = first >= "a" and first <= "z"
        is_upper: bool = first >= "A" and first <= "Z"
        if not is_lower and not is_upper:
            return False
    i: int = 1
    while i < len(s):
        ch: str = s[i]
        is_lower2: bool = ch >= "a" and ch <= "z"
        is_upper2: bool = ch >= "A" and ch <= "Z"
        is_digit: bool = ch >= "0" and ch <= "9"
        if not is_lower2 and not is_upper2 and not is_digit and ch != "_":
            return False
        i += 1
    return True


def is_valid_email_simple(s: str) -> bool:
    """Simple email validation: has exactly one @ with text on both sides."""
    at_count: int = 0
    at_pos: int = -1
    i: int = 0
    while i < len(s):
        if s[i] == "@":
            at_count += 1
            at_pos = i
        i += 1
    if at_count != 1:
        return False
    if at_pos == 0 or at_pos == len(s) - 1:
        return False
    return True


def count_vowels(s: str) -> int:
    """Count vowels in a string."""
    count: int = 0
    for ch in s:
        if ch == "a" or ch == "e" or ch == "i" or ch == "o" or ch == "u":
            count += 1
        elif ch == "A" or ch == "E" or ch == "I" or ch == "O" or ch == "U":
            count += 1
    return count


def test_validators() -> bool:
    """Test string validators."""
    ok: bool = True
    if not is_palindrome("racecar"):
        ok = False
    if is_palindrome("hello"):
        ok = False
    if not is_balanced_parens("(())()"):
        ok = False
    if is_balanced_parens("(()"):
        ok = False
    if not is_all_digits("12345"):
        ok = False
    if not is_valid_identifier("hello_world"):
        ok = False
    if is_valid_identifier("123abc"):
        ok = False
    return ok
