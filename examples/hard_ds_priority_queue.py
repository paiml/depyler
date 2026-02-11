"""Priority queue using sorted insertion in a list.

Tests: insert, extract_min, peek, size, merge two priority queues.
"""


def pq_create() -> list[int]:
    """Create empty priority queue (sorted ascending)."""
    return []


def pq_insert(pq: list[int], val: int) -> int:
    """Insert value maintaining sorted order. Returns new size."""
    sz: int = len(pq)
    if sz == 0:
        pq.append(val)
        return 1
    pos: int = 0
    while pos < sz:
        if pq[pos] > val:
            break
        pos = pos + 1
    pq.append(0)
    i: int = sz
    while i > pos:
        pq[i] = pq[i - 1]
        i = i - 1
    pq[pos] = val
    return len(pq)


def pq_extract_min(pq: list[int]) -> int:
    """Remove and return minimum. Returns -1 if empty."""
    sz: int = len(pq)
    if sz == 0:
        return -1
    val: int = pq[0]
    i: int = 0
    while i < sz - 1:
        pq[i] = pq[i + 1]
        i = i + 1
    pq.pop()
    return val


def pq_peek_min(pq: list[int]) -> int:
    """Peek at minimum without removing. Returns -1 if empty."""
    if len(pq) == 0:
        return -1
    return pq[0]


def pq_size(pq: list[int]) -> int:
    """Return current size."""
    return len(pq)


def pq_is_empty(pq: list[int]) -> int:
    """Return 1 if empty, 0 otherwise."""
    if len(pq) == 0:
        return 1
    return 0


def pq_merge(a: list[int], b: list[int]) -> list[int]:
    """Merge two sorted priority queues into one."""
    result: list[int] = []
    i: int = 0
    j: int = 0
    a_len: int = len(a)
    b_len: int = len(b)
    while i < a_len:
        while j < b_len:
            if b[j] <= a[i]:
                result.append(b[j])
                j = j + 1
            else:
                break
        result.append(a[i])
        i = i + 1
    while j < b_len:
        result.append(b[j])
        j = j + 1
    return result


def pq_kth_smallest(pq: list[int], k_val: int) -> int:
    """Return k-th smallest (1-indexed). Returns -1 if out of bounds."""
    sz: int = len(pq)
    if k_val < 1:
        return -1
    if k_val > sz:
        return -1
    return pq[k_val - 1]


def test_module() -> int:
    """Test priority queue operations."""
    passed: int = 0

    pq: list[int] = pq_create()
    pq_insert(pq, 5)
    pq_insert(pq, 2)
    pq_insert(pq, 8)
    pq_insert(pq, 1)
    pq_insert(pq, 9)

    if pq_peek_min(pq) == 1:
        passed = passed + 1

    if pq_size(pq) == 5:
        passed = passed + 1

    v1: int = pq_extract_min(pq)
    if v1 == 1:
        passed = passed + 1

    v2: int = pq_extract_min(pq)
    if v2 == 2:
        passed = passed + 1

    a: list[int] = [1, 3, 5]
    b: list[int] = [2, 4, 6]
    merged: list[int] = pq_merge(a, b)
    if merged == [1, 2, 3, 4, 5, 6]:
        passed = passed + 1

    if pq_kth_smallest(merged, 3) == 3:
        passed = passed + 1

    empty_pq: list[int] = pq_create()
    if pq_is_empty(empty_pq) == 1:
        passed = passed + 1

    if pq_extract_min(empty_pq) == -1:
        passed = passed + 1

    return passed
