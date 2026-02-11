"""Check if a matrix is a magic square."""


def row_sum(mat: list[int], n: int, row: int) -> int:
    """Sum of row in n x n flat matrix."""
    total: int = 0
    c: int = 0
    while c < n:
        total = total + mat[row * n + c]
        c = c + 1
    return total


def col_sum(mat: list[int], n: int, col: int) -> int:
    """Sum of column in n x n flat matrix."""
    total: int = 0
    r: int = 0
    while r < n:
        total = total + mat[r * n + col]
        r = r + 1
    return total


def main_diag_sum(mat: list[int], n: int) -> int:
    """Sum of main diagonal."""
    total: int = 0
    i: int = 0
    while i < n:
        total = total + mat[i * n + i]
        i = i + 1
    return total


def anti_diag_sum(mat: list[int], n: int) -> int:
    """Sum of anti-diagonal."""
    total: int = 0
    i: int = 0
    while i < n:
        total = total + mat[i * n + (n - 1 - i)]
        i = i + 1
    return total


def is_magic_square(mat: list[int], n: int) -> int:
    """Returns 1 if mat is a magic square."""
    if n == 0:
        return 0
    target: int = row_sum(mat, n, 0)
    r: int = 1
    while r < n:
        if row_sum(mat, n, r) != target:
            return 0
        r = r + 1
    c: int = 0
    while c < n:
        if col_sum(mat, n, c) != target:
            return 0
        c = c + 1
    if main_diag_sum(mat, n) != target:
        return 0
    if anti_diag_sum(mat, n) != target:
        return 0
    return 1


def magic_constant(n: int) -> int:
    """The magic constant for n x n magic square: n*(n*n+1)/2."""
    return n * (n * n + 1) // 2


def test_module() -> int:
    """Test magic square checking."""
    ok: int = 0
    m1: list[int] = [2, 7, 6, 9, 5, 1, 4, 3, 8]
    if is_magic_square(m1, 3) == 1:
        ok = ok + 1
    m2: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9]
    if is_magic_square(m2, 3) == 0:
        ok = ok + 1
    if magic_constant(3) == 15:
        ok = ok + 1
    if magic_constant(4) == 34:
        ok = ok + 1
    if main_diag_sum(m1, 3) == 15:
        ok = ok + 1
    if anti_diag_sum(m1, 3) == 15:
        ok = ok + 1
    m3: list[int] = [1]
    if is_magic_square(m3, 1) == 1:
        ok = ok + 1
    return ok
