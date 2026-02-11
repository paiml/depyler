"""Binomial coefficients and binomial theorem applications."""


def choose(n: int, r: int) -> int:
    """Binomial coefficient C(n,r)."""
    if r < 0 or r > n:
        return 0
    if r == 0 or r == n:
        return 1
    if r > n - r:
        r = n - r
    result: int = 1
    i: int = 0
    while i < r:
        result = result * (n - i)
        result = result // (i + 1)
        i = i + 1
    return result


def int_pow(bv: int, exp: int) -> int:
    """Integer exponentiation."""
    result: int = 1
    i: int = 0
    while i < exp:
        result = result * bv
        i = i + 1
    return result


def binomial_expand(a: int, b: int, n: int) -> int:
    """Compute (a+b)^n using binomial theorem."""
    total: int = 0
    k: int = 0
    while k <= n:
        coeff: int = choose(n, k)
        total = total + coeff * int_pow(a, n - k) * int_pow(b, k)
        k = k + 1
    return total


def vandermonde_check(m: int, n: int, r: int) -> int:
    """Verify Vandermonde identity: C(m+n,r) = sum C(m,k)*C(n,r-k)."""
    lhs: int = choose(m + n, r)
    rhs: int = 0
    k: int = 0
    while k <= r:
        rhs = rhs + choose(m, k) * choose(n, r - k)
        k = k + 1
    if lhs == rhs:
        return 1
    return 0


def test_module() -> int:
    """Test binomial coefficient functions."""
    ok: int = 0
    if choose(10, 4) == 210:
        ok = ok + 1
    if binomial_expand(1, 1, 5) == 32:
        ok = ok + 1
    if binomial_expand(2, 1, 3) == 27:
        ok = ok + 1
    if vandermonde_check(3, 4, 3) == 1:
        ok = ok + 1
    if choose(6, 3) == 20:
        ok = ok + 1
    return ok
