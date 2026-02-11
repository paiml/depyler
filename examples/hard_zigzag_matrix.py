"""Fill a matrix in zigzag (diagonal) order."""


def zigzag_fill(n: int) -> list[int]:
    """Fill an n x n matrix in zigzag diagonal order. Returns flat list."""
    mat: list[int] = []
    total: int = n * n
    i: int = 0
    while i < total:
        mat.append(0)
        i = i + 1
    val: int = 1
    diag: int = 0
    while diag < 2 * n - 1:
        if diag % 2 == 0:
            r: int = diag
            if r >= n:
                r = n - 1
            c: int = diag - r
            while r >= 0 and c < n:
                mat[r * n + c] = val
                val = val + 1
                r = r - 1
                c = c + 1
        else:
            c2: int = diag
            if c2 >= n:
                c2 = n - 1
            r2: int = diag - c2
            while c2 >= 0 and r2 < n:
                mat[r2 * n + c2] = val
                val = val + 1
                c2 = c2 - 1
                r2 = r2 + 1
        diag = diag + 1
    return mat


def spiral_fill(n: int) -> list[int]:
    """Fill an n x n matrix in spiral order. Returns flat list."""
    mat: list[int] = []
    total: int = n * n
    i: int = 0
    while i < total:
        mat.append(0)
        i = i + 1
    val: int = 1
    top: int = 0
    bottom: int = n - 1
    left: int = 0
    right: int = n - 1
    while val <= total:
        c: int = left
        while c <= right and val <= total:
            mat[top * n + c] = val
            val = val + 1
            c = c + 1
        top = top + 1
        r: int = top
        while r <= bottom and val <= total:
            mat[r * n + right] = val
            val = val + 1
            r = r + 1
        right = right - 1
        c = right
        while c >= left and val <= total:
            mat[bottom * n + c] = val
            val = val + 1
            c = c - 1
        bottom = bottom - 1
        r = bottom
        while r >= top and val <= total:
            mat[r * n + left] = val
            val = val + 1
            r = r - 1
        left = left + 1
    return mat


def test_module() -> int:
    """Test zigzag matrix fill."""
    passed: int = 0

    z3: list[int] = zigzag_fill(3)
    if z3[0] == 1 and z3[1] == 2 and z3[3] == 3:
        passed = passed + 1

    if z3[8] == 9:
        passed = passed + 1

    z1: list[int] = zigzag_fill(1)
    if z1[0] == 1:
        passed = passed + 1

    s3: list[int] = spiral_fill(3)
    if s3[0] == 1 and s3[1] == 2 and s3[2] == 3:
        passed = passed + 1

    if s3[4] == 9:
        passed = passed + 1

    z4: list[int] = zigzag_fill(4)
    if len(z4) == 16:
        passed = passed + 1

    return passed
