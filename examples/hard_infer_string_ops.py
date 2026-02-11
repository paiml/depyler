# Type inference test: String manipulations
# Strategy: Only test_module() -> int annotated, everything else inferred
# NOTE: String params NEED annotations for the transpiler
# FINDING: Single-char comparison s[i] == ch fails when ch is str param


def str_length(s: str) -> int:
    """Return length of string."""
    return len(s)


def str_is_palindrome(s: str) -> int:
    """Check if string is palindrome. Returns 1 if yes, 0 if no."""
    n: int = len(s)
    i: int = 0
    while i < n // 2:
        if s[i] != s[n - 1 - i]:
            return 0
        i = i + 1
    return 1


def str_count_a(s: str) -> int:
    """Count occurrences of 'a' in string using literal comparison."""
    count: int = 0
    i: int = 0
    while i < len(s):
        if s[i] == "a":
            count = count + 1
        i = i + 1
    return count


def str_has_prefix(s: str, prefix: str) -> int:
    """Check if s starts with prefix. Returns 1 if yes, 0 if no."""
    if len(prefix) > len(s):
        return 0
    i: int = 0
    while i < len(prefix):
        if s[i] != prefix[i]:
            return 0
        i = i + 1
    return 1


def str_common_prefix_len(s1: str, s2: str) -> int:
    """Return length of common prefix between two strings."""
    max_len: int = len(s1)
    if len(s2) < max_len:
        max_len = len(s2)
    i: int = 0
    while i < max_len:
        if s1[i] != s2[i]:
            return i
        i = i + 1
    return max_len


def str_equals(s1: str, s2: str) -> int:
    """Check if two strings are equal. Returns 1 if yes, 0 if no."""
    if len(s1) != len(s2):
        return 0
    i: int = 0
    while i < len(s1):
        if s1[i] != s2[i]:
            return 0
        i = i + 1
    return 1


def str_starts_with_digit(s: str) -> int:
    """Check if string starts with a digit char (0-9). Returns 1/0."""
    if len(s) == 0:
        return 0
    if s[0] == "0" or s[0] == "1" or s[0] == "2" or s[0] == "3":
        return 1
    if s[0] == "4" or s[0] == "5" or s[0] == "6" or s[0] == "7":
        return 1
    if s[0] == "8" or s[0] == "9":
        return 1
    return 0


def test_module() -> int:
    """Test all string ops inference functions."""
    total: int = 0

    # str_length tests
    if str_length("hello") == 5:
        total = total + 1
    if str_length("") == 0:
        total = total + 1

    # str_count_a tests
    if str_count_a("banana") == 3:
        total = total + 1
    if str_count_a("hello") == 0:
        total = total + 1

    # str_is_palindrome tests
    if str_is_palindrome("racecar") == 1:
        total = total + 1
    if str_is_palindrome("hello") == 0:
        total = total + 1
    if str_is_palindrome("a") == 1:
        total = total + 1

    # str_has_prefix tests
    if str_has_prefix("hello", "hel") == 1:
        total = total + 1
    if str_has_prefix("hello", "xyz") == 0:
        total = total + 1
    if str_has_prefix("hi", "hello") == 0:
        total = total + 1

    # str_common_prefix_len tests
    if str_common_prefix_len("hello", "help") == 3:
        total = total + 1
    if str_common_prefix_len("abc", "xyz") == 0:
        total = total + 1

    # str_equals tests
    if str_equals("hello", "hello") == 1:
        total = total + 1
    if str_equals("hello", "world") == 0:
        total = total + 1

    # str_starts_with_digit tests
    if str_starts_with_digit("3abc") == 1:
        total = total + 1
    if str_starts_with_digit("abc") == 0:
        total = total + 1

    return total
