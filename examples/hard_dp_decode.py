"""Number decoding ways: how many ways to decode a digit string to letters."""


def decode_ways(digits: str) -> int:
    """Count ways to decode digit string where 1-26 map to A-Z."""
    n: int = len(digits)
    if n == 0:
        return 0
    if digits[0] == "0":
        return 0
    prev2: int = 1
    prev1: int = 1
    i: int = 1
    while i < n:
        curr: int = 0
        if digits[i] != "0":
            curr = prev1
        two_digit: int = int(digits[i - 1]) * 10 + int(digits[i])
        if two_digit >= 10 and two_digit <= 26:
            curr = curr + prev2
        prev2 = prev1
        prev1 = curr
        i = i + 1
    return prev1


def digit_at(s: str, pos: int) -> int:
    """Get numeric value of digit at position."""
    ch: str = s[pos]
    return int(ch)


def is_valid_encoding(digits: str) -> int:
    """Check if the digit string has at least one valid decoding. Returns 1/0."""
    ways: int = decode_ways(digits)
    if ways > 0:
        return 1
    return 0


def max_decode_value(digits: str) -> int:
    """Find the maximum single letter value achievable from any two-char window."""
    n: int = len(digits)
    if n == 0:
        return 0
    best: int = digit_at(digits, 0)
    i: int = 0
    while i < n - 1:
        two_val: int = digit_at(digits, i) * 10 + digit_at(digits, i + 1)
        if two_val >= 1 and two_val <= 26 and two_val > best:
            best = two_val
        single: int = digit_at(digits, i + 1)
        if single > best:
            best = single
        i = i + 1
    return best


def test_module() -> int:
    passed: int = 0

    if decode_ways("12") == 2:
        passed = passed + 1

    if decode_ways("226") == 3:
        passed = passed + 1

    if decode_ways("06") == 0:
        passed = passed + 1

    if decode_ways("10") == 1:
        passed = passed + 1

    if is_valid_encoding("111") == 1:
        passed = passed + 1

    if is_valid_encoding("0") == 0:
        passed = passed + 1

    if max_decode_value("1926") == 26:
        passed = passed + 1

    if decode_ways("11106") == 2:
        passed = passed + 1

    return passed
