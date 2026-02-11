# Pathological mixed: Functions converting between int and str representations
# Tests: int-to-str, str-to-int, formatting, parsing


def int_to_binary_str(n: int) -> str:
    """Convert integer to binary string representation."""
    if n == 0:
        return "0"
    is_neg: bool = n < 0
    val: int = n
    if is_neg == True:
        val = 0 - n
    bits: str = ""
    while val > 0:
        if val % 2 == 1:
            bits = "1" + bits
        else:
            bits = "0" + bits
        val = val // 2
    if is_neg == True:
        return "-" + bits
    return bits


def int_to_hex_str(n: int) -> str:
    """Convert integer to hex string (lowercase)."""
    if n == 0:
        return "0"
    hex_digits: str = "0123456789abcdef"
    is_neg: bool = n < 0
    val: int = n
    if is_neg == True:
        val = 0 - n
    result: str = ""
    while val > 0:
        remainder: int = val % 16
        result = hex_digits[remainder] + result
        val = val // 16
    if is_neg == True:
        return "-" + result
    return result


def parse_int_from_str(s: str) -> int:
    """Parse integer from string, character by character."""
    if len(s) == 0:
        return 0
    is_neg: bool = False
    start: int = 0
    c0: str = s[0]
    if c0 == "-":
        is_neg = True
        start = 1
    result: int = 0
    i: int = start
    while i < len(s):
        c: str = s[i]
        digit: int = ord(c) - ord("0")
        result = result * 10 + digit
        i = i + 1
    if is_neg == True:
        return 0 - result
    return result


def format_with_commas(n: int) -> str:
    """Format integer with comma separators (e.g., 1,234,567)."""
    s: str = str(n)
    if len(s) <= 3:
        return s
    # Build from right to left
    result: str = ""
    count: int = 0
    i: int = len(s) - 1
    while i >= 0:
        if count > 0 and count % 3 == 0:
            result = "," + result
        result = s[i] + result
        count = count + 1
        i = i - 1
    return result


def digit_sum(n: int) -> int:
    """Sum of digits using string conversion."""
    s: str = str(n)
    total: int = 0
    i: int = 0
    while i < len(s):
        c: str = s[i]
        if c >= "0" and c <= "9":
            total = total + ord(c) - ord("0")
        i = i + 1
    return total


def test_module() -> int:
    passed: int = 0
    # Test 1: binary of 10 = "1010"
    if int_to_binary_str(10) == "1010":
        passed = passed + 1
    # Test 2: binary of 0
    if int_to_binary_str(0) == "0":
        passed = passed + 1
    # Test 3: hex of 255 = "ff"
    if int_to_hex_str(255) == "ff":
        passed = passed + 1
    # Test 4: parse int
    if parse_int_from_str("42") == 42:
        passed = passed + 1
    # Test 5: parse negative
    if parse_int_from_str("-123") == 0 - 123:
        passed = passed + 1
    # Test 6: format with commas
    if format_with_commas(1234567) == "1,234,567":
        passed = passed + 1
    # Test 7: digit sum
    if digit_sum(12345) == 15:
        passed = passed + 1
    return passed
