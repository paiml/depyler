"""Matrix determinant for 2x2 and 3x3 matrices (flattened arrays).

Tests: determinant, trace, transpose, is_identity.
"""


def det_2x2(m: list[int]) -> int:
    """Determinant of 2x2 matrix stored as [a,b,c,d] -> ad-bc."""
    return m[0] * m[3] - m[1] * m[2]


def det_3x3(m: list[int]) -> int:
    """Determinant of 3x3 matrix stored row-major (9 elements)."""
    a: int = m[0] * (m[4] * m[8] - m[5] * m[7])
    b: int = m[1] * (m[3] * m[8] - m[5] * m[6])
    c: int = m[2] * (m[3] * m[7] - m[4] * m[6])
    return a - b + c


def trace_nxn(m: list[int], n: int) -> int:
    """Trace (sum of diagonal) of n x n matrix."""
    total: int = 0
    i: int = 0
    while i < n:
        total = total + m[i * n + i]
        i = i + 1
    return total


def transpose_nxn(m: list[int], n: int) -> list[int]:
    """Transpose an n x n matrix."""
    result: list[int] = [0] * (n * n)
    i: int = 0
    while i < n:
        j: int = 0
        while j < n:
            result[j * n + i] = m[i * n + j]
            j = j + 1
        i = i + 1
    return result


def is_identity_val(m: list[int], n: int) -> int:
    """Check if matrix is identity. Returns 1 if yes, 0 if no."""
    i: int = 0
    while i < n:
        j: int = 0
        while j < n:
            expected: int = 0
            if i == j:
                expected = 1
            if m[i * n + j] != expected:
                return 0
            j = j + 1
        i = i + 1
    return 1


def test_module() -> None:
    m2: list[int] = [1, 2, 3, 4]
    assert det_2x2(m2) == -2
    m3: list[int] = [1, 2, 3, 0, 1, 4, 5, 6, 0]
    assert det_3x3(m3) == 1
    assert trace_nxn(m3, 3) == 2
    t: list[int] = transpose_nxn([1, 2, 3, 4], 2)
    assert t[0] == 1
    assert t[1] == 3
    assert t[2] == 2
    assert t[3] == 4
    ident: list[int] = [1, 0, 0, 0, 1, 0, 0, 0, 1]
    assert is_identity_val(ident, 3) == 1
    assert is_identity_val(m3, 3) == 0
    assert det_2x2([1, 0, 0, 1]) == 1
