"""Real-world matrix operations library.

Mimics: numpy matrix operations, scipy linear algebra basics.
Implements add, multiply, transpose, determinant, identity.
"""


def mat_create(rows: int, cols: int, fill: int) -> list[list[int]]:
    """Create a rows x cols matrix filled with given value."""
    result: list[list[int]] = []
    r: int = 0
    while r < rows:
        row: list[int] = []
        c: int = 0
        while c < cols:
            row.append(fill)
            c = c + 1
        result.append(row)
        r = r + 1
    return result


def mat_identity(n: int) -> list[list[int]]:
    """Create n x n identity matrix."""
    result: list[list[int]] = mat_create(n, n, 0)
    i: int = 0
    while i < n:
        result[i][i] = 1
        i = i + 1
    return result


def mat_add(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    """Add two matrices element-wise."""
    rows: int = len(a)
    cols: int = len(a[0])
    result: list[list[int]] = mat_create(rows, cols, 0)
    r: int = 0
    while r < rows:
        c: int = 0
        while c < cols:
            result[r][c] = a[r][c] + b[r][c]
            c = c + 1
        r = r + 1
    return result


def mat_scale(m: list[list[int]], scalar: int) -> list[list[int]]:
    """Multiply matrix by scalar."""
    rows: int = len(m)
    cols: int = len(m[0])
    result: list[list[int]] = mat_create(rows, cols, 0)
    r: int = 0
    while r < rows:
        c: int = 0
        while c < cols:
            result[r][c] = m[r][c] * scalar
            c = c + 1
        r = r + 1
    return result


def mat_transpose(m: list[list[int]]) -> list[list[int]]:
    """Transpose a matrix."""
    rows: int = len(m)
    cols: int = len(m[0])
    result: list[list[int]] = mat_create(cols, rows, 0)
    r: int = 0
    while r < rows:
        c: int = 0
        while c < cols:
            result[c][r] = m[r][c]
            c = c + 1
        r = r + 1
    return result


def mat_multiply(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    """Multiply two matrices (a_rows x a_cols) * (b_rows x b_cols)."""
    a_rows: int = len(a)
    a_cols: int = len(a[0])
    b_cols: int = len(b[0])
    result: list[list[int]] = mat_create(a_rows, b_cols, 0)
    r: int = 0
    while r < a_rows:
        c: int = 0
        while c < b_cols:
            total: int = 0
            k: int = 0
            while k < a_cols:
                total = total + a[r][k] * b[k][c]
                k = k + 1
            result[r][c] = total
            c = c + 1
        r = r + 1
    return result


def mat_trace(m: list[list[int]]) -> int:
    """Compute trace (sum of diagonal elements)."""
    n: int = len(m)
    total: int = 0
    i: int = 0
    while i < n:
        total = total + m[i][i]
        i = i + 1
    return total


def mat_det_2x2(m: list[list[int]]) -> int:
    """Compute determinant of 2x2 matrix."""
    return m[0][0] * m[1][1] - m[0][1] * m[1][0]


def mat_det_3x3(m: list[list[int]]) -> int:
    """Compute determinant of 3x3 matrix using cofactor expansion."""
    a: int = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
    b: int = m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
    c: int = m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    return a - b + c


def mat_is_symmetric(m: list[list[int]]) -> bool:
    """Check if matrix is symmetric (m[i][j] == m[j][i])."""
    n: int = len(m)
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            if m[i][j] != m[j][i]:
                return False
            j = j + 1
        i = i + 1
    return True


def test_module() -> int:
    """Test matrix operations module."""
    passed: int = 0

    # Test 1: create and identity
    eye: list[list[int]] = mat_identity(3)
    if eye[0][0] == 1 and eye[0][1] == 0 and eye[1][1] == 1:
        passed = passed + 1

    # Test 2: add matrices
    a: list[list[int]] = [[1, 2], [3, 4]]
    b: list[list[int]] = [[5, 6], [7, 8]]
    c: list[list[int]] = mat_add(a, b)
    if c[0][0] == 6 and c[1][1] == 12:
        passed = passed + 1

    # Test 3: scale
    scaled: list[list[int]] = mat_scale(a, 3)
    if scaled[0][0] == 3 and scaled[1][1] == 12:
        passed = passed + 1

    # Test 4: transpose
    rect: list[list[int]] = [[1, 2, 3], [4, 5, 6]]
    t: list[list[int]] = mat_transpose(rect)
    if len(t) == 3 and len(t[0]) == 2 and t[0][1] == 4:
        passed = passed + 1

    # Test 5: multiply
    m1: list[list[int]] = [[1, 2], [3, 4]]
    m2: list[list[int]] = [[5, 6], [7, 8]]
    prod: list[list[int]] = mat_multiply(m1, m2)
    if prod[0][0] == 19 and prod[1][1] == 50:
        passed = passed + 1

    # Test 6: trace
    if mat_trace(m1) == 5:
        passed = passed + 1

    # Test 7: determinant 2x2
    if mat_det_2x2(m1) == -2:
        passed = passed + 1

    # Test 8: symmetric check
    sym: list[list[int]] = [[1, 2, 3], [2, 5, 6], [3, 6, 9]]
    if mat_is_symmetric(sym):
        passed = passed + 1

    return passed
