"""Modular arithmetic, GCD, and extended Euclidean algorithm."""


def gcd_euclidean(a: int, b: int) -> int:
    """Compute GCD using Euclidean algorithm."""
    x: int = a
    y: int = b
    if x < 0:
        x = 0 - x
    if y < 0:
        y = 0 - y
    while y != 0:
        temp: int = y
        y = x % y
        x = temp
    return x


def lcm(a: int, b: int) -> int:
    """Compute least common multiple."""
    if a == 0 or b == 0:
        return 0
    g: int = gcd_euclidean(a, b)
    return (a // g) * b


def extended_gcd(a: int, b: int) -> list[int]:
    """Extended Euclidean: returns [gcd, x, y] where ax + by = gcd."""
    if b == 0:
        return [a, 1, 0]
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


def mod_inverse(a: int, m: int) -> int:
    """Compute modular inverse of a mod m. Returns -1 if none exists."""
    result: list[int] = extended_gcd(a, m)
    g: int = result[0]
    x: int = result[1]
    if g != 1:
        return -1
    return ((x % m) + m) % m


def mod_add(a: int, b: int, m: int) -> int:
    """Modular addition."""
    return ((a % m) + (b % m)) % m


def mod_sub(a: int, b: int, m: int) -> int:
    """Modular subtraction."""
    return (((a % m) - (b % m)) + m) % m


def mod_mul(a: int, b: int, m: int) -> int:
    """Modular multiplication."""
    return ((a % m) * (b % m)) % m


def mod_pow(base_val: int, exp: int, m: int) -> int:
    """Modular exponentiation."""
    if m == 1:
        return 0
    result: int = 1
    b: int = base_val % m
    e: int = exp
    while e > 0:
        if (e & 1) == 1:
            result = (result * b) % m
        e = e >> 1
        b = (b * b) % m
    return result


def is_coprime(a: int, b: int) -> int:
    """Return 1 if a and b are coprime, 0 otherwise."""
    g: int = gcd_euclidean(a, b)
    if g == 1:
        return 1
    return 0


def euler_totient(n: int) -> int:
    """Compute Euler's totient function."""
    if n <= 0:
        return 0
    result: int = n
    p: int = 2
    val: int = n
    while p * p <= val:
        if val % p == 0:
            while val % p == 0:
                val = val // p
            result = result - result // p
        p = p + 1
    if val > 1:
        result = result - result // val
    return result


def test_module() -> int:
    """Test all modular arithmetic functions."""
    passed: int = 0
    if gcd_euclidean(48, 18) == 6:
        passed = passed + 1
    if gcd_euclidean(0, 5) == 5:
        passed = passed + 1
    if lcm(4, 6) == 12:
        passed = passed + 1
    eg: list[int] = extended_gcd(35, 15)
    if eg[0] == 5:
        passed = passed + 1
    verify: int = 35 * eg[1] + 15 * eg[2]
    if verify == 5:
        passed = passed + 1
    inv: int = mod_inverse(3, 11)
    if inv == 4:
        passed = passed + 1
    if mod_inverse(2, 4) == -1:
        passed = passed + 1
    if mod_add(7, 8, 10) == 5:
        passed = passed + 1
    if mod_sub(3, 7, 10) == 6:
        passed = passed + 1
    if mod_mul(7, 8, 10) == 6:
        passed = passed + 1
    if mod_pow(2, 10, 1000) == 24:
        passed = passed + 1
    if is_coprime(15, 28) == 1:
        passed = passed + 1
    if euler_totient(12) == 4:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
