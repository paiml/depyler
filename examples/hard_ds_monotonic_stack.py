"""Monotonic stack patterns: next greater element, histogram area, daily temperatures.

Tests: next_greater, prev_smaller, largest_rectangle, stock_span.
"""


def next_greater_element(arr: list[int]) -> list[int]:
    """For each element, find next greater element to the right. -1 if none."""
    n: int = len(arr)
    result: list[int] = []
    i: int = 0
    while i < n:
        result.append(-1)
        i = i + 1
    stack: list[int] = []
    j: int = 0
    while j < n:
        while len(stack) > 0:
            top_idx: int = stack[len(stack) - 1]
            if arr[top_idx] < arr[j]:
                stack.pop()
                result[top_idx] = arr[j]
            else:
                break
        stack.append(j)
        j = j + 1
    return result


def prev_smaller_element(arr: list[int]) -> list[int]:
    """For each element, find previous smaller element. -1 if none."""
    n: int = len(arr)
    result: list[int] = []
    i: int = 0
    while i < n:
        result.append(-1)
        i = i + 1
    stack: list[int] = []
    j: int = 0
    while j < n:
        while len(stack) > 0:
            top_val: int = stack[len(stack) - 1]
            if top_val >= arr[j]:
                stack.pop()
            else:
                break
        if len(stack) > 0:
            result[j] = stack[len(stack) - 1]
        stack.append(arr[j])
        j = j + 1
    return result


def largest_rectangle_histogram(heights: list[int]) -> int:
    """Find largest rectangle in histogram using monotonic stack."""
    n: int = len(heights)
    stack: list[int] = []
    max_area: int = 0
    i: int = 0
    while i <= n:
        if i < n:
            curr_h: int = heights[i]
        else:
            curr_h = 0
        while len(stack) > 0:
            top_idx: int = stack[len(stack) - 1]
            if heights[top_idx] > curr_h:
                stack.pop()
                h: int = heights[top_idx]
                if len(stack) == 0:
                    w: int = i
                else:
                    w = i - stack[len(stack) - 1] - 1
                area: int = h * w
                if area > max_area:
                    max_area = area
            else:
                break
        stack.append(i)
        i = i + 1
    return max_area


def stock_span(prices: list[int]) -> list[int]:
    """Calculate stock span: consecutive days price was <= current day."""
    n: int = len(prices)
    span: list[int] = []
    i: int = 0
    while i < n:
        span.append(1)
        i = i + 1
    stack: list[int] = []
    j: int = 0
    while j < n:
        while len(stack) > 0:
            top_idx: int = stack[len(stack) - 1]
            if prices[top_idx] <= prices[j]:
                stack.pop()
            else:
                break
        if len(stack) == 0:
            span[j] = j + 1
        else:
            span[j] = j - stack[len(stack) - 1]
        stack.append(j)
        j = j + 1
    return span


def daily_temperatures(temps: list[int]) -> list[int]:
    """Days until warmer temperature. 0 if never warmer."""
    n: int = len(temps)
    result: list[int] = []
    i: int = 0
    while i < n:
        result.append(0)
        i = i + 1
    stack: list[int] = []
    j: int = 0
    while j < n:
        while len(stack) > 0:
            top_idx: int = stack[len(stack) - 1]
            if temps[top_idx] < temps[j]:
                stack.pop()
                result[top_idx] = j - top_idx
            else:
                break
        stack.append(j)
        j = j + 1
    return result


def test_module() -> int:
    """Test monotonic stack operations."""
    passed: int = 0

    nge: list[int] = next_greater_element([4, 5, 2, 25])
    if nge == [5, 25, 25, -1]:
        passed = passed + 1

    pse: list[int] = prev_smaller_element([4, 5, 2, 10, 8])
    if pse == [-1, 4, -1, 2, 2]:
        passed = passed + 1

    area: int = largest_rectangle_histogram([2, 1, 5, 6, 2, 3])
    if area == 10:
        passed = passed + 1

    spans: list[int] = stock_span([100, 80, 60, 70, 60, 75, 85])
    if spans == [1, 1, 1, 2, 1, 4, 6]:
        passed = passed + 1

    dt: list[int] = daily_temperatures([73, 74, 75, 71, 69, 72, 76, 73])
    if dt == [1, 1, 4, 2, 1, 1, 0, 0]:
        passed = passed + 1

    nge2: list[int] = next_greater_element([3, 2, 1])
    if nge2 == [-1, -1, -1]:
        passed = passed + 1

    return passed
