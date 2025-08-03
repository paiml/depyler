# Test set operations
from typing import Set

def create_sets() -> tuple[Set[int], Set[int]]:
    s1 = {1, 2, 3, 4, 5}
    s2 = {4, 5, 6, 7, 8}
    return s1, s2

def test_set_intersection() -> Set[int]:
    s1 = {1, 2, 3, 4, 5}
    s2 = {4, 5, 6, 7, 8}
    # Intersection using & operator
    result = s1 & s2
    return result

def test_set_union() -> Set[int]:
    s1 = {1, 2, 3}
    s2 = {3, 4, 5}
    # Union using | operator
    result = s1 | s2
    return result

def test_set_difference() -> Set[int]:
    s1 = {1, 2, 3, 4, 5}
    s2 = {4, 5, 6}
    # Difference using - operator
    result = s1 - s2
    return result

def test_set_symmetric_difference() -> Set[int]:
    s1 = {1, 2, 3, 4}
    s2 = {3, 4, 5, 6}
    # Symmetric difference using ^ operator
    result = s1 ^ s2
    return result