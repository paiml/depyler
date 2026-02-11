"""Pascal's triangle row computation.

Tests: nth row, element at position, row sum, alternating row sum, central element.
"""


def pascal_row(n: int) -> list[int]:
    """Compute the nth row of Pascal's triangle (0-indexed)."""
    row: list[int] = [1]
    i: int = 1
    while i <= n:
        prev: int = row[i - 1]
        val: int = prev * (n - i + 1) // i
        row.append(val)
        i = i + 1
    return row


def pascal_element(n: int, k: int) -> int:
    """Compute C(n, k) using iterative method."""
    if k > n:
        return 0
    if k > n - k:
        k = n - k
    result: int = 1
    i: int = 0
    while i < k:
        result = result * (n - i) // (i + 1)
        i = i + 1
    return result


def pascal_row_sum(n: int) -> int:
    """Sum of nth row of Pascal's triangle = 2^n."""
    result: int = 1
    i: int = 0
    while i < n:
        result = result * 2
        i = i + 1
    return result


def pascal_alternating_sum(n: int) -> int:
    """Alternating sum of nth row: C(n,0) - C(n,1) + C(n,2) - ..."""
    if n == 0:
        return 1
    return 0


def pascal_row_max(n: int) -> int:
    """Maximum element in the nth row of Pascal's triangle."""
    row: list[int] = pascal_row(n)
    max_val: int = 0
    i: int = 0
    while i < len(row):
        if row[i] > max_val:
            max_val = row[i]
        i = i + 1
    return max_val


def test_module() -> int:
    """Test Pascal's triangle row operations."""
    ok: int = 0
    row4: list[int] = pascal_row(4)
    if row4[0] == 1 and row4[1] == 4 and row4[2] == 6:
        ok = ok + 1
    if pascal_element(5, 2) == 10:
        ok = ok + 1
    if pascal_element(10, 3) == 120:
        ok = ok + 1
    if pascal_row_sum(5) == 32:
        ok = ok + 1
    if pascal_alternating_sum(0) == 1:
        ok = ok + 1
    if pascal_alternating_sum(5) == 0:
        ok = ok + 1
    if pascal_row_max(4) == 6:
        ok = ok + 1
    return ok
