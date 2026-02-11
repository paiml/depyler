# Type conversion patterns for transpiler stress testing
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def int_to_digits(n: int) -> list[int]:
    """Convert integer to list of its digits."""
    if n == 0:
        return [0]
    if n < 0:
        n = -n
    digits: list[int] = []
    while n > 0:
        digits.append(n % 10)
        n = n // 10
    result: list[int] = []
    i: int = len(digits) - 1
    while i >= 0:
        result.append(digits[i])
        i = i - 1
    return result


def digits_to_int(digits: list[int]) -> int:
    """Convert list of digits back to integer."""
    result: int = 0
    for d in digits:
        result = result * 10 + d
    return result


def int_to_binary_str(n: int) -> str:
    """Convert non-negative integer to binary string representation."""
    if n == 0:
        return "0"
    result: str = ""
    temp: int = n
    while temp > 0:
        if temp % 2 == 0:
            result = "0" + result
        else:
            result = "1" + result
        temp = temp // 2
    return result


def count_digits(n: int) -> int:
    """Count the number of digits in an integer."""
    if n == 0:
        return 1
    if n < 0:
        n = -n
    count: int = 0
    while n > 0:
        count = count + 1
        n = n // 10
    return count


def sum_digits(n: int) -> int:
    """Sum all digits of an integer."""
    if n < 0:
        n = -n
    total: int = 0
    while n > 0:
        total = total + n % 10
        n = n // 10
    return total


def test_module() -> int:
    """Test all numeric conversion functions."""
    assert int_to_digits(123) == [1, 2, 3]
    assert int_to_digits(0) == [0]
    assert int_to_digits(9) == [9]
    assert digits_to_int([1, 2, 3]) == 123
    assert digits_to_int([0]) == 0
    assert digits_to_int([4, 5, 6, 7]) == 4567
    assert int_to_binary_str(0) == "0"
    assert int_to_binary_str(5) == "101"
    assert int_to_binary_str(10) == "1010"
    assert int_to_binary_str(255) == "11111111"
    assert count_digits(0) == 1
    assert count_digits(123) == 3
    assert count_digits(-456) == 3
    assert count_digits(1000000) == 7
    assert sum_digits(123) == 6
    assert sum_digits(999) == 27
    assert sum_digits(0) == 0
    return 0


if __name__ == "__main__":
    test_module()
