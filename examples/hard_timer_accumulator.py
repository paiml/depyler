"""Timer accumulator class for tracking elapsed intervals.

Tests: add time, total, average, max interval, count.
"""


class Timer:
    """Tracks accumulated time intervals."""

    def __init__(self) -> None:
        self.total: int = 0
        self.count: int = 0
        self.max_interval: int = 0

    def add_interval(self, ms: int) -> None:
        self.total = self.total + ms
        self.count = self.count + 1
        if ms > self.max_interval:
            self.max_interval = ms

    def get_total(self) -> int:
        return self.total

    def get_count(self) -> int:
        return self.count

    def get_max(self) -> int:
        return self.max_interval

    def get_average(self) -> int:
        if self.count == 0:
            return 0
        return self.total // self.count


def sum_intervals(intervals: list[int]) -> int:
    """Sum all intervals."""
    total: int = 0
    for iv in intervals:
        total = total + iv
    return total


def max_interval(intervals: list[int]) -> int:
    """Find the maximum interval."""
    if len(intervals) == 0:
        return 0
    best: int = intervals[0]
    i: int = 1
    while i < len(intervals):
        if intervals[i] > best:
            best = intervals[i]
        i = i + 1
    return best


def test_module() -> int:
    """Test timer accumulator."""
    ok: int = 0
    t = Timer()
    t.add_interval(100)
    t.add_interval(200)
    t.add_interval(150)
    if t.get_total() == 450:
        ok = ok + 1
    if t.get_count() == 3:
        ok = ok + 1
    if t.get_max() == 200:
        ok = ok + 1
    if t.get_average() == 150:
        ok = ok + 1
    if sum_intervals([10, 20, 30]) == 60:
        ok = ok + 1
    if max_interval([5, 99, 3, 50]) == 99:
        ok = ok + 1
    if max_interval([]) == 0:
        ok = ok + 1
    return ok
