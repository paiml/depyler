"""
Simple example for debugging features.
"""

def calculate_sum(numbers: list[int]) -> int:
    """Calculate sum of numbers."""
    total = 0
    for num in numbers:
        total = total + num
    return total


def find_max(values: list[int]) -> int:
    """Find maximum value."""
    if not values:
        return 0
    
    max_val = values[0]
    for val in values:
        if val > max_val:
            max_val = val
    
    return max_val


def process_data(data: list[int]) -> dict[str, int]:
    """Process data and return statistics."""
    result = {
        "sum": calculate_sum(data),
        "max": find_max(data),
        "count": len(data)
    }
    return result