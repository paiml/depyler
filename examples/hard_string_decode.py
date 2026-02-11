"""String decoding operations.

Implements various string decoding algorithms including
Caesar cipher decoding, character shifting, and encoding reversal.
"""


def caesar_decode(s: str, shift: int) -> str:
    """Decode a Caesar cipher with given shift on lowercase letters."""
    result: str = ""
    s_len: int = len(s)
    i: int = 0
    while i < s_len:
        ch: str = s[i]
        code: int = ord(ch)
        if code >= 97 and code <= 122:
            shifted: int = ((code - 97 - shift) % 26 + 26) % 26 + 97
            decoded_ch: str = chr(shifted)
            result = result + decoded_ch
        else:
            result = result + ch
        i = i + 1
    return result


def decode_offset_string(s: str) -> str:
    """Decode string where each char is offset by its position index."""
    result: str = ""
    s_len: int = len(s)
    i: int = 0
    while i < s_len:
        code: int = ord(s[i])
        decoded: int = code - i
        result = result + chr(decoded)
        i = i + 1
    return result


def decode_pairs_to_chars(encoded: list[int], size: int) -> str:
    """Decode pairs of integers into characters. Each pair (high, low) = high*16+low."""
    result: str = ""
    i: int = 0
    while i + 1 < size:
        high: int = encoded[i]
        low: int = encoded[i + 1]
        char_code: int = high * 16 + low
        result = result + chr(char_code)
        i = i + 2
    return result


def xor_decode(encoded: list[int], size: int, mask: int) -> str:
    """Decode by XORing each value with a mask to get character codes."""
    result: str = ""
    i: int = 0
    while i < size:
        code: int = encoded[i] ^ mask
        result = result + chr(code)
        i = i + 1
    return result


def test_module() -> int:
    """Test string decoding operations."""
    ok: int = 0

    decoded: str = caesar_decode("def", 3)
    if decoded == "abc":
        ok = ok + 1

    round_trip: str = caesar_decode("abc", 0)
    if round_trip == "abc":
        ok = ok + 1

    pairs: list[int] = [6, 1, 6, 2, 6, 3]
    tmp_decoded: str = decode_pairs_to_chars(pairs, 6)
    if tmp_decoded == "abc":
        ok = ok + 1

    enc: list[int] = [97 ^ 42, 98 ^ 42, 99 ^ 42]
    xor_result: str = xor_decode(enc, 3, 42)
    if xor_result == "abc":
        ok = ok + 1

    return ok
