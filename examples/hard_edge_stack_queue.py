"""Stack and queue using lists (no classes)."""


def stack_push(stack_data: list[int], val: int) -> list[int]:
    """Push value onto stack."""
    result: list[int] = []
    i: int = 0
    while i < len(stack_data):
        result.append(stack_data[i])
        i = i + 1
    result.append(val)
    return result


def stack_pop(stack_data: list[int]) -> list[int]:
    """Pop from stack. Returns [popped_value, ...remaining_stack]."""
    if len(stack_data) == 0:
        return [-1]
    last_idx: int = len(stack_data) - 1
    popped: int = stack_data[last_idx]
    result: list[int] = [popped]
    i: int = 0
    while i < last_idx:
        result.append(stack_data[i])
        i = i + 1
    return result


def stack_peek(stack_data: list[int]) -> int:
    """Peek at top of stack."""
    if len(stack_data) == 0:
        return -1
    last_idx: int = len(stack_data) - 1
    return stack_data[last_idx]


def evaluate_postfix(tokens: list[int]) -> int:
    """Evaluate postfix expression. Tokens: positive = operand,
    -1 = add, -2 = subtract, -3 = multiply."""
    stack_data: list[int] = []
    i: int = 0
    while i < len(tokens):
        tok: int = tokens[i]
        if tok >= 0:
            stack_data.append(tok)
        else:
            if len(stack_data) < 2:
                return -1
            b_idx: int = len(stack_data) - 1
            b: int = stack_data[b_idx]
            stack_data.pop()
            a_idx: int = len(stack_data) - 1
            a: int = stack_data[a_idx]
            stack_data.pop()
            res: int = 0
            if tok == -1:
                res = a + b
            elif tok == -2:
                res = a - b
            elif tok == -3:
                res = a * b
            stack_data.append(res)
        i = i + 1
    if len(stack_data) == 0:
        return 0
    return stack_data[len(stack_data) - 1]


def queue_enqueue(queue_data: list[int], val: int) -> list[int]:
    """Enqueue value."""
    result: list[int] = []
    i: int = 0
    while i < len(queue_data):
        result.append(queue_data[i])
        i = i + 1
    result.append(val)
    return result


def queue_dequeue(queue_data: list[int]) -> list[int]:
    """Dequeue. Returns [dequeued_value, ...remaining_queue]."""
    if len(queue_data) == 0:
        return [-1]
    front: int = queue_data[0]
    result: list[int] = [front]
    i: int = 1
    while i < len(queue_data):
        result.append(queue_data[i])
        i = i + 1
    return result


def check_balanced_parens(parens: list[int]) -> int:
    """Check balanced parentheses using stack. 1=open, 2=close.
    Returns 1 if balanced, 0 otherwise."""
    stack_data: list[int] = []
    i: int = 0
    while i < len(parens):
        if parens[i] == 1:
            stack_data.append(1)
        elif parens[i] == 2:
            if len(stack_data) == 0:
                return 0
            stack_data.pop()
        i = i + 1
    if len(stack_data) == 0:
        return 1
    return 0


def next_greater_element(arr: list[int]) -> list[int]:
    """Find next greater element for each position using stack."""
    n: int = len(arr)
    result: list[int] = []
    i: int = 0
    while i < n:
        result.append(-1)
        i = i + 1
    stack_data: list[int] = []
    i = 0
    while i < n:
        while len(stack_data) > 0:
            top_idx: int = len(stack_data) - 1
            top: int = stack_data[top_idx]
            if arr[top] < arr[i]:
                result[top] = arr[i]
                stack_data.pop()
            else:
                break
        stack_data.append(i)
        i = i + 1
    return result


def test_module() -> int:
    """Test all stack and queue functions."""
    passed: int = 0
    s1: list[int] = stack_push([], 10)
    s2: list[int] = stack_push(s1, 20)
    if stack_peek(s2) == 20:
        passed = passed + 1
    popped: list[int] = stack_pop(s2)
    if popped[0] == 20:
        passed = passed + 1
    if stack_pop([])[0] == -1:
        passed = passed + 1
    pf: int = evaluate_postfix([3, 4, -1, 2, -3])
    if pf == 14:
        passed = passed + 1
    q1: list[int] = queue_enqueue([], 10)
    q2: list[int] = queue_enqueue(q1, 20)
    dq: list[int] = queue_dequeue(q2)
    if dq[0] == 10:
        passed = passed + 1
    bp: int = check_balanced_parens([1, 1, 2, 2])
    if bp == 1:
        passed = passed + 1
    bp2: int = check_balanced_parens([1, 2, 2, 1])
    if bp2 == 0:
        passed = passed + 1
    nge: list[int] = next_greater_element([4, 5, 2, 25])
    if nge[0] == 5:
        passed = passed + 1
    if nge[3] == -1:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
