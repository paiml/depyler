"""
Comprehensive test of Python array and queue modules transpilation to Rust.

This example demonstrates how Depyler transpiles Python's array and queue modules
to their Rust equivalents.

Expected Rust mappings:
- array.array() -> Vec or fixed-size arrays
- queue.Queue -> VecDeque or channel-based queue
- queue.LifoQueue -> Vec (stack)
- queue.PriorityQueue -> BinaryHeap

Note: Manual implementations provided for learning.
"""

from typing import List, Optional


# ============================================================================
# ARRAY MODULE TESTS
# ============================================================================

def test_array_creation() -> List[int]:
    """Test creating array with type code"""
    # Python array.array('i', [1, 2, 3])
    # Simulated with list
    arr: List[int] = [1, 2, 3, 4, 5]

    return arr


def test_array_append() -> List[int]:
    """Test appending to array"""
    arr: List[int] = [1, 2, 3]
    arr.append(4)
    arr.append(5)

    return arr


def test_array_extend() -> List[int]:
    """Test extending array"""
    arr: List[int] = [1, 2, 3]
    extension: List[int] = [4, 5, 6]

    arr.extend(extension)

    return arr


def test_array_insert() -> List[int]:
    """Test inserting into array"""
    arr: List[int] = [1, 2, 4, 5]
    arr.insert(2, 3)

    return arr


def test_array_remove() -> List[int]:
    """Test removing from array"""
    arr: List[int] = [1, 2, 3, 4, 5]
    arr.remove(3)

    return arr


def test_array_pop() -> tuple:
    """Test popping from array"""
    arr: List[int] = [1, 2, 3, 4, 5]
    popped: int = arr.pop()

    return (popped, arr)


def test_array_index() -> int:
    """Test finding index in array"""
    arr: List[int] = [10, 20, 30, 40, 50]
    idx: int = arr.index(30)

    return idx


def test_array_count() -> int:
    """Test counting in array"""
    arr: List[int] = [1, 2, 2, 3, 2, 4]
    count: int = arr.count(2)

    return count


def test_array_reverse() -> List[int]:
    """Test reversing array"""
    arr: List[int] = [1, 2, 3, 4, 5]
    arr.reverse()

    return arr


def test_array_tolist() -> List[int]:
    """Test converting array to list"""
    arr: List[int] = [1, 2, 3, 4, 5]
    # In Python: arr.tolist(), but already a list
    return arr.copy()


# ============================================================================
# QUEUE MODULE TESTS
# ============================================================================

class SimpleQueue:
    """Simple FIFO queue implementation"""

    def __init__(self) -> None:
        self.items: List[int] = []

    def put(self, item: int) -> None:
        """Add item to queue"""
        self.items.append(item)

    def get(self) -> int:
        """Remove and return item from queue"""
        if len(self.items) == 0:
            return -1

        item: int = self.items[0]

        # Remove first element
        new_items: List[int] = []
        for i in range(1, len(self.items)):
            new_items.append(self.items[i])

        self.items = new_items

        return item

    def size(self) -> int:
        """Get queue size"""
        return len(self.items)

    def empty(self) -> bool:
        """Check if queue is empty"""
        return len(self.items) == 0


class SimpleStack:
    """Simple LIFO stack implementation"""

    def __init__(self) -> None:
        self.items: List[int] = []

    def push(self, item: int) -> None:
        """Push item onto stack"""
        self.items.append(item)

    def pop(self) -> int:
        """Pop item from stack"""
        if len(self.items) == 0:
            return -1

        return self.items.pop()

    def size(self) -> int:
        """Get stack size"""
        return len(self.items)

    def empty(self) -> bool:
        """Check if stack is empty"""
        return len(self.items) == 0

    def peek(self) -> int:
        """Peek at top item without removing"""
        if len(self.items) == 0:
            return -1

        return self.items[len(self.items) - 1]


def test_queue_fifo() -> List[int]:
    """Test FIFO queue operations"""
    q: SimpleQueue = SimpleQueue()

    # Enqueue
    q.put(1)
    q.put(2)
    q.put(3)

    # Dequeue
    results: List[int] = []
    while not q.empty():
        item: int = q.get()
        results.append(item)

    return results


def test_stack_lifo() -> List[int]:
    """Test LIFO stack operations"""
    s: SimpleStack = SimpleStack()

    # Push
    s.push(1)
    s.push(2)
    s.push(3)

    # Pop
    results: List[int] = []
    while not s.empty():
        item: int = s.pop()
        results.append(item)

    return results


def test_queue_size() -> int:
    """Test queue size tracking"""
    q: SimpleQueue = SimpleQueue()

    q.put(1)
    q.put(2)
    q.put(3)

    size: int = q.size()

    return size


def test_stack_peek() -> int:
    """Test stack peek operation"""
    s: SimpleStack = SimpleStack()

    s.push(1)
    s.push(2)
    s.push(3)

    top: int = s.peek()

    return top


# Priority Queue (min-heap implementation)
class SimplePriorityQueue:
    """Simple priority queue implementation"""

    def __init__(self) -> None:
        self.items: List[tuple] = []

    def put(self, priority: int, item: str) -> None:
        """Add item with priority"""
        self.items.append((priority, item))

        # Sort by priority (manual sort)
        for i in range(len(self.items)):
            for j in range(i + 1, len(self.items)):
                if self.items[j][0] < self.items[i][0]:
                    temp: tuple = self.items[i]
                    self.items[i] = self.items[j]
                    self.items[j] = temp

    def get(self) -> str:
        """Get highest priority item"""
        if len(self.items) == 0:
            return ""

        item: tuple = self.items[0]

        # Remove first element
        new_items: List[tuple] = []
        for i in range(1, len(self.items)):
            new_items.append(self.items[i])

        self.items = new_items

        return item[1]

    def empty(self) -> bool:
        """Check if queue is empty"""
        return len(self.items) == 0


def test_priority_queue() -> List[str]:
    """Test priority queue"""
    pq: SimplePriorityQueue = SimplePriorityQueue()

    # Add with priorities
    pq.put(3, "low")
    pq.put(1, "high")
    pq.put(2, "medium")

    # Get in priority order
    results: List[str] = []
    while not pq.empty():
        item: str = pq.get()
        results.append(item)

    return results


def test_circular_buffer(size: int) -> List[int]:
    """Test circular buffer implementation"""
    buffer: List[int] = []
    max_size: int = size

    values: List[int] = [1, 2, 3, 4, 5, 6, 7, 8]

    for val in values:
        buffer.append(val)

        # Remove oldest if over capacity
        if len(buffer) > max_size:
            # Remove first element
            new_buffer: List[int] = []
            for i in range(1, len(buffer)):
                new_buffer.append(buffer[i])
            buffer = new_buffer

    return buffer


def test_deque_simulation() -> List[int]:
    """Simulate double-ended queue"""
    deque: List[int] = []

    # Append right
    deque.append(1)
    deque.append(2)
    deque.append(3)

    # Append left (insert at 0)
    deque.insert(0, 0)

    # Pop right
    deque.pop()

    # Pop left (remove at 0)
    if len(deque) > 0:
        new_deque: List[int] = []
        for i in range(1, len(deque)):
            new_deque.append(deque[i])
        deque = new_deque

    return deque


def test_all_array_queue_features() -> None:
    """Run all array and queue tests"""
    # Array tests
    arr: List[int] = test_array_creation()
    appended: List[int] = test_array_append()
    extended: List[int] = test_array_extend()
    inserted: List[int] = test_array_insert()
    removed: List[int] = test_array_remove()
    pop_result: tuple = test_array_pop()
    idx: int = test_array_index()
    count: int = test_array_count()
    reversed_arr: List[int] = test_array_reverse()
    as_list: List[int] = test_array_tolist()

    # Queue tests
    fifo_result: List[int] = test_queue_fifo()
    lifo_result: List[int] = test_stack_lifo()
    size: int = test_queue_size()
    top: int = test_stack_peek()

    # Priority queue
    priority_result: List[str] = test_priority_queue()

    # Advanced structures
    circular: List[int] = test_circular_buffer(3)
    deque_result: List[int] = test_deque_simulation()

    print("All array and queue module tests completed successfully")
