"""Matrix trace and diagonal operations.

Tests: trace, anti-diagonal sum, diagonal extraction.
"""


def matrix_trace(mat: list[int], n: int) -> int:
    """Compute trace of n x n matrix stored as flat array."""
    total: int = 0
    i: int = 0
    while i < n:
        total = total + mat[i * n + i]
        i = i + 1
    return total


def anti_diagonal_sum(mat: list[int], n: int) -> int:
    """Sum of anti-diagonal elements (top-right to bottom-left)."""
    total: int = 0
    i: int = 0
    while i < n:
        total = total + mat[i * n + (n - 1 - i)]
        i = i + 1
    return total


def extract_diagonal(mat: list[int], n: int) -> list[int]:
    """Extract main diagonal of n x n matrix."""
    diag: list[int] = []
    i: int = 0
    while i < n:
        diag.append(mat[i * n + i])
        i = i + 1
    return diag


def is_diagonal_dominant(mat: list[int], n: int) -> int:
    """Check if matrix is diagonally dominant. Returns 1 if yes."""
    i: int = 0
    while i < n:
        diag_val: int = mat[i * n + i]
        if diag_val < 0:
            diag_val = -diag_val
        row_sum: int = 0
        j: int = 0
        while j < n:
            if j != i:
                val: int = mat[i * n + j]
                if val < 0:
                    val = -val
                row_sum = row_sum + val
            j = j + 1
        if diag_val < row_sum:
            return 0
        i = i + 1
    return 1


def test_module() -> int:
    """Test matrix trace operations."""
    ok: int = 0
    mat: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9]
    if matrix_trace(mat, 3) == 15:
        ok = ok + 1
    if anti_diagonal_sum(mat, 3) == 15:
        ok = ok + 1
    diag: list[int] = extract_diagonal(mat, 3)
    if diag[0] == 1:
        ok = ok + 1
    if diag[1] == 5:
        ok = ok + 1
    if diag[2] == 9:
        ok = ok + 1
    dom: list[int] = [5, 1, 1, 1, 6, 1, 1, 1, 7]
    if is_diagonal_dominant(dom, 3) == 1:
        ok = ok + 1
    return ok
