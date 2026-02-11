"""Backtracking: permutations, combinations, subsets.

Tests: permutation count, combination count, subset generation, n-queens count.
"""


def count_permutations(n: int) -> int:
    """Count permutations of n elements = n!"""
    if n <= 1:
        return 1
    result: int = 1
    i: int = 2
    while i <= n:
        result = result * i
        i = i + 1
    return result


def generate_subsets_count(n: int) -> int:
    """Count all subsets of n elements = 2^n."""
    result: int = 1
    i: int = 0
    while i < n:
        result = result * 2
        i = i + 1
    return result


def combination_count(n: int, r: int) -> int:
    """Count C(n, r) using iterative computation."""
    if r > n:
        return 0
    if r == 0:
        return 1
    if r > n - r:
        r = n - r
    result: int = 1
    i: int = 0
    while i < r:
        result = result * (n - i)
        result = result // (i + 1)
        i = i + 1
    return result


def bt_permutations_sum(arr: list[int]) -> int:
    """Sum of first elements of all permutations (backtracking simulation)."""
    n: int = len(arr)
    if n == 0:
        return 0
    total: int = 0
    perm_of_rest: int = count_permutations(n - 1)
    i: int = 0
    while i < n:
        total = total + arr[i] * perm_of_rest
        i = i + 1
    return total


def bt_subset_sums(arr: list[int]) -> list[int]:
    """Generate all subset sums using bit enumeration."""
    n: int = len(arr)
    num_subsets: int = 1
    i: int = 0
    while i < n:
        num_subsets = num_subsets * 2
        i = i + 1
    result: list[int] = []
    mask: int = 0
    while mask < num_subsets:
        total: int = 0
        bit: int = 0
        while bit < n:
            if (mask >> bit) & 1 == 1:
                total = total + arr[bit]
            bit = bit + 1
        result.append(total)
        mask = mask + 1
    return result


def bt_n_queens_count(n: int) -> int:
    """Count solutions to N-Queens problem using backtracking."""
    cols: list[int] = []
    i: int = 0
    while i < n:
        cols.append(-1)
        i = i + 1
    count: list[int] = [0]
    bt_nq_solve(cols, 0, n, count)
    return count[0]


def bt_nq_solve(cols: list[int], row: int, n: int, count: list[int]) -> int:
    """Recursive N-Queens solver."""
    if row == n:
        count[0] = count[0] + 1
        return 1
    col: int = 0
    while col < n:
        if bt_nq_is_safe(cols, row, col) == 1:
            cols[row] = col
            bt_nq_solve(cols, row + 1, n, count)
            cols[row] = -1
        col = col + 1
    return 0


def bt_nq_is_safe(cols: list[int], row: int, col: int) -> int:
    """Check if placing queen at (row, col) is safe."""
    r: int = 0
    while r < row:
        c: int = cols[r]
        if c == col:
            return 0
        diff_r: int = row - r
        diff_c: int = col - c
        if diff_c < 0:
            diff_c = 0 - diff_c
        if diff_r == diff_c:
            return 0
        r = r + 1
    return 1


def test_module() -> int:
    """Test backtracking operations."""
    passed: int = 0

    if count_permutations(4) == 24:
        passed = passed + 1

    if generate_subsets_count(3) == 8:
        passed = passed + 1

    if combination_count(5, 2) == 10:
        passed = passed + 1

    if combination_count(10, 0) == 1:
        passed = passed + 1

    sums: list[int] = bt_subset_sums([1, 2, 3])
    if len(sums) == 8:
        passed = passed + 1

    if bt_n_queens_count(4) == 2:
        passed = passed + 1

    if bt_n_queens_count(1) == 1:
        passed = passed + 1

    ps: int = bt_permutations_sum([1, 2, 3])
    if ps == 12:
        passed = passed + 1

    return passed
