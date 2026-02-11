"""Encoding and decoding patterns.

Tests: run-length encoding/decoding, base conversion, variable-length encoding,
delta encoding, and zigzag encoding.
"""


def rle_encode(data: list[int]) -> list[list[int]]:
    """Run-length encode a list of integers. Returns pairs [value, count]."""
    n: int = len(data)
    if n == 0:
        return []
    result: list[list[int]] = []
    current: int = data[0]
    count: int = 1
    i: int = 1
    while i < n:
        if data[i] == current:
            count = count + 1
        else:
            result.append([current, count])
            current = data[i]
            count = 1
        i = i + 1
    result.append([current, count])
    return result


def rle_decode(encoded: list[list[int]]) -> list[int]:
    """Decode run-length encoded data."""
    result: list[int] = []
    i: int = 0
    while i < len(encoded):
        val: int = encoded[i][0]
        count: int = encoded[i][1]
        j: int = 0
        while j < count:
            result.append(val)
            j = j + 1
        i = i + 1
    return result


def int_to_base(num: int, base: int) -> str:
    """Convert non-negative integer to string in given base (2-16)."""
    if num == 0:
        return "0"
    digits: str = "0123456789abcdef"
    result: str = ""
    val: int = num
    while val > 0:
        remainder: int = val % base
        result = digits[remainder] + result
        val = val // base
    return result


def base_to_int(s: str, base: int) -> int:
    """Convert string in given base back to integer."""
    result: int = 0
    i: int = 0
    while i < len(s):
        c: str = s[i]
        digit: int = 0
        if c >= "0" and c <= "9":
            digit = ord(c) - ord("0")
        elif c >= "a" and c <= "f":
            digit = ord(c) - ord("a") + 10
        result = result * base + digit
        i = i + 1
    return result


def delta_encode(data: list[int]) -> list[int]:
    """Delta encoding: store differences between consecutive elements."""
    n: int = len(data)
    if n == 0:
        return []
    result: list[int] = [data[0]]
    i: int = 1
    while i < n:
        result.append(data[i] - data[i - 1])
        i = i + 1
    return result


def delta_decode(encoded: list[int]) -> list[int]:
    """Decode delta-encoded data back to original."""
    n: int = len(encoded)
    if n == 0:
        return []
    result: list[int] = [encoded[0]]
    i: int = 1
    while i < n:
        result.append(result[i - 1] + encoded[i])
        i = i + 1
    return result


def zigzag_encode(n: int) -> int:
    """ZigZag encoding: maps signed ints to unsigned. -1->1, 1->2, -2->3, 2->4."""
    if n >= 0:
        return 2 * n
    return 2 * (0 - n) - 1


def zigzag_decode(n: int) -> int:
    """ZigZag decoding: reverse of zigzag_encode."""
    if n % 2 == 0:
        return n // 2
    return 0 - ((n + 1) // 2)


def test_module() -> bool:
    """Test all encoding/decoding functions."""
    ok: bool = True

    enc: list[list[int]] = rle_encode([1, 1, 1, 2, 2, 3, 3, 3, 3])
    if len(enc) != 3:
        ok = False
    if enc[0] != [1, 3]:
        ok = False
    dec: list[int] = rle_decode(enc)
    if dec != [1, 1, 1, 2, 2, 3, 3, 3, 3]:
        ok = False

    if int_to_base(255, 16) != "ff":
        ok = False
    if int_to_base(10, 2) != "1010":
        ok = False
    if base_to_int("ff", 16) != 255:
        ok = False
    if base_to_int("1010", 2) != 10:
        ok = False

    data: list[int] = [10, 13, 15, 20, 18]
    denc: list[int] = delta_encode(data)
    ddec: list[int] = delta_decode(denc)
    if ddec != data:
        ok = False

    if zigzag_encode(0) != 0:
        ok = False
    if zigzag_encode(-1) != 1:
        ok = False
    if zigzag_encode(1) != 2:
        ok = False
    if zigzag_decode(0) != 0:
        ok = False
    if zigzag_decode(1) != -1:
        ok = False
    if zigzag_decode(2) != 1:
        ok = False

    return ok
