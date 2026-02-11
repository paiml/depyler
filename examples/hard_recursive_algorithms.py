# Typed recursive functions for transpiler stress testing
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def gcd(a: int, b: int) -> int:
    """Compute greatest common divisor using Euclid's algorithm."""
    if a < 0:
        a = -a
    if b < 0:
        b = -b
    while b != 0:
        temp: int = b
        b = a % b
        a = temp
    return a


def lcm(a: int, b: int) -> int:
    """Compute least common multiple."""
    if a == 0 or b == 0:
        return 0
    g: int = gcd(a, b)
    if g == 0:
        return 0
    return (a // g) * b


def fibonacci_iterative(n: int) -> int:
    """Compute nth fibonacci number iteratively."""
    if n <= 0:
        return 0
    if n == 1:
        return 1
    a: int = 0
    b: int = 1
    i: int = 2
    while i <= n:
        temp: int = a + b
        a = b
        b = temp
        i = i + 1
    return b


def binary_search(nums: list[int], target: int) -> int:
    """Binary search returning index or -1 if not found."""
    low: int = 0
    high: int = len(nums) - 1
    while low <= high:
        mid: int = (low + high) // 2
        if nums[mid] == target:
            return mid
        elif nums[mid] < target:
            low = mid + 1
        else:
            high = mid - 1
    return -1


def factorial_iterative(n: int) -> int:
    """Compute factorial iteratively."""
    if n < 0:
        return 0
    result: int = 1
    i: int = 2
    while i <= n:
        result = result * i
        i = i + 1
    return result


def test_module() -> int:
    """Test all recursive algorithm functions."""
    assert gcd(48, 18) == 6
    assert gcd(7, 13) == 1
    assert gcd(0, 5) == 5
    assert lcm(4, 6) == 12
    assert lcm(3, 7) == 21
    assert lcm(0, 5) == 0
    assert fibonacci_iterative(0) == 0
    assert fibonacci_iterative(1) == 1
    assert fibonacci_iterative(10) == 55
    assert fibonacci_iterative(15) == 610
    assert binary_search([1, 3, 5, 7, 9], 5) == 2
    assert binary_search([1, 3, 5, 7, 9], 1) == 0
    assert binary_search([1, 3, 5, 7, 9], 9) == 4
    assert binary_search([1, 3, 5, 7, 9], 4) == -1
    assert factorial_iterative(0) == 1
    assert factorial_iterative(5) == 120
    assert factorial_iterative(10) == 3628800
    return 0


if __name__ == "__main__":
    test_module()
