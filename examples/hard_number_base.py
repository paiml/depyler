"""Number base conversion: decimal to binary, binary to decimal, hex.

Tests: dec_to_bin, bin_to_dec, dec_to_hex, hex_to_dec.
"""


def dec_to_bin(n: int) -> str:
    """Convert non-negative decimal to binary string."""
    if n == 0:
        return "0"
    result: str = ""
    val: int = n
    while val > 0:
        remainder: int = val % 2
        result = str(remainder) + result
        val = val // 2
    return result


def bin_to_dec(s: str) -> int:
    """Convert binary string to decimal integer."""
    result: int = 0
    i: int = 0
    while i < len(s):
        result = result * 2
        if s[i] == "1":
            result = result + 1
        i = i + 1
    return result


def dec_to_hex(n: int) -> str:
    """Convert non-negative decimal to lowercase hex string."""
    if n == 0:
        return "0"
    hex_chars: str = "0123456789abcdef"
    result: str = ""
    val: int = n
    while val > 0:
        remainder: int = val % 16
        result = hex_chars[remainder] + result
        val = val // 16
    return result


def hex_to_dec(s: str) -> int:
    """Convert lowercase hex string to decimal integer."""
    result: int = 0
    i: int = 0
    while i < len(s):
        result = result * 16
        c: str = s[i]
        if c >= "0" and c <= "9":
            result = result + ord(c) - ord("0")
        else:
            result = result + ord(c) - ord("a") + 10
        i = i + 1
    return result


def test_module() -> int:
    """Test number base conversions."""
    ok: int = 0

    if dec_to_bin(10) == "1010":
        ok = ok + 1

    if dec_to_bin(0) == "0":
        ok = ok + 1

    if bin_to_dec("1010") == 10:
        ok = ok + 1

    if bin_to_dec("0") == 0:
        ok = ok + 1

    if dec_to_hex(255) == "ff":
        ok = ok + 1

    if dec_to_hex(0) == "0":
        ok = ok + 1

    if hex_to_dec("ff") == 255:
        ok = ok + 1

    if hex_to_dec("1a") == 26:
        ok = ok + 1

    return ok
