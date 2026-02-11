"""Vector operations: dot product, cross product, magnitude squared."""


def dot_product(a: list[int], b: list[int]) -> int:
    """Compute dot product of two integer vectors."""
    n: int = len(a)
    result: int = 0
    i: int = 0
    while i < n:
        result = result + a[i] * b[i]
        i = i + 1
    return result


def cross_product_3d(a: list[int], b: list[int]) -> list[int]:
    """Cross product of two 3D vectors."""
    x: int = a[1] * b[2] - a[2] * b[1]
    y: int = a[2] * b[0] - a[0] * b[2]
    z: int = a[0] * b[1] - a[1] * b[0]
    return [x, y, z]


def magnitude_squared(v: list[int]) -> int:
    """Compute magnitude squared of a vector."""
    return dot_product(v, v)


def vector_add(a: list[int], b: list[int]) -> list[int]:
    """Add two vectors element-wise."""
    n: int = len(a)
    result: list[int] = []
    i: int = 0
    while i < n:
        result.append(a[i] + b[i])
        i = i + 1
    return result


def vector_scale(v: list[int], s: int) -> list[int]:
    """Scale a vector by integer scalar."""
    result: list[int] = []
    i: int = 0
    n: int = len(v)
    while i < n:
        result.append(v[i] * s)
        i = i + 1
    return result


def vector_subtract(a: list[int], b: list[int]) -> list[int]:
    """Subtract vector b from a."""
    n: int = len(a)
    result: list[int] = []
    i: int = 0
    while i < n:
        result.append(a[i] - b[i])
        i = i + 1
    return result


def test_module() -> int:
    """Test vector operations."""
    ok: int = 0
    a: list[int] = [1, 2, 3]
    b: list[int] = [4, 5, 6]
    if dot_product(a, b) == 32:
        ok = ok + 1
    cp: list[int] = cross_product_3d(a, b)
    if cp[0] == 0 - 3:
        ok = ok + 1
    if cp[1] == 6:
        ok = ok + 1
    if magnitude_squared(a) == 14:
        ok = ok + 1
    s: list[int] = vector_add(a, b)
    if s[0] == 5:
        ok = ok + 1
    sc: list[int] = vector_scale(a, 3)
    if sc[2] == 9:
        ok = ok + 1
    return ok
