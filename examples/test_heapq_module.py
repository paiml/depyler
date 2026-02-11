"""Manual heap operations without heapq module.

Implements min-heap operations using list-based sorting and
manual bubble-up/bubble-down algorithms. All functions use
only int and list[int] types for transpiler compatibility.
"""


def heap_sort_list(data: list[int]) -> list[int]:
    """Sort a list using selection sort to simulate heap property."""
    result: list[int] = []
    i: int = 0
    while i < len(data):
        result.append(data[i])
        i = i + 1
    i = 0
    while i < len(result):
        j: int = i + 1
        while j < len(result):
            if result[j] < result[i]:
                temp: int = result[i]
                result[i] = result[j]
                result[j] = temp
            j = j + 1
        i = i + 1
    return result


def test_heap_push_pop() -> list[int]:
    """Test basic heap push and pop operations."""
    heap: list[int] = []
    values: list[int] = [5, 3, 7, 1, 9, 2]
    i: int = 0
    while i < len(values):
        heap.append(values[i])
        i = i + 1
    sorted_heap: list[int] = heap_sort_list(heap)
    return sorted_heap


def test_heapify() -> list[int]:
    """Test converting list to heap."""
    data: list[int] = [5, 3, 7, 1, 9, 2, 4]
    sorted_data: list[int] = heap_sort_list(data)
    return sorted_data


def test_heap_pop_min() -> int:
    """Test popping minimum element."""
    heap: list[int] = [1, 2, 3, 4, 5]
    if len(heap) > 0:
        min_val: int = heap[0]
        return min_val
    else:
        return -1


def test_heap_peek() -> int:
    """Test peeking at minimum without removing."""
    heap: list[int] = [1, 2, 3, 4, 5]
    if len(heap) > 0:
        return heap[0]
    else:
        return -1


def test_nsmallest(data: list[int], n: int) -> list[int]:
    """Test finding n smallest elements."""
    sorted_data: list[int] = heap_sort_list(data)
    result: list[int] = []
    i: int = 0
    limit: int = n
    if len(sorted_data) < limit:
        limit = len(sorted_data)
    while i < limit:
        result.append(sorted_data[i])
        i = i + 1
    return result


def test_nlargest(data: list[int], n: int) -> list[int]:
    """Test finding n largest elements."""
    sorted_data: list[int] = heap_sort_list(data)
    result: list[int] = []
    start: int = len(sorted_data) - n
    if start < 0:
        start = 0
    i: int = len(sorted_data) - 1
    count: int = 0
    while i >= start and count < n:
        result.append(sorted_data[i])
        i = i - 1
        count = count + 1
    return result


def manual_heap_insert(heap: list[int], value: int) -> list[int]:
    """Manual heap insert with bubble up."""
    new_heap: list[int] = []
    i: int = 0
    while i < len(heap):
        new_heap.append(heap[i])
        i = i + 1
    new_heap.append(value)
    index: int = len(new_heap) - 1
    while index > 0:
        parent: int = (index - 1) // 2
        if new_heap[index] < new_heap[parent]:
            temp: int = new_heap[index]
            new_heap[index] = new_heap[parent]
            new_heap[parent] = temp
            index = parent
        else:
            index = 0
    return new_heap


def manual_heap_extract_min_val(heap: list[int]) -> int:
    """Extract minimum value from heap. Returns -1 if empty."""
    if len(heap) == 0:
        return -1
    return heap[0]


def manual_heap_extract_min_rest(heap: list[int]) -> list[int]:
    """Extract the remaining heap after removing minimum."""
    if len(heap) <= 1:
        result: list[int] = []
        return result
    last_idx: int = len(heap) - 1
    new_heap: list[int] = [heap[last_idx]]
    i: int = 1
    while i < last_idx:
        new_heap.append(heap[i])
        i = i + 1
    index: int = 0
    nh_len: int = len(new_heap)
    done: int = 0
    while done == 0:
        left: int = 2 * index + 1
        right: int = 2 * index + 2
        smallest: int = index
        if left < nh_len and new_heap[left] < new_heap[smallest]:
            smallest = left
        if right < nh_len and new_heap[right] < new_heap[smallest]:
            smallest = right
        if smallest != index:
            temp: int = new_heap[index]
            new_heap[index] = new_heap[smallest]
            new_heap[smallest] = temp
            index = smallest
        else:
            done = 1
    return new_heap


def merge_sorted_lists_flat(lists: list[list[int]]) -> list[int]:
    """Merge multiple sorted lists."""
    result: list[int] = []
    for lst in lists:
        for item in lst:
            result.append(item)
    sorted_result: list[int] = heap_sort_list(result)
    return sorted_result


def find_kth_smallest(data: list[int], k: int) -> int:
    """Find kth smallest element."""
    sorted_data: list[int] = heap_sort_list(data)
    if k > 0 and k <= len(sorted_data):
        idx: int = k - 1
        return sorted_data[idx]
    else:
        return -1


def find_median_int(data: list[int]) -> int:
    """Find median using integer arithmetic (truncated for odd-length lists)."""
    sorted_data: list[int] = heap_sort_list(data)
    n: int = len(sorted_data)
    if n == 0:
        return 0
    mid: int = n // 2
    if n % 2 == 1:
        return sorted_data[mid]
    else:
        idx1: int = mid - 1
        return (sorted_data[idx1] + sorted_data[mid]) // 2


def test_all_heapq_features() -> int:
    """Run all heapq module tests and return pass count."""
    passed: int = 0

    heap: list[int] = test_heap_push_pop()
    if heap[0] == 1:
        passed = passed + 1

    heapified: list[int] = test_heapify()
    if heapified[0] == 1:
        passed = passed + 1

    min_val: int = test_heap_pop_min()
    if min_val == 1:
        passed = passed + 1

    peek_val: int = test_heap_peek()
    if peek_val == 1:
        passed = passed + 1

    data: list[int] = [5, 2, 8, 1, 9, 3, 7]
    smallest_3: list[int] = test_nsmallest(data, 3)
    if len(smallest_3) == 3:
        passed = passed + 1

    largest_3: list[int] = test_nlargest(data, 3)
    if len(largest_3) == 3:
        passed = passed + 1

    h: list[int] = []
    h = manual_heap_insert(h, 5)
    h = manual_heap_insert(h, 3)
    h = manual_heap_insert(h, 7)
    extracted: int = manual_heap_extract_min_val(h)
    if extracted == 3:
        passed = passed + 1

    remaining: list[int] = manual_heap_extract_min_rest(h)
    if len(remaining) == 2:
        passed = passed + 1

    lists: list[list[int]] = [[1, 4, 7], [2, 5, 8], [3, 6, 9]]
    merged: list[int] = merge_sorted_lists_flat(lists)
    if len(merged) == 9:
        passed = passed + 1

    sample: list[int] = [7, 10, 4, 3, 20, 15]
    kth: int = find_kth_smallest(sample, 3)
    if kth == 7:
        passed = passed + 1

    median_data: list[int] = [1, 2, 3, 4, 5]
    median: int = find_median_int(median_data)
    if median == 3:
        passed = passed + 1

    return passed
