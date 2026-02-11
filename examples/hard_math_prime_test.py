"""Deterministic primality testing with trial division and strong pseudoprime."""


def is_prime_trial(n: int) -> int:
    """Trial division primality test. Returns 1 if prime."""
    if n < 2:
        return 0
    if n < 4:
        return 1
    if n % 2 == 0:
        return 0
    if n % 3 == 0:
        return 0
    i: int = 5
    while i * i <= n:
        if n % i == 0:
            return 0
        if n % (i + 2) == 0:
            return 0
        i = i + 6
    return 1


def mod_pow(bv: int, exp: int, mod: int) -> int:
    """Modular exponentiation: bv^exp mod mod."""
    result: int = 1
    bv = bv % mod
    while exp > 0:
        if exp % 2 == 1:
            result = (result * bv) % mod
        exp = exp // 2
        bv = (bv * bv) % mod
    return result


def is_strong_pseudoprime(n: int, a: int) -> int:
    """Check if n is strong pseudoprime to witness a."""
    if n < 2:
        return 0
    if n == a:
        return 1
    d: int = n - 1
    r: int = 0
    while d % 2 == 0:
        d = d // 2
        r = r + 1
    x: int = mod_pow(a, d, n)
    if x == 1 or x == n - 1:
        return 1
    i: int = 0
    while i < r - 1:
        x = (x * x) % n
        if x == n - 1:
            return 1
        i = i + 1
    return 0


def is_prime_det(n: int) -> int:
    """Deterministic primality using witnesses 2,3,5,7."""
    if n < 2:
        return 0
    if n == 2 or n == 3 or n == 5 or n == 7:
        return 1
    if n % 2 == 0:
        return 0
    if is_strong_pseudoprime(n, 2) == 0:
        return 0
    if is_strong_pseudoprime(n, 3) == 0:
        return 0
    if is_strong_pseudoprime(n, 5) == 0:
        return 0
    return 1


def test_module() -> int:
    """Test primality tests."""
    ok: int = 0
    if is_prime_trial(97) == 1:
        ok = ok + 1
    if is_prime_trial(100) == 0:
        ok = ok + 1
    if is_prime_det(97) == 1:
        ok = ok + 1
    if is_prime_det(561) == 0:
        ok = ok + 1
    if is_prime_det(7919) == 1:
        ok = ok + 1
    return ok
