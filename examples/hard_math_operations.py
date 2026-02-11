# Advanced arithmetic operations for transpiler stress testing
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def power_iterative(base: int, exp: int) -> int:
    """Compute base^exp iteratively."""
    if exp < 0:
        return 0
    result: int = 1
    i: int = 0
    while i < exp:
        result = result * base
        i = i + 1
    return result


def abs_value(x: int) -> int:
    """Return the absolute value of x."""
    if x < 0:
        return -x
    return x


def floor_div_mod(a: int, b: int) -> int:
    """Return floor division result plus remainder combined."""
    if b == 0:
        return 0
    quotient: int = a // b
    remainder: int = a % b
    return quotient * 1000 + remainder


def sum_of_powers(n: int, p: int) -> int:
    """Compute sum of i^p for i from 1 to n."""
    total: int = 0
    i: int = 1
    while i <= n:
        total = total + power_iterative(i, p)
        i = i + 1
    return total


def digital_root(n: int) -> int:
    """Compute the digital root of n (repeated digit sum until single digit)."""
    if n < 0:
        n = -n
    while n >= 10:
        total: int = 0
        temp: int = n
        while temp > 0:
            total = total + temp % 10
            temp = temp // 10
        n = total
    return n


def test_module() -> int:
    """Test all math operations."""
    assert power_iterative(2, 10) == 1024
    assert power_iterative(3, 0) == 1
    assert power_iterative(5, 3) == 125
    assert abs_value(-42) == 42
    assert abs_value(42) == 42
    assert abs_value(0) == 0
    assert floor_div_mod(17, 5) == 3002
    assert floor_div_mod(10, 3) == 3001
    assert sum_of_powers(3, 2) == 14
    assert sum_of_powers(4, 1) == 10
    assert digital_root(493) == 7
    assert digital_root(9) == 9
    assert digital_root(0) == 0
    return 0


if __name__ == "__main__":
    test_module()
