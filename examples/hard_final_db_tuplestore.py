"""Tuple store (row-based storage) with slotted page simulation.

Pages store fixed-size tuples. Slot directory tracks which slots are occupied.
Operations: insert, delete, scan, update.
"""


def create_page(num_slots: int, tuple_width: int) -> list[int]:
    """Create a page with slot directory + data area.

    Layout: [slot_dir(num_slots)] + [data(num_slots * tuple_width)].
    Slot dir: 0=free, 1=occupied.
    """
    page_data: list[int] = []
    i: int = 0
    while i < num_slots:
        page_data.append(0)
        i = i + 1
    j: int = 0
    while j < num_slots * tuple_width:
        page_data.append(0)
        j = j + 1
    return page_data


def insert_tuple(page_data: list[int], num_slots: int, tuple_width: int, values: list[int]) -> int:
    """Insert tuple into first free slot. Returns slot index or -1."""
    slot: int = 0
    while slot < num_slots:
        sv: int = page_data[slot]
        if sv == 0:
            page_data[slot] = 1
            offset: int = num_slots + slot * tuple_width
            j: int = 0
            while j < tuple_width:
                if j < len(values):
                    vv: int = values[j]
                    page_data[offset + j] = vv
                j = j + 1
            return slot
        slot = slot + 1
    return 0 - 1


def delete_tuple(page_data: list[int], slot: int) -> int:
    """Delete tuple at slot. Returns 1 if deleted, 0 if already free."""
    sv: int = page_data[slot]
    if sv == 0:
        return 0
    page_data[slot] = 0
    return 1


def read_tuple(page_data: list[int], num_slots: int, tuple_width: int, slot: int) -> list[int]:
    """Read tuple from slot. Returns empty list if slot is free."""
    sv: int = page_data[slot]
    if sv == 0:
        return []
    result: list[int] = []
    offset: int = num_slots + slot * tuple_width
    j: int = 0
    while j < tuple_width:
        dv: int = page_data[offset + j]
        result.append(dv)
        j = j + 1
    return result


def update_tuple(page_data: list[int], num_slots: int, tuple_width: int, slot: int, values: list[int]) -> int:
    """Update tuple at slot. Returns 1 if updated, 0 if slot free."""
    sv: int = page_data[slot]
    if sv == 0:
        return 0
    offset: int = num_slots + slot * tuple_width
    j: int = 0
    while j < tuple_width:
        if j < len(values):
            vv: int = values[j]
            page_data[offset + j] = vv
        j = j + 1
    return 1


def scan_page(page_data: list[int], num_slots: int, tuple_width: int, col_idx: int, target_val: int) -> list[int]:
    """Scan page for tuples where column col_idx equals target_val. Returns matching slot indices."""
    matches: list[int] = []
    slot: int = 0
    while slot < num_slots:
        sv: int = page_data[slot]
        if sv == 1:
            offset: int = num_slots + slot * tuple_width + col_idx
            dv: int = page_data[offset]
            if dv == target_val:
                matches.append(slot)
        slot = slot + 1
    return matches


def count_occupied(page_data: list[int], num_slots: int) -> int:
    """Count occupied slots."""
    cnt: int = 0
    i: int = 0
    while i < num_slots:
        sv: int = page_data[i]
        if sv == 1:
            cnt = cnt + 1
        i = i + 1
    return cnt


def test_module() -> int:
    """Test tuple store."""
    ok: int = 0
    ns: int = 4
    tw: int = 3
    pg: list[int] = create_page(ns, tw)
    s0: int = insert_tuple(pg, ns, tw, [10, 20, 30])
    s1: int = insert_tuple(pg, ns, tw, [40, 50, 60])
    if s0 == 0:
        ok = ok + 1
    if s1 == 1:
        ok = ok + 1
    t0: list[int] = read_tuple(pg, ns, tw, 0)
    v0: int = t0[0]
    if v0 == 10:
        ok = ok + 1
    if count_occupied(pg, ns) == 2:
        ok = ok + 1
    matches: list[int] = scan_page(pg, ns, tw, 0, 40)
    if len(matches) == 1:
        ok = ok + 1
    return ok
