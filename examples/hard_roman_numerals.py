"""Roman numeral encode and decode."""


def roman_to_int(s: str) -> int:
    """Convert Roman numeral string to integer."""
    result: int = 0
    prev: int = 0
    i: int = len(s) - 1
    while i >= 0:
        val: int = 0
        ch: str = s[i]
        if ch == "I":
            val = 1
        elif ch == "V":
            val = 5
        elif ch == "X":
            val = 10
        elif ch == "L":
            val = 50
        elif ch == "C":
            val = 100
        elif ch == "D":
            val = 500
        elif ch == "M":
            val = 1000
        if val < prev:
            result = result - val
        else:
            result = result + val
        prev = val
        i = i - 1
    return result


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


def is_valid_roman(s: str) -> int:
    """Check if string is a valid Roman numeral. Returns 1/0."""
    n: int = len(s)
    if n == 0:
        return 0
    i: int = 0
    while i < n:
        ch: str = s[i]
        if ch != "I" and ch != "V" and ch != "X" and ch != "L" and ch != "C" and ch != "D" and ch != "M":
            return 0
        i = i + 1
    converted: int = roman_to_int(s)
    roundtrip: str = int_to_roman(converted)
    if roundtrip == s:
        return 1
    return 0


def test_module() -> int:
    passed: int = 0

    if roman_to_int("III") == 3:
        passed = passed + 1

    if roman_to_int("IV") == 4:
        passed = passed + 1

    if roman_to_int("MCMXCIV") == 1994:
        passed = passed + 1

    if int_to_roman(58) == "LVIII":
        passed = passed + 1

    if int_to_roman(1994) == "MCMXCIV":
        passed = passed + 1

    if is_valid_roman("XIV") == 1:
        passed = passed + 1

    if roman_to_int("CDXLIV") == 444:
        passed = passed + 1

    if int_to_roman(3999) == "MMMCMXCIX":
        passed = passed + 1

    return passed
