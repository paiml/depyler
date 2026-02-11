# Stack and queue implementations using lists
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def stack_push(stack: list[int], value: int) -> list[int]:
    """Push a value onto a stack (end of list)."""
    result: list[int] = []
    i: int = 0
    while i < len(stack):
        result.append(stack[i])
        i = i + 1
    result.append(value)
    return result


def stack_pop(stack: list[int]) -> list[int]:
    """Pop the top value from a stack (remove last element)."""
    if len(stack) == 0:
        return []
    result: list[int] = []
    i: int = 0
    while i < len(stack) - 1:
        result.append(stack[i])
        i = i + 1
    return result


def stack_peek(stack: list[int]) -> int:
    """Peek at the top of the stack. Returns -1 if empty."""
    if len(stack) == 0:
        return -1
    return stack[len(stack) - 1]


def queue_enqueue(queue: list[int], value: int) -> list[int]:
    """Enqueue a value at the end."""
    result: list[int] = []
    i: int = 0
    while i < len(queue):
        result.append(queue[i])
        i = i + 1
    result.append(value)
    return result


def queue_dequeue(queue: list[int]) -> list[int]:
    """Dequeue the front value (remove first element)."""
    if len(queue) == 0:
        return []
    result: list[int] = []
    i: int = 1
    while i < len(queue):
        result.append(queue[i])
        i = i + 1
    return result


def test_module() -> int:
    s: list[int] = []
    s = stack_push(s, 10)
    s = stack_push(s, 20)
    s = stack_push(s, 30)
    assert stack_peek(s) == 30
    s = stack_pop(s)
    assert stack_peek(s) == 20
    assert len(s) == 2

    q: list[int] = []
    q = queue_enqueue(q, 1)
    q = queue_enqueue(q, 2)
    q = queue_enqueue(q, 3)
    assert q[0] == 1
    q = queue_dequeue(q)
    assert q[0] == 2
    assert len(q) == 2
    return 0


if __name__ == "__main__":
    test_module()
