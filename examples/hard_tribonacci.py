"""Tribonacci and N-step sequences.

Tests: tribonacci, tetranacci, n-step sum, padovan sequence.
"""


def tribonacci(n: int) -> int:
    """Compute nth Tribonacci number (0, 0, 1, 1, 2, 4, 7, 13, ...)."""
    if n == 0:
        return 0
    if n == 1:
        return 0
    if n == 2:
        return 1
    a: int = 0
    b: int = 0
    c: int = 1
    i: int = 3
    while i <= n:
        temp: int = a + b + c
        a = b
        b = c
        c = temp
        i = i + 1
    return c


def tetranacci(n: int) -> int:
    """Compute nth Tetranacci number."""
    if n <= 1:
        return 0
    if n == 2:
        return 0
    if n == 3:
        return 1
    a: int = 0
    b: int = 0
    c: int = 0
    d: int = 1
    i: int = 4
    while i <= n:
        temp: int = a + b + c + d
        a = b
        b = c
        c = d
        d = temp
        i = i + 1
    return d


def padovan(n: int) -> int:
    """Compute nth Padovan number: P(n) = P(n-2) + P(n-3)."""
    if n <= 2:
        return 1
    a: int = 1
    b: int = 1
    c: int = 1
    i: int = 3
    while i <= n:
        temp: int = a + b
        a = b
        b = c
        c = temp
        i = i + 1
    return c


def n_step_sum(n: int, steps: int) -> int:
    """Generalized n-step number: sum of previous 'steps' values.
    
    Base: first (steps-1) values are 0, the steps-th is 1.
    """
    if n < steps:
        return 0
    if n == steps:
        return 1
    dp: list[int] = [0] * (n + 1)
    dp[steps] = 1
    i: int = steps + 1
    while i <= n:
        total: int = 0
        j: int = 1
        while j <= steps and i - j >= 0:
            total = total + dp[i - j]
            j = j + 1
        dp[i] = total
        i = i + 1
    return dp[n]


def test_module() -> int:
    """Test Tribonacci and N-step sequences."""
    ok: int = 0
    if tribonacci(0) == 0:
        ok = ok + 1
    if tribonacci(4) == 2:
        ok = ok + 1
    if tribonacci(7) == 13:
        ok = ok + 1
    if tetranacci(3) == 1:
        ok = ok + 1
    if tetranacci(5) == 2:
        ok = ok + 1
    if padovan(0) == 1:
        ok = ok + 1
    if padovan(5) == 3:
        ok = ok + 1
    if padovan(8) == 9:
        ok = ok + 1
    return ok
