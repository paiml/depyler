"""Trapping rainwater 1D problem.

Tests: water trapped, max water level, container volumes.
"""


def trap_rainwater(heights: list[int]) -> int:
    """Compute trapped rainwater given elevation map."""
    n: int = len(heights)
    if n < 3:
        return 0
    left_max: list[int] = []
    right_max: list[int] = []
    i: int = 0
    while i < n:
        left_max.append(0)
        right_max.append(0)
        i = i + 1
    left_max[0] = heights[0]
    j: int = 1
    while j < n:
        if heights[j] > left_max[j - 1]:
            left_max[j] = heights[j]
        else:
            left_max[j] = left_max[j - 1]
        j = j + 1
    right_max[n - 1] = heights[n - 1]
    k: int = n - 2
    while k >= 0:
        if heights[k] > right_max[k + 1]:
            right_max[k] = heights[k]
        else:
            right_max[k] = right_max[k + 1]
        k = k - 1
    water: int = 0
    m: int = 0
    while m < n:
        min_h: int = left_max[m]
        if right_max[m] < min_h:
            min_h = right_max[m]
        if min_h > heights[m]:
            water = water + min_h - heights[m]
        m = m + 1
    return water


def max_water_between_two(heights: list[int]) -> int:
    """Find max water between any two bars (container problem)."""
    n: int = len(heights)
    if n < 2:
        return 0
    max_water: int = 0
    left: int = 0
    right: int = n - 1
    while left < right:
        h: int = heights[left]
        if heights[right] < h:
            h = heights[right]
        area: int = h * (right - left)
        if area > max_water:
            max_water = area
        if heights[left] < heights[right]:
            left = left + 1
        else:
            right = right - 1
    return max_water


def total_elevation(heights: list[int]) -> int:
    """Sum all heights."""
    total: int = 0
    i: int = 0
    while i < len(heights):
        total = total + heights[i]
        i = i + 1
    return total


def test_module() -> int:
    """Test rainwater operations."""
    ok: int = 0
    h: list[int] = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]
    if trap_rainwater(h) == 6:
        ok = ok + 1
    h2: list[int] = [4, 2, 0, 3, 2, 5]
    if trap_rainwater(h2) == 9:
        ok = ok + 1
    if max_water_between_two([1, 8, 6, 2, 5, 4, 8, 3, 7]) == 49:
        ok = ok + 1
    if total_elevation([1, 2, 3]) == 6:
        ok = ok + 1
    return ok
