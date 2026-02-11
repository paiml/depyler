def skip_create(max_level: int, capacity: int) -> list[int]:
    size: int = max_level * capacity
    data: list[int] = []
    i: int = 0
    while i < size:
        data.append(-1)
        i = i + 1
    return data


def skip_get(data: list[int], capacity: int, level: int, pos: int) -> int:
    return data[level * capacity + pos]


def skip_set(data: list[int], capacity: int, level: int, pos: int, val: int) -> list[int]:
    data[level * capacity + pos] = val
    return data


def sorted_insert(arr: list[int], length: int, val: int) -> int:
    pos: int = 0
    while pos < length and arr[pos] < val:
        pos = pos + 1
    i: int = length
    while i > pos:
        arr[i] = arr[i - 1]
        i = i - 1
    arr[pos] = val
    return length + 1


def sorted_search(arr: list[int], length: int, val: int) -> int:
    low: int = 0
    high: int = length - 1
    while low <= high:
        mid: int = (low + high) // 2
        if arr[mid] == val:
            return mid
        elif arr[mid] < val:
            low = mid + 1
        else:
            high = mid - 1
    return -1


def sorted_contains(arr: list[int], length: int, val: int) -> int:
    if sorted_search(arr, length, val) >= 0:
        return 1
    return 0


def test_module() -> int:
    passed: int = 0
    data: list[int] = skip_create(3, 10)
    if len(data) == 30:
        passed = passed + 1
    data = skip_set(data, 10, 0, 0, 42)
    if skip_get(data, 10, 0, 0) == 42:
        passed = passed + 1
    arr: list[int] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    ln: int = 0
    ln = sorted_insert(arr, ln, 5)
    ln = sorted_insert(arr, ln, 3)
    ln = sorted_insert(arr, ln, 7)
    if arr[0] == 3 and arr[1] == 5 and arr[2] == 7:
        passed = passed + 1
    if sorted_search(arr, ln, 5) == 1:
        passed = passed + 1
    if sorted_search(arr, ln, 4) == -1:
        passed = passed + 1
    if sorted_contains(arr, ln, 7) == 1:
        passed = passed + 1
    if sorted_contains(arr, ln, 99) == 0:
        passed = passed + 1
    return passed
