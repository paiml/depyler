"""Reproduction for walrus operator in list comprehension.

The issue: walrus variable assigned in 'if' condition must be
accessible in the tuple expression of the comprehension.

Python: [(w, length) for w in words if (length := len(w)) > 3]

WRONG Rust (separate closures):
  .filter(|w| { let length = ...; length > 3 })
  .map(|w| (w, length))  // ERROR: length not in scope

RIGHT Rust (filter_map):
  .filter_map(|w| {
      let length = w.len();
      if length > 3 { Some((w, length)) } else { None }
  })
"""


def count_long_words(words: list[str]) -> list[tuple[str, int]]:
    """Filter and capture word lengths using walrus operator."""
    return [(w, length) for w in words if (length := len(w)) > 3]
