from typing import List

def calculate_sum(numbers: List[int]) -> int:
    """Calculate the sum of a list of integers."""
    total: int = 0
    for n in numbers:
        total += n
    return total