"""N-Queens backtracking counter."""


def is_safe(queens: list[int], row: int, col: int) -> int:
    """Check if placing queen at (row, col) is safe. Returns 1 if safe."""
    i: int = 0
    while i < row:
        qc: int = queens[i]
        if qc == col:
            return 0
        diff_row: int = row - i
        diff_col: int = col - qc
        if diff_col < 0:
            diff_col = 0 - diff_col
        if diff_row == diff_col:
            return 0
        i = i + 1
    return 1


def solve_nqueens(queens: list[int], row: int, n: int) -> int:
    """Count all valid N-Queens solutions recursively."""
    if row == n:
        return 1
    count: int = 0
    col: int = 0
    while col < n:
        if is_safe(queens, row, col) == 1:
            queens[row] = col
            found: int = solve_nqueens(queens, row + 1, n)
            count = count + found
            queens[row] = 0 - 1
        col = col + 1
    return count


def nqueens_count(n: int) -> int:
    """Count total solutions for n-queens problem."""
    queens: list[int] = []
    i: int = 0
    while i < n:
        queens.append(0 - 1)
        i = i + 1
    return solve_nqueens(queens, 0, n)


def first_solution(n: int) -> list[int]:
    """Find first valid solution. Returns queen column positions."""
    queens: list[int] = []
    i: int = 0
    while i < n:
        queens.append(0 - 1)
        i = i + 1
    find_first(queens, 0, n)
    return queens


def find_first(queens: list[int], row: int, n: int) -> int:
    """Find first solution. Returns 1 if found."""
    if row == n:
        return 1
    col: int = 0
    while col < n:
        if is_safe(queens, row, col) == 1:
            queens[row] = col
            result: int = find_first(queens, row + 1, n)
            if result == 1:
                return 1
            queens[row] = 0 - 1
        col = col + 1
    return 0


def test_module() -> int:
    """Test N-Queens."""
    passed: int = 0

    if nqueens_count(1) == 1:
        passed = passed + 1

    if nqueens_count(4) == 2:
        passed = passed + 1

    if nqueens_count(5) == 10:
        passed = passed + 1

    sol4: list[int] = first_solution(4)
    valid: int = 1
    i: int = 0
    while i < 4:
        if is_safe(sol4, i, sol4[i]) == 0:
            if i > 0:
                valid = 0
        i = i + 1
    if valid == 1:
        passed = passed + 1

    if nqueens_count(2) == 0:
        passed = passed + 1

    if nqueens_count(3) == 0:
        passed = passed + 1

    return passed
