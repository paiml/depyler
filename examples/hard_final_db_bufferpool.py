"""Buffer pool manager simulation with LRU eviction.

Manages a fixed-size pool of page frames. Tracks dirty pages,
pin counts, and implements clock/LRU eviction.
"""


def create_pool(pool_size: int) -> list[int]:
    """Create buffer pool. Returns page_ids array initialized to -1."""
    pages: list[int] = []
    i: int = 0
    while i < pool_size:
        pages.append(0 - 1)
        i = i + 1
    return pages


def find_page(page_ids: list[int], target_page: int) -> int:
    """Find frame index for given page_id. Returns -1 if not in pool."""
    i: int = 0
    while i < len(page_ids):
        pv: int = page_ids[i]
        if pv == target_page:
            return i
        i = i + 1
    return 0 - 1


def find_free_frame(page_ids: list[int]) -> int:
    """Find first free frame (page_id == -1). Returns -1 if none."""
    i: int = 0
    while i < len(page_ids):
        pv: int = page_ids[i]
        if pv < 0:
            return i
        i = i + 1
    return 0 - 1


def clock_evict(page_ids: list[int], pin_counts: list[int], ref_bits: list[int], clock_hand: int) -> int:
    """Clock algorithm eviction. Returns frame to evict, -1 if all pinned."""
    n: int = len(page_ids)
    scanned: int = 0
    hand: int = clock_hand
    while scanned < n * 2:
        frame: int = hand % n
        pc: int = pin_counts[frame]
        if pc == 0:
            rb: int = ref_bits[frame]
            if rb == 0:
                return frame
            else:
                ref_bits[frame] = 0
        hand = hand + 1
        scanned = scanned + 1
    return 0 - 1


def fetch_page(page_ids: list[int], pin_counts: list[int], ref_bits: list[int], dirty: list[int], target_page: int, clock_hand: int) -> int:
    """Fetch a page into pool. Returns frame index."""
    existing: int = find_page(page_ids, target_page)
    if existing >= 0:
        old_pin: int = pin_counts[existing]
        pin_counts[existing] = old_pin + 1
        ref_bits[existing] = 1
        return existing
    free: int = find_free_frame(page_ids)
    if free >= 0:
        page_ids[free] = target_page
        pin_counts[free] = 1
        ref_bits[free] = 1
        dirty[free] = 0
        return free
    evict: int = clock_evict(page_ids, pin_counts, ref_bits, clock_hand)
    if evict >= 0:
        page_ids[evict] = target_page
        pin_counts[evict] = 1
        ref_bits[evict] = 1
        dirty[evict] = 0
        return evict
    return 0 - 1


def unpin_page(pin_counts: list[int], frame: int, is_dirty: int, dirty: list[int]) -> int:
    """Unpin a page frame. Returns new pin count."""
    pc: int = pin_counts[frame]
    if pc > 0:
        pin_counts[frame] = pc - 1
    if is_dirty == 1:
        dirty[frame] = 1
    return pin_counts[frame]


def count_dirty(dirty: list[int]) -> int:
    """Count dirty pages in pool."""
    cnt: int = 0
    i: int = 0
    while i < len(dirty):
        dv: int = dirty[i]
        if dv == 1:
            cnt = cnt + 1
        i = i + 1
    return cnt


def test_module() -> int:
    """Test buffer pool manager."""
    ok: int = 0
    psize: int = 3
    pids: list[int] = create_pool(psize)
    pins: list[int] = [0, 0, 0]
    refs: list[int] = [0, 0, 0]
    drt: list[int] = [0, 0, 0]
    f1: int = fetch_page(pids, pins, refs, drt, 100, 0)
    if f1 == 0:
        ok = ok + 1
    f2: int = fetch_page(pids, pins, refs, drt, 200, 0)
    if f2 == 1:
        ok = ok + 1
    f1_again: int = fetch_page(pids, pins, refs, drt, 100, 0)
    if f1_again == 0:
        ok = ok + 1
    unpin_page(pins, 0, 1, drt)
    unpin_page(pins, 0, 0, drt)
    if count_dirty(drt) == 1:
        ok = ok + 1
    found: int = find_page(pids, 200)
    if found == 1:
        ok = ok + 1
    return ok
