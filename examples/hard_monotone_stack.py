"""Monotone stack problems: next greater, next smaller, and stock span."""


def next_greater_elements(arr: list[int]) -> list[int]:
    """For each element, find the next greater element to the right.
    Returns -1 if no greater element exists."""
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
            top_idx: int = len(stack) - 1
            top_val: int = stack[top_idx]
            if arr[top_val] < arr[j]:
                result[top_val] = arr[j]
                stack.pop()
            else:
                top_idx = -1
                top_val = -1
                break
        stack.append(j)
        j = j + 1
    return result


def next_smaller_elements(arr: list[int]) -> list[int]:
    """For each element, find the next smaller element to the right.
    Returns -1 if no smaller element exists."""
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
            top_idx: int = len(stack) - 1
            top_pos: int = stack[top_idx]
            if arr[top_pos] > arr[j]:
                result[top_pos] = arr[j]
                stack.pop()
            else:
                top_idx = -1
                top_pos = -1
                break
        stack.append(j)
        j = j + 1
    return result


def stock_span(prices: list[int]) -> list[int]:
    """Calculate stock span for each day.
    Span is the number of consecutive days before (including today)
    with price <= today's price."""
    n: int = len(prices)
    spans: list[int] = []
    i: int = 0
    while i < n:
        spans.append(1)
        i = i + 1
    stack: list[int] = []
    j: int = 0
    while j < n:
        while len(stack) > 0:
            top_idx: int = len(stack) - 1
            top_pos: int = stack[top_idx]
            if prices[top_pos] <= prices[j]:
                stack.pop()
            else:
                top_idx = -1
                top_pos = -1
                break
        if len(stack) == 0:
            spans[j] = j + 1
        else:
            prev_idx: int = len(stack) - 1
            spans[j] = j - stack[prev_idx]
        stack.append(j)
        j = j + 1
    return spans


def test_module() -> int:
    """Test monotone stack functions."""
    ok: int = 0

    arr1: list[int] = [4, 5, 2, 10, 8]
    nge: list[int] = next_greater_elements(arr1)
    if nge[0] == 5 and nge[1] == 10 and nge[2] == 10:
        ok = ok + 1

    last_idx: int = len(nge) - 1
    if nge[last_idx] == -1:
        ok = ok + 1

    nse: list[int] = next_smaller_elements(arr1)
    if nse[0] == 2 and nse[1] == 2:
        ok = ok + 1

    prices: list[int] = [100, 80, 60, 70, 60, 75, 85]
    sp: list[int] = stock_span(prices)
    if sp[0] == 1 and sp[1] == 1 and sp[2] == 1:
        ok = ok + 1

    if sp[3] == 2:
        ok = ok + 1

    if sp[5] == 4:
        ok = ok + 1

    return ok
