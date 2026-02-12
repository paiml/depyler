"""Text processing: String hashing and fingerprinting.

Tests: polynomial hash, rolling hash, hash-based deduplication,
fingerprint comparison, hash collision handling.
"""

from typing import Dict, List, Tuple


def polynomial_hash(s: str, base: int, mod: int) -> int:
    """Compute polynomial rolling hash of string."""
    h: int = 0
    i: int = 0
    while i < len(s):
        h = (h * base + ord(s[i])) % mod
        i += 1
    return h


def rolling_hash_windows(text: str, window_size: int) -> List[int]:
    """Compute rolling hash for all windows of given size."""
    result: List[int] = []
    n: int = len(text)
    if window_size > n or window_size <= 0:
        return result
    base: int = 31
    mod: int = 999999937
    power: int = 1
    i: int = 0
    ws_minus1: int = window_size - 1
    while i < ws_minus1:
        power = (power * base) % mod
        i += 1
    h: int = 0
    i = 0
    while i < window_size:
        h = (h * base + ord(text[i])) % mod
        i += 1
    result.append(h)
    i = window_size
    while i < n:
        prev_idx: int = i - window_size
        h = (h - ord(text[prev_idx]) * power % mod + mod) % mod
        h = (h * base + ord(text[i])) % mod
        result.append(h)
        i += 1
    return result


def deduplicate_strings(strings: List[str]) -> List[str]:
    """Remove duplicate strings using hash-based approach."""
    seen: Dict[str, int] = {}
    result: List[str] = []
    for s in strings:
        if s not in seen:
            seen[s] = 1
            result.append(s)
    return result


def string_fingerprint(s: str) -> int:
    """Compute a multi-hash fingerprint for stronger identification."""
    h1: int = polynomial_hash(s, 31, 999999937)
    h2: int = polynomial_hash(s, 37, 999999929)
    combined: int = h1 * 1000000 + h2
    return combined


def find_common_substrings(text: str, substr_len: int) -> List[int]:
    """Find positions where substrings of given length repeat."""
    hashes: List[int] = rolling_hash_windows(text, substr_len)
    seen: Dict[int, int] = {}
    result: List[int] = []
    i: int = 0
    while i < len(hashes):
        h: int = hashes[i]
        if h in seen:
            result.append(i)
        else:
            seen[h] = i
        i += 1
    return result


def test_hashing() -> bool:
    """Test string hashing functions."""
    ok: bool = True
    h: int = polynomial_hash("hello", 31, 999999937)
    if h <= 0:
        ok = False
    rh: List[int] = rolling_hash_windows("abcdef", 3)
    if len(rh) != 4:
        ok = False
    dd: List[str] = deduplicate_strings(["a", "b", "a", "c", "b"])
    if len(dd) != 3:
        ok = False
    return ok
