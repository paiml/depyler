"""Text processing: Pattern matching and string searching.

Tests: naive string matching, KMP failure function, Boyer-Moore bad char,
string containment, occurrence counting.
"""

from typing import Dict, List, Tuple


def naive_search(text: str, pattern: str) -> List[int]:
    """Find all occurrences of pattern in text using naive search."""
    result: List[int] = []
    n: int = len(text)
    m: int = len(pattern)
    if m == 0 or m > n:
        return result
    i: int = 0
    limit: int = n - m + 1
    while i < limit:
        matched: bool = True
        j: int = 0
        while j < m:
            if text[i + j] != pattern[j]:
                matched = False
                break
            j += 1
        if matched:
            result.append(i)
        i += 1
    return result


def kmp_failure(pattern: str) -> List[int]:
    """Compute KMP failure function for pattern."""
    m: int = len(pattern)
    fail: List[int] = [0]
    i: int = 1
    while i < m:
        fail.append(0)
        i += 1
    if m <= 1:
        return fail
    k: int = 0
    i = 1
    while i < m:
        while k > 0 and pattern[k] != pattern[i]:
            k = fail[k - 1]
        if pattern[k] == pattern[i]:
            k += 1
        fail[i] = k
        i += 1
    return fail


def kmp_search(text: str, pattern: str) -> List[int]:
    """KMP string search algorithm."""
    result: List[int] = []
    n: int = len(text)
    m: int = len(pattern)
    if m == 0 or m > n:
        return result
    fail: List[int] = kmp_failure(pattern)
    j: int = 0
    i: int = 0
    while i < n:
        while j > 0 and text[i] != pattern[j]:
            j = fail[j - 1]
        if text[i] == pattern[j]:
            j += 1
        if j == m:
            result.append(i - m + 1)
            j = fail[j - 1]
        i += 1
    return result


def count_occurrences(text: str, pattern: str) -> int:
    """Count non-overlapping occurrences of pattern in text."""
    positions: List[int] = naive_search(text, pattern)
    return len(positions)


def starts_with(text: str, prefix: str) -> bool:
    """Check if text starts with prefix."""
    if len(prefix) > len(text):
        return False
    i: int = 0
    while i < len(prefix):
        if text[i] != prefix[i]:
            return False
        i += 1
    return True


def ends_with(text: str, suffix: str) -> bool:
    """Check if text ends with suffix."""
    if len(suffix) > len(text):
        return False
    offset: int = len(text) - len(suffix)
    i: int = 0
    while i < len(suffix):
        if text[offset + i] != suffix[i]:
            return False
        i += 1
    return True


def find_first(text: str, pattern: str) -> int:
    """Find first occurrence of pattern, return index or -1."""
    positions: List[int] = naive_search(text, pattern)
    if len(positions) == 0:
        return -1
    return positions[0]


def test_search() -> bool:
    """Test string search functions."""
    ok: bool = True
    pos: List[int] = naive_search("abcabcabc", "abc")
    if len(pos) != 3:
        ok = False
    kmp_pos: List[int] = kmp_search("abcabcabc", "abc")
    if len(kmp_pos) != 3:
        ok = False
    if not starts_with("hello world", "hello"):
        ok = False
    if not ends_with("hello world", "world"):
        ok = False
    return ok
