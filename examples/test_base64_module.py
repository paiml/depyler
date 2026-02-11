"""
Comprehensive test suite for base64-like encoding module.
Following TDD Book methodology: minimal examples, incremental complexity.

Tests base64 core features using integer-array implementation:
- Encoding and decoding with standard alphabet
- Padding handling
- Roundtrip verification
- Edge cases (empty, single byte, multi-byte)
"""


def b64_char_to_index(code: int) -> int:
    """Map a character code to a base64 index (0-63) or -1 for padding/invalid."""
    if code >= 65 and code <= 90:
        idx: int = code - 65
        return idx
    if code >= 97 and code <= 122:
        idx2: int = code - 97 + 26
        return idx2
    if code >= 48 and code <= 57:
        idx3: int = code - 48 + 52
        return idx3
    if code == 43:
        return 62
    if code == 47:
        return 63
    return -1


def b64_index_to_char(idx: int) -> int:
    """Map a base64 index (0-63) to an ASCII character code."""
    if idx >= 0 and idx <= 25:
        code: int = idx + 65
        return code
    if idx >= 26 and idx <= 51:
        code2: int = idx - 26 + 97
        return code2
    if idx >= 52 and idx <= 61:
        code3: int = idx - 52 + 48
        return code3
    if idx == 62:
        return 43
    if idx == 63:
        return 47
    return 0


def b64_encode_bytes(data: list[int]) -> list[int]:
    """Encode a list of byte values (0-255) into base64 character codes."""
    output: list[int] = []
    i: int = 0
    length: int = len(data)
    while i < length:
        b0: int = data[i]
        b1: int = 0
        b2: int = 0
        remaining: int = length - i
        if remaining >= 2:
            b1 = data[i + 1]
        if remaining >= 3:
            b2 = data[i + 2]

        c0: int = (b0 >> 2) & 63
        c1: int = ((b0 & 3) << 4) | ((b1 >> 4) & 15)
        c2: int = ((b1 & 15) << 2) | ((b2 >> 6) & 3)
        c3: int = b2 & 63

        output.append(b64_index_to_char(c0))
        output.append(b64_index_to_char(c1))
        if remaining >= 2:
            output.append(b64_index_to_char(c2))
        else:
            output.append(61)
        if remaining >= 3:
            output.append(b64_index_to_char(c3))
        else:
            output.append(61)
        i = i + 3
    return output


def b64_decode_chars(encoded: list[int]) -> list[int]:
    """Decode base64 character codes back to byte values."""
    output: list[int] = []
    i: int = 0
    length: int = len(encoded)
    while i + 3 < length:
        i0: int = b64_char_to_index(encoded[i])
        i1: int = b64_char_to_index(encoded[i + 1])
        i2: int = b64_char_to_index(encoded[i + 2])
        i3: int = b64_char_to_index(encoded[i + 3])
        if i0 < 0:
            i0 = 0
        if i1 < 0:
            i1 = 0

        b0: int = ((i0 << 2) | (i1 >> 4)) & 255
        output.append(b0)

        if encoded[i + 2] != 61:
            if i2 < 0:
                i2 = 0
            b1: int = (((i1 & 15) << 4) | (i2 >> 2)) & 255
            output.append(b1)

        if encoded[i + 3] != 61:
            if i3 < 0:
                i3 = 0
            b2: int = (((i2 & 3) << 6) | i3) & 255
            output.append(b2)

        i = i + 4
    return output


def str_to_bytes(text: str) -> list[int]:
    """Convert a string to a list of ASCII byte values."""
    result: list[int] = []
    for ch in text:
        code: int = ord(ch)
        result.append(code)
    return result


def bytes_to_str(data: list[int]) -> str:
    """Convert a list of byte values to a string."""
    result: str = ""
    for val in data:
        result = result + chr(val)
    return result


def lists_equal(a: list[int], b: list[int]) -> int:
    """Return 1 if lists are equal, 0 otherwise."""
    if len(a) != len(b):
        return 0
    i: int = 0
    length: int = len(a)
    while i < length:
        if a[i] != b[i]:
            return 0
        i = i + 1
    return 1


def test_encode_hello() -> int:
    """Test encoding 'Hello' to base64."""
    data: list[int] = [72, 101, 108, 108, 111]
    encoded: list[int] = b64_encode_bytes(data)
    expected: list[int] = [83, 71, 86, 115, 98, 71, 56, 61]
    return lists_equal(encoded, expected)


def test_decode_hello() -> int:
    """Test decoding base64 back to 'Hello'."""
    encoded: list[int] = [83, 71, 86, 115, 98, 71, 56, 61]
    decoded: list[int] = b64_decode_chars(encoded)
    expected: list[int] = [72, 101, 108, 108, 111]
    return lists_equal(decoded, expected)


def test_roundtrip_abc() -> int:
    """Test encode-decode roundtrip for 'ABC'."""
    original: list[int] = [65, 66, 67]
    encoded: list[int] = b64_encode_bytes(original)
    decoded: list[int] = b64_decode_chars(encoded)
    return lists_equal(decoded, original)


def test_roundtrip_single_byte() -> int:
    """Test encode-decode roundtrip for a single byte."""
    original: list[int] = [42]
    encoded: list[int] = b64_encode_bytes(original)
    decoded: list[int] = b64_decode_chars(encoded)
    return lists_equal(decoded, original)


def test_roundtrip_two_bytes() -> int:
    """Test encode-decode roundtrip for two bytes."""
    original: list[int] = [200, 150]
    encoded: list[int] = b64_encode_bytes(original)
    decoded: list[int] = b64_decode_chars(encoded)
    return lists_equal(decoded, original)


def test_roundtrip_three_bytes() -> int:
    """Test encode-decode roundtrip for exactly three bytes (no padding)."""
    original: list[int] = [77, 97, 110]
    encoded: list[int] = b64_encode_bytes(original)
    decoded: list[int] = b64_decode_chars(encoded)
    eq: int = lists_equal(decoded, original)
    has_no_pad: int = 0
    if len(encoded) == 4:
        if encoded[2] != 61 and encoded[3] != 61:
            has_no_pad = 1
    if eq == 1 and has_no_pad == 1:
        return 1
    return 0


def test_encode_empty() -> int:
    """Test encoding empty data."""
    data: list[int] = []
    encoded: list[int] = b64_encode_bytes(data)
    if len(encoded) == 0:
        return 1
    return 0


def test_index_mapping_roundtrip() -> int:
    """Test that b64 index mapping is consistent for all 64 values."""
    i: int = 0
    while i < 64:
        ch: int = b64_index_to_char(i)
        back: int = b64_char_to_index(ch)
        if back != i:
            return 0
        i = i + 1
    return 1


def test_module() -> int:
    """Test base64-like encoding operations. Returns count of passed tests."""
    passed: int = 0

    r1: int = test_encode_hello()
    passed = passed + r1

    r2: int = test_decode_hello()
    passed = passed + r2

    r3: int = test_roundtrip_abc()
    passed = passed + r3

    r4: int = test_roundtrip_single_byte()
    passed = passed + r4

    r5: int = test_roundtrip_two_bytes()
    passed = passed + r5

    r6: int = test_roundtrip_three_bytes()
    passed = passed + r6

    r7: int = test_encode_empty()
    passed = passed + r7

    r8: int = test_index_mapping_roundtrip()
    passed = passed + r8

    return passed
