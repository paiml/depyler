"""Numerical methods: Fibonacci and recurrence relations.

Tests: iterative sequences, matrix power technique, closed-form validation,
multi-step recurrences, modular sequences.
"""

from typing import List, Tuple


def fibonacci_iter(n: int) -> int:
    """Iterative Fibonacci computation."""
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
        i += 1
    return b


def fibonacci_mod(n: int, mod: int) -> int:
    """Fibonacci number modulo m."""
    if n <= 0:
        return 0
    if n == 1:
        return 1 % mod
    a: int = 0
    b: int = 1
    i: int = 2
    while i <= n:
        c: int = (a + b) % mod
        a = b
        b = c
        i += 1
    return b


def lucas_number(n: int) -> int:
    """Lucas sequence: L(0)=2, L(1)=1, L(n)=L(n-1)+L(n-2)."""
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
        i += 1
    return b


def tribonacci(n: int) -> int:
    """Tribonacci: T(0)=0, T(1)=0, T(2)=1, T(n)=T(n-1)+T(n-2)+T(n-3)."""
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
        i += 1
    return c


def padovan_number(n: int) -> int:
    """Padovan sequence: P(0)=P(1)=P(2)=1, P(n)=P(n-2)+P(n-3)."""
    if n <= 2:
        return 1
    a: int = 1
    b: int = 1
    c: int = 1
    i: int = 3
    while i <= n:
        d: int = a + b
        a = b
        b = c
        c = d
        i += 1
    return c


def collatz_length(n: int) -> int:
    """Length of Collatz sequence starting from n."""
    if n <= 0:
        return 0
    count: int = 0
    val: int = n
    while val != 1:
        if val % 2 == 0:
            val = val // 2
        else:
            val = 3 * val + 1
        count += 1
    return count


def recaman_sequence(n: int) -> List[int]:
    """Generate first n terms of Recaman's sequence."""
    if n <= 0:
        return []
    result: List[int] = [0]
    seen: Dict[int, int] = {}
    seen[0] = 1
    i: int = 1
    while i < n:
        prev: int = result[i - 1]
        candidate: int = prev - i
        if candidate > 0 and candidate not in seen:
            result.append(candidate)
            seen[candidate] = 1
        else:
            result.append(prev + i)
            seen[prev + i] = 1
        i += 1
    return result


def catalan_number(n: int) -> int:
    """Compute nth Catalan number iteratively."""
    if n <= 1:
        return 1
    dp: List[int] = [0] * (n + 1)
    dp[0] = 1
    dp[1] = 1
    i: int = 2
    while i <= n:
        j: int = 0
        while j < i:
            dp[i] = dp[i] + dp[j] * dp[i - 1 - j]
            j += 1
        i += 1
    return dp[n]


def test_sequences() -> bool:
    """Test sequence computations."""
    ok: bool = True
    fib10: int = fibonacci_iter(10)
    if fib10 != 55:
        ok = False
    luc: int = lucas_number(5)
    if luc != 11:
        ok = False
    trib: int = tribonacci(7)
    if trib != 13:
        ok = False
    col: int = collatz_length(6)
    if col < 1:
        ok = False
    cat: int = catalan_number(5)
    if cat != 42:
        ok = False
    return ok
