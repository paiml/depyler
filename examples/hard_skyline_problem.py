"""Simplified skyline problem using height arrays.

Tests: max height at each position, skyline outline, building overlap.
"""


def max_height_at_positions(buildings: list[int], n: int) -> list[int]:
    """Given buildings as [left, right, height, ...] triples, compute max height at each x.
    Width is fixed at max position n."""
    heights: list[int] = []
    i: int = 0
    while i < n:
        heights.append(0)
        i = i + 1
    b: int = 0
    while b < len(buildings):
        left: int = buildings[b]
        right: int = buildings[b + 1]
        h: int = buildings[b + 2]
        x: int = left
        while x < right:
            if x < n:
                if h > heights[x]:
                    heights[x] = h
            x = x + 1
        b = b + 3
    return heights


def count_height_changes(heights: list[int]) -> int:
    """Count number of times the height changes in a skyline profile."""
    if len(heights) == 0:
        return 0
    changes: int = 1
    i: int = 1
    while i < len(heights):
        if heights[i] != heights[i - 1]:
            changes = changes + 1
        i = i + 1
    return changes


def total_covered_width(heights: list[int]) -> int:
    """Count positions with non-zero height."""
    count: int = 0
    i: int = 0
    while i < len(heights):
        if heights[i] > 0:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test skyline operations."""
    ok: int = 0
    buildings: list[int] = [0, 3, 5, 2, 5, 3, 4, 7, 2]
    heights: list[int] = max_height_at_positions(buildings, 8)
    if heights[0] == 5:
        ok = ok + 1
    if heights[2] == 5:
        ok = ok + 1
    if heights[5] == 2:
        ok = ok + 1
    if count_height_changes(heights) > 0:
        ok = ok + 1
    if total_covered_width(heights) == 7:
        ok = ok + 1
    return ok
