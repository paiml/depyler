"""Reproduction for sorted() on float list.

The issue: sorted(roots) where roots is list[float] generates
sorted_vec.sort() but f64 doesn't implement Ord.

Error: E0277: the trait bound `f64: Ord` is not satisfied
"""


def find_roots(a: float, b: float) -> list[float]:
    """Find and return sorted roots."""
    roots = [b, a]  # list[float]
    return sorted(roots)  # BUG: generates .sort() which requires Ord
