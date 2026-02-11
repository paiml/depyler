"""Simple base64-like encoding using integer arrays."""


def encode_byte(val: int) -> list[int]:
    """Encode a single byte (0-255) into two base64-like values (0-63)."""
    high: int = (val >> 2) & 63
    low: int = (val & 3) << 4
    result: list[int] = [high, low]
    return result


def decode_pair(high: int, low: int) -> int:
    """Decode a pair of base64-like values back to a byte."""
    val: int = ((high & 63) << 2) | ((low >> 4) & 3)
    return val


def encode_array(data: list[int]) -> list[int]:
    """Encode an array of bytes into base64-like values."""
    encoded: list[int] = []
    i: int = 0
    length: int = len(data)
    while i < length:
        val: int = data[i]
        pair: list[int] = encode_byte(val)
        encoded.append(pair[0])
        encoded.append(pair[1])
        i = i + 1
    return encoded


def decode_array(encoded: list[int]) -> list[int]:
    """Decode base64-like values back to bytes."""
    decoded: list[int] = []
    i: int = 0
    length: int = len(encoded)
    while i < length:
        next_i: int = i + 1
        if next_i < length:
            val: int = decode_pair(encoded[i], encoded[next_i])
            decoded.append(val)
        i = i + 2
    return decoded


def test_module() -> int:
    """Test base64-like encoding operations."""
    passed: int = 0

    pair: list[int] = encode_byte(65)
    if pair[0] == 16:
        passed = passed + 1

    r2: int = decode_pair(16, 16)
    if r2 == 65:
        passed = passed + 1

    data: list[int] = [65, 66, 67]
    enc: list[int] = encode_array(data)
    if len(enc) == 6:
        passed = passed + 1

    dec: list[int] = decode_array(enc)
    if len(dec) == 3:
        passed = passed + 1

    if dec[0] == 65:
        passed = passed + 1

    if dec[1] == 66:
        passed = passed + 1

    if dec[2] == 67:
        passed = passed + 1

    empty_enc: list[int] = encode_array([])
    if len(empty_enc) == 0:
        passed = passed + 1

    return passed
