def pascal_row(n: int) -> list[int]:
    row: list[int] = [1]
    i: int = 1
    while i <= n:
        prev: int = row[i - 1]
        val: int = prev * (n - i + 1) // i
        row.append(val)
        i = i + 1
    return row

def pascal_element(n: int, k: int) -> int:
    if k > n:
        return 0
    if k == 0 or k == n:
        return 1
    result: int = 1
    i: int = 0
    while i < k:
        result = result * (n - i)
        result = result // (i + 1)
        i = i + 1
    return result

def pascal_row_sum(n: int) -> int:
    result: int = 1
    i: int = 0
    while i < n:
        result = result * 2
        i = i + 1
    return result

def pascal_is_symmetric(n: int) -> int:
    row: list[int] = pascal_row(n)
    sz: int = len(row)
    i: int = 0
    while i < sz // 2:
        j: int = sz - 1 - i
        if row[i] != row[j]:
            return 0
        i = i + 1
    return 1

def test_module() -> int:
    passed: int = 0
    r0: list[int] = pascal_row(0)
    if len(r0) == 1 and r0[0] == 1:
        passed = passed + 1
    r4: list[int] = pascal_row(4)
    if len(r4) == 5 and r4[2] == 6:
        passed = passed + 1
    if pascal_element(5, 2) == 10:
        passed = passed + 1
    if pascal_row_sum(4) == 16:
        passed = passed + 1
    if pascal_is_symmetric(5) == 1:
        passed = passed + 1
    if pascal_element(10, 0) == 1:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
