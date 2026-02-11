"""Simple SVD-related computations for 2x2 integer matrices."""


def mat_transpose_2x2(m: list[int]) -> list[int]:
    """Transpose 2x2 flat matrix."""
    return [m[0], m[2], m[1], m[3]]


def mat_mult_2x2(a: list[int], b: list[int]) -> list[int]:
    """Multiply two 2x2 matrices."""
    r0: int = a[0] * b[0] + a[1] * b[2]
    r1: int = a[0] * b[1] + a[1] * b[3]
    r2: int = a[2] * b[0] + a[3] * b[2]
    r3: int = a[2] * b[1] + a[3] * b[3]
    return [r0, r1, r2, r3]


def gram_matrix(m: list[int]) -> list[int]:
    """Compute M^T * M (Gram matrix)."""
    mt: list[int] = mat_transpose_2x2(m)
    return mat_mult_2x2(mt, m)


def trace_2x2(m: list[int]) -> int:
    """Trace of 2x2 matrix."""
    return m[0] + m[3]


def det_2x2(m: list[int]) -> int:
    """Determinant of 2x2 matrix."""
    return m[0] * m[3] - m[1] * m[2]


def frobenius_norm_sq(m: list[int]) -> int:
    """Frobenius norm squared of 2x2 matrix."""
    return m[0] * m[0] + m[1] * m[1] + m[2] * m[2] + m[3] * m[3]


def singular_values_sum_sq(m: list[int]) -> int:
    """Sum of squared singular values = trace(M^T M) = Frobenius^2."""
    g: list[int] = gram_matrix(m)
    return trace_2x2(g)


def singular_values_product_sq(m: list[int]) -> int:
    """Product of squared singular values = det(M^T M) = det(M)^2."""
    d: int = det_2x2(m)
    return d * d


def is_orthogonal_2x2(m: list[int]) -> int:
    """Check if 2x2 matrix is orthogonal: M^T M = I (scaled)."""
    g: list[int] = gram_matrix(m)
    if g[0] != g[3]:
        return 0
    if g[1] != 0:
        return 0
    if g[2] != 0:
        return 0
    return 1


def test_module() -> int:
    """Test SVD-related computations."""
    ok: int = 0
    m: list[int] = [1, 2, 3, 4]
    g: list[int] = gram_matrix(m)
    if g[0] == 10:
        ok = ok + 1
    if trace_2x2(m) == 5:
        ok = ok + 1
    if frobenius_norm_sq(m) == 30:
        ok = ok + 1
    if singular_values_sum_sq(m) == 30:
        ok = ok + 1
    orth: list[int] = [1, 0, 0, 1]
    if is_orthogonal_2x2(orth) == 1:
        ok = ok + 1
    return ok
