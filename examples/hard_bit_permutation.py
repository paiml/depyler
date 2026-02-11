"""Bit permutation operations.

Implements operations that rearrange bits within integers
according to various permutation patterns.
"""


def permute_bits(value: int, perm_table: list[int], num_bits: int) -> int:
    """Permute bits of value according to permutation table.

    perm_table[i] indicates which source bit goes to position i.
    """
    result: int = 0
    i: int = 0
    while i < num_bits:
        src_pos: int = perm_table[i]
        bit: int = (value >> src_pos) & 1
        result = result | (bit << i)
        i = i + 1
    return result


def reverse_bits(value: int, num_bits: int) -> int:
    """Reverse the order of the lowest num_bits bits."""
    result: int = 0
    i: int = 0
    while i < num_bits:
        bit: int = (value >> i) & 1
        dest: int = num_bits - 1 - i
        result = result | (bit << dest)
        i = i + 1
    return result


def swap_bit_pairs(value: int, pos_a: int, pos_b: int) -> int:
    """Swap bits at positions pos_a and pos_b."""
    bit_a: int = (value >> pos_a) & 1
    bit_b: int = (value >> pos_b) & 1
    if bit_a != bit_b:
        value = value ^ (1 << pos_a)
        value = value ^ (1 << pos_b)
    return value


def rotate_bits_left(value: int, shift: int, num_bits: int) -> int:
    """Rotate the lowest num_bits bits left by shift positions."""
    mask: int = (1 << num_bits) - 1
    masked: int = value & mask
    effective: int = shift % num_bits
    rotated: int = ((masked << effective) | (masked >> (num_bits - effective))) & mask
    high_bits: int = value & (~mask)
    result: int = high_bits | rotated
    return result


def count_bit_transitions(value: int, num_bits: int) -> int:
    """Count the number of 0->1 or 1->0 transitions in bit representation."""
    transitions: int = 0
    i: int = 1
    while i < num_bits:
        prev_bit: int = (value >> (i - 1)) & 1
        curr_bit: int = (value >> i) & 1
        if prev_bit != curr_bit:
            transitions = transitions + 1
        i = i + 1
    return transitions


def test_module() -> int:
    """Test bit permutation operations."""
    ok: int = 0

    rev: int = reverse_bits(0b1100, 4)
    if rev == 0b0011:
        ok = ok + 1

    swapped: int = swap_bit_pairs(0b1010, 0, 1)
    if swapped == 0b1001:
        ok = ok + 1

    rotated: int = rotate_bits_left(0b0011, 1, 4)
    if rotated == 0b0110:
        ok = ok + 1

    trans: int = count_bit_transitions(0b1010, 4)
    if trans == 3:
        ok = ok + 1

    perm: list[int] = [3, 2, 1, 0]
    pval: int = permute_bits(0b1100, perm, 4)
    if pval == 0b0011:
        ok = ok + 1

    return ok
