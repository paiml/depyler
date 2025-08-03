"""
Example demonstrating type inference hints in Depyler.
Functions without type annotations will receive inference suggestions.
"""

def process_numbers(data):
    """Process a list of numbers."""
    total = 0
    for num in data:
        total += num
    return total / len(data)


def manipulate_text(text):
    """Various string operations."""
    result = text.upper()
    if result.startswith("HELLO"):
        result = result.replace("HELLO", "HI")
    return result.strip()


def mixed_operations(x, y):
    """Mixed numeric operations."""
    # Numeric operations suggest int/float types
    sum_val = x + y
    product = x * y
    
    # Comparison operations
    if x > y:
        return sum_val
    else:
        return product


def container_operations(items):
    """Operations on containers."""
    # Using len() suggests container type
    if len(items) > 0:
        # Indexing also suggests container
        first = items[0]
        # List methods suggest list type
        items.append(42)
        return first
    return None


def inferred_return_types():
    """Function with inferable return type."""
    x = 10
    y = 20
    # Return type can be inferred as int
    return x + y


def string_formatting(name, age):
    """String formatting with mixed types."""
    # name used with string methods -> str
    formatted_name = name.upper()
    # age used in numeric context -> int
    next_age = age + 1
    
    return f"{formatted_name} will be {next_age} next year"


def iterator_usage(collection, predicate):
    """Using variables as iterators."""
    results = []
    # collection used as iterator
    for item in collection:
        # predicate used as callable
        if predicate(item):
            results.append(item)
    return results


def type_conversions(value):
    """Type conversion hints."""
    # Conversion functions provide strong hints
    text = str(value)
    number = int(value)
    decimal = float(value)
    
    return (text, number, decimal)


# Function with partial annotations for comparison
def partial_annotations(data: list, multiplier) -> list:
    """Only some parameters have annotations."""
    result = []
    for item in data:
        # multiplier used in numeric context
        result.append(item * multiplier)
    return result