"""Permutation counting and properties."""


def factorial(n: int) -> int:
    """Compute n factorial iteratively."""
    if n <= 1:
        return 1
    result: int = 1
    i: int = 2
    while i <= n:
        result = result * i
        i = i + 1
    return result


def permutation_count(n: int, r: int) -> int:
    """P(n,r) = n! / (n-r)!"""
    if r > n or r < 0:
        return 0
    result: int = 1
    i: int = 0
    while i < r:
        result = result * (n - i)
        i = i + 1
    return result


def count_fixed_points(perm: list[int]) -> int:
    """Count fixed points in a permutation (where perm[i] == i)."""
    count: int = 0
    i: int = 0
    n: int = len(perm)
    while i < n:
        if perm[i] == i:
            count = count + 1
        i = i + 1
    return count


def inversion_count(perm: list[int]) -> int:
    """Count inversions (i < j but perm[i] > perm[j])."""
    n: int = len(perm)
    count: int = 0
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            if perm[i] > perm[j]:
                count = count + 1
            j = j + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test permutation functions."""
    ok: int = 0
    if factorial(5) == 120:
        ok = ok + 1
    if permutation_count(5, 3) == 60:
        ok = ok + 1
    p: list[int] = [0, 2, 1]
    if count_fixed_points(p) == 1:
        ok = ok + 1
    if inversion_count(p) == 1:
        ok = ok + 1
    if permutation_count(4, 4) == 24:
        ok = ok + 1
    return ok
