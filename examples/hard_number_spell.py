"""Number to digit-word conversion.

Tests: single digit spelling, multi-digit, digit sum spelled, reverse digits spelled.
"""


def digit_to_word(d: int) -> str:
    """Convert single digit 0-9 to English word."""
    if d == 0:
        return "zero"
    if d == 1:
        return "one"
    if d == 2:
        return "two"
    if d == 3:
        return "three"
    if d == 4:
        return "four"
    if d == 5:
        return "five"
    if d == 6:
        return "six"
    if d == 7:
        return "seven"
    if d == 8:
        return "eight"
    if d == 9:
        return "nine"
    return "unknown"


def number_to_digit_words(n: int) -> str:
    """Convert number to space-separated digit words."""
    if n == 0:
        return "zero"
    val: int = n
    if val < 0:
        val = -val
    digits: list[int] = []
    while val > 0:
        digits.append(val % 10)
        val = val // 10
    result: str = ""
    i: int = len(digits) - 1
    while i >= 0:
        if i < len(digits) - 1:
            result = result + " "
        result = result + digit_to_word(digits[i])
        i = i - 1
    return result


def count_digit_letters(n: int) -> int:
    """Count total letters in digit-word representation."""
    words: str = number_to_digit_words(n)
    count: int = 0
    i: int = 0
    while i < len(words):
        if words[i] != " ":
            count = count + 1
        i = i + 1
    return count


def digit_word_length_sum(n: int) -> int:
    """Sum of lengths of each digit word for a number."""
    val: int = n
    if val < 0:
        val = -val
    if val == 0:
        return 4
    total: int = 0
    while val > 0:
        d: int = val % 10
        w: str = digit_to_word(d)
        total = total + len(w)
        val = val // 10
    return total


def test_module() -> int:
    """Test number spelling operations."""
    ok: int = 0
    if digit_to_word(5) == "five":
        ok = ok + 1
    if digit_to_word(0) == "zero":
        ok = ok + 1
    if number_to_digit_words(42) == "four two":
        ok = ok + 1
    if number_to_digit_words(0) == "zero":
        ok = ok + 1
    if count_digit_letters(12) == 6:
        ok = ok + 1
    if digit_word_length_sum(10) == 7:
        ok = ok + 1
    return ok
