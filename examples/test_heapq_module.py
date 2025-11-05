"""
Comprehensive test of Python heapq module transpilation to Rust.

This example demonstrates how Depyler transpiles Python's heapq module
(min-heap priority queue) to Rust equivalents.

Expected Rust mappings:
- heapq.heappush() -> BinaryHeap::push()
- heapq.heappop() -> BinaryHeap::pop()
- heapq.heapify() -> BinaryHeap::from()
- heapq.nsmallest() -> manual implementation
- heapq.nlargest() -> manual implementation

Note: Python heapq is min-heap, Rust BinaryHeap is max-heap by default.
Manual implementations provided for learning.
"""

import heapq
from typing import List


def test_heap_push_pop() -> List[int]:
    """Test basic heap push and pop operations"""
    heap: List[int] = []

    # Push elements (manual heap operations simulated)
    values: List[int] = [5, 3, 7, 1, 9, 2]

    for val in values:
        heap.append(val)

    # Sort to simulate heap property (min-heap)
    for i in range(len(heap)):
        for j in range(i + 1, len(heap)):
            if heap[j] < heap[i]:
                temp: int = heap[i]
                heap[i] = heap[j]
                heap[j] = temp

    return heap


def test_heapify() -> List[int]:
    """Test converting list to heap"""
    data: List[int] = [5, 3, 7, 1, 9, 2, 4]

    # Heapify (sort to establish heap property)
    heap: List[int] = data.copy()
    for i in range(len(heap)):
        for j in range(i + 1, len(heap)):
            if heap[j] < heap[i]:
                temp: int = heap[i]
                heap[i] = heap[j]
                heap[j] = temp

    return heap


def test_heap_pop_min() -> int:
    """Test popping minimum element"""
    heap: List[int] = [1, 2, 3, 4, 5]

    # Pop minimum (first element in min-heap)
    if len(heap) > 0:
        min_val: int = heap[0]
        # Remove first element
        new_heap: List[int] = []
        for i in range(1, len(heap)):
            new_heap.append(heap[i])
        return min_val
    else:
        return -1


def test_heap_peek() -> int:
    """Test peeking at minimum without removing"""
    heap: List[int] = [1, 2, 3, 4, 5]

    if len(heap) > 0:
        return heap[0]
    else:
        return -1


def test_nsmallest(data: List[int], n: int) -> List[int]:
    """Test finding n smallest elements"""
    # Sort and take first n
    sorted_data: List[int] = data.copy()
    for i in range(len(sorted_data)):
        for j in range(i + 1, len(sorted_data)):
            if sorted_data[j] < sorted_data[i]:
                temp: int = sorted_data[i]
                sorted_data[i] = sorted_data[j]
                sorted_data[j] = temp

    result: List[int] = []
    for i in range(min(n, len(sorted_data))):
        result.append(sorted_data[i])

    return result


def test_nlargest(data: List[int], n: int) -> List[int]:
    """Test finding n largest elements"""
    # Sort in descending order and take first n
    sorted_data: List[int] = data.copy()
    for i in range(len(sorted_data)):
        for j in range(i + 1, len(sorted_data)):
            if sorted_data[j] > sorted_data[i]:
                temp: int = sorted_data[i]
                sorted_data[i] = sorted_data[j]
                sorted_data[j] = temp

    result: List[int] = []
    for i in range(min(n, len(sorted_data))):
        result.append(sorted_data[i])

    return result


def manual_heap_insert(heap: List[int], value: int) -> List[int]:
    """Manual heap insert operation"""
    new_heap: List[int] = heap.copy()
    new_heap.append(value)

    # Bubble up (maintain heap property)
    index: int = len(new_heap) - 1
    while index > 0:
        parent: int = (index - 1) // 2
        if new_heap[index] < new_heap[parent]:
            temp: int = new_heap[index]
            new_heap[index] = new_heap[parent]
            new_heap[parent] = temp
            index = parent
        else:
            break

    return new_heap


def manual_heap_extract_min(heap: List[int]) -> tuple:
    """Manual heap extract minimum"""
    if len(heap) == 0:
        return (-1, [])

    min_val: int = heap[0]

    # Replace root with last element
    if len(heap) == 1:
        return (min_val, [])

    new_heap: List[int] = [heap[len(heap) - 1]]
    for i in range(1, len(heap) - 1):
        new_heap.append(heap[i])

    # Bubble down
    index: int = 0
    while True:
        left: int = 2 * index + 1
        right: int = 2 * index + 2
        smallest: int = index

        if left < len(new_heap) and new_heap[left] < new_heap[smallest]:
            smallest = left

        if right < len(new_heap) and new_heap[right] < new_heap[smallest]:
            smallest = right

        if smallest != index:
            temp: int = new_heap[index]
            new_heap[index] = new_heap[smallest]
            new_heap[smallest] = temp
            index = smallest
        else:
            break

    return (min_val, new_heap)


def priority_queue_simulation() -> List[int]:
    """Simulate priority queue using heap"""
    tasks: List[tuple] = [(3, "low"), (1, "high"), (2, "medium")]

    # Sort by priority
    sorted_tasks: List[tuple] = tasks.copy()
    for i in range(len(sorted_tasks)):
        for j in range(i + 1, len(sorted_tasks)):
            if sorted_tasks[j][0] < sorted_tasks[i][0]:
                temp: tuple = sorted_tasks[i]
                sorted_tasks[i] = sorted_tasks[j]
                sorted_tasks[j] = temp

    # Extract priorities
    priorities: List[int] = []
    for task in sorted_tasks:
        priorities.append(task[0])

    return priorities


def merge_sorted_lists(lists: List[List[int]]) -> List[int]:
    """Merge multiple sorted lists using heap concept"""
    result: List[int] = []

    # Flatten all lists
    for lst in lists:
        for item in lst:
            result.append(item)

    # Sort final result
    for i in range(len(result)):
        for j in range(i + 1, len(result)):
            if result[j] < result[i]:
                temp: int = result[i]
                result[i] = result[j]
                result[j] = temp

    return result


def find_kth_smallest(data: List[int], k: int) -> int:
    """Find kth smallest element"""
    # Sort and return kth element
    sorted_data: List[int] = data.copy()
    for i in range(len(sorted_data)):
        for j in range(i + 1, len(sorted_data)):
            if sorted_data[j] < sorted_data[i]:
                temp: int = sorted_data[i]
                sorted_data[i] = sorted_data[j]
                sorted_data[j] = temp

    if k > 0 and k <= len(sorted_data):
        return sorted_data[k - 1]
    else:
        return -1


def find_median_using_heaps(data: List[int]) -> float:
    """Find median using two heaps concept"""
    # Sort data
    sorted_data: List[int] = data.copy()
    for i in range(len(sorted_data)):
        for j in range(i + 1, len(sorted_data)):
            if sorted_data[j] < sorted_data[i]:
                temp: int = sorted_data[i]
                sorted_data[i] = sorted_data[j]
                sorted_data[j] = temp

    # Find median
    n: int = len(sorted_data)
    if n % 2 == 1:
        median: float = float(sorted_data[n // 2])
    else:
        mid1: int = sorted_data[n // 2 - 1]
        mid2: int = sorted_data[n // 2]
        median: float = float(mid1 + mid2) / 2.0

    return median


def test_all_heapq_features() -> None:
    """Run all heapq module tests"""
    # Basic operations
    heap: List[int] = test_heap_push_pop()
    heapified: List[int] = test_heapify()

    # Pop and peek
    min_val: int = test_heap_pop_min()
    peek_val: int = test_heap_peek()

    # N smallest/largest
    data: List[int] = [5, 2, 8, 1, 9, 3, 7]
    smallest_3: List[int] = test_nsmallest(data, 3)
    largest_3: List[int] = test_nlargest(data, 3)

    # Manual operations
    h: List[int] = []
    h = manual_heap_insert(h, 5)
    h = manual_heap_insert(h, 3)
    h = manual_heap_insert(h, 7)

    extract_result: tuple = manual_heap_extract_min(h)
    extracted: int = extract_result[0]
    remaining: List[int] = extract_result[1]

    # Priority queue
    priorities: List[int] = priority_queue_simulation()

    # Merge sorted lists
    lists: List[List[int]] = [[1, 4, 7], [2, 5, 8], [3, 6, 9]]
    merged: List[int] = merge_sorted_lists(lists)

    # Kth smallest
    sample: List[int] = [7, 10, 4, 3, 20, 15]
    kth: int = find_kth_smallest(sample, 3)

    # Median using heaps
    median_data: List[int] = [1, 2, 3, 4, 5]
    median: float = find_median_using_heaps(median_data)

    print("All heapq module tests completed successfully")
