"""Maximum sum submatrix using Kadane's algorithm on 2D arrays.

Tests: all positive, all negative, mixed, single element, single row/column.
"""


def kadane_max(arr: list[int]) -> int:
    """Return maximum subarray sum using Kadane's algorithm."""
    n: int = len(arr)
    if n == 0:
        return 0
    best: int = arr[0]
    current: int = arr[0]
    i: int = 1
    while i < n:
        if current + arr[i] > arr[i]:
            current = current + arr[i]
        else:
            current = arr[i]
        if current > best:
            best = current
        i = i + 1
    return best


def max_sum_submatrix(matrix: list[list[int]]) -> int:
    """Return maximum sum of any submatrix."""
    rows: int = len(matrix)
    if rows == 0:
        return 0
    cols: int = len(matrix[0])
    if cols == 0:
        return 0
    best: int = matrix[0][0]
    left: int = 0
    while left < cols:
        temp: list[int] = []
        r: int = 0
        while r < rows:
            temp.append(0)
            r = r + 1
        right: int = left
        while right < cols:
            r = 0
            while r < rows:
                temp[r] = temp[r] + matrix[r][right]
                r = r + 1
            sub_max: int = kadane_max(temp)
            if sub_max > best:
                best = sub_max
            right = right + 1
        left = left + 1
    return best


def max_sum_rectangle(matrix: list[list[int]], max_rows: int, max_cols: int) -> int:
    """Return maximum sum of submatrix with at most max_rows rows and max_cols cols."""
    rows: int = len(matrix)
    if rows == 0:
        return 0
    cols: int = len(matrix[0])
    if cols == 0:
        return 0
    prefix: list[list[int]] = []
    i: int = 0
    while i <= rows:
        row: list[int] = []
        j: int = 0
        while j <= cols:
            row.append(0)
            j = j + 1
        prefix.append(row)
        i = i + 1
    i = 1
    while i <= rows:
        j: int = 1
        while j <= cols:
            prefix[i][j] = matrix[i - 1][j - 1] + prefix[i - 1][j] + prefix[i][j - 1] - prefix[i - 1][j - 1]
            j = j + 1
        i = i + 1
    best: int = matrix[0][0]
    r1: int = 1
    while r1 <= rows:
        c1: int = 1
        while c1 <= cols:
            r2: int = r1
            while r2 <= rows and r2 - r1 + 1 <= max_rows:
                c2: int = c1
                while c2 <= cols and c2 - c1 + 1 <= max_cols:
                    val: int = prefix[r2][c2] - prefix[r1 - 1][c2] - prefix[r2][c1 - 1] + prefix[r1 - 1][c1 - 1]
                    if val > best:
                        best = val
                    c2 = c2 + 1
                r2 = r2 + 1
            c1 = c1 + 1
        r1 = r1 + 1
    return best


def test_module() -> int:
    """Test max sum submatrix."""
    ok: int = 0

    m1: list[list[int]] = [[1, 2, -1], [-3, 4, 2], [1, -1, 3]]
    if max_sum_submatrix(m1) == 9:
        ok = ok + 1

    m2: list[list[int]] = [[-1, -2], [-3, -4]]
    if max_sum_submatrix(m2) == -1:
        ok = ok + 1

    m3: list[list[int]] = [[5]]
    if max_sum_submatrix(m3) == 5:
        ok = ok + 1

    m4: list[list[int]] = [[1, 2, 3]]
    if max_sum_submatrix(m4) == 6:
        ok = ok + 1

    arr1: list[int] = [-2, 1, -3, 4, -1, 2, 1, -5, 4]
    if kadane_max(arr1) == 6:
        ok = ok + 1

    arr2: list[int] = [1, 2, 3, 4]
    if kadane_max(arr2) == 10:
        ok = ok + 1

    m5: list[list[int]] = [[2, 1], [3, 4]]
    if max_sum_submatrix(m5) == 10:
        ok = ok + 1

    if max_sum_rectangle(m5, 1, 1) == 4:
        ok = ok + 1

    m6: list[list[int]] = [[-1, 2], [3, -4]]
    if max_sum_submatrix(m6) == 3:
        ok = ok + 1

    arr3: list[int] = [-1]
    if kadane_max(arr3) == -1:
        ok = ok + 1

    return ok
