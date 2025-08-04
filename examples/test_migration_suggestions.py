"""
Example demonstrating patterns that trigger migration suggestions.
"""

def accumulator_pattern(items):
    """Pattern: accumulator - should suggest iterator methods."""
    result = []
    for item in items:
        if item > 0:
            result.append(item * 2)
    return result


def error_with_none(value):
    """Pattern: returning None for errors - should suggest Result."""
    if not validate(value):
        return None
    
    processed = process_data(value)
    if processed is None:
        return None
    
    return processed


def mutating_parameter(data):
    """Pattern: mutating parameters - should suggest ownership patterns."""
    data.append(42)
    data.sort()
    return data


def type_checking_pattern(value):
    """Pattern: runtime type checking - should suggest enums."""
    if isinstance(value, str):
        return value.upper()
    elif isinstance(value, int):
        return value * 2
    else:
        return str(value)


def inefficient_string_building(items):
    """Pattern: string concatenation - should suggest efficient methods."""
    result = ""
    for item in items:
        result = result + str(item) + ", "
    return result


def enumerate_pattern(items):
    """Pattern: range(len()) - should suggest enumerate."""
    for i in range(len(items)):
        print(f"{i}: {items[i]}")


def filter_map_pattern(data):
    """Pattern: filter + map in loop - should suggest filter_map."""
    output = []
    for x in data:
        if x > 0:
            output.append(x * x)
    return output


def while_true_pattern():
    """Pattern: while True - should suggest loop."""
    counter = 0
    while True:
        counter += 1
        if counter > 10:
            break
    return counter


def none_checking_pattern(optional_value):
    """Pattern: None checking - should suggest pattern matching."""
    if optional_value is not None:
        return process(optional_value)
    else:
        return default_value()


# Helper functions (would be defined elsewhere)
def validate(x):
    return x > 0

def process_data(x):
    return x * 2

def process(x):
    return x

def default_value():
    return 0