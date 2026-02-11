"""Matrix inverse for 2x2 integer matrices (scaled by determinant)."""


def det_2x2(m: list[int]) -> int:
    """Determinant of 2x2 flat matrix."""
    return m[0] * m[3] - m[1] * m[2]


def adjugate_2x2(m: list[int]) -> list[int]:
    """Adjugate (classical adjoint) of 2x2 matrix."""
    return [m[3], 0 - m[1], 0 - m[2], m[0]]


def inverse_2x2_scaled(m: list[int]) -> list[int]:
    """Return adjugate of 2x2 matrix (inverse * det). Avoids fractions."""
    return adjugate_2x2(m)


def mat_mult_2x2(a: list[int], b: list[int]) -> list[int]:
    """Multiply two 2x2 flat matrices."""
    r0: int = a[0] * b[0] + a[1] * b[2]
    r1: int = a[0] * b[1] + a[1] * b[3]
    r2: int = a[2] * b[0] + a[3] * b[2]
    r3: int = a[2] * b[1] + a[3] * b[3]
    return [r0, r1, r2, r3]


def verify_inverse_2x2(m: list[int]) -> int:
    """Verify M * adj(M) = det(M) * I. Returns 1 if correct."""
    adj: list[int] = adjugate_2x2(m)
    prod: list[int] = mat_mult_2x2(m, adj)
    d: int = det_2x2(m)
    if prod[0] != d:
        return 0
    if prod[3] != d:
        return 0
    if prod[1] != 0:
        return 0
    if prod[2] != 0:
        return 0
    return 1


def cofactor_3x3(m: list[int], r: int, c: int) -> int:
    """Compute cofactor C(r,c) for 3x3 matrix."""
    sub: list[int] = []
    i: int = 0
    while i < 3:
        if i != r:
            j: int = 0
            while j < 3:
                if j != c:
                    idx: int = i * 3 + j
                    sub.append(m[idx])
                j = j + 1
        i = i + 1
    d: int = sub[0] * sub[3] - sub[1] * sub[2]
    sign: int = 1
    if (r + c) % 2 == 1:
        sign = 0 - 1
    return sign * d


def test_module() -> int:
    """Test matrix inverse functions."""
    ok: int = 0
    m: list[int] = [4, 7, 2, 6]
    d: int = det_2x2(m)
    if d == 10:
        ok = ok + 1
    adj: list[int] = adjugate_2x2(m)
    if adj[0] == 6:
        ok = ok + 1
    if verify_inverse_2x2(m) == 1:
        ok = ok + 1
    m3: list[int] = [1, 0, 0, 0, 1, 0, 0, 0, 1]
    cf: int = cofactor_3x3(m3, 0, 0)
    if cf == 1:
        ok = ok + 1
    cf2: int = cofactor_3x3(m3, 0, 1)
    if cf2 == 0:
        ok = ok + 1
    return ok
