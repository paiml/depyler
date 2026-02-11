"""Integer division edge cases: negative divisors, remainders, zero handling."""


def safe_div(a: int, b: int) -> int:
    """Safe division returning 0 on divide by zero."""
    if b == 0:
        return 0
    return a // b


def safe_mod(a: int, b: int) -> int:
    """Safe modulo returning 0 on divide by zero."""
    if b == 0:
        return 0
    return a % b


def divmod_pair(a: int, b: int) -> list[int]:
    """Return [quotient, remainder]."""
    if b == 0:
        return [0, 0]
    q: int = a // b
    r: int = a % b
    return [q, r]


def div_round_up(a: int, b: int) -> int:
    """Integer division rounding up (ceiling division for positives)."""
    if b == 0:
        return 0
    if a >= 0 and b > 0:
        return (a + b - 1) // b
    return a // b


def is_divisible(a: int, b: int) -> int:
    """Return 1 if a is evenly divisible by b."""
    if b == 0:
        return 0
    if a % b == 0:
        return 1
    return 0


def count_divisors(n: int) -> int:
    """Count the number of positive divisors of n."""
    if n <= 0:
        return 0
    count: int = 0
    i: int = 1
    while i * i <= n:
        if n % i == 0:
            count = count + 1
            if i != n // i:
                count = count + 1
        i = i + 1
    return count


def sum_divisors(n: int) -> int:
    """Sum all positive divisors of n."""
    if n <= 0:
        return 0
    total: int = 0
    i: int = 1
    while i * i <= n:
        if n % i == 0:
            total = total + i
            other: int = n // i
            if other != i:
                total = total + other
        i = i + 1
    return total


def is_perfect_number(n: int) -> int:
    """Return 1 if n is a perfect number (sum of proper divisors = n)."""
    if n <= 1:
        return 0
    s: int = sum_divisors(n) - n
    if s == n:
        return 1
    return 0


def integer_sqrt(n: int) -> int:
    """Integer square root using binary search."""
    if n < 0:
        return 0
    if n == 0:
        return 0
    lo: int = 1
    hi: int = n
    if hi > 46340:
        hi = 46340
    result: int = 0
    while lo <= hi:
        mid: int = (lo + hi) // 2
        sq: int = mid * mid
        if sq == n:
            return mid
        elif sq < n:
            result = mid
            lo = mid + 1
        else:
            hi = mid - 1
    return result


def repeated_division(n: int, d: int) -> int:
    """Count how many times n is divisible by d."""
    if d <= 1 or n <= 0:
        return 0
    count: int = 0
    val: int = n
    while val % d == 0:
        val = val // d
        count = count + 1
    return count


def test_module() -> int:
    """Test all integer division edge case functions."""
    passed: int = 0
    if safe_div(10, 3) == 3:
        passed = passed + 1
    if safe_div(10, 0) == 0:
        passed = passed + 1
    if safe_mod(10, 3) == 1:
        passed = passed + 1
    if safe_mod(10, 0) == 0:
        passed = passed + 1
    dm: list[int] = divmod_pair(17, 5)
    if dm[0] == 3:
        passed = passed + 1
    if dm[1] == 2:
        passed = passed + 1
    if div_round_up(10, 3) == 4:
        passed = passed + 1
    if div_round_up(9, 3) == 3:
        passed = passed + 1
    if is_divisible(12, 4) == 1:
        passed = passed + 1
    if is_divisible(13, 4) == 0:
        passed = passed + 1
    if count_divisors(12) == 6:
        passed = passed + 1
    if sum_divisors(12) == 28:
        passed = passed + 1
    if is_perfect_number(6) == 1:
        passed = passed + 1
    if is_perfect_number(10) == 0:
        passed = passed + 1
    if integer_sqrt(16) == 4:
        passed = passed + 1
    if integer_sqrt(15) == 3:
        passed = passed + 1
    if repeated_division(24, 2) == 3:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
