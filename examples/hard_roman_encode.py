"""Roman numeral encoding and validation using integer representation."""


def int_to_roman_value(n: int) -> str:
    """Convert integer (1-3999) to Roman numeral string."""
    result: str = ""
    while n >= 1000:
        result = result + "M"
        n = n - 1000
    while n >= 900:
        result = result + "CM"
        n = n - 900
    while n >= 500:
        result = result + "D"
        n = n - 500
    while n >= 400:
        result = result + "CD"
        n = n - 400
    while n >= 100:
        result = result + "C"
        n = n - 100
    while n >= 90:
        result = result + "XC"
        n = n - 90
    while n >= 50:
        result = result + "L"
        n = n - 50
    while n >= 40:
        result = result + "XL"
        n = n - 40
    while n >= 10:
        result = result + "X"
        n = n - 10
    while n >= 9:
        result = result + "IX"
        n = n - 9
    while n >= 5:
        result = result + "V"
        n = n - 5
    while n >= 4:
        result = result + "IV"
        n = n - 4
    while n >= 1:
        result = result + "I"
        n = n - 1
    return result


def roman_char_value(ch: str) -> int:
    """Get numeric value of a single Roman numeral character."""
    if ch == "M":
        return 1000
    if ch == "D":
        return 500
    if ch == "C":
        return 100
    if ch == "L":
        return 50
    if ch == "X":
        return 10
    if ch == "V":
        return 5
    if ch == "I":
        return 1
    return 0


def roman_to_int(roman: str) -> int:
    """Convert Roman numeral string to integer."""
    total: int = 0
    i: int = 0
    n: int = len(roman)
    while i < n:
        current: int = roman_char_value(roman[i])
        if i + 1 < n:
            next_val: int = roman_char_value(roman[i + 1])
            if current < next_val:
                total = total + next_val - current
                i = i + 2
            else:
                total = total + current
                i = i + 1
        else:
            total = total + current
            i = i + 1
    return total


def roman_numeral_length(n: int) -> int:
    """Calculate the length of the Roman numeral representation."""
    s: str = int_to_roman_value(n)
    return len(s)


def test_module() -> int:
    """Test Roman numeral encoding functions."""
    ok: int = 0

    if int_to_roman_value(1) == "I":
        ok = ok + 1

    if int_to_roman_value(4) == "IV":
        ok = ok + 1

    if int_to_roman_value(1994) == "MCMXCIV":
        ok = ok + 1

    if roman_to_int("XIV") == 14:
        ok = ok + 1

    if roman_to_int("MCMXCIV") == 1994:
        ok = ok + 1

    if roman_numeral_length(8) == 4:
        ok = ok + 1

    return ok
