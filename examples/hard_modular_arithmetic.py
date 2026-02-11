# Modular arithmetic patterns for transpiler stress testing
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def mod_add(a: int, b: int, m: int) -> int:
    """Modular addition: (a + b) % m."""
    if m <= 0:
        return 0
    return (a % m + b % m) % m


def mod_multiply(a: int, b: int, m: int) -> int:
    """Modular multiplication: (a * b) % m."""
    if m <= 0:
        return 0
    return ((a % m) * (b % m)) % m


def mod_power(base: int, exp: int, m: int) -> int:
    """Modular exponentiation: (base^exp) % m using repeated squaring."""
    if m <= 0:
        return 0
    if m == 1:
        return 0
    result: int = 1
    base = base % m
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % m
        exp = exp // 2
        base = (base * base) % m
    return result


def is_divisible(a: int, b: int) -> bool:
    """Check if a is divisible by b."""
    if b == 0:
        return False
    return a % b == 0


def sum_multiples(limit: int, factor: int) -> int:
    """Sum all multiples of factor below limit."""
    if factor <= 0:
        return 0
    total: int = 0
    i: int = factor
    while i < limit:
        total = total + i
        i = i + factor
    return total


def test_module() -> int:
    """Test all modular arithmetic functions."""
    assert mod_add(7, 5, 6) == 0
    assert mod_add(3, 4, 10) == 7
    assert mod_multiply(7, 8, 5) == 1
    assert mod_multiply(3, 4, 7) == 5
    assert mod_power(2, 10, 1000) == 24
    assert mod_power(3, 5, 13) == 9
    assert mod_power(2, 0, 5) == 1
    assert is_divisible(10, 5) == True
    assert is_divisible(10, 3) == False
    assert is_divisible(0, 5) == True
    assert is_divisible(5, 0) == False
    assert sum_multiples(10, 3) == 18
    assert sum_multiples(20, 5) == 30
    assert sum_multiples(1000, 3) == 166833
    return 0


if __name__ == "__main__":
    test_module()
