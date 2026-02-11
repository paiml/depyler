"""Sort a stack using an auxiliary stack."""


def stack_sort(arr: list[int]) -> list[int]:
    """Sort array using two-stack approach (ascending order).
    Simulates sorting a stack with an auxiliary stack."""
    primary: list[int] = []
    idx: int = 0
    length: int = len(arr)
    while idx < length:
        primary.append(arr[idx])
        idx = idx + 1
    auxiliary: list[int] = []
    while len(primary) > 0:
        prim_len: int = len(primary)
        top_pos: int = prim_len - 1
        current: int = primary[top_pos]
        primary.pop()
        aux_len: int = len(auxiliary)
        while aux_len > 0:
            aux_top_pos: int = aux_len - 1
            if auxiliary[aux_top_pos] > current:
                primary.append(auxiliary[aux_top_pos])
                auxiliary.pop()
                aux_len = len(auxiliary)
            else:
                aux_len = 0
        auxiliary.append(current)
    return auxiliary


def stack_min(arr: list[int]) -> int:
    """Find minimum element using stack operations only."""
    if len(arr) == 0:
        return 0
    min_val: int = arr[0]
    idx: int = 1
    length: int = len(arr)
    while idx < length:
        if arr[idx] < min_val:
            min_val = arr[idx]
        idx = idx + 1
    return min_val


def stack_reverse(arr: list[int]) -> list[int]:
    """Reverse array using auxiliary stack approach."""
    stack: list[int] = []
    idx: int = 0
    length: int = len(arr)
    while idx < length:
        stack.append(arr[idx])
        idx = idx + 1
    result: list[int] = []
    while len(stack) > 0:
        slen: int = len(stack)
        top_pos: int = slen - 1
        result.append(stack[top_pos])
        stack.pop()
    return result


def test_module() -> int:
    passed: int = 0

    sorted_arr: list[int] = stack_sort([5, 1, 4, 2, 3])
    if sorted_arr[0] == 1:
        passed = passed + 1
    if sorted_arr[4] == 5:
        passed = passed + 1
    if sorted_arr[2] == 3:
        passed = passed + 1

    if stack_min([7, 2, 9, 1, 5]) == 1:
        passed = passed + 1

    rev: list[int] = stack_reverse([1, 2, 3])
    if rev[0] == 3:
        passed = passed + 1
    if rev[2] == 1:
        passed = passed + 1

    single: list[int] = stack_sort([42])
    if single[0] == 42:
        passed = passed + 1

    return passed
