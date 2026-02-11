"""Bell numbers and set partition counting."""


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


def bell_number(n: int) -> int:
    """Compute nth Bell number using Bell triangle.
    B(0)=1, B(n) = sum C(n-1,k)*B(k) for k=0..n-1."""
    if n == 0:
        return 1
    b: list[int] = []
    i: int = 0
    while i <= n:
        b.append(0)
        i = i + 1
    b[0] = 1
    ni: int = 1
    while ni <= n:
        total: int = 0
        k: int = 0
        while k < ni:
            total = total + choose(ni - 1, k) * b[k]
            k = k + 1
        b[ni] = total
        ni = ni + 1
    return b[n]


def bell_triangle_row(n: int) -> list[int]:
    """Compute nth row of Bell triangle. First element is B(n)."""
    if n == 0:
        return [1]
    prev: list[int] = bell_triangle_row(n - 1)
    np: int = len(prev)
    last_prev: int = prev[np - 1]
    row: list[int] = [last_prev]
    i: int = 0
    while i < np:
        nxt: int = row[i] + prev[i]
        row.append(nxt)
        i = i + 1
    return row


def sum_bell(n: int) -> int:
    """Sum of Bell numbers B(0) through B(n)."""
    total: int = 0
    i: int = 0
    while i <= n:
        total = total + bell_number(i)
        i = i + 1
    return total


def test_module() -> int:
    """Test Bell number functions."""
    ok: int = 0
    if bell_number(0) == 1:
        ok = ok + 1
    if bell_number(3) == 5:
        ok = ok + 1
    if bell_number(4) == 15:
        ok = ok + 1
    if bell_number(5) == 52:
        ok = ok + 1
    row: list[int] = bell_triangle_row(3)
    if row[0] == 5:
        ok = ok + 1
    return ok
