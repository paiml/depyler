# While loop patterns for transpiler stress testing
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def collatz_steps(n: int) -> int:
    """Count steps to reach 1 in the Collatz sequence."""
    if n <= 0:
        return 0
    steps: int = 0
    while n != 1:
        if n % 2 == 0:
            n = n // 2
        else:
            n = 3 * n + 1
        steps = steps + 1
    return steps


def integer_sqrt(n: int) -> int:
    """Compute integer square root using Newton's method."""
    if n < 0:
        return 0
    if n == 0:
        return 0
    guess: int = n
    while guess * guess > n:
        guess = (guess + n // guess) // 2
    return guess


def find_first_above(nums: list[int], threshold: int) -> int:
    """Find index of first element above threshold, or -1."""
    i: int = 0
    while i < len(nums):
        if nums[i] > threshold:
            return i
        i = i + 1
    return -1


def convergent_sum(limit: int) -> int:
    """Sum 1/2^i * scale for i=0..limit (scaled by 1000)."""
    total: int = 0
    term: int = 1000
    i: int = 0
    while i < limit and term > 0:
        total = total + term
        term = term // 2
        i = i + 1
    return total


def count_trailing_zeros(n: int) -> int:
    """Count trailing zeros in binary representation."""
    if n == 0:
        return 0
    count: int = 0
    while n % 2 == 0:
        count = count + 1
        n = n // 2
    return count


def test_module() -> int:
    """Test all while loop pattern functions."""
    assert collatz_steps(1) == 0
    assert collatz_steps(6) == 8
    assert collatz_steps(27) == 111
    assert integer_sqrt(0) == 0
    assert integer_sqrt(1) == 1
    assert integer_sqrt(4) == 2
    assert integer_sqrt(16) == 4
    assert integer_sqrt(100) == 10
    assert integer_sqrt(99) == 9
    assert find_first_above([1, 3, 5, 7, 9], 4) == 2
    assert find_first_above([1, 2, 3], 10) == -1
    assert find_first_above([], 0) == -1
    assert convergent_sum(1) == 1000
    assert convergent_sum(10) == 1994
    assert count_trailing_zeros(8) == 3
    assert count_trailing_zeros(12) == 2
    assert count_trailing_zeros(1) == 0
    assert count_trailing_zeros(0) == 0
    return 0


if __name__ == "__main__":
    test_module()
