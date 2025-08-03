"""
Simple example demonstrating type inference hints in Depyler.
"""

def add_numbers(a, b):
    """Add two numbers - should infer numeric types."""
    return a + b


def process_text(text):
    """Process text - should infer string type."""
    result = text.upper()
    return result


def calculate_average(numbers):
    """Calculate average - should infer list of numbers."""
    total = 0
    count = 0
    for num in numbers:
        total = total + num
        count = count + 1
    
    if count > 0:
        return total / count
    return 0


def string_checker(s):
    """Check string properties."""
    if s.startswith("hello"):
        return True
    return False


def list_operations(items):
    """Perform list operations."""
    items.append(42)
    return len(items)