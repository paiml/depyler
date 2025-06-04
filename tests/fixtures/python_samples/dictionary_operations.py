from typing import Dict, List, Optional

# Test Case 36: Get dictionary value
def get_dict_value(d: Dict[str, int], key: str) -> Optional[int]:
    if key in d:
        return d[key]
    return None

# Test Case 37: Set dictionary value
def set_dict_value(d: Dict[str, int], key: str, value: int) -> Dict[str, int]:
    d[key] = value
    return d

# Test Case 38: Dictionary keys count
def count_dict_keys(d: Dict[str, int]) -> int:
    return len(d)

# Test Case 39: Dictionary contains key
def dict_contains_key(d: Dict[str, int], key: str) -> bool:
    return key in d

# Test Case 40: Get all dictionary keys
def get_dict_keys(d: Dict[str, int]) -> List[str]:
    return list(d.keys())

# Test Case 41: Get all dictionary values
def get_dict_values(d: Dict[str, int]) -> List[int]:
    return list(d.values())

# Test Case 42: Sum dictionary values
def sum_dict_values(d: Dict[str, int]) -> int:
    total: int = 0
    for value in d.values():
        total += value
    return total

# Test Case 43: Find key by value
def find_key_by_value(d: Dict[str, int], target_value: int) -> Optional[str]:
    for key, value in d.items():
        if value == target_value:
            return key
    return None

# Test Case 44: Merge dictionaries
def merge_dicts(d1: Dict[str, int], d2: Dict[str, int]) -> Dict[str, int]:
    result: Dict[str, int] = {}
    for key, value in d1.items():
        result[key] = value
    for key, value in d2.items():
        result[key] = value
    return result

# Test Case 45: Filter dictionary by value
def filter_dict_by_value(d: Dict[str, int], min_value: int) -> Dict[str, int]:
    result: Dict[str, int] = {}
    for key, value in d.items():
        if value >= min_value:
            result[key] = value
    return result

# Test Case 46: Dictionary is empty
def is_dict_empty(d: Dict[str, int]) -> bool:
    return len(d) == 0

# Test Case 47: Count positive values in dict
def count_positive_values(d: Dict[str, int]) -> int:
    count: int = 0
    for value in d.values():
        if value > 0:
            count += 1
    return count

# Test Case 48: Get maximum value from dict
def max_dict_value(d: Dict[str, int]) -> Optional[int]:
    if not d:
        return None
    max_val: int = next(iter(d.values()))
    for value in d.values():
        if value > max_val:
            max_val = value
    return max_val

# Test Case 49: Create inverted dictionary
def invert_dict(d: Dict[str, int]) -> Dict[int, str]:
    result: Dict[int, str] = {}
    for key, value in d.items():
        result[value] = key
    return result

# Test Case 50: Remove key from dictionary
def remove_dict_key(d: Dict[str, int], key: str) -> Dict[str, int]:
    if key in d:
        del d[key]
    return d