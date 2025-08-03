"""
Comprehensive showcase of Depyler's type inference capabilities.
This example demonstrates how Depyler can infer types from usage patterns.
"""

def numeric_operations(x, y):
    """Infers numeric types from arithmetic operations."""
    sum_val = x + y
    diff = x - y
    product = x * y
    quotient = x / y
    
    # Comparison operations also provide hints
    if x > y:
        return sum_val
    else:
        return product


def string_manipulation(text):
    """Infers string type from string methods."""
    # String methods strongly suggest str type
    upper_text = text.upper()
    lower_text = text.lower()
    
    if text.startswith("Hello"):
        return text.replace("Hello", "Hi")
    
    return text.strip()


def list_processing(items):
    """Infers list type from list operations."""
    # List methods suggest list type
    items.append("new item")
    items.extend(["more", "items"])
    
    # Iteration suggests container type
    result = []
    for item in items:
        result.append(item.upper())
    
    return result


def mixed_inference(data, multiplier):
    """Multiple inference sources for better confidence."""
    # 'data' used as iterator -> container type
    total = 0
    for value in data:
        # 'multiplier' used in numeric context -> numeric type
        total = total + value * multiplier
    
    # len() also suggests container for 'data'
    average = total / len(data)
    return average


def type_conversions_hint(value):
    """Type conversion functions provide strong hints."""
    # These conversions strongly suggest the target type
    as_string = str(value)  # Suggests value could be any type
    as_int = int(value)     # Suggests value is convertible to int
    as_float = float(value) # Suggests value is convertible to float
    
    return (as_string, as_int, as_float)


def boolean_logic(a, b, c):
    """Boolean operations suggest bool type."""
    # Logical operators suggest boolean context
    if a and b:
        return True
    elif b or c:
        return False
    else:
        return not c


def dictionary_operations(mapping):
    """Dictionary method usage."""
    # Dictionary methods suggest dict type
    keys = mapping.keys()
    values = mapping.values()
    
    if "key" in mapping:
        return mapping.get("key", "default")
    
    return None


def function_composition(transform, data):
    """Using parameters as callables."""
    # 'transform' used as callable suggests function type
    result = []
    for item in data:
        transformed = transform(item)
        result.append(transformed)
    
    return result


def confidence_levels_demo(certain_str, probable_num, possible_container):
    """Demonstrates different confidence levels."""
    # High confidence: multiple string operations
    processed = certain_str.upper().strip().replace(" ", "_")
    
    # Medium confidence: single numeric operation
    doubled = probable_num * 2
    
    # Low confidence: minimal usage
    size = len(possible_container)
    
    return (processed, doubled, size)