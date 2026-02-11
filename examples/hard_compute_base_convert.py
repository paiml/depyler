"""Base conversion: decimal/binary/hex/octal as strings.

Tests: dec_to_bin, bin_to_dec, dec_to_hex, hex_to_dec, dec_to_oct.
"""


def dec_to_bin(n: int) -> str:
    """Convert non-negative decimal to binary string."""
    if n == 0:
        return "0"
    result: str = ""
    val: int = n
    while val > 0:
        bit: int = val % 2
        if bit == 0:
            result = "0" + result
        else:
            result = "1" + result
        val = val // 2
    return result


def bin_to_dec(s: str) -> int:
    """Convert binary string to decimal integer."""
    result: int = 0
    i: int = 0
    length: int = len(s)
    while i < length:
        result = result * 2
        if s[i] == "1":
            result = result + 1
        i = i + 1
    return result


def dec_to_oct(n: int) -> str:
    """Convert non-negative decimal to octal string."""
    if n == 0:
        return "0"
    result: str = ""
    val: int = n
    while val > 0:
        digit: int = val % 8
        if digit == 0:
            result = "0" + result
        elif digit == 1:
            result = "1" + result
        elif digit == 2:
            result = "2" + result
        elif digit == 3:
            result = "3" + result
        elif digit == 4:
            result = "4" + result
        elif digit == 5:
            result = "5" + result
        elif digit == 6:
            result = "6" + result
        else:
            result = "7" + result
        val = val // 8
    return result


def oct_to_dec(s: str) -> int:
    """Convert octal string to decimal."""
    result: int = 0
    i: int = 0
    length: int = len(s)
    while i < length:
        result = result * 8
        ch: str = s[i]
        if ch == "1":
            result = result + 1
        elif ch == "2":
            result = result + 2
        elif ch == "3":
            result = result + 3
        elif ch == "4":
            result = result + 4
        elif ch == "5":
            result = result + 5
        elif ch == "6":
            result = result + 6
        elif ch == "7":
            result = result + 7
        i = i + 1
    return result


def dec_to_hex(n: int) -> str:
    """Convert non-negative decimal to hexadecimal string (lowercase)."""
    if n == 0:
        return "0"
    hex_chars: str = "0123456789abcdef"
    result: str = ""
    val: int = n
    while val > 0:
        digit: int = val % 16
        ch: str = hex_chars[digit]
        result = ch + result
        val = val // 16
    return result


def hex_to_dec(s: str) -> int:
    """Convert hex string (lowercase) to decimal."""
    result: int = 0
    i: int = 0
    length: int = len(s)
    while i < length:
        result = result * 16
        ch: str = s[i]
        if ch == "a":
            result = result + 10
        elif ch == "b":
            result = result + 11
        elif ch == "c":
            result = result + 12
        elif ch == "d":
            result = result + 13
        elif ch == "e":
            result = result + 14
        elif ch == "f":
            result = result + 15
        else:
            result = result + ord(ch) - 48
        i = i + 1
    return result


def base_convert(n: int, from_base: int, to_base: int) -> int:
    """Convert between arbitrary bases (2-10). Input and output as decimal of digits."""
    decimal: int = 0
    place: int = 1
    val: int = n
    while val > 0:
        digit: int = val % 10
        decimal = decimal + digit * place
        place = place * from_base
        val = val // 10
    result: int = 0
    out_place: int = 1
    while decimal > 0:
        digit2: int = decimal % to_base
        result = result + digit2 * out_place
        out_place = out_place * 10
        decimal = decimal // to_base
    return result


def test_module() -> int:
    """Test base conversion algorithms."""
    passed: int = 0

    if dec_to_bin(10) == "1010":
        passed = passed + 1

    if bin_to_dec("1010") == 10:
        passed = passed + 1

    if dec_to_hex(255) == "ff":
        passed = passed + 1

    if hex_to_dec("ff") == 255:
        passed = passed + 1

    if dec_to_oct(8) == "10":
        passed = passed + 1

    if oct_to_dec("10") == 8:
        passed = passed + 1

    if dec_to_bin(0) == "0":
        passed = passed + 1

    rt: int = hex_to_dec(dec_to_hex(12345))
    if rt == 12345:
        passed = passed + 1

    return passed
