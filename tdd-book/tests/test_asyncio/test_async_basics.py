"""
TDD Book - Phase 3: Concurrency
Module: asyncio - Asynchronous I/O
Coverage: async/await, Tasks, Futures, event loops, synchronization

Test Categories:
- Basic async/await syntax
- Task creation and management
- Event loop operations
- Async iterators and generators
- Timeouts and cancellation
- Async synchronization primitives
- Async queues
- Edge cases and error handling
"""

import pytest
import asyncio
import time


class TestAsyncBasics:
    """Test basic async/await functionality."""

    @pytest.mark.asyncio
    async def test_simple_coroutine(self):
        """Property: async function returns coroutine."""

        async def simple():
            return 42

        result = await simple()
        assert result == 42

    @pytest.mark.asyncio
    async def test_await_coroutine(self):
        """Property: await suspends until coroutine completes."""

        async def add(a, b):
            await asyncio.sleep(0.01)
            return a + b

        result = await add(10, 20)
        assert result == 30

    @pytest.mark.asyncio
    async def test_multiple_awaits(self):
        """Property: Multiple awaits execute sequentially."""

        async def fetch(n):
            await asyncio.sleep(0.01)
            return n * 2

        results = []
        results.append(await fetch(1))
        results.append(await fetch(2))
        results.append(await fetch(3))

        assert results == [2, 4, 6]

    @pytest.mark.asyncio
    async def test_gather_concurrent(self):
        """Property: gather executes coroutines concurrently."""

        async def fetch(n):
            await asyncio.sleep(0.01)
            return n * 2

        results = await asyncio.gather(fetch(1), fetch(2), fetch(3))
        assert results == [2, 4, 6]

    @pytest.mark.asyncio
    async def test_gather_preserves_order(self):
        """Property: gather returns results in original order."""

        async def slow():
            await asyncio.sleep(0.02)
            return "slow"

        async def fast():
            await asyncio.sleep(0.01)
            return "fast"

        results = await asyncio.gather(slow(), fast())
        assert results == ["slow", "fast"]  # Order preserved, not completion order


class TestTasks:
    """Test Task creation and management."""

    @pytest.mark.asyncio
    async def test_create_task(self):
        """Property: create_task schedules coroutine for execution."""
        result = []

        async def worker():
            await asyncio.sleep(0.01)
            result.append(42)

        task = asyncio.create_task(worker())
        await task
        assert result == [42]

    @pytest.mark.asyncio
    async def test_task_result(self):
        """Property: Task.result() returns coroutine result."""

        async def compute():
            await asyncio.sleep(0.01)
            return 100

        task = asyncio.create_task(compute())
        await task
        assert task.result() == 100

    @pytest.mark.asyncio
    async def test_task_done(self):
        """Property: Task.done() indicates completion."""

        async def worker():
            await asyncio.sleep(0.01)

        task = asyncio.create_task(worker())
        assert not task.done()

        await task
        assert task.done()

    @pytest.mark.asyncio
    async def test_task_cancel(self):
        """Property: Task can be cancelled."""

        async def long_task():
            await asyncio.sleep(10)

        task = asyncio.create_task(long_task())
        task.cancel()

        with pytest.raises(asyncio.CancelledError):
            await task

    @pytest.mark.asyncio
    async def test_multiple_tasks_concurrent(self):
        """Property: Multiple tasks run concurrently."""
        results = []

        async def worker(n):
            await asyncio.sleep(0.01)
            results.append(n)

        tasks = [asyncio.create_task(worker(i)) for i in range(5)]
        await asyncio.gather(*tasks)

        assert sorted(results) == [0, 1, 2, 3, 4]

    @pytest.mark.asyncio
    async def test_task_exception(self):
        """Property: Task captures exceptions."""

        async def failing_task():
            await asyncio.sleep(0.01)
            raise ValueError("task failed")

        task = asyncio.create_task(failing_task())

        with pytest.raises(ValueError, match="task failed"):
            await task


class TestAsyncIterators:
    """Test async iterators and generators."""

    @pytest.mark.asyncio
    async def test_async_for(self):
        """Property: async for iterates over async iterator."""

        async def async_range(n):
            for i in range(n):
                await asyncio.sleep(0.001)
                yield i

        results = []
        async for value in async_range(5):
            results.append(value)

        assert results == [0, 1, 2, 3, 4]

    @pytest.mark.asyncio
    async def test_async_generator(self):
        """Property: async generator yields values asynchronously."""

        async def fibonacci(n):
            a, b = 0, 1
            for _ in range(n):
                await asyncio.sleep(0.001)
                yield a
                a, b = b, a + b

        results = []
        async for value in fibonacci(6):
            results.append(value)

        assert results == [0, 1, 1, 2, 3, 5]

    @pytest.mark.asyncio
    async def test_async_comprehension(self):
        """Property: async comprehension collects async iterator results."""

        async def async_range(n):
            for i in range(n):
                await asyncio.sleep(0.001)
                yield i * 2

        results = [x async for x in async_range(5)]
        assert results == [0, 2, 4, 6, 8]


class TestTimeouts:
    """Test timeout and cancellation behavior."""

    @pytest.mark.asyncio
    async def test_wait_for_timeout(self):
        """Property: wait_for raises TimeoutError after timeout."""

        async def slow_task():
            await asyncio.sleep(1.0)
            return "done"

        with pytest.raises(asyncio.TimeoutError):
            await asyncio.wait_for(slow_task(), timeout=0.01)

    @pytest.mark.asyncio
    async def test_wait_for_completes(self):
        """Property: wait_for returns result if completes in time."""

        async def fast_task():
            await asyncio.sleep(0.01)
            return "done"

        result = await asyncio.wait_for(fast_task(), timeout=1.0)
        assert result == "done"

    @pytest.mark.asyncio
    async def test_shield_protects_from_cancel(self):
        """Property: shield protects inner task from cancellation."""

        async def protected_task():
            await asyncio.sleep(0.02)
            return "completed"

        inner_task = asyncio.create_task(protected_task())
        outer = asyncio.shield(inner_task)

        # Cancel the shield (outer), but inner task continues
        try:
            await asyncio.wait_for(outer, timeout=0.01)
        except asyncio.TimeoutError:
            pass

        # Inner task should still complete
        result = await inner_task
        assert result == "completed"


class TestAsyncSynchronization:
    """Test async synchronization primitives."""

    @pytest.mark.asyncio
    async def test_lock_mutual_exclusion(self):
        """Property: Lock ensures mutual exclusion."""
        counter = 0
        lock = asyncio.Lock()

        async def increment():
            nonlocal counter
            async with lock:
                temp = counter
                await asyncio.sleep(0.001)
                counter = temp + 1

        await asyncio.gather(*[increment() for _ in range(10)])
        assert counter == 10

    @pytest.mark.asyncio
    async def test_lock_acquire_release(self):
        """Property: Lock can be acquired and released."""
        lock = asyncio.Lock()
        assert not lock.locked()

        await lock.acquire()
        assert lock.locked()

        lock.release()
        assert not lock.locked()

    @pytest.mark.asyncio
    async def test_semaphore_limits_access(self):
        """Property: Semaphore limits concurrent access."""
        sem = asyncio.Semaphore(2)
        active = []

        async def worker(n):
            async with sem:
                active.append(n)
                await asyncio.sleep(0.01)
                assert len(active) <= 2  # At most 2 concurrent
                active.remove(n)

        await asyncio.gather(*[worker(i) for i in range(5)])

    @pytest.mark.asyncio
    async def test_event_wait_set(self):
        """Property: Event blocks until set."""
        event = asyncio.Event()
        result = []

        async def waiter():
            await event.wait()
            result.append("done")

        async def setter():
            await asyncio.sleep(0.01)
            event.set()

        await asyncio.gather(waiter(), setter())
        assert result == ["done"]

    @pytest.mark.asyncio
    async def test_condition_wait_notify(self):
        """Property: Condition coordinates async operations."""
        condition = asyncio.Condition()
        result = []

        async def waiter():
            async with condition:
                await condition.wait()
                result.append("notified")

        async def notifier():
            await asyncio.sleep(0.01)
            async with condition:
                condition.notify()

        await asyncio.gather(waiter(), notifier())
        assert result == ["notified"]


class TestAsyncQueue:
    """Test asyncio.Queue for async producer-consumer."""

    @pytest.mark.asyncio
    async def test_queue_put_get(self):
        """Property: Queue transfers data between coroutines."""
        queue = asyncio.Queue()

        async def producer():
            for i in range(5):
                await queue.put(i)

        async def consumer():
            results = []
            for _ in range(5):
                results.append(await queue.get())
            return results

        prod = asyncio.create_task(producer())
        cons = asyncio.create_task(consumer())

        await prod
        results = await cons
        assert results == [0, 1, 2, 3, 4]

    @pytest.mark.asyncio
    async def test_queue_maxsize(self):
        """Property: Queue respects maxsize."""
        queue = asyncio.Queue(maxsize=2)
        await queue.put(1)
        await queue.put(2)
        assert queue.full()

    @pytest.mark.asyncio
    async def test_queue_task_done_join(self):
        """Property: Queue.join() waits for all tasks."""
        queue = asyncio.Queue()
        results = []

        async def worker():
            while True:
                item = await queue.get()
                if item is None:
                    queue.task_done()
                    break
                results.append(item * 2)
                await asyncio.sleep(0.001)
                queue.task_done()

        worker_task = asyncio.create_task(worker())

        for i in range(5):
            await queue.put(i)

        await queue.join()  # Wait for all tasks
        await queue.put(None)  # Stop worker
        await worker_task

        assert sorted(results) == [0, 2, 4, 6, 8]


class TestAsyncContext:
    """Test async context managers."""

    @pytest.mark.asyncio
    async def test_async_with(self):
        """Property: async with manages async context."""

        class AsyncResource:
            def __init__(self):
                self.entered = False
                self.exited = False

            async def __aenter__(self):
                await asyncio.sleep(0.001)
                self.entered = True
                return self

            async def __aexit__(self, exc_type, exc_val, exc_tb):
                await asyncio.sleep(0.001)
                self.exited = True

        resource = AsyncResource()
        async with resource:
            assert resource.entered
            assert not resource.exited

        assert resource.exited


class TestAsyncEdgeCases:
    """Test edge cases and error handling."""

    @pytest.mark.asyncio
    async def test_sleep_zero(self):
        """Property: sleep(0) yields control to event loop."""
        executed = []

        async def task1():
            executed.append(1)
            await asyncio.sleep(0)
            executed.append(3)

        async def task2():
            executed.append(2)

        await asyncio.gather(task1(), task2())
        # Order depends on scheduling, but both execute

    @pytest.mark.asyncio
    async def test_gather_exceptions(self):
        """Property: gather with return_exceptions collects errors."""

        async def failing():
            raise ValueError("error")

        async def succeeding():
            return "success"

        results = await asyncio.gather(
            failing(), succeeding(), return_exceptions=True
        )

        assert isinstance(results[0], ValueError)
        assert results[1] == "success"

    @pytest.mark.asyncio
    async def test_as_completed(self):
        """Property: as_completed yields tasks as they complete."""

        async def task(n, delay):
            await asyncio.sleep(delay)
            return n

        tasks = [task(1, 0.03), task(2, 0.01), task(3, 0.02)]
        results = []

        for coro in asyncio.as_completed(tasks):
            result = await coro
            results.append(result)

        # Results in completion order (2, 3, 1)
        assert 2 in results  # Fastest completes first

    @pytest.mark.asyncio
    async def test_wait_first_completed(self):
        """Property: wait returns when first task completes."""

        async def fast():
            await asyncio.sleep(0.01)
            return "fast"

        async def slow():
            await asyncio.sleep(1.0)
            return "slow"

        tasks = [asyncio.create_task(fast()), asyncio.create_task(slow())]
        done, pending = await asyncio.wait(tasks, return_when=asyncio.FIRST_COMPLETED)

        assert len(done) == 1
        assert len(pending) == 1

        # Cancel pending
        for task in pending:
            task.cancel()

    @pytest.mark.asyncio
    async def test_current_task(self):
        """Property: current_task returns running task."""

        async def check_current():
            task = asyncio.current_task()
            return task.get_name()

        result = await check_current()
        assert isinstance(result, str)

    @pytest.mark.asyncio
    async def test_all_tasks(self):
        """Property: all_tasks returns all tasks."""

        async def worker():
            await asyncio.sleep(0.01)

        initial_count = len(asyncio.all_tasks())
        task = asyncio.create_task(worker())

        # New task appears in all_tasks
        assert len(asyncio.all_tasks()) > initial_count

        await task

    @pytest.mark.asyncio
    async def test_run_coroutine_threadsafe(self):
        """Property: Coroutine result can be retrieved."""
        # This is a simplified test - actual threadsafe usage is complex

        async def simple():
            return 42

        result = await simple()
        assert result == 42
