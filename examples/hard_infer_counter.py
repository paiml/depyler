# Type inference test: Counting patterns
# Strategy: Only test_module() -> int annotated, everything else inferred


def count_up_to(limit):
    """Count from 1 to limit, return sum."""
    total = 0
    i = 1
    while i <= limit:
        total = total + i
        i = i + 1
    return total


def count_divisible(n, divisor):
    """Count how many numbers from 1..n are divisible by divisor."""
    if divisor == 0:
        return 0
    count = 0
    i = 1
    while i <= n:
        if i % divisor == 0:
            count = count + 1
        i = i + 1
    return count


def count_digits(n):
    """Count number of decimal digits in n."""
    if n == 0:
        return 1
    val = n
    if val < 0:
        val = 0 - val
    count = 0
    while val > 0:
        val = val // 10
        count = count + 1
    return count


def digit_sum(n):
    """Sum of digits of n."""
    val = n
    if val < 0:
        val = 0 - val
    total = 0
    while val > 0:
        total = total + val % 10
        val = val // 10
    return total


def count_primes_simple(limit):
    """Count primes up to limit using trial division."""
    if limit < 2:
        return 0
    count = 0
    num = 2
    while num <= limit:
        is_prime_flag = 1
        divisor = 2
        while divisor * divisor <= num:
            if num % divisor == 0:
                is_prime_flag = 0
                divisor = num
            divisor = divisor + 1
        if is_prime_flag == 1:
            count = count + 1
        num = num + 1
    return count


def count_perfect_squares(low, high):
    """Count perfect squares in range [low, high]."""
    count = 0
    i = low
    while i <= high:
        root = 0
        while (root + 1) * (root + 1) <= i:
            root = root + 1
        if root * root == i:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test all counting inference functions."""
    total: int = 0

    # count_up_to tests
    if count_up_to(10) == 55:
        total = total + 1
    if count_up_to(0) == 0:
        total = total + 1
    if count_up_to(1) == 1:
        total = total + 1

    # count_divisible tests
    if count_divisible(10, 2) == 5:
        total = total + 1
    if count_divisible(10, 3) == 3:
        total = total + 1
    if count_divisible(10, 0) == 0:
        total = total + 1

    # count_digits tests
    if count_digits(0) == 1:
        total = total + 1
    if count_digits(123) == 3:
        total = total + 1
    if count_digits(99999) == 5:
        total = total + 1

    # digit_sum tests
    if digit_sum(123) == 6:
        total = total + 1
    if digit_sum(9999) == 36:
        total = total + 1

    # count_primes_simple tests
    if count_primes_simple(10) == 4:
        total = total + 1
    if count_primes_simple(1) == 0:
        total = total + 1

    # count_perfect_squares tests
    if count_perfect_squares(1, 100) == 10:
        total = total + 1
    if count_perfect_squares(5, 8) == 0:
        total = total + 1

    return total
