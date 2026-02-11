"""Matrix determinant via cofactor expansion (integer matrices)."""


def mat_get(m: list[int], cols: int, r: int, c: int) -> int:
    """Get element at row r, col c."""
    idx: int = r * cols + c
    return m[idx]


def submatrix(m: list[int], n: int, skip_r: int, skip_c: int) -> list[int]:
    """Build (n-1)x(n-1) submatrix excluding row skip_r, col skip_c."""
    result: list[int] = []
    r: int = 0
    while r < n:
        if r != skip_r:
            c: int = 0
            while c < n:
                if c != skip_c:
                    idx: int = r * n + c
                    val: int = m[idx]
                    result.append(val)
                c = c + 1
        r = r + 1
    return result


def determinant(m: list[int], n: int) -> int:
    """Compute determinant of n x n integer matrix."""
    if n == 1:
        return m[0]
    if n == 2:
        return m[0] * m[3] - m[1] * m[2]
    det: int = 0
    sign: int = 1
    j: int = 0
    while j < n:
        sub: list[int] = submatrix(m, n, 0, j)
        cofactor: int = sign * m[j] * determinant(sub, n - 1)
        det = det + cofactor
        sign = 0 - sign
        j = j + 1
    return det


def det_2x2(a: int, b: int, c: int, d: int) -> int:
    """Quick 2x2 determinant."""
    return a * d - b * c


def det_3x3_direct(m: list[int]) -> int:
    """Direct Sarrus rule for 3x3."""
    a: int = m[0]
    b: int = m[1]
    c: int = m[2]
    d: int = m[3]
    e: int = m[4]
    f: int = m[5]
    g: int = m[6]
    h: int = m[7]
    ii: int = m[8]
    return a * e * ii + b * f * g + c * d * h - c * e * g - b * d * ii - a * f * h


def test_module() -> int:
    """Test determinant functions."""
    ok: int = 0
    if det_2x2(1, 2, 3, 4) == 0 - 2:
        ok = ok + 1
    m2: list[int] = [1, 2, 3, 4]
    if determinant(m2, 2) == 0 - 2:
        ok = ok + 1
    m3: list[int] = [1, 2, 3, 0, 1, 4, 5, 6, 0]
    if determinant(m3, 3) == 1:
        ok = ok + 1
    if det_3x3_direct(m3) == 1:
        ok = ok + 1
    ident: list[int] = [1, 0, 0, 0, 1, 0, 0, 0, 1]
    if determinant(ident, 3) == 1:
        ok = ok + 1
    return ok
