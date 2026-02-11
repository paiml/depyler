"""Substring search patterns: naive search, KMP-like manual, and variations."""


def naive_find(text: str, needle: str) -> int:
    """Find first occurrence of needle in text, return -1 if not found."""
    n: int = len(text)
    m: int = len(needle)
    if m == 0:
        return 0
    if m > n:
        return -1
    i: int = 0
    while i <= n - m:
        found: int = 1
        j: int = 0
        while j < m:
            tc: str = text[i + j]
            nc: str = needle[j]
            if tc != nc:
                found = 0
                j = m
            else:
                j = j + 1
        if found == 1:
            return i
        i = i + 1
    return -1


def count_occurrences(text: str, needle: str) -> int:
    """Count non-overlapping occurrences of needle in text."""
    n: int = len(text)
    m: int = len(needle)
    if m == 0:
        return 0
    count: int = 0
    i: int = 0
    while i <= n - m:
        found: int = 1
        j: int = 0
        while j < m:
            tc: str = text[i + j]
            nc: str = needle[j]
            if tc != nc:
                found = 0
                j = m
            else:
                j = j + 1
        if found == 1:
            count = count + 1
            i = i + m
        else:
            i = i + 1
    return count


def find_all_positions(text: str, needle: str) -> list[int]:
    """Find all starting positions of needle in text (overlapping)."""
    positions: list[int] = []
    n: int = len(text)
    m: int = len(needle)
    if m == 0 or m > n:
        return positions
    i: int = 0
    while i <= n - m:
        found: int = 1
        j: int = 0
        while j < m:
            tc: str = text[i + j]
            nc: str = needle[j]
            if tc != nc:
                found = 0
                j = m
            else:
                j = j + 1
        if found == 1:
            positions.append(i)
        i = i + 1
    return positions


def kmp_build_table(needle: str) -> list[int]:
    """Build KMP partial match table."""
    m: int = len(needle)
    if m == 0:
        return []
    table: list[int] = []
    i: int = 0
    while i < m:
        table.append(0)
        i = i + 1
    length: int = 0
    i = 1
    while i < m:
        ci: str = needle[i]
        cl: str = needle[length]
        if ci == cl:
            length = length + 1
            table[i] = length
            i = i + 1
        elif length != 0:
            length = table[length - 1]
        else:
            table[i] = 0
            i = i + 1
    return table


def kmp_find(text: str, needle: str) -> int:
    """KMP search for first occurrence."""
    n: int = len(text)
    m: int = len(needle)
    if m == 0:
        return 0
    if m > n:
        return -1
    table: list[int] = kmp_build_table(needle)
    ti: int = 0
    ni: int = 0
    while ti < n:
        tc: str = text[ti]
        nc: str = needle[ni]
        if tc == nc:
            ti = ti + 1
            ni = ni + 1
            if ni == m:
                return ti - m
        elif ni != 0:
            ni = table[ni - 1]
        else:
            ti = ti + 1
    return -1


def starts_with(text: str, prefix: str) -> int:
    """Return 1 if text starts with prefix."""
    if len(prefix) > len(text):
        return 0
    i: int = 0
    while i < len(prefix):
        tc: str = text[i]
        pc: str = prefix[i]
        if tc != pc:
            return 0
        i = i + 1
    return 1


def ends_with(text: str, suffix: str) -> int:
    """Return 1 if text ends with suffix."""
    tlen: int = len(text)
    slen: int = len(suffix)
    if slen > tlen:
        return 0
    offset: int = tlen - slen
    i: int = 0
    while i < slen:
        tc: str = text[offset + i]
        sc: str = suffix[i]
        if tc != sc:
            return 0
        i = i + 1
    return 1


def test_module() -> int:
    """Test all string search functions."""
    passed: int = 0
    if naive_find("hello world", "world") == 6:
        passed = passed + 1
    if naive_find("hello", "xyz") == -1:
        passed = passed + 1
    if naive_find("aaa", "a") == 0:
        passed = passed + 1
    if count_occurrences("ababab", "ab") == 3:
        passed = passed + 1
    if count_occurrences("aaaa", "aa") == 2:
        passed = passed + 1
    positions: list[int] = find_all_positions("abab", "ab")
    if len(positions) == 2:
        passed = passed + 1
    if kmp_find("abcabcabd", "abcabd") == 3:
        passed = passed + 1
    if kmp_find("aaaaa", "bbb") == -1:
        passed = passed + 1
    if starts_with("hello", "hel") == 1:
        passed = passed + 1
    if starts_with("hello", "xyz") == 0:
        passed = passed + 1
    if ends_with("hello", "llo") == 1:
        passed = passed + 1
    if ends_with("hello", "xyz") == 0:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
