"""
Comprehensive stdlib method test for Depyler transpiler
Tests all common list methods

Purpose: Verify that all Python stdlib collection methods transpile correctly
Expected: All methods should generate idiomatic, compilable Rust code
"""

from typing import List

# ============================================================================
# LIST METHODS TESTS
# ============================================================================

def test_list_append() -> int:
    """Test list.append() method"""
    numbers: List[int] = [1, 2, 3]
    numbers.append(4)
    return len(numbers)  # Expected: 4


def test_list_extend() -> int:
    """Test list.extend() method"""
    numbers: List[int] = [1, 2]
    numbers.extend([3, 4])
    return len(numbers)  # Expected: 4


def test_list_insert() -> int:
    """Test list.insert() method"""
    numbers: List[int] = [1, 3]
    numbers.insert(1, 2)
    return numbers[1]  # Expected: 2


def test_list_remove() -> int:
    """Test list.remove() method"""
    numbers: List[int] = [1, 2, 3, 2]
    numbers.remove(2)
    return len(numbers)  # Expected: 3


def test_list_pop() -> int:
    """Test list.pop() method"""
    numbers: List[int] = [1, 2, 3]
    last: int = numbers.pop()
    return last  # Expected: 3


def test_list_pop_index() -> int:
    """Test list.pop(index) method"""
    numbers: List[int] = [1, 2, 3]
    middle: int = numbers.pop(1)
    return middle  # Expected: 2


def test_list_clear() -> int:
    """Test list.clear() method"""
    numbers: List[int] = [1, 2, 3]
    numbers.clear()
    return len(numbers)  # Expected: 0


def test_list_index() -> int:
    """Test list.index() method"""
    numbers: List[int] = [10, 20, 30]
    pos: int = numbers.index(20)
    return pos  # Expected: 1


def test_list_count() -> int:
    """Test list.count() method"""
    numbers: List[int] = [1, 2, 2, 3, 2]
    occurrences: int = numbers.count(2)
    return occurrences  # Expected: 3


def test_list_reverse() -> int:
    """Test list.reverse() method"""
    numbers: List[int] = [1, 2, 3]
    numbers.reverse()
    return numbers[0]  # Expected: 3


def test_list_sort() -> int:
    """Test list.sort() method"""
    numbers: List[int] = [3, 1, 2]
    numbers.sort()
    return numbers[0]  # Expected: 1


# ============================================================================
# DICT METHODS TESTS (using string key/value dicts)
# ============================================================================

def test_dict_contains() -> int:
    """Test dict key containment check"""
    data = {"a": 1, "b": 2, "c": 3}
    count: int = 0
    if "a" in data:
        count = count + 1
    if "d" in data:
        count = count + 1
    return count  # Expected: 1


def test_dict_len() -> int:
    """Test dict len"""
    data = {"a": 1, "b": 2, "c": 3}
    return len(data)  # Expected: 3


def test_dict_clear() -> int:
    """Test dict.clear() method"""
    data = {"a": 1, "b": 2}
    data.clear()
    return len(data)  # Expected: 0


# ============================================================================
# LIST-BASED SET SIMULATION TESTS
# ============================================================================

def list_contains(lst: List[int], val: int) -> int:
    """Check if list contains value. 1 if yes, 0 if no."""
    for x in lst:
        if x == val:
            return 1
    return 0


def list_add_unique(lst: List[int], val: int) -> int:
    """Add value to list if not already present. Return new length."""
    if list_contains(lst, val) == 0:
        lst.append(val)
    return len(lst)


def list_remove_val(lst: List[int], val: int) -> int:
    """Remove value from list. Return new length."""
    i: int = 0
    while i < len(lst):
        if lst[i] == val:
            lst.remove(val)
            return len(lst)
        i = i + 1
    return len(lst)


def test_unique_add() -> int:
    """Test adding unique elements."""
    numbers: List[int] = [1, 2]
    result: int = list_add_unique(numbers, 3)
    return result  # Expected: 3


def test_unique_add_duplicate() -> int:
    """Test adding duplicate element."""
    numbers: List[int] = [1, 2, 3]
    result: int = list_add_unique(numbers, 2)
    return result  # Expected: 3 (unchanged)


def test_unique_remove() -> int:
    """Test removing element."""
    numbers: List[int] = [1, 2, 3]
    result: int = list_remove_val(numbers, 2)
    return result  # Expected: 2


def test_module() -> int:
    """Run all tests and return pass count."""
    passed: int = 0

    # List tests
    if test_list_append() == 4:
        passed = passed + 1
    if test_list_extend() == 4:
        passed = passed + 1
    if test_list_insert() == 2:
        passed = passed + 1
    if test_list_remove() == 3:
        passed = passed + 1
    if test_list_pop() == 3:
        passed = passed + 1
    if test_list_pop_index() == 2:
        passed = passed + 1
    if test_list_clear() == 0:
        passed = passed + 1
    if test_list_index() == 1:
        passed = passed + 1
    if test_list_count() == 3:
        passed = passed + 1
    if test_list_reverse() == 3:
        passed = passed + 1
    if test_list_sort() == 1:
        passed = passed + 1

    # Dict tests
    if test_dict_contains() == 1:
        passed = passed + 1
    if test_dict_len() == 3:
        passed = passed + 1
    if test_dict_clear() == 0:
        passed = passed + 1

    # Set-like tests (using lists)
    if test_unique_add() == 3:
        passed = passed + 1
    if test_unique_add_duplicate() == 3:
        passed = passed + 1
    if test_unique_remove() == 2:
        passed = passed + 1

    return passed
