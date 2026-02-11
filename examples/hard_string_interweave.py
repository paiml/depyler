"""Interweave strings.

Implements operations for interleaving characters from
multiple strings in various patterns.
"""


def interweave_two(s1: str, s2: str) -> str:
    """Interweave two strings character by character."""
    result: str = ""
    len1: int = len(s1)
    len2: int = len(s2)
    max_len: int = len1
    if len2 > max_len:
        max_len = len2
    i: int = 0
    while i < max_len:
        if i < len1:
            result = result + s1[i]
        if i < len2:
            result = result + s2[i]
        i = i + 1
    return result


def riffle_shuffle(s: str) -> str:
    """Split string in half and riffle shuffle (interleave halves)."""
    s_len: int = len(s)
    half: int = s_len // 2
    result: str = ""
    i: int = 0
    while i < half:
        result = result + s[i]
        idx: int = half + i
        if idx < s_len:
            result = result + s[idx]
        i = i + 1
    if s_len % 2 == 1:
        last_idx: int = s_len - 1
        result = result + s[last_idx]
    return result


def weave_with_separator(s1: str, s2: str, sep: str) -> str:
    """Interweave two strings with a separator between each pair."""
    result: str = ""
    len1: int = len(s1)
    len2: int = len(s2)
    max_len: int = len1
    if len2 > max_len:
        max_len = len2
    i: int = 0
    while i < max_len:
        if i > 0:
            result = result + sep
        if i < len1:
            result = result + s1[i]
        if i < len2:
            result = result + s2[i]
        i = i + 1
    return result


def reverse_interweave(s: str) -> str:
    """De-interweave a string into odd and even positioned chars, then concatenate."""
    evens: str = ""
    odds: str = ""
    s_len: int = len(s)
    i: int = 0
    while i < s_len:
        if i % 2 == 0:
            evens = evens + s[i]
        else:
            odds = odds + s[i]
        i = i + 1
    result: str = evens + odds
    return result


def test_module() -> int:
    """Test string interweave operations."""
    ok: int = 0

    woven: str = interweave_two("abc", "123")
    if woven == "a1b2c3":
        ok = ok + 1

    uneven: str = interweave_two("abcd", "12")
    if uneven == "a1b2cd":
        ok = ok + 1

    shuffled: str = riffle_shuffle("abcdef")
    if shuffled == "adbecf":
        ok = ok + 1

    sep_result: str = weave_with_separator("ab", "12", "-")
    if sep_result == "a1-b2":
        ok = ok + 1

    return ok
