"""Bit counting operations: popcount, leading/trailing zeros, bit reversal."""


def popcount(n: int) -> int:
    """Count the number of set bits in n."""
    count: int = 0
    val: int = n
    while val > 0:
        count = count + (val & 1)
        val = val >> 1
    return count


def leading_zeros_16(n: int) -> int:
    """Count leading zeros in a 16-bit representation."""
    if n == 0:
        return 16
    count: int = 0
    bit_pos: int = 15
    while bit_pos >= 0:
        mask: int = 1 << bit_pos
        if (n & mask) != 0:
            return count
        count = count + 1
        bit_pos = bit_pos - 1
    return count


def trailing_zeros(n: int) -> int:
    """Count trailing zeros in binary representation."""
    if n == 0:
        return 32
    count: int = 0
    val: int = n
    while (val & 1) == 0:
        count = count + 1
        val = val >> 1
    return count


def reverse_bits_8(n: int) -> int:
    """Reverse the bits of an 8-bit number."""
    result: int = 0
    val: int = n & 255
    bit_idx: int = 0
    while bit_idx < 8:
        result = result << 1
        result = result | (val & 1)
        val = val >> 1
        bit_idx = bit_idx + 1
    return result


def test_module() -> int:
    passed: int = 0

    if popcount(0) == 0:
        passed = passed + 1
    if popcount(7) == 3:
        passed = passed + 1
    if popcount(255) == 8:
        passed = passed + 1
    if leading_zeros_16(1) == 15:
        passed = passed + 1
    if leading_zeros_16(0) == 16:
        passed = passed + 1
    if trailing_zeros(8) == 3:
        passed = passed + 1
    if reverse_bits_8(1) == 128:
        passed = passed + 1
    if reverse_bits_8(0) == 0:
        passed = passed + 1

    return passed
