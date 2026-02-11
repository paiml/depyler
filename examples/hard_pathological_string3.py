# Pathological string: String building via concatenation in loops
# Tests: building complex strings character by character


def build_number_str(n: int) -> str:
    """Build string representation of number manually using repeated division."""
    if n == 0:
        return "0"
    is_negative: bool = n < 0
    val: int = n
    if is_negative == True:
        val = 0 - n
    digits: str = ""
    while val > 0:
        remainder: int = val % 10
        digit_char: str = str(remainder)
        digits = digit_char + digits
        val = val // 10
    if is_negative == True:
        return "-" + digits
    return digits


def build_csv_row(vals: list[int]) -> str:
    """Build a CSV row string from list of ints."""
    result: str = ""
    i: int = 0
    while i < len(vals):
        if i > 0:
            result = result + ","
        result = result + str(vals[i])
        i = i + 1
    return result


def build_histogram(vals: list[int], max_width: int) -> str:
    """Build text histogram: each value gets a line of '#' chars."""
    result: str = ""
    i: int = 0
    while i < len(vals):
        width: int = vals[i]
        if width > max_width:
            width = max_width
        j: int = 0
        while j < width:
            result = result + "#"
            j = j + 1
        if i < len(vals) - 1:
            result = result + "|"
        i = i + 1
    return result


def interleave_strings(s1: str, s2: str) -> str:
    """Interleave two strings character by character."""
    result: str = ""
    i: int = 0
    len1: int = len(s1)
    len2: int = len(s2)
    while i < len1 or i < len2:
        if i < len1:
            result = result + s1[i]
        if i < len2:
            result = result + s2[i]
        i = i + 1
    return result


def caesar_cipher(text: str, shift: int) -> str:
    """Simple caesar cipher for lowercase letters only."""
    result: str = ""
    i: int = 0
    while i < len(text):
        c: str = text[i]
        if c >= "a" and c <= "z":
            # Get ordinal offset from 'a'
            ord_val: int = ord(c) - ord("a")
            shifted: int = (ord_val + shift) % 26
            result = result + chr(shifted + ord("a"))
        else:
            result = result + c
        i = i + 1
    return result


def test_module() -> int:
    passed: int = 0
    # Test 1: build number str
    if build_number_str(12345) == "12345":
        passed = passed + 1
    # Test 2: negative
    if build_number_str(0 - 42) == "-42":
        passed = passed + 1
    # Test 3: CSV row
    if build_csv_row([1, 2, 3]) == "1,2,3":
        passed = passed + 1
    # Test 4: histogram
    if build_histogram([3, 2, 1], 10) == "###|##|#":
        passed = passed + 1
    # Test 5: interleave
    if interleave_strings("abc", "123") == "a1b2c3":
        passed = passed + 1
    # Test 6: caesar cipher
    if caesar_cipher("abc", 1) == "bcd":
        passed = passed + 1
    # Test 7: zero
    if build_number_str(0) == "0":
        passed = passed + 1
    return passed
