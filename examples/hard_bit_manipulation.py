"""Advanced bit manipulation patterns.

Tests: popcount, power of two check, single number XOR, bit reversal,
and hamming distance computation.
"""


def popcount(n: int) -> int:
    """Count number of set bits (population count)."""
    count: int = 0
    val: int = n
    while val > 0:
        count = count + (val & 1)
        val = val >> 1
    return count


def is_power_of_two(n: int) -> bool:
    """Check if n is a power of two."""
    if n <= 0:
        return False
    return (n & (n - 1)) == 0


def single_number(nums: list[int]) -> int:
    """Find the element appearing once (all others appear twice) using XOR."""
    result: int = 0
    i: int = 0
    while i < len(nums):
        result = result ^ nums[i]
        i = i + 1
    return result


def reverse_bits(n: int, bit_width: int) -> int:
    """Reverse the bits of n within given bit width."""
    result: int = 0
    val: int = n
    i: int = 0
    while i < bit_width:
        result = (result << 1) | (val & 1)
        val = val >> 1
        i = i + 1
    return result


def hamming_distance_bits(a: int, b: int) -> int:
    """Hamming distance between two integers (number of differing bits)."""
    xor: int = a ^ b
    return popcount(xor)


def gray_code(n: int) -> list[int]:
    """Generate n-bit Gray code sequence."""
    if n <= 0:
        return [0]
    count: int = 1 << n
    result: list[int] = []
    i: int = 0
    while i < count:
        result.append(i ^ (i >> 1))
        i = i + 1
    return result


def count_bits_range(n: int) -> list[int]:
    """For each number 0..n, count number of 1 bits."""
    result: list[int] = [0] * (n + 1)
    i: int = 1
    while i <= n:
        result[i] = result[i >> 1] + (i & 1)
        i = i + 1
    return result


def test_module() -> bool:
    """Test all bit manipulation functions."""
    ok: bool = True

    if popcount(0) != 0:
        ok = False
    if popcount(7) != 3:
        ok = False
    if popcount(255) != 8:
        ok = False

    if not is_power_of_two(1):
        ok = False
    if not is_power_of_two(16):
        ok = False
    if is_power_of_two(6):
        ok = False
    if is_power_of_two(0):
        ok = False

    if single_number([2, 1, 4, 5, 2, 4, 1]) != 5:
        ok = False

    if reverse_bits(6, 4) != 6:
        ok = False
    if reverse_bits(1, 4) != 8:
        ok = False

    if hamming_distance_bits(1, 4) != 3:
        ok = False

    gc: list[int] = gray_code(3)
    if len(gc) != 8:
        ok = False
    if gc[0] != 0:
        ok = False
    if gc[1] != 1:
        ok = False

    bits: list[int] = count_bits_range(5)
    if bits != [0, 1, 1, 2, 1, 2]:
        ok = False

    return ok
