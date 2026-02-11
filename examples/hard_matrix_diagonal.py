"""Diagonal matrix operations: extract, sum, and manipulate diagonals."""


def extract_main_diagonal(matrix: list[int], rows: int, cols: int) -> list[int]:
    """Extract main diagonal from a flat matrix."""
    result: list[int] = []
    i: int = 0
    limit: int = rows
    if cols < limit:
        limit = cols
    while i < limit:
        idx: int = i * cols + i
        result.append(matrix[idx])
        i = i + 1
    return result


def extract_anti_diagonal(matrix: list[int], rows: int, cols: int) -> list[int]:
    """Extract anti-diagonal from a flat matrix."""
    result: list[int] = []
    i: int = 0
    limit: int = rows
    if cols < limit:
        limit = cols
    while i < limit:
        col_idx: int = cols - 1 - i
        idx: int = i * cols + col_idx
        result.append(matrix[idx])
        i = i + 1
    return result


def diagonal_sum(matrix: list[int], size: int) -> int:
    """Sum both diagonals of a square matrix. Center counted once if odd size."""
    total: int = 0
    i: int = 0
    while i < size:
        main_idx: int = i * size + i
        total = total + matrix[main_idx]
        anti_col: int = size - 1 - i
        anti_idx: int = i * size + anti_col
        total = total + matrix[anti_idx]
        i = i + 1
    if size % 2 == 1:
        center: int = size // 2
        center_idx: int = center * size + center
        total = total - matrix[center_idx]
    return total


def is_diagonally_dominant(matrix: list[int], size: int) -> int:
    """Check if a square matrix is diagonally dominant.
    Returns 1 if true, 0 otherwise."""
    i: int = 0
    while i < size:
        diag_idx: int = i * size + i
        diag_val: int = matrix[diag_idx]
        if diag_val < 0:
            diag_val = -diag_val
        row_sum: int = 0
        j: int = 0
        while j < size:
            if j != i:
                idx: int = i * size + j
                val: int = matrix[idx]
                if val < 0:
                    val = -val
                row_sum = row_sum + val
            j = j + 1
        if diag_val < row_sum:
            return 0
        i = i + 1
    return 1


def test_module() -> int:
    """Test diagonal matrix operations."""
    ok: int = 0

    mat: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9]
    diag: list[int] = extract_main_diagonal(mat, 3, 3)
    if diag[0] == 1 and diag[1] == 5 and diag[2] == 9:
        ok = ok + 1

    anti: list[int] = extract_anti_diagonal(mat, 3, 3)
    if anti[0] == 3 and anti[1] == 5 and anti[2] == 7:
        ok = ok + 1

    if diagonal_sum(mat, 3) == 25:
        ok = ok + 1

    dom_mat: list[int] = [5, 1, 1, 1, 6, 1, 1, 1, 7]
    if is_diagonally_dominant(dom_mat, 3) == 1:
        ok = ok + 1

    non_dom: list[int] = [1, 5, 5, 5, 1, 5, 5, 5, 1]
    if is_diagonally_dominant(non_dom, 3) == 0:
        ok = ok + 1

    rect: list[int] = [1, 2, 3, 4, 5, 6]
    rect_diag: list[int] = extract_main_diagonal(rect, 2, 3)
    if len(rect_diag) == 2 and rect_diag[0] == 1 and rect_diag[1] == 5:
        ok = ok + 1

    return ok
