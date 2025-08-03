# Test set methods
from typing import Set

def test_set_add() -> Set[int]:
    s = {1, 2, 3}
    s.add(4)
    s.add(3)  # Adding existing element should be no-op
    return s

def test_set_remove() -> Set[str]:
    s = {"apple", "banana", "cherry"}
    s.remove("banana")
    # Note: s.remove("grape") would raise KeyError in Python
    return s

def test_set_discard() -> Set[int]:
    s = {1, 2, 3, 4, 5}
    s.discard(3)
    s.discard(10)  # discard() doesn't raise error if element not found
    return s

def test_set_clear():
    s = {1, 2, 3}
    s.clear()
    return len(s) == 0

def test_set_pop() -> int:
    s = {42}  # Single element for predictable behavior
    value = s.pop()
    return value