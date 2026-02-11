"""Trapping rain water problem using histogram bars."""


def trap_water(heights: list[int]) -> int:
    """Compute trapped water using two-pointer technique."""
    length: int = len(heights)
    if length < 3:
        return 0
    left: int = 0
    right: int = length - 1
    left_max: int = heights[left]
    right_max: int = heights[right]
    water: int = 0
    while left < right:
        if left_max <= right_max:
            left = left + 1
            if heights[left] > left_max:
                left_max = heights[left]
            else:
                water = water + left_max - heights[left]
        else:
            right = right - 1
            if heights[right] > right_max:
                right_max = heights[right]
            else:
                water = water + right_max - heights[right]
    return water


def trap_water_dp(heights: list[int]) -> int:
    """Compute trapped water using DP approach for verification."""
    length: int = len(heights)
    if length < 3:
        return 0
    left_max_arr: list[int] = []
    right_max_arr: list[int] = []
    i: int = 0
    while i < length:
        left_max_arr.append(0)
        right_max_arr.append(0)
        i = i + 1
    left_max_arr[0] = heights[0]
    i = 1
    while i < length:
        prev: int = i - 1
        if heights[i] > left_max_arr[prev]:
            left_max_arr[i] = heights[i]
        else:
            left_max_arr[i] = left_max_arr[prev]
        i = i + 1
    last: int = length - 1
    right_max_arr[last] = heights[last]
    i = last - 1
    while i >= 0:
        next_i: int = i + 1
        if heights[i] > right_max_arr[next_i]:
            right_max_arr[i] = heights[i]
        else:
            right_max_arr[i] = right_max_arr[next_i]
        i = i - 1
    water: int = 0
    i = 0
    while i < length:
        min_height: int = left_max_arr[i]
        if right_max_arr[i] < min_height:
            min_height = right_max_arr[i]
        diff: int = min_height - heights[i]
        if diff > 0:
            water = water + diff
        i = i + 1
    return water


def max_bar_height(heights: list[int]) -> int:
    """Find the maximum bar height."""
    if len(heights) == 0:
        return 0
    max_h: int = heights[0]
    i: int = 1
    length: int = len(heights)
    while i < length:
        if heights[i] > max_h:
            max_h = heights[i]
        i = i + 1
    return max_h


def test_module() -> int:
    """Test trapping rain water operations."""
    passed: int = 0

    r1: int = trap_water([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1])
    if r1 == 6:
        passed = passed + 1

    r2: int = trap_water([4, 2, 0, 3, 2, 5])
    if r2 == 9:
        passed = passed + 1

    r3: int = trap_water([1, 2, 3])
    if r3 == 0:
        passed = passed + 1

    r4: int = trap_water([])
    if r4 == 0:
        passed = passed + 1

    r5: int = trap_water_dp([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1])
    if r5 == 6:
        passed = passed + 1

    if trap_water([4, 2, 0, 3, 2, 5]) == trap_water_dp([4, 2, 0, 3, 2, 5]):
        passed = passed + 1

    if max_bar_height([3, 1, 4, 1, 5]) == 5:
        passed = passed + 1

    if max_bar_height([]) == 0:
        passed = passed + 1

    return passed
