"""Next greater element using stack-based approach."""


def next_greater_element(arr: list[int]) -> list[int]:
    """For each element, find next greater element to the right. -1 if none."""
    length: int = len(arr)
    result: list[int] = []
    idx: int = 0
    while idx < length:
        result.append(-1)
        idx = idx + 1
    stack: list[int] = []
    si: int = 0
    while si < length:
        stack_len: int = len(stack)
        while stack_len > 0:
            top_idx: int = stack_len - 1
            top_val: int = stack[top_idx]
            if arr[top_val] < arr[si]:
                result[top_val] = arr[si]
                stack.pop()
                stack_len = len(stack)
            else:
                stack_len = 0
        stack.append(si)
        si = si + 1
    return result


def next_smaller_element(arr: list[int]) -> list[int]:
    """For each element, find next smaller element to the right. -1 if none."""
    length: int = len(arr)
    result: list[int] = []
    idx: int = 0
    while idx < length:
        result.append(-1)
        idx = idx + 1
    stack: list[int] = []
    si: int = 0
    while si < length:
        stack_len: int = len(stack)
        while stack_len > 0:
            top_idx: int = stack_len - 1
            top_val: int = stack[top_idx]
            if arr[top_val] > arr[si]:
                result[top_val] = arr[si]
                stack.pop()
                stack_len = len(stack)
            else:
                stack_len = 0
        stack.append(si)
        si = si + 1
    return result


def count_elements_with_next_greater(arr: list[int]) -> int:
    """Count how many elements have a next greater element."""
    nge: list[int] = next_greater_element(arr)
    count: int = 0
    idx: int = 0
    length: int = len(nge)
    while idx < length:
        if nge[idx] != -1:
            count = count + 1
        idx = idx + 1
    return count


def test_module() -> int:
    passed: int = 0

    nge: list[int] = next_greater_element([4, 5, 2, 25])
    if nge[0] == 5:
        passed = passed + 1
    if nge[1] == 25:
        passed = passed + 1
    if nge[3] == -1:
        passed = passed + 1

    nse: list[int] = next_smaller_element([4, 8, 5, 2, 25])
    if nse[0] == 2:
        passed = passed + 1
    if nse[1] == 5:
        passed = passed + 1

    if count_elements_with_next_greater([1, 2, 3]) == 2:
        passed = passed + 1

    nge2: list[int] = next_greater_element([3, 2, 1])
    if nge2[0] == -1:
        passed = passed + 1

    return passed
