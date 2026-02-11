"""Matrix transformation patterns.

Tests: matrix transpose, rotate 90 degrees, flip horizontal/vertical,
matrix addition, and matrix multiply.
"""


def transpose(matrix: list[list[int]]) -> list[list[int]]:
    """Transpose a matrix."""
    rows: int = len(matrix)
    if rows == 0:
        return []
    cols: int = len(matrix[0])
    result: list[list[int]] = []
    c: int = 0
    while c < cols:
        row: list[int] = []
        r: int = 0
        while r < rows:
            row.append(matrix[r][c])
            r = r + 1
        result.append(row)
        c = c + 1
    return result


def rotate_90_clockwise(matrix: list[list[int]]) -> list[list[int]]:
    """Rotate square matrix 90 degrees clockwise."""
    n: int = len(matrix)
    if n == 0:
        return []
    result: list[list[int]] = []
    r: int = 0
    while r < n:
        row: list[int] = [0] * n
        result.append(row)
        r = r + 1
    i: int = 0
    while i < n:
        j: int = 0
        while j < n:
            result[j][n - 1 - i] = matrix[i][j]
            j = j + 1
        i = i + 1
    return result


def flip_horizontal(matrix: list[list[int]]) -> list[list[int]]:
    """Flip matrix horizontally (left-right)."""
    rows: int = len(matrix)
    if rows == 0:
        return []
    cols: int = len(matrix[0])
    result: list[list[int]] = []
    r: int = 0
    while r < rows:
        row: list[int] = []
        c: int = cols - 1
        while c >= 0:
            row.append(matrix[r][c])
            c = c - 1
        result.append(row)
        r = r + 1
    return result


def matrix_add(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    """Add two matrices element-wise."""
    rows: int = len(a)
    if rows == 0:
        return []
    cols: int = len(a[0])
    result: list[list[int]] = []
    r: int = 0
    while r < rows:
        row: list[int] = []
        c: int = 0
        while c < cols:
            row.append(a[r][c] + b[r][c])
            c = c + 1
        result.append(row)
        r = r + 1
    return result


def matrix_multiply(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    """Multiply two matrices."""
    m: int = len(a)
    if m == 0:
        return []
    k: int = len(a[0])
    n: int = len(b[0])
    result: list[list[int]] = []
    r: int = 0
    while r < m:
        row: list[int] = [0] * n
        result.append(row)
        r = r + 1
    i: int = 0
    while i < m:
        j: int = 0
        while j < n:
            total: int = 0
            p: int = 0
            while p < k:
                total = total + a[i][p] * b[p][j]
                p = p + 1
            result[i][j] = total
            j = j + 1
        i = i + 1
    return result


def test_module() -> bool:
    """Test all matrix transformation functions."""
    ok: bool = True

    m1: list[list[int]] = [[1, 2, 3], [4, 5, 6]]
    t: list[list[int]] = transpose(m1)
    if t != [[1, 4], [2, 5], [3, 6]]:
        ok = False

    sq: list[list[int]] = [[1, 2], [3, 4]]
    rot: list[list[int]] = rotate_90_clockwise(sq)
    if rot != [[3, 1], [4, 2]]:
        ok = False

    fl: list[list[int]] = flip_horizontal([[1, 2, 3], [4, 5, 6]])
    if fl != [[3, 2, 1], [6, 5, 4]]:
        ok = False

    a: list[list[int]] = [[1, 2], [3, 4]]
    b: list[list[int]] = [[5, 6], [7, 8]]
    s: list[list[int]] = matrix_add(a, b)
    if s != [[6, 8], [10, 12]]:
        ok = False

    prod: list[list[int]] = matrix_multiply([[1, 2], [3, 4]], [[5, 6], [7, 8]])
    if prod != [[19, 22], [43, 50]]:
        ok = False

    return ok
