"""Character frequency analysis.

Tests: count specific chars, uppercase count, lowercase count, digit count, non-alpha count.
"""


def count_char_occurrences(text: str, ch: str) -> int:
    """Count occurrences of ch in text using index comparison."""
    count: int = 0
    i: int = 0
    while i < len(text):
        if text[i] == ch[0]:
            count = count + 1
        i = i + 1
    return count


def count_uppercase_letters(text: str) -> int:
    """Count uppercase letters A-Z."""
    count: int = 0
    i: int = 0
    while i < len(text):
        if text[i] >= "A" and text[i] <= "Z":
            count = count + 1
        i = i + 1
    return count


def count_lowercase_letters(text: str) -> int:
    """Count lowercase letters a-z."""
    count: int = 0
    i: int = 0
    while i < len(text):
        if text[i] >= "a" and text[i] <= "z":
            count = count + 1
        i = i + 1
    return count


def count_digit_chars(text: str) -> int:
    """Count digit characters 0-9."""
    count: int = 0
    i: int = 0
    while i < len(text):
        if text[i] >= "0" and text[i] <= "9":
            count = count + 1
        i = i + 1
    return count


def total_alpha_count(text: str) -> int:
    """Count all alphabetic characters."""
    return count_uppercase_letters(text) + count_lowercase_letters(text)


def test_module() -> int:
    """Test character frequency."""
    ok: int = 0
    if count_uppercase_letters("Hello World") == 2:
        ok = ok + 1
    if count_lowercase_letters("Hello World") == 8:
        ok = ok + 1
    if count_digit_chars("abc123def456") == 6:
        ok = ok + 1
    if total_alpha_count("Hi 123!") == 2:
        ok = ok + 1
    if count_lowercase_letters("ABC") == 0:
        ok = ok + 1
    return ok
