"""Number base conversion (bases 2-16).

Tests: decimal to base, base to decimal, binary/hex string operations.
"""


def decimal_to_base(num: int, base: int) -> str:
    """Convert decimal to string in given base (2-16)."""
    if num == 0:
        return "0"
    digits: str = "0123456789abcdef"
    result: str = ""
    negative: int = 0
    n: int = num
    if n < 0:
        negative = 1
        n = -n
    while n > 0:
        remainder: int = n % base
        result = digits[remainder] + result
        n = n // base
    if negative == 1:
        result = "-" + result
    return result


def base_to_decimal(s: str, base: int) -> int:
    """Convert string in given base to decimal."""
    result: int = 0
    start: int = 0
    negative: int = 0
    if len(s) > 0 and s[0] == "-":
        negative = 1
        start = 1
    i: int = start
    while i < len(s):
        c: str = s[i]
        val: int = 0
        if c >= "0" and c <= "9":
            val = ord(c) - ord("0")
        elif c >= "a" and c <= "f":
            val = ord(c) - ord("a") + 10
        elif c >= "A" and c <= "F":
            val = ord(c) - ord("A") + 10
        result = result * base + val
        i = i + 1
    if negative == 1:
        result = -result
    return result


def count_ones_binary(n: int) -> int:
    """Count number of 1s in binary representation."""
    count: int = 0
    val: int = n
    if val < 0:
        val = -val
    while val > 0:
        if val % 2 == 1:
            count = count + 1
        val = val // 2
    return count


def is_power_of_base_val(n: int, base: int) -> int:
    """Check if n is a power of given base. Returns 1 if yes, 0 if no."""
    if n <= 0 or base <= 1:
        return 0
    val: int = n
    while val > 1:
        if val % base != 0:
            return 0
        val = val // base
    return 1


def digit_sum_in_base(n: int, base: int) -> int:
    """Sum of digits when n is represented in given base."""
    total: int = 0
    val: int = n
    if val < 0:
        val = -val
    while val > 0:
        total = total + val % base
        val = val // base
    return total


def test_module() -> None:
    assert decimal_to_base(10, 2) == "1010"
    assert decimal_to_base(255, 16) == "ff"
    assert decimal_to_base(0, 10) == "0"
    assert decimal_to_base(7, 8) == "7"
    assert decimal_to_base(8, 8) == "10"
    assert base_to_decimal("1010", 2) == 10
    assert base_to_decimal("ff", 16) == 255
    assert base_to_decimal("10", 8) == 8
    assert count_ones_binary(7) == 3
    assert count_ones_binary(8) == 1
    assert count_ones_binary(0) == 0
    assert is_power_of_base_val(8, 2) == 1
    assert is_power_of_base_val(27, 3) == 1
    assert is_power_of_base_val(10, 2) == 0
    assert digit_sum_in_base(255, 16) == 30
    assert digit_sum_in_base(10, 2) == 2
