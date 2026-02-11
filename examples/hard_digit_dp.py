"""Digit-level operations: sum, product, reverse, palindrome check."""


def digit_sum(n: int) -> int:
    """Sum of digits of a non-negative integer."""
    if n < 0:
        n = -n
    total: int = 0
    val: int = n
    while val > 0:
        total = total + val % 10
        val = val // 10
    return total


def digit_product(n: int) -> int:
    """Product of digits of a positive integer. Returns 0 for 0."""
    if n == 0:
        return 0
    if n < 0:
        n = -n
    prod: int = 1
    val: int = n
    while val > 0:
        prod = prod * (val % 10)
        val = val // 10
    return prod


def reverse_number(n: int) -> int:
    """Reverse digits of a non-negative integer."""
    rev: int = 0
    val: int = n
    if val < 0:
        val = -val
    while val > 0:
        rev = rev * 10 + val % 10
        val = val // 10
    return rev


def is_palindrome_number(n: int) -> int:
    """Check if number reads same forwards and backwards. Returns 1/0."""
    if n < 0:
        return 0
    if n == reverse_number(n):
        return 1
    return 0


def count_digits(n: int) -> int:
    """Count number of digits in a non-negative integer."""
    if n == 0:
        return 1
    val: int = n
    if val < 0:
        val = -val
    count: int = 0
    while val > 0:
        count = count + 1
        val = val // 10
    return count


def digital_root(n: int) -> int:
    """Repeatedly sum digits until single digit."""
    val: int = n
    if val < 0:
        val = -val
    while val >= 10:
        val = digit_sum(val)
    return val


def test_module() -> int:
    passed: int = 0

    if digit_sum(12345) == 15:
        passed = passed + 1

    if digit_product(234) == 24:
        passed = passed + 1

    if reverse_number(12345) == 54321:
        passed = passed + 1

    if is_palindrome_number(12321) == 1:
        passed = passed + 1

    if is_palindrome_number(12345) == 0:
        passed = passed + 1

    if count_digits(99999) == 5:
        passed = passed + 1

    if digital_root(493) == 7:
        passed = passed + 1

    if digit_sum(0) == 0:
        passed = passed + 1

    return passed
