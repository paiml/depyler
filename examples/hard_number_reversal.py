"""Number digit reversal operations.

Tests: reverse digits, palindrome check, reverse and add.
"""


def reverse_number(n: int) -> int:
    """Reverse the digits of a non-negative integer."""
    if n < 0:
        return -reverse_number(-n)
    reversed_num: int = 0
    remaining: int = n
    while remaining > 0:
        reversed_num = reversed_num * 10 + remaining % 10
        remaining = remaining // 10
    return reversed_num


def is_palindrome_number(n: int) -> int:
    """Check if number is a palindrome. Returns 1 if yes."""
    if n < 0:
        return 0
    if n == reverse_number(n):
        return 1
    return 0


def reverse_add_steps(n: int, max_steps: int) -> int:
    """Count steps of reverse-and-add to reach palindrome.
    Returns -1 if not reached within max_steps."""
    current: int = n
    step: int = 0
    while step < max_steps:
        if is_palindrome_number(current) == 1:
            if step > 0:
                return step
        current = current + reverse_number(current)
        step = step + 1
    return -1


def digit_count(n: int) -> int:
    """Count number of digits."""
    if n == 0:
        return 1
    count: int = 0
    val: int = n
    if val < 0:
        val = -val
    while val > 0:
        count = count + 1
        val = val // 10
    return count


def test_module() -> int:
    """Test number reversal operations."""
    ok: int = 0
    if reverse_number(123) == 321:
        ok = ok + 1
    if reverse_number(1200) == 21:
        ok = ok + 1
    if is_palindrome_number(121) == 1:
        ok = ok + 1
    if is_palindrome_number(123) == 0:
        ok = ok + 1
    if digit_count(12345) == 5:
        ok = ok + 1
    if digit_count(0) == 1:
        ok = ok + 1
    return ok
