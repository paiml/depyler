"""Bitwise operations with edge values: AND, OR, XOR, shifts, masks."""


def bitwise_and_mask(val: int, mask: int) -> int:
    """Apply AND mask to extract bits."""
    return val & mask


def bitwise_or_combine(a: int, b: int) -> int:
    """Combine two values with OR."""
    return a | b


def bitwise_xor_toggle(val: int, bits: int) -> int:
    """Toggle specific bits using XOR."""
    return val ^ bits


def left_shift_safe(val: int, amount: int) -> int:
    """Left shift with bounds check."""
    if amount < 0:
        return val
    if amount > 30:
        return 0
    return (val << amount) & 0x7FFFFFFF


def right_shift_safe(val: int, amount: int) -> int:
    """Right shift with bounds check."""
    if amount < 0:
        return val
    if amount > 30:
        return 0
    return val >> amount


def count_trailing_zeros(n: int) -> int:
    """Count trailing zero bits."""
    if n == 0:
        return 32
    count: int = 0
    val: int = n
    while (val & 1) == 0:
        count = count + 1
        val = val >> 1
    return count


def bit_set_at(val: int, pos: int) -> int:
    """Return 1 if bit at pos is set, 0 otherwise."""
    return (val >> pos) & 1


def set_bit(val: int, pos: int) -> int:
    """Set bit at position pos."""
    return val | (1 << pos)


def clear_bit(val: int, pos: int) -> int:
    """Clear bit at position pos."""
    mask: int = ~(1 << pos)
    return val & mask


def xor_swap(a: int, b: int) -> list[int]:
    """Swap two values using XOR trick. Returns [b, a]."""
    x: int = a
    y: int = b
    x = x ^ y
    y = x ^ y
    x = x ^ y
    result: list[int] = [x, y]
    return result


def all_bits_set(width: int) -> int:
    """Create a value with width bits all set to 1."""
    if width <= 0:
        return 0
    if width >= 31:
        return 0x7FFFFFFF
    return (1 << width) - 1


def bit_reverse_8(n: int) -> int:
    """Reverse the lower 8 bits of n."""
    val: int = n & 0xFF
    result: int = 0
    i: int = 0
    while i < 8:
        result = (result << 1) | (val & 1)
        val = val >> 1
        i = i + 1
    return result


def test_module() -> int:
    """Test all bitwise edge case functions."""
    passed: int = 0
    if bitwise_and_mask(0xFF, 0x0F) == 0x0F:
        passed = passed + 1
    if bitwise_or_combine(0xF0, 0x0F) == 0xFF:
        passed = passed + 1
    if bitwise_xor_toggle(0xFF, 0x0F) == 0xF0:
        passed = passed + 1
    if left_shift_safe(1, 0) == 1:
        passed = passed + 1
    if left_shift_safe(1, 31) == 0:
        passed = passed + 1
    if right_shift_safe(256, 4) == 16:
        passed = passed + 1
    if count_trailing_zeros(0) == 32:
        passed = passed + 1
    if count_trailing_zeros(8) == 3:
        passed = passed + 1
    if bit_set_at(5, 0) == 1:
        passed = passed + 1
    if bit_set_at(5, 1) == 0:
        passed = passed + 1
    swapped: list[int] = xor_swap(42, 99)
    if swapped[0] == 99:
        passed = passed + 1
    if swapped[1] == 42:
        passed = passed + 1
    if all_bits_set(4) == 15:
        passed = passed + 1
    if bit_reverse_8(1) == 128:
        passed = passed + 1
    if bit_reverse_8(0) == 0:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
