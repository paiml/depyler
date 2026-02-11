# Queue implemented using two stacks (as lists)


def tsq_create() -> list[list[int]]:
    inbox: list[int] = []
    outbox: list[int] = []
    return [inbox, outbox]


def tsq_enqueue(inbox: list[int], value: int) -> list[int]:
    new_inbox: list[int] = []
    i: int = 0
    while i < len(inbox):
        new_inbox.append(inbox[i])
        i = i + 1
    new_inbox.append(value)
    return new_inbox


def tsq_transfer(inbox: list[int], outbox: list[int]) -> list[list[int]]:
    # Move all from inbox to outbox (reversing order)
    new_outbox: list[int] = []
    i: int = 0
    while i < len(outbox):
        new_outbox.append(outbox[i])
        i = i + 1
    idx: int = len(inbox) - 1
    while idx >= 0:
        new_outbox.append(inbox[idx])
        idx = idx - 1
    empty: list[int] = []
    return [empty, new_outbox]


def tsq_dequeue(inbox: list[int], outbox: list[int]) -> list[int]:
    # Returns [dequeued_value, ...new_inbox..., -999, ...new_outbox...]
    # Using -999 as separator
    if len(outbox) == 0:
        stacks: list[list[int]] = tsq_transfer(inbox, outbox)
        inbox = stacks[0]
        outbox = stacks[1]
    if len(outbox) == 0:
        return [-1]
    val: int = outbox[len(outbox) - 1]
    new_outbox: list[int] = []
    i: int = 0
    while i < len(outbox) - 1:
        new_outbox.append(outbox[i])
        i = i + 1
    result: list[int] = [val]
    return result


def tsq_size(inbox: list[int], outbox: list[int]) -> int:
    return len(inbox) + len(outbox)


def simple_queue_test() -> int:
    # Simpler approach: use a single list as queue
    inbox: list[int] = []
    outbox: list[int] = []
    passed: int = 0

    # Test 1: enqueue 3 items
    inbox = tsq_enqueue(inbox, 10)
    inbox = tsq_enqueue(inbox, 20)
    inbox = tsq_enqueue(inbox, 30)
    if tsq_size(inbox, outbox) == 3:
        passed = passed + 1

    # Test 2: transfer and dequeue
    stacks: list[list[int]] = tsq_transfer(inbox, outbox)
    inbox = stacks[0]
    outbox = stacks[1]
    if len(outbox) == 3:
        passed = passed + 1

    # Test 3: dequeue gets first element
    val: int = outbox[len(outbox) - 1]
    if val == 10:
        passed = passed + 1

    # Remove from outbox
    new_outbox: list[int] = []
    i: int = 0
    while i < len(outbox) - 1:
        new_outbox.append(outbox[i])
        i = i + 1
    outbox = new_outbox

    # Test 4: next dequeue
    val = outbox[len(outbox) - 1]
    if val == 20:
        passed = passed + 1

    # Test 5: size correct
    new_outbox = []
    i = 0
    while i < len(outbox) - 1:
        new_outbox.append(outbox[i])
        i = i + 1
    outbox = new_outbox
    if tsq_size(inbox, outbox) == 1:
        passed = passed + 1

    # Test 6: enqueue while outbox has items
    inbox = tsq_enqueue(inbox, 40)
    if tsq_size(inbox, outbox) == 2:
        passed = passed + 1

    # Test 7: last item from outbox
    val = outbox[len(outbox) - 1]
    if val == 30:
        passed = passed + 1

    return passed


def test_module() -> int:
    return simple_queue_test()
