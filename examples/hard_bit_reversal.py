"""Bit reversal and mirroring operations.

Tests: reverse bits, mirror bits, count trailing zeros, highest bit, bit palindrome.
"""


def reverse_bits(n: int, width: int) -> int:
    """Reverse the lowest 'width' bits of n."""
    result: int = 0
    val: int = n
    i: int = 0
    while i < width:
        result = result * 2 + (val % 2)
        val = val // 2
        i = i + 1
    return result


def count_trailing_zeros(n: int) -> int:
    """Count trailing zero bits."""
    if n == 0:
        return 0
    count: int = 0
    val: int = n
    while val % 2 == 0:
        count = count + 1
        val = val // 2
    return count


def highest_set_bit(n: int) -> int:
    """Position of highest set bit (0-indexed). Returns -1 for 0."""
    if n == 0:
        return -1
    pos: int = 0
    val: int = n
    while val > 1:
        val = val // 2
        pos = pos + 1
    return pos


def is_bit_palindrome(n: int) -> int:
    """Check if binary representation is a palindrome. Returns 1 or 0."""
    if n == 0:
        return 1
    width: int = highest_set_bit(n) + 1
    rev: int = reverse_bits(n, width)
    if rev == n:
        return 1
    return 0


def swap_adjacent_bits(n: int) -> int:
    """Swap adjacent bit pairs by extracting and shifting."""
    result: int = 0
    val: int = n
    pos: int = 0
    while val > 0:
        bit0: int = val % 2
        val = val // 2
        bit1: int = val % 2
        val = val // 2
        shift: int = 1
        k: int = 0
        while k < pos:
            shift = shift * 2
            k = k + 1
        result = result + bit1 * shift
        result = result + bit0 * shift * 2
        pos = pos + 2
    return result


def test_module() -> int:
    """Test bit reversal operations."""
    ok: int = 0
    if reverse_bits(13, 4) == 11:
        ok = ok + 1
    if reverse_bits(1, 8) == 128:
        ok = ok + 1
    if count_trailing_zeros(12) == 2:
        ok = ok + 1
    if count_trailing_zeros(1) == 0:
        ok = ok + 1
    if highest_set_bit(8) == 3:
        ok = ok + 1
    if is_bit_palindrome(9) == 1:
        ok = ok + 1
    if is_bit_palindrome(5) == 1:
        ok = ok + 1
    return ok
