"""Text processing: Number formatting and conversion.

Tests: integer to string, string to integer, base conversion,
decimal formatting, sign handling.
"""

from typing import Dict, List, Tuple


def int_to_str(n: int) -> str:
    """Convert integer to string representation."""
    if n == 0:
        return "0"
    negative: bool = n < 0
    val: int = n
    if negative:
        val = -val
    digits: List[str] = []
    while val > 0:
        d: int = val % 10
        digits.append(chr(d + ord("0")))
        val = val // 10
    if negative:
        digits.append("-")
    digits.reverse()
    return "".join(digits)


def str_to_int(s: str) -> int:
    """Convert string to integer."""
    if len(s) == 0:
        return 0
    negative: bool = False
    start: int = 0
    if s[0] == "-":
        negative = True
        start = 1
    elif s[0] == "+":
        start = 1
    result: int = 0
    i: int = start
    while i < len(s):
        if s[i] >= "0" and s[i] <= "9":
            result = result * 10 + (ord(s[i]) - ord("0"))
        i += 1
    if negative:
        result = -result
    return result


def int_to_binary(n: int) -> str:
    """Convert integer to binary string."""
    if n == 0:
        return "0"
    val: int = n
    if val < 0:
        val = -val
    bits: List[str] = []
    while val > 0:
        if val % 2 == 1:
            bits.append("1")
        else:
            bits.append("0")
        val = val // 2
    bits.reverse()
    return "".join(bits)


def int_to_hex(n: int) -> str:
    """Convert integer to hexadecimal string."""
    if n == 0:
        return "0"
    hex_chars: str = "0123456789abcdef"
    val: int = n
    if val < 0:
        val = -val
    digits: List[str] = []
    while val > 0:
        digits.append(hex_chars[val % 16])
        val = val // 16
    digits.reverse()
    return "".join(digits)


def int_to_octal(n: int) -> str:
    """Convert integer to octal string."""
    if n == 0:
        return "0"
    val: int = n
    if val < 0:
        val = -val
    digits: List[str] = []
    while val > 0:
        digits.append(chr(val % 8 + ord("0")))
        val = val // 8
    digits.reverse()
    return "".join(digits)


def format_with_commas(n: int) -> str:
    """Format integer with comma separators."""
    s: str = int_to_str(n)
    if len(s) <= 3:
        return s
    result: List[str] = []
    count: int = 0
    i: int = len(s) - 1
    while i >= 0:
        if s[i] == "-":
            result.append(s[i])
        else:
            if count > 0 and count % 3 == 0:
                result.append(",")
            result.append(s[i])
            count += 1
        i -= 1
    result.reverse()
    return "".join(result)


def test_number_format() -> bool:
    """Test number formatting functions."""
    ok: bool = True
    if int_to_str(42) != "42":
        ok = False
    if int_to_str(-7) != "-7":
        ok = False
    if str_to_int("123") != 123:
        ok = False
    if int_to_binary(10) != "1010":
        ok = False
    if int_to_hex(255) != "ff":
        ok = False
    if int_to_octal(8) != "10":
        ok = False
    return ok
