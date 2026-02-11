"""Matrix multiplication.

Tests: multiply 2D matrices, scalar multiply, matrix power, identity check.
"""


def matrix_multiply(a: list[list[int]], b: list[list[int]], m: int, n: int, p: int) -> list[list[int]]:
    """Multiply m x n matrix A by n x p matrix B."""
    result: list[list[int]] = []
    i: int = 0
    while i < m:
        row: list[int] = []
        j: int = 0
        while j < p:
            total: int = 0
            k: int = 0
            while k < n:
                total = total + a[i][k] * b[k][j]
                k = k + 1
            row.append(total)
            j = j + 1
        result.append(row)
        i = i + 1
    return result


def scalar_multiply(matrix: list[list[int]], scalar: int, rows: int, cols: int) -> list[list[int]]:
    """Multiply every element by scalar."""
    result: list[list[int]] = []
    i: int = 0
    while i < rows:
        row: list[int] = []
        j: int = 0
        while j < cols:
            row.append(matrix[i][j] * scalar)
            j = j + 1
        result.append(row)
        i = i + 1
    return result


def is_identity(matrix: list[list[int]], n: int) -> int:
    """Check if matrix is n x n identity. Returns 1 or 0."""
    i: int = 0
    while i < n:
        j: int = 0
        while j < n:
            if i == j:
                if matrix[i][j] != 1:
                    return 0
            else:
                if matrix[i][j] != 0:
                    return 0
            j = j + 1
        i = i + 1
    return 1


def matrix_trace(matrix: list[list[int]], n: int) -> int:
    """Sum of diagonal elements."""
    total: int = 0
    i: int = 0
    while i < n:
        total = total + matrix[i][i]
        i = i + 1
    return total


def test_module() -> int:
    """Test matrix multiplication operations."""
    ok: int = 0
    a: list[list[int]] = [[1, 2], [3, 4]]
    b: list[list[int]] = [[5, 6], [7, 8]]
    c: list[list[int]] = matrix_multiply(a, b, 2, 2, 2)
    if c[0][0] == 19 and c[0][1] == 22:
        ok = ok + 1
    if c[1][0] == 43 and c[1][1] == 50:
        ok = ok + 1
    s: list[list[int]] = scalar_multiply(a, 3, 2, 2)
    if s[0][0] == 3 and s[1][1] == 12:
        ok = ok + 1
    ident: list[list[int]] = [[1, 0], [0, 1]]
    if is_identity(ident, 2) == 1:
        ok = ok + 1
    if is_identity(a, 2) == 0:
        ok = ok + 1
    if matrix_trace(a, 2) == 5:
        ok = ok + 1
    return ok
