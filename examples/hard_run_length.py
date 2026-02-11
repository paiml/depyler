"""Run-length encoding and decoding for strings."""


def rle_encode(text: str) -> str:
    """Encode string using run-length encoding."""
    if len(text) == 0:
        return ""
    result: str = ""
    i: int = 0
    length: int = len(text)
    while i < length:
        current: str = text[i]
        count: int = 1
        next_idx: int = i + 1
        while next_idx < length and text[next_idx] == current:
            count = count + 1
            next_idx = next_idx + 1
        if count > 1:
            result = result + str(count) + current
        else:
            result = result + current
        i = next_idx
    return result


def rle_decode(encoded: str) -> str:
    """Decode a run-length encoded string."""
    result: str = ""
    i: int = 0
    length: int = len(encoded)
    while i < length:
        num_str: str = ""
        while i < length and encoded[i].isdigit():
            num_str = num_str + encoded[i]
            i = i + 1
        if i < length:
            ch: str = encoded[i]
            if num_str == "":
                result = result + ch
            else:
                count: int = int(num_str)
                j: int = 0
                while j < count:
                    result = result + ch
                    j = j + 1
            i = i + 1
    return result


def rle_compress_ratio(text: str) -> int:
    """Return compression ratio as percentage (100 = same size)."""
    if len(text) == 0:
        return 100
    encoded: str = rle_encode(text)
    original_len: int = len(text)
    encoded_len: int = len(encoded)
    ratio: int = (encoded_len * 100) // original_len
    return ratio


def test_module() -> int:
    """Test run-length encoding operations."""
    passed: int = 0

    r1: str = rle_encode("aaabbc")
    if r1 == "3a2bc":
        passed = passed + 1

    r2: str = rle_encode("abc")
    if r2 == "abc":
        passed = passed + 1

    r3: str = rle_decode("3a2bc")
    if r3 == "aaabbc":
        passed = passed + 1

    r4: str = rle_decode("abc")
    if r4 == "abc":
        passed = passed + 1

    roundtrip: str = rle_decode(rle_encode("aaaaabbbbcc"))
    if roundtrip == "aaaaabbbbcc":
        passed = passed + 1

    r6: str = rle_encode("")
    if r6 == "":
        passed = passed + 1

    r7: int = rle_compress_ratio("aaaaaaaaaa")
    if r7 < 100:
        passed = passed + 1

    r8: int = rle_compress_ratio("abcdef")
    if r8 == 100:
        passed = passed + 1

    return passed
