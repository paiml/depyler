"""
Depyler v1.1.0 Feature Showcase
Demonstrates all new features added in v1.1.0
"""

def showcase_dictionary_assignment():
    """Showcase nested dictionary assignment support"""
    # Simple assignment
    d = {}
    d['key'] = 'value'
    
    # Nested assignment
    nested = {'level1': {'level2': {}}}
    nested['level1']['level2']['level3'] = 'deep value'
    
    # Tuple key assignment
    coords = {}
    coords[(10, 20)] = 'location A'
    coords[(30, 40)] = 'location B'
    
    return nested, coords


def showcase_set_operations():
    """Showcase comprehensive set support"""
    # Set creation and operators
    set1 = {1, 2, 3, 4, 5}
    set2 = {4, 5, 6, 7, 8}
    
    # Set operators
    intersection = set1 & set2  # {4, 5}
    union = set1 | set2         # {1, 2, 3, 4, 5, 6, 7, 8}
    difference = set1 - set2    # {1, 2, 3}
    symmetric_diff = set1 ^ set2  # {1, 2, 3, 6, 7, 8}
    
    # Set methods
    mutable_set = {1, 2, 3}
    mutable_set.add(4)
    mutable_set.remove(2)
    mutable_set.discard(5)  # No error if not present
    
    return intersection, union, difference, symmetric_diff, mutable_set


def showcase_set_comprehensions():
    """Showcase set comprehensions"""
    # Basic set comprehension
    squares = {x * x for x in range(10)}
    
    # Set comprehension with condition
    even_squares = {x * x for x in range(10) if x % 2 == 0}
    
    # Set comprehension from string
    unique_chars = {c.upper() for c in "hello world" if c.isalpha()}
    
    return squares, even_squares, unique_chars


def showcase_frozen_sets():
    """Showcase frozen set support"""
    # Create frozen sets
    immutable1 = frozenset([1, 2, 3, 4])
    immutable2 = frozenset(range(3, 6))
    
    # Frozen sets can be used in set operations
    result = immutable1 & immutable2  # Returns a new frozenset
    
    # Frozen sets can be dictionary keys
    frozen_dict = {immutable1: "first set", immutable2: "second set"}
    
    return result, frozen_dict


def showcase_control_flow():
    """Showcase break and continue statements"""
    # Break example
    result1 = []
    for i in range(10):
        if i == 5:
            break
        result1.append(i)
    
    # Continue example
    result2 = []
    for i in range(10):
        if i % 2 == 0:
            continue
        result2.append(i)
    
    # Nested loops with break
    result3 = []
    for i in range(3):
        for j in range(3):
            if i == 1 and j == 1:
                break  # Only breaks inner loop
            result3.append((i, j))
    
    return result1, result2, result3


def showcase_power_operator():
    """Showcase power operator support"""
    # Integer power
    int_power = 2 ** 10  # 1024
    
    # Float power
    float_power = 2.5 ** 3.0  # 15.625
    
    # Negative exponent
    inverse = 2 ** -3  # 0.125
    
    # Large powers (with overflow protection in Rust)
    large = 10 ** 6  # 1000000
    
    return int_power, float_power, inverse, large


def main():
    """Run all showcases"""
    print("Dictionary Assignment:", showcase_dictionary_assignment())
    print("Set Operations:", showcase_set_operations())
    print("Set Comprehensions:", showcase_set_comprehensions())
    print("Frozen Sets:", showcase_frozen_sets())
    print("Control Flow:", showcase_control_flow())
    print("Power Operator:", showcase_power_operator())


if __name__ == "__main__":
    main()