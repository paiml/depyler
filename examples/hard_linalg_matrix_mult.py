"""Matrix multiplication with flat list representation."""


def mat_create(rows: int, cols: int) -> list[int]:
    """Create zero matrix as flat list. Layout: row-major, size rows*cols."""
    result: list[int] = []
    i: int = 0
    total: int = rows * cols
    while i < total:
        result.append(0)
        i = i + 1
    return result


def mat_get(m: list[int], cols: int, r: int, c: int) -> int:
    """Get element at row r, col c from flat matrix with given cols."""
    idx: int = r * cols + c
    return m[idx]


def mat_set(m: list[int], cols: int, r: int, c: int, val: int) -> list[int]:
    """Set element and return updated matrix."""
    idx: int = r * cols + c
    m[idx] = val
    return m


def mat_multiply(a: list[int], b: list[int], ar: int, ac: int, bc: int) -> list[int]:
    """Multiply matrix a (ar x ac) by b (ac x bc). Returns flat result."""
    result: list[int] = mat_create(ar, bc)
    i: int = 0
    while i < ar:
        j: int = 0
        while j < bc:
            s: int = 0
            k: int = 0
            while k < ac:
                av: int = mat_get(a, ac, i, k)
                bv: int = mat_get(b, bc, k, j)
                s = s + av * bv
                k = k + 1
            result = mat_set(result, bc, i, j, s)
            j = j + 1
        i = i + 1
    return result


def mat_identity(n: int) -> list[int]:
    """Create n x n identity matrix."""
    result: list[int] = mat_create(n, n)
    i: int = 0
    while i < n:
        result = mat_set(result, n, i, i, 1)
        i = i + 1
    return result


def test_module() -> int:
    """Test matrix multiplication."""
    ok: int = 0
    a: list[int] = [1, 2, 3, 4]
    b: list[int] = [5, 6, 7, 8]
    c: list[int] = mat_multiply(a, b, 2, 2, 2)
    if mat_get(c, 2, 0, 0) == 19:
        ok = ok + 1
    if mat_get(c, 2, 0, 1) == 22:
        ok = ok + 1
    if mat_get(c, 2, 1, 0) == 43:
        ok = ok + 1
    ident: list[int] = mat_identity(2)
    d: list[int] = mat_multiply(a, ident, 2, 2, 2)
    if mat_get(d, 2, 0, 0) == 1:
        ok = ok + 1
    if mat_get(d, 2, 1, 1) == 4:
        ok = ok + 1
    return ok
