"""Gaussian elimination on integer matrices (scaled to avoid fractions)."""


def mat_get(mat: list[int], cols: int, r: int, c: int) -> int:
    """Get element from flat matrix."""
    return mat[r * cols + c]


def mat_set(mat: list[int], cols: int, r: int, c: int, val: int) -> int:
    """Set element in flat matrix. Returns 0."""
    mat[r * cols + c] = val
    return 0


def swap_rows(mat: list[int], cols: int, r1: int, r2: int) -> int:
    """Swap two rows in flat matrix. Returns 0."""
    c: int = 0
    while c < cols:
        tmp: int = mat[r1 * cols + c]
        mat[r1 * cols + c] = mat[r2 * cols + c]
        mat[r2 * cols + c] = tmp
        c = c + 1
    return 0


def gauss_forward(mat: list[int], rows: int, cols: int) -> int:
    """Forward elimination. Returns rank."""
    pivot_row: int = 0
    pivot_col: int = 0
    while pivot_row < rows and pivot_col < cols:
        max_row: int = pivot_row
        max_val: int = mat[pivot_row * cols + pivot_col]
        if max_val < 0:
            max_val = 0 - max_val
        r: int = pivot_row + 1
        while r < rows:
            cur: int = mat[r * cols + pivot_col]
            if cur < 0:
                cur = 0 - cur
            if cur > max_val:
                max_val = cur
                max_row = r
            r = r + 1
        if max_val == 0:
            pivot_col = pivot_col + 1
            continue
        if max_row != pivot_row:
            swap_rows(mat, cols, pivot_row, max_row)
        r = pivot_row + 1
        while r < rows:
            factor: int = mat[r * cols + pivot_col]
            pivot_val: int = mat[pivot_row * cols + pivot_col]
            if factor != 0:
                c: int = pivot_col
                while c < cols:
                    scaled: int = mat[r * cols + c] * pivot_val - factor * mat[pivot_row * cols + c]
                    mat[r * cols + c] = scaled
                    c = c + 1
            r = r + 1
        pivot_row = pivot_row + 1
        pivot_col = pivot_col + 1
    return pivot_row


def compute_rank(mat: list[int], rows: int, cols: int) -> int:
    """Compute rank of matrix via Gaussian elimination."""
    dup: list[int] = []
    i: int = 0
    while i < len(mat):
        dup.append(mat[i])
        i = i + 1
    return gauss_forward(dup, rows, cols)


def test_module() -> int:
    """Test Gaussian elimination."""
    ok: int = 0
    m1: list[int] = [1, 0, 0, 0, 1, 0, 0, 0, 1]
    if compute_rank(m1, 3, 3) == 3:
        ok = ok + 1
    m2: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9]
    if compute_rank(m2, 3, 3) == 2:
        ok = ok + 1
    m3: list[int] = [0, 0, 0, 0]
    if compute_rank(m3, 2, 2) == 0:
        ok = ok + 1
    m4: list[int] = [1, 2, 2, 4]
    if compute_rank(m4, 2, 2) == 1:
        ok = ok + 1
    m5: list[int] = [1, 0, 0, 1]
    if compute_rank(m5, 2, 2) == 2:
        ok = ok + 1
    m6: list[int] = [2, 4, 1, 2]
    if compute_rank(m6, 2, 2) == 1:
        ok = ok + 1
    m7: list[int] = [1, 2, 3, 4, 5, 6]
    if compute_rank(m7, 2, 3) == 2:
        ok = ok + 1
    return ok
