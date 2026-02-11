"""Heap sort implementation with sift_down, build_heap, and heap_sort.

Tests: sift_down correctness, build_heap ordering, heap_sort ascending.
"""


def sift_down(arr: list[int], start: int, end: int) -> list[int]:
    """Sift element at start down to maintain max-heap property."""
    root: int = start
    result: list[int] = arr[:]
    cont: int = 1
    while cont == 1:
        child: int = 2 * root + 1
        if child > end:
            cont = 0
        else:
            swap: int = root
            if result[swap] < result[child]:
                swap = child
            if child + 1 <= end and result[swap] < result[child + 1]:
                swap = child + 1
            if swap == root:
                cont = 0
            else:
                tmp: int = result[root]
                result[root] = result[swap]
                result[swap] = tmp
                root = swap
    return result


def build_heap(arr: list[int]) -> list[int]:
    """Build a max-heap from an unsorted array."""
    result: list[int] = arr[:]
    n: int = len(result)
    start: int = (n - 2) // 2
    while start >= 0:
        result = sift_down(result, start, n - 1)
        start = start - 1
    return result


def heap_sort(arr: list[int]) -> list[int]:
    """Sort array in ascending order using heap sort."""
    result: list[int] = build_heap(arr)
    end: int = len(result) - 1
    while end > 0:
        tmp: int = result[0]
        result[0] = result[end]
        result[end] = tmp
        end = end - 1
        result = sift_down(result, 0, end)
    return result


def test_module() -> int:
    """Test heap sort implementation."""
    ok: int = 0

    r1: list[int] = heap_sort([4, 10, 3, 5, 1])
    if r1 == [1, 3, 4, 5, 10]:
        ok = ok + 1

    r2: list[int] = heap_sort([])
    if r2 == []:
        ok = ok + 1

    r3: list[int] = heap_sort([1])
    if r3 == [1]:
        ok = ok + 1

    r4: list[int] = heap_sort([5, 4, 3, 2, 1])
    if r4 == [1, 2, 3, 4, 5]:
        ok = ok + 1

    r5: list[int] = heap_sort([1, 2, 3, 4, 5])
    if r5 == [1, 2, 3, 4, 5]:
        ok = ok + 1

    h: list[int] = build_heap([1, 5, 3, 4, 2])
    if h[0] == 5:
        ok = ok + 1

    return ok
