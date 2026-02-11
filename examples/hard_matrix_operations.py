# Matrix-like operations on 2D lists (transpose, multiply, etc.)
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def matrix_add(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    """Add two matrices element-wise."""
    result: list[list[int]] = []
    i: int = 0
    while i < len(a):
        row: list[int] = []
        j: int = 0
        while j < len(a[i]):
            row.append(a[i][j] + b[i][j])
            j = j + 1
        result.append(row)
        i = i + 1
    return result


def matrix_scale(m: list[list[int]], scalar: int) -> list[list[int]]:
    """Multiply every element in a matrix by a scalar."""
    result: list[list[int]] = []
    i: int = 0
    while i < len(m):
        row: list[int] = []
        j: int = 0
        while j < len(m[i]):
            row.append(m[i][j] * scalar)
            j = j + 1
        result.append(row)
        i = i + 1
    return result


def matrix_row_sum(m: list[list[int]]) -> list[int]:
    """Compute the sum of each row in a matrix."""
    result: list[int] = []
    i: int = 0
    while i < len(m):
        total: int = 0
        j: int = 0
        while j < len(m[i]):
            total = total + m[i][j]
            j = j + 1
        result.append(total)
        i = i + 1
    return result


def matrix_flatten(m: list[list[int]]) -> list[int]:
    """Flatten a 2D matrix into a 1D list."""
    result: list[int] = []
    i: int = 0
    while i < len(m):
        j: int = 0
        while j < len(m[i]):
            result.append(m[i][j])
            j = j + 1
        i = i + 1
    return result


def matrix_diagonal(m: list[list[int]]) -> list[int]:
    """Extract the main diagonal of a square matrix."""
    result: list[int] = []
    i: int = 0
    while i < len(m):
        if i < len(m[i]):
            result.append(m[i][i])
        i = i + 1
    return result


def test_module() -> int:
    a: list[list[int]] = [[1, 2], [3, 4]]
    b: list[list[int]] = [[5, 6], [7, 8]]
    assert matrix_add(a, b) == [[6, 8], [10, 12]]
    assert matrix_scale(a, 3) == [[3, 6], [9, 12]]
    assert matrix_row_sum(a) == [3, 7]
    assert matrix_flatten(a) == [1, 2, 3, 4]
    assert matrix_diagonal(a) == [1, 4]
    assert matrix_diagonal([[10, 20, 30], [40, 50, 60], [70, 80, 90]]) == [10, 50, 90]
    return 0


if __name__ == "__main__":
    test_module()
