"""Pathological async-like patterns for transpiler stress testing.

Tests async def syntax, coroutine-like generators, state machines
that simulate async behavior, and event loop simulation.
All without external dependencies - only stdlib.
"""

from typing import List, Dict, Tuple, Optional, Callable, Any, Generator


# --- Coroutine-like patterns using generators ---

class Future:
    """Simple future/promise that holds a value or an exception."""

    def __init__(self):
        self._value: Any = None
        self._exception: Optional[Exception] = None
        self._done: bool = False
        self._callbacks: List[Callable] = []

    def set_result(self, value: Any):
        if self._done:
            raise RuntimeError("Future already resolved")
        self._value = value
        self._done = True
        for cb in self._callbacks:
            cb(self)

    def set_exception(self, exc: Exception):
        if self._done:
            raise RuntimeError("Future already resolved")
        self._exception = exc
        self._done = True
        for cb in self._callbacks:
            cb(self)

    def result(self) -> Any:
        if not self._done:
            raise RuntimeError("Future not yet resolved")
        if self._exception is not None:
            raise self._exception
        return self._value

    def done(self) -> bool:
        return self._done

    def add_callback(self, callback: Callable):
        self._callbacks.append(callback)
        if self._done:
            callback(self)


class Task:
    """Wraps a generator-based coroutine as a schedulable task."""

    _next_id: int = 0

    def __init__(self, coroutine: Generator, name: str = ""):
        Task._next_id += 1
        self.id = Task._next_id
        self.name = name if name else f"task-{self.id}"
        self._coroutine = coroutine
        self._result: Any = None
        self._done: bool = False
        self._exception: Optional[Exception] = None

    def step(self) -> bool:
        """Advance the coroutine one step. Returns True if still running."""
        if self._done:
            return False
        try:
            next(self._coroutine)
            return True
        except StopIteration as e:
            self._result = e.value
            self._done = True
            return False
        except Exception as e:
            self._exception = e
            self._done = True
            return False

    def is_done(self) -> bool:
        return self._done

    def result(self) -> Any:
        if not self._done:
            raise RuntimeError(f"Task {self.name} not done")
        if self._exception is not None:
            raise self._exception
        return self._result

    def __repr__(self) -> str:
        status = "done" if self._done else "running"
        return f"Task({self.name}, {status})"


class EventLoop:
    """Simple cooperative event loop that runs generator-based tasks."""

    def __init__(self):
        self._ready: List[Task] = []
        self._completed: List[Task] = []
        self._tick_count: int = 0
        self._log: List[str] = []

    def add_task(self, coroutine: Generator, name: str = "") -> Task:
        task = Task(coroutine, name)
        self._ready.append(task)
        self._log.append(f"scheduled: {task.name}")
        return task

    def run_until_complete(self) -> List[Task]:
        """Run all tasks until they complete (round-robin scheduling)."""
        while self._ready:
            self._tick_count += 1
            still_running: List[Task] = []
            for task in self._ready:
                if task.step():
                    still_running.append(task)
                else:
                    self._completed.append(task)
                    self._log.append(f"completed: {task.name}")
            self._ready = still_running
        return self._completed

    def tick_count(self) -> int:
        return self._tick_count

    def get_log(self) -> List[str]:
        return list(self._log)


# --- State machine simulating async FSM ---

class StateMachine:
    """Finite state machine that can simulate async-like transitions."""

    def __init__(self, name: str):
        self.name = name
        self._state: str = "INIT"
        self._transitions: Dict[Tuple[str, str], str] = {}
        self._actions: Dict[str, Callable] = {}
        self._history: List[str] = ["INIT"]

    def add_transition(self, from_state: str, event: str, to_state: str):
        self._transitions[(from_state, event)] = to_state

    def add_action(self, state: str, action: Callable):
        self._actions[state] = action

    def send_event(self, event: str) -> bool:
        key = (self._state, event)
        if key in self._transitions:
            new_state = self._transitions[key]
            self._state = new_state
            self._history.append(new_state)
            if new_state in self._actions:
                self._actions[new_state](self)
            return True
        return False

    def current_state(self) -> str:
        return self._state

    def history(self) -> List[str]:
        return list(self._history)


class ConnectionStateMachine(StateMachine):
    """Simulates a network connection lifecycle."""

    def __init__(self):
        super().__init__("connection")
        self.data_sent: List[str] = []
        self.data_received: List[str] = []

        # Transitions
        self.add_transition("INIT", "connect", "CONNECTING")
        self.add_transition("CONNECTING", "connected", "CONNECTED")
        self.add_transition("CONNECTING", "error", "ERROR")
        self.add_transition("CONNECTED", "send", "SENDING")
        self.add_transition("SENDING", "sent", "CONNECTED")
        self.add_transition("CONNECTED", "receive", "RECEIVING")
        self.add_transition("RECEIVING", "received", "CONNECTED")
        self.add_transition("CONNECTED", "disconnect", "DISCONNECTING")
        self.add_transition("DISCONNECTING", "disconnected", "CLOSED")
        self.add_transition("ERROR", "retry", "CONNECTING")
        self.add_transition("ERROR", "abort", "CLOSED")

    def send_data(self, data: str) -> bool:
        if self._state == "CONNECTED":
            self.send_event("send")
            self.data_sent.append(data)
            self.send_event("sent")
            return True
        return False

    def receive_data(self, data: str) -> bool:
        if self._state == "CONNECTED":
            self.send_event("receive")
            self.data_received.append(data)
            self.send_event("received")
            return True
        return False


# --- Generator-based coroutines ---

def counting_coroutine(name, count):
    """Generator that counts and yields at each step - untyped."""
    results = []
    for i in range(count):
        results.append(f"{name}:{i}")
        yield  # Cooperative yield point
    return results


def fibonacci_coroutine(n):
    """Generator-coroutine that computes fibonacci cooperatively - untyped."""
    a, b = 0, 1
    results = []
    for _ in range(n):
        results.append(a)
        a, b = b, a + b
        yield  # Yield after each computation
    return results


def pipeline_coroutine(data, transforms):
    """Apply transforms cooperatively, yielding between each - untyped."""
    result = data
    for transform in transforms:
        result = transform(result)
        yield  # Yield between transforms
    return result


def producer_consumer_simulation(items):
    """Simulate producer/consumer with alternating yields - untyped."""
    buffer = []
    produced = []
    consumed = []

    # Interleaved: produce one, yield, consume one, yield
    for item in items:
        buffer.append(item)
        produced.append(item)
        yield  # Producer yields

    # Now consume all buffered items
    while buffer:
        consumed.append(buffer.pop(0))
        yield  # Consumer yields

    return {"produced": produced, "consumed": consumed}


# --- Sync wrappers that LOOK like async ---

def sync_fetch(url: str) -> Dict[str, Any]:
    """Simulates an async fetch but is actually synchronous."""
    # In a real system this would be async
    parts = url.split("/")
    resource = parts[-1] if parts else "unknown"
    return {
        "url": url,
        "status": 200,
        "body": f"Response from {resource}",
        "headers": {"content-type": "text/plain"},
    }


def sync_sleep(ticks: int) -> int:
    """Simulates sleep by counting ticks - synchronous."""
    count = 0
    for _ in range(ticks):
        count += 1
    return count


def sync_parallel_map(func: Callable, items: List[Any]) -> List[Any]:
    """Simulates parallel map but runs sequentially."""
    results: List[Any] = []
    for item in items:
        results.append(func(item))
    return results


def sync_gather(*callables) -> List[Any]:
    """Simulates asyncio.gather but runs sequentially."""
    results: List[Any] = []
    for callable_fn in callables:
        results.append(callable_fn())
    return results


# --- Untyped helpers ---

def run_coroutine_to_completion(gen):
    """Run a generator-coroutine to completion and return its value - untyped."""
    try:
        while True:
            next(gen)
    except StopIteration as e:
        return e.value


def chain_coroutines(coroutines):
    """Run coroutines in round-robin until all complete - untyped."""
    results = {}
    active = list(enumerate(coroutines))
    while active:
        still_active = []
        for idx, coro in active:
            try:
                next(coro)
                still_active.append((idx, coro))
            except StopIteration as e:
                results[idx] = e.value
        active = still_active
    return results


def build_state_machine_from_spec(spec):
    """Build a state machine from a dict spec - untyped."""
    sm = StateMachine(spec.get("name", "unnamed"))
    for transition in spec.get("transitions", []):
        sm.add_transition(transition[0], transition[1], transition[2])
    return sm


def simulate_retries(operation, max_retries, should_fail_until):
    """Simulate an operation that fails N times then succeeds - untyped."""
    attempts = 0
    last_error = None
    for attempt in range(max_retries + 1):
        attempts += 1
        if attempt < should_fail_until:
            last_error = f"Attempt {attempt} failed"
        else:
            return {"success": True, "attempts": attempts, "result": operation()}
    return {"success": False, "attempts": attempts, "error": last_error}


# --- async def functions (valid Python, tested via sync wrapping) ---

async def async_add(a: int, b: int) -> int:
    """Async function that adds two numbers."""
    return a + b


async def async_factorial(n: int) -> int:
    """Async factorial computation."""
    result = 1
    for i in range(2, n + 1):
        result *= i
    return result


async def async_fibonacci_list(n: int) -> List[int]:
    """Async function that returns fibonacci list."""
    fibs: List[int] = []
    a, b = 0, 1
    for _ in range(n):
        fibs.append(a)
        a, b = b, a + b
    return fibs


async def async_process_data(data: List[int]) -> Dict[str, Any]:
    """Async data processing pipeline."""
    total = 0
    count = 0
    min_val = data[0] if data else 0
    max_val = data[0] if data else 0

    for val in data:
        total += val
        count += 1
        if val < min_val:
            min_val = val
        if val > max_val:
            max_val = val

    avg = total / count if count > 0 else 0.0
    return {
        "total": total,
        "count": count,
        "average": avg,
        "min": min_val,
        "max": max_val,
    }


# --- Typed test functions ---

def test_future():
    """Test Future basic operations."""
    f = Future()
    assert not f.done()

    f.set_result(42)
    assert f.done()
    assert f.result() == 42

    f2 = Future()
    f2.set_exception(ValueError("test"))
    assert f2.done()
    raised = False
    try:
        f2.result()
    except ValueError:
        raised = True
    assert raised

    # Test callback
    callback_results: List[Any] = []
    f3 = Future()
    f3.add_callback(lambda fut: callback_results.append(fut.result()))
    f3.set_result(99)
    assert callback_results == [99]
    return True


def test_event_loop():
    """Test EventLoop with generator coroutines."""
    loop = EventLoop()

    t1 = loop.add_task(counting_coroutine("A", 3), "counter-A")
    t2 = loop.add_task(counting_coroutine("B", 2), "counter-B")

    completed = loop.run_until_complete()
    assert len(completed) == 2

    # All tasks should be done
    assert t1.is_done()
    assert t2.is_done()

    r1 = t1.result()
    assert r1 == ["A:0", "A:1", "A:2"]

    r2 = t2.result()
    assert r2 == ["B:0", "B:1"]

    assert loop.tick_count() >= 2
    return True


def test_fibonacci_coroutine():
    """Test fibonacci generator-coroutine."""
    result = run_coroutine_to_completion(fibonacci_coroutine(8))
    assert result == [0, 1, 1, 2, 3, 5, 8, 13]
    return True


def test_pipeline_coroutine():
    """Test cooperative pipeline."""
    transforms = [
        lambda x: x * 2,
        lambda x: x + 10,
        lambda x: x * x,
    ]
    result = run_coroutine_to_completion(pipeline_coroutine(5, transforms))
    # 5 * 2 = 10, 10 + 10 = 20, 20 * 20 = 400
    assert result == 400
    return True


def test_producer_consumer():
    """Test producer/consumer simulation."""
    result = run_coroutine_to_completion(
        producer_consumer_simulation([1, 2, 3, 4, 5])
    )
    assert result["produced"] == [1, 2, 3, 4, 5]
    assert result["consumed"] == [1, 2, 3, 4, 5]
    return True


def test_chain_coroutines():
    """Test round-robin coroutine chaining."""
    results = chain_coroutines([
        counting_coroutine("X", 2),
        fibonacci_coroutine(5),
    ])
    assert results[0] == ["X:0", "X:1"]
    assert results[1] == [0, 1, 1, 2, 3]
    return True


def test_connection_state_machine():
    """Test ConnectionStateMachine lifecycle."""
    conn = ConnectionStateMachine()
    assert conn.current_state() == "INIT"

    conn.send_event("connect")
    assert conn.current_state() == "CONNECTING"

    conn.send_event("connected")
    assert conn.current_state() == "CONNECTED"

    assert conn.send_data("hello")
    assert conn.send_data("world")
    assert conn.data_sent == ["hello", "world"]

    assert conn.receive_data("response1")
    assert conn.data_received == ["response1"]

    conn.send_event("disconnect")
    conn.send_event("disconnected")
    assert conn.current_state() == "CLOSED"

    history = conn.history()
    assert history[0] == "INIT"
    assert "CONNECTED" in history
    assert history[-1] == "CLOSED"
    return True


def test_error_retry_state_machine():
    """Test error and retry transitions."""
    conn = ConnectionStateMachine()
    conn.send_event("connect")
    conn.send_event("error")
    assert conn.current_state() == "ERROR"

    conn.send_event("retry")
    assert conn.current_state() == "CONNECTING"

    conn.send_event("connected")
    assert conn.current_state() == "CONNECTED"
    return True


def test_sync_wrappers():
    """Test sync functions that simulate async."""
    resp = sync_fetch("https://example.com/api/data")
    assert resp["status"] == 200
    assert "data" in resp["body"]

    ticks = sync_sleep(100)
    assert ticks == 100

    results = sync_parallel_map(lambda x: x * x, [1, 2, 3, 4])
    assert results == [1, 4, 9, 16]

    gathered = sync_gather(
        lambda: 1 + 1,
        lambda: 2 * 3,
        lambda: 10 - 4,
    )
    assert gathered == [2, 6, 6]
    return True


def test_retry_simulation():
    """Test retry simulation."""
    result = simulate_retries(lambda: "success!", 5, 3)
    assert result["success"] == True
    assert result["attempts"] == 4
    assert result["result"] == "success!"

    result2 = simulate_retries(lambda: "ok", 2, 5)
    assert result2["success"] == False
    assert result2["attempts"] == 3
    return True


def test_async_functions():
    """Test that async functions produce coroutines we can send() into."""
    # We can call .send(None) on the coroutine to run it
    coro = async_add(3, 4)
    try:
        coro.send(None)
    except StopIteration as e:
        assert e.value == 7

    coro2 = async_factorial(5)
    try:
        coro2.send(None)
    except StopIteration as e:
        assert e.value == 120

    coro3 = async_fibonacci_list(6)
    try:
        coro3.send(None)
    except StopIteration as e:
        assert e.value == [0, 1, 1, 2, 3, 5]

    coro4 = async_process_data([10, 20, 30, 40, 50])
    try:
        coro4.send(None)
    except StopIteration as e:
        result = e.value
        assert result["total"] == 150
        assert result["count"] == 5
        assert abs(result["average"] - 30.0) < 1e-9
        assert result["min"] == 10
        assert result["max"] == 50
    return True


def test_state_machine_from_spec() -> bool:
    """Test building state machine from spec dict."""
    spec = {
        "name": "traffic_light",
        "transitions": [
            ("INIT", "start", "RED"),
            ("RED", "timer", "GREEN"),
            ("GREEN", "timer", "YELLOW"),
            ("YELLOW", "timer", "RED"),
        ]
    }
    sm = build_state_machine_from_spec(spec)
    sm.send_event("start")
    assert sm.current_state() == "RED"
    sm.send_event("timer")
    assert sm.current_state() == "GREEN"
    sm.send_event("timer")
    assert sm.current_state() == "YELLOW"
    sm.send_event("timer")
    assert sm.current_state() == "RED"
    return True


def test_all() -> bool:
    """Run all tests."""
    assert test_future()
    assert test_event_loop()
    assert test_fibonacci_coroutine()
    assert test_pipeline_coroutine()
    assert test_producer_consumer()
    assert test_chain_coroutines()
    assert test_connection_state_machine()
    assert test_error_retry_state_machine()
    assert test_sync_wrappers()
    assert test_retry_simulation()
    assert test_async_functions()
    assert test_state_machine_from_spec()
    return True


def main():
    """Entry point."""
    if test_all():
        print("hard_async_patterns: ALL TESTS PASSED")
    else:
        print("hard_async_patterns: TESTS FAILED")


if __name__ == "__main__":
    main()
