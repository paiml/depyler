# Pathological mixed: Numeric formatting (int to binary/hex/octal manually)
# Tests: base conversion, string building from numeric operations


def int_to_base_str(n: int, radix: int) -> str:
    """Convert integer to string in given base (2-16)."""
    if n == 0:
        return "0"
    digits: str = "0123456789abcdef"
    is_neg: bool = n < 0
    val: int = n
    if is_neg == True:
        val = 0 - n
    result: str = ""
    while val > 0:
        remainder: int = val % radix
        result = digits[remainder] + result
        val = val // radix
    if is_neg == True:
        return "-" + result
    return result


def pad_left_zeros(s: str, width: int) -> str:
    """Pad string with leading zeros to width."""
    result: str = s
    while len(result) < width:
        result = "0" + result
    return result


def format_byte_as_hex(val: int) -> str:
    """Format a byte (0-255) as two hex digits."""
    hex_str: str = int_to_base_str(val, 16)
    return pad_left_zeros(hex_str, 2)


def format_ip_address(a: int, b: int, c: int, d: int) -> str:
    """Format four octets as dotted IP string."""
    result: str = str(a) + "." + str(b) + "." + str(c) + "." + str(d)
    return result


def format_bytes_as_hex(vals: list[int]) -> str:
    """Format list of bytes as hex string with separators."""
    result: str = ""
    i: int = 0
    while i < len(vals):
        if i > 0:
            result = result + ":"
        result = result + format_byte_as_hex(vals[i])
        i = i + 1
    return result


def count_set_bits(n: int) -> int:
    """Count number of 1-bits in binary representation."""
    binary: str = int_to_base_str(n, 2)
    count: int = 0
    i: int = 0
    while i < len(binary):
        c: str = binary[i]
        if c == "1":
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    passed: int = 0
    # Test 1: base 2
    if int_to_base_str(42, 2) == "101010":
        passed = passed + 1
    # Test 2: base 8
    if int_to_base_str(255, 8) == "377":
        passed = passed + 1
    # Test 3: pad zeros
    if pad_left_zeros("ff", 4) == "00ff":
        passed = passed + 1
    # Test 4: format byte
    if format_byte_as_hex(10) == "0a":
        passed = passed + 1
    # Test 5: format IP
    if format_ip_address(192, 168, 1, 1) == "192.168.1.1":
        passed = passed + 1
    # Test 6: hex bytes
    if format_bytes_as_hex([255, 0, 171]) == "ff:00:ab":
        passed = passed + 1
    # Test 7: count set bits (42 = 101010 = 3 ones)
    if count_set_bits(42) == 3:
        passed = passed + 1
    return passed
