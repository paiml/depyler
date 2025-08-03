# Test set comprehensions
from typing import Set

def test_simple_set_comp() -> Set[int]:
    """Basic set comprehension"""
    return {x * 2 for x in range(5)}

def test_set_comp_with_condition() -> Set[int]:
    """Set comprehension with condition"""
    return {x for x in range(10) if x % 2 == 0}

def test_set_comp_from_list() -> Set[str]:
    """Set comprehension from list"""
    words = ["hello", "world", "hello", "python"]
    return {w.upper() for w in words}

def test_set_comp_complex_expr() -> Set[int]:
    """Set comprehension with complex expression"""
    numbers = [1, 2, 3, 4, 5]
    return {x * x + 1 for x in numbers if x < 4}

def test_set_comp_with_expression() -> Set[int]:
    """Set comprehension with expression"""
    return {x * x for x in range(5) if x > 0}