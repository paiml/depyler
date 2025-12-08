"""Reproduction for map(str, ...) pattern.

The issue: Python's `map(str, items)` converts each item to string.
This generates `map(str, &items)` but `str` is a Rust type, not a function.

Error: E0423: expected value, found builtin type `str`

Python: print(" ".join(map(str, result)))

WRONG Rust:
  map(str, &result)  // ERROR: str is a type, not a value

RIGHT Rust:
  result.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")
"""


def format_numbers(nums: list[int]) -> str:
    """Convert list of numbers to space-separated string."""
    return " ".join(map(str, nums))
