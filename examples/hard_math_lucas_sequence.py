"""Lucas numbers and Pell numbers sequences."""


def lucas_number(n: int) -> int:
    """Compute nth Lucas number. L(0)=2, L(1)=1."""
    if n == 0:
        return 2
    if n == 1:
        return 1
    a: int = 2
    b: int = 1
    i: int = 2
    while i <= n:
        c: int = a + b
        a = b
        b = c
        i = i + 1
    return b


def pell_number(n: int) -> int:
    """Compute nth Pell number. P(0)=0, P(1)=1."""
    if n == 0:
        return 0
    if n == 1:
        return 1
    a: int = 0
    b: int = 1
    i: int = 2
    while i <= n:
        c: int = 2 * b + a
        a = b
        b = c
        i = i + 1
    return b


def fibonacci_iter(n: int) -> int:
    """Iterative Fibonacci for verification."""
    if n <= 0:
        return 0
    if n == 1:
        return 1
    a: int = 0
    b: int = 1
    i: int = 2
    while i <= n:
        c: int = a + b
        a = b
        b = c
        i = i + 1
    return b


def lucas_identity_check(n: int) -> int:
    """Check L(n)^2 - 5*F(n)^2 = 4*(-1)^n."""
    ln: int = lucas_number(n)
    fn: int = fibonacci_iter(n)
    lhs: int = ln * ln - 5 * fn * fn
    rhs: int = 4
    if n % 2 == 1:
        rhs = 0 - 4
    if lhs == rhs:
        return 1
    return 0


def test_module() -> int:
    """Test Lucas and Pell sequences."""
    ok: int = 0
    if lucas_number(0) == 2:
        ok = ok + 1
    if lucas_number(5) == 11:
        ok = ok + 1
    if pell_number(5) == 29:
        ok = ok + 1
    if pell_number(0) == 0:
        ok = ok + 1
    if lucas_identity_check(6) == 1:
        ok = ok + 1
    return ok
