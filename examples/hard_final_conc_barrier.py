"""Barrier synchronization simulation.

Simulates a barrier where N threads must all arrive before any can proceed.
Tracks arrival count, generation number, and waiting threads.
"""


def create_barrier(num_threads: int) -> list[int]:
    """Create barrier state: [count_needed, arrived, generation]."""
    return [num_threads, 0, 0]


def barrier_arrive(state: list[int], thread_id: int) -> int:
    """Thread arrives at barrier. Returns generation when released (or -1 if still waiting)."""
    needed: int = state[0]
    arrived: int = state[1]
    arrived = arrived + 1
    state[1] = arrived
    if arrived >= needed:
        gen: int = state[2]
        state[1] = 0
        state[2] = gen + 1
        return gen
    return 0 - 1


def simulate_phases(num_threads: int, num_phases: int) -> list[int]:
    """Simulate multi-phase computation with barrier sync.

    Returns [total_work_done, phases_completed].
    """
    state: list[int] = create_barrier(num_threads)
    work_done: int = 0
    phases: int = 0
    phase: int = 0
    while phase < num_phases:
        tid: int = 0
        while tid < num_threads:
            work_done = work_done + 1
            result: int = barrier_arrive(state, tid)
            if result >= 0:
                phases = phases + 1
            tid = tid + 1
        phase = phase + 1
    return [work_done, phases]


def sense_barrier_step(arrived: list[int], sense: list[int], thread_id: int, num_threads: int) -> int:
    """Sense-reversing barrier step. Returns 1 when all threads arrived."""
    old: int = arrived[0]
    arrived[0] = old + 1
    if arrived[0] >= num_threads:
        arrived[0] = 0
        sv: int = sense[0]
        if sv == 0:
            sense[0] = 1
        else:
            sense[0] = 0
        return 1
    return 0


def tree_barrier_arrive(level_counts: list[int], levels: int, thread_id: int, fan_in: int) -> int:
    """Tree barrier: thread arrives at leaf, propagates up. Returns 1 when root reached."""
    lv: int = 0
    while lv < levels:
        old: int = level_counts[lv]
        level_counts[lv] = old + 1
        if level_counts[lv] >= fan_in:
            level_counts[lv] = 0
            lv = lv + 1
        else:
            return 0
    return 1


def test_module() -> int:
    """Test barrier simulation."""
    ok: int = 0
    bs: list[int] = create_barrier(3)
    r0: int = barrier_arrive(bs, 0)
    if r0 == 0 - 1:
        ok = ok + 1
    r1: int = barrier_arrive(bs, 1)
    if r1 == 0 - 1:
        ok = ok + 1
    r2: int = barrier_arrive(bs, 2)
    if r2 == 0:
        ok = ok + 1
    result: list[int] = simulate_phases(4, 3)
    rv0: int = result[0]
    rv1: int = result[1]
    if rv0 == 12:
        ok = ok + 1
    if rv1 == 3:
        ok = ok + 1
    return ok
