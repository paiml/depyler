"""Integer sequences: Fibonacci, Lucas, Pell, Catalan."""


def fibonacci(n: int) -> int:
    """Compute nth Fibonacci number. F(0)=0, F(1)=1."""
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


def lucas(n: int) -> int:
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


def pell(n: int) -> int:
    """Compute nth Pell number. P(0)=0, P(1)=1, P(n)=2*P(n-1)+P(n-2)."""
    if n <= 0:
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


def catalan(n: int) -> int:
    """Compute nth Catalan number using DP."""
    if n <= 1:
        return 1
    dp: list[int] = []
    i: int = 0
    while i <= n:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    dp[1] = 1
    k: int = 2
    while k <= n:
        j: int = 0
        while j < k:
            dp[k] = dp[k] + dp[j] * dp[k - 1 - j]
            j = j + 1
        k = k + 1
    return dp[n]


def tribonacci(n: int) -> int:
    """Compute nth Tribonacci. T(0)=0, T(1)=0, T(2)=1."""
    if n <= 1:
        return 0
    if n == 2:
        return 1
    a: int = 0
    b: int = 0
    c: int = 1
    i: int = 3
    while i <= n:
        d: int = a + b + c
        a = b
        b = c
        c = d
        i = i + 1
    return c


def test_module() -> int:
    passed: int = 0

    if fibonacci(10) == 55:
        passed = passed + 1

    if fibonacci(0) == 0:
        passed = passed + 1

    if lucas(5) == 11:
        passed = passed + 1

    if lucas(0) == 2:
        passed = passed + 1

    if pell(5) == 29:
        passed = passed + 1

    if catalan(5) == 42:
        passed = passed + 1

    if tribonacci(7) == 13:
        passed = passed + 1

    if tribonacci(0) == 0:
        passed = passed + 1

    return passed
