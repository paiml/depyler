"""Band matrix operations.

Implements operations on band matrices stored as flat arrays,
where only elements near the diagonal are non-zero.
"""


def create_band_matrix(size: int, bandwidth: int) -> list[int]:
    """Create a band matrix with 1s within bandwidth of diagonal.

    Stored as flat array of size*size elements.
    """
    total: int = size * size
    matrix: list[int] = []
    i: int = 0
    while i < total:
        matrix.append(0)
        i = i + 1

    row: int = 0
    while row < size:
        col: int = 0
        while col < size:
            diff: int = row - col
            if diff < 0:
                diff = -diff
            if diff <= bandwidth:
                idx: int = row * size + col
                matrix[idx] = 1
            col = col + 1
        row = row + 1
    return matrix


def band_matrix_trace(matrix: list[int], size: int) -> int:
    """Compute trace (sum of diagonal elements) of a band matrix."""
    total: int = 0
    i: int = 0
    while i < size:
        idx: int = i * size + i
        total = total + matrix[idx]
        i = i + 1
    return total


def band_matrix_nonzeros(matrix: list[int], size: int) -> int:
    """Count non-zero elements in the band matrix."""
    count: int = 0
    total: int = size * size
    i: int = 0
    while i < total:
        if matrix[i] != 0:
            count = count + 1
        i = i + 1
    return count


def band_matrix_row_sum(matrix: list[int], size: int, row: int) -> int:
    """Compute sum of elements in a specific row."""
    total: int = 0
    col: int = 0
    while col < size:
        idx: int = row * size + col
        total = total + matrix[idx]
        col = col + 1
    return total


def band_matrix_multiply_vector(matrix: list[int], size: int, vec: list[int]) -> list[int]:
    """Multiply band matrix by a vector."""
    result: list[int] = []
    row: int = 0
    while row < size:
        val: int = 0
        col: int = 0
        while col < size:
            idx: int = row * size + col
            val = val + matrix[idx] * vec[col]
            col = col + 1
        result.append(val)
        row = row + 1
    return result


def test_module() -> int:
    """Test band matrix operations."""
    ok: int = 0

    tmp_mat: list[int] = create_band_matrix(4, 1)
    trace: int = band_matrix_trace(tmp_mat, 4)
    if trace == 4:
        ok = ok + 1

    nz: int = band_matrix_nonzeros(tmp_mat, 4)
    if nz == 10:
        ok = ok + 1

    row_s: int = band_matrix_row_sum(tmp_mat, 4, 0)
    if row_s == 2:
        ok = ok + 1

    vec: list[int] = [1, 1, 1, 1]
    tmp_result: list[int] = band_matrix_multiply_vector(tmp_mat, 4, vec)
    if tmp_result[0] == 2 and tmp_result[1] == 3:
        ok = ok + 1

    return ok
