"""Histogram largest rectangle area computation.

Tests: largest rectangle, max consecutive, bar sums.
"""


def largest_rectangle_area(heights: list[int]) -> int:
    """Find largest rectangle area in histogram using brute force."""
    n: int = len(heights)
    max_area: int = 0
    i: int = 0
    while i < n:
        min_h: int = heights[i]
        j: int = i
        while j < n:
            if heights[j] < min_h:
                min_h = heights[j]
            area: int = min_h * (j - i + 1)
            if area > max_area:
                max_area = area
            j = j + 1
        i = i + 1
    return max_area


def max_consecutive_bars(heights: list[int], threshold: int) -> int:
    """Count max consecutive bars with height >= threshold."""
    max_count: int = 0
    current: int = 0
    i: int = 0
    while i < len(heights):
        if heights[i] >= threshold:
            current = current + 1
            if current > max_count:
                max_count = current
        else:
            current = 0
        i = i + 1
    return max_count


def histogram_volume(heights: list[int]) -> int:
    """Total volume (sum of all bar heights)."""
    total: int = 0
    i: int = 0
    while i < len(heights):
        total = total + heights[i]
        i = i + 1
    return total


def test_module() -> int:
    """Test histogram area operations."""
    ok: int = 0
    h: list[int] = [2, 1, 5, 6, 2, 3]
    if largest_rectangle_area(h) == 10:
        ok = ok + 1
    h2: list[int] = [2, 4]
    if largest_rectangle_area(h2) == 4:
        ok = ok + 1
    if max_consecutive_bars([1, 3, 5, 2, 4], 3) == 2:
        ok = ok + 1
    if histogram_volume([1, 2, 3]) == 6:
        ok = ok + 1
    return ok
