"""Counter class with increment, decrement, and reset.

Tests: counter increment, decrement, bounds, reset, step operations.
"""


class Counter:
    """Simple bounded counter."""

    def __init__(self, start: int, max_val: int) -> None:
        self.value: int = start
        self.max_val: int = max_val
        self.ops: int = 0

    def increment(self) -> None:
        if self.value < self.max_val:
            self.value = self.value + 1
        self.ops = self.ops + 1

    def decrement(self) -> None:
        if self.value > 0:
            self.value = self.value - 1
        self.ops = self.ops + 1

    def reset(self) -> None:
        self.value = 0
        self.ops = self.ops + 1

    def get_value(self) -> int:
        return self.value

    def get_ops(self) -> int:
        return self.ops


def counter_step(start: int, steps: int, max_val: int) -> int:
    """Increment a counter 'steps' times and return final value."""
    val: int = start
    i: int = 0
    while i < steps:
        if val < max_val:
            val = val + 1
        i = i + 1
    return val


def count_within_range(values: list[int], lo: int, hi: int) -> int:
    """Count values in [lo, hi]."""
    count: int = 0
    for v in values:
        if v >= lo and v <= hi:
            count = count + 1
    return count


def test_module() -> int:
    """Test counter operations."""
    ok: int = 0
    c = Counter(0, 10)
    c.increment()
    c.increment()
    c.increment()
    if c.get_value() == 3:
        ok = ok + 1
    c.decrement()
    if c.get_value() == 2:
        ok = ok + 1
    if c.get_ops() == 4:
        ok = ok + 1
    c.reset()
    if c.get_value() == 0:
        ok = ok + 1
    if counter_step(0, 5, 10) == 5:
        ok = ok + 1
    if counter_step(8, 5, 10) == 10:
        ok = ok + 1
    if count_within_range([1, 5, 10, 15, 20], 5, 15) == 3:
        ok = ok + 1
    return ok
