"""Fill matrix in spiral order.

Tests: spiral fill, spiral sum, corner values.
"""


def spiral_fill(n: int) -> list[int]:
    """Fill an n x n matrix in spiral order, stored as flat array."""
    total: int = n * n
    mat: list[int] = []
    i: int = 0
    while i < total:
        mat.append(0)
        i = i + 1
    top: int = 0
    bottom: int = n - 1
    left: int = 0
    right: int = n - 1
    num: int = 1
    while num <= total:
        c: int = left
        while c <= right:
            mat[top * n + c] = num
            num = num + 1
            c = c + 1
        top = top + 1
        r: int = top
        while r <= bottom:
            mat[r * n + right] = num
            num = num + 1
            r = r + 1
        right = right - 1
        c2: int = right
        while c2 >= left:
            mat[bottom * n + c2] = num
            num = num + 1
            c2 = c2 - 1
        bottom = bottom - 1
        r2: int = bottom
        while r2 >= top:
            mat[r2 * n + left] = num
            num = num + 1
            r2 = r2 - 1
        left = left + 1
    return mat


def spiral_sum(mat: list[int]) -> int:
    """Sum all elements in the spiral-filled matrix."""
    total: int = 0
    i: int = 0
    while i < len(mat):
        total = total + mat[i]
        i = i + 1
    return total


def spiral_corners(mat: list[int], n: int) -> list[int]:
    """Get the four corner values of n x n matrix."""
    corners: list[int] = []
    corners.append(mat[0])
    corners.append(mat[n - 1])
    corners.append(mat[(n - 1) * n])
    corners.append(mat[n * n - 1])
    return corners


def test_module() -> int:
    """Test spiral fill operations."""
    ok: int = 0
    mat: list[int] = spiral_fill(3)
    if mat[0] == 1:
        ok = ok + 1
    if mat[1] == 2:
        ok = ok + 1
    if mat[4] == 9:
        ok = ok + 1
    if spiral_sum(mat) == 45:
        ok = ok + 1
    corners: list[int] = spiral_corners(mat, 3)
    if corners[0] == 1:
        ok = ok + 1
    return ok
