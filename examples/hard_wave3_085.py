"""Systems: Priority queue / min-heap simulation.
Tests: heap operations, priority ordering, insertion/extraction.
"""
from typing import Dict, List, Tuple


def heap_sift_up(heap: List[int], idx: int) -> int:
    """Sift element up to maintain heap property. Returns final index."""
    while idx > 0:
        parent: int = (idx - 1) // 2
        if heap[idx] < heap[parent]:
            temp: int = heap[idx]
            heap[idx] = heap[parent]
            heap[parent] = temp
            idx = parent
        else:
            break
    return idx


def heap_sift_down(heap: List[int], idx: int, n: int) -> int:
    """Sift element down to maintain heap property. Returns final index."""
    while True:
        smallest: int = idx
        left: int = 2 * idx + 1
        right: int = 2 * idx + 2
        if left < n and heap[left] < heap[smallest]:
            smallest = left
        if right < n and heap[right] < heap[smallest]:
            smallest = right
        if smallest != idx:
            temp: int = heap[idx]
            heap[idx] = heap[smallest]
            heap[smallest] = temp
            idx = smallest
        else:
            break
    return idx


def heap_insert(heap: List[int], size: int, val: int) -> int:
    """Insert value into heap at position size. Returns new size."""
    heap[size] = val
    idx: int = size
    while idx > 0:
        parent: int = (idx - 1) // 2
        if heap[idx] < heap[parent]:
            temp: int = heap[idx]
            heap[idx] = heap[parent]
            heap[parent] = temp
            idx = parent
        else:
            break
    return size + 1


def heap_extract_min(heap: List[int], size: int) -> Tuple[int, int]:
    """Extract minimum from heap. Returns (min_val, new_size)."""
    if size == 0:
        return (-1, 0)
    min_val: int = heap[0]
    new_size: int = size - 1
    heap[0] = heap[new_size]
    idx: int = 0
    while True:
        smallest: int = idx
        left: int = 2 * idx + 1
        right: int = 2 * idx + 2
        if left < new_size and heap[left] < heap[smallest]:
            smallest = left
        if right < new_size and heap[right] < heap[smallest]:
            smallest = right
        if smallest != idx:
            temp: int = heap[idx]
            heap[idx] = heap[smallest]
            heap[smallest] = temp
            idx = smallest
        else:
            break
    return (min_val, new_size)


def heap_peek(heap: List[int], size: int) -> int:
    """Peek at minimum element without removing."""
    if size == 0:
        return -1
    return heap[0]


def heap_is_valid(heap: List[int], size: int) -> bool:
    """Check if array satisfies min-heap property."""
    i: int = 0
    while i < size:
        left: int = 2 * i + 1
        right: int = 2 * i + 2
        if left < size and heap[left] < heap[i]:
            return False
        if right < size and heap[right] < heap[i]:
            return False
        i += 1
    return True


def heapify(data: List[int], n: int) -> int:
    """Convert array to min-heap in-place. Returns n."""
    start: int = (n // 2) - 1
    while start >= 0:
        idx: int = start
        while True:
            smallest: int = idx
            left: int = 2 * idx + 1
            right: int = 2 * idx + 2
            if left < n and data[left] < data[smallest]:
                smallest = left
            if right < n and data[right] < data[smallest]:
                smallest = right
            if smallest != idx:
                temp: int = data[idx]
                data[idx] = data[smallest]
                data[smallest] = temp
                idx = smallest
            else:
                break
        start -= 1
    return n


def heap_sort_inplace(data: List[int], n: int) -> int:
    """Sort array using heap sort. Returns n."""
    sz: int = heapify(data, n)
    result_count: int = 0
    while sz > 0:
        r: Tuple[int, int] = heap_extract_min(data, sz)
        sz = r[1]
        data[sz] = r[0]
        result_count += 1
    i: int = 0
    j: int = n - 1
    while i < j:
        temp: int = data[i]
        data[i] = data[j]
        data[j] = temp
        i += 1
        j -= 1
    return n


def test_heap() -> bool:
    ok: bool = True
    max_cap: int = 10
    heap: List[int] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    size: int = 0
    size = heap_insert(heap, size, 5)
    size = heap_insert(heap, size, 3)
    size = heap_insert(heap, size, 7)
    size = heap_insert(heap, size, 1)
    if size != 4:
        ok = False
    if not heap_is_valid(heap, size):
        ok = False
    pk: int = heap_peek(heap, size)
    if pk != 1:
        ok = False
    r: Tuple[int, int] = heap_extract_min(heap, size)
    if r[0] != 1:
        ok = False
    size = r[1]
    if size != 3:
        ok = False
    data: List[int] = [5, 3, 1, 4, 2, 0, 0, 0, 0, 0]
    n: int = heapify(data, 5)
    if not heap_is_valid(data, 5):
        ok = False
    return ok
