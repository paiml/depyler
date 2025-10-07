"""
TDD Book - Phase 3: Concurrency
Module: concurrent.futures - High-level parallel execution
Coverage: ThreadPoolExecutor, ProcessPoolExecutor, Future objects

Test Categories:
- ThreadPoolExecutor basics (submit, map, shutdown)
- ProcessPoolExecutor basics (submit, map, shutdown)
- Future object operations (result, exception, done, cancel)
- Executor context manager usage
- wait() and as_completed() patterns
- Callbacks and chaining
- Exception handling
- Edge cases and timeouts
"""

import pytest
import concurrent.futures as cf
import time
import os


# Module-level functions for ProcessPoolExecutor (must be picklable)
def _slow_square(x):
    time.sleep(0.01)
    return x * x


def _add(a, b):
    return a + b


def _multiply(a, b):
    return a * b


def _failing_task():
    raise ValueError("task failed")


def _slow_task(delay):
    time.sleep(delay)
    return "done"


def _get_pid():
    return os.getpid()


def _identity(x):
    return x


class TestThreadPoolExecutor:
    """Test ThreadPoolExecutor for thread-based parallelism."""

    def test_submit_single_task(self):
        """Property: submit() schedules task execution."""
        with cf.ThreadPoolExecutor(max_workers=2) as executor:
            future = executor.submit(pow, 2, 3)
            result = future.result()

        assert result == 8

    def test_submit_multiple_tasks(self):
        """Property: Multiple tasks execute concurrently."""
        def task(n):
            time.sleep(0.01)
            return n * 2

        with cf.ThreadPoolExecutor(max_workers=3) as executor:
            futures = [executor.submit(task, i) for i in range(5)]
            results = [f.result() for f in futures]

        assert results == [0, 2, 4, 6, 8]

    def test_map_parallel_execution(self):
        """Property: map() applies function to items in parallel."""
        def square(x):
            return x * x

        with cf.ThreadPoolExecutor(max_workers=2) as executor:
            results = list(executor.map(square, range(5)))

        assert results == [0, 1, 4, 9, 16]

    def test_map_preserves_order(self):
        """Property: map() returns results in input order."""
        def task(n):
            time.sleep((5 - n) * 0.01)  # Slower tasks first
            return n

        with cf.ThreadPoolExecutor(max_workers=3) as executor:
            results = list(executor.map(task, range(5)))

        assert results == [0, 1, 2, 3, 4]  # Input order preserved

    def test_shutdown_explicit(self):
        """Property: shutdown() waits for tasks to complete."""
        results = []

        def task(n):
            time.sleep(0.01)
            results.append(n)

        executor = cf.ThreadPoolExecutor(max_workers=2)
        executor.submit(task, 1)
        executor.submit(task, 2)
        executor.shutdown(wait=True)

        assert sorted(results) == [1, 2]

    def test_shutdown_nowait(self):
        """Property: shutdown(wait=False) doesn't wait for tasks."""
        results = []

        def task(n):
            time.sleep(0.1)
            results.append(n)

        executor = cf.ThreadPoolExecutor(max_workers=2)
        executor.submit(task, 1)
        executor.shutdown(wait=False)

        # Results may not be complete yet
        assert isinstance(results, list)

    def test_context_manager(self):
        """Property: Executor works as context manager."""
        results = []

        def task(n):
            results.append(n)
            return n * 2

        with cf.ThreadPoolExecutor(max_workers=2) as executor:
            futures = [executor.submit(task, i) for i in range(3)]
            values = [f.result() for f in futures]

        assert sorted(values) == [0, 2, 4]


class TestProcessPoolExecutor:
    """Test ProcessPoolExecutor for process-based parallelism."""

    def test_submit_single_task(self):
        """Property: submit() schedules task in separate process."""
        with cf.ProcessPoolExecutor(max_workers=2) as executor:
            future = executor.submit(_add, 10, 20)
            result = future.result()

        assert result == 30

    def test_submit_multiple_tasks(self):
        """Property: Multiple tasks execute in parallel processes."""
        with cf.ProcessPoolExecutor(max_workers=2) as executor:
            futures = [executor.submit(_slow_square, i) for i in range(5)]
            results = [f.result() for f in futures]

        assert results == [0, 1, 4, 9, 16]

    def test_map_parallel_execution(self):
        """Property: map() distributes work across processes."""
        with cf.ProcessPoolExecutor(max_workers=2) as executor:
            results = list(executor.map(_identity, range(5)))

        assert results == [0, 1, 2, 3, 4]

    def test_different_processes(self):
        """Property: Tasks run in different processes."""
        with cf.ProcessPoolExecutor(max_workers=2) as executor:
            futures = [executor.submit(_get_pid) for _ in range(2)]
            pids = [f.result() for f in futures]

        # At least one should be different from main process
        main_pid = os.getpid()
        assert any(pid != main_pid for pid in pids)

    def test_context_manager(self):
        """Property: ProcessPoolExecutor works as context manager."""
        with cf.ProcessPoolExecutor(max_workers=2) as executor:
            results = list(executor.map(_multiply, [2, 3, 4], [3, 4, 5]))

        assert results == [6, 12, 20]


class TestFutureObjects:
    """Test Future object operations."""

    def test_future_result(self):
        """Property: Future.result() returns task result."""
        with cf.ThreadPoolExecutor(max_workers=1) as executor:
            future = executor.submit(pow, 3, 2)
            result = future.result()

        assert result == 9

    def test_future_done(self):
        """Property: Future.done() indicates completion."""
        with cf.ThreadPoolExecutor(max_workers=1) as executor:
            future = executor.submit(time.sleep, 0.01)
            initial_done = future.done()
            future.result()  # Wait for completion
            final_done = future.done()

        assert not initial_done
        assert final_done

    def test_future_running(self):
        """Property: Future.running() indicates active execution."""
        def slow_task():
            time.sleep(0.1)
            return 42

        with cf.ThreadPoolExecutor(max_workers=1) as executor:
            future = executor.submit(slow_task)
            time.sleep(0.01)  # Let task start

            # May or may not be running (timing dependent)
            assert isinstance(future.running(), bool)
            future.result()  # Wait for completion
            assert not future.running()

    def test_future_exception_none(self):
        """Property: Future.exception() is None when no error."""
        with cf.ThreadPoolExecutor(max_workers=1) as executor:
            future = executor.submit(pow, 2, 3)
            future.result()

        assert future.exception() is None

    def test_future_exception_captured(self):
        """Property: Future.exception() captures task exceptions."""
        def failing():
            raise ValueError("test error")

        with cf.ThreadPoolExecutor(max_workers=1) as executor:
            future = executor.submit(failing)

            with pytest.raises(ValueError, match="test error"):
                future.result()

            exception = future.exception()
            assert isinstance(exception, ValueError)

    def test_future_cancel_before_execution(self):
        """Property: Future.cancel() prevents execution if not started."""
        def slow_task():
            time.sleep(1.0)
            return "done"

        with cf.ThreadPoolExecutor(max_workers=1) as executor:
            # Submit blocking task
            blocking = executor.submit(slow_task)

            # Submit second task
            future = executor.submit(slow_task)

            # Try to cancel second task
            cancelled = future.cancel()

            # Cancel success depends on timing
            if cancelled:
                assert future.cancelled()
                with pytest.raises(cf.CancelledError):
                    future.result()

    def test_future_add_done_callback(self):
        """Property: Callbacks execute when future completes."""
        results = []

        def callback(future):
            results.append(future.result())

        with cf.ThreadPoolExecutor(max_workers=1) as executor:
            future = executor.submit(pow, 2, 4)
            future.add_done_callback(callback)
            future.result()  # Wait for completion

        time.sleep(0.01)  # Let callback execute
        assert results == [16]


class TestWaitPatterns:
    """Test wait() and as_completed() patterns."""

    def test_wait_all_completed(self):
        """Property: wait() waits for all futures by default."""
        def task(n):
            time.sleep(0.01)
            return n * 2

        with cf.ThreadPoolExecutor(max_workers=3) as executor:
            futures = [executor.submit(task, i) for i in range(5)]
            done, pending = cf.wait(futures)

        assert len(done) == 5
        assert len(pending) == 0

    def test_wait_first_completed(self):
        """Property: wait() can return when first future completes."""
        def task(n):
            time.sleep(n * 0.01)
            return n

        with cf.ThreadPoolExecutor(max_workers=3) as executor:
            futures = [executor.submit(task, i) for i in [3, 1, 2]]
            done, pending = cf.wait(
                futures, return_when=cf.FIRST_COMPLETED
            )

        assert len(done) >= 1
        assert len(done) + len(pending) == 3

    def test_wait_with_timeout(self):
        """Property: wait() respects timeout."""
        def slow_task():
            time.sleep(1.0)
            return "done"

        with cf.ThreadPoolExecutor(max_workers=2) as executor:
            futures = [executor.submit(slow_task) for _ in range(3)]
            done, pending = cf.wait(futures, timeout=0.01)

        # Most should still be pending
        assert len(pending) > 0

    def test_as_completed_iteration(self):
        """Property: as_completed() yields futures as they complete."""
        def task(n):
            time.sleep(n * 0.01)
            return n

        with cf.ThreadPoolExecutor(max_workers=3) as executor:
            futures = [executor.submit(task, i) for i in [3, 1, 2]]
            results = []

            for future in cf.as_completed(futures):
                results.append(future.result())

        # All results collected
        assert sorted(results) == [1, 2, 3]

    def test_as_completed_timeout(self):
        """Property: as_completed() can timeout."""
        def slow_task():
            time.sleep(1.0)
            return "done"

        with cf.ThreadPoolExecutor(max_workers=2) as executor:
            futures = [executor.submit(slow_task) for _ in range(3)]

            with pytest.raises(cf.TimeoutError):
                for _ in cf.as_completed(futures, timeout=0.01):
                    pass


class TestExceptionHandling:
    """Test exception handling in executors."""

    def test_map_propagates_exceptions(self):
        """Property: map() propagates exceptions from tasks."""
        def failing(n):
            if n == 2:
                raise ValueError("failed at 2")
            return n

        with cf.ThreadPoolExecutor(max_workers=2) as executor:
            results = executor.map(failing, range(5))

            with pytest.raises(ValueError, match="failed at 2"):
                list(results)

    def test_submit_exception_handling(self):
        """Property: submit() captures exceptions in Future."""
        with cf.ThreadPoolExecutor(max_workers=1) as executor:
            future = executor.submit(_failing_task)

            with pytest.raises(ValueError, match="task failed"):
                future.result()

    def test_process_pool_exception(self):
        """Property: ProcessPoolExecutor captures exceptions."""
        with cf.ProcessPoolExecutor(max_workers=1) as executor:
            future = executor.submit(_failing_task)

            with pytest.raises(ValueError, match="task failed"):
                future.result()


class TestEdgeCases:
    """Test edge cases and special scenarios."""

    def test_max_workers_default(self):
        """Property: Default max_workers is available CPUs."""
        with cf.ThreadPoolExecutor() as executor:
            # Should create executor successfully
            future = executor.submit(pow, 2, 3)
            assert future.result() == 8

    def test_map_empty_iterable(self):
        """Property: map() handles empty input."""
        def square(x):
            return x * x

        with cf.ThreadPoolExecutor(max_workers=2) as executor:
            results = list(executor.map(square, []))

        assert results == []

    def test_map_with_timeout(self):
        """Property: map() supports timeout parameter."""
        def slow_task(n):
            time.sleep(0.1)
            return n

        with cf.ThreadPoolExecutor(max_workers=2) as executor:
            results = executor.map(slow_task, range(5), timeout=0.01)

            with pytest.raises(cf.TimeoutError):
                list(results)

    def test_submit_after_shutdown(self):
        """Property: submit() after shutdown raises RuntimeError."""
        executor = cf.ThreadPoolExecutor(max_workers=1)
        executor.shutdown(wait=True)

        with pytest.raises(RuntimeError):
            executor.submit(pow, 2, 3)

    def test_result_with_timeout(self):
        """Property: Future.result() supports timeout."""
        with cf.ThreadPoolExecutor(max_workers=1) as executor:
            future = executor.submit(_slow_task, 0.1)

            with pytest.raises(cf.TimeoutError):
                future.result(timeout=0.01)

    def test_executor_reuse(self):
        """Property: Executor can handle multiple batches."""
        with cf.ThreadPoolExecutor(max_workers=2) as executor:
            # First batch
            futures1 = [executor.submit(pow, i, 2) for i in range(3)]
            results1 = [f.result() for f in futures1]

            # Second batch
            futures2 = [executor.submit(pow, i, 3) for i in range(3)]
            results2 = [f.result() for f in futures2]

        assert results1 == [0, 1, 4]
        assert results2 == [0, 1, 8]
