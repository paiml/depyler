"""Sum of divisors and sigma function operations.

Tests: sum of divisors, proper divisors, aliquot sum.
"""


def sum_of_divisors(n: int) -> int:
    """Compute sum of all divisors of n (including n)."""
    if n <= 0:
        return 0
    total: int = 0
    d: int = 1
    while d * d <= n:
        if n % d == 0:
            total = total + d
            if d != n // d:
                total = total + n // d
        d = d + 1
    return total


def sum_of_proper_divisors(n: int) -> int:
    """Sum of proper divisors (excluding n itself)."""
    return sum_of_divisors(n) - n


def count_divisors(n: int) -> int:
    """Count number of divisors of n."""
    if n <= 0:
        return 0
    count: int = 0
    d: int = 1
    while d * d <= n:
        if n % d == 0:
            count = count + 1
            if d != n // d:
                count = count + 1
        d = d + 1
    return count


def is_perfect_number(n: int) -> int:
    """Check if n is a perfect number. Returns 1 if yes."""
    if n <= 1:
        return 0
    if sum_of_proper_divisors(n) == n:
        return 1
    return 0


def aliquot_sum_steps(n: int, max_steps: int) -> int:
    """Count steps in aliquot sequence to reach 0 or cycle.
    Returns number of steps or max_steps if not terminated."""
    current: int = n
    step: int = 0
    while step < max_steps:
        if current <= 0:
            return step
        current = sum_of_proper_divisors(current)
        step = step + 1
    return max_steps


def test_module() -> int:
    """Test divisor operations."""
    ok: int = 0
    if sum_of_divisors(12) == 28:
        ok = ok + 1
    if sum_of_proper_divisors(12) == 16:
        ok = ok + 1
    if count_divisors(12) == 6:
        ok = ok + 1
    if is_perfect_number(6) == 1:
        ok = ok + 1
    if is_perfect_number(28) == 1:
        ok = ok + 1
    if is_perfect_number(12) == 0:
        ok = ok + 1
    return ok
