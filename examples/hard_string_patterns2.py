"""String pattern matching: wildcard match, KMP failure function.

Tests: wildcard_match, kmp_failure, kmp_search.
"""


def wildcard_match(text: str, pattern: str) -> int:
    """Match text against pattern with '?' (any single char) and '*' (any sequence).
    
    Returns 1 if matches, 0 otherwise. Uses iterative approach.
    """
    t: int = len(text)
    p: int = len(pattern)
    ti: int = 0
    pi: int = 0
    star_idx: int = -1
    match_idx: int = 0
    while ti < t:
        if pi < p and (pattern[pi] == "?" or pattern[pi] == text[ti]):
            ti = ti + 1
            pi = pi + 1
        elif pi < p and pattern[pi] == "*":
            star_idx = pi
            match_idx = ti
            pi = pi + 1
        elif star_idx != -1:
            pi = star_idx + 1
            match_idx = match_idx + 1
            ti = match_idx
        else:
            return 0
    while pi < p and pattern[pi] == "*":
        pi = pi + 1
    if pi == p:
        return 1
    return 0


def kmp_failure(pattern: str) -> list[int]:
    """Compute KMP failure function (partial match table)."""
    n: int = len(pattern)
    fail: list[int] = []
    i: int = 0
    while i < n:
        fail.append(0)
        i = i + 1
    if n == 0:
        return fail
    k: int = 0
    i = 1
    while i < n:
        while k > 0 and pattern[k] != pattern[i]:
            k = fail[k - 1]
        if pattern[k] == pattern[i]:
            k = k + 1
        fail[i] = k
        i = i + 1
    return fail


def kmp_search(text: str, pattern: str) -> int:
    """Find first occurrence of pattern in text using KMP. Returns index or -1."""
    if len(pattern) == 0:
        return 0
    fail: list[int] = kmp_failure(pattern)
    j: int = 0
    i: int = 0
    while i < len(text):
        while j > 0 and text[i] != pattern[j]:
            j = fail[j - 1]
        if text[i] == pattern[j]:
            j = j + 1
        if j == len(pattern):
            return i - len(pattern) + 1
        i = i + 1
    return -1


def test_module() -> int:
    """Test string pattern matching."""
    ok: int = 0

    if wildcard_match("hello", "h?llo") == 1:
        ok = ok + 1

    if wildcard_match("hello", "h*o") == 1:
        ok = ok + 1

    if wildcard_match("hello", "h*x") == 0:
        ok = ok + 1

    if wildcard_match("", "*") == 1:
        ok = ok + 1

    if kmp_failure("abcabd") == [0, 0, 0, 1, 2, 0]:
        ok = ok + 1

    if kmp_search("abcabcabd", "abcabd") == 3:
        ok = ok + 1

    if kmp_search("hello", "xyz") == -1:
        ok = ok + 1

    if kmp_search("aaa", "") == 0:
        ok = ok + 1

    return ok
