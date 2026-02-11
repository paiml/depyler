"""Manual binary heap with sift up and sift down operations."""


def heap_parent(i: int) -> int:
    """Return parent index."""
    result: int = (i - 1) // 2
    return result


def heap_left(i: int) -> int:
    """Return left child index."""
    result: int = 2 * i + 1
    return result


def heap_right(i: int) -> int:
    """Return right child index."""
    result: int = 2 * i + 2
    return result


def sift_up(heap: list[int], idx: int) -> int:
    """Sift element up to restore heap property. Returns final index."""
    pos: int = idx
    while pos > 0:
        par: int = heap_parent(pos)
        if heap[pos] < heap[par]:
            tmp: int = heap[pos]
            heap[pos] = heap[par]
            heap[par] = tmp
            pos = par
        else:
            return pos
    return pos


def sift_down(heap: list[int], size: int, idx: int) -> int:
    """Sift element down to restore min-heap property. Returns final index."""
    pos: int = idx
    while pos < size:
        left: int = heap_left(pos)
        right: int = heap_right(pos)
        smallest: int = pos
        if left < size and heap[left] < heap[smallest]:
            smallest = left
        if right < size and heap[right] < heap[smallest]:
            smallest = right
        if smallest == pos:
            return pos
        tmp: int = heap[pos]
        heap[pos] = heap[smallest]
        heap[smallest] = tmp
        pos = smallest
    return pos


def heap_insert(heap: list[int], size: int, val: int) -> int:
    """Insert value into heap. Returns new size."""
    heap.append(val)
    new_size: int = size + 1
    sift_up(heap, new_size - 1)
    return new_size


def heap_extract_min(heap: list[int], size: int) -> int:
    """Extract minimum from heap. Returns the min value."""
    if size == 0:
        return 0
    min_val: int = heap[0]
    heap[0] = heap[size - 1]
    sift_down(heap, size - 1, 0)
    return min_val


def build_min_heap(arr: list[int]) -> int:
    """Build min heap in place. Returns size."""
    n: int = len(arr)
    i: int = n // 2 - 1
    while i >= 0:
        sift_down(arr, n, i)
        i = i - 1
    return n


def heap_peek(heap: list[int], size: int) -> int:
    """Return minimum without removing."""
    if size == 0:
        return 0
    return heap[0]


def test_module() -> int:
    """Test binary heap operations."""
    passed: int = 0

    h: list[int] = []
    sz: int = 0
    sz = heap_insert(h, sz, 5)
    sz = heap_insert(h, sz, 3)
    sz = heap_insert(h, sz, 7)
    sz = heap_insert(h, sz, 1)

    if heap_peek(h, sz) == 1:
        passed = passed + 1

    mn: int = heap_extract_min(h, sz)
    sz = sz - 1
    if mn == 1:
        passed = passed + 1

    if heap_peek(h, sz) == 3:
        passed = passed + 1

    arr: list[int] = [9, 4, 7, 1, 3]
    build_min_heap(arr)
    if arr[0] == 1:
        passed = passed + 1

    if heap_parent(3) == 1:
        passed = passed + 1

    if heap_left(1) == 3:
        passed = passed + 1

    return passed
