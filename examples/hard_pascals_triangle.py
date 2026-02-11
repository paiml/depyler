"""Pascal's triangle operations.

Tests: row generation, element access, row sum, alternating sum.
"""


def pascal_element(row: int, col: int) -> int:
    """Get element at given row and column of Pascal's triangle."""
    if col < 0 or col > row:
        return 0
    if col == 0 or col == row:
        return 1
    result: int = 1
    i: int = 0
    kk: int = col
    if kk > row - kk:
        kk = row - kk
    while i < kk:
        result = result * (row - i)
        result = result // (i + 1)
        i = i + 1
    return result


def pascal_row_sum(row: int) -> int:
    """Sum of all elements in a row of Pascal's triangle (should be 2^row)."""
    result: int = 1
    i: int = 0
    while i < row:
        result = result * 2
        i = i + 1
    return result


def pascal_diagonal_sum(n: int) -> int:
    """Sum along the nth diagonal of Pascal's triangle."""
    total: int = 0
    k: int = 0
    while k <= n:
        total = total + pascal_element(n - k + k, k)
        k = k + 1
    return total


def pascal_center(n: int) -> int:
    """Central element of even-numbered row."""
    if n % 2 != 0:
        return 0
    return pascal_element(n, n // 2)


def test_module() -> int:
    """Test Pascal's triangle operations."""
    ok: int = 0
    if pascal_element(4, 2) == 6:
        ok = ok + 1
    if pascal_element(5, 0) == 1:
        ok = ok + 1
    if pascal_element(6, 3) == 20:
        ok = ok + 1
    if pascal_row_sum(0) == 1:
        ok = ok + 1
    if pascal_row_sum(4) == 16:
        ok = ok + 1
    if pascal_row_sum(10) == 1024:
        ok = ok + 1
    if pascal_center(4) == 6:
        ok = ok + 1
    if pascal_center(6) == 20:
        ok = ok + 1
    return ok
