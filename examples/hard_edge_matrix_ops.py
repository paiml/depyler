"""2D list operations: transpose, multiply, rotate, determinant."""


def create_matrix(rows: int, cols: int, fill: int) -> list[list[int]]:
    """Create a rows x cols matrix filled with fill value."""
    mat: list[list[int]] = []
    i: int = 0
    while i < rows:
        row: list[int] = []
        j: int = 0
        while j < cols:
            row.append(fill)
            j = j + 1
        mat.append(row)
        i = i + 1
    return mat


def transpose(mat: list[list[int]]) -> list[list[int]]:
    """Transpose a matrix."""
    rows: int = len(mat)
    if rows == 0:
        return []
    first_row: list[int] = mat[0]
    cols: int = len(first_row)
    result: list[list[int]] = create_matrix(cols, rows, 0)
    i: int = 0
    while i < rows:
        row: list[int] = mat[i]
        j: int = 0
        while j < cols:
            r_row: list[int] = result[j]
            r_row[i] = row[j]
            j = j + 1
        i = i + 1
    return result


def mat_add(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    """Add two matrices."""
    rows: int = len(a)
    if rows == 0:
        return []
    first_row: list[int] = a[0]
    cols: int = len(first_row)
    result: list[list[int]] = create_matrix(rows, cols, 0)
    i: int = 0
    while i < rows:
        a_row: list[int] = a[i]
        b_row: list[int] = b[i]
        r_row: list[int] = result[i]
        j: int = 0
        while j < cols:
            r_row[j] = a_row[j] + b_row[j]
            j = j + 1
        i = i + 1
    return result


def mat_multiply(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    """Multiply two matrices."""
    rows_a: int = len(a)
    if rows_a == 0:
        return []
    first_a: list[int] = a[0]
    cols_a: int = len(first_a)
    first_b: list[int] = b[0]
    cols_b: int = len(first_b)
    result: list[list[int]] = create_matrix(rows_a, cols_b, 0)
    i: int = 0
    while i < rows_a:
        j: int = 0
        while j < cols_b:
            total: int = 0
            k: int = 0
            while k < cols_a:
                a_row: list[int] = a[i]
                b_row: list[int] = b[k]
                total = total + a_row[k] * b_row[j]
                k = k + 1
            r_row: list[int] = result[i]
            r_row[j] = total
            j = j + 1
        i = i + 1
    return result


def identity_matrix(n: int) -> list[list[int]]:
    """Create n x n identity matrix."""
    result: list[list[int]] = create_matrix(n, n, 0)
    i: int = 0
    while i < n:
        row: list[int] = result[i]
        row[i] = 1
        i = i + 1
    return result


def trace(mat: list[list[int]]) -> int:
    """Compute trace (sum of diagonal elements)."""
    total: int = 0
    n: int = len(mat)
    i: int = 0
    while i < n:
        row: list[int] = mat[i]
        total = total + row[i]
        i = i + 1
    return total


def flatten(mat: list[list[int]]) -> list[int]:
    """Flatten 2D matrix to 1D list."""
    result: list[int] = []
    i: int = 0
    while i < len(mat):
        row: list[int] = mat[i]
        j: int = 0
        while j < len(row):
            result.append(row[j])
            j = j + 1
        i = i + 1
    return result


def test_module() -> int:
    """Test all matrix operations."""
    passed: int = 0
    m1: list[list[int]] = [[1, 2], [3, 4]]
    t1: list[list[int]] = transpose(m1)
    t1_r0: list[int] = t1[0]
    t1_r1: list[int] = t1[1]
    if t1_r0[0] == 1 and t1_r0[1] == 3:
        passed = passed + 1
    if t1_r1[0] == 2 and t1_r1[1] == 4:
        passed = passed + 1
    m2: list[list[int]] = [[5, 6], [7, 8]]
    added: list[list[int]] = mat_add(m1, m2)
    a_r0: list[int] = added[0]
    if a_r0[0] == 6 and a_r0[1] == 8:
        passed = passed + 1
    prod: list[list[int]] = mat_multiply(m1, m2)
    p_r0: list[int] = prod[0]
    if p_r0[0] == 19 and p_r0[1] == 22:
        passed = passed + 1
    eye: list[list[int]] = identity_matrix(3)
    if trace(eye) == 3:
        passed = passed + 1
    tr: int = trace(m1)
    if tr == 5:
        passed = passed + 1
    flat: list[int] = flatten(m1)
    if flat == [1, 2, 3, 4]:
        passed = passed + 1
    empty_t: list[list[int]] = transpose([])
    if len(empty_t) == 0:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
