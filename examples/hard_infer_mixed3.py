# Type inference test: Return type inference from body
# Strategy: parameter types given, return types inferred from function body


def double_val(x: int):
    """Return type inferred from x * 2."""
    return x * 2


def half_val(x: int):
    """Return type inferred from integer division."""
    return x // 2


def remainder(x: int, y: int):
    """Return type inferred from modulo."""
    if y == 0:
        return 0
    return x % y


def is_even(x: int):
    """Return type inferred from comparison to 0."""
    if x % 2 == 0:
        return 1
    return 0


def is_odd(x: int):
    """Return type inferred from complement of is_even."""
    if is_even(x) == 1:
        return 0
    return 1


def triangle_number(n: int):
    """Return n-th triangle number. Type inferred from arithmetic."""
    return n * (n + 1) // 2


def sum_range(start: int, end: int):
    """Sum integers from start to end inclusive."""
    total = 0
    i = start
    while i <= end:
        total = total + i
        i = i + 1
    return total


def count_set_bits(n: int):
    """Count set bits (population count). Return type inferred."""
    count = 0
    val = n
    if val < 0:
        val = 0 - val
    while val > 0:
        count = count + (val & 1)
        val = val >> 1
    return count


def collatz_steps(n: int):
    """Count Collatz sequence steps to reach 1."""
    if n <= 1:
        return 0
    steps = 0
    val = n
    while val != 1:
        if val % 2 == 0:
            val = val // 2
        else:
            val = 3 * val + 1
        steps = steps + 1
        if steps > 1000:
            return 0 - 1
    return steps


def digital_root(n: int):
    """Compute digital root (repeated digit sum until single digit)."""
    val = n
    if val < 0:
        val = 0 - val
    while val >= 10:
        s = 0
        while val > 0:
            s = s + val % 10
            val = val // 10
        val = s
    return val


def test_module() -> int:
    """Test return type inference from body."""
    total: int = 0

    # double_val tests
    if double_val(5) == 10:
        total = total + 1
    if double_val(0) == 0:
        total = total + 1

    # half_val tests
    if half_val(10) == 5:
        total = total + 1
    if half_val(7) == 3:
        total = total + 1

    # remainder tests
    if remainder(10, 3) == 1:
        total = total + 1
    if remainder(10, 0) == 0:
        total = total + 1

    # is_even / is_odd tests
    if is_even(4) == 1:
        total = total + 1
    if is_odd(3) == 1:
        total = total + 1

    # triangle_number tests
    if triangle_number(10) == 55:
        total = total + 1

    # sum_range tests
    if sum_range(1, 10) == 55:
        total = total + 1
    if sum_range(5, 5) == 5:
        total = total + 1

    # count_set_bits tests
    if count_set_bits(7) == 3:
        total = total + 1
    if count_set_bits(0) == 0:
        total = total + 1

    # collatz_steps tests
    if collatz_steps(1) == 0:
        total = total + 1
    if collatz_steps(6) == 8:
        total = total + 1

    # digital_root tests
    if digital_root(9999) == 9:
        total = total + 1
    if digital_root(0) == 0:
        total = total + 1

    return total
