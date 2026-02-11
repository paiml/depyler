# Stack with O(1) min operation using auxiliary min stack


def stack_create() -> list[int]:
    # We store pairs: [value, min_at_this_level, value, min, ...]
    # Even indices = values, odd indices = running min
    result: list[int] = []
    return result


def stack_push(stack: list[int], value: int) -> list[int]:
    new_stack: list[int] = []
    i: int = 0
    while i < len(stack):
        new_stack.append(stack[i])
        i = i + 1
    current_min: int = value
    if len(stack) >= 2:
        prev_min: int = stack[len(stack) - 1]
        if prev_min < current_min:
            current_min = prev_min
    new_stack.append(value)
    new_stack.append(current_min)
    return new_stack


def stack_pop(stack: list[int]) -> list[int]:
    if len(stack) < 2:
        return []
    result: list[int] = []
    i: int = 0
    while i < len(stack) - 2:
        result.append(stack[i])
        i = i + 1
    return result


def stack_top(stack: list[int]) -> int:
    if len(stack) < 2:
        return -1
    return stack[len(stack) - 2]


def stack_min(stack: list[int]) -> int:
    if len(stack) < 2:
        return -1
    return stack[len(stack) - 1]


def stack_size(stack: list[int]) -> int:
    return len(stack) // 2


def test_module() -> int:
    passed: int = 0

    # Test 1: empty stack
    s: list[int] = stack_create()
    if stack_size(s) == 0:
        passed = passed + 1

    # Test 2: push and top
    s = stack_push(s, 5)
    if stack_top(s) == 5:
        passed = passed + 1

    # Test 3: min after single push
    if stack_min(s) == 5:
        passed = passed + 1

    # Test 4: push smaller, min updates
    s = stack_push(s, 3)
    if stack_min(s) == 3:
        passed = passed + 1

    # Test 5: push larger, min stays
    s = stack_push(s, 7)
    if stack_min(s) == 3:
        passed = passed + 1

    # Test 6: pop, min remains correct
    s = stack_pop(s)
    if stack_min(s) == 3 and stack_top(s) == 3:
        passed = passed + 1

    # Test 7: pop again, min reverts
    s = stack_pop(s)
    if stack_min(s) == 5 and stack_top(s) == 5:
        passed = passed + 1

    # Test 8: size tracking
    s = stack_push(s, 1)
    s = stack_push(s, 2)
    if stack_size(s) == 3:
        passed = passed + 1

    return passed
