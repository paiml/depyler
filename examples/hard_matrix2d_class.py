"""2D matrix operations using flat list representation.

Tests: create, sum, trace, row sum, column sum.
"""


def matrix_create(rows: int, cols: int) -> list[int]:
    """Create a flat zero matrix."""
    result: list[int] = []
    i: int = 0
    while i < rows * cols:
        result.append(0)
        i = i + 1
    return result


def matrix_sum(data: list[int]) -> int:
    """Sum all elements in flat matrix."""
    total: int = 0
    for v in data:
        total = total + v
    return total


def matrix_trace(data: list[int], n: int) -> int:
    """Sum of diagonal elements for n x n matrix."""
    total: int = 0
    i: int = 0
    while i < n:
        total = total + data[i * n + i]
        i = i + 1
    return total


def matrix_row_sum(data: list[int], cols: int, row: int) -> int:
    """Sum of elements in a given row."""
    total: int = 0
    c: int = 0
    while c < cols:
        total = total + data[row * cols + c]
        c = c + 1
    return total


def matrix_max(data: list[int]) -> int:
    """Maximum element in flat matrix."""
    if len(data) == 0:
        return 0
    best: int = data[0]
    for v in data:
        if v > best:
            best = v
    return best


def test_module() -> int:
    """Test matrix operations."""
    ok: int = 0
    m: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9]
    if matrix_sum(m) == 45:
        ok = ok + 1
    if matrix_trace(m, 3) == 15:
        ok = ok + 1
    if matrix_row_sum(m, 3, 1) == 15:
        ok = ok + 1
    if matrix_max(m) == 9:
        ok = ok + 1
    z: list[int] = matrix_create(2, 3)
    if matrix_sum(z) == 0:
        ok = ok + 1
    if len(z) == 6:
        ok = ok + 1
    return ok
