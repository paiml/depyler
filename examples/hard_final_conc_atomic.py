"""Atomic counter and CAS-based operations simulation.

Simulates atomic increment, decrement, compare-and-swap, and fetch-and-add
operations on shared integer state.
"""


def atomic_load(state: list[int], idx: int) -> int:
    """Atomic load from state array."""
    return state[idx]


def atomic_store(state: list[int], idx: int, value: int) -> int:
    """Atomic store to state array. Returns stored value."""
    state[idx] = value
    return value


def atomic_cas(state: list[int], idx: int, expected: int, desired: int) -> int:
    """Compare-and-swap. Returns 1 if swapped, 0 if not."""
    current: int = state[idx]
    if current == expected:
        state[idx] = desired
        return 1
    return 0


def atomic_fetch_add(state: list[int], idx: int, delta: int) -> int:
    """Fetch-and-add. Returns old value."""
    old: int = state[idx]
    state[idx] = old + delta
    return old


def atomic_fetch_sub(state: list[int], idx: int, delta: int) -> int:
    """Fetch-and-subtract. Returns old value."""
    old: int = state[idx]
    state[idx] = old - delta
    return old


def atomic_exchange(state: list[int], idx: int, new_val: int) -> int:
    """Atomic exchange. Returns old value."""
    old: int = state[idx]
    state[idx] = new_val
    return old


def spin_increment(state: list[int], idx: int, target: int) -> int:
    """Spin until CAS succeeds to increment to target. Returns attempts needed."""
    attempts: int = 0
    done: int = 0
    while done == 0:
        old: int = atomic_load(state, idx)
        result: int = atomic_cas(state, idx, old, old + 1)
        attempts = attempts + 1
        if result == 1:
            if state[idx] >= target:
                done = 1
        if attempts > 1000:
            done = 1
    return attempts


def atomic_min(state: list[int], idx: int, candidate: int) -> int:
    """Atomically set state[idx] = min(state[idx], candidate). Returns old value."""
    old: int = state[idx]
    if candidate < old:
        state[idx] = candidate
    return old


def atomic_max(state: list[int], idx: int, candidate: int) -> int:
    """Atomically set state[idx] = max(state[idx], candidate). Returns old value."""
    old: int = state[idx]
    if candidate > old:
        state[idx] = candidate
    return old


def simulate_counter(num_threads: int, increments_per: int) -> int:
    """Simulate concurrent counter increments. Returns final count."""
    state: list[int] = [0]
    tid: int = 0
    while tid < num_threads:
        inc: int = 0
        while inc < increments_per:
            atomic_fetch_add(state, 0, 1)
            inc = inc + 1
        tid = tid + 1
    return state[0]


def test_module() -> int:
    """Test atomic operations."""
    ok: int = 0
    st: list[int] = [0, 100, 200]
    old: int = atomic_fetch_add(st, 0, 5)
    if old == 0:
        ok = ok + 1
    v0: int = st[0]
    if v0 == 5:
        ok = ok + 1
    cas_ok: int = atomic_cas(st, 1, 100, 999)
    if cas_ok == 1:
        ok = ok + 1
    cas_fail: int = atomic_cas(st, 2, 0, 999)
    if cas_fail == 0:
        ok = ok + 1
    final_cnt: int = simulate_counter(4, 10)
    if final_cnt == 40:
        ok = ok + 1
    return ok
