"""
Demonstration of Depyler's type inference capabilities.
"""

def process_numbers(a, b):
    """Numeric operations suggest int/float types."""
    result = a + b
    result = result * 2
    return result


def handle_text(message):
    """String methods suggest str type."""
    formatted = message.upper()
    return formatted


def work_with_list(data):
    """List operations suggest list type."""
    data.append(100)
    total = 0
    for item in data:
        total = total + item
    return total


def check_condition(flag):
    """Boolean context suggests bool type."""
    if flag:
        return 1
    else:
        return 0