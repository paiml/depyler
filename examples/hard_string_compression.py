"""String compression: run-length encoding, decoding, basic compression.

Tests: rle_encode, rle_decode, compress_string, decompress_string.
"""


def rle_encode(s: str) -> str:
    """Run-length encode: 'aaabbc' -> 'a3b2c1'."""
    if len(s) == 0:
        return ""
    result: str = ""
    i: int = 0
    while i < len(s):
        ch: str = s[i]
        count: int = 1
        while i + count < len(s) and s[i + count] == ch:
            count = count + 1
        result = result + ch + str(count)
        i = i + count
    return result


def rle_decode(s: str) -> str:
    """Decode run-length encoded string: 'a3b2c1' -> 'aaabbc'."""
    result: str = ""
    i: int = 0
    while i < len(s):
        ch: str = s[i]
        i = i + 1
        num_str: str = ""
        while i < len(s) and s[i] >= "0" and s[i] <= "9":
            num_str = num_str + s[i]
            i = i + 1
        count: int = int(num_str)
        j: int = 0
        while j < count:
            result = result + ch
            j = j + 1
    return result


def compress_string(s: str) -> str:
    """Compress if RLE is shorter, otherwise return original."""
    encoded: str = rle_encode(s)
    if len(encoded) < len(s):
        return encoded
    return s


def count_chars(s: str) -> int:
    """Count total characters in string."""
    return len(s)


def test_module() -> int:
    """Test string compression operations."""
    ok: int = 0

    if rle_encode("aaabbc") == "a3b2c1":
        ok = ok + 1

    if rle_encode("") == "":
        ok = ok + 1

    if rle_encode("a") == "a1":
        ok = ok + 1

    if rle_decode("a3b2c1") == "aaabbc":
        ok = ok + 1

    if rle_decode("x5") == "xxxxx":
        ok = ok + 1

    if compress_string("aaaaabbbcc") == "a5b3c2":
        ok = ok + 1

    if compress_string("abc") == "abc":
        ok = ok + 1

    return ok
