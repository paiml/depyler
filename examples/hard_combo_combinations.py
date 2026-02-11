"""Combination counting and Pascal's triangle."""


def choose(n: int, r: int) -> int:
    """Binomial coefficient C(n,r) via multiplicative formula."""
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


def pascal_row(n: int) -> list[int]:
    """Generate nth row of Pascal's triangle."""
    row: list[int] = [1]
    i: int = 0
    while i < n:
        prev: int = row[i]
        nxt: int = prev * (n - i) // (i + 1)
        row.append(nxt)
        i = i + 1
    return row


def pascal_sum(n: int) -> int:
    """Sum of nth row of Pascal's triangle = 2^n."""
    row: list[int] = pascal_row(n)
    total: int = 0
    i: int = 0
    nr: int = len(row)
    while i < nr:
        total = total + row[i]
        i = i + 1
    return total


def multiset_coeff(n: int, r: int) -> int:
    """Multiset coefficient: C(n+r-1, r)."""
    return choose(n + r - 1, r)


def central_binomial(n: int) -> int:
    """Central binomial coefficient C(2n, n)."""
    return choose(2 * n, n)


def test_module() -> int:
    """Test combination functions."""
    ok: int = 0
    if choose(5, 2) == 10:
        ok = ok + 1
    if choose(10, 3) == 120:
        ok = ok + 1
    if pascal_sum(4) == 16:
        ok = ok + 1
    row: list[int] = pascal_row(4)
    if row[2] == 6:
        ok = ok + 1
    if central_binomial(3) == 20:
        ok = ok + 1
    return ok
