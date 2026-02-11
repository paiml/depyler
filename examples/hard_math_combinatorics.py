"""Combinatorics: nCr, nPr, factorial, derangement count."""


def factorial(n: int) -> int:
    """Compute n! iteratively."""
    if n <= 1:
        return 1
    result: int = 1
    i: int = 2
    while i <= n:
        result = result * i
        i = i + 1
    return result


def permutations(n: int, r: int) -> int:
    """Compute nPr = n! / (n-r)!"""
    if r > n or r < 0:
        return 0
    result: int = 1
    i: int = 0
    while i < r:
        result = result * (n - i)
        i = i + 1
    return result


def combinations(n: int, r: int) -> int:
    """Compute nCr = n! / (r! * (n-r)!)"""
    if r > n or r < 0:
        return 0
    if r > n - r:
        r = n - r
    result: int = 1
    i: int = 0
    while i < r:
        result = result * (n - i)
        result = result // (i + 1)
        i = i + 1
    return result


def derangement(n: int) -> int:
    """Count derangements D(n) using iterative formula.
    D(0) = 1, D(1) = 0, D(n) = (n-1) * (D(n-1) + D(n-2))
    """
    if n == 0:
        return 1
    if n == 1:
        return 0
    prev2: int = 1
    prev1: int = 0
    i: int = 2
    while i <= n:
        curr: int = (i - 1) * (prev1 + prev2)
        prev2 = prev1
        prev1 = curr
        i = i + 1
    return prev1


def catalan(n: int) -> int:
    """Compute nth Catalan number using C(2n, n) / (n+1)."""
    return combinations(2 * n, n) // (n + 1)


def test_module() -> int:
    passed: int = 0

    if factorial(5) == 120:
        passed = passed + 1

    if factorial(0) == 1:
        passed = passed + 1

    if permutations(5, 3) == 60:
        passed = passed + 1

    if combinations(5, 2) == 10:
        passed = passed + 1

    if combinations(10, 3) == 120:
        passed = passed + 1

    if derangement(4) == 9:
        passed = passed + 1

    if derangement(0) == 1:
        passed = passed + 1

    if catalan(4) == 14:
        passed = passed + 1

    return passed
