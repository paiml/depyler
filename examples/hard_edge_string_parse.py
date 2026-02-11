"""Parsing integers and floats from strings manually."""


def parse_int(s: str) -> int:
    """Parse an integer from string. Returns 0 on invalid input."""
    if len(s) == 0:
        return 0
    is_neg: int = 0
    start: int = 0
    if s[0] == "-":
        is_neg = 1
        start = 1
    elif s[0] == "+":
        start = 1
    result: int = 0
    i: int = start
    while i < len(s):
        code: int = ord(s[i])
        if code >= 48 and code <= 57:
            digit: int = code - 48
            result = result * 10 + digit
        else:
            return 0
        i = i + 1
    if is_neg == 1:
        return 0 - result
    return result


def parse_unsigned(s: str) -> int:
    """Parse an unsigned integer from string."""
    result: int = 0
    i: int = 0
    while i < len(s):
        code: int = ord(s[i])
        if code >= 48 and code <= 57:
            result = result * 10 + (code - 48)
        else:
            return result
        i = i + 1
    return result


def is_digit_char(ch: str) -> int:
    """Return 1 if character is a digit."""
    code: int = ord(ch)
    if code >= 48 and code <= 57:
        return 1
    return 0


def is_numeric_string(s: str) -> int:
    """Return 1 if entire string is a valid integer."""
    if len(s) == 0:
        return 0
    start: int = 0
    if s[0] == "-" or s[0] == "+":
        start = 1
    if start >= len(s):
        return 0
    i: int = start
    while i < len(s):
        code: int = ord(s[i])
        if code < 48 or code > 57:
            return 0
        i = i + 1
    return 1


def int_to_string(n: int) -> str:
    """Convert integer to string."""
    if n == 0:
        return "0"
    is_neg: int = 0
    val: int = n
    if val < 0:
        is_neg = 1
        val = 0 - val
    result: str = ""
    while val > 0:
        d: int = val % 10
        result = chr(48 + d) + result
        val = val // 10
    if is_neg == 1:
        result = "-" + result
    return result


def parse_int_list(s: str) -> list[int]:
    """Parse comma-separated integers: '1,2,3' -> [1, 2, 3]."""
    result: list[int] = []
    current: str = ""
    i: int = 0
    while i < len(s):
        ch: str = s[i]
        if ch == ",":
            if len(current) > 0:
                val: int = parse_int(current)
                result.append(val)
                current = ""
        else:
            current = current + ch
        i = i + 1
    if len(current) > 0:
        last_val: int = parse_int(current)
        result.append(last_val)
    return result


def count_digits(n: int) -> int:
    """Count number of digits in integer."""
    if n == 0:
        return 1
    val: int = n
    if val < 0:
        val = 0 - val
    count: int = 0
    while val > 0:
        val = val // 10
        count = count + 1
    return count


def extract_digits(n: int) -> list[int]:
    """Extract individual digits of integer into a list."""
    if n == 0:
        return [0]
    val: int = n
    if val < 0:
        val = 0 - val
    digits: list[int] = []
    while val > 0:
        digits.append(val % 10)
        val = val // 10
    reversed_digits: list[int] = []
    i: int = len(digits) - 1
    while i >= 0:
        reversed_digits.append(digits[i])
        i = i - 1
    return reversed_digits


def test_module() -> int:
    """Test all string parsing functions."""
    passed: int = 0
    if parse_int("12345") == 12345:
        passed = passed + 1
    if parse_int("-42") == -42:
        passed = passed + 1
    if parse_int("") == 0:
        passed = passed + 1
    if parse_int("abc") == 0:
        passed = passed + 1
    if parse_unsigned("999") == 999:
        passed = passed + 1
    if is_digit_char("5") == 1:
        passed = passed + 1
    if is_digit_char("a") == 0:
        passed = passed + 1
    if is_numeric_string("123") == 1:
        passed = passed + 1
    if is_numeric_string("12a3") == 0:
        passed = passed + 1
    if int_to_string(42) == "42":
        passed = passed + 1
    if int_to_string(0) == "0":
        passed = passed + 1
    nums: list[int] = parse_int_list("10,20,30")
    if len(nums) == 3:
        passed = passed + 1
    if nums[0] == 10:
        passed = passed + 1
    if count_digits(12345) == 5:
        passed = passed + 1
    digits: list[int] = extract_digits(123)
    if len(digits) == 3:
        passed = passed + 1
    if digits[0] == 1:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
