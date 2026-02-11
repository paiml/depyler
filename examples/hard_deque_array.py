# Double-ended queue on array


def deque_create() -> list[int]:
    result: list[int] = []
    return result


def deque_push_front(deque: list[int], value: int) -> list[int]:
    new_deque: list[int] = [value]
    i: int = 0
    while i < len(deque):
        new_deque.append(deque[i])
        i = i + 1
    return new_deque


def deque_push_back(deque: list[int], value: int) -> list[int]:
    new_deque: list[int] = []
    i: int = 0
    while i < len(deque):
        new_deque.append(deque[i])
        i = i + 1
    new_deque.append(value)
    return new_deque


def deque_pop_front(deque: list[int]) -> list[int]:
    if len(deque) == 0:
        return []
    result: list[int] = []
    i: int = 1
    while i < len(deque):
        result.append(deque[i])
        i = i + 1
    return result


def deque_pop_back(deque: list[int]) -> list[int]:
    if len(deque) == 0:
        return []
    result: list[int] = []
    i: int = 0
    while i < len(deque) - 1:
        result.append(deque[i])
        i = i + 1
    return result


def deque_front(deque: list[int]) -> int:
    if len(deque) == 0:
        return -1
    return deque[0]


def deque_back(deque: list[int]) -> int:
    if len(deque) == 0:
        return -1
    return deque[len(deque) - 1]


def deque_size(deque: list[int]) -> int:
    return len(deque)


def test_module() -> int:
    passed: int = 0

    # Test 1: empty deque
    d: list[int] = deque_create()
    if deque_size(d) == 0:
        passed = passed + 1

    # Test 2: push front
    d = deque_push_front(d, 10)
    d = deque_push_front(d, 20)
    if deque_front(d) == 20:
        passed = passed + 1

    # Test 3: push back
    d = deque_push_back(d, 30)
    if deque_back(d) == 30:
        passed = passed + 1

    # Test 4: pop front
    d = deque_pop_front(d)
    if deque_front(d) == 10:
        passed = passed + 1

    # Test 5: pop back
    d = deque_pop_back(d)
    if deque_back(d) == 10:
        passed = passed + 1

    # Test 6: size
    if deque_size(d) == 1:
        passed = passed + 1

    # Test 7: multiple operations
    d = deque_create()
    d = deque_push_back(d, 1)
    d = deque_push_back(d, 2)
    d = deque_push_front(d, 0)
    if deque_front(d) == 0 and deque_back(d) == 2 and deque_size(d) == 3:
        passed = passed + 1

    return passed
