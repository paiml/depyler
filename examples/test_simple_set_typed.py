from typing import Set

def test_simple_set() -> Set[int]:
    s1 = {1, 2, 3}
    s2 = {3, 4, 5}
    
    # Single operation
    result = s1 | s2
    
    return result

def test_set_method() -> Set[str]:
    fruits = {"apple", "banana"}
    fruits.add("cherry")
    return fruits