"""Ternary (3-ary) heap operations - min heap with 3 children per node."""


def ternary_parent(i: int) -> int:
    """Return parent index in ternary heap."""
    if i == 0:
        return 0
    result: int = (i - 1) // 3
    return result


def ternary_child(i: int, which: int) -> int:
    """Return child index (which=0,1,2) in ternary heap."""
    result: int = 3 * i + which + 1
    return result


def ternary_sift_up(heap: list[int], idx: int) -> int:
    """Sift up in ternary heap. Returns final position."""
    pos: int = idx
    while pos > 0:
        par: int = ternary_parent(pos)
        if heap[pos] < heap[par]:
            tmp: int = heap[pos]
            heap[pos] = heap[par]
            heap[par] = tmp
            pos = par
        else:
            return pos
    return pos


def ternary_sift_down(heap: list[int], size: int, idx: int) -> int:
    """Sift down in ternary heap. Returns final position."""
    pos: int = idx
    while pos < size:
        smallest: int = pos
        c: int = 0
        while c < 3:
            child: int = ternary_child(pos, c)
            if child < size and heap[child] < heap[smallest]:
                smallest = child
            c = c + 1
        if smallest == pos:
            return pos
        tmp: int = heap[pos]
        heap[pos] = heap[smallest]
        heap[smallest] = tmp
        pos = smallest
    return pos


def ternary_heap_insert(heap: list[int], size: int, val: int) -> int:
    """Insert into ternary heap. Returns new size."""
    heap.append(val)
    new_size: int = size + 1
    ternary_sift_up(heap, new_size - 1)
    return new_size


def ternary_heap_extract(heap: list[int], size: int) -> int:
    """Extract minimum. Returns min value."""
    if size == 0:
        return 0
    min_val: int = heap[0]
    heap[0] = heap[size - 1]
    ternary_sift_down(heap, size - 1, 0)
    return min_val


def build_ternary_heap(arr: list[int]) -> int:
    """Build ternary min heap in place. Returns size."""
    n: int = len(arr)
    i: int = (n - 2) // 3
    while i >= 0:
        ternary_sift_down(arr, n, i)
        i = i - 1
    return n


def test_module() -> int:
    """Test ternary heap."""
    passed: int = 0

    h: list[int] = []
    sz: int = 0
    sz = ternary_heap_insert(h, sz, 10)
    sz = ternary_heap_insert(h, sz, 5)
    sz = ternary_heap_insert(h, sz, 15)
    sz = ternary_heap_insert(h, sz, 2)

    if h[0] == 2:
        passed = passed + 1

    mn: int = ternary_heap_extract(h, sz)
    sz = sz - 1
    if mn == 2:
        passed = passed + 1

    if h[0] == 5:
        passed = passed + 1

    arr: list[int] = [8, 3, 7, 1, 5, 9, 2, 4, 6]
    build_ternary_heap(arr)
    if arr[0] == 1:
        passed = passed + 1

    if ternary_parent(4) == 1:
        passed = passed + 1

    if ternary_child(0, 0) == 1 and ternary_child(0, 2) == 3:
        passed = passed + 1

    return passed
