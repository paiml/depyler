"""Expand compressed strings.

Implements run-length decoding and other string expansion
algorithms that convert compact representations to full strings.
"""


def rle_expand(encoded: str) -> str:
    """Expand a run-length encoded string like '3a2b' to 'aaabb'."""
    result: str = ""
    i: int = 0
    enc_len: int = len(encoded)
    while i < enc_len:
        count: int = 0
        while i < enc_len and encoded[i] >= "0" and encoded[i] <= "9":
            digit: int = ord(encoded[i]) - ord("0")
            count = count * 10 + digit
            i = i + 1
        if i < enc_len:
            ch: str = encoded[i]
            j: int = 0
            while j < count:
                result = result + ch
                j = j + 1
            i = i + 1
    return result


def repeat_string(s: str, times: int) -> str:
    """Repeat a string a given number of times."""
    result: str = ""
    i: int = 0
    while i < times:
        result = result + s
        i = i + 1
    return result


def expand_pairs(s: str) -> str:
    """Expand pairs where first char is count digit and second is character."""
    result: str = ""
    s_len: int = len(s)
    i: int = 0
    while i + 1 < s_len:
        count: int = ord(s[i]) - ord("0")
        ch: str = s[i + 1]
        j: int = 0
        while j < count:
            result = result + ch
            j = j + 1
        i = i + 2
    return result


def count_expanded_length(encoded: str) -> int:
    """Count the length of expanded string without actually building it."""
    total: int = 0
    i: int = 0
    enc_len: int = len(encoded)
    while i < enc_len:
        count: int = 0
        while i < enc_len and encoded[i] >= "0" and encoded[i] <= "9":
            digit: int = ord(encoded[i]) - ord("0")
            count = count * 10 + digit
            i = i + 1
        if i < enc_len:
            total = total + count
            i = i + 1
    return total


def test_module() -> int:
    """Test string expansion operations."""
    ok: int = 0

    expanded: str = rle_expand("3a2b1c")
    if expanded == "aaabbc":
        ok = ok + 1

    repeated: str = repeat_string("ab", 3)
    if repeated == "ababab":
        ok = ok + 1

    pairs: str = expand_pairs("3a2b")
    if pairs == "aaabb":
        ok = ok + 1

    length: int = count_expanded_length("3a2b1c")
    if length == 6:
        ok = ok + 1

    return ok
