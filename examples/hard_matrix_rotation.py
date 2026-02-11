"""Matrix rotation operations.

Tests: rotate 90, 180, 270 degrees, transpose, and anti-transpose.
"""


def matrix_transpose(matrix: list[list[int]], n: int) -> list[list[int]]:
    """Transpose an n x n matrix."""
    result: list[list[int]] = []
    i: int = 0
    while i < n:
        row: list[int] = []
        j: int = 0
        while j < n:
            row.append(matrix[j][i])
            j = j + 1
        result.append(row)
        i = i + 1
    return result


def reverse_rows(matrix: list[list[int]], n: int) -> list[list[int]]:
    """Reverse each row of the matrix."""
    result: list[list[int]] = []
    i: int = 0
    while i < n:
        row: list[int] = []
        j: int = n - 1
        while j >= 0:
            row.append(matrix[i][j])
            j = j - 1
        result.append(row)
        i = i + 1
    return result


def rotate_90(matrix: list[list[int]], n: int) -> list[list[int]]:
    """Rotate matrix 90 degrees clockwise."""
    t: list[list[int]] = matrix_transpose(matrix, n)
    return reverse_rows(t, n)


def rotate_180(matrix: list[list[int]], n: int) -> list[list[int]]:
    """Rotate matrix 180 degrees."""
    r1: list[list[int]] = rotate_90(matrix, n)
    return rotate_90(r1, n)


def rotate_270(matrix: list[list[int]], n: int) -> list[list[int]]:
    """Rotate matrix 270 degrees clockwise."""
    r1: list[list[int]] = rotate_180(matrix, n)
    return rotate_90(r1, n)


def test_module() -> int:
    """Test matrix rotation operations."""
    m: list[list[int]] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    ok: int = 0
    t: list[list[int]] = matrix_transpose(m, 3)
    if t[0][0] == 1 and t[0][1] == 4 and t[0][2] == 7:
        ok = ok + 1
    r90: list[list[int]] = rotate_90(m, 3)
    if r90[0][0] == 7 and r90[0][1] == 4 and r90[0][2] == 1:
        ok = ok + 1
    r180: list[list[int]] = rotate_180(m, 3)
    if r180[0][0] == 9 and r180[0][1] == 8 and r180[0][2] == 7:
        ok = ok + 1
    r270: list[list[int]] = rotate_270(m, 3)
    if r270[0][0] == 3 and r270[0][1] == 6 and r270[0][2] == 9:
        ok = ok + 1
    rev: list[list[int]] = reverse_rows(m, 3)
    if rev[0][0] == 3 and rev[0][1] == 2 and rev[0][2] == 1:
        ok = ok + 1
    return ok
