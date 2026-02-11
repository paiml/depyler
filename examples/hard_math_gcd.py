"""GCD, LCM, extended Euclidean algorithm, coprimality check."""


def gcd(a: int, b: int) -> int:
    """Compute GCD using Euclidean algorithm."""
    x: int = a
    y: int = b
    if x < 0:
        x = -x
    if y < 0:
        y = -y
    while y != 0:
        temp: int = y
        y = x % y
        x = temp
    return x


def lcm(a: int, b: int) -> int:
    """Compute LCM of two numbers."""
    if a == 0 or b == 0:
        return 0
    g: int = gcd(a, b)
    result: int = a // g * b
    if result < 0:
        result = -result
    return result


def extended_gcd(a: int, b: int) -> list[int]:
    """Extended Euclidean algorithm. Returns [gcd, x, y] where ax + by = gcd."""
    if a == 0:
        return [b, 0, 1]
    old_r: int = a
    r: int = b
    old_s: int = 1
    s: int = 0
    old_t: int = 0
    t: int = 1
    while r != 0:
        quotient: int = old_r // r
        temp_r: int = r
        r = old_r - quotient * r
        old_r = temp_r
        temp_s: int = s
        s = old_s - quotient * s
        old_s = temp_s
        temp_t: int = t
        t = old_t - quotient * t
        old_t = temp_t
    return [old_r, old_s, old_t]


def are_coprime(a: int, b: int) -> int:
    """Check if two numbers are coprime. Returns 1 or 0."""
    if gcd(a, b) == 1:
        return 1
    return 0


def gcd_of_list(arr: list[int]) -> int:
    """Compute GCD of all elements in a list."""
    if len(arr) == 0:
        return 0
    result: int = arr[0]
    i: int = 1
    while i < len(arr):
        result = gcd(result, arr[i])
        i = i + 1
    return result


def test_module() -> int:
    passed: int = 0

    if gcd(12, 8) == 4:
        passed = passed + 1

    if gcd(7, 13) == 1:
        passed = passed + 1

    if lcm(4, 6) == 12:
        passed = passed + 1

    ext: list[int] = extended_gcd(35, 15)
    if ext[0] == 5:
        passed = passed + 1

    # Verify ax + by = gcd
    check: int = 35 * ext[1] + 15 * ext[2]
    if check == ext[0]:
        passed = passed + 1

    if are_coprime(8, 15) == 1:
        passed = passed + 1

    if are_coprime(8, 12) == 0:
        passed = passed + 1

    if gcd_of_list([12, 18, 24]) == 6:
        passed = passed + 1

    return passed
