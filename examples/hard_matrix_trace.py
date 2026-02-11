"""Matrix trace, diagonal extraction, identity check on flat arrays."""


def matrix_trace(mat: list[int], n: int) -> int:
    """Compute the trace (sum of diagonal) of an n x n flat matrix."""
    total: int = 0
    idx: int = 0
    while idx < n:
        pos: int = idx * n + idx
        total = total + mat[pos]
        idx = idx + 1
    return total


def extract_diagonal(mat: list[int], n: int) -> list[int]:
    """Extract the main diagonal of an n x n flat matrix."""
    diag: list[int] = []
    idx: int = 0
    while idx < n:
        pos: int = idx * n + idx
        diag.append(mat[pos])
        idx = idx + 1
    return diag


def is_identity(mat: list[int], n: int) -> int:
    """Return 1 if mat is the n x n identity matrix, else 0."""
    row: int = 0
    while row < n:
        col: int = 0
        while col < n:
            pos: int = row * n + col
            if row == col:
                if mat[pos] != 1:
                    return 0
            else:
                if mat[pos] != 0:
                    return 0
            col = col + 1
        row = row + 1
    return 1


def make_identity(n: int) -> list[int]:
    """Create an n x n identity matrix as flat array."""
    result: list[int] = []
    row: int = 0
    while row < n:
        col: int = 0
        while col < n:
            if row == col:
                result.append(1)
            else:
                result.append(0)
            col = col + 1
        row = row + 1
    return result


def test_module() -> int:
    passed: int = 0

    mat: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9]
    if matrix_trace(mat, 3) == 15:
        passed = passed + 1

    diag: list[int] = extract_diagonal(mat, 3)
    if diag[0] == 1:
        passed = passed + 1
    if diag[2] == 9:
        passed = passed + 1

    identity: list[int] = make_identity(3)
    if is_identity(identity, 3) == 1:
        passed = passed + 1
    if is_identity(mat, 3) == 0:
        passed = passed + 1

    if matrix_trace(identity, 3) == 3:
        passed = passed + 1
    if len(identity) == 9:
        passed = passed + 1

    return passed
