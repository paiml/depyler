"""Binary min-heap using flat list with swim/sink operations.

Tests: insert, extract_min, peek, heapify, heap_sort.
"""


def heap_create() -> list[int]:
    """Create empty min-heap."""
    return []


def heap_parent(idx: int) -> int:
    """Get parent index."""
    return (idx - 1) // 2


def heap_left(idx: int) -> int:
    """Get left child index."""
    return 2 * idx + 1


def heap_right(idx: int) -> int:
    """Get right child index."""
    return 2 * idx + 2


def heap_swim(h: list[int], idx: int) -> int:
    """Swim element up to maintain heap property. Returns final index."""
    pos: int = idx
    while pos > 0:
        p: int = heap_parent(pos)
        if h[pos] < h[p]:
            tmp: int = h[pos]
            h[pos] = h[p]
            h[p] = tmp
            pos = p
        else:
            return pos
    return pos


def heap_sink(h: list[int], idx: int, sz: int) -> int:
    """Sink element down to maintain heap property. Returns final index."""
    pos: int = idx
    while True:
        smallest: int = pos
        lc: int = heap_left(pos)
        rc: int = heap_right(pos)
        if lc < sz:
            if h[lc] < h[smallest]:
                smallest = lc
        if rc < sz:
            if h[rc] < h[smallest]:
                smallest = rc
        if smallest == pos:
            return pos
        tmp: int = h[pos]
        h[pos] = h[smallest]
        h[smallest] = tmp
        pos = smallest
    return pos


def heap_insert(h: list[int], val: int) -> int:
    """Insert value into heap. Returns new size."""
    h.append(val)
    sz: int = len(h)
    heap_swim(h, sz - 1)
    return sz


def heap_extract_min(h: list[int]) -> int:
    """Extract minimum from heap. Returns -1 if empty."""
    sz: int = len(h)
    if sz == 0:
        return -1
    min_val: int = h[0]
    last_idx: int = sz - 1
    h[0] = h[last_idx]
    h.pop()
    new_sz: int = len(h)
    if new_sz > 0:
        heap_sink(h, 0, new_sz)
    return min_val


def heap_peek(h: list[int]) -> int:
    """Peek at minimum without removing. Returns -1 if empty."""
    if len(h) == 0:
        return -1
    return h[0]


def heapify(arr: list[int]) -> list[int]:
    """Convert array to min-heap in-place. Returns the array."""
    sz: int = len(arr)
    i: int = sz // 2 - 1
    while i >= 0:
        heap_sink(arr, i, sz)
        i = i - 1
    return arr


def heap_sort(arr: list[int]) -> list[int]:
    """Sort array using heap. Returns sorted list."""
    h: list[int] = []
    i: int = 0
    n: int = len(arr)
    while i < n:
        heap_insert(h, arr[i])
        i = i + 1
    result: list[int] = []
    while len(h) > 0:
        v: int = heap_extract_min(h)
        result.append(v)
    return result


def test_module() -> int:
    """Test binary heap operations."""
    passed: int = 0

    h: list[int] = heap_create()
    heap_insert(h, 5)
    heap_insert(h, 3)
    heap_insert(h, 8)
    heap_insert(h, 1)

    if heap_peek(h) == 1:
        passed = passed + 1

    v1: int = heap_extract_min(h)
    if v1 == 1:
        passed = passed + 1

    v2: int = heap_extract_min(h)
    if v2 == 3:
        passed = passed + 1

    sorted_arr: list[int] = heap_sort([5, 2, 8, 1, 9, 3])
    if sorted_arr == [1, 2, 3, 5, 8, 9]:
        passed = passed + 1

    arr: list[int] = [9, 4, 7, 1, 3]
    heapify(arr)
    if arr[0] == 1:
        passed = passed + 1

    empty_h: list[int] = heap_create()
    if heap_extract_min(empty_h) == -1:
        passed = passed + 1

    return passed
