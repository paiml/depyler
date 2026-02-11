"""Stack-based algorithms: next greater element, stock span.

Tests: monotonic decreasing stack, index tracking.
"""


def next_greater_element(arr: list[int]) -> list[int]:
    """For each element, find the next greater element to its right."""
    n: int = len(arr)
    result: list[int] = []
    i: int = 0
    while i < n:
        result.append(-1)
        i += 1
    stack: list[int] = []
    i = 0
    while i < n:
        while len(stack) > 0 and arr[stack[len(stack) - 1]] < arr[i]:
            idx: int = stack.pop()
            result[idx] = arr[i]
        stack.append(i)
        i += 1
    return result


def stock_span(prices: list[int]) -> list[int]:
    """Compute stock span for each day."""
    n: int = len(prices)
    spans: list[int] = []
    i: int = 0
    while i < n:
        spans.append(1)
        i += 1
    stack: list[int] = []
    i = 0
    while i < n:
        while len(stack) > 0 and prices[stack[len(stack) - 1]] <= prices[i]:
            stack.pop()
        if len(stack) == 0:
            spans[i] = i + 1
        else:
            spans[i] = i - stack[len(stack) - 1]
        stack.append(i)
        i += 1
    return spans


def previous_smaller(arr: list[int]) -> list[int]:
    """For each element, find the previous smaller element."""
    n: int = len(arr)
    result: list[int] = []
    i: int = 0
    while i < n:
        result.append(-1)
        i += 1
    stack: list[int] = []
    i = 0
    while i < n:
        while len(stack) > 0 and stack[len(stack) - 1] >= arr[i]:
            stack.pop()
        if len(stack) > 0:
            result[i] = stack[len(stack) - 1]
        stack.append(arr[i])
        i += 1
    return result


def evaluate_postfix(tokens: list[int]) -> int:
    """Evaluate postfix expression.
    Positive numbers are operands.
    -1 = add, -2 = subtract, -3 = multiply.
    """
    stack: list[int] = []
    for t in tokens:
        if t >= 0:
            stack.append(t)
        elif t == -1:
            b: int = stack.pop()
            a: int = stack.pop()
            stack.append(a + b)
        elif t == -2:
            b2: int = stack.pop()
            a2: int = stack.pop()
            stack.append(a2 - b2)
        elif t == -3:
            b3: int = stack.pop()
            a3: int = stack.pop()
            stack.append(a3 * b3)
    if len(stack) > 0:
        return stack[0]
    return 0


def test_module() -> int:
    """Test stack-based algorithms."""
    ok: int = 0

    nge: list[int] = next_greater_element([4, 5, 2, 10])
    if nge == [5, 10, 10, -1]:
        ok += 1

    sp: list[int] = stock_span([100, 80, 60, 70, 60, 75, 85])
    if sp == [1, 1, 1, 2, 1, 4, 6]:
        ok += 1

    ps: list[int] = previous_smaller([4, 5, 2, 10, 8])
    if ps == [-1, 4, -1, 2, 2]:
        ok += 1

    # 3 4 + => 7
    val: int = evaluate_postfix([3, 4, -1])
    if val == 7:
        ok += 1

    # 5 3 - => 2
    val2: int = evaluate_postfix([5, 3, -2])
    if val2 == 2:
        ok += 1

    return ok
