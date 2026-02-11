"""Stack with min/max tracking using parallel arrays.

Tests: push, pop, get min, get max.
"""


def stack_operations(ops: list[int], vals: list[int]) -> list[int]:
    """Simulate stack with min tracking. ops: 1=push, 2=pop, 3=getmin.
    Returns list of getmin results."""
    stack: list[int] = []
    min_stack: list[int] = []
    results: list[int] = []
    i: int = 0
    while i < len(ops):
        if ops[i] == 1:
            stack.append(vals[i])
            if len(min_stack) == 0:
                min_stack.append(vals[i])
            else:
                top_min: int = min_stack[len(min_stack) - 1]
                if vals[i] < top_min:
                    min_stack.append(vals[i])
                else:
                    min_stack.append(top_min)
        elif ops[i] == 2:
            if len(stack) > 0:
                stack.pop()
                min_stack.pop()
        elif ops[i] == 3:
            if len(min_stack) > 0:
                results.append(min_stack[len(min_stack) - 1])
            else:
                results.append(-1)
        i = i + 1
    return results


def find_running_min(arr: list[int]) -> list[int]:
    """Compute running minimum array."""
    result: list[int] = []
    if len(arr) == 0:
        return result
    current_min: int = arr[0]
    result.append(current_min)
    i: int = 1
    while i < len(arr):
        if arr[i] < current_min:
            current_min = arr[i]
        result.append(current_min)
        i = i + 1
    return result


def find_running_max(arr: list[int]) -> list[int]:
    """Compute running maximum array."""
    result: list[int] = []
    if len(arr) == 0:
        return result
    current_max: int = arr[0]
    result.append(current_max)
    i: int = 1
    while i < len(arr):
        if arr[i] > current_max:
            current_max = arr[i]
        result.append(current_max)
        i = i + 1
    return result


def test_module() -> int:
    """Test min-max stack operations."""
    ok: int = 0
    ops: list[int] = [1, 1, 1, 3, 2, 3]
    vals: list[int] = [5, 2, 8, 0, 0, 0]
    results: list[int] = stack_operations(ops, vals)
    if results[0] == 2:
        ok = ok + 1
    if results[1] == 5:
        ok = ok + 1
    rmin: list[int] = find_running_min([3, 1, 4, 1, 5])
    if rmin[0] == 3:
        ok = ok + 1
    if rmin[1] == 1:
        ok = ok + 1
    rmax: list[int] = find_running_max([1, 3, 2, 5, 4])
    if rmax[3] == 5:
        ok = ok + 1
    return ok
