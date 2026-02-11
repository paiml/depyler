"""Singly linked list using two parallel lists (next-indices + values).

Tests: create, append, prepend, delete, find, reverse, length.
"""


def ll_create() -> list[int]:
    """Create empty linked list. Returns [head, size]."""
    return [-1, 0]


def ll_values_create() -> list[int]:
    """Create empty values store."""
    return []


def ll_nexts_create() -> list[int]:
    """Create empty next-pointer store."""
    return []


def ll_append(meta: list[int], vals: list[int], nexts: list[int], val: int) -> int:
    """Append value to end of list. Returns new node index."""
    node_idx: int = len(vals)
    vals.append(val)
    nexts.append(-1)
    head: int = meta[0]
    if head == -1:
        meta[0] = node_idx
    else:
        curr: int = head
        nxt: int = nexts[curr]
        while nxt != -1:
            curr = nxt
            nxt = nexts[curr]
        nexts[curr] = node_idx
    sz: int = meta[1]
    meta[1] = sz + 1
    return node_idx


def ll_prepend(meta: list[int], vals: list[int], nexts: list[int], val: int) -> int:
    """Prepend value to front of list. Returns new node index."""
    node_idx: int = len(vals)
    vals.append(val)
    nexts.append(meta[0])
    meta[0] = node_idx
    sz: int = meta[1]
    meta[1] = sz + 1
    return node_idx


def ll_get(meta: list[int], vals: list[int], nexts: list[int], pos: int) -> int:
    """Get value at position. Returns -1 if out of bounds."""
    sz: int = meta[1]
    if pos < 0:
        return -1
    if pos >= sz:
        return -1
    curr: int = meta[0]
    i: int = 0
    while i < pos:
        curr = nexts[curr]
        i = i + 1
    return vals[curr]


def ll_find(meta: list[int], vals: list[int], nexts: list[int], val: int) -> int:
    """Find first occurrence of val. Returns position or -1."""
    curr: int = meta[0]
    pos: int = 0
    while curr != -1:
        if vals[curr] == val:
            return pos
        curr = nexts[curr]
        pos = pos + 1
    return -1


def ll_length(meta: list[int]) -> int:
    """Return length of list."""
    return meta[1]


def ll_to_list(meta: list[int], vals: list[int], nexts: list[int]) -> list[int]:
    """Convert linked list to regular list."""
    result: list[int] = []
    curr: int = meta[0]
    while curr != -1:
        result.append(vals[curr])
        curr = nexts[curr]
    return result


def ll_reverse(meta: list[int], nexts: list[int]) -> int:
    """Reverse the linked list in-place. Returns new head."""
    prev: int = -1
    curr: int = meta[0]
    while curr != -1:
        nxt: int = nexts[curr]
        nexts[curr] = prev
        prev = curr
        curr = nxt
    meta[0] = prev
    return prev


def test_module() -> int:
    """Test linked list operations."""
    passed: int = 0
    meta: list[int] = ll_create()
    vals: list[int] = ll_values_create()
    nxts: list[int] = ll_nexts_create()

    ll_append(meta, vals, nxts, 10)
    ll_append(meta, vals, nxts, 20)
    ll_append(meta, vals, nxts, 30)

    if ll_length(meta) == 3:
        passed = passed + 1

    if ll_get(meta, vals, nxts, 0) == 10:
        passed = passed + 1

    if ll_get(meta, vals, nxts, 2) == 30:
        passed = passed + 1

    if ll_find(meta, vals, nxts, 20) == 1:
        passed = passed + 1

    if ll_find(meta, vals, nxts, 99) == -1:
        passed = passed + 1

    ll_prepend(meta, vals, nxts, 5)
    if ll_get(meta, vals, nxts, 0) == 5:
        passed = passed + 1

    ll_reverse(meta, nxts)
    if ll_get(meta, vals, nxts, 0) == 30:
        passed = passed + 1

    r: list[int] = ll_to_list(meta, vals, nxts)
    if len(r) == 4:
        passed = passed + 1

    return passed
