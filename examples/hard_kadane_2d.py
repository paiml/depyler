"""Maximum subarray sum in 2D matrix using Kadane's extension."""


def kadane_max(arr: list[int]) -> int:
    """Standard 1D Kadane's algorithm for max subarray sum."""
    if len(arr) == 0:
        return 0
    max_sum: int = arr[0]
    cur_sum: int = arr[0]
    i: int = 1
    while i < len(arr):
        if cur_sum + arr[i] > arr[i]:
            cur_sum = cur_sum + arr[i]
        else:
            cur_sum = arr[i]
        if cur_sum > max_sum:
            max_sum = cur_sum
        i = i + 1
    return max_sum


def max_subarray_2d(mat: list[int], rows: int, cols: int) -> int:
    """Find maximum sum submatrix using Kadane's 2D extension."""
    if rows == 0 or cols == 0:
        return 0
    best: int = mat[0]
    left: int = 0
    while left < cols:
        temp: list[int] = []
        r: int = 0
        while r < rows:
            temp.append(0)
            r = r + 1
        right: int = left
        while right < cols:
            r2: int = 0
            while r2 < rows:
                temp[r2] = temp[r2] + mat[r2 * cols + right]
                r2 = r2 + 1
            cur: int = kadane_max(temp)
            if cur > best:
                best = cur
            right = right + 1
        left = left + 1
    return best


def row_sum(mat: list[int], cols: int, row: int) -> int:
    """Sum of a single row."""
    total: int = 0
    c: int = 0
    while c < cols:
        total = total + mat[row * cols + c]
        c = c + 1
    return total


def col_sum(mat: list[int], rows: int, cols: int, col: int) -> int:
    """Sum of a single column."""
    total: int = 0
    r: int = 0
    while r < rows:
        total = total + mat[r * cols + col]
        r = r + 1
    return total


def test_module() -> int:
    """Test 2D Kadane's."""
    ok: int = 0
    arr1: list[int] = [0 - 2, 1, 0 - 3, 4, 0 - 1, 2, 1, 0 - 5, 4]
    if kadane_max(arr1) == 6:
        ok = ok + 1
    mat1: list[int] = [1, 2, 0 - 1, 0 - 3, 0 - 1, 4, 1, 0 - 1, 2]
    if max_subarray_2d(mat1, 3, 3) == 6:
        ok = ok + 1
    mat2: list[int] = [1, 2, 3, 4]
    if max_subarray_2d(mat2, 2, 2) == 10:
        ok = ok + 1
    if row_sum(mat2, 2, 0) == 3:
        ok = ok + 1
    if col_sum(mat2, 2, 2, 0) == 4:
        ok = ok + 1
    neg: list[int] = [0 - 1, 0 - 2, 0 - 3, 0 - 4]
    v: int = max_subarray_2d(neg, 2, 2)
    if v == 0 - 1:
        ok = ok + 1
    empty_k: list[int] = []
    if kadane_max(empty_k) == 0:
        ok = ok + 1
    return ok
