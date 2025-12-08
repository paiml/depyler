"""Reproduction for tuple literal not iterable.

The issue: Python `set((1, 2, 3))` creates a set from tuple.
In Rust, tuples don't implement IntoIterator, so `(1, 2, 3).into_iter()`
doesn't work.

Error: E0599: `(i32, i32, i32)` is not an iterator

Should generate: `vec![1, 2, 3].into_iter()` instead of tuple
"""

from typing import Set


def make_set() -> Set[int]:
    """Create a set from a tuple literal."""
    return set((1, 2, 3))
