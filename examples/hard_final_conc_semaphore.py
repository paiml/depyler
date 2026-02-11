"""Counting semaphore simulation.

Simulates semaphore operations (wait/signal) with a queue of waiting processes.
Tracks semaphore value, wait queue, and execution ordering.
"""


def sem_create(initial_val: int) -> list[int]:
    """Create semaphore state: [value, num_waiting]."""
    return [initial_val, 0]


def sem_wait(sem_state: list[int], wait_queue: list[int], proc_id: int) -> int:
    """Process attempts to acquire semaphore. Returns 1 if acquired, 0 if blocked."""
    val: int = sem_state[0]
    if val > 0:
        sem_state[0] = val - 1
        return 1
    nw: int = sem_state[1]
    sem_state[1] = nw + 1
    wait_queue.append(proc_id)
    return 0


def sem_signal(sem_state: list[int], wait_queue: list[int]) -> int:
    """Signal semaphore. Returns unblocked process id or -1."""
    nw: int = sem_state[1]
    if nw > 0:
        sem_state[1] = nw - 1
        unblocked: int = wait_queue[0]
        new_queue: list[int] = []
        i: int = 1
        while i < len(wait_queue):
            qv: int = wait_queue[i]
            new_queue.append(qv)
            i = i + 1
        i2: int = 0
        while i2 < len(wait_queue):
            if i2 < len(new_queue):
                wait_queue[i2] = new_queue[i2]
            i2 = i2 + 1
        while len(wait_queue) > len(new_queue):
            wait_queue.pop()
        return unblocked
    val: int = sem_state[0]
    sem_state[0] = val + 1
    return 0 - 1


def simulate_dining(num_philosophers: int, num_rounds: int) -> int:
    """Simulate dining philosophers with a counting semaphore.

    Allow at most (num_philosophers - 1) to eat simultaneously.
    Returns total meals consumed.
    """
    sem_state: list[int] = sem_create(num_philosophers - 1)
    wait_q: list[int] = []
    meals: int = 0
    rnd: int = 0
    while rnd < num_rounds:
        phil: int = 0
        while phil < num_philosophers:
            acquired: int = sem_wait(sem_state, wait_q, phil)
            if acquired == 1:
                meals = meals + 1
                sem_signal(sem_state, wait_q)
            phil = phil + 1
        rnd = rnd + 1
    return meals


def resource_pool(pool_size: int, requests: list[int]) -> int:
    """Simulate resource pool. requests[i]: 1=acquire, 0=release. Returns total successful acquires."""
    sem_state: list[int] = sem_create(pool_size)
    wait_q: list[int] = []
    acquired: int = 0
    i: int = 0
    while i < len(requests):
        rv: int = requests[i]
        if rv == 1:
            result: int = sem_wait(sem_state, wait_q, i)
            if result == 1:
                acquired = acquired + 1
        else:
            sem_signal(sem_state, wait_q)
        i = i + 1
    return acquired


def test_module() -> int:
    """Test semaphore simulation."""
    ok: int = 0
    ss: list[int] = sem_create(2)
    wq: list[int] = []
    r1: int = sem_wait(ss, wq, 0)
    if r1 == 1:
        ok = ok + 1
    r2: int = sem_wait(ss, wq, 1)
    if r2 == 1:
        ok = ok + 1
    r3: int = sem_wait(ss, wq, 2)
    if r3 == 0:
        ok = ok + 1
    unb: int = sem_signal(ss, wq)
    if unb == 2:
        ok = ok + 1
    meals: int = simulate_dining(3, 2)
    if meals > 0:
        ok = ok + 1
    return ok
