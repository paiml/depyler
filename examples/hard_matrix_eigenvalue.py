"""Matrix eigenvalue approximation: power iteration and related computations."""


def matrix_vector_mult_2x2(m: list[int], v: list[int]) -> list[int]:
    """Multiply a 2x2 flat matrix by a 2-element vector."""
    r0: int = m[0] * v[0] + m[1] * v[1]
    r1: int = m[2] * v[0] + m[3] * v[1]
    result: list[int] = [r0, r1]
    return result


def vector_max_abs(v: list[int]) -> int:
    """Find maximum absolute value in vector."""
    if len(v) == 0:
        return 0
    max_val: int = v[0]
    if max_val < 0:
        max_val = -max_val
    i: int = 1
    while i < len(v):
        val: int = v[i]
        if val < 0:
            val = -val
        if val > max_val:
            max_val = val
        i = i + 1
    return max_val


def power_iteration_2x2(m: list[int], iterations: int) -> int:
    """Approximate dominant eigenvalue of 2x2 matrix using power iteration.
    Returns eigenvalue * 100 for precision."""
    v: list[int] = [100, 100]
    step: int = 0
    eigenvalue_x100: int = 0
    while step < iterations:
        new_v: list[int] = matrix_vector_mult_2x2(m, v)
        max_val: int = vector_max_abs(new_v)
        if max_val == 0:
            return 0
        eigenvalue_x100 = max_val
        # Normalize: scale down to prevent overflow
        v[0] = new_v[0] * 100 // max_val
        v[1] = new_v[1] * 100 // max_val
        step = step + 1
    return eigenvalue_x100


def matrix_trace_2x2(m: list[int]) -> int:
    """Trace of a 2x2 matrix."""
    return m[0] + m[3]


def matrix_det_2x2(m: list[int]) -> int:
    """Determinant of a 2x2 matrix."""
    return m[0] * m[3] - m[1] * m[2]


def test_module() -> int:
    """Test matrix eigenvalue functions."""
    ok: int = 0

    m1: list[int] = [2, 0, 0, 1]
    v1: list[int] = [3, 4]
    mv: list[int] = matrix_vector_mult_2x2(m1, v1)
    if mv[0] == 6 and mv[1] == 4:
        ok = ok + 1

    vals: list[int] = [-5, 3, -1]
    if vector_max_abs(vals) == 5:
        ok = ok + 1

    # For diagonal matrix [[3,0],[0,1]], dominant eigenvalue is 3
    m2: list[int] = [3, 0, 0, 1]
    ev: int = power_iteration_2x2(m2, 10)
    if ev >= 290 and ev <= 310:
        ok = ok + 1

    if matrix_trace_2x2(m2) == 4:
        ok = ok + 1

    if matrix_det_2x2(m2) == 3:
        ok = ok + 1

    m3: list[int] = [1, 2, 3, 4]
    if matrix_det_2x2(m3) == -2:
        ok = ok + 1

    return ok
