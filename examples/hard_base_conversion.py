def to_base10(digits: list[int], base: int) -> int:
    result: int = 0
    i: int = 0
    while i < len(digits):
        result = result * base + digits[i]
        i = i + 1
    return result


def from_base10(n: int, base: int) -> list[int]:
    if n == 0:
        return [0]
    digits: list[int] = []
    val: int = n
    while val > 0:
        digits.append(val % base)
        val = val // base
    result: list[int] = []
    i: int = len(digits) - 1
    while i >= 0:
        result.append(digits[i])
        i = i - 1
    return result


def convert_base(digits: list[int], from_base: int, to_base: int) -> list[int]:
    val: int = to_base10(digits, from_base)
    return from_base10(val, to_base)


def count_digits_in_base(n: int, base: int) -> int:
    if n == 0:
        return 1
    count: int = 0
    val: int = n
    while val > 0:
        count = count + 1
        val = val // base
    return count


def is_palindrome_in_base(n: int, base: int) -> int:
    digits: list[int] = from_base10(n, base)
    left: int = 0
    right: int = len(digits) - 1
    while left < right:
        if digits[left] != digits[right]:
            return 0
        left = left + 1
        right = right - 1
    return 1


def test_module() -> int:
    passed: int = 0
    if to_base10([1, 0, 1], 2) == 5:
        passed = passed + 1
    if from_base10(10, 2) == [1, 0, 1, 0]:
        passed = passed + 1
    if from_base10(0, 2) == [0]:
        passed = passed + 1
    if convert_base([1, 0, 1], 2, 10) == [5]:
        passed = passed + 1
    if count_digits_in_base(255, 16) == 2:
        passed = passed + 1
    if count_digits_in_base(0, 10) == 1:
        passed = passed + 1
    if is_palindrome_in_base(9, 2) == 1:
        passed = passed + 1
    if is_palindrome_in_base(10, 2) == 0:
        passed = passed + 1
    return passed
