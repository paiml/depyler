"""
Example demonstrating patterns that trigger performance warnings.
"""

def string_concat_in_loop(items):
    """String concatenation in loop - O(n²) complexity."""
    result = ""
    for item in items:
        result = result + str(item)  # Warning: creates new string each time
    return result


def nested_loops_cubic(matrix):
    """Deeply nested loops - O(n³) complexity."""
    total = 0
    for i in range(len(matrix)):
        for j in range(len(matrix[i])):
            for k in range(len(matrix[i][j])):
                total += matrix[i][j][k]
    return total


def repeated_expensive_computation(data):
    """Expensive operations in loop."""
    results = []
    for item in data:
        # Sorting repeatedly - O(n² log n)
        sorted_data = sorted(data)
        results.append(item * len(sorted_data))
    return results


def inefficient_list_operations(items):
    """Inefficient list operations."""
    while len(items) > 0:
        # O(n) remove in loop creates O(n²)
        items.remove(items[0])


def large_list_in_loop(n):
    """Creating large objects in loops."""
    results = []
    for i in range(n):
        # Creating large list repeatedly
        temp = [j for j in range(1000)]
        results.append(sum(temp))
    return results


def linear_search_in_loop(items, targets):
    """Linear search in nested loop - O(n²)."""
    found = []
    for target in targets:
        # index() is O(n) search
        if target in items:
            idx = items.index(target)
            found.append((target, idx))
    return found


def power_in_tight_loop(values):
    """Expensive math operations in loop."""
    results = []
    for x in values:
        # Power operation is expensive
        result = x ** 3.5
        results.append(result)
    return results


def range_len_antipattern(items):
    """Using range(len()) instead of enumerate."""
    for i in range(len(items)):
        process_item(i, items[i])


def aggregate_in_nested_loop(matrix):
    """Computing aggregates repeatedly."""
    result = 0
    for row in matrix:
        for col in row:
            # Computing sum repeatedly
            total = sum(row)
            result += col * total
    return result


def large_parameter_by_value(huge_list: list, huge_dict: dict) -> int:
    """Large parameters passed by value."""
    # These large collections are copied
    return len(huge_list) + len(huge_dict)


# Helper function
def process_item(idx, item):
    pass