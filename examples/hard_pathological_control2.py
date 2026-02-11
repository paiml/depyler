# Pathological control flow: While loops with multiple break/continue conditions
# Tests: complex loop termination, early exits, nested continue/break


def find_first_peak(nums: list[int]) -> int:
    """Find index of first local maximum. Returns -1 if none."""
    if len(nums) < 3:
        return 0 - 1
    i: int = 1
    while i < len(nums) - 1:
        if nums[i] <= nums[i - 1]:
            i = i + 1
            continue
        if nums[i] <= nums[i + 1]:
            i = i + 1
            continue
        return i
    return 0 - 1


def skip_and_collect(nums: list[int], skip_val: int, stop_val: int) -> list[int]:
    """Collect nums, skipping skip_val, stopping at stop_val."""
    result: list[int] = []
    i: int = 0
    while i < len(nums):
        val: int = nums[i]
        i = i + 1
        if val == stop_val:
            break
        if val == skip_val:
            continue
        result.append(val)
    return result


def count_ranges(nums: list[int]) -> int:
    """Count ascending ranges. A range ends when value decreases."""
    if len(nums) == 0:
        return 0
    ranges: int = 1
    i: int = 1
    while i < len(nums):
        if nums[i] < nums[i - 1]:
            ranges = ranges + 1
        i = i + 1
    return ranges


def nested_break_search(grid: list[int], rows: int, cols: int, target: int) -> int:
    """Search flat grid for target. Returns row*1000+col or -1."""
    r: int = 0
    found_row: int = 0 - 1
    found_col: int = 0 - 1
    while r < rows:
        c: int = 0
        while c < cols:
            idx: int = r * cols + c
            if grid[idx] == target:
                found_row = r
                found_col = c
                break
            c = c + 1
        if found_row >= 0:
            break
        r = r + 1
    if found_row >= 0:
        return found_row * 1000 + found_col
    return 0 - 1


def converge_sequence(start: int, limit: int) -> int:
    """Apply Collatz-like sequence until convergence or limit reached."""
    val: int = start
    steps: int = 0
    while steps < limit:
        if val == 1:
            break
        if val % 2 == 0:
            val = val // 2
        else:
            val = val * 3 + 1
        steps = steps + 1
    return steps


def test_module() -> int:
    passed: int = 0
    # Test 1: find peak
    if find_first_peak([1, 3, 2, 4, 1]) == 1:
        passed = passed + 1
    # Test 2: no peak
    if find_first_peak([1, 2, 3, 4]) == 0 - 1:
        passed = passed + 1
    # Test 3: skip and collect
    r: list[int] = skip_and_collect([1, 2, 3, 4, 5], 3, 5)
    if len(r) == 3:
        passed = passed + 1
    # Test 4: count ranges
    if count_ranges([1, 3, 5, 2, 4, 6, 1]) == 3:
        passed = passed + 1
    # Test 5: nested grid search (3x3 grid, find 7 at row 2 col 1)
    grid: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9]
    if nested_break_search(grid, 3, 3, 7) == 2000:
        passed = passed + 1
    # Test 6: collatz convergence
    if converge_sequence(6, 100) == 8:
        passed = passed + 1
    # Test 7: already at 1
    if converge_sequence(1, 100) == 0:
        passed = passed + 1
    return passed
