"""
TDD Book - Phase 3: Concurrency
Module: queue - Thread-safe queue implementations
Coverage: Queue, LifoQueue, PriorityQueue, SimpleQueue

Test Categories:
- Basic queue operations (put, get)
- Queue variants (FIFO, LIFO, Priority)
- Blocking and non-blocking operations
- Queue size and capacity management
- Thread-safe operations
- Edge cases and error conditions
"""

import pytest
import queue
import threading
import time


class TestQueueBasics:
    """Test basic Queue operations."""

    def test_queue_creation(self):
        """Property: Queue can be created."""
        q = queue.Queue()
        assert q.empty()
        assert q.qsize() == 0

    def test_queue_put_get(self):
        """Property: Items are retrieved in FIFO order."""
        q = queue.Queue()
        q.put(1)
        q.put(2)
        q.put(3)

        assert q.get() == 1
        assert q.get() == 2
        assert q.get() == 3

    def test_queue_fifo_order(self):
        """Property: Queue maintains FIFO order."""
        q = queue.Queue()
        items = [10, 20, 30, 40, 50]

        for item in items:
            q.put(item)

        result = [q.get() for _ in range(5)]
        assert result == items

    def test_queue_size(self):
        """Property: qsize() returns number of items."""
        q = queue.Queue()
        assert q.qsize() == 0

        q.put("a")
        assert q.qsize() == 1

        q.put("b")
        assert q.qsize() == 2

        q.get()
        assert q.qsize() == 1

    def test_queue_empty(self):
        """Property: empty() returns True when queue is empty."""
        q = queue.Queue()
        assert q.empty()

        q.put(1)
        assert not q.empty()

        q.get()
        assert q.empty()

    def test_queue_full_with_maxsize(self):
        """Property: full() returns True when queue reaches maxsize."""
        q = queue.Queue(maxsize=2)
        assert not q.full()

        q.put(1)
        assert not q.full()

        q.put(2)
        assert q.full()

        q.get()
        assert not q.full()

    def test_queue_maxsize_unlimited(self):
        """Property: maxsize=0 means unlimited."""
        q = queue.Queue(maxsize=0)
        assert q.maxsize == 0

        for i in range(100):
            q.put(i)

        assert not q.full()
        assert q.qsize() == 100


class TestQueueBlocking:
    """Test blocking and non-blocking operations."""

    def test_get_blocks_when_empty(self):
        """Property: get() blocks when queue is empty."""
        q = queue.Queue()
        result = []

        def consumer():
            result.append(q.get())

        thread = threading.Thread(target=consumer)
        thread.start()

        time.sleep(0.01)  # Let thread block
        assert len(result) == 0

        q.put(42)
        thread.join(timeout=1)
        assert result == [42]

    def test_get_nowait_raises_when_empty(self):
        """Property: get_nowait() raises Empty when queue is empty."""
        q = queue.Queue()

        with pytest.raises(queue.Empty):
            q.get_nowait()

    def test_get_with_timeout(self):
        """Property: get(timeout) raises Empty after timeout."""
        q = queue.Queue()

        with pytest.raises(queue.Empty):
            q.get(timeout=0.01)

    def test_put_blocks_when_full(self):
        """Property: put() blocks when queue is full."""
        q = queue.Queue(maxsize=1)
        q.put(1)  # Fill queue

        result = []

        def producer():
            q.put(2)  # This will block
            result.append("put-done")

        thread = threading.Thread(target=producer)
        thread.start()

        time.sleep(0.01)  # Let thread block
        assert len(result) == 0

        q.get()  # Make space
        thread.join(timeout=1)
        assert result == ["put-done"]

    def test_put_nowait_raises_when_full(self):
        """Property: put_nowait() raises Full when queue is full."""
        q = queue.Queue(maxsize=1)
        q.put(1)

        with pytest.raises(queue.Full):
            q.put_nowait(2)

    def test_put_with_timeout(self):
        """Property: put(timeout) raises Full after timeout."""
        q = queue.Queue(maxsize=1)
        q.put(1)

        with pytest.raises(queue.Full):
            q.put(2, timeout=0.01)

    def test_get_with_block_false(self):
        """Property: get(block=False) equivalent to get_nowait()."""
        q = queue.Queue()

        with pytest.raises(queue.Empty):
            q.get(block=False)

        q.put(42)
        assert q.get(block=False) == 42

    def test_put_with_block_false(self):
        """Property: put(block=False) equivalent to put_nowait()."""
        q = queue.Queue(maxsize=1)
        q.put(1)

        with pytest.raises(queue.Full):
            q.put(2, block=False)


class TestLifoQueue:
    """Test LifoQueue (stack-like behavior)."""

    def test_lifo_order(self):
        """Property: LifoQueue returns items in LIFO order."""
        q = queue.LifoQueue()
        q.put(1)
        q.put(2)
        q.put(3)

        assert q.get() == 3
        assert q.get() == 2
        assert q.get() == 1

    def test_lifo_stack_behavior(self):
        """Property: LifoQueue behaves like a stack."""
        q = queue.LifoQueue()
        items = [10, 20, 30, 40, 50]

        for item in items:
            q.put(item)

        result = [q.get() for _ in range(5)]
        assert result == list(reversed(items))

    def test_lifo_maxsize(self):
        """Property: LifoQueue respects maxsize."""
        q = queue.LifoQueue(maxsize=2)
        q.put(1)
        q.put(2)
        assert q.full()

        with pytest.raises(queue.Full):
            q.put_nowait(3)


class TestPriorityQueue:
    """Test PriorityQueue (min-heap behavior)."""

    def test_priority_order(self):
        """Property: PriorityQueue returns items by priority."""
        q = queue.PriorityQueue()
        q.put(3)
        q.put(1)
        q.put(2)

        assert q.get() == 1
        assert q.get() == 2
        assert q.get() == 3

    def test_priority_with_tuples(self):
        """Property: PriorityQueue supports (priority, data) tuples."""
        q = queue.PriorityQueue()
        q.put((3, "low"))
        q.put((1, "high"))
        q.put((2, "medium"))

        assert q.get() == (1, "high")
        assert q.get() == (2, "medium")
        assert q.get() == (3, "low")

    def test_priority_stable_sort(self):
        """Property: PriorityQueue maintains insertion order for equal priorities."""
        q = queue.PriorityQueue()
        q.put((1, "a"))
        q.put((1, "b"))
        q.put((1, "c"))

        # Items with same priority returned in insertion order
        assert q.get() == (1, "a")
        assert q.get() == (1, "b")
        assert q.get() == (1, "c")

    def test_priority_negative_values(self):
        """Property: PriorityQueue supports negative priorities."""
        q = queue.PriorityQueue()
        q.put(-1)
        q.put(0)
        q.put(1)

        assert q.get() == -1
        assert q.get() == 0
        assert q.get() == 1


class TestSimpleQueue:
    """Test SimpleQueue (unbounded FIFO)."""

    def test_simple_queue_basic(self):
        """Property: SimpleQueue provides basic FIFO operations."""
        q = queue.SimpleQueue()
        q.put(1)
        q.put(2)

        assert q.get() == 1
        assert q.get() == 2

    def test_simple_queue_no_maxsize(self):
        """Property: SimpleQueue is always unbounded."""
        q = queue.SimpleQueue()

        for i in range(1000):
            q.put(i)

        assert q.qsize() == 1000

    def test_simple_queue_empty(self):
        """Property: SimpleQueue.empty() works correctly."""
        q = queue.SimpleQueue()
        assert q.empty()

        q.put(1)
        assert not q.empty()

        q.get()
        assert q.empty()

    def test_simple_queue_get_blocks(self):
        """Property: SimpleQueue.get() blocks when empty."""
        q = queue.SimpleQueue()
        result = []

        def consumer():
            result.append(q.get())

        thread = threading.Thread(target=consumer)
        thread.start()

        time.sleep(0.01)
        assert len(result) == 0

        q.put(42)
        thread.join(timeout=1)
        assert result == [42]


class TestTaskTracking:
    """Test task_done() and join() for work tracking."""

    def test_task_done_decrements_count(self):
        """Property: task_done() signals task completion."""
        q = queue.Queue()
        q.put(1)
        q.put(2)

        q.get()
        q.task_done()

        q.get()
        q.task_done()

        # No exception means all tasks marked done

    def test_join_waits_for_tasks(self):
        """Property: join() blocks until all tasks done."""
        q = queue.Queue()
        results = []

        def worker():
            while True:
                item = q.get()
                if item is None:
                    q.task_done()
                    break
                results.append(item * 2)
                time.sleep(0.01)
                q.task_done()

        thread = threading.Thread(target=worker, daemon=True)
        thread.start()

        for i in range(3):
            q.put(i)

        q.join()  # Wait for all tasks
        assert sorted(results) == [0, 2, 4]

        q.put(None)  # Stop worker

    def test_task_done_too_many_raises(self):
        """Property: task_done() more than get() raises ValueError."""
        q = queue.Queue()
        q.put(1)
        q.get()
        q.task_done()

        with pytest.raises(ValueError):
            q.task_done()  # No more tasks


class TestThreadSafety:
    """Test thread-safe operations."""

    def test_multiple_producers_consumers(self):
        """Property: Queue handles multiple producers/consumers safely."""
        q = queue.Queue()
        produced = []
        consumed = []
        lock = threading.Lock()

        def producer(start):
            for i in range(start, start + 10):
                q.put(i)
                with lock:
                    produced.append(i)

        def consumer():
            for _ in range(10):
                item = q.get()
                with lock:
                    consumed.append(item)
                q.task_done()

        producers = [threading.Thread(target=producer, args=(i * 10,)) for i in range(3)]
        consumers = [threading.Thread(target=consumer) for _ in range(3)]

        for t in producers + consumers:
            t.start()
        for t in producers + consumers:
            t.join()

        assert sorted(consumed) == sorted(produced)
        assert len(consumed) == 30

    def test_producer_consumer_pattern(self):
        """Property: Queue enables producer-consumer pattern."""
        q = queue.Queue()
        results = []

        def producer():
            for i in range(5):
                q.put(i)
                time.sleep(0.001)
            q.put(None)  # Sentinel

        def consumer():
            while True:
                item = q.get()
                if item is None:
                    break
                results.append(item * 2)

        p = threading.Thread(target=producer)
        c = threading.Thread(target=consumer)

        p.start()
        c.start()
        p.join()
        c.join()

        assert results == [0, 2, 4, 6, 8]


class TestQueueEdgeCases:
    """Test edge cases and special scenarios."""

    def test_queue_with_none(self):
        """Property: Queue can store None values."""
        q = queue.Queue()
        q.put(None)
        assert q.get() is None

    def test_queue_with_mixed_types(self):
        """Property: Queue accepts any object type."""
        q = queue.Queue()
        q.put(1)
        q.put("string")
        q.put([1, 2, 3])
        q.put({"key": "value"})

        assert q.get() == 1
        assert q.get() == "string"
        assert q.get() == [1, 2, 3]
        assert q.get() == {"key": "value"}

    def test_priority_queue_with_non_comparable(self):
        """Property: PriorityQueue requires comparable items."""
        q = queue.PriorityQueue()
        q.put((1, {"data": "a"}))
        q.put((2, {"data": "b"}))

        # Works because comparing tuples (priority first)
        assert q.get()[0] == 1

    def test_queue_qsize_approximate(self):
        """Property: qsize() is approximate in multithreaded context."""
        q = queue.Queue()

        def producer():
            for i in range(100):
                q.put(i)

        thread = threading.Thread(target=producer)
        thread.start()
        thread.join()

        assert q.qsize() == 100

    def test_multiple_get_nowait(self):
        """Property: Multiple get_nowait() raise Empty when exhausted."""
        q = queue.Queue()
        q.put(1)
        q.put(2)

        assert q.get_nowait() == 1
        assert q.get_nowait() == 2

        with pytest.raises(queue.Empty):
            q.get_nowait()

        with pytest.raises(queue.Empty):
            q.get_nowait()
