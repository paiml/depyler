"""Monotonic stack algorithms.

Tests: increasing/decreasing stacks, area computation, trapping water.
"""


def largest_rectangle_histogram(heights: list[int]) -> int:
    """Find largest rectangle area in histogram."""
    n: int = len(heights)
    stack: list[int] = []
    max_area: int = 0
    i: int = 0
    while i <= n:
        h: int = 0
        if i < n:
            h = heights[i]
        while len(stack) > 0 and h < heights[stack[len(stack) - 1]]:
            top_idx: int = stack.pop()
            width: int = i
            if len(stack) > 0:
                width = i - stack[len(stack) - 1] - 1
            area: int = heights[top_idx] * width
            if area > max_area:
                max_area = area
        stack.append(i)
        i += 1
    return max_area


def trap_water(heights: list[int]) -> int:
    """Compute trapped water using two-pointer approach."""
    n: int = len(heights)
    if n < 3:
        return 0
    left: int = 0
    right: int = n - 1
    left_max: int = heights[left]
    right_max: int = heights[right]
    water: int = 0
    while left < right:
        if heights[left] < heights[right]:
            left += 1
            if heights[left] > left_max:
                left_max = heights[left]
            else:
                water += left_max - heights[left]
        else:
            right -= 1
            if heights[right] > right_max:
                right_max = heights[right]
            else:
                water += right_max - heights[right]
    return water


def daily_temperatures(temps: list[int]) -> list[int]:
    """For each day, find how many days until warmer temperature."""
    n: int = len(temps)
    result: list[int] = []
    i: int = 0
    while i < n:
        result.append(0)
        i += 1
    stack: list[int] = []
    i = 0
    while i < n:
        while len(stack) > 0 and temps[stack[len(stack) - 1]] < temps[i]:
            idx: int = stack.pop()
            result[idx] = i - idx
        stack.append(i)
        i += 1
    return result


def sum_of_subarray_minimums(arr: list[int]) -> int:
    """Sum of minimums of all subarrays."""
    n: int = len(arr)
    total: int = 0
    i: int = 0
    while i < n:
        j: int = i
        while j < n:
            min_val: int = arr[i]
            k: int = i
            while k <= j:
                if arr[k] < min_val:
                    min_val = arr[k]
                k += 1
            total += min_val
            j += 1
        i += 1
    return total


def test_module() -> int:
    """Test monotonic stack algorithms."""
    ok: int = 0

    area: int = largest_rectangle_histogram([2, 1, 5, 6, 2, 3])
    if area == 10:
        ok += 1

    w: int = trap_water([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1])
    if w == 6:
        ok += 1

    dt: list[int] = daily_temperatures([73, 74, 75, 71, 69, 72, 76, 73])
    if dt == [1, 1, 4, 2, 1, 1, 0, 0]:
        ok += 1

    s: int = sum_of_subarray_minimums([3, 1, 2])
    # subarrays: [3]=3, [1]=1, [2]=2, [3,1]=1, [1,2]=1, [3,1,2]=1 => 9
    if s == 9:
        ok += 1

    return ok
