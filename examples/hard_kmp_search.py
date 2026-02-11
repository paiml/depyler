"""KMP pattern matching: failure function and search."""


def build_failure(pattern: str) -> list[int]:
    """Build KMP failure (partial match) table."""
    m: int = len(pattern)
    fail: list[int] = []
    idx: int = 0
    while idx < m:
        fail.append(0)
        idx = idx + 1
    if m == 0:
        return fail
    fail[0] = 0
    length: int = 0
    i: int = 1
    while i < m:
        while length > 0 and pattern[i] != pattern[length]:
            length = fail[length - 1]
        if pattern[i] == pattern[length]:
            length = length + 1
        fail[i] = length
        i = i + 1
    return fail


def kmp_search(text: str, pattern: str) -> int:
    """Find first occurrence of pattern in text. Returns index or -1."""
    n: int = len(text)
    m: int = len(pattern)
    if m == 0:
        return 0
    if n < m:
        return -1
    fail: list[int] = build_failure(pattern)
    j: int = 0
    i: int = 0
    while i < n:
        while j > 0 and text[i] != pattern[j]:
            j = fail[j - 1]
        if text[i] == pattern[j]:
            j = j + 1
        if j == m:
            return i - m + 1
        i = i + 1
    return -1


def kmp_count(text: str, pattern: str) -> int:
    """Count all non-overlapping occurrences of pattern in text."""
    n: int = len(text)
    m: int = len(pattern)
    if m == 0:
        return 0
    fail: list[int] = build_failure(pattern)
    count: int = 0
    j: int = 0
    i: int = 0
    while i < n:
        while j > 0 and text[i] != pattern[j]:
            j = fail[j - 1]
        if text[i] == pattern[j]:
            j = j + 1
        if j == m:
            count = count + 1
            j = 0
        i = i + 1
    return count


def test_module() -> int:
    passed: int = 0

    if kmp_search("ABABDABACDABABCABAB", "ABABCABAB") == 10:
        passed = passed + 1

    if kmp_search("hello world", "world") == 6:
        passed = passed + 1

    if kmp_search("abcdef", "xyz") == -1:
        passed = passed + 1

    if kmp_count("aaaa", "aa") == 2:
        passed = passed + 1

    if kmp_count("abcabcabc", "abc") == 3:
        passed = passed + 1

    fail_test: list[int] = build_failure("ABAB")
    if fail_test[0] == 0 and fail_test[2] == 1 and fail_test[3] == 2:
        passed = passed + 1

    if kmp_search("", "abc") == -1:
        passed = passed + 1

    return passed
