"""Advanced stack patterns: min-stack, two-stack queue, stack sorting.

Tests: push, pop, get_min, enqueue, dequeue, sort_stack.
"""


def minstack_create() -> list[int]:
    """Create min-stack: even indices=values, odd indices=min-at-that-point."""
    return []


def minstack_push(stk: list[int], val: int) -> int:
    """Push value onto min-stack. Returns new size."""
    sz: int = len(stk)
    if sz == 0:
        stk.append(val)
        stk.append(val)
    else:
        prev_min: int = stk[sz - 1]
        stk.append(val)
        if val < prev_min:
            stk.append(val)
        else:
            stk.append(prev_min)
    return len(stk) // 2


def minstack_pop(stk: list[int]) -> int:
    """Pop value from min-stack. Returns popped value."""
    sz: int = len(stk)
    if sz == 0:
        return -1
    val_idx: int = sz - 2
    result: int = stk[val_idx]
    stk.pop()
    stk.pop()
    return result


def minstack_top(stk: list[int]) -> int:
    """Peek at top value."""
    sz: int = len(stk)
    if sz == 0:
        return -1
    return stk[sz - 2]


def minstack_get_min(stk: list[int]) -> int:
    """Get current minimum in O(1)."""
    sz: int = len(stk)
    if sz == 0:
        return -1
    return stk[sz - 1]


def twostack_queue_create() -> list[list[int]]:
    """Create queue using two stacks: [inbox, outbox]."""
    inbox: list[int] = []
    outbox: list[int] = []
    result: list[list[int]] = []
    result.append(inbox)
    result.append(outbox)
    return result


def twostack_enqueue(queue: list[list[int]], val: int) -> int:
    """Enqueue value. Returns queue size."""
    inbox: list[int] = queue[0]
    inbox.append(val)
    return len(inbox) + len(queue[1])


def twostack_transfer(queue: list[list[int]]) -> int:
    """Transfer from inbox to outbox if outbox empty."""
    inbox: list[int] = queue[0]
    outbox: list[int] = queue[1]
    if len(outbox) == 0:
        while len(inbox) > 0:
            v: int = inbox.pop()
            outbox.append(v)
    return len(outbox)


def twostack_dequeue(queue: list[list[int]]) -> int:
    """Dequeue value. Returns dequeued value or -1."""
    twostack_transfer(queue)
    outbox: list[int] = queue[1]
    if len(outbox) == 0:
        return -1
    return outbox.pop()


def sort_stack(stk: list[int]) -> list[int]:
    """Sort a stack using an auxiliary stack. Returns sorted list (ascending)."""
    temp: list[int] = []
    while len(stk) > 0:
        val: int = stk.pop()
        while len(temp) > 0:
            top_t: int = temp[len(temp) - 1]
            if top_t > val:
                stk.append(temp.pop())
            else:
                break
        temp.append(val)
    return temp


def stack_reverse(stk: list[int]) -> list[int]:
    """Reverse a stack using auxiliary stack."""
    aux: list[int] = []
    while len(stk) > 0:
        aux.append(stk.pop())
    return aux


def test_module() -> int:
    """Test stack operations."""
    passed: int = 0

    ms: list[int] = minstack_create()
    minstack_push(ms, 5)
    minstack_push(ms, 3)
    minstack_push(ms, 7)
    minstack_push(ms, 1)

    if minstack_get_min(ms) == 1:
        passed = passed + 1

    minstack_pop(ms)
    if minstack_get_min(ms) == 3:
        passed = passed + 1

    if minstack_top(ms) == 7:
        passed = passed + 1

    q: list[list[int]] = twostack_queue_create()
    twostack_enqueue(q, 10)
    twostack_enqueue(q, 20)
    twostack_enqueue(q, 30)
    r1: int = twostack_dequeue(q)
    if r1 == 10:
        passed = passed + 1

    r2: int = twostack_dequeue(q)
    if r2 == 20:
        passed = passed + 1

    s: list[int] = [3, 1, 4, 1, 5]
    sorted_s: list[int] = sort_stack(s)
    if sorted_s == [1, 1, 3, 4, 5]:
        passed = passed + 1

    s2: list[int] = [1, 2, 3]
    rev: list[int] = stack_reverse(s2)
    if rev == [3, 2, 1]:
        passed = passed + 1

    empty_q: list[list[int]] = twostack_queue_create()
    if twostack_dequeue(empty_q) == -1:
        passed = passed + 1

    return passed
