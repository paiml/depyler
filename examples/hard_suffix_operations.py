"""Suffix operations on strings.

Tests: common suffix, suffix match count, suffix length, strip suffix.
"""


def common_suffix_len(s1: str, s2: str) -> int:
    """Length of common suffix between s1 and s2."""
    n1: int = len(s1)
    n2: int = len(s2)
    length: int = 0
    i1: int = n1 - 1
    i2: int = n2 - 1
    while i1 >= 0 and i2 >= 0:
        if s1[i1] != s2[i2]:
            return length
        length = length + 1
        i1 = i1 - 1
        i2 = i2 - 1
    return length


def count_suffix_matches(words: list[str], suffix: str) -> int:
    """Count how many words end with the given suffix."""
    count: int = 0
    slen: int = len(suffix)
    for w in words:
        wlen: int = len(w)
        if wlen >= slen:
            match: int = 1
            j: int = 0
            while j < slen:
                if w[wlen - slen + j] != suffix[j]:
                    match = 0
                    j = slen
                else:
                    j = j + 1
            if match == 1:
                count = count + 1
    return count


def longest_suffix_palindrome_len(s: str) -> int:
    """Length of the longest suffix of s that is a palindrome."""
    n: int = len(s)
    best: int = 0
    start: int = n - 1
    while start >= 0:
        length: int = n - start
        is_pal: int = 1
        lo: int = start
        hi: int = n - 1
        while lo < hi:
            if s[lo] != s[hi]:
                is_pal = 0
                lo = hi
            else:
                lo = lo + 1
                hi = hi - 1
        if is_pal == 1 and length > best:
            best = length
        start = start - 1
    return best


def test_module() -> int:
    """Test suffix operations."""
    ok: int = 0
    if common_suffix_len("testing", "running") == 4:
        ok = ok + 1
    if common_suffix_len("abc", "xyz") == 0:
        ok = ok + 1
    if count_suffix_matches(["running", "jumping", "sat"], "ing") == 2:
        ok = ok + 1
    if count_suffix_matches(["hello", "world"], "xyz") == 0:
        ok = ok + 1
    if longest_suffix_palindrome_len("abcba") == 5:
        ok = ok + 1
    if longest_suffix_palindrome_len("abcd") == 1:
        ok = ok + 1
    return ok
