def pascal_row(n: int) -> list[int]:
    row: list[int] = [1]
    i: int = 1
    while i <= n:
        prev: int = row[i - 1]
        val: int = prev * (n - i + 1) // i
        row.append(val)
        i = i + 1
    return row

def pascal_triangle(n: int) -> list[list[int]]:
    result: list[list[int]] = []
    i: int = 0
    while i < n:
        pr: list[int] = pascal_row(i)
        result.append(pr)
        i = i + 1
    return result

def pascal_element(row: int, col: int) -> int:
    if col < 0 or col > row:
        return 0
    if col == 0 or col == row:
        return 1
    r: list[int] = pascal_row(row)
    return r[col]

def pascal_row_sum(n: int) -> int:
    return 1 << n

def pascal_diagonal(n: int, diag: int) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < n:
        row: int = diag + i
        pe: int = pascal_element(row, diag)
        result.append(pe)
        i = i + 1
    return result

def binomial_coefficient(n: int, r: int) -> int:
    if r > n:
        return 0
    if r == 0 or r == n:
        return 1
    effective_r: int = r
    if r > n - r:
        effective_r = n - r
    result: int = 1
    i: int = 0
    while i < effective_r:
        result = result * (n - i)
        result = result // (i + 1)
        i = i + 1
    return result

def test_module() -> int:
    passed: int = 0
    r1: list[int] = pascal_row(4)
    if r1 == [1, 4, 6, 4, 1]:
        passed = passed + 1
    t: list[list[int]] = pascal_triangle(4)
    nt: int = len(t)
    if nt == 4:
        passed = passed + 1
    r3: int = pascal_element(5, 2)
    if r3 == 10:
        passed = passed + 1
    r4: int = pascal_row_sum(4)
    if r4 == 16:
        passed = passed + 1
    r5: int = binomial_coefficient(10, 3)
    if r5 == 120:
        passed = passed + 1
    r6: list[int] = pascal_diagonal(4, 0)
    if r6 == [1, 1, 1, 1]:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
