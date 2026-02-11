"""Matrix addition and subtraction using flat arrays."""


def matrix_add(a: list[int], b: list[int], rows: int, cols: int) -> list[int]:
    """Add two matrices stored as flat arrays."""
    result: list[int] = []
    idx: int = 0
    total: int = rows * cols
    while idx < total:
        result.append(a[idx] + b[idx])
        idx = idx + 1
    return result


def matrix_sub(a: list[int], b: list[int], rows: int, cols: int) -> list[int]:
    """Subtract matrix b from a, both stored as flat arrays."""
    result: list[int] = []
    idx: int = 0
    total: int = rows * cols
    while idx < total:
        result.append(a[idx] - b[idx])
        idx = idx + 1
    return result


def matrix_scalar_mul(a: list[int], scalar: int, size: int) -> list[int]:
    """Multiply every element by a scalar."""
    result: list[int] = []
    idx: int = 0
    while idx < size:
        result.append(a[idx] * scalar)
        idx = idx + 1
    return result


def matrix_negate(a: list[int], size: int) -> list[int]:
    """Negate every element of the matrix."""
    result: list[int] = []
    idx: int = 0
    while idx < size:
        result.append(-a[idx])
        idx = idx + 1
    return result


def test_module() -> int:
    passed: int = 0

    a: list[int] = [1, 2, 3, 4]
    b: list[int] = [5, 6, 7, 8]

    added: list[int] = matrix_add(a, b, 2, 2)
    if added[0] == 6:
        passed = passed + 1
    if added[3] == 12:
        passed = passed + 1

    subtracted: list[int] = matrix_sub(b, a, 2, 2)
    if subtracted[0] == 4:
        passed = passed + 1
    if subtracted[2] == 4:
        passed = passed + 1

    scaled: list[int] = matrix_scalar_mul(a, 3, 4)
    if scaled[1] == 6:
        passed = passed + 1
    if scaled[3] == 12:
        passed = passed + 1

    negated: list[int] = matrix_negate(a, 4)
    if negated[0] == -1:
        passed = passed + 1
    if negated[2] == -3:
        passed = passed + 1

    return passed
