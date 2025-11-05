"""
Comprehensive test of Python copy module transpilation to Rust.

This example demonstrates how Depyler transpiles Python's copy module
operations to their Rust equivalents.

Expected Rust mappings:
- copy.copy() -> .clone() (shallow copy)
- copy.deepcopy() -> .clone() (deep copy if implemented)

Note: Deep copy semantics may differ between Python and Rust.
"""

import copy
from typing import List, Dict


def test_shallow_copy_list() -> List[int]:
    """Test shallow copy of list"""
    original: List[int] = [1, 2, 3, 4, 5]

    # Shallow copy
    copied: List[int] = copy.copy(original)

    # Modify copy
    copied.append(6)

    # Original should remain unchanged (in separate test)
    return copied


def test_shallow_copy_dict() -> Dict[str, int]:
    """Test shallow copy of dictionary"""
    original: Dict[str, int] = {"a": 1, "b": 2, "c": 3}

    # Shallow copy
    copied: Dict[str, int] = copy.copy(original)

    # Modify copy
    copied["d"] = 4

    return copied


def test_list_copy_method() -> List[int]:
    """Test list.copy() method"""
    original: List[int] = [10, 20, 30]

    # Using .copy() method
    copied: List[int] = original.copy()

    # Modify
    copied[0] = 99

    return copied


def test_dict_copy_method() -> Dict[str, str]:
    """Test dict.copy() method"""
    original: Dict[str, str] = {"key1": "value1", "key2": "value2"}

    # Using .copy() method
    copied: Dict[str, str] = original.copy()

    # Modify
    copied["key3"] = "value3"

    return copied


def test_nested_list_shallow_copy() -> List[List[int]]:
    """Test shallow copy behavior with nested lists"""
    original: List[List[int]] = [[1, 2], [3, 4], [5, 6]]

    # Shallow copy
    copied: List[List[int]] = copy.copy(original)

    # Modify nested list (affects original in Python due to shallow copy)
    # In Rust, this depends on clone implementation
    copied.append([7, 8])

    return copied


def test_deep_copy_nested_list() -> List[List[int]]:
    """Test deep copy of nested list"""
    original: List[List[int]] = [[1, 2], [3, 4], [5, 6]]

    # Deep copy
    copied: List[List[int]] = copy.deepcopy(original)

    # Modify nested structure
    if len(copied) > 0:
        copied[0].append(99)

    return copied


def test_deep_copy_nested_dict() -> Dict[str, Dict[str, int]]:
    """Test deep copy of nested dictionary"""
    original: Dict[str, Dict[str, int]] = {
        "group1": {"a": 1, "b": 2},
        "group2": {"c": 3, "d": 4}
    }

    # Deep copy
    copied: Dict[str, Dict[str, int]] = copy.deepcopy(original)

    # Modify nested dict
    if "group1" in copied:
        copied["group1"]["e"] = 5

    return copied


def manual_shallow_copy_list(original: List[int]) -> List[int]:
    """Manual implementation of shallow list copy"""
    copied: List[int] = []

    for item in original:
        copied.append(item)

    return copied


def manual_shallow_copy_dict(original: Dict[str, int]) -> Dict[str, int]:
    """Manual implementation of shallow dict copy"""
    copied: Dict[str, int] = {}

    for key in original.keys():
        copied[key] = original[key]

    return copied


def manual_deep_copy_nested_list(original: List[List[int]]) -> List[List[int]]:
    """Manual implementation of deep copy for nested lists"""
    copied: List[List[int]] = []

    for sublist in original:
        # Copy each sublist
        new_sublist: List[int] = []
        for item in sublist:
            new_sublist.append(item)
        copied.append(new_sublist)

    return copied


def test_copy_with_modification() -> bool:
    """Test that copy creates independent object"""
    original: List[int] = [1, 2, 3]
    copied: List[int] = copy.copy(original)

    # Modify original
    original.append(4)

    # Check that copied is independent
    is_independent: bool = len(copied) != len(original)

    return is_independent


def test_reference_vs_copy() -> bool:
    """Test difference between reference and copy"""
    original: List[int] = [1, 2, 3]

    # Copy
    copied: List[int] = copy.copy(original)

    # Reference (assignment)
    reference: List[int] = original

    # Modify original
    original.append(4)

    # Copy should be different, reference should be same
    copy_different: bool = len(copied) != len(original)
    reference_same: bool = len(reference) == len(original)

    return copy_different and reference_same


def clone_list_with_transform(original: List[int], multiplier: int) -> List[int]:
    """Clone list and apply transformation"""
    cloned: List[int] = []

    for item in original:
        cloned.append(item * multiplier)

    return cloned


def clone_dict_with_filter(original: Dict[str, int], threshold: int) -> Dict[str, int]:
    """Clone dictionary with filtering"""
    filtered: Dict[str, int] = {}

    for key in original.keys():
        value: int = original[key]
        if value > threshold:
            filtered[key] = value

    return filtered


def merge_copied_dicts(dict1: Dict[str, int], dict2: Dict[str, int]) -> Dict[str, int]:
    """Merge two dictionaries by copying"""
    merged: Dict[str, int] = copy.copy(dict1)

    for key in dict2.keys():
        merged[key] = dict2[key]

    return merged


def test_copy_empty_collections() -> tuple:
    """Test copying empty collections"""
    empty_list: List[int] = []
    empty_dict: Dict[str, int] = {}

    copied_list: List[int] = copy.copy(empty_list)
    copied_dict: Dict[str, int] = copy.copy(empty_dict)

    return (len(copied_list), len(copied_dict))


def test_copy_single_element() -> tuple:
    """Test copying single-element collections"""
    single_list: List[int] = [42]
    single_dict: Dict[str, int] = {"answer": 42}

    copied_list: List[int] = copy.copy(single_list)
    copied_dict: Dict[str, int] = copy.copy(single_dict)

    return (copied_list[0], copied_dict["answer"])


def test_all_copy_features() -> None:
    """Run all copy module tests"""
    # Shallow copy tests
    list_copy: List[int] = test_shallow_copy_list()
    dict_copy: Dict[str, int] = test_shallow_copy_dict()

    # Method-based copies
    list_method: List[int] = test_list_copy_method()
    dict_method: Dict[str, str] = test_dict_copy_method()

    # Nested structure tests
    nested_shallow: List[List[int]] = test_nested_list_shallow_copy()
    nested_deep_list: List[List[int]] = test_deep_copy_nested_list()
    nested_deep_dict: Dict[str, Dict[str, int]] = test_deep_copy_nested_dict()

    # Manual implementations
    manual_list: List[int] = manual_shallow_copy_list([1, 2, 3])
    manual_dict: Dict[str, int] = manual_shallow_copy_dict({"x": 10, "y": 20})
    manual_deep: List[List[int]] = manual_deep_copy_nested_list([[1, 2], [3, 4]])

    # Independence tests
    is_independent: bool = test_copy_with_modification()
    ref_vs_copy: bool = test_reference_vs_copy()

    # Transformation tests
    data: List[int] = [1, 2, 3, 4, 5]
    transformed: List[int] = clone_list_with_transform(data, 2)

    scores: Dict[str, int] = {"alice": 85, "bob": 72, "charlie": 95}
    filtered: Dict[str, int] = clone_dict_with_filter(scores, 80)

    # Merge test
    d1: Dict[str, int] = {"a": 1, "b": 2}
    d2: Dict[str, int] = {"c": 3, "d": 4}
    merged: Dict[str, int] = merge_copied_dicts(d1, d2)

    # Edge cases
    empty_sizes: tuple = test_copy_empty_collections()
    single_values: tuple = test_copy_single_element()

    print("All copy module tests completed successfully")
