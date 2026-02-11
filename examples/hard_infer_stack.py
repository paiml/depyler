# Type inference test: Stack operations with untyped params
# Strategy: Return types annotated, parameter types MISSING on some functions


def stack_push(stack: list[int], val) -> int:
    """Push value onto stack. Returns new size."""
    stack.append(val)
    return len(stack)


def stack_pop(stack: list[int]) -> int:
    """Pop value from stack. Returns popped value or -1 if empty."""
    if len(stack) == 0:
        return 0 - 1
    val: int = stack[len(stack) - 1]
    stack.pop()
    return val


def stack_peek(stack: list[int]) -> int:
    """Peek at top of stack without removing."""
    if len(stack) == 0:
        return 0 - 1
    return stack[len(stack) - 1]


def stack_size(stack: list[int]) -> int:
    """Return size of stack."""
    return len(stack)


def stack_is_empty(stack: list[int]) -> int:
    """Return 1 if stack is empty, 0 otherwise."""
    if len(stack) == 0:
        return 1
    return 0


def stack_min(stack: list[int]) -> int:
    """Find minimum element in stack."""
    if len(stack) == 0:
        return 0 - 1
    result: int = stack[0]
    i: int = 1
    while i < len(stack):
        if stack[i] < result:
            result = stack[i]
        i = i + 1
    return result


def stack_reverse_sum(stack: list[int]) -> int:
    """Sum elements from top to bottom."""
    total: int = 0
    i: int = len(stack) - 1
    while i >= 0:
        total = total + stack[i]
        i = i - 1
    return total


def balanced_parens_check(depths: list[int]) -> int:
    """Check if a sequence of depth changes represents balanced parens.
    1 = open, -1 = close. Returns 1 if balanced, 0 otherwise."""
    depth: int = 0
    i: int = 0
    while i < len(depths):
        depth = depth + depths[i]
        if depth < 0:
            return 0
        i = i + 1
    if depth == 0:
        return 1
    return 0


def stack_sort_ascending(stack: list[int]) -> int:
    """Sort stack in ascending order using auxiliary stack pattern.
    Returns 1 if sorted successfully."""
    aux: list[int] = []
    while len(stack) > 0:
        temp: int = stack[len(stack) - 1]
        stack.pop()
        while len(aux) > 0 and aux[len(aux) - 1] > temp:
            val: int = aux[len(aux) - 1]
            aux.pop()
            stack.append(val)
        aux.append(temp)
    while len(aux) > 0:
        val2: int = aux[len(aux) - 1]
        aux.pop()
        stack.append(val2)
    return 1


def test_module() -> int:
    """Test all stack inference functions."""
    total: int = 0

    # Basic stack operations
    stk: list[int] = []
    if stack_is_empty(stk) == 1:
        total = total + 1

    stack_push(stk, 10)
    stack_push(stk, 20)
    stack_push(stk, 30)
    if stack_size(stk) == 3:
        total = total + 1
    if stack_peek(stk) == 30:
        total = total + 1

    popped: int = stack_pop(stk)
    if popped == 30:
        total = total + 1
    if stack_size(stk) == 2:
        total = total + 1

    # stack_min test
    stk2: list[int] = [5, 2, 8, 1, 9]
    if stack_min(stk2) == 1:
        total = total + 1

    # stack_reverse_sum test
    if stack_reverse_sum(stk2) == 25:
        total = total + 1

    # balanced_parens_check tests
    balanced: list[int] = [1, 1, 0 - 1, 0 - 1]
    if balanced_parens_check(balanced) == 1:
        total = total + 1

    unbalanced: list[int] = [1, 0 - 1, 0 - 1]
    if balanced_parens_check(unbalanced) == 0:
        total = total + 1

    # stack_sort test
    sort_stk: list[int] = [3, 1, 4, 1, 5]
    stack_sort_ascending(sort_stk)
    if sort_stk[0] >= sort_stk[len(sort_stk) - 1]:
        total = total + 1

    # Empty stack pop test
    empty_stk: list[int] = []
    if stack_pop(empty_stk) == 0 - 1:
        total = total + 1

    return total
