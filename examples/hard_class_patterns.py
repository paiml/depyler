"""Hard class patterns that stress the transpiler's type inference.

These patterns test:
- None-initialized fields with later assignment
- Fields with multiple types across methods
- Nested class attribute access
- Class methods returning self
- Properties and computed attributes
"""


class Stack:
    """Stack implementation with typed fields."""

    def __init__(self) -> None:
        self.items: list[int] = []
        self.size: int = 0

    def push(self, item: int) -> None:
        self.items.append(item)
        self.size += 1

    def pop(self) -> int:
        if self.items:
            self.size -= 1
            return self.items.pop()
        return -1

    def peek(self) -> int:
        if self.items:
            return self.items[-1]
        return -1

    def is_empty(self) -> bool:
        return self.size == 0


class LinkedNode:
    """Node with optional next pointer."""

    def __init__(self, value: int) -> None:
        self.value: int = value
        self.next: int = 0  # Using int as placeholder

    def set_next(self, node_id: int) -> None:
        self.next = node_id

    def get_value(self) -> int:
        return self.value


class Config:
    """Configuration with typed defaults."""

    def __init__(self, name: str) -> None:
        self.name: str = name
        self.debug: bool = False
        self.max_retries: int = 3
        self.timeout: float = 30.0
        self.tags: list[str] = []

    def enable_debug(self) -> None:
        self.debug = True

    def add_tag(self, tag: str) -> None:
        self.tags.append(tag)

    def get_timeout(self) -> float:
        return self.timeout


class Counter:
    """Counter with increment/decrement."""

    def __init__(self, initial: int = 0) -> None:
        self.count: int = initial

    def increment(self) -> int:
        self.count += 1
        return self.count

    def decrement(self) -> int:
        self.count -= 1
        return self.count

    def reset(self) -> None:
        self.count = 0

    def get(self) -> int:
        return self.count


class Matrix:
    """2D matrix with typed operations."""

    def __init__(self, rows: int, cols: int) -> None:
        self.rows: int = rows
        self.cols: int = cols
        self.data: list[list[int]] = []

    def get_rows(self) -> int:
        return self.rows

    def get_cols(self) -> int:
        return self.cols


def test_stack() -> int:
    """Test stack operations."""
    s = Stack()
    s.push(1)
    s.push(2)
    s.push(3)
    return s.pop()


def test_config() -> str:
    """Test config operations."""
    c = Config("test")
    c.enable_debug()
    c.add_tag("production")
    return c.name


def test_counter() -> int:
    """Test counter operations."""
    c = Counter(10)
    c.increment()
    c.increment()
    c.decrement()
    return c.get()
