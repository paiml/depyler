"""KMP (Knuth-Morris-Pratt) string matching algorithm."""


def build_failure(pattern: list[int]) -> list[int]:
    """Build the failure function (partial match table) for KMP."""
    m: int = len(pattern)
    fail: list[int] = []
    i: int = 0
    while i < m:
        fail.append(0)
        i = i + 1
    if m == 0:
        return fail
    fail[0] = 0
    k: int = 0
    i = 1
    while i < m:
        while k > 0 and pattern[k] != pattern[i]:
            k = fail[k - 1]
        if pattern[k] == pattern[i]:
            k = k + 1
        fail[i] = k
        i = i + 1
    return fail


def kmp_search(text: list[int], pattern: list[int]) -> int:
    """Find first occurrence of pattern in text. Returns index or -1."""
    n: int = len(text)
    m: int = len(pattern)
    if m == 0:
        return 0
    if n < m:
        return 0 - 1
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
    return 0 - 1


def kmp_count(text: list[int], pattern: list[int]) -> int:
    """Count non-overlapping occurrences of pattern in text."""
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
            j = fail[j - 1]
        i = i + 1
    return count


def str_to_codes(s: str) -> list[int]:
    """Convert string to list of char codes."""
    result: list[int] = []
    i: int = 0
    n: int = len(s)
    while i < n:
        result.append(ord(s[i]))
        i = i + 1
    return result


def test_module() -> int:
    """Test KMP algorithm."""
    passed: int = 0

    text: list[int] = [1, 2, 3, 1, 2, 3, 4]
    pat: list[int] = [1, 2, 3, 4]
    if kmp_search(text, pat) == 3:
        passed = passed + 1

    pat2: list[int] = [5, 6]
    if kmp_search(text, pat2) == 0 - 1:
        passed = passed + 1

    text2: list[int] = [1, 1, 1, 1, 1]
    pat3: list[int] = [1, 1]
    if kmp_count(text2, pat3) == 2:
        passed = passed + 1

    empty: list[int] = []
    if kmp_search(empty, pat) == 0 - 1:
        passed = passed + 1

    fail: list[int] = build_failure([1, 2, 1, 2, 3])
    if fail[0] == 0 and fail[2] == 1 and fail[3] == 2:
        passed = passed + 1

    pat4: list[int] = [1, 2]
    text3: list[int] = [1, 2, 3, 1, 2, 3, 1, 2]
    if kmp_count(text3, pat4) == 3:
        passed = passed + 1

    return passed
