"""Read-write lock simulation.

Multiple readers can hold the lock simultaneously.
Writers need exclusive access. Readers-preference implementation.
"""


def create_rwlock() -> list[int]:
    """Create rwlock state: [reader_count, writer_active, waiting_writers]."""
    return [0, 0, 0]


def acquire_read(state: list[int]) -> int:
    """Acquire read lock. Returns 1 if acquired, 0 if blocked by writer."""
    writer_active: int = state[1]
    if writer_active == 1:
        return 0
    rc: int = state[0]
    state[0] = rc + 1
    return 1


def release_read(state: list[int]) -> int:
    """Release read lock. Returns remaining reader count."""
    rc: int = state[0]
    if rc > 0:
        state[0] = rc - 1
    return state[0]


def acquire_write(state: list[int]) -> int:
    """Acquire write lock. Returns 1 if acquired, 0 if blocked."""
    rc: int = state[0]
    wa: int = state[1]
    if rc == 0:
        if wa == 0:
            state[1] = 1
            return 1
    ww: int = state[2]
    state[2] = ww + 1
    return 0


def release_write(state: list[int]) -> int:
    """Release write lock. Returns 1."""
    state[1] = 0
    return 1


def simulate_rw_workload(reads: int, writes: int) -> list[int]:
    """Simulate a read-heavy workload. Returns [successful_reads, successful_writes]."""
    state: list[int] = create_rwlock()
    succ_reads: int = 0
    succ_writes: int = 0
    total: int = reads + writes
    i: int = 0
    while i < total:
        if i < reads:
            got: int = acquire_read(state)
            if got == 1:
                succ_reads = succ_reads + 1
                release_read(state)
        else:
            got2: int = acquire_write(state)
            if got2 == 1:
                succ_writes = succ_writes + 1
                release_write(state)
        i = i + 1
    return [succ_reads, succ_writes]


def fair_rw_schedule(ops: list[int]) -> list[int]:
    """Execute a schedule of ops: 0=read_acquire, 1=read_release, 2=write_acquire, 3=write_release.

    Returns [reads_held_max, writes_held].
    """
    state: list[int] = create_rwlock()
    max_readers: int = 0
    total_writes: int = 0
    i: int = 0
    while i < len(ops):
        ov: int = ops[i]
        if ov == 0:
            acquire_read(state)
            rc: int = state[0]
            if rc > max_readers:
                max_readers = rc
        if ov == 1:
            release_read(state)
        if ov == 2:
            res: int = acquire_write(state)
            if res == 1:
                total_writes = total_writes + 1
        if ov == 3:
            release_write(state)
        i = i + 1
    return [max_readers, total_writes]


def test_module() -> int:
    """Test read-write lock."""
    ok: int = 0
    st: list[int] = create_rwlock()
    r1: int = acquire_read(st)
    r2: int = acquire_read(st)
    if r1 == 1:
        ok = ok + 1
    if r2 == 1:
        ok = ok + 1
    w1: int = acquire_write(st)
    if w1 == 0:
        ok = ok + 1
    release_read(st)
    release_read(st)
    w2: int = acquire_write(st)
    if w2 == 1:
        ok = ok + 1
    sim: list[int] = simulate_rw_workload(10, 5)
    sr: int = sim[0]
    if sr == 10:
        ok = ok + 1
    return ok
