"""Tridiagonal matrix operations: verification, multiplication, and trace."""


def is_tridiagonal(matrix: list[int], size: int) -> int:
    """Check if a flat matrix is tridiagonal.
    Returns 1 if tridiagonal, 0 otherwise."""
    r: int = 0
    while r < size:
        c: int = 0
        while c < size:
            idx: int = r * size + c
            diff: int = r - c
            if diff < 0:
                diff = -diff
            if diff > 1 and matrix[idx] != 0:
                return 0
            c = c + 1
        r = r + 1
    return 1


def extract_band(matrix: list[int], size: int, bandwidth: int) -> list[int]:
    """Extract the band of a matrix as a flat array.
    For each row, extracts elements within bandwidth of the diagonal."""
    result: list[int] = []
    r: int = 0
    while r < size:
        c: int = 0
        while c < size:
            diff: int = r - c
            if diff < 0:
                diff = -diff
            if diff <= bandwidth:
                idx: int = r * size + c
                result.append(matrix[idx])
            c = c + 1
        r = r + 1
    return result


def tridiagonal_multiply(lower: list[int], diag: list[int], upper: list[int], vec: list[int]) -> list[int]:
    """Multiply tridiagonal matrix by vector.
    lower[i] is subdiagonal (index 0 unused), diag[i] is diagonal, upper[i] is superdiagonal."""
    n: int = len(diag)
    result: list[int] = []
    i: int = 0
    while i < n:
        val: int = diag[i] * vec[i]
        if i > 0:
            prev: int = i - 1
            val = val + lower[i] * vec[prev]
        if i < n - 1:
            next_i: int = i + 1
            val = val + upper[i] * vec[next_i]
        result.append(val)
        i = i + 1
    return result


def tridiagonal_trace(lower: list[int], diag: list[int], upper: list[int]) -> int:
    """Sum of diagonal elements of a tridiagonal matrix."""
    total: int = 0
    i: int = 0
    while i < len(diag):
        total = total + diag[i]
        i = i + 1
    return total


def test_module() -> int:
    """Test tridiagonal matrix operations."""
    ok: int = 0

    tri: list[int] = [2, 1, 0, 1, 3, 1, 0, 1, 4]
    if is_tridiagonal(tri, 3) == 1:
        ok = ok + 1

    non_tri: list[int] = [2, 1, 5, 1, 3, 1, 0, 1, 4]
    if is_tridiagonal(non_tri, 3) == 0:
        ok = ok + 1

    band: list[int] = extract_band(tri, 3, 1)
    if len(band) == 7:
        ok = ok + 1

    lower: list[int] = [0, 1, 1]
    diag: list[int] = [2, 3, 4]
    upper: list[int] = [1, 1, 0]
    vec: list[int] = [1, 2, 3]
    prod: list[int] = tridiagonal_multiply(lower, diag, upper, vec)
    if prod[0] == 4 and prod[1] == 10 and prod[2] == 14:
        ok = ok + 1

    if tridiagonal_trace(lower, diag, upper) == 9:
        ok = ok + 1

    identity: list[int] = [1, 0, 0, 0, 1, 0, 0, 0, 1]
    if is_tridiagonal(identity, 3) == 1:
        ok = ok + 1

    return ok
