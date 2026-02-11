"""Spiral traversal of a flat n x n matrix."""


def spiral_order(mat: list[int], n: int) -> list[int]:
    """Return elements of n x n flat matrix in spiral order."""
    result: list[int] = []
    top: int = 0
    bottom: int = n - 1
    left: int = 0
    right: int = n - 1

    while top <= bottom and left <= right:
        col: int = left
        while col <= right:
            result.append(mat[top * n + col])
            col = col + 1
        top = top + 1

        row: int = top
        while row <= bottom:
            result.append(mat[row * n + right])
            row = row + 1
        right = right - 1

        if top <= bottom:
            col2: int = right
            while col2 >= left:
                result.append(mat[bottom * n + col2])
                col2 = col2 - 1
            bottom = bottom - 1

        if left <= right:
            row2: int = bottom
            while row2 >= top:
                result.append(mat[row2 * n + left])
                row2 = row2 - 1
            left = left + 1

    return result


def spiral_sum(mat: list[int], n: int) -> int:
    """Sum all elements in spiral order (same as total sum)."""
    total: int = 0
    idx: int = 0
    size: int = n * n
    while idx < size:
        total = total + mat[idx]
        idx = idx + 1
    return total


def spiral_layer_sum(mat: list[int], n: int, layer: int) -> int:
    """Sum elements on a given spiral layer (0 = outermost)."""
    total: int = 0
    top: int = layer
    bottom: int = n - 1 - layer
    left: int = layer
    right: int = n - 1 - layer

    if top > bottom or left > right:
        return 0

    col: int = left
    while col <= right:
        total = total + mat[top * n + col]
        col = col + 1

    row: int = top + 1
    while row <= bottom:
        total = total + mat[row * n + right]
        row = row + 1

    if top < bottom:
        col2: int = right - 1
        while col2 >= left:
            total = total + mat[bottom * n + col2]
            col2 = col2 - 1

    if left < right:
        row2: int = bottom - 1
        while row2 > top:
            total = total + mat[row2 * n + left]
            row2 = row2 - 1

    return total


def test_module() -> int:
    passed: int = 0

    mat3: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9]
    sp: list[int] = spiral_order(mat3, 3)
    if sp[0] == 1:
        passed = passed + 1
    if sp[1] == 2:
        passed = passed + 1
    if sp[4] == 6:
        passed = passed + 1
    if len(sp) == 9:
        passed = passed + 1

    if spiral_sum(mat3, 3) == 45:
        passed = passed + 1

    if spiral_layer_sum(mat3, 3, 0) == 40:
        passed = passed + 1

    mat2: list[int] = [1, 2, 3, 4]
    sp2: list[int] = spiral_order(mat2, 2)
    if sp2[0] == 1:
        passed = passed + 1

    return passed
