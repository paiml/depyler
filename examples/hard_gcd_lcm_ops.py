"""GCD and LCM operations on arrays.

Tests: gcd, lcm, gcd of array, lcm of array, coprime check.
"""


def gcd_two(a: int, b: int) -> int:
    """GCD of two non-negative integers using Euclidean algorithm."""
    x: int = a
    y: int = b
    if x < 0:
        x = -x
    if y < 0:
        y = -y
    while y > 0:
        temp: int = y
        y = x % y
        x = temp
    return x


def lcm_two(a: int, b: int) -> int:
    """LCM of two positive integers."""
    if a == 0 or b == 0:
        return 0
    g: int = gcd_two(a, b)
    return (a // g) * b


def gcd_array(arr: list[int]) -> int:
    """GCD of all elements in array."""
    n: int = len(arr)
    if n == 0:
        return 0
    result: int = arr[0]
    i: int = 1
    while i < n:
        result = gcd_two(result, arr[i])
        i = i + 1
    return result


def lcm_array(arr: list[int]) -> int:
    """LCM of all elements in array."""
    n: int = len(arr)
    if n == 0:
        return 0
    result: int = arr[0]
    i: int = 1
    while i < n:
        result = lcm_two(result, arr[i])
        i = i + 1
    return result


def count_coprimes(arr: list[int]) -> int:
    """Count pairs of coprime elements in array."""
    n: int = len(arr)
    count: int = 0
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            if gcd_two(arr[i], arr[j]) == 1:
                count = count + 1
            j = j + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test GCD and LCM operations."""
    ok: int = 0
    if gcd_two(12, 8) == 4:
        ok = ok + 1
    if gcd_two(17, 13) == 1:
        ok = ok + 1
    if lcm_two(4, 6) == 12:
        ok = ok + 1
    if gcd_array([12, 18, 24]) == 6:
        ok = ok + 1
    if lcm_array([2, 3, 4]) == 12:
        ok = ok + 1
    if count_coprimes([2, 3, 4, 5]) == 4:
        ok = ok + 1
    return ok
