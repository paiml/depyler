"""Rolling hash computation.

Tests: polynomial hash, rolling window hash, hash match count.
"""


def poly_hash(s: str, base: int, modulus: int) -> int:
    """Compute polynomial hash of string."""
    h: int = 0
    i: int = 0
    n: int = len(s)
    while i < n:
        code: int = ord(s[i])
        h = (h * base + code) % modulus
        i = i + 1
    return h


def power_mod(base: int, exp: int, modulus: int) -> int:
    """Compute base^exp mod modulus."""
    result: int = 1
    b: int = base % modulus
    e: int = exp
    while e > 0:
        if e % 2 == 1:
            result = (result * b) % modulus
        b = (b * b) % modulus
        e = e // 2
    return result


def count_hash_matches(text: str, pattern_len: int, target_hash: int, base: int, modulus: int) -> int:
    """Count substrings of given length matching the target hash."""
    n: int = len(text)
    if n < pattern_len:
        return 0
    count: int = 0
    high_pow: int = power_mod(base, pattern_len - 1, modulus)
    h: int = 0
    i: int = 0
    while i < pattern_len:
        h = (h * base + ord(text[i])) % modulus
        i = i + 1
    if h == target_hash:
        count = count + 1
    i = 1
    while i <= n - pattern_len:
        old_char: int = ord(text[i - 1])
        new_char: int = ord(text[i + pattern_len - 1])
        h = (h - old_char * high_pow % modulus + modulus) % modulus
        h = (h * base + new_char) % modulus
        if h == target_hash:
            count = count + 1
        i = i + 1
    return count


def hash_distance(s1: str, s2: str, base: int, modulus: int) -> int:
    """Absolute difference of hashes of two strings."""
    h1: int = poly_hash(s1, base, modulus)
    h2: int = poly_hash(s2, base, modulus)
    diff: int = h1 - h2
    if diff < 0:
        diff = -diff
    return diff


def test_module() -> int:
    """Test rolling hash operations."""
    ok: int = 0
    h1: int = poly_hash("abc", 31, 1000000007)
    h2: int = poly_hash("abc", 31, 1000000007)
    if h1 == h2:
        ok = ok + 1
    h3: int = poly_hash("abd", 31, 1000000007)
    if h1 != h3:
        ok = ok + 1
    if power_mod(2, 10, 1000) == 24:
        ok = ok + 1
    if power_mod(3, 5, 100) == 43:
        ok = ok + 1
    dist: int = hash_distance("abc", "abc", 31, 1000000007)
    if dist == 0:
        ok = ok + 1
    return ok
