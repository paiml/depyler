"""Derangements (permutations with no fixed points) and subfactorial."""


def factorial(n: int) -> int:
    """Compute n factorial."""
    if n <= 1:
        return 1
    result: int = 1
    i: int = 2
    while i <= n:
        result = result * i
        i = i + 1
    return result


def subfactorial(n: int) -> int:
    """Subfactorial !n = number of derangements of n elements.
    D(0)=1, D(1)=0, D(n)=(n-1)*(D(n-1)+D(n-2))."""
    if n == 0:
        return 1
    if n == 1:
        return 0
    prev2: int = 1
    prev1: int = 0
    i: int = 2
    while i <= n:
        cur: int = (i - 1) * (prev1 + prev2)
        prev2 = prev1
        prev1 = cur
        i = i + 1
    return prev1


def is_derangement(perm: list[int]) -> int:
    """Check if permutation is a derangement (no fixed points)."""
    n: int = len(perm)
    i: int = 0
    while i < n:
        if perm[i] == i:
            return 0
        i = i + 1
    return 1


def derangement_probability_num(n: int) -> int:
    """Numerator of P(derangement) = !n (subfactorial)."""
    return subfactorial(n)


def derangement_probability_den(n: int) -> int:
    """Denominator of P(derangement) = n!."""
    return factorial(n)


def count_fixed_points(perm: list[int]) -> int:
    """Count how many elements are in their original position."""
    n: int = len(perm)
    count: int = 0
    i: int = 0
    while i < n:
        if perm[i] == i:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test derangement functions."""
    ok: int = 0
    if subfactorial(0) == 1:
        ok = ok + 1
    if subfactorial(1) == 0:
        ok = ok + 1
    if subfactorial(4) == 9:
        ok = ok + 1
    if subfactorial(5) == 44:
        ok = ok + 1
    d: list[int] = [1, 0, 3, 2]
    if is_derangement(d) == 1:
        ok = ok + 1
    nd: list[int] = [0, 1, 2, 3]
    if is_derangement(nd) == 0:
        ok = ok + 1
    return ok
