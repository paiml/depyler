from typing import Set

def test_set_creation() -> Set[int]:
    """Test basic set creation and operations"""
    # Create sets using literal syntax
    s1 = {1, 2, 3}
    s2 = {3, 4, 5}
    
    # Create empty set
    empty = set()
    
    return s1

def test_set_operators() -> Set[int]:
    """Test set operators"""
    s1 = {1, 2, 3}
    s2 = {3, 4, 5}
    
    # Union
    union = s1 | s2  # {1, 2, 3, 4, 5}
    
    # Intersection
    intersection = s1 & s2  # {3}
    
    # Difference
    diff = s1 - s2  # {1, 2}
    
    # Symmetric difference
    sym_diff = s1 ^ s2  # {1, 2, 4, 5}
    
    return union

def test_set_methods():
    """Test set mutation methods"""
    s = {1, 2, 3}
    
    # Add element
    s.add(4)
    
    # Remove element (raises KeyError if not found)
    s.remove(2)
    
    # Discard element (no error if not found)
    s.discard(5)
    
    return s

def test_set_comprehension() -> Set[int]:
    """Test set comprehension"""
    # Set comprehension
    squares = {x * x for x in range(5)}
    
    # With condition
    even_squares = {x * x for x in range(10) if x % 2 == 0}
    
    return even_squares