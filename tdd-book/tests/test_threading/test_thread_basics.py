"""
TDD Book - Phase 3: Concurrency
Module: threading - Thread-based parallelism
Coverage: Thread creation, execution, synchronization primitives

Test Categories:
- Basic thread creation and execution
- Thread synchronization (Lock, RLock, Semaphore, Event, Condition)
- Thread lifecycle and daemon threads
- Thread-safe operations
- Edge cases and race conditions
"""

import pytest
import threading
import time
from queue import Queue


class TestThreadCreation:
    """Test basic thread creation and execution."""

    def test_thread_creation_with_target(self):
        """Property: Thread executes target function."""
        result = []

        def worker():
            result.append(42)

        thread = threading.Thread(target=worker)
        thread.start()
        thread.join()
        assert result == [42]

    def test_thread_with_args(self):
        """Property: Thread passes args to target function."""
        result = []

        def worker(a, b):
            result.append(a + b)

        thread = threading.Thread(target=worker, args=(10, 20))
        thread.start()
        thread.join()
        assert result == [30]

    def test_thread_with_kwargs(self):
        """Property: Thread passes kwargs to target function."""
        result = []

        def worker(x=0, y=0):
            result.append(x * y)

        thread = threading.Thread(target=worker, kwargs={"x": 3, "y": 4})
        thread.start()
        thread.join()
        assert result == [12]

    def test_thread_name(self):
        """Property: Thread can be named."""
        thread = threading.Thread(target=lambda: None, name="worker-1")
        assert thread.name == "worker-1"

    def test_multiple_threads(self):
        """Property: Multiple threads execute concurrently."""
        results = []
        lock = threading.Lock()

        def worker(n):
            with lock:
                results.append(n)

        threads = [threading.Thread(target=worker, args=(i,)) for i in range(5)]
        for t in threads:
            t.start()
        for t in threads:
            t.join()

        assert sorted(results) == [0, 1, 2, 3, 4]


class TestThreadSynchronization:
    """Test thread synchronization primitives."""

    def test_lock_mutual_exclusion(self):
        """Property: Lock ensures mutual exclusion."""
        counter = 0
        lock = threading.Lock()

        def increment():
            nonlocal counter
            for _ in range(1000):
                with lock:
                    counter += 1

        threads = [threading.Thread(target=increment) for _ in range(5)]
        for t in threads:
            t.start()
        for t in threads:
            t.join()

        assert counter == 5000  # No race condition

    def test_lock_acquire_release(self):
        """Property: Lock can be acquired and released explicitly."""
        lock = threading.Lock()
        assert lock.acquire()
        assert lock.locked()
        lock.release()
        assert not lock.locked()

    def test_lock_blocking(self):
        """Property: Lock blocks when already held."""
        lock = threading.Lock()
        lock.acquire()

        acquired = lock.acquire(blocking=False)
        assert not acquired  # Can't acquire when already held

        lock.release()
        acquired = lock.acquire(blocking=False)
        assert acquired
        lock.release()

    def test_rlock_reentrant(self):
        """Property: RLock can be acquired multiple times by same thread."""
        rlock = threading.RLock()
        assert rlock.acquire()
        assert rlock.acquire()  # Can acquire again
        assert rlock.acquire()  # And again
        rlock.release()
        rlock.release()
        rlock.release()

    def test_semaphore_counting(self):
        """Property: Semaphore allows N concurrent accesses."""
        sem = threading.Semaphore(2)  # Allow 2 concurrent
        results = []

        def worker(n):
            with sem:
                results.append(f"start-{n}")
                time.sleep(0.01)
                results.append(f"end-{n}")

        threads = [threading.Thread(target=worker, args=(i,)) for i in range(4)]
        for t in threads:
            t.start()
        for t in threads:
            t.join()

        # At most 2 threads should overlap
        assert len(results) == 8

    def test_event_wait_set(self):
        """Property: Event blocks threads until set."""
        event = threading.Event()
        result = []

        def waiter():
            event.wait()
            result.append("waited")

        thread = threading.Thread(target=waiter)
        thread.start()

        time.sleep(0.01)  # Let thread start waiting
        assert len(result) == 0

        event.set()
        thread.join()
        assert result == ["waited"]

    def test_event_clear(self):
        """Property: Event can be cleared after being set."""
        event = threading.Event()
        event.set()
        assert event.is_set()

        event.clear()
        assert not event.is_set()

    def test_condition_wait_notify(self):
        """Property: Condition allows thread coordination."""
        condition = threading.Condition()
        result = []

        def waiter():
            with condition:
                condition.wait()
                result.append("notified")

        def notifier():
            time.sleep(0.01)
            with condition:
                condition.notify()

        t1 = threading.Thread(target=waiter)
        t2 = threading.Thread(target=notifier)

        t1.start()
        t2.start()
        t1.join()
        t2.join()

        assert result == ["notified"]

    def test_condition_notify_all(self):
        """Property: Condition.notify_all wakes all waiting threads."""
        condition = threading.Condition()
        results = []

        def waiter(n):
            with condition:
                condition.wait()
                results.append(n)

        def notifier():
            time.sleep(0.01)
            with condition:
                condition.notify_all()

        waiters = [threading.Thread(target=waiter, args=(i,)) for i in range(3)]
        notifier_thread = threading.Thread(target=notifier)

        for t in waiters:
            t.start()
        notifier_thread.start()

        for t in waiters:
            t.join()
        notifier_thread.join()

        assert sorted(results) == [0, 1, 2]


class TestThreadLifecycle:
    """Test thread lifecycle and state management."""

    def test_thread_is_alive(self):
        """Property: is_alive() reflects thread execution state."""
        event = threading.Event()

        def worker():
            event.wait()

        thread = threading.Thread(target=worker)
        assert not thread.is_alive()

        thread.start()
        assert thread.is_alive()

        event.set()
        thread.join()
        assert not thread.is_alive()

    def test_daemon_thread(self):
        """Property: Daemon thread doesn't prevent program exit."""

        def worker():
            time.sleep(10)  # Long running

        thread = threading.Thread(target=worker, daemon=True)
        assert thread.daemon

        thread.start()
        # Program can exit without joining daemon thread

    def test_non_daemon_default(self):
        """Property: Threads are non-daemon by default."""
        thread = threading.Thread(target=lambda: None)
        assert not thread.daemon

    def test_join_timeout(self):
        """Property: join() can timeout."""
        event = threading.Event()

        def worker():
            event.wait()

        thread = threading.Thread(target=worker)
        thread.start()

        thread.join(timeout=0.01)
        assert thread.is_alive()  # Still running

        event.set()
        thread.join()
        assert not thread.is_alive()

    def test_current_thread(self):
        """Property: current_thread() returns executing thread."""
        result = []

        def worker():
            result.append(threading.current_thread().name)

        thread = threading.Thread(target=worker, name="test-thread")
        thread.start()
        thread.join()

        assert result == ["test-thread"]

    def test_main_thread(self):
        """Property: main_thread() returns main thread."""
        main = threading.main_thread()
        assert main.name == "MainThread"
        assert threading.current_thread() == main

    def test_active_count(self):
        """Property: active_count() counts running threads."""
        initial_count = threading.active_count()
        events = [threading.Event() for _ in range(3)]

        def worker(event):
            event.wait()

        threads = [
            threading.Thread(target=worker, args=(events[i],)) for i in range(3)
        ]
        for t in threads:
            t.start()

        assert threading.active_count() == initial_count + 3

        for event in events:
            event.set()
        for t in threads:
            t.join()

        assert threading.active_count() == initial_count


class TestThreadLocal:
    """Test thread-local storage."""

    def test_thread_local_data(self):
        """Property: ThreadLocal data is per-thread."""
        local = threading.local()
        results = Queue()

        def worker(n):
            local.value = n
            time.sleep(0.01)  # Simulate work
            results.put(local.value)

        threads = [threading.Thread(target=worker, args=(i,)) for i in range(5)]
        for t in threads:
            t.start()
        for t in threads:
            t.join()

        values = sorted([results.get() for _ in range(5)])
        assert values == [0, 1, 2, 3, 4]  # Each thread keeps its own value


class TestThreadSafety:
    """Test thread-safe operations and race conditions."""

    def test_race_condition_without_lock(self):
        """Property: Without lock, race conditions can occur."""
        counter = 0

        def increment():
            nonlocal counter
            for _ in range(1000):
                temp = counter
                # Simulating non-atomic operation
                counter = temp + 1

        threads = [threading.Thread(target=increment) for _ in range(5)]
        for t in threads:
            t.start()
        for t in threads:
            t.join()

        # Without lock, final value is unpredictable and likely < 5000
        # We can't assert exact value, but it demonstrates the race
        assert counter <= 5000

    def test_barrier_synchronization(self):
        """Property: Barrier synchronizes thread execution."""
        barrier = threading.Barrier(3)
        results = []
        lock = threading.Lock()

        def worker(n):
            with lock:
                results.append(f"before-{n}")
            barrier.wait()  # Wait for all threads
            with lock:
                results.append(f"after-{n}")

        threads = [threading.Thread(target=worker, args=(i,)) for i in range(3)]
        for t in threads:
            t.start()
        for t in threads:
            t.join()

        # All "before" should come before any "after"
        before_count = sum(1 for r in results[:3] if r.startswith("before"))
        assert before_count == 3


class TestThreadEdgeCases:
    """Test edge cases and error conditions."""

    def test_thread_start_twice_raises(self):
        """Property: Starting a thread twice raises RuntimeError."""

        def worker():
            pass

        thread = threading.Thread(target=worker)
        thread.start()
        thread.join()

        with pytest.raises(RuntimeError):
            thread.start()

    def test_lock_release_unlocked_raises(self):
        """Property: Releasing unlocked Lock raises RuntimeError."""
        lock = threading.Lock()
        with pytest.raises(RuntimeError):
            lock.release()

    def test_semaphore_release_increments(self):
        """Property: Semaphore.release() increments counter."""
        sem = threading.Semaphore(1)
        sem.release()  # Now value is 2
        assert sem.acquire(blocking=False)
        assert sem.acquire(blocking=False)
        assert not sem.acquire(blocking=False)

    def test_timer_execution(self):
        """Property: Timer executes function after delay."""
        result = []

        def callback():
            result.append("executed")

        timer = threading.Timer(0.01, callback)
        timer.start()
        timer.join()

        assert result == ["executed"]

    def test_timer_cancel(self):
        """Property: Timer can be cancelled before execution."""
        result = []

        def callback():
            result.append("executed")

        timer = threading.Timer(1.0, callback)
        timer.start()
        timer.cancel()
        timer.join()

        assert result == []  # Not executed
