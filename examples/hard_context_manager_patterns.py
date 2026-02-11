"""Pathological context manager patterns for transpiler stress testing.

Tests __enter__/__exit__, nested context managers, exception suppression,
resource management, and value-returning context managers.
"""

from typing import List, Optional, Dict, Any, Tuple


class ManagedResource:
    """A resource that tracks open/close state and validates proper cleanup."""

    _global_open_count: int = 0

    def __init__(self, name: str, fail_on_enter: bool = False, fail_on_exit: bool = False):
        self.name = name
        self.is_open = False
        self.operations: List[str] = []
        self._fail_on_enter = fail_on_enter
        self._fail_on_exit = fail_on_exit

    def __enter__(self):
        if self._fail_on_enter:
            raise RuntimeError(f"Failed to open resource: {self.name}")
        self.is_open = True
        ManagedResource._global_open_count += 1
        self.operations.append("opened")
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.is_open = False
        ManagedResource._global_open_count -= 1
        self.operations.append("closed")
        if self._fail_on_exit:
            raise RuntimeError(f"Failed to close resource: {self.name}")
        return False  # Don't suppress exceptions

    def read(self) -> str:
        if not self.is_open:
            raise RuntimeError(f"Cannot read from closed resource: {self.name}")
        self.operations.append("read")
        return f"data_from_{self.name}"

    def write(self, data: str) -> int:
        if not self.is_open:
            raise RuntimeError(f"Cannot write to closed resource: {self.name}")
        self.operations.append(f"write({data})")
        return len(data)


class ExceptionSuppressor:
    """Context manager that selectively suppresses exceptions by type."""

    def __init__(self, *exception_types):
        self.suppressed_types = exception_types
        self.caught_exception = None
        self.was_suppressed = False

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        if exc_type is not None:
            for suppressed in self.suppressed_types:
                if issubclass(exc_type, suppressed):
                    self.caught_exception = exc_val
                    self.was_suppressed = True
                    return True  # Suppress the exception
        return False


class TransactionContext:
    """Simulates a database transaction with commit/rollback semantics."""

    def __init__(self, name: str = "default"):
        self.name = name
        self.journal: List[Tuple[str, Any]] = []
        self.committed = False
        self.rolled_back = False
        self._data: Dict[str, Any] = {}
        self._snapshot: Dict[str, Any] = {}

    def __enter__(self):
        self._snapshot = dict(self._data)
        self.journal.append(("BEGIN", self.name))
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        if exc_type is not None:
            # Rollback on exception
            self._data = self._snapshot
            self.rolled_back = True
            self.journal.append(("ROLLBACK", str(exc_val)))
            return True  # Suppress the exception
        else:
            self.committed = True
            self.journal.append(("COMMIT", self.name))
        return False

    def set(self, key: str, value: Any):
        self.journal.append(("SET", f"{key}={value}"))
        self._data[key] = value

    def get(self, key: str, default: Any = None) -> Any:
        return self._data.get(key, default)

    def get_journal(self) -> List[str]:
        return [f"{op}:{detail}" for op, detail in self.journal]


class TimingContext:
    """Context manager that measures elapsed 'ticks' (simulated, no time import)."""

    def __init__(self, label: str = ""):
        self.label = label
        self.tick_count: int = 0
        self._entered = False

    def __enter__(self):
        self._entered = True
        self.tick_count = 0
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self._entered = False
        return False

    def tick(self) -> int:
        if not self._entered:
            raise RuntimeError("Cannot tick outside of context")
        self.tick_count += 1
        return self.tick_count


class StackContext:
    """Context manager that maintains a stack, popping on exit."""

    def __init__(self):
        self._stack: List[str] = []
        self._depth = 0

    def push(self, frame: str):
        self._stack.append(frame)

    def __enter__(self):
        self._depth += 1
        self._stack.append(f"frame_{self._depth}")
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        if self._stack:
            self._stack.pop()
        return False

    def depth(self) -> int:
        return len(self._stack)

    def current(self) -> Optional[str]:
        if self._stack:
            return self._stack[-1]
        return None

    def snapshot(self) -> List[str]:
        return list(self._stack)


# --- Untyped helper functions ---

def use_resource_safely(name):
    """Use a managed resource and return its operations - untyped."""
    with ManagedResource(name) as res:
        res.read()
        res.write("hello")
        res.read()
    return res.operations


def nested_resources(names):
    """Open multiple resources in nested with statements - untyped."""
    results = []
    if len(names) == 0:
        return results

    with ManagedResource(names[0]) as r1:
        results.append(r1.read())
        if len(names) > 1:
            with ManagedResource(names[1]) as r2:
                results.append(r2.read())
                if len(names) > 2:
                    with ManagedResource(names[2]) as r3:
                        results.append(r3.read())
    return results


def suppress_and_capture(func, *exception_types):
    """Run func inside an ExceptionSuppressor and return (result, caught) - untyped."""
    suppressor = ExceptionSuppressor(*exception_types)
    result = None
    with suppressor:
        result = func()
    return result, suppressor.caught_exception


def run_transaction(operations):
    """Run a list of (key, value) operations in a transaction - untyped."""
    tx = TransactionContext("batch")
    with tx:
        for key, value in operations:
            tx.set(key, value)
    return tx.committed, tx.get_journal()


def run_failing_transaction(operations, fail_index):
    """Run operations and raise at fail_index, testing rollback - untyped."""
    tx = TransactionContext("failing")
    with tx:
        for i, (key, value) in enumerate(operations):
            if i == fail_index:
                raise ValueError(f"Simulated failure at index {i}")
            tx.set(key, value)
    return tx.rolled_back, tx.get_journal()


# --- Typed test functions ---

def test_basic_context_manager() -> bool:
    """Test basic __enter__/__exit__ protocol."""
    res = ManagedResource("test1")
    assert not res.is_open

    with res:
        assert res.is_open
        data = res.read()
        assert data == "data_from_test1"
        written = res.write("payload")
        assert written == 7

    assert not res.is_open
    assert res.operations == ["opened", "read", "write(payload)", "closed"]
    return True


def test_nested_context_managers() -> bool:
    """Test deeply nested context managers."""
    results = nested_resources(["a", "b", "c"])
    assert results == ["data_from_a", "data_from_b", "data_from_c"]

    results_single = nested_resources(["only"])
    assert results_single == ["data_from_only"]

    results_empty = nested_resources([])
    assert results_empty == []
    return True


def test_exception_suppression() -> bool:
    """Test selective exception suppression."""
    # Suppress ValueError
    suppressor = ExceptionSuppressor(ValueError)
    with suppressor:
        raise ValueError("test error")
    assert suppressor.was_suppressed
    assert str(suppressor.caught_exception) == "test error"

    # Don't suppress TypeError when only ValueError is listed
    suppressor2 = ExceptionSuppressor(ValueError)
    caught_type_error = False
    try:
        with suppressor2:
            raise TypeError("wrong type")
    except TypeError:
        caught_type_error = True
    assert caught_type_error
    assert not suppressor2.was_suppressed

    # Suppress multiple types
    suppressor3 = ExceptionSuppressor(ValueError, KeyError, IndexError)
    with suppressor3:
        raise KeyError("missing")
    assert suppressor3.was_suppressed
    return True


def test_transaction_commit() -> bool:
    """Test transaction with successful commit."""
    committed, journal = run_transaction([("a", 1), ("b", 2), ("c", 3)])
    assert committed
    assert "BEGIN:batch" in journal
    assert "COMMIT:batch" in journal
    assert "SET:a=1" in journal
    return True


def test_transaction_rollback() -> bool:
    """Test transaction rollback on exception."""
    rolled_back, journal = run_failing_transaction(
        [("x", 10), ("y", 20), ("z", 30)], fail_index=1
    )
    assert rolled_back
    assert "BEGIN:failing" in journal
    assert any("ROLLBACK" in j for j in journal)
    # Only the first SET should be in journal (before failure)
    assert "SET:x=10" in journal
    assert "SET:y=20" not in journal
    return True


def test_timing_context() -> bool:
    """Test timing context tick counting."""
    timer = TimingContext("bench")
    with timer:
        for _ in range(10):
            timer.tick()
        assert timer.tick_count == 10
        timer.tick()
        assert timer.tick_count == 11

    # Cannot tick outside context
    raised = False
    try:
        timer.tick()
    except RuntimeError:
        raised = True
    assert raised
    return True


def test_stack_context() -> bool:
    """Test nested stack context managers."""
    stack = StackContext()
    assert stack.depth() == 0

    with stack:
        assert stack.depth() == 1
        assert stack.current() == "frame_1"

        with stack:
            assert stack.depth() == 2
            assert stack.current() == "frame_2"

            with stack:
                assert stack.depth() == 3
                snap = stack.snapshot()
                assert len(snap) == 3

            assert stack.depth() == 2

        assert stack.depth() == 1

    assert stack.depth() == 0
    return True


def test_resource_safety() -> bool:
    """Test that resources are always cleaned up."""
    ops = use_resource_safely("safe_test")
    assert ops[0] == "opened"
    assert ops[-1] == "closed"
    assert "read" in ops
    return True


def test_suppress_and_capture() -> bool:
    """Test the suppress_and_capture helper."""
    def raise_value_error():
        raise ValueError("captured!")

    result, caught = suppress_and_capture(raise_value_error, ValueError)
    assert result is None
    assert str(caught) == "captured!"

    def return_42():
        return 42

    result2, caught2 = suppress_and_capture(return_42, ValueError)
    assert result2 == 42
    assert caught2 is None
    return True


def test_all() -> bool:
    """Run all tests."""
    assert test_basic_context_manager()
    assert test_nested_context_managers()
    assert test_exception_suppression()
    assert test_transaction_commit()
    assert test_transaction_rollback()
    assert test_timing_context()
    assert test_stack_context()
    assert test_resource_safety()
    assert test_suppress_and_capture()
    return True


def main():
    """Entry point."""
    if test_all():
        print("hard_context_manager_patterns: ALL TESTS PASSED")
    else:
        print("hard_context_manager_patterns: TESTS FAILED")


if __name__ == "__main__":
    main()
