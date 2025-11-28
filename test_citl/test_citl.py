def process_data(items: list) -> dict:
    """Process items and return counts."""
    result = {}
    for item in items:
        if item in result:
            result[item] += 1
        else:
            result[item] = 1
    return result

def find_max(numbers: list) -> int:
    """Find maximum value."""
    max_val = numbers[0]
    for n in numbers:
        if n > max_val:
            max_val = n
    return max_val
