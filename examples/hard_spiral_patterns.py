"""Spiral matrix traversal patterns.

Tests: 2D list access, boundary tracking, direction changes.
"""


def spiral_order(matrix: list[list[int]]) -> list[int]:
    """Extract elements in spiral order from a 2D matrix."""
    result: list[int] = []
    if len(matrix) == 0:
        return result
    top: int = 0
    bottom: int = len(matrix) - 1
    left: int = 0
    right: int = len(matrix[0]) - 1
    while top <= bottom and left <= right:
        col: int = left
        while col <= right:
            result.append(matrix[top][col])
            col += 1
        top += 1
        row: int = top
        while row <= bottom:
            result.append(matrix[row][right])
            row += 1
        right -= 1
        if top <= bottom:
            col = right
            while col >= left:
                result.append(matrix[bottom][col])
                col -= 1
            bottom -= 1
        if left <= right:
            row = bottom
            while row >= top:
                result.append(matrix[row][left])
                row -= 1
            left += 1
    return result


def generate_spiral(n: int) -> list[list[int]]:
    """Generate n x n matrix filled in spiral order 1..n*n."""
    matrix: list[list[int]] = []
    i: int = 0
    while i < n:
        row: list[int] = []
        j: int = 0
        while j < n:
            row.append(0)
            j += 1
        matrix.append(row)
        i += 1
    top: int = 0
    bottom: int = n - 1
    left: int = 0
    right: int = n - 1
    num: int = 1
    while top <= bottom and left <= right:
        col: int = left
        while col <= right:
            matrix[top][col] = num
            num += 1
            col += 1
        top += 1
        row2: int = top
        while row2 <= bottom:
            matrix[row2][right] = num
            num += 1
            row2 += 1
        right -= 1
        if top <= bottom:
            col = right
            while col >= left:
                matrix[bottom][col] = num
                num += 1
                col -= 1
            bottom -= 1
        if left <= right:
            row2 = bottom
            while row2 >= top:
                matrix[row2][left] = num
                num += 1
                row2 -= 1
            left += 1
    return matrix


def flatten_matrix(matrix: list[list[int]]) -> list[int]:
    """Flatten 2D matrix to 1D list."""
    result: list[int] = []
    for row in matrix:
        for val in row:
            result.append(val)
    return result


def test_module() -> int:
    """Test spiral operations."""
    ok: int = 0

    m: list[list[int]] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    s: list[int] = spiral_order(m)
    if s == [1, 2, 3, 6, 9, 8, 7, 4, 5]:
        ok += 1

    g: list[list[int]] = generate_spiral(2)
    f: list[int] = flatten_matrix(g)
    if f == [1, 2, 4, 3]:
        ok += 1

    g3: list[list[int]] = generate_spiral(1)
    f3: list[int] = flatten_matrix(g3)
    if f3 == [1]:
        ok += 1

    empty: list[int] = spiral_order([])
    if len(empty) == 0:
        ok += 1

    return ok
