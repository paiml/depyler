"""Large integer arithmetic near overflow boundaries for i64."""


def safe_add(a: int, b: int) -> int:
    """Add two integers with overflow check."""
    max_val: int = 2147483647
    min_val: int = 0 - 2147483647
    if b > 0 and a > max_val - b:
        return max_val
    if b < 0 and a < min_val - b:
        return min_val
    return a + b


def safe_multiply(a: int, b: int) -> int:
    """Multiply two integers with overflow check."""
    max_val: int = 2147483647
    if a == 0 or b == 0:
        return 0
    result: int = a * b
    check: int = result // a
    if check != b:
        return max_val
    return result


def power_mod(base_val: int, exp: int, mod_val: int) -> int:
    """Compute base_val^exp mod mod_val using fast exponentiation."""
    if mod_val == 1:
        return 0
    result: int = 1
    b: int = base_val % mod_val
    e: int = exp
    while e > 0:
        if (e & 1) == 1:
            result = (result * b) % mod_val
        e = e >> 1
        b = (b * b) % mod_val
    return result


def large_factorial_mod(n: int, mod_val: int) -> int:
    """Compute n! mod mod_val."""
    result: int = 1
    i: int = 2
    while i <= n:
        result = (result * i) % mod_val
        i = i + 1
    return result


def fibonacci_mod(n: int, mod_val: int) -> int:
    """Compute nth Fibonacci number mod mod_val."""
    if n <= 0:
        return 0
    if n == 1:
        return 1
    a: int = 0
    b: int = 1
    i: int = 2
    while i <= n:
        temp: int = (a + b) % mod_val
        a = b
        b = temp
        i = i + 1
    return b


def sum_of_squares(n: int) -> int:
    """Sum of squares 1^2 + 2^2 + ... + n^2 using formula."""
    return (n * (n + 1) * (2 * n + 1)) // 6


def digit_sum_repeated(n: int) -> int:
    """Repeatedly sum digits until single digit."""
    val: int = n
    if val < 0:
        val = 0 - val
    while val >= 10:
        total: int = 0
        temp: int = val
        while temp > 0:
            total = total + (temp % 10)
            temp = temp // 10
        val = total
    return val


def multiply_by_repeated_add(a: int, b: int) -> int:
    """Multiply using repeated addition for small values."""
    if a == 0 or b == 0:
        return 0
    result: int = 0
    multiplier: int = b
    if multiplier < 0:
        multiplier = 0 - multiplier
    i: int = 0
    while i < multiplier:
        result = result + a
        i = i + 1
    if b < 0:
        result = 0 - result
    return result


def test_module() -> int:
    """Test all overflow edge case functions."""
    passed: int = 0
    r1: int = safe_add(2147483647, 1)
    if r1 == 2147483647:
        passed = passed + 1
    r2: int = safe_add(100, 200)
    if r2 == 300:
        passed = passed + 1
    r3: int = safe_multiply(0, 999999)
    if r3 == 0:
        passed = passed + 1
    r4: int = power_mod(2, 10, 1000)
    if r4 == 24:
        passed = passed + 1
    r5: int = large_factorial_mod(10, 1000000007)
    if r5 == 3628800:
        passed = passed + 1
    r6: int = fibonacci_mod(10, 1000000007)
    if r6 == 55:
        passed = passed + 1
    r7: int = sum_of_squares(10)
    if r7 == 385:
        passed = passed + 1
    r8: int = digit_sum_repeated(9999)
    if r8 == 9:
        passed = passed + 1
    r9: int = digit_sum_repeated(0)
    if r9 == 0:
        passed = passed + 1
    r10: int = multiply_by_repeated_add(5, 7)
    if r10 == 35:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
