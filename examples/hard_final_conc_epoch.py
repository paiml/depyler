"""Epoch-based garbage collection simulation.

Tracks global epoch, per-thread epochs, and deferred reclamation lists.
Objects retired in epoch E can be freed when all threads have advanced past E.
"""


def create_epoch_state(num_threads: int) -> list[int]:
    """Create state: [global_epoch, thread_epoch_0, ..., thread_epoch_N-1]."""
    state: list[int] = [0]
    i: int = 0
    while i < num_threads:
        state.append(0)
        i = i + 1
    return state


def get_global_epoch(state: list[int]) -> int:
    """Get current global epoch."""
    return state[0]


def pin_thread(state: list[int], thread_id: int) -> int:
    """Pin thread to current global epoch. Returns epoch value."""
    ge: int = state[0]
    state[thread_id + 1] = ge
    return ge


def unpin_thread(state: list[int], thread_id: int) -> int:
    """Unpin thread (set to -1 meaning inactive)."""
    state[thread_id + 1] = 0 - 1
    return 1


def try_advance_epoch(state: list[int], num_threads: int) -> int:
    """Try to advance global epoch. Returns 1 if advanced.

    Can only advance if all active threads are at current epoch.
    """
    ge: int = state[0]
    i: int = 0
    while i < num_threads:
        te: int = state[i + 1]
        if te >= 0:
            if te < ge:
                return 0
        i = i + 1
    state[0] = ge + 1
    return 1


def retire_in_epoch(garbage_bins: list[int], epoch: int, item_id: int) -> int:
    """Add item to garbage bin for given epoch. Format: [epoch, item, epoch, item, ...]."""
    garbage_bins.append(epoch)
    garbage_bins.append(item_id)
    return len(garbage_bins) // 2


def collect_garbage(garbage_bins: list[int], safe_epoch: int) -> list[int]:
    """Collect all items retired in epochs <= safe_epoch. Returns [collected_count, remaining_items]."""
    collected: int = 0
    remaining: list[int] = []
    i: int = 0
    while i < len(garbage_bins):
        ep: int = garbage_bins[i]
        item: int = garbage_bins[i + 1]
        if ep <= safe_epoch:
            collected = collected + 1
        else:
            remaining.append(ep)
            remaining.append(item)
        i = i + 2
    while len(garbage_bins) > 0:
        garbage_bins.pop()
    j: int = 0
    while j < len(remaining):
        rv: int = remaining[j]
        garbage_bins.append(rv)
        j = j + 1
    remaining_count: int = len(garbage_bins) // 2
    return [collected, remaining_count]


def safe_epoch_for_collection(state: list[int], num_threads: int) -> int:
    """Find the minimum epoch across all active threads. Items below this can be collected."""
    min_ep: int = state[0]
    i: int = 0
    while i < num_threads:
        te: int = state[i + 1]
        if te >= 0:
            if te < min_ep:
                min_ep = te
        i = i + 1
    return min_ep - 1


def test_module() -> int:
    """Test epoch-based GC."""
    ok: int = 0
    nt: int = 3
    st: list[int] = create_epoch_state(nt)
    ge: int = get_global_epoch(st)
    if ge == 0:
        ok = ok + 1
    pin_thread(st, 0)
    pin_thread(st, 1)
    pin_thread(st, 2)
    adv: int = try_advance_epoch(st, nt)
    if adv == 1:
        ok = ok + 1
    garbage: list[int] = []
    retire_in_epoch(garbage, 0, 100)
    retire_in_epoch(garbage, 0, 200)
    retire_in_epoch(garbage, 1, 300)
    pin_thread(st, 0)
    pin_thread(st, 1)
    pin_thread(st, 2)
    try_advance_epoch(st, nt)
    safe_ep: int = safe_epoch_for_collection(st, nt)
    result: list[int] = collect_garbage(garbage, safe_ep)
    collected: int = result[0]
    if collected == 2:
        ok = ok + 1
    remaining: int = result[1]
    if remaining == 1:
        ok = ok + 1
    unpin_thread(st, 2)
    ge2: int = get_global_epoch(st)
    if ge2 == 2:
        ok = ok + 1
    return ok
