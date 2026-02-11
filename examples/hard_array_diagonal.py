def main_diagonal(matrix: list[list[int]], n: int) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < n:
        row: list[int] = matrix[i]
        result.append(row[i])
        i = i + 1
    return result

def anti_diagonal(matrix: list[list[int]], n: int) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < n:
        row: list[int] = matrix[i]
        col: int = n - 1 - i
        result.append(row[col])
        i = i + 1
    return result

def diagonal_sum(matrix: list[list[int]], n: int) -> int:
    total: int = 0
    i: int = 0
    while i < n:
        row: list[int] = matrix[i]
        total = total + row[i]
        col: int = n - 1 - i
        total = total + row[col]
        i = i + 1
    if n % 2 == 1:
        mid: int = n // 2
        midrow: list[int] = matrix[mid]
        total = total - midrow[mid]
    return total

def all_diagonals(matrix: list[list[int]], n: int) -> int:
    count: int = 2 * n - 1
    return count

def trace_matrix(matrix: list[list[int]], n: int) -> int:
    total: int = 0
    i: int = 0
    while i < n:
        row: list[int] = matrix[i]
        total = total + row[i]
        i = i + 1
    return total

def test_module() -> int:
    passed: int = 0
    m: list[list[int]] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    d: list[int] = main_diagonal(m, 3)
    if d[0] == 1 and d[1] == 5 and d[2] == 9:
        passed = passed + 1
    ad: list[int] = anti_diagonal(m, 3)
    if ad[0] == 3 and ad[1] == 5 and ad[2] == 7:
        passed = passed + 1
    if diagonal_sum(m, 3) == 25:
        passed = passed + 1
    if all_diagonals(m, 3) == 5:
        passed = passed + 1
    if trace_matrix(m, 3) == 15:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
