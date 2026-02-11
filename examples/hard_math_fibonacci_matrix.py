"""Matrix exponentiation for fast Fibonacci computation."""


def mat_mult_2x2(a: list[int], b: list[int]) -> list[int]:
    """Multiply two 2x2 matrices stored as flat [a,b,c,d]."""
    r0: int = a[0] * b[0] + a[1] * b[2]
    r1: int = a[0] * b[1] + a[1] * b[3]
    r2: int = a[2] * b[0] + a[3] * b[2]
    r3: int = a[2] * b[1] + a[3] * b[3]
    return [r0, r1, r2, r3]


def mat_pow_2x2(m: list[int], n: int) -> list[int]:
    """Raise 2x2 matrix to power n."""
    result: list[int] = [1, 0, 0, 1]
    bm: list[int] = [m[0], m[1], m[2], m[3]]
    while n > 0:
        if n % 2 == 1:
            result = mat_mult_2x2(result, bm)
        bm = mat_mult_2x2(bm, bm)
        n = n // 2
    return result


def fibonacci_matrix(n: int) -> int:
    """Compute nth Fibonacci using matrix exponentiation. F(0)=0, F(1)=1."""
    if n <= 0:
        return 0
    if n == 1:
        return 1
    fib_mat: list[int] = [1, 1, 1, 0]
    result: list[int] = mat_pow_2x2(fib_mat, n - 1)
    return result[0]


def fibonacci_naive(n: int) -> int:
    """Compute nth Fibonacci iteratively for verification."""
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


def fibonacci_sum(n: int) -> int:
    """Sum of first n Fibonacci numbers."""
    total: int = 0
    i: int = 0
    while i <= n:
        total = total + fibonacci_matrix(i)
        i = i + 1
    return total


def test_module() -> int:
    """Test Fibonacci matrix functions."""
    ok: int = 0
    if fibonacci_matrix(0) == 0:
        ok = ok + 1
    if fibonacci_matrix(10) == 55:
        ok = ok + 1
    if fibonacci_matrix(15) == 610:
        ok = ok + 1
    if fibonacci_matrix(10) == fibonacci_naive(10):
        ok = ok + 1
    if fibonacci_sum(5) == 12:
        ok = ok + 1
    return ok
