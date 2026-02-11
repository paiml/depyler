"""Naive string search and pattern counting."""


def naive_search(text: str, pattern: str) -> int:
    """Find first occurrence of pattern in text. Returns index or -1."""
    n: int = len(text)
    m: int = len(pattern)
    if m == 0:
        return 0
    if m > n:
        return -1
    i: int = 0
    while i <= n - m:
        j: int = 0
        while j < m and text[i + j] == pattern[j]:
            j = j + 1
        if j == m:
            return i
        i = i + 1
    return -1


def count_pattern(text: str, pattern: str) -> int:
    """Count non-overlapping occurrences of pattern in text."""
    n: int = len(text)
    m: int = len(pattern)
    if m == 0:
        return 0
    count: int = 0
    i: int = 0
    while i <= n - m:
        j: int = 0
        while j < m and text[i + j] == pattern[j]:
            j = j + 1
        if j == m:
            count = count + 1
            i = i + m
        else:
            i = i + 1
    return count


def count_overlapping(text: str, pattern: str) -> int:
    """Count overlapping occurrences of pattern in text."""
    n: int = len(text)
    m: int = len(pattern)
    if m == 0:
        return 0
    count: int = 0
    i: int = 0
    while i <= n - m:
        j: int = 0
        while j < m and text[i + j] == pattern[j]:
            j = j + 1
        if j == m:
            count = count + 1
        i = i + 1
    return count


def starts_with(text: str, prefix: str) -> int:
    """Check if text starts with prefix. Returns 1 or 0."""
    if len(prefix) > len(text):
        return 0
    i: int = 0
    while i < len(prefix):
        if text[i] != prefix[i]:
            return 0
        i = i + 1
    return 1


def test_module() -> int:
    passed: int = 0

    if naive_search("hello world", "world") == 6:
        passed = passed + 1

    if naive_search("abcdef", "xyz") == -1:
        passed = passed + 1

    if count_pattern("abababab", "ab") == 4:
        passed = passed + 1

    if count_overlapping("aaaa", "aa") == 3:
        passed = passed + 1

    if starts_with("hello", "hel") == 1:
        passed = passed + 1

    if starts_with("hello", "world") == 0:
        passed = passed + 1

    if naive_search("", "a") == -1:
        passed = passed + 1

    if count_pattern("aaa", "b") == 0:
        passed = passed + 1

    return passed
