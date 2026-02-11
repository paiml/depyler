# Data conversion patterns (base conversion, encoding)
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def int_to_binary_str(n: int) -> str:
    """Convert a non-negative integer to its binary string representation."""
    if n == 0:
        return "0"
    result: str = ""
    val: int = n
    while val > 0:
        if val % 2 == 1:
            result = "1" + result
        else:
            result = "0" + result
        val = val // 2
    return result


def binary_str_to_int(s: str) -> int:
    """Convert a binary string to an integer."""
    result: int = 0
    i: int = 0
    while i < len(s):
        result = result * 2
        if s[i] == "1":
            result = result + 1
        i = i + 1
    return result


def celsius_to_fahrenheit(c: int) -> int:
    """Convert Celsius to Fahrenheit (integer approximation)."""
    return (c * 9) // 5 + 32


def fahrenheit_to_celsius(f: int) -> int:
    """Convert Fahrenheit to Celsius (integer approximation)."""
    return ((f - 32) * 5) // 9


def digit_sum(n: int) -> int:
    """Sum the digits of a non-negative integer."""
    total: int = 0
    val: int = n
    if val < 0:
        val = 0 - val
    while val > 0:
        total = total + val % 10
        val = val // 10
    return total


def test_module() -> int:
    assert int_to_binary_str(0) == "0"
    assert int_to_binary_str(5) == "101"
    assert int_to_binary_str(10) == "1010"
    assert int_to_binary_str(255) == "11111111"
    assert binary_str_to_int("0") == 0
    assert binary_str_to_int("101") == 5
    assert binary_str_to_int("1010") == 10
    assert celsius_to_fahrenheit(0) == 32
    assert celsius_to_fahrenheit(100) == 212
    assert fahrenheit_to_celsius(32) == 0
    assert fahrenheit_to_celsius(212) == 100
    assert digit_sum(123) == 6
    assert digit_sum(9999) == 36
    assert digit_sum(0) == 0
    return 0


if __name__ == "__main__":
    test_module()
