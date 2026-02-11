"""Stack class implementation with various operations.

Tests: list-backed stack, size tracking, empty checks.
"""


class IntStack:
    """Stack of integers backed by a list."""

    def __init__(self) -> None:
        self.items: list[int] = []
        self.size: int = 0

    def push(self, val: int) -> None:
        self.items.append(val)
        self.size += 1

    def pop(self) -> int:
        if self.size == 0:
            return -1
        self.size -= 1
        return self.items.pop()

    def peek(self) -> int:
        if self.size == 0:
            return -1
        return self.items[self.size - 1]

    def is_empty(self) -> bool:
        return self.size == 0

    def get_size(self) -> int:
        return self.size


def reverse_list(arr: list[int]) -> list[int]:
    """Reverse a list using a stack approach."""
    result: list[int] = []
    i: int = len(arr) - 1
    while i >= 0:
        result.append(arr[i])
        i -= 1
    return result


def min_of_list(arr: list[int]) -> int:
    """Find minimum in list, stack-style iteration."""
    if len(arr) == 0:
        return 0
    m: int = arr[0]
    i: int = 1
    while i < len(arr):
        if arr[i] < m:
            m = arr[i]
        i += 1
    return m


def test_module() -> int:
    """Test stack operations."""
    ok: int = 0

    s = IntStack()
    s.push(10)
    s.push(20)
    s.push(30)
    if s.get_size() == 3:
        ok += 1
    if s.peek() == 30:
        ok += 1

    v: int = s.pop()
    if v == 30:
        ok += 1
    if s.get_size() == 2:
        ok += 1

    r: list[int] = reverse_list([1, 2, 3, 4, 5])
    if r == [5, 4, 3, 2, 1]:
        ok += 1

    m: int = min_of_list([5, 3, 8, 1, 4])
    if m == 1:
        ok += 1

    return ok
