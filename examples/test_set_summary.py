# Comprehensive test of implemented set features
from typing import Set

def test_all_set_features() -> Set[int]:
    # Set creation
    s1 = {1, 2, 3}
    s2 = set([4, 5, 6])
    empty = set()
    
    # Set operators
    union = s1 | s2
    intersection = {1, 2, 3} & {2, 3, 4}
    difference = {1, 2, 3, 4} - {3, 4, 5}
    symmetric_diff = {1, 2, 3} ^ {2, 3, 4}
    
    # Set methods
    s1.add(4)
    s2.discard(5)
    
    # Combine results
    result = union | intersection | difference | symmetric_diff
    return result