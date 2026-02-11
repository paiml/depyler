"""Cycle detection: Floyd's algorithm via index arrays, functional iteration.

Tests: detect_cycle, find_cycle_start, cycle_length, functional_cycle.
"""


def floyd_detect(nexts: list[int], head: int) -> int:
    """Detect cycle using Floyd's tortoise and hare. Returns 1 if cycle, 0 if not."""
    if head == -1:
        return 0
    slow: int = head
    fast: int = head
    while True:
        slow = nexts[slow]
        if nexts[fast] == -1:
            return 0
        fast = nexts[fast]
        if nexts[fast] == -1:
            return 0
        fast = nexts[fast]
        if slow == fast:
            return 1
        if slow == -1:
            return 0
    return 0


def floyd_cycle_start(nexts: list[int], head: int) -> int:
    """Find start of cycle. Returns -1 if no cycle."""
    if head == -1:
        return -1
    slow: int = head
    fast: int = head
    found: int = 0
    while True:
        slow = nexts[slow]
        if nexts[fast] == -1:
            return -1
        fast = nexts[fast]
        if nexts[fast] == -1:
            return -1
        fast = nexts[fast]
        if slow == fast:
            found = 1
            break
        if slow == -1:
            return -1
    if found == 0:
        return -1
    slow = head
    while slow != fast:
        slow = nexts[slow]
        fast = nexts[fast]
    return slow


def floyd_cycle_length(nexts: list[int], head: int) -> int:
    """Find length of cycle. Returns 0 if no cycle."""
    start: int = floyd_cycle_start(nexts, head)
    if start == -1:
        return 0
    length: int = 1
    curr: int = nexts[start]
    while curr != start:
        curr = nexts[curr]
        length = length + 1
    return length


def functional_cycle_detect(x0: int, limit: int) -> int:
    """Detect cycle in f(x) = (x*x + 1) % limit using Floyd's."""
    slow: int = x0
    fast: int = x0
    i: int = 0
    while i < limit * 2:
        slow = (slow * slow + 1) % limit
        fast = (fast * fast + 1) % limit
        fast = (fast * fast + 1) % limit
        if slow == fast:
            return 1
        i = i + 1
    return 0


def functional_cycle_length(x0: int, limit: int) -> int:
    """Find cycle length in f(x) = (x*x + 1) % limit."""
    slow: int = x0
    fast: int = x0
    i: int = 0
    while i < limit * 2:
        slow = (slow * slow + 1) % limit
        fast = (fast * fast + 1) % limit
        fast = (fast * fast + 1) % limit
        if slow == fast:
            break
        i = i + 1
    length: int = 1
    curr: int = (slow * slow + 1) % limit
    while curr != slow:
        curr = (curr * curr + 1) % limit
        length = length + 1
    return length


def has_duplicate_in_range(arr: list[int], n: int) -> int:
    """Use Floyd's on array values as next pointers to find duplicate in [1,n]."""
    slow: int = arr[0]
    fast: int = arr[0]
    while True:
        slow = arr[slow]
        fast = arr[arr[fast]]
        if slow == fast:
            break
    slow = arr[0]
    while slow != fast:
        slow = arr[slow]
        fast = arr[fast]
    return slow


def test_module() -> int:
    """Test cycle detection algorithms."""
    passed: int = 0

    nexts_cycle: list[int] = [1, 2, 3, 4, 2]
    if floyd_detect(nexts_cycle, 0) == 1:
        passed = passed + 1

    nexts_no: list[int] = [1, 2, 3, -1]
    if floyd_detect(nexts_no, 0) == 0:
        passed = passed + 1

    cs: int = floyd_cycle_start(nexts_cycle, 0)
    if cs == 2:
        passed = passed + 1

    cl: int = floyd_cycle_length(nexts_cycle, 0)
    if cl == 3:
        passed = passed + 1

    if functional_cycle_detect(2, 100) == 1:
        passed = passed + 1

    dup_arr: list[int] = [0, 1, 3, 4, 2, 3]
    d: int = has_duplicate_in_range(dup_arr, 5)
    if d == 3:
        passed = passed + 1

    return passed
