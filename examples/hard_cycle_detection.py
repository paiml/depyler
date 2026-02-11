"""Cycle detection in sequences using Floyd's and Brent's algorithms."""


def floyd_cycle_length(arr: list[int], start: int) -> int:
    """Detect cycle in a functional graph stored as array.
    arr[i] gives the next index. Returns cycle length or 0 if no cycle."""
    n: int = len(arr)
    if start < 0 or start >= n:
        return 0
    slow: int = start
    fast: int = start
    steps: int = 0
    found: int = 0
    while steps < n + 1:
        slow_next: int = arr[slow]
        if slow_next < 0 or slow_next >= n:
            return 0
        slow = slow_next
        fast_next1: int = arr[fast]
        if fast_next1 < 0 or fast_next1 >= n:
            return 0
        fast_next2: int = arr[fast_next1]
        if fast_next2 < 0 or fast_next2 >= n:
            return 0
        fast = fast_next2
        steps = steps + 1
        if slow == fast:
            found = 1
            steps = n + 2
    if found == 0:
        return 0
    cycle_len: int = 1
    runner: int = arr[slow]
    while runner != slow:
        runner = arr[runner]
        cycle_len = cycle_len + 1
    return cycle_len


def detect_cycle_start(arr: list[int], start: int) -> int:
    """Find the start index of a cycle in a functional graph.
    Returns -1 if no cycle found."""
    n: int = len(arr)
    if start < 0 or start >= n:
        return -1
    slow: int = start
    fast: int = start
    steps: int = 0
    found: int = 0
    while steps < n + 1:
        slow = arr[slow]
        fast_mid: int = arr[fast]
        fast = arr[fast_mid]
        steps = steps + 1
        if slow == fast:
            found = 1
            steps = n + 2
    if found == 0:
        return -1
    ptr1: int = start
    ptr2: int = slow
    while ptr1 != ptr2:
        ptr1 = arr[ptr1]
        ptr2 = arr[ptr2]
    return ptr1


def collatz_cycle_check(n: int, max_steps: int) -> int:
    """Check if Collatz sequence reaches 1 within max_steps.
    Returns number of steps or -1 if exceeds max_steps."""
    if n <= 0:
        return -1
    steps: int = 0
    current: int = n
    while current != 1 and steps < max_steps:
        if current % 2 == 0:
            current = current // 2
        else:
            current = 3 * current + 1
        steps = steps + 1
    if current == 1:
        return steps
    return -1


def test_module() -> int:
    """Test cycle detection functions."""
    ok: int = 0

    # Cycle: 0->1->2->3->1 (cycle of length 3 starting at 1)
    arr1: list[int] = [1, 2, 3, 1]
    if floyd_cycle_length(arr1, 0) == 3:
        ok = ok + 1

    if detect_cycle_start(arr1, 0) == 1:
        ok = ok + 1

    # Self-loop: 0->0
    arr2: list[int] = [0, 2, 1]
    if floyd_cycle_length(arr2, 0) == 1:
        ok = ok + 1

    if collatz_cycle_check(6, 100) == 8:
        ok = ok + 1

    if collatz_cycle_check(1, 100) == 0:
        ok = ok + 1

    if collatz_cycle_check(0, 100) == -1:
        ok = ok + 1

    return ok
