"""Largest rectangle in histogram using stack-based approach."""


def largest_rect_histogram(heights: list[int]) -> int:
    """Find area of largest rectangle in histogram."""
    n: int = len(heights)
    if n == 0:
        return 0
    stack: list[int] = []
    max_area: int = 0
    i: int = 0
    while i <= n:
        if i < n:
            cur_h: int = heights[i]
        else:
            cur_h = 0
        while len(stack) > 0 and cur_h < heights[stack[len(stack) - 1]]:
            top: int = stack[len(stack) - 1]
            stack.pop()
            h: int = heights[top]
            if len(stack) == 0:
                w: int = i
            else:
                w = i - stack[len(stack) - 1] - 1
            area: int = h * w
            if area > max_area:
                max_area = area
        stack.append(i)
        i = i + 1
    return max_area


def max_height(heights: list[int]) -> int:
    """Find maximum height in histogram."""
    if len(heights) == 0:
        return 0
    best: int = heights[0]
    i: int = 1
    while i < len(heights):
        if heights[i] > best:
            best = heights[i]
        i = i + 1
    return best


def total_area(heights: list[int]) -> int:
    """Sum of all bar areas (width=1 each)."""
    total: int = 0
    i: int = 0
    while i < len(heights):
        total = total + heights[i]
        i = i + 1
    return total


def test_module() -> int:
    """Test largest rectangle in histogram."""
    ok: int = 0
    h1: list[int] = [2, 1, 5, 6, 2, 3]
    if largest_rect_histogram(h1) == 10:
        ok = ok + 1
    h2: list[int] = [2, 4]
    if largest_rect_histogram(h2) == 4:
        ok = ok + 1
    h3: list[int] = [1, 1, 1, 1]
    if largest_rect_histogram(h3) == 4:
        ok = ok + 1
    h4: list[int] = [5]
    if largest_rect_histogram(h4) == 5:
        ok = ok + 1
    empty: list[int] = []
    if largest_rect_histogram(empty) == 0:
        ok = ok + 1
    if max_height(h1) == 6:
        ok = ok + 1
    if total_area(h1) == 19:
        ok = ok + 1
    return ok
