"""Matrix determinant calculation for 2x2 and 3x3 matrices.

Tests: det2x2, det3x3, singular matrix detection, identity determinant.
"""


def det2x2(a: int, b: int, c: int, d: int) -> int:
    """Calculate determinant of 2x2 matrix [[a,b],[c,d]]."""
    return a * d - b * c


def mat3x3_det(m: list[list[int]]) -> int:
    """Calculate determinant of 3x3 matrix using cofactor expansion."""
    a: int = m[0][0]
    b: int = m[0][1]
    c: int = m[0][2]
    d: int = m[1][0]
    e: int = m[1][1]
    f: int = m[1][2]
    g: int = m[2][0]
    h: int = m[2][1]
    i: int = m[2][2]
    return a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)


def is_singular(m: list[list[int]]) -> int:
    """Check if a 3x3 matrix is singular (det == 0). Returns 1 if singular."""
    if mat3x3_det(m) == 0:
        return 1
    return 0


def minor2x2(m: list[list[int]], row: int, col: int) -> int:
    """Get the minor of element at (row, col) in a 3x3 matrix."""
    vals: list[int] = []
    r: int = 0
    while r < 3:
        if r != row:
            cc: int = 0
            while cc < 3:
                if cc != col:
                    vals.append(m[r][cc])
                cc = cc + 1
        r = r + 1
    return vals[0] * vals[3] - vals[1] * vals[2]


def cofactor(m: list[list[int]], row: int, col: int) -> int:
    """Get cofactor of element at (row, col)."""
    sign: int = 1
    if (row + col) % 2 == 1:
        sign = -1
    return sign * minor2x2(m, row, col)


def test_module() -> int:
    """Test matrix determinant calculations."""
    ok: int = 0

    if det2x2(1, 2, 3, 4) == -2:
        ok = ok + 1

    if det2x2(2, 0, 0, 2) == 4:
        ok = ok + 1

    identity: list[list[int]] = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
    if mat3x3_det(identity) == 1:
        ok = ok + 1

    m1: list[list[int]] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    if mat3x3_det(m1) == 0:
        ok = ok + 1

    if is_singular(m1) == 1:
        ok = ok + 1

    if is_singular(identity) == 0:
        ok = ok + 1

    m2: list[list[int]] = [[6, 1, 1], [4, -2, 5], [2, 8, 7]]
    if mat3x3_det(m2) == -306:
        ok = ok + 1

    if cofactor(identity, 0, 0) == 1:
        ok = ok + 1

    return ok
