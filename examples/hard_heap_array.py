"""Min-heap operations using array representation.

Tests: sift up, sift down, insert, extract min.
"""


def heap_parent(i: int) -> int:
    """Get parent index in heap."""
    return (i - 1) // 2


def heap_sift_up(heap: list[int], idx: int) -> list[int]:
    """Sift element up to maintain min-heap property."""
    result: list[int] = []
    i: int = 0
    while i < len(heap):
        result.append(heap[i])
        i = i + 1
    pos: int = idx
    while pos > 0:
        parent: int = (pos - 1) // 2
        if result[pos] < result[parent]:
            tmp: int = result[pos]
            result[pos] = result[parent]
            result[parent] = tmp
            pos = parent
        else:
            pos = 0
    return result


def heap_sift_down(heap: list[int], size: int) -> list[int]:
    """Sift root element down to maintain min-heap property."""
    result: list[int] = []
    i: int = 0
    while i < len(heap):
        result.append(heap[i])
        i = i + 1
    pos: int = 0
    done: int = 0
    while done == 0:
        left: int = 2 * pos + 1
        right: int = 2 * pos + 2
        smallest: int = pos
        if left < size and result[left] < result[smallest]:
            smallest = left
        if right < size and result[right] < result[smallest]:
            smallest = right
        if smallest != pos:
            tmp: int = result[pos]
            result[pos] = result[smallest]
            result[smallest] = tmp
            pos = smallest
        else:
            done = 1
    return result


def heap_insert(heap: list[int], val: int) -> list[int]:
    """Insert value into min-heap."""
    result: list[int] = []
    i: int = 0
    while i < len(heap):
        result.append(heap[i])
        i = i + 1
    result.append(val)
    pos: int = len(result) - 1
    while pos > 0:
        parent: int = (pos - 1) // 2
        if result[pos] < result[parent]:
            tmp: int = result[pos]
            result[pos] = result[parent]
            result[parent] = tmp
            pos = parent
        else:
            pos = 0
    return result


def is_min_heap_val(arr: list[int]) -> int:
    """Check if array satisfies min-heap property. Returns 1 for true, 0 for false."""
    n: int = len(arr)
    i: int = 0
    while i < n // 2:
        left: int = 2 * i + 1
        right: int = 2 * i + 2
        if left < n and arr[i] > arr[left]:
            return 0
        if right < n and arr[i] > arr[right]:
            return 0
        i = i + 1
    return 1


def test_module() -> None:
    assert heap_parent(1) == 0
    assert heap_parent(2) == 0
    assert heap_parent(3) == 1
    h1: list[int] = [1, 3, 5, 7]
    assert is_min_heap_val(h1) == 1
    h2: list[int] = [5, 3, 1, 7]
    assert is_min_heap_val(h2) == 0
    h3: list[int] = heap_insert([1, 3, 5], 0)
    assert h3[0] == 0
    assert is_min_heap_val([1, 2, 3, 4, 5]) == 1
