"""Matrix transpose, symmetry check, and trace computation."""


def mat_get(m: list[int], cols: int, r: int, c: int) -> int:
    """Get element at row r, col c."""
    idx: int = r * cols + c
    return m[idx]


def mat_transpose(m: list[int], rows: int, cols: int) -> list[int]:
    """Transpose matrix (rows x cols) to (cols x rows)."""
    result: list[int] = []
    total: int = rows * cols
    i: int = 0
    while i < total:
        result.append(0)
        i = i + 1
    r: int = 0
    while r < rows:
        c: int = 0
        while c < cols:
            val: int = mat_get(m, cols, r, c)
            idx: int = c * rows + r
            result[idx] = val
            c = c + 1
        r = r + 1
    return result


def mat_trace(m: list[int], n: int) -> int:
    """Trace of n x n square matrix."""
    total: int = 0
    i: int = 0
    while i < n:
        total = total + mat_get(m, n, i, i)
        i = i + 1
    return total


def is_symmetric(m: list[int], n: int) -> int:
    """Returns 1 if n x n matrix is symmetric."""
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            aij: int = mat_get(m, n, i, j)
            aji: int = mat_get(m, n, j, i)
            if aij != aji:
                return 0
            j = j + 1
        i = i + 1
    return 1


def mat_add(a: list[int], b: list[int], rows: int, cols: int) -> list[int]:
    """Add two matrices element-wise."""
    result: list[int] = []
    total: int = rows * cols
    i: int = 0
    while i < total:
        result.append(a[i] + b[i])
        i = i + 1
    return result


def test_module() -> int:
    """Test transpose, trace, symmetry."""
    ok: int = 0
    m: list[int] = [1, 2, 3, 4, 5, 6]
    t: list[int] = mat_transpose(m, 2, 3)
    if mat_get(t, 2, 0, 0) == 1:
        ok = ok + 1
    if mat_get(t, 2, 1, 0) == 2:
        ok = ok + 1
    sq: list[int] = [1, 2, 2, 3]
    if mat_trace(sq, 2) == 4:
        ok = ok + 1
    if is_symmetric(sq, 2) == 1:
        ok = ok + 1
    nsq: list[int] = [1, 2, 3, 4]
    if is_symmetric(nsq, 2) == 0:
        ok = ok + 1
    return ok
