"""Queue operations: reverse, rotate, interleave."""


def queue_reverse(arr: list[int]) -> list[int]:
    """Reverse a queue (list) using a stack approach."""
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


def queue_rotate(arr: list[int], positions: int) -> list[int]:
    """Rotate queue left by given number of positions."""
    length: int = len(arr)
    if length == 0:
        empty_result: list[int] = []
        return empty_result
    effective: int = positions % length
    result: list[int] = []
    idx: int = effective
    while idx < length:
        result.append(arr[idx])
        idx = idx + 1
    idx2: int = 0
    while idx2 < effective:
        result.append(arr[idx2])
        idx2 = idx2 + 1
    return result


def queue_interleave(arr: list[int]) -> list[int]:
    """Interleave first half with second half of queue.
    [1,2,3,4,5,6] -> [1,4,2,5,3,6]."""
    length: int = len(arr)
    half: int = length // 2
    result: list[int] = []
    idx: int = 0
    while idx < half:
        result.append(arr[idx])
        second_idx: int = half + idx
        if second_idx < length:
            result.append(arr[second_idx])
        idx = idx + 1
    if length % 2 == 1:
        last_idx: int = length - 1
        result.append(arr[last_idx])
    return result


def queue_dedup_preserve_order(arr: list[int]) -> list[int]:
    """Remove duplicates while preserving order."""
    seen: dict[str, int] = {}
    result: list[int] = []
    idx: int = 0
    length: int = len(arr)
    while idx < length:
        val_str: str = str(arr[idx])
        if val_str not in seen:
            result.append(arr[idx])
            seen[val_str] = 1
        idx = idx + 1
    return result


def test_module() -> int:
    passed: int = 0

    rev: list[int] = queue_reverse([1, 2, 3, 4])
    if rev[0] == 4:
        passed = passed + 1
    if rev[3] == 1:
        passed = passed + 1

    rot: list[int] = queue_rotate([1, 2, 3, 4, 5], 2)
    if rot[0] == 3:
        passed = passed + 1

    inter: list[int] = queue_interleave([1, 2, 3, 4, 5, 6])
    if inter[0] == 1:
        passed = passed + 1
    if inter[1] == 4:
        passed = passed + 1

    dedup: list[int] = queue_dedup_preserve_order([1, 2, 1, 3, 2, 4])
    if len(dedup) == 4:
        passed = passed + 1

    if queue_rotate([1], 5)[0] == 1:
        passed = passed + 1

    return passed
