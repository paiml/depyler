"""Matrix exponentiation for 2x2 integer matrices."""


def mat2_mul(a: list[int], b: list[int]) -> list[int]:
    """Multiply two 2x2 matrices stored as flat list[4]."""
    result: list[int] = []
    r00: int = a[0] * b[0] + a[1] * b[2]
    r01: int = a[0] * b[1] + a[1] * b[3]
    r10: int = a[2] * b[0] + a[3] * b[2]
    r11: int = a[2] * b[1] + a[3] * b[3]
    result.append(r00)
    result.append(r01)
    result.append(r10)
    result.append(r11)
    return result


def mat2_identity() -> list[int]:
    """Return 2x2 identity matrix."""
    result: list[int] = [1, 0, 0, 1]
    return result


def mat2_pow(m: list[int], exp: int) -> list[int]:
    """Compute m^exp for 2x2 matrix using binary exponentiation."""
    result: list[int] = mat2_identity()
    cur: list[int] = []
    cur.append(m[0])
    cur.append(m[1])
    cur.append(m[2])
    cur.append(m[3])
    p: int = exp
    while p > 0:
        if p % 2 == 1:
            result = mat2_mul(result, cur)
        cur = mat2_mul(cur, cur)
        p = p // 2
    return result


def fibonacci_matrix(n: int) -> int:
    """Compute nth Fibonacci number using matrix exponentiation."""
    if n <= 0:
        return 0
    if n == 1:
        return 1
    fib_mat: list[int] = [1, 1, 1, 0]
    result: list[int] = mat2_pow(fib_mat, n - 1)
    return result[0]


def mat2_trace(m: list[int]) -> int:
    """Trace of 2x2 matrix."""
    return m[0] + m[3]


def mat2_det(m: list[int]) -> int:
    """Determinant of 2x2 matrix."""
    return m[0] * m[3] - m[1] * m[2]


def test_module() -> int:
    """Test matrix exponentiation."""
    ok: int = 0
    identity: list[int] = mat2_identity()
    if identity[0] == 1 and identity[3] == 1:
        ok = ok + 1
    a: list[int] = [1, 2, 3, 4]
    a2: list[int] = mat2_mul(a, a)
    if a2[0] == 7:
        ok = ok + 1
    if fibonacci_matrix(1) == 1:
        ok = ok + 1
    if fibonacci_matrix(6) == 8:
        ok = ok + 1
    if fibonacci_matrix(10) == 55:
        ok = ok + 1
    if mat2_det(identity) == 1:
        ok = ok + 1
    if mat2_trace(a) == 5:
        ok = ok + 1
    return ok
