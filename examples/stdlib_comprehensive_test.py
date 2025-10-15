"""
Comprehensive stdlib method test for Depyler transpiler
Tests all common list, dict, and set methods

Purpose: Verify that all Python stdlib collection methods transpile correctly
Expected: All methods should generate idiomatic, compilable Rust code
"""

# ============================================================================
# LIST METHODS TESTS
# ============================================================================

def test_list_append() -> int:
    """Test list.append() method"""
    numbers = [1, 2, 3]
    numbers.append(4)
    return len(numbers)  # Expected: 4


def test_list_extend() -> int:
    """Test list.extend() method"""
    numbers = [1, 2]
    numbers.extend([3, 4])
    return len(numbers)  # Expected: 4


def test_list_insert() -> int:
    """Test list.insert() method"""
    numbers = [1, 3]
    numbers.insert(1, 2)
    return numbers[1]  # Expected: 2


def test_list_remove() -> int:
    """Test list.remove() method"""
    numbers = [1, 2, 3, 2]
    numbers.remove(2)
    return len(numbers)  # Expected: 3


def test_list_pop() -> int:
    """Test list.pop() method"""
    numbers = [1, 2, 3]
    last = numbers.pop()
    return last  # Expected: 3


def test_list_pop_index() -> int:
    """Test list.pop(index) method"""
    numbers = [1, 2, 3]
    middle = numbers.pop(1)
    return middle  # Expected: 2


def test_list_clear() -> int:
    """Test list.clear() method"""
    numbers = [1, 2, 3]
    numbers.clear()
    return len(numbers)  # Expected: 0


def test_list_index() -> int:
    """Test list.index() method"""
    numbers = [10, 20, 30]
    pos = numbers.index(20)
    return pos  # Expected: 1


def test_list_count() -> int:
    """Test list.count() method"""
    numbers = [1, 2, 2, 3, 2]
    occurrences = numbers.count(2)
    return occurrences  # Expected: 3


def test_list_reverse() -> int:
    """Test list.reverse() method"""
    numbers = [1, 2, 3]
    numbers.reverse()
    return numbers[0]  # Expected: 3


def test_list_sort() -> int:
    """Test list.sort() method"""
    numbers = [3, 1, 2]
    numbers.sort()
    return numbers[0]  # Expected: 1


# ============================================================================
# DICT METHODS TESTS
# ============================================================================

def test_dict_get() -> int:
    """Test dict.get() method"""
    data = {"a": 1, "b": 2}
    value = data.get("a")
    return value  # Expected: 1


def test_dict_get_default() -> int:
    """Test dict.get() with default"""
    data = {"a": 1}
    value = data.get("b", 0)
    return value  # Expected: 0


def test_dict_keys() -> int:
    """Test dict.keys() method"""
    data = {"a": 1, "b": 2, "c": 3}
    keys = data.keys()
    return len(list(keys))  # Expected: 3


def test_dict_values() -> int:
    """Test dict.values() method"""
    data = {"a": 10, "b": 20}
    values = data.values()
    total = 0
    for v in values:
        total = total + v
    return total  # Expected: 30


def test_dict_items() -> int:
    """Test dict.items() method"""
    data = {"a": 1, "b": 2}
    items = data.items()
    # Note: Tuple unpacking in for loops not yet supported
    # Will need separate test for iteration
    return len(list(items))  # Expected: 2


def test_dict_pop() -> int:
    """Test dict.pop() method"""
    data = {"a": 1, "b": 2}
    value = data.pop("a")
    return value  # Expected: 1


def test_dict_clear() -> int:
    """Test dict.clear() method"""
    data = {"a": 1, "b": 2}
    data.clear()
    return len(data)  # Expected: 0


def test_dict_update() -> int:
    """Test dict.update() method"""
    data = {"a": 1}
    data.update({"b": 2})
    return len(data)  # Expected: 2


def test_dict_setdefault() -> int:
    """Test dict.setdefault() method - existing key"""
    data = {"a": 1, "b": 2}
    value = data.setdefault("a", 999)
    return value  # Expected: 1 (existing value, not default)


def test_dict_setdefault_new() -> int:
    """Test dict.setdefault() method - new key"""
    data = {"a": 1}
    value = data.setdefault("b", 42)
    return value  # Expected: 42 (inserted default)


def test_dict_popitem() -> int:
    """Test dict.popitem() method"""
    data = {"a": 1, "b": 2, "c": 3}
    # Note: popitem removes arbitrary item, so we just check length reduction
    data.popitem()
    return len(data)  # Expected: 2


# ============================================================================
# SET METHODS TESTS
# ============================================================================

def test_set_add() -> int:
    """Test set.add() method"""
    numbers = {1, 2}
    numbers.add(3)
    return len(numbers)  # Expected: 3


def test_set_remove() -> int:
    """Test set.remove() method"""
    numbers = {1, 2, 3}
    numbers.remove(2)
    return len(numbers)  # Expected: 2


def test_set_discard() -> int:
    """Test set.discard() method"""
    numbers = {1, 2, 3}
    numbers.discard(2)
    return len(numbers)  # Expected: 2


def test_set_pop() -> bool:
    """Test set.pop() method"""
    numbers = {1, 2, 3}
    value = numbers.pop()
    return len(numbers) == 2  # Expected: True


def test_set_clear() -> int:
    """Test set.clear() method"""
    numbers = {1, 2, 3}
    numbers.clear()
    return len(numbers)  # Expected: 0


def test_set_union() -> int:
    """Test set.union() method"""
    set1 = {1, 2}
    set2 = {2, 3}
    result = set1.union(set2)
    return len(result)  # Expected: 3


def test_set_intersection() -> int:
    """Test set.intersection() method"""
    set1 = {1, 2, 3}
    set2 = {2, 3, 4}
    result = set1.intersection(set2)
    return len(result)  # Expected: 2


def test_set_difference() -> int:
    """Test set.difference() method"""
    set1 = {1, 2, 3}
    set2 = {2, 3}
    result = set1.difference(set2)
    return len(result)  # Expected: 1


def test_set_update() -> int:
    """Test set.update() method"""
    numbers = {1, 2}
    numbers.update({3, 4})
    return len(numbers)  # Expected: 4
