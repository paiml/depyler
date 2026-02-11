"""Happy number detection and sequence analysis."""


def digit_square_sum(n: int) -> int:
    """Compute sum of squares of digits of n."""
    total: int = 0
    val: int = n
    if val < 0:
        val = -val
    while val > 0:
        digit: int = val % 10
        total = total + digit * digit
        val = val // 10
    return total


def is_happy(n: int) -> int:
    """Check if n is a happy number. Returns 1 if yes, 0 if no."""
    slow: int = n
    fast: int = n
    slow = digit_square_sum(slow)
    fast = digit_square_sum(digit_square_sum(fast))
    while slow != fast:
        slow = digit_square_sum(slow)
        fast = digit_square_sum(digit_square_sum(fast))
    if slow == 1:
        return 1
    return 0


def happy_sequence(n: int, steps: int) -> list[int]:
    """Generate the happy number sequence for n up to steps iterations."""
    result: list[int] = [n]
    current: int = n
    i: int = 0
    while i < steps:
        current = digit_square_sum(current)
        result.append(current)
        if current == 1:
            break
        i = i + 1
    return result


def count_happy_in_range(low: int, high: int) -> int:
    """Count happy numbers in range [low, high]."""
    count: int = 0
    current: int = low
    while current <= high:
        if is_happy(current) == 1:
            count = count + 1
        current = current + 1
    return count


def test_module() -> int:
    """Test happy number operations."""
    passed: int = 0

    if digit_square_sum(19) == 82:
        passed = passed + 1

    if is_happy(1) == 1:
        passed = passed + 1

    if is_happy(7) == 1:
        passed = passed + 1

    if is_happy(2) == 0:
        passed = passed + 1

    seq: list[int] = happy_sequence(7, 10)
    if seq[0] == 7:
        passed = passed + 1

    last_idx: int = len(seq) - 1
    if seq[last_idx] == 1:
        passed = passed + 1

    if count_happy_in_range(1, 10) == 3:
        passed = passed + 1

    if digit_square_sum(100) == 1:
        passed = passed + 1

    return passed
