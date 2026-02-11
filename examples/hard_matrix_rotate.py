"""Matrix rotation: 90, 180, 270 degree rotation of flat n x n matrix."""


def rotate_90_cw(mat: list[int], n: int) -> list[int]:
    """Rotate n x n flat matrix 90 degrees clockwise."""
    result: list[int] = []
    total: int = n * n
    idx: int = 0
    while idx < total:
        result.append(0)
        idx = idx + 1
    row: int = 0
    while row < n:
        col: int = 0
        while col < n:
            src: int = row * n + col
            dst: int = col * n + (n - 1 - row)
            result[dst] = mat[src]
            col = col + 1
        row = row + 1
    return result


def rotate_180(mat: list[int], n: int) -> list[int]:
    """Rotate n x n flat matrix 180 degrees."""
    result: list[int] = []
    total: int = n * n
    idx: int = 0
    while idx < total:
        result.append(0)
        idx = idx + 1
    row: int = 0
    while row < n:
        col: int = 0
        while col < n:
            src: int = row * n + col
            dst: int = (n - 1 - row) * n + (n - 1 - col)
            result[dst] = mat[src]
            col = col + 1
        row = row + 1
    return result


def rotate_270_cw(mat: list[int], n: int) -> list[int]:
    """Rotate n x n flat matrix 270 degrees clockwise (= 90 CCW)."""
    result: list[int] = []
    total: int = n * n
    idx: int = 0
    while idx < total:
        result.append(0)
        idx = idx + 1
    row: int = 0
    while row < n:
        col: int = 0
        while col < n:
            src: int = row * n + col
            dst: int = (n - 1 - col) * n + row
            result[dst] = mat[src]
            col = col + 1
        row = row + 1
    return result


def test_module() -> int:
    passed: int = 0

    mat: list[int] = [1, 2, 3, 4]

    r90: list[int] = rotate_90_cw(mat, 2)
    if r90[0] == 3:
        passed = passed + 1
    if r90[1] == 1:
        passed = passed + 1

    r180: list[int] = rotate_180(mat, 2)
    if r180[0] == 4:
        passed = passed + 1
    if r180[3] == 1:
        passed = passed + 1

    r270: list[int] = rotate_270_cw(mat, 2)
    if r270[0] == 2:
        passed = passed + 1
    if r270[1] == 4:
        passed = passed + 1

    big: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9]
    r90b: list[int] = rotate_90_cw(big, 3)
    if r90b[0] == 7:
        passed = passed + 1
    if r90b[2] == 1:
        passed = passed + 1

    return passed
