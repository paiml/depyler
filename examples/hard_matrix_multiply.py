def matrix_get(m: list[int], rows: int, cols: int, r: int, c: int) -> int:
    return m[r * cols + c]


def matrix_set(m: list[int], cols: int, r: int, c: int, val: int) -> list[int]:
    m[r * cols + c] = val
    return m


def matrix_multiply(a: list[int], a_rows: int, a_cols: int, b: list[int], b_cols: int) -> list[int]:
    result: list[int] = []
    total: int = a_rows * b_cols
    i: int = 0
    while i < total:
        result.append(0)
        i = i + 1
    r: int = 0
    while r < a_rows:
        c: int = 0
        while c < b_cols:
            s: int = 0
            k: int = 0
            while k < a_cols:
                s = s + matrix_get(a, a_rows, a_cols, r, k) * matrix_get(b, a_cols, b_cols, k, c)
                k = k + 1
            result = matrix_set(result, b_cols, r, c, s)
            c = c + 1
        r = r + 1
    return result


def matrix_transpose(m: list[int], rows: int, cols: int) -> list[int]:
    result: list[int] = []
    total: int = rows * cols
    i: int = 0
    while i < total:
        result.append(0)
        i = i + 1
    r: int = 0
    while r < rows:
        c: int = 0
        while c < cols:
            result[c * rows + r] = m[r * cols + c]
            c = c + 1
        r = r + 1
    return result


def matrix_identity(n: int) -> list[int]:
    result: list[int] = []
    total: int = n * n
    i: int = 0
    while i < total:
        result.append(0)
        i = i + 1
    i = 0
    while i < n:
        result[i * n + i] = 1
        i = i + 1
    return result


def test_module() -> int:
    passed: int = 0
    a: list[int] = [1, 2, 3, 4]
    b: list[int] = [5, 6, 7, 8]
    r: list[int] = matrix_multiply(a, 2, 2, b, 2)
    if r == [19, 22, 43, 50]:
        passed = passed + 1
    t: list[int] = matrix_transpose([1, 2, 3, 4, 5, 6], 2, 3)
    if t == [1, 4, 2, 5, 3, 6]:
        passed = passed + 1
    ident: list[int] = matrix_identity(3)
    if ident == [1, 0, 0, 0, 1, 0, 0, 0, 1]:
        passed = passed + 1
    if matrix_get([1, 2, 3, 4], 2, 2, 1, 0) == 3:
        passed = passed + 1
    r2: list[int] = matrix_multiply([1, 0, 0, 1], 2, 2, [5, 6, 7, 8], 2)
    if r2 == [5, 6, 7, 8]:
        passed = passed + 1
    if matrix_identity(2) == [1, 0, 0, 1]:
        passed = passed + 1
    return passed
