"""Triple-nested loops with break and continue patterns."""


def find_triplet_sum(arr: list[int], target: int) -> list[int]:
    """Find first triplet that sums to target. Returns indices or empty."""
    n: int = len(arr)
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            k: int = j + 1
            while k < n:
                total: int = arr[i] + arr[j] + arr[k]
                if total == target:
                    return [i, j, k]
                k = k + 1
            j = j + 1
        i = i + 1
    return []


def count_triplets_less_than(arr: list[int], threshold: int) -> int:
    """Count triplets whose sum is less than threshold."""
    n: int = len(arr)
    count: int = 0
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            k: int = j + 1
            while k < n:
                total: int = arr[i] + arr[j] + arr[k]
                if total < threshold:
                    count = count + 1
                k = k + 1
            j = j + 1
        i = i + 1
    return count


def matrix_search(matrix: list[list[int]], target: int) -> list[int]:
    """Search for target in 2D list. Returns [row, col] or [-1, -1]."""
    rows: int = len(matrix)
    if rows == 0:
        return [-1, -1]
    i: int = 0
    while i < rows:
        row: list[int] = matrix[i]
        cols: int = len(row)
        j: int = 0
        while j < cols:
            if row[j] == target:
                return [i, j]
            j = j + 1
        i = i + 1
    return [-1, -1]


def nested_skip_pattern(n: int) -> int:
    """Count iterations with skip pattern in nested loops."""
    count: int = 0
    i: int = 0
    while i < n:
        j: int = 0
        while j < n:
            if (i + j) % 3 == 0:
                j = j + 1
                continue
            if i * j > n * n // 2:
                break
            count = count + 1
            j = j + 1
        i = i + 1
    return count


def find_pair_in_matrix(m: list[list[int]], target: int) -> int:
    """Find if any two elements in the matrix sum to target. Return 1/0."""
    rows: int = len(m)
    seen: dict[int, int] = {}
    i: int = 0
    while i < rows:
        row: list[int] = m[i]
        j: int = 0
        while j < len(row):
            complement: int = target - row[j]
            if complement in seen:
                return 1
            seen[row[j]] = 1
            j = j + 1
        i = i + 1
    return 0


def triple_nested_sum(n: int) -> int:
    """Sum i*j*k for all valid i,j,k with break conditions."""
    total: int = 0
    i: int = 1
    while i <= n:
        j: int = 1
        while j <= n:
            k: int = 1
            while k <= n:
                product: int = i * j * k
                if product > 100:
                    break
                total = total + product
                k = k + 1
            j = j + 1
        i = i + 1
    return total


def test_module() -> int:
    """Test all nested loop functions."""
    passed: int = 0
    r1: list[int] = find_triplet_sum([1, 2, 3, 4, 5], 9)
    if len(r1) == 3:
        passed = passed + 1
    r2: list[int] = find_triplet_sum([1, 2, 3], 100)
    if len(r2) == 0:
        passed = passed + 1
    ct: int = count_triplets_less_than([1, 2, 3, 4], 8)
    if ct == 2:
        passed = passed + 1
    mat: list[list[int]] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    r3: list[int] = matrix_search(mat, 5)
    if r3[0] == 1:
        passed = passed + 1
    if r3[1] == 1:
        passed = passed + 1
    r4: list[int] = matrix_search(mat, 99)
    if r4[0] == -1:
        passed = passed + 1
    ns: int = nested_skip_pattern(5)
    if ns > 0:
        passed = passed + 1
    fp: int = find_pair_in_matrix(mat, 10)
    if fp == 1:
        passed = passed + 1
    ts: int = triple_nested_sum(5)
    if ts > 0:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
