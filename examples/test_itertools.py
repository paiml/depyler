# Test itertools module mapping
import itertools
from typing import List, Tuple

def chain_lists(list1: List[int], list2: List[int]) -> List[int]:
    """Chain two lists together"""
    return list(itertools.chain(list1, list2))

def get_combinations(items: List[str], r: int) -> List[Tuple[str, ...]]:
    """Get all combinations of r items"""
    return list(itertools.combinations(items, r))

def group_consecutive(numbers: List[int]) -> List[List[int]]:
    """Group consecutive numbers"""
    groups = []
    for k, g in itertools.groupby(enumerate(numbers), lambda x: x[0] - x[1]):
        groups.append([x[1] for x in g])
    return groups

def take_while_positive(numbers: List[int]) -> List[int]:
    """Take numbers while they are positive"""
    return list(itertools.takewhile(lambda x: x > 0, numbers))