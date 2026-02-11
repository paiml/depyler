"""Linear Feedback Shift Register (LFSR) for pseudo-random bit generation.

Implements a 16-bit LFSR with configurable taps. Used in stream ciphers
and pseudo-random number generation.
"""


def lfsr_step(state: int, tap1: int, tap2: int) -> int:
    """Single LFSR step. Taps are bit positions (0-indexed from LSB).

    Feedback = XOR of tap bits. New bit shifts in from MSB (bit 15).
    """
    bit_at_tap1: int = (state // (1 * pow2(tap1))) % 2
    bit_at_tap2: int = (state // (1 * pow2(tap2))) % 2
    new_bit: int = 0
    if bit_at_tap1 != bit_at_tap2:
        new_bit = 1
    state = state // 2
    state = state + new_bit * 32768
    return state


def pow2(n: int) -> int:
    """Compute 2^n."""
    result: int = 1
    i: int = 0
    while i < n:
        result = result * 2
        i = i + 1
    return result


def lfsr_generate(seed: int, tap1: int, tap2: int, num_bits: int) -> list[int]:
    """Generate num_bits pseudo-random bits from LFSR."""
    bits: list[int] = []
    state: int = seed
    i: int = 0
    while i < num_bits:
        bits.append(state % 2)
        state = lfsr_step(state, tap1, tap2)
        i = i + 1
    return bits


def lfsr_period(seed: int, tap1: int, tap2: int) -> int:
    """Find period of LFSR (max 65536 steps to avoid infinite loop)."""
    initial: int = seed
    state: int = lfsr_step(seed, tap1, tap2)
    count: int = 1
    while state != initial:
        if count >= 65536:
            return count
        state = lfsr_step(state, tap1, tap2)
        count = count + 1
    return count


def bits_to_int(bits: list[int], start: int, num_bits: int) -> int:
    """Convert num_bits bits starting at start to integer."""
    result: int = 0
    i: int = 0
    while i < num_bits:
        bv: int = bits[start + i]
        if bv == 1:
            result = result + pow2(i)
        i = i + 1
    return result


def lfsr_byte_stream(seed: int, tap1: int, tap2: int, num_bytes: int) -> list[int]:
    """Generate bytes from LFSR."""
    total_bits: int = num_bytes * 8
    bits: list[int] = lfsr_generate(seed, tap1, tap2, total_bits)
    result: list[int] = []
    i: int = 0
    while i < num_bytes:
        byte_val: int = bits_to_int(bits, i * 8, 8)
        result.append(byte_val)
        i = i + 1
    return result


def test_module() -> int:
    """Test LFSR implementation."""
    ok: int = 0
    p: int = pow2(4)
    if p == 16:
        ok = ok + 1
    bits: list[int] = lfsr_generate(1, 0, 2, 10)
    if len(bits) == 10:
        ok = ok + 1
    s1: int = lfsr_step(1, 0, 2)
    if s1 != 1:
        ok = ok + 1
    bv: int = bits_to_int([1, 0, 1, 0], 0, 4)
    if bv == 5:
        ok = ok + 1
    bs: list[int] = lfsr_byte_stream(12345, 0, 2, 3)
    if len(bs) == 3:
        ok = ok + 1
    return ok
