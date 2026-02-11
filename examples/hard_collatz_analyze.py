"""Collatz sequence analysis (3n+1 problem)."""


def collatz_next(n: int) -> int:
    """Compute the next number in the Collatz sequence."""
    if n % 2 == 0:
        return n // 2
    return 3 * n + 1


def collatz_length(n: int) -> int:
    """Compute the length of the Collatz sequence starting at n."""
    if n <= 0:
        return 0
    steps: int = 0
    current: int = n
    while current != 1:
        current = collatz_next(current)
        steps = steps + 1
    return steps


def collatz_sequence(n: int) -> list[int]:
    """Generate the full Collatz sequence starting at n."""
    if n <= 0:
        return []
    result: list[int] = [n]
    current: int = n
    while current != 1:
        current = collatz_next(current)
        result.append(current)
    return result


def collatz_max_value(n: int) -> int:
    """Find the maximum value reached in the Collatz sequence starting at n."""
    if n <= 0:
        return 0
    max_val: int = n
    current: int = n
    while current != 1:
        current = collatz_next(current)
        if current > max_val:
            max_val = current
    return max_val


def longest_collatz_in_range(low: int, high: int) -> int:
    """Find the starting number with the longest Collatz sequence in range."""
    best_start: int = low
    best_len: int = 0
    current: int = low
    while current <= high:
        cl: int = collatz_length(current)
        if cl > best_len:
            best_len = cl
            best_start = current
        current = current + 1
    return best_start


def test_module() -> int:
    """Test Collatz sequence analysis."""
    passed: int = 0

    if collatz_next(6) == 3:
        passed = passed + 1

    if collatz_next(3) == 10:
        passed = passed + 1

    if collatz_length(1) == 0:
        passed = passed + 1

    if collatz_length(6) == 8:
        passed = passed + 1

    seq: list[int] = collatz_sequence(6)
    if seq[0] == 6 and seq[1] == 3:
        passed = passed + 1

    last_idx: int = len(seq) - 1
    if seq[last_idx] == 1:
        passed = passed + 1

    if collatz_max_value(6) == 16:
        passed = passed + 1

    best: int = longest_collatz_in_range(1, 10)
    if best == 9:
        passed = passed + 1

    return passed
