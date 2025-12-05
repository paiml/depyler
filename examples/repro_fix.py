# DEPYLER-0717: Parameter type leaking between functions
# Python pattern: Two functions with same param name but different types
# Problem: Type from first function (List[str]) leaks to second function (List[int])
# Expected: Each function should have independent parameter types

from typing import List

def process_strings(items: List[str]) -> int:
    """Process list of strings."""
    return len(items)

def process_integers(items: List[int]) -> int:
    """Process list of integers."""
    total: int = 0
    for item in items:
        total += item % 10  # This fails if item is String
    return total
