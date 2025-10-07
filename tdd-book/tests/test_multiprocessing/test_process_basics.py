"""
TDD Book - Phase 3: Concurrency
Module: multiprocessing - Process-based parallelism
Coverage: Process creation, IPC (Queue, Pipe, Value, Array), synchronization

Test Categories:
- Basic process creation and execution
- Inter-process communication (Queue, Pipe)
- Shared memory (Value, Array)
- Process pools and parallel execution
- Process synchronization primitives
- Edge cases and platform differences
"""

import pytest
import multiprocessing as mp
import time
import os


# Module-level functions for Pool tests (must be picklable)
def _square(x):
    return x * x


def _add(a, b):
    return a + b


def _multiply(a, b):
    return a * b


def _double(x):
    return x * 2


def _identity(x):
    return x


class TestProcessCreation:
    """Test basic process creation and execution."""

    def test_process_creation_with_target(self):
        """Property: Process executes target function."""
        result = mp.Queue()

        def worker(q):
            q.put(42)

        process = mp.Process(target=worker, args=(result,))
        process.start()
        process.join()

        assert result.get() == 42

    def test_process_with_args(self):
        """Property: Process passes args to target."""
        result = mp.Queue()

        def worker(q, a, b):
            q.put(a + b)

        process = mp.Process(target=worker, args=(result, 10, 20))
        process.start()
        process.join()

        assert result.get() == 30

    def test_process_with_kwargs(self):
        """Property: Process passes kwargs to target."""
        result = mp.Queue()

        def worker(q, x=0, y=0):
            q.put(x * y)

        process = mp.Process(target=worker, args=(result,), kwargs={"x": 3, "y": 4})
        process.start()
        process.join()

        assert result.get() == 12

    def test_process_name(self):
        """Property: Process can be named."""
        process = mp.Process(target=lambda: None, name="worker-1")
        assert process.name == "worker-1"

    def test_multiple_processes(self):
        """Property: Multiple processes execute independently."""
        result = mp.Queue()

        def worker(q, n):
            q.put(n * 2)

        processes = [mp.Process(target=worker, args=(result, i)) for i in range(5)]
        for p in processes:
            p.start()
        for p in processes:
            p.join()

        results = sorted([result.get() for _ in range(5)])
        assert results == [0, 2, 4, 6, 8]

    def test_process_pid(self):
        """Property: Process has unique PID."""
        result = mp.Queue()

        def worker(q):
            q.put(os.getpid())

        process = mp.Process(target=worker, args=(result,))
        process.start()
        process.join()

        child_pid = result.get()
        assert child_pid != os.getpid()  # Different from parent


class TestQueueIPC:
    """Test Queue for inter-process communication."""

    def test_queue_put_get(self):
        """Property: Queue transfers data between processes."""
        q = mp.Queue()

        def producer(queue):
            for i in range(5):
                queue.put(i)

        def consumer(queue, result):
            items = [queue.get() for _ in range(5)]
            result.put(items)

        result = mp.Queue()
        p1 = mp.Process(target=producer, args=(q,))
        p2 = mp.Process(target=consumer, args=(q, result))

        p1.start()
        p1.join()
        p2.start()
        p2.join()

        assert result.get() == [0, 1, 2, 3, 4]

    def test_queue_multiple_producers(self):
        """Property: Queue handles multiple producers."""
        q = mp.Queue()

        def producer(queue, start):
            for i in range(start, start + 3):
                queue.put(i)

        processes = [mp.Process(target=producer, args=(q, i * 10)) for i in range(3)]
        for p in processes:
            p.start()
        for p in processes:
            p.join()

        results = sorted([q.get() for _ in range(9)])
        assert results == [0, 1, 2, 10, 11, 12, 20, 21, 22]

    def test_queue_empty_exception(self):
        """Property: Queue.get_nowait() raises Empty when empty."""
        q = mp.Queue()

        with pytest.raises(Exception):  # queue.Empty
            q.get_nowait()

    def test_queue_qsize_approximate(self):
        """Property: Queue.qsize() returns approximate size."""
        q = mp.Queue()

        def producer(queue):
            for i in range(10):
                queue.put(i)

        p = mp.Process(target=producer, args=(q,))
        p.start()
        p.join()

        # qsize is approximate but should be around 10
        assert 8 <= q.qsize() <= 10


class TestPipeIPC:
    """Test Pipe for bidirectional communication."""

    def test_pipe_bidirectional(self):
        """Property: Pipe enables bidirectional communication."""
        parent_conn, child_conn = mp.Pipe()

        def child(conn):
            data = conn.recv()
            conn.send(data * 2)
            conn.close()

        p = mp.Process(target=child, args=(child_conn,))
        p.start()

        parent_conn.send(10)
        result = parent_conn.recv()
        p.join()

        assert result == 20

    def test_pipe_multiple_messages(self):
        """Property: Pipe can transfer multiple messages."""
        parent_conn, child_conn = mp.Pipe()

        def child(conn):
            for _ in range(3):
                data = conn.recv()
                conn.send(data + 1)
            conn.close()

        p = mp.Process(target=child, args=(child_conn,))
        p.start()

        results = []
        for i in range(3):
            parent_conn.send(i)
            results.append(parent_conn.recv())
        p.join()

        assert results == [1, 2, 3]

    def test_pipe_duplex_false(self):
        """Property: Pipe(duplex=False) is one-way."""
        parent_conn, child_conn = mp.Pipe(duplex=False)

        def child(conn):
            conn.send(42)
            conn.close()

        p = mp.Process(target=child, args=(child_conn,))
        p.start()

        result = parent_conn.recv()
        p.join()

        assert result == 42


class TestSharedMemory:
    """Test shared memory objects (Value, Array)."""

    def test_value_shared(self):
        """Property: Value shares data between processes."""
        counter = mp.Value("i", 0)

        def increment(val):
            with val.get_lock():
                val.value += 1

        processes = [mp.Process(target=increment, args=(counter,)) for _ in range(10)]
        for p in processes:
            p.start()
        for p in processes:
            p.join()

        assert counter.value == 10

    def test_value_types(self):
        """Property: Value supports various types."""
        int_val = mp.Value("i", 5)
        float_val = mp.Value("d", 3.14)

        assert int_val.value == 5
        assert abs(float_val.value - 3.14) < 0.001

    def test_array_shared(self):
        """Property: Array shares data between processes."""
        arr = mp.Array("i", [0, 0, 0, 0, 0])

        def increment_all(array):
            for i in range(len(array)):
                array[i] += 1

        processes = [mp.Process(target=increment_all, args=(arr,)) for _ in range(3)]
        for p in processes:
            p.start()
        for p in processes:
            p.join()

        assert list(arr) == [3, 3, 3, 3, 3]

    def test_array_with_lock(self):
        """Property: Array provides lock for synchronization."""
        arr = mp.Array("i", [0] * 5, lock=True)

        def safe_increment(array, index):
            with array.get_lock():
                array[index] += 1

        processes = []
        for i in range(5):
            for _ in range(10):
                p = mp.Process(target=safe_increment, args=(arr, i))
                processes.append(p)

        for p in processes:
            p.start()
        for p in processes:
            p.join()

        assert list(arr) == [10, 10, 10, 10, 10]


class TestProcessPool:
    """Test Process pool for parallel execution."""

    def test_pool_map(self):
        """Property: Pool.map distributes work across processes."""
        with mp.Pool(processes=2) as pool:
            results = pool.map(_square, [1, 2, 3, 4, 5])

        assert results == [1, 4, 9, 16, 25]

    def test_pool_apply(self):
        """Property: Pool.apply executes function in worker."""
        with mp.Pool(processes=2) as pool:
            result = pool.apply(_add, (10, 20))

        assert result == 30

    def test_pool_starmap(self):
        """Property: Pool.starmap unpacks argument tuples."""
        with mp.Pool(processes=2) as pool:
            results = pool.starmap(_multiply, [(2, 3), (4, 5), (6, 7)])

        assert results == [6, 20, 42]

    def test_pool_imap(self):
        """Property: Pool.imap returns iterator of results."""
        with mp.Pool(processes=2) as pool:
            results = list(pool.imap(_double, range(5)))

        assert results == [0, 2, 4, 6, 8]

    def test_pool_context_manager(self):
        """Property: Pool works as context manager."""
        with mp.Pool(processes=2) as pool:
            results = pool.map(_identity, [1, 2, 3])

        assert results == [1, 2, 3]


class TestProcessLifecycle:
    """Test process lifecycle and state management."""

    def test_process_is_alive(self):
        """Property: is_alive() reflects process state."""
        event = mp.Event()

        def worker(e):
            e.wait()

        process = mp.Process(target=worker, args=(event,))
        assert not process.is_alive()

        process.start()
        assert process.is_alive()

        event.set()
        process.join()
        assert not process.is_alive()

    def test_process_exitcode(self):
        """Property: exitcode reflects process termination status."""

        def worker():
            pass

        process = mp.Process(target=worker)
        assert process.exitcode is None

        process.start()
        process.join()
        assert process.exitcode == 0

    def test_daemon_process(self):
        """Property: Daemon process doesn't block program exit."""

        def worker():
            time.sleep(10)

        process = mp.Process(target=worker, daemon=True)
        assert process.daemon

        process.start()
        # Program can exit without joining daemon

    def test_join_timeout(self):
        """Property: join() can timeout."""
        event = mp.Event()

        def worker(e):
            e.wait()

        process = mp.Process(target=worker, args=(event,))
        process.start()

        process.join(timeout=0.01)
        assert process.is_alive()

        event.set()
        process.join()
        assert not process.is_alive()

    def test_terminate(self):
        """Property: terminate() stops process."""
        event = mp.Event()

        def worker(e):
            e.wait()

        process = mp.Process(target=worker, args=(event,))
        process.start()
        assert process.is_alive()

        process.terminate()
        process.join(timeout=1)
        assert not process.is_alive()


class TestProcessSynchronization:
    """Test process synchronization primitives."""

    def test_lock_mutual_exclusion(self):
        """Property: Lock ensures mutual exclusion across processes."""
        counter = mp.Value("i", 0)
        lock = mp.Lock()

        def increment(val, l):
            for _ in range(100):
                with l:
                    val.value += 1

        processes = [mp.Process(target=increment, args=(counter, lock)) for _ in range(5)]
        for p in processes:
            p.start()
        for p in processes:
            p.join()

        assert counter.value == 500

    def test_semaphore(self):
        """Property: Semaphore limits concurrent access."""
        sem = mp.Semaphore(2)
        result = mp.Queue()

        def worker(s, q, n):
            with s:
                q.put(f"start-{n}")
                time.sleep(0.01)
                q.put(f"end-{n}")

        processes = [mp.Process(target=worker, args=(sem, result, i)) for i in range(4)]
        for p in processes:
            p.start()
        for p in processes:
            p.join()

        items = [result.get() for _ in range(8)]
        assert len(items) == 8

    def test_event(self):
        """Property: Event coordinates processes."""
        event = mp.Event()
        result = mp.Queue()

        def waiter(e, q):
            e.wait()
            q.put("done")

        def setter(e):
            time.sleep(0.01)
            e.set()

        p1 = mp.Process(target=waiter, args=(event, result))
        p2 = mp.Process(target=setter, args=(event,))

        p1.start()
        p2.start()
        p1.join()
        p2.join()

        assert result.get() == "done"


class TestProcessEdgeCases:
    """Test edge cases and error conditions."""

    def test_process_start_twice_raises(self):
        """Property: Starting process twice raises error."""

        def worker():
            pass

        process = mp.Process(target=worker)
        process.start()
        process.join()

        with pytest.raises(AssertionError):
            process.start()

    def test_cpu_count(self):
        """Property: cpu_count() returns number of CPUs."""
        count = mp.cpu_count()
        assert count >= 1
        assert isinstance(count, int)

    def test_current_process(self):
        """Property: current_process() returns current process."""
        result = mp.Queue()

        def worker(q):
            proc = mp.current_process()
            q.put(proc.name)

        process = mp.Process(target=worker, args=(result,), name="test-process")
        process.start()
        process.join()

        assert result.get() == "test-process"

    def test_active_children(self):
        """Property: active_children() returns live child processes."""
        event = mp.Event()

        def worker(e):
            e.wait()

        processes = [mp.Process(target=worker, args=(event,)) for _ in range(3)]
        for p in processes:
            p.start()

        # Clean up zombie processes
        mp.active_children()

        assert len(mp.active_children()) >= 3

        event.set()
        for p in processes:
            p.join()

    def test_manager_list(self):
        """Property: Manager provides shared list."""
        with mp.Manager() as manager:
            shared_list = manager.list([1, 2, 3])

            def append_item(lst, item):
                lst.append(item)

            p = mp.Process(target=append_item, args=(shared_list, 4))
            p.start()
            p.join()

            assert list(shared_list) == [1, 2, 3, 4]

    def test_manager_dict(self):
        """Property: Manager provides shared dict."""
        with mp.Manager() as manager:
            shared_dict = manager.dict({"a": 1})

            def update_dict(d):
                d["b"] = 2

            p = mp.Process(target=update_dict, args=(shared_dict,))
            p.start()
            p.join()

            assert dict(shared_dict) == {"a": 1, "b": 2}
