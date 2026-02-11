"""KMP pattern search returning all match positions."""


def compute_lps(pattern: str) -> list[int]:
    """Compute longest proper prefix which is also suffix array."""
    m: int = len(pattern)
    lps: list[int] = []
    i: int = 0
    while i < m:
        lps.append(0)
        i = i + 1
    if m == 0:
        return lps
    length: int = 0
    i = 1
    while i < m:
        if ord(pattern[i]) == ord(pattern[length]):
            length = length + 1
            lps[i] = length
            i = i + 1
        else:
            if length != 0:
                length = lps[length - 1]
            else:
                lps[i] = 0
                i = i + 1
    return lps


def kmp_find_all(text: str, pattern: str) -> list[int]:
    """Find all occurrences of pattern in text. Returns list of positions."""
    n: int = len(text)
    m: int = len(pattern)
    positions: list[int] = []
    if m == 0:
        return positions
    lps: list[int] = compute_lps(pattern)
    i: int = 0
    j: int = 0
    while i < n:
        if ord(text[i]) == ord(pattern[j]):
            i = i + 1
            j = j + 1
        if j == m:
            pos: int = i - j
            positions.append(pos)
            j = lps[j - 1]
        elif i < n and ord(text[i]) != ord(pattern[j]):
            if j != 0:
                j = lps[j - 1]
            else:
                i = i + 1
    return positions


def kmp_first(text: str, pattern: str) -> int:
    """Find first occurrence of pattern in text. Returns -1 if not found."""
    positions: list[int] = kmp_find_all(text, pattern)
    if len(positions) == 0:
        return 0 - 1
    return positions[0]


def kmp_count_matches(text: str, pattern: str) -> int:
    """Count all occurrences of pattern in text."""
    positions: list[int] = kmp_find_all(text, pattern)
    return len(positions)


def test_module() -> int:
    """Test KMP string search."""
    passed: int = 0

    pos1: list[int] = kmp_find_all("ababababab", "abab")
    if len(pos1) == 2:
        passed = passed + 1

    if kmp_first("hello world", "world") == 6:
        passed = passed + 1

    if kmp_first("hello", "xyz") == 0 - 1:
        passed = passed + 1

    if kmp_count_matches("aaaaaa", "aa") == 3:
        passed = passed + 1

    lps: list[int] = compute_lps("abcabc")
    if lps[3] == 1 and lps[4] == 2 and lps[5] == 3:
        passed = passed + 1

    if kmp_count_matches("abcabc", "abc") == 2:
        passed = passed + 1

    return passed
