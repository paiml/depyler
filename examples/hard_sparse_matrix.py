# Sparse matrix as list of (row, col, val) triples stored in flat list


def sparse_create() -> list[int]:
    # Triples stored as [r0, c0, v0, r1, c1, v1, ...]
    result: list[int] = []
    return result


def sparse_set(matrix: list[int], row: int, col: int, val: int) -> list[int]:
    result: list[int] = []
    found: int = 0
    i: int = 0
    while i < len(matrix):
        r: int = matrix[i]
        c: int = matrix[i + 1]
        v: int = matrix[i + 2]
        if r == row and c == col:
            if val != 0:
                result.append(r)
                result.append(c)
                result.append(val)
            found = 1
        else:
            result.append(r)
            result.append(c)
            result.append(v)
        i = i + 3
    if found == 0 and val != 0:
        result.append(row)
        result.append(col)
        result.append(val)
    return result


def sparse_get(matrix: list[int], row: int, col: int) -> int:
    i: int = 0
    while i < len(matrix):
        if matrix[i] == row and matrix[i + 1] == col:
            return matrix[i + 2]
        i = i + 3
    return 0


def sparse_add(a: list[int], b: list[int]) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < len(a):
        result.append(a[i])
        result.append(a[i + 1])
        result.append(a[i + 2])
        i = i + 3
    j: int = 0
    while j < len(b):
        r: int = b[j]
        c: int = b[j + 1]
        v: int = b[j + 2]
        existing: int = sparse_get(result, r, c)
        result = sparse_set(result, r, c, existing + v)
        j = j + 3
    return result


def sparse_nnz(matrix: list[int]) -> int:
    return len(matrix) // 3


def sparse_transpose(matrix: list[int]) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < len(matrix):
        result.append(matrix[i + 1])
        result.append(matrix[i])
        result.append(matrix[i + 2])
        i = i + 3
    return result


def test_module() -> int:
    passed: int = 0

    # Test 1: empty matrix
    m: list[int] = sparse_create()
    if sparse_nnz(m) == 0:
        passed = passed + 1

    # Test 2: set and get
    m = sparse_set(m, 0, 0, 5)
    if sparse_get(m, 0, 0) == 5:
        passed = passed + 1

    # Test 3: get missing returns 0
    if sparse_get(m, 1, 1) == 0:
        passed = passed + 1

    # Test 4: overwrite
    m = sparse_set(m, 0, 0, 10)
    if sparse_get(m, 0, 0) == 10:
        passed = passed + 1

    # Test 5: nnz count
    m = sparse_set(m, 1, 2, 3)
    m = sparse_set(m, 2, 0, 7)
    if sparse_nnz(m) == 3:
        passed = passed + 1

    # Test 6: addition
    a: list[int] = sparse_set(sparse_create(), 0, 0, 1)
    b: list[int] = sparse_set(sparse_create(), 0, 0, 2)
    c: list[int] = sparse_add(a, b)
    if sparse_get(c, 0, 0) == 3:
        passed = passed + 1

    # Test 7: transpose
    t: list[int] = sparse_create()
    t = sparse_set(t, 1, 2, 9)
    tr: list[int] = sparse_transpose(t)
    if sparse_get(tr, 2, 1) == 9:
        passed = passed + 1

    return passed
