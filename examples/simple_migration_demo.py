"""
Simple example to demonstrate migration suggestions.
"""

def accumulator_example(items):
    """Shows accumulator pattern."""
    result = []
    for item in items:
        result.append(item * 2)
    return result


def string_concat_example(values):
    """Shows inefficient string building."""
    output = ""
    for val in values:
        output = output + str(val)
    return output


def enumerate_example(data):
    """Shows range(len()) antipattern."""
    for i in range(len(data)):
        print(i, data[i])


def while_true_example():
    """Shows while True pattern."""
    count = 0
    while True:
        count = count + 1
        if count > 10:
            break
    return count