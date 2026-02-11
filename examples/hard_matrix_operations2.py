"""Matrix operations: add, subtract, scalar multiply, transpose.

Tests: matrix_add, matrix_sub, scalar_multiply, transpose.
"""


def matrix_add(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    """Add two matrices element-wise."""
    rows: int = len(a)
    result: list[list[int]] = []
    r: int = 0
    while r < rows:
        row: list[int] = []
        c: int = 0
        while c < len(a[r]):
            row.append(a[r][c] + b[r][c])
            c = c + 1
        result.append(row)
        r = r + 1
    return result


def matrix_sub(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    """Subtract matrix b from matrix a element-wise."""
    rows: int = len(a)
    result: list[list[int]] = []
    r: int = 0
    while r < rows:
        row: list[int] = []
        c: int = 0
        while c < len(a[r]):
            row.append(a[r][c] - b[r][c])
            c = c + 1
        result.append(row)
        r = r + 1
    return result


def scalar_multiply(m: list[list[int]], s: int) -> list[list[int]]:
    """Multiply every element of matrix by scalar s."""
    result: list[list[int]] = []
    r: int = 0
    while r < len(m):
        row: list[int] = []
        c: int = 0
        while c < len(m[r]):
            row.append(m[r][c] * s)
            c = c + 1
        result.append(row)
        r = r + 1
    return result


def transpose(m: list[list[int]]) -> list[list[int]]:
    """Transpose a matrix (swap rows and columns)."""
    if len(m) == 0:
        return []
    rows: int = len(m)
    cols: int = len(m[0])
    result: list[list[int]] = []
    c: int = 0
    while c < cols:
        row: list[int] = []
        r: int = 0
        while r < rows:
            row.append(m[r][c])
            r = r + 1
        result.append(row)
        c = c + 1
    return result


def matrix_trace(m: list[list[int]]) -> int:
    """Compute trace (sum of diagonal elements) of a square matrix."""
    total: int = 0
    i: int = 0
    while i < len(m):
        total = total + m[i][i]
        i = i + 1
    return total


def test_module() -> int:
    """Test matrix operations."""
    ok: int = 0

    a: list[list[int]] = [[1, 2], [3, 4]]
    b: list[list[int]] = [[5, 6], [7, 8]]

    if matrix_add(a, b) == [[6, 8], [10, 12]]:
        ok = ok + 1

    if matrix_sub(b, a) == [[4, 4], [4, 4]]:
        ok = ok + 1

    if scalar_multiply(a, 3) == [[3, 6], [9, 12]]:
        ok = ok + 1

    if transpose(a) == [[1, 3], [2, 4]]:
        ok = ok + 1

    r: list[list[int]] = [[1, 2, 3]]
    if transpose(r) == [[1], [2], [3]]:
        ok = ok + 1

    if matrix_trace(a) == 5:
        ok = ok + 1

    if transpose([]) == []:
        ok = ok + 1

    return ok
