"""Perfect, abundant, and deficient number classification."""


def sum_of_divisors(n: int) -> int:
    """Compute the sum of proper divisors of n."""
    if n <= 1:
        return 0
    total: int = 1
    i: int = 2
    while i * i <= n:
        if n % i == 0:
            total = total + i
            other: int = n // i
            if other != i:
                total = total + other
        i = i + 1
    return total


def classify_number(n: int) -> int:
    """Classify number: -1=deficient, 0=perfect, 1=abundant."""
    if n <= 1:
        return -1
    s: int = sum_of_divisors(n)
    if s == n:
        return 0
    elif s > n:
        return 1
    return -1


def count_perfect_in_range(low: int, high: int) -> int:
    """Count perfect numbers in range [low, high]."""
    count: int = 0
    current: int = low
    while current <= high:
        if classify_number(current) == 0:
            count = count + 1
        current = current + 1
    return count


def get_divisors(n: int) -> list[int]:
    """Get all proper divisors of n."""
    if n <= 1:
        return []
    divs: list[int] = [1]
    i: int = 2
    while i * i <= n:
        if n % i == 0:
            divs.append(i)
            other: int = n // i
            if other != i:
                divs.append(other)
        i = i + 1
    return divs


def test_module() -> int:
    """Test perfect number operations."""
    passed: int = 0

    if sum_of_divisors(6) == 6:
        passed = passed + 1

    if sum_of_divisors(28) == 28:
        passed = passed + 1

    if classify_number(6) == 0:
        passed = passed + 1

    if classify_number(12) == 1:
        passed = passed + 1

    if classify_number(7) == -1:
        passed = passed + 1

    if count_perfect_in_range(1, 30) == 2:
        passed = passed + 1

    divs: list[int] = get_divisors(12)
    if len(divs) == 5:
        passed = passed + 1

    if sum_of_divisors(1) == 0:
        passed = passed + 1

    return passed
