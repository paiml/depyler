"""Golden Async/Concurrency Example - DEPYLER-0984

A fully type-annotated Python file specifically designed to test
async/await transformations (asyncio → Tokio).

Purpose: Falsify hypothesis that async codegen is sound.
Method: Every function has explicit type annotations, isolating
async-specific bugs from type inference issues.

Async patterns tested:
1. Basic async def functions
2. await expressions
3. async functions calling other async functions
4. async with context managers
5. async for loops (async iterators)
6. Concurrent execution (gather-like patterns)
7. Mixing sync and async code
8. Async generators
9. Return value propagation through Future

Expected Rust transformations:
- async def → async fn
- await expr → expr.await
- asyncio.run() → #[tokio::main] or block_on
- asyncio.gather() → tokio::join! or futures::join_all
- async with → async block with Drop guard
- async for → while let with .next().await
"""

from typing import List, Optional, Dict, Tuple
import asyncio


# =============================================================================
# Basic Async Functions (Pattern 1, 2)
# =============================================================================

async def simple_async() -> int:
    """Simplest async function.

    Python: async def → Future[int]
    Rust: async fn simple_async() -> i64
    """
    return 42


async def async_with_await() -> int:
    """Async function that awaits another.

    Python: await simple_async()
    Rust: simple_async().await
    """
    result: int = await simple_async()
    return result * 2


async def async_with_sleep(seconds: float) -> str:
    """Async function with sleep.

    Python: await asyncio.sleep(seconds)
    Rust: tokio::time::sleep(Duration::from_secs_f64(seconds)).await
    """
    await asyncio.sleep(seconds)
    return f"Slept for {seconds} seconds"


# =============================================================================
# Async Function Chains (Pattern 3)
# =============================================================================

async def compute_step1(x: int) -> int:
    """First step in async computation chain."""
    await asyncio.sleep(0.001)  # Simulate async work
    return x + 10


async def compute_step2(x: int) -> int:
    """Second step in async computation chain."""
    await asyncio.sleep(0.001)
    return x * 2


async def compute_step3(x: int) -> int:
    """Third step in async computation chain."""
    await asyncio.sleep(0.001)
    return x - 5


async def async_computation_chain(start: int) -> int:
    """Chain of async function calls.

    Python: Sequential awaits
    Rust: Sequential .await calls
    """
    step1: int = await compute_step1(start)
    step2: int = await compute_step2(step1)
    step3: int = await compute_step3(step2)
    return step3


# =============================================================================
# Async Context Managers (Pattern 4)
# =============================================================================

class AsyncResource:
    """Simulated async resource (e.g., database connection).

    Python: async with AsyncResource() as resource
    Rust: Async block with Drop guard or explicit cleanup
    """

    def __init__(self, name: str) -> None:
        self.name: str = name
        self.is_open: bool = False
        self.data: str = ""

    async def __aenter__(self) -> "AsyncResource":
        await asyncio.sleep(0.001)  # Simulate connection
        self.is_open = True
        return self

    async def __aexit__(
        self,
        exc_type: Optional[type],
        exc_val: Optional[BaseException],
        exc_tb: Optional[object]
    ) -> bool:
        await asyncio.sleep(0.001)  # Simulate cleanup
        self.is_open = False
        return False  # Don't suppress exceptions


async def use_async_context_manager(name: str) -> str:
    """Use async context manager.

    Python: async with AsyncResource(name) as r
    Rust: Async block with resource management
    """
    result: str = ""
    async with AsyncResource(name) as resource:
        resource.data = f"Data for {resource.name}"
        result = resource.data
    return result


async def nested_async_context_managers(name1: str, name2: str) -> str:
    """Nested async context managers.

    Python: Nested async with statements
    Rust: Nested async blocks
    """
    results: List[str] = []
    async with AsyncResource(name1) as r1:
        async with AsyncResource(name2) as r2:
            results.append(r1.name)
            results.append(r2.name)
    return ",".join(results)


# =============================================================================
# Async Iterators / async for (Pattern 5)
# =============================================================================

class AsyncCounter:
    """Async iterator that yields numbers.

    Python: async for x in AsyncCounter(5)
    Rust: while let Some(x) = counter.next().await
    """

    def __init__(self, limit: int) -> None:
        self.limit: int = limit
        self.current: int = 0

    def __aiter__(self) -> "AsyncCounter":
        return self

    async def __anext__(self) -> int:
        if self.current >= self.limit:
            raise StopAsyncIteration
        await asyncio.sleep(0.001)  # Simulate async work
        value: int = self.current
        self.current += 1
        return value


async def async_for_loop(limit: int) -> int:
    """Use async for loop.

    Python: async for x in AsyncCounter(limit)
    Rust: while let Some(x) = counter.next().await
    """
    total: int = 0
    async for value in AsyncCounter(limit):
        total += value
    return total


async def async_for_with_break(limit: int, stop_at: int) -> int:
    """Async for with early break.

    Python: async for with break condition
    Rust: break in while let loop
    """
    total: int = 0
    async for value in AsyncCounter(limit):
        if value >= stop_at:
            break
        total += value
    return total


# =============================================================================
# Concurrent Execution (Pattern 6)
# =============================================================================

async def task_a() -> str:
    """First concurrent task."""
    await asyncio.sleep(0.01)
    return "A"


async def task_b() -> str:
    """Second concurrent task."""
    await asyncio.sleep(0.01)
    return "B"


async def task_c() -> str:
    """Third concurrent task."""
    await asyncio.sleep(0.01)
    return "C"


async def concurrent_gather() -> List[str]:
    """Run tasks concurrently with gather.

    Python: asyncio.gather(task_a(), task_b(), task_c())
    Rust: tokio::join!(task_a(), task_b(), task_c())
    """
    results: List[str] = await asyncio.gather(task_a(), task_b(), task_c())
    return results


async def concurrent_with_results(values: List[int]) -> List[int]:
    """Process multiple values concurrently.

    Python: asyncio.gather(*[process(v) for v in values])
    Rust: futures::future::join_all or tokio::join!
    """
    async def process(x: int) -> int:
        await asyncio.sleep(0.001)
        return x * 2

    tasks = [process(v) for v in values]
    results: List[int] = await asyncio.gather(*tasks)
    return results


async def concurrent_with_timeout(timeout_secs: float) -> Optional[str]:
    """Concurrent execution with timeout.

    Python: asyncio.wait_for with timeout
    Rust: tokio::time::timeout
    """
    async def slow_task() -> str:
        await asyncio.sleep(1.0)  # Intentionally slow
        return "completed"

    try:
        result: str = await asyncio.wait_for(slow_task(), timeout=timeout_secs)
        return result
    except asyncio.TimeoutError:
        return None


# =============================================================================
# Mixing Sync and Async (Pattern 7)
# =============================================================================

def sync_helper(x: int) -> int:
    """Synchronous helper function."""
    return x * 3


async def async_calling_sync(x: int) -> int:
    """Async function calling sync function.

    Python: Call regular function from async
    Rust: Direct call (sync functions can be called from async)
    """
    intermediate: int = sync_helper(x)
    await asyncio.sleep(0.001)
    return intermediate + 1


async def async_with_sync_computation(values: List[int]) -> int:
    """Mix of async and sync operations.

    Python: Sync operations between await points
    Rust: Regular Rust code between .await points
    """
    total: int = 0
    for v in values:
        # Sync computation
        doubled: int = v * 2
        total += doubled

    # Async operation
    await asyncio.sleep(0.001)

    # More sync computation
    result: int = total // len(values) if values else 0
    return result


# =============================================================================
# Async with Exception Handling (Combined patterns)
# =============================================================================

async def async_with_exception_handling(x: int) -> int:
    """Async function with try/except.

    Python: try/except in async function
    Rust: Result handling in async fn
    """
    try:
        if x < 0:
            raise ValueError("Negative value")
        await asyncio.sleep(0.001)
        return x * 2
    except ValueError:
        return -1


async def async_reraise_exception(x: int) -> int:
    """Async function that re-raises exception.

    Python: raise in async except block
    Rust: return Err(e) in async fn
    """
    try:
        if x == 0:
            raise ZeroDivisionError("Cannot be zero")
        await asyncio.sleep(0.001)
        return 100 // x
    except ZeroDivisionError:
        raise


# =============================================================================
# Async Return Type Variations
# =============================================================================

async def async_return_optional(x: int) -> Optional[int]:
    """Async function returning Optional.

    Python: async def -> Optional[int]
    Rust: async fn ... -> Option<i64>
    """
    await asyncio.sleep(0.001)
    if x < 0:
        return None
    return x


async def async_return_tuple(a: int, b: int) -> Tuple[int, int]:
    """Async function returning tuple.

    Python: async def -> Tuple[int, int]
    Rust: async fn ... -> (i64, i64)
    """
    await asyncio.sleep(0.001)
    return (a + b, a * b)


async def async_return_dict(keys: List[str]) -> Dict[str, int]:
    """Async function returning dict.

    Python: async def -> Dict[str, int]
    Rust: async fn ... -> HashMap<String, i64>
    """
    result: Dict[str, int] = {}
    for i, key in enumerate(keys):
        await asyncio.sleep(0.001)
        result[key] = i
    return result


# =============================================================================
# Main Entry Point
# =============================================================================

async def async_main() -> int:
    """Async main function exercising all patterns."""

    # Test simple async
    assert await simple_async() == 42

    # Test async with await
    assert await async_with_await() == 84

    # Test async computation chain: (5 + 10) * 2 - 5 = 25
    assert await async_computation_chain(5) == 25

    # Test async context manager
    result: str = await use_async_context_manager("test")
    assert result == "Data for test"

    # Test nested context managers
    nested: str = await nested_async_context_managers("a", "b")
    assert nested == "a,b"

    # Test async for loop: 0+1+2+3+4 = 10
    assert await async_for_loop(5) == 10

    # Test async for with break: 0+1+2 = 3 (break at 3)
    assert await async_for_with_break(10, 3) == 3

    # Test concurrent gather
    gather_results: List[str] = await concurrent_gather()
    assert set(gather_results) == {"A", "B", "C"}

    # Test concurrent with results
    concurrent_results: List[int] = await concurrent_with_results([1, 2, 3])
    assert concurrent_results == [2, 4, 6]

    # Test timeout (should return None for short timeout)
    timeout_result: Optional[str] = await concurrent_with_timeout(0.001)
    assert timeout_result is None

    # Test async calling sync
    assert await async_calling_sync(10) == 31  # 10*3 + 1

    # Test async with sync computation
    assert await async_with_sync_computation([2, 4, 6]) == 8  # (4+8+12)/3 = 8

    # Test async exception handling
    assert await async_with_exception_handling(5) == 10
    assert await async_with_exception_handling(-5) == -1

    # Test async return optional
    assert await async_return_optional(5) == 5
    assert await async_return_optional(-5) is None

    # Test async return tuple
    tuple_result: Tuple[int, int] = await async_return_tuple(3, 4)
    assert tuple_result == (7, 12)

    # Test async return dict
    dict_result: Dict[str, int] = await async_return_dict(["a", "b", "c"])
    assert dict_result == {"a": 0, "b": 1, "c": 2}

    return 0


def main() -> int:
    """Entry point that runs the async main."""
    result: int = asyncio.run(async_main())
    return result


if __name__ == "__main__":
    exit(main())
