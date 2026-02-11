# Queue implemented using two stacks (as lists)


def tsq_enqueue(inbox: list[int], value: int) -> list[int]:
    new_inbox: list[int] = []
    i: int = 0
    while i < len(inbox):
        new_inbox.append(inbox[i])
        i = i + 1
    new_inbox.append(value)
    return new_inbox


def tsq_transfer_to_outbox(inbox: list[int], outbox: list[int]) -> list[int]:
    # Move all from inbox to outbox (reversing order), return new outbox
    new_outbox: list[int] = []
    i: int = 0
    while i < len(outbox):
        new_outbox.append(outbox[i])
        i = i + 1
    idx: int = len(inbox) - 1
    while idx >= 0:
        new_outbox.append(inbox[idx])
        idx = idx - 1
    return new_outbox


def tsq_size(inbox: list[int], outbox: list[int]) -> int:
    return len(inbox) + len(outbox)


def tsq_pop_back(arr: list[int]) -> list[int]:
    # Return a new list with the last element removed
    result: list[int] = []
    i: int = 0
    limit: int = len(arr) - 1
    while i < limit:
        result.append(arr[i])
        i = i + 1
    return result


def tsq_peek_back(arr: list[int]) -> int:
    # Return the last element
    idx: int = len(arr) - 1
    return arr[idx]


def simple_queue_test() -> int:
    inbox: list[int] = []
    outbox: list[int] = []
    passed: int = 0

    # Test 1: enqueue 3 items
    inbox = tsq_enqueue(inbox, 10)
    inbox = tsq_enqueue(inbox, 20)
    inbox = tsq_enqueue(inbox, 30)
    if tsq_size(inbox, outbox) == 3:
        passed = passed + 1

    # Test 2: transfer and check outbox size
    outbox = tsq_transfer_to_outbox(inbox, outbox)
    inbox = []
    if len(outbox) == 3:
        passed = passed + 1

    # Test 3: dequeue gets first element (FIFO)
    val: int = tsq_peek_back(outbox)
    if val == 10:
        passed = passed + 1

    # Remove from outbox
    outbox = tsq_pop_back(outbox)

    # Test 4: next dequeue
    val = tsq_peek_back(outbox)
    if val == 20:
        passed = passed + 1

    # Test 5: size correct after removals
    outbox = tsq_pop_back(outbox)
    if tsq_size(inbox, outbox) == 1:
        passed = passed + 1

    # Test 6: enqueue while outbox has items
    inbox = tsq_enqueue(inbox, 40)
    if tsq_size(inbox, outbox) == 2:
        passed = passed + 1

    # Test 7: last item from outbox
    val = tsq_peek_back(outbox)
    if val == 30:
        passed = passed + 1

    return passed


def test_module() -> int:
    return simple_queue_test()
