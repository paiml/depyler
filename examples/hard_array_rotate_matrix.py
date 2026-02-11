def rotate_90(matrix: list[list[int]], n: int) -> list[list[int]]:
    result: list[list[int]] = []
    i: int = 0
    while i < n:
        row: list[int] = []
        j: int = 0
        while j < n:
            src_row: int = n - 1 - j
            src: list[int] = matrix[src_row]
            row.append(src[i])
            j = j + 1
        result.append(row)
        i = i + 1
    return result

def rotate_180(matrix: list[list[int]], n: int) -> list[list[int]]:
    result: list[list[int]] = []
    i: int = 0
    while i < n:
        row: list[int] = []
        j: int = 0
        while j < n:
            sr: int = n - 1 - i
            sc: int = n - 1 - j
            src: list[int] = matrix[sr]
            row.append(src[sc])
            j = j + 1
        result.append(row)
        i = i + 1
    return result

def rotate_270(matrix: list[list[int]], n: int) -> list[list[int]]:
    result: list[list[int]] = []
    i: int = 0
    while i < n:
        row: list[int] = []
        j: int = 0
        while j < n:
            src: list[int] = matrix[j]
            sc: int = n - 1 - i
            row.append(src[sc])
            j = j + 1
        result.append(row)
        i = i + 1
    return result

def matrix_equal(a: list[list[int]], b: list[list[int]], n: int) -> int:
    i: int = 0
    while i < n:
        j: int = 0
        while j < n:
            ra: list[int] = a[i]
            rb: list[int] = b[i]
            if ra[j] != rb[j]:
                return 0
            j = j + 1
        i = i + 1
    return 1

def test_module() -> int:
    passed: int = 0
    m: list[list[int]] = [[1, 2], [3, 4]]
    r90: list[list[int]] = rotate_90(m, 2)
    r90_0: list[int] = r90[0]
    if r90_0[0] == 3 and r90_0[1] == 1:
        passed = passed + 1
    r180: list[list[int]] = rotate_180(m, 2)
    r180_0: list[int] = r180[0]
    if r180_0[0] == 4 and r180_0[1] == 3:
        passed = passed + 1
    r270: list[list[int]] = rotate_270(m, 2)
    r270_0: list[int] = r270[0]
    if r270_0[0] == 2 and r270_0[1] == 4:
        passed = passed + 1
    double: list[list[int]] = rotate_90(rotate_90(m, 2), 2)
    if matrix_equal(double, r180, 2) == 1:
        passed = passed + 1
    quad: list[list[int]] = rotate_90(rotate_90(rotate_90(rotate_90(m, 2), 2), 2), 2)
    if matrix_equal(quad, m, 2) == 1:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
