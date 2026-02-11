# Bitwise operations for transpiler stress testing
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def bitwise_and(a: int, b: int) -> int:
    """Compute bitwise AND."""
    return a & b


def bitwise_or(a: int, b: int) -> int:
    """Compute bitwise OR."""
    return a | b


def bitwise_xor(a: int, b: int) -> int:
    """Compute bitwise XOR."""
    return a ^ b


def left_shift(a: int, n: int) -> int:
    """Left shift a by n positions."""
    return a << n


def right_shift(a: int, n: int) -> int:
    """Right shift a by n positions."""
    return a >> n


def count_set_bits(n: int) -> int:
    """Count the number of set bits (1s) in binary representation."""
    if n < 0:
        return 0
    count: int = 0
    while n > 0:
        count = count + (n & 1)
        n = n >> 1
    return count


def is_power_of_two(n: int) -> bool:
    """Check if n is a power of two."""
    if n <= 0:
        return False
    return (n & (n - 1)) == 0


def test_module() -> int:
    """Test all bitwise operation functions."""
    assert bitwise_and(12, 10) == 8
    assert bitwise_and(255, 15) == 15
    assert bitwise_or(12, 10) == 14
    assert bitwise_or(0, 0) == 0
    assert bitwise_xor(12, 10) == 6
    assert bitwise_xor(5, 5) == 0
    assert left_shift(1, 4) == 16
    assert left_shift(3, 2) == 12
    assert right_shift(16, 4) == 1
    assert right_shift(12, 2) == 3
    assert count_set_bits(0) == 0
    assert count_set_bits(7) == 3
    assert count_set_bits(255) == 8
    assert count_set_bits(1024) == 1
    assert is_power_of_two(1) == True
    assert is_power_of_two(16) == True
    assert is_power_of_two(15) == False
    assert is_power_of_two(0) == False
    return 0


if __name__ == "__main__":
    test_module()
