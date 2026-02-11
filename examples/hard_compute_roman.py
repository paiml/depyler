"""Roman numeral conversion: encode and decode.

Tests: int_to_roman, roman_to_int, is_valid_roman, round_trip.
"""


def int_to_roman(num: int) -> str:
    """Convert integer (1-3999) to Roman numeral string."""
    result: str = ""
    val: int = num
    while val >= 1000:
        result = result + "M"
        val = val - 1000
    while val >= 900:
        result = result + "CM"
        val = val - 900
    while val >= 500:
        result = result + "D"
        val = val - 500
    while val >= 400:
        result = result + "CD"
        val = val - 400
    while val >= 100:
        result = result + "C"
        val = val - 100
    while val >= 90:
        result = result + "XC"
        val = val - 90
    while val >= 50:
        result = result + "L"
        val = val - 50
    while val >= 40:
        result = result + "XL"
        val = val - 40
    while val >= 10:
        result = result + "X"
        val = val - 10
    while val >= 9:
        result = result + "IX"
        val = val - 9
    while val >= 5:
        result = result + "V"
        val = val - 5
    while val >= 4:
        result = result + "IV"
        val = val - 4
    while val >= 1:
        result = result + "I"
        val = val - 1
    return result


def roman_char_value(ch: str) -> int:
    """Get numeric value of a single Roman numeral character."""
    if ch == "I":
        return 1
    if ch == "V":
        return 5
    if ch == "X":
        return 10
    if ch == "L":
        return 50
    if ch == "C":
        return 100
    if ch == "D":
        return 500
    if ch == "M":
        return 1000
    return 0


def roman_to_int(s: str) -> int:
    """Convert Roman numeral string to integer."""
    n: int = len(s)
    if n == 0:
        return 0
    total: int = 0
    i: int = 0
    while i < n:
        curr: int = roman_char_value(s[i])
        if i + 1 < n:
            nxt: int = roman_char_value(s[i + 1])
            if curr < nxt:
                total = total + nxt - curr
                i = i + 2
            else:
                total = total + curr
                i = i + 1
        else:
            total = total + curr
            i = i + 1
    return total


def is_valid_roman(s: str) -> int:
    """Check if string contains only valid Roman numeral characters. Returns 1 or 0."""
    n: int = len(s)
    if n == 0:
        return 0
    i: int = 0
    while i < n:
        v: int = roman_char_value(s[i])
        if v == 0:
            return 0
        i = i + 1
    return 1


def roman_compare(a: str, b: str) -> int:
    """Compare two Roman numerals. Returns -1, 0, or 1."""
    va: int = roman_to_int(a)
    vb: int = roman_to_int(b)
    if va < vb:
        return -1
    if va > vb:
        return 1
    return 0


def roman_add(a: str, b: str) -> str:
    """Add two Roman numerals, return result as Roman."""
    va: int = roman_to_int(a)
    vb: int = roman_to_int(b)
    return int_to_roman(va + vb)


def test_module() -> int:
    """Test Roman numeral operations."""
    passed: int = 0

    if int_to_roman(3) == "III":
        passed = passed + 1

    if int_to_roman(58) == "LVIII":
        passed = passed + 1

    if int_to_roman(1994) == "MCMXCIV":
        passed = passed + 1

    if roman_to_int("III") == 3:
        passed = passed + 1

    if roman_to_int("MCMXCIV") == 1994:
        passed = passed + 1

    rt: int = roman_to_int(int_to_roman(2024))
    if rt == 2024:
        passed = passed + 1

    if is_valid_roman("XIV") == 1:
        passed = passed + 1

    sum_r: str = roman_add("X", "V")
    if sum_r == "XV":
        passed = passed + 1

    return passed
