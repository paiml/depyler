"""Traverse a matrix diagonally (flat representation)."""


def get_element(mat: list[int], rows: int, cols: int, r: int, c: int) -> int:
    """Get element at (r, c) from flat matrix."""
    return mat[r * cols + c]


def diagonal_traverse(mat: list[int], rows: int, cols: int) -> list[int]:
    """Traverse matrix in diagonal order (top-right to bottom-left)."""
    result: list[int] = []
    d: int = 0
    total_diags: int = rows + cols - 1
    while d < total_diags:
        if d % 2 == 0:
            r: int = d
            if r >= rows:
                r = rows - 1
            c: int = d - r
            while r >= 0 and c < cols:
                val: int = get_element(mat, rows, cols, r, c)
                result.append(val)
                r = r - 1
                c = c + 1
        else:
            c2: int = d
            if c2 >= cols:
                c2 = cols - 1
            r2: int = d - c2
            while c2 >= 0 and r2 < rows:
                val2: int = get_element(mat, rows, cols, r2, c2)
                result.append(val2)
                r2 = r2 + 1
                c2 = c2 - 1
        d = d + 1
    return result


def anti_diagonal_sum(mat: list[int], rows: int, cols: int, d: int) -> int:
    """Sum of elements on anti-diagonal d."""
    total: int = 0
    r: int = 0
    while r < rows:
        c: int = d - r
        if c >= 0 and c < cols:
            total = total + get_element(mat, rows, cols, r, c)
        r = r + 1
    return total


def main_diagonal_sum(mat: list[int], n: int) -> int:
    """Sum of main diagonal elements of n x n matrix."""
    total: int = 0
    i: int = 0
    while i < n:
        total = total + mat[i * n + i]
        i = i + 1
    return total


def test_module() -> int:
    """Test diagonal traversal."""
    ok: int = 0
    mat: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9]
    diag: list[int] = diagonal_traverse(mat, 3, 3)
    if len(diag) == 9:
        ok = ok + 1
    if diag[0] == 1:
        ok = ok + 1
    if main_diagonal_sum(mat, 3) == 15:
        ok = ok + 1
    s0: int = anti_diagonal_sum(mat, 3, 3, 0)
    if s0 == 1:
        ok = ok + 1
    s2: int = anti_diagonal_sum(mat, 3, 3, 2)
    if s2 == 15:
        ok = ok + 1
    mat2: list[int] = [1, 2, 3, 4]
    if main_diagonal_sum(mat2, 2) == 5:
        ok = ok + 1
    d2: list[int] = diagonal_traverse(mat2, 2, 2)
    if len(d2) == 4:
        ok = ok + 1
    return ok
