"""Depyler v1.1.0 Core Features Demo"""

def test_dictionary_assignment():
    """Nested dictionary assignment"""
    d = {}
    d["key"] = "value"
    
    # Nested assignment
    nested = {}
    nested["level1"] = {}
    nested["level1"]["level2"] = "deep"
    
    return nested


def test_set_operations():
    """Set operations with operators"""
    set1 = {1, 2, 3}
    set2 = {2, 3, 4}
    
    # Note: These need to be assigned to variables first
    # Direct operations on literals are treated as bitwise
    intersection = set1 & set2
    union = set1 | set2
    
    return intersection, union


def test_power_operator():
    """Power operator examples"""
    x = 2 ** 3  # 8
    y = 5 ** 2  # 25
    return x + y


def test_break_continue():
    """Break and continue in loops"""
    # Break example
    for i in range(10):
        if i == 5:
            break
            
    # Continue example  
    count = 0
    for i in range(10):
        if i % 2 == 0:
            continue
        count = count + 1
    
    return count