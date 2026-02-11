"""Gram-Schmidt orthogonalization using integer-scaled arithmetic."""


def dot_product(a: list[int], b: list[int]) -> int:
    """Dot product of two vectors."""
    n: int = len(a)
    result: int = 0
    i: int = 0
    while i < n:
        result = result + a[i] * b[i]
        i = i + 1
    return result


def vec_scale(v: list[int], s: int) -> list[int]:
    """Multiply vector by scalar."""
    result: list[int] = []
    i: int = 0
    n: int = len(v)
    while i < n:
        result.append(v[i] * s)
        i = i + 1
    return result


def vec_sub(a: list[int], b: list[int]) -> list[int]:
    """Subtract vector b from a."""
    result: list[int] = []
    i: int = 0
    n: int = len(a)
    while i < n:
        result.append(a[i] - b[i])
        i = i + 1
    return result


def are_orthogonal(a: list[int], b: list[int]) -> int:
    """Returns 1 if dot product is zero."""
    if dot_product(a, b) == 0:
        return 1
    return 0


def gram_schmidt_2d(v1: list[int], v2: list[int]) -> list[int]:
    """Orthogonalize two 2D vectors. Returns [u1x,u1y,u2x,u2y] scaled."""
    d11: int = dot_product(v1, v1)
    d21: int = dot_product(v2, v1)
    s1: list[int] = vec_scale(v2, d11)
    s2: list[int] = vec_scale(v1, d21)
    u2: list[int] = vec_sub(s1, s2)
    return [v1[0], v1[1], u2[0], u2[1]]


def test_module() -> int:
    """Test Gram-Schmidt functions."""
    ok: int = 0
    a: list[int] = [3, 1]
    b: list[int] = [2, 2]
    gs: list[int] = gram_schmidt_2d(a, b)
    u1x: int = gs[0]
    u1y: int = gs[1]
    u2x: int = gs[2]
    u2y: int = gs[3]
    d: int = u1x * u2x + u1y * u2y
    if d == 0:
        ok = ok + 1
    if are_orthogonal([1, 0], [0, 1]) == 1:
        ok = ok + 1
    if are_orthogonal([1, 1], [1, 0]) == 0:
        ok = ok + 1
    if dot_product([2, 2], [3, 1]) == 8:
        ok = ok + 1
    if dot_product([3, 1], [3, 1]) == 10:
        ok = ok + 1
    return ok
