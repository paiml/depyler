"""Min-heap operations: insert, extract_min, heapify."""


def heap_parent(i: int) -> int:
    """Get parent index in heap."""
    return (i - 1) // 2


def heap_left(i: int) -> int:
    """Get left child index."""
    return 2 * i + 1


def heap_right(i: int) -> int:
    """Get right child index."""
    return 2 * i + 2


def sift_up(heap: list[int], idx: int) -> list[int]:
    """Sift element up to maintain min-heap property."""
    result: list[int] = []
    i: int = 0
    while i < len(heap):
        result.append(heap[i])
        i = i + 1
    ci: int = idx
    while ci > 0:
        pi: int = heap_parent(ci)
        if result[ci] < result[pi]:
            tmp: int = result[ci]
            result[ci] = result[pi]
            result[pi] = tmp
            ci = pi
        else:
            ci = 0
    return result


def sift_down(heap: list[int], idx: int, size: int) -> list[int]:
    """Sift element down to maintain min-heap property."""
    result: list[int] = []
    i: int = 0
    while i < len(heap):
        result.append(heap[i])
        i = i + 1
    ci: int = idx
    while heap_left(ci) < size:
        smallest: int = ci
        left: int = heap_left(ci)
        right: int = heap_right(ci)
        if left < size and result[left] < result[smallest]:
            smallest = left
        if right < size and result[right] < result[smallest]:
            smallest = right
        if smallest != ci:
            tmp: int = result[ci]
            result[ci] = result[smallest]
            result[smallest] = tmp
            ci = smallest
        else:
            ci = size
    return result


def heap_insert(heap: list[int], val: int) -> list[int]:
    """Insert value into min-heap."""
    new_heap: list[int] = []
    i: int = 0
    while i < len(heap):
        new_heap.append(heap[i])
        i = i + 1
    new_heap.append(val)
    return sift_up(new_heap, len(new_heap) - 1)


def heap_extract_min(heap: list[int]) -> list[int]:
    """Extract minimum and return [min_val, rest_of_heap...]."""
    if len(heap) == 0:
        return [-1]
    min_val: int = heap[0]
    if len(heap) == 1:
        return [min_val]
    new_heap: list[int] = []
    new_heap.append(heap[len(heap) - 1])
    i: int = 1
    while i < len(heap) - 1:
        new_heap.append(heap[i])
        i = i + 1
    new_heap = sift_down(new_heap, 0, len(new_heap))
    result: list[int] = [min_val]
    j: int = 0
    while j < len(new_heap):
        result.append(new_heap[j])
        j = j + 1
    return result


def build_heap(arr: list[int]) -> list[int]:
    """Build min-heap from array using bottom-up heapify."""
    heap: list[int] = []
    i: int = 0
    while i < len(arr):
        heap.append(arr[i])
        i = i + 1
    idx: int = len(heap) // 2 - 1
    while idx >= 0:
        heap = sift_down(heap, idx, len(heap))
        idx = idx - 1
    return heap


def test_module() -> int:
    passed: int = 0

    h: list[int] = []
    h = heap_insert(h, 5)
    h = heap_insert(h, 3)
    h = heap_insert(h, 7)
    h = heap_insert(h, 1)
    if h[0] == 1:
        passed = passed + 1

    ext: list[int] = heap_extract_min(h)
    if ext[0] == 1:
        passed = passed + 1

    bh: list[int] = build_heap([5, 3, 8, 1, 2])
    if bh[0] == 1:
        passed = passed + 1

    if heap_parent(3) == 1:
        passed = passed + 1

    if heap_left(1) == 3:
        passed = passed + 1

    empty_ext: list[int] = heap_extract_min([])
    if empty_ext[0] == -1:
        passed = passed + 1

    single_ext: list[int] = heap_extract_min([42])
    if single_ext[0] == 42 and len(single_ext) == 1:
        passed = passed + 1

    return passed
