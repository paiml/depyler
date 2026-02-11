"""Flatten 2D array to 1D, reshape 1D to 2D."""


def flatten_2d(matrix: list[list[int]]) -> list[int]:
    """Flatten a 2D array into a 1D array."""
    result: list[int] = []
    i: int = 0
    while i < len(matrix):
        j: int = 0
        while j < len(matrix[i]):
            result.append(matrix[i][j])
            j = j + 1
        i = i + 1
    return result


def reshape_1d_to_2d(arr: list[int], rows: int, cols: int) -> list[list[int]]:
    """Reshape a 1D array into a 2D array with given rows and cols."""
    result: list[list[int]] = []
    if rows * cols != len(arr):
        return result
    i: int = 0
    while i < rows:
        row: list[int] = []
        j: int = 0
        while j < cols:
            row.append(arr[i * cols + j])
            j = j + 1
        result.append(row)
        i = i + 1
    return result


def transpose(matrix: list[list[int]]) -> list[list[int]]:
    """Transpose a 2D matrix."""
    if len(matrix) == 0:
        return []
    rows: int = len(matrix)
    cols: int = len(matrix[0])
    result: list[list[int]] = []
    i: int = 0
    while i < cols:
        row: list[int] = []
        j: int = 0
        while j < rows:
            row.append(matrix[j][i])
            j = j + 1
        result.append(row)
        i = i + 1
    return result


def sum_2d(matrix: list[list[int]]) -> int:
    """Sum all elements in a 2D matrix."""
    total: int = 0
    i: int = 0
    while i < len(matrix):
        j: int = 0
        while j < len(matrix[i]):
            total = total + matrix[i][j]
            j = j + 1
        i = i + 1
    return total


def test_module() -> int:
    passed: int = 0

    f1: list[int] = flatten_2d([[1, 2], [3, 4], [5, 6]])
    if f1 == [1, 2, 3, 4, 5, 6]:
        passed = passed + 1

    f2: list[int] = flatten_2d([])
    if f2 == []:
        passed = passed + 1

    r1: list[list[int]] = reshape_1d_to_2d([1, 2, 3, 4, 5, 6], 2, 3)
    if r1 == [[1, 2, 3], [4, 5, 6]]:
        passed = passed + 1

    r2: list[list[int]] = reshape_1d_to_2d([1, 2, 3], 2, 2)
    if r2 == []:
        passed = passed + 1

    t1: list[list[int]] = transpose([[1, 2, 3], [4, 5, 6]])
    if t1 == [[1, 4], [2, 5], [3, 6]]:
        passed = passed + 1

    s1: int = sum_2d([[1, 2], [3, 4]])
    if s1 == 10:
        passed = passed + 1

    r3: list[list[int]] = reshape_1d_to_2d([10, 20, 30, 40], 2, 2)
    if r3 == [[10, 20], [30, 40]]:
        passed = passed + 1

    return passed
