"""Reproduction for list replication pattern.

The issue: Python's `[True] * n` creates list with n copies of True.
This generates `vec![true] * n` but Vec doesn't support * operator.

Error: E0369: cannot multiply `Vec<bool>` by `i32`

Python: is_prime = [True] * (limit + 1)

WRONG Rust:
  vec![true] * (limit + 1)  // ERROR: cannot multiply Vec by i32

RIGHT Rust:
  vec![true; (limit + 1) as usize]
"""


def create_sieve(limit: int) -> list[bool]:
    """Create boolean sieve array."""
    return [True] * (limit + 1)
