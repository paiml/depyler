from typing import Dict, Tuple

def test_tuple_keys() -> Dict[Tuple[int, int], str]:
    """Test dictionary with tuple keys"""
    d: Dict[Tuple[int, int], str] = {}
    
    # Assign values using tuple keys
    d[(0, 0)] = "origin"
    d[(1, 0)] = "right"
    d[(0, 1)] = "up"
    d[(1, 1)] = "diagonal"
    
    # Build tuple dynamically
    x = 2
    y = 3
    d[(x, y)] = "dynamic"
    
    return d