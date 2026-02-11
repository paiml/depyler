"""String matching algorithm patterns.

Tests: brute force search, KMP-style matching, Boyer-Moore-like bad character,
string hashing for match, and wildcard matching.
"""


def brute_force_search(text: str, pattern: str) -> list[int]:
    """Find all occurrences of pattern in text using brute force."""
    n: int = len(text)
    m: int = len(pattern)
    result: list[int] = []
    if m == 0 or m > n:
        return result
    i: int = 0
    while i <= n - m:
        match: bool = True
        j: int = 0
        while j < m:
            if text[i + j] != pattern[j]:
                match = False
                j = m
            else:
                j = j + 1
        if match:
            result.append(i)
        i = i + 1
    return result


def compute_lps(pattern: str) -> list[int]:
    """Compute longest proper prefix-suffix array for KMP."""
    m: int = len(pattern)
    lps: list[int] = [0] * m
    length: int = 0
    i: int = 1
    while i < m:
        if pattern[i] == pattern[length]:
            length = length + 1
            lps[i] = length
            i = i + 1
        elif length != 0:
            length = lps[length - 1]
        else:
            lps[i] = 0
            i = i + 1
    return lps


def kmp_search_all(text: str, pattern: str) -> list[int]:
    """KMP pattern matching: find all occurrences."""
    n: int = len(text)
    m: int = len(pattern)
    if m == 0:
        return []
    lps: list[int] = compute_lps(pattern)
    result: list[int] = []
    ti: int = 0
    pi: int = 0
    while ti < n:
        if text[ti] == pattern[pi]:
            ti = ti + 1
            pi = pi + 1
        if pi == m:
            result.append(ti - m)
            pi = lps[pi - 1]
        elif ti < n and text[ti] != pattern[pi]:
            if pi != 0:
                pi = lps[pi - 1]
            else:
                ti = ti + 1
    return result


def count_occurrences(text: str, pattern: str) -> int:
    """Count non-overlapping occurrences of pattern in text."""
    n: int = len(text)
    m: int = len(pattern)
    if m == 0:
        return 0
    count: int = 0
    i: int = 0
    while i <= n - m:
        match: bool = True
        j: int = 0
        while j < m:
            if text[i + j] != pattern[j]:
                match = False
                j = m
            else:
                j = j + 1
        if match:
            count = count + 1
            i = i + m
        else:
            i = i + 1
    return count


def replace_all(text: str, old: str, new: str) -> str:
    """Replace all non-overlapping occurrences of old with new."""
    n: int = len(text)
    m: int = len(old)
    if m == 0:
        return text
    result: str = ""
    i: int = 0
    while i < n:
        if i <= n - m:
            match: bool = True
            j: int = 0
            while j < m:
                if text[i + j] != old[j]:
                    match = False
                    j = m
                else:
                    j = j + 1
            if match:
                result = result + new
                i = i + m
            else:
                result = result + text[i]
                i = i + 1
        else:
            result = result + text[i]
            i = i + 1
    return result


def test_module() -> bool:
    """Test all string matching functions."""
    ok: bool = True

    bf: list[int] = brute_force_search("abcabcabc", "abc")
    if bf != [0, 3, 6]:
        ok = False

    lps: list[int] = compute_lps("aabaab")
    if lps != [0, 1, 0, 1, 2, 3]:
        ok = False

    kmp: list[int] = kmp_search_all("ababababab", "abab")
    if len(kmp) < 2:
        ok = False
    if kmp[0] != 0:
        ok = False

    if count_occurrences("aaaa", "aa") != 2:
        ok = False

    rep: str = replace_all("hello world hello", "hello", "hi")
    if rep != "hi world hi":
        ok = False

    return ok
