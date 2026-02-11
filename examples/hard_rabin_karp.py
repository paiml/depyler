"""Rabin-Karp hash-based string matching."""


def rk_hash(s: str, start: int, length: int, base: int, mod: int) -> int:
    """Compute rolling hash of s[start:start+length]."""
    h: int = 0
    i: int = 0
    while i < length:
        h = (h * base + ord(s[start + i])) % mod
        i = i + 1
    return h


def power_mod(base: int, exp: int, mod: int) -> int:
    """Compute (base^exp) % mod."""
    result: int = 1
    b: int = base % mod
    e: int = exp
    while e > 0:
        if e % 2 == 1:
            result = (result * b) % mod
        b = (b * b) % mod
        e = e // 2
    return result


def rabin_karp_search(text: str, pattern: str) -> int:
    """Find first occurrence of pattern in text using Rabin-Karp. Returns index or -1."""
    n: int = len(text)
    m: int = len(pattern)
    if m == 0:
        return 0
    if n < m:
        return -1
    base: int = 256
    mod: int = 1000000007
    pat_hash: int = rk_hash(pattern, 0, m, base, mod)
    txt_hash: int = rk_hash(text, 0, m, base, mod)
    high_pow: int = power_mod(base, m - 1, mod)
    i: int = 0
    while i <= n - m:
        if txt_hash == pat_hash:
            match: int = 1
            j: int = 0
            while j < m:
                if text[i + j] != pattern[j]:
                    match = 0
                    break
                j = j + 1
            if match == 1:
                return i
        if i < n - m:
            txt_hash = (txt_hash - ord(text[i]) * high_pow % mod + mod) % mod
            txt_hash = (txt_hash * base + ord(text[i + m])) % mod
        i = i + 1
    return -1


def rabin_karp_count(text: str, pattern: str) -> int:
    """Count non-overlapping occurrences of pattern in text."""
    n: int = len(text)
    m: int = len(pattern)
    if m == 0:
        return 0
    count: int = 0
    pos: int = 0
    while pos <= n - m:
        sub_start: int = pos
        found: int = 1
        j: int = 0
        while j < m:
            if text[sub_start + j] != pattern[j]:
                found = 0
                break
            j = j + 1
        if found == 1:
            count = count + 1
            pos = pos + m
        else:
            pos = pos + 1
    return count


def test_module() -> int:
    passed: int = 0

    if rabin_karp_search("hello world", "world") == 6:
        passed = passed + 1

    if rabin_karp_search("abcabc", "cab") == 2:
        passed = passed + 1

    if rabin_karp_search("aaaa", "bbb") == -1:
        passed = passed + 1

    if rabin_karp_count("ababab", "ab") == 3:
        passed = passed + 1

    if rabin_karp_count("aaaaaa", "aa") == 3:
        passed = passed + 1

    if rabin_karp_search("abcdef", "") == 0:
        passed = passed + 1

    h1: int = rk_hash("abc", 0, 3, 256, 1000000007)
    h2: int = rk_hash("abc", 0, 3, 256, 1000000007)
    if h1 == h2:
        passed = passed + 1

    return passed
