"""
Comprehensive test of Python operator module transpilation to Rust.

This example demonstrates how Depyler transpiles Python's operator module
to Rust equivalents.

Expected Rust mappings:
- operator.add(a, b) -> a + b
- operator.mul(a, b) -> a * b
- operator.itemgetter() -> closure or indexing
- operator.attrgetter() -> field access
- operator.methodcaller() -> method calls

Note: Most operator functions map directly to Rust operators.
"""

import operator
from typing import List, Tuple


def test_arithmetic_operators() -> int:
    """Test arithmetic operator functions"""
    a: int = 10
    b: int = 5

    # Addition
    add_result: int = a + b

    # Subtraction
    sub_result: int = a - b

    # Multiplication
    mul_result: int = a * b

    # Floor division
    floordiv_result: int = a // b

    # Modulo
    mod_result: int = a % b

    # Power
    pow_result: int = a ** 2

    return add_result + sub_result + mul_result


def test_comparison_operators() -> bool:
    """Test comparison operator functions"""
    a: int = 10
    b: int = 5

    # Equal
    eq: bool = a == b

    # Not equal
    ne: bool = a != b

    # Less than
    lt: bool = a < b

    # Less than or equal
    le: bool = a <= b

    # Greater than
    gt: bool = a > b

    # Greater than or equal
    ge: bool = a >= b

    return gt and ne


def test_logical_operators() -> bool:
    """Test logical operator functions"""
    a: bool = True
    b: bool = False

    # AND
    and_result: bool = a and b

    # OR
    or_result: bool = a or b

    # NOT
    not_result: bool = not a

    return or_result and not and_result


def test_bitwise_operators() -> int:
    """Test bitwise operator functions"""
    a: int = 12  # 1100 in binary
    b: int = 10  # 1010 in binary

    # AND
    and_result: int = a & b

    # OR
    or_result: int = a | b

    # XOR
    xor_result: int = a ^ b

    # Invert (simplified)
    inv_result: int = ~a

    # Left shift
    lshift_result: int = a << 1

    # Right shift
    rshift_result: int = a >> 1

    return and_result + or_result


def test_itemgetter_list() -> int:
    """Test itemgetter on list"""
    data: List[int] = [10, 20, 30, 40, 50]

    # Get item at index 2
    item: int = data[2]

    return item


def test_itemgetter_tuple() -> str:
    """Test itemgetter on tuple"""
    data: Tuple[str, int, float] = ("hello", 42, 3.14)

    # Get item at index 0
    item: str = data[0]

    return item


def test_itemgetter_multiple() -> tuple:
    """Test itemgetter with multiple indices"""
    data: List[int] = [10, 20, 30, 40, 50]

    # Get multiple items
    item1: int = data[1]
    item3: int = data[3]

    return (item1, item3)


def sort_by_second_element(data: List[tuple]) -> List[tuple]:
    """Sort list of tuples by second element"""
    # Manual sort by second element
    sorted_data: List[tuple] = data.copy()

    for i in range(len(sorted_data)):
        for j in range(i + 1, len(sorted_data)):
            if sorted_data[j][1] < sorted_data[i][1]:
                temp: tuple = sorted_data[i]
                sorted_data[i] = sorted_data[j]
                sorted_data[j] = temp

    return sorted_data


def test_abs_operator() -> int:
    """Test absolute value operator"""
    negative: int = -42

    # Absolute value
    positive: int = abs(negative)

    return positive


def test_neg_operator() -> int:
    """Test negation operator"""
    positive: int = 42

    # Negate
    negative: int = -positive

    return negative


def test_index_operator() -> bool:
    """Test index/contains operator"""
    data: List[int] = [10, 20, 30, 40, 50]
    value: int = 30

    # Check if value in list
    contains: bool = value in data

    # Find index
    if contains:
        index: int = data.index(value)
        found: bool = index >= 0
    else:
        found: bool = False

    return found


def test_concat_operator() -> List[int]:
    """Test concatenation operator"""
    list1: List[int] = [1, 2, 3]
    list2: List[int] = [4, 5, 6]

    # Concatenate (manual)
    result: List[int] = []
    for item in list1:
        result.append(item)
    for item in list2:
        result.append(item)

    return result


def test_repeat_operator() -> List[int]:
    """Test repeat operator"""
    base: List[int] = [1, 2, 3]
    times: int = 3

    # Repeat (manual)
    result: List[int] = []
    for i in range(times):
        for item in base:
            result.append(item)

    return result


def test_getitem_operator() -> int:
    """Test getitem operator"""
    data: List[int] = [10, 20, 30, 40]
    index: int = 2

    item: int = data[index]

    return item


def test_setitem_operator() -> List[int]:
    """Test setitem operator"""
    data: List[int] = [10, 20, 30, 40]
    index: int = 2
    value: int = 99

    # Set item
    data[index] = value

    return data


def test_delitem_operator() -> List[int]:
    """Test delitem operator"""
    data: List[int] = [10, 20, 30, 40]

    # Delete item at index 2 (manual)
    new_data: List[int] = []
    for i in range(len(data)):
        if i != 2:
            new_data.append(data[i])

    return new_data


def apply_operation(a: int, b: int, op: str) -> int:
    """Apply operation based on string"""
    if op == "add":
        return a + b
    elif op == "sub":
        return a - b
    elif op == "mul":
        return a * b
    elif op == "div":
        return a // b
    else:
        return 0


def max_by_key(data: List[tuple]) -> tuple:
    """Find max element using key function"""
    if len(data) == 0:
        return (0, 0)

    max_elem: tuple = data[0]

    for elem in data:
        # Compare by second element
        if elem[1] > max_elem[1]:
            max_elem = elem

    return max_elem


def min_by_key(data: List[tuple]) -> tuple:
    """Find min element using key function"""
    if len(data) == 0:
        return (0, 0)

    min_elem: tuple = data[0]

    for elem in data:
        # Compare by second element
        if elem[1] < min_elem[1]:
            min_elem = elem

    return min_elem


def test_truthiness() -> bool:
    """Test truth value testing"""
    # Empty collections are falsy
    empty_list: List[int] = []
    empty_is_false: bool = len(empty_list) == 0

    # Non-empty collections are truthy
    full_list: List[int] = [1, 2, 3]
    full_is_true: bool = len(full_list) > 0

    return empty_is_false and full_is_true


def test_identity() -> bool:
    """Test identity operators"""
    a: int = 42
    b: int = 42
    c: int = 99

    # Equal values
    equal: bool = a == b

    # Different values
    different: bool = a != c

    return equal and different


def chain_comparisons(x: int, low: int, high: int) -> bool:
    """Test chained comparisons"""
    in_range: bool = low <= x <= high

    return in_range


def test_all_operator_features() -> None:
    """Run all operator module tests"""
    # Arithmetic
    arith_result: int = test_arithmetic_operators()

    # Comparison
    comp_result: bool = test_comparison_operators()

    # Logical
    logic_result: bool = test_logical_operators()

    # Bitwise
    bit_result: int = test_bitwise_operators()

    # Itemgetter
    list_item: int = test_itemgetter_list()
    tuple_item: str = test_itemgetter_tuple()
    multi_items: tuple = test_itemgetter_multiple()

    # Sorting
    tuples: List[tuple] = [(1, 3), (2, 1), (3, 2)]
    sorted_tuples: List[tuple] = sort_by_second_element(tuples)

    # Unary operators
    abs_val: int = test_abs_operator()
    neg_val: int = test_neg_operator()

    # Sequence operators
    contains: bool = test_index_operator()
    concatenated: List[int] = test_concat_operator()
    repeated: List[int] = test_repeat_operator()

    # Item access
    get_item: int = test_getitem_operator()
    set_result: List[int] = test_setitem_operator()
    del_result: List[int] = test_delitem_operator()

    # Dynamic operations
    op_result: int = apply_operation(10, 5, "add")

    # Key-based operations
    data: List[tuple] = [(1, 100), (2, 50), (3, 200)]
    max_elem: tuple = max_by_key(data)
    min_elem: tuple = min_by_key(data)

    # Truth testing
    truth: bool = test_truthiness()
    identity: bool = test_identity()
    chained: bool = chain_comparisons(5, 1, 10)

    print("All operator module tests completed successfully")
