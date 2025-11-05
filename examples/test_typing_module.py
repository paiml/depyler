"""
Comprehensive test of Python typing module transpilation to Rust.

This example demonstrates how Depyler uses Python's typing module
hints to generate strongly-typed Rust code.

Expected Rust mappings:
- List[int] -> Vec<i32>
- Dict[str, int] -> HashMap<String, i32>
- Optional[int] -> Option<i32>
- Union[int, str] -> enum or generics
- Tuple[int, str] -> (i32, String)

Note: These tests verify type annotations are correctly interpreted.
"""

from typing import List, Dict, Tuple, Optional, Union, Set, Any


# ============================================================================
# BASIC TYPE ANNOTATIONS
# ============================================================================

def test_list_typing() -> List[int]:
    """Test List type annotation"""
    numbers: List[int] = [1, 2, 3, 4, 5]
    return numbers


def test_dict_typing() -> Dict[str, int]:
    """Test Dict type annotation"""
    ages: Dict[str, int] = {"Alice": 30, "Bob": 25, "Charlie": 35}
    return ages


def test_set_typing() -> Set[str]:
    """Test Set type annotation"""
    colors: Set[str] = {"red", "green", "blue"}
    return colors


def test_tuple_typing() -> Tuple[str, int, float]:
    """Test Tuple type annotation"""
    person: Tuple[str, int, float] = ("Alice", 30, 5.6)
    return person


# ============================================================================
# OPTIONAL AND UNION TYPES
# ============================================================================

def test_optional_return(value: int) -> Optional[int]:
    """Test Optional return type"""
    if value > 0:
        return value
    else:
        return None


def test_optional_parameter(value: Optional[int]) -> int:
    """Test Optional parameter"""
    if value is not None:
        return value
    else:
        return 0


def test_union_simple(value: int) -> Union[int, str]:
    """Test Union type"""
    if value > 0:
        return value
    else:
        return "negative"


# ============================================================================
# NESTED TYPE ANNOTATIONS
# ============================================================================

def test_nested_list() -> List[List[int]]:
    """Test nested List type"""
    matrix: List[List[int]] = [[1, 2], [3, 4], [5, 6]]
    return matrix


def test_nested_dict() -> Dict[str, Dict[str, int]]:
    """Test nested Dict type"""
    data: Dict[str, Dict[str, int]] = {
        "group1": {"a": 1, "b": 2},
        "group2": {"c": 3, "d": 4}
    }
    return data


def test_list_of_tuples() -> List[Tuple[str, int]]:
    """Test List of Tuples"""
    items: List[Tuple[str, int]] = [("apple", 5), ("banana", 3), ("cherry", 8)]
    return items


def test_dict_of_lists() -> Dict[str, List[int]]:
    """Test Dict of Lists"""
    grades: Dict[str, List[int]] = {
        "Alice": [85, 90, 88],
        "Bob": [78, 82, 80]
    }
    return grades


# ============================================================================
# FUNCTION SIGNATURES WITH COMPLEX TYPES
# ============================================================================

def process_user_data(
    name: str,
    age: int,
    scores: List[float],
    metadata: Optional[Dict[str, str]]
) -> Tuple[str, float]:
    """Test complex function signature"""
    # Calculate average score
    total: float = 0.0
    for score in scores:
        total = total + score

    avg_score: float = total / float(len(scores)) if len(scores) > 0 else 0.0

    # Format result
    result: str = f"{name} ({age})"

    return (result, avg_score)


def merge_data(
    dict1: Dict[str, int],
    dict2: Dict[str, int]
) -> Dict[str, int]:
    """Test Dict parameters and return"""
    merged: Dict[str, int] = {}

    # Add all from dict1
    for key in dict1.keys():
        merged[key] = dict1[key]

    # Add all from dict2
    for key in dict2.keys():
        merged[key] = dict2[key]

    return merged


def filter_positive(numbers: List[int]) -> List[int]:
    """Test List processing"""
    result: List[int] = []

    for num in numbers:
        if num > 0:
            result.append(num)

    return result


def count_by_type(items: List[Union[int, str]]) -> Tuple[int, int]:
    """Test Union types in collections"""
    int_count: int = 0
    str_count: int = 0

    for item in items:
        # Type checking would be needed here
        # Simplified for transpilation
        if isinstance(item, int):
            int_count = int_count + 1
        else:
            str_count = str_count + 1

    return (int_count, str_count)


# ============================================================================
# GENERIC TYPES AND COLLECTIONS
# ============================================================================

def first_element(items: List[int]) -> Optional[int]:
    """Get first element or None"""
    if len(items) > 0:
        return items[0]
    else:
        return None


def last_element(items: List[int]) -> Optional[int]:
    """Get last element or None"""
    if len(items) > 0:
        return items[len(items) - 1]
    else:
        return None


def safe_divide(a: int, b: int) -> Optional[float]:
    """Safe division returning Optional"""
    if b == 0:
        return None
    else:
        return float(a) / float(b)


def get_value(data: Dict[str, int], key: str) -> Optional[int]:
    """Safe dict access"""
    if key in data:
        return data[key]
    else:
        return None


# ============================================================================
# TYPE ALIASES AND CUSTOM TYPES
# ============================================================================

def create_point() -> Tuple[float, float]:
    """Create point (type alias simulation)"""
    point: Tuple[float, float] = (3.0, 4.0)
    return point


def distance_between_points(
    p1: Tuple[float, float],
    p2: Tuple[float, float]
) -> float:
    """Calculate distance using point type"""
    dx: float = p2[0] - p1[0]
    dy: float = p2[1] - p1[1]

    import math
    distance: float = math.sqrt(dx * dx + dy * dy)

    return distance


# ============================================================================
# ADVANCED TYPE PATTERNS
# ============================================================================

def validate_config(config: Dict[str, Any]) -> bool:
    """Test Any type usage"""
    # Check required keys exist
    required: List[str] = ["host", "port", "timeout"]

    for key in required:
        if key not in config:
            return False

    return True


def transform_data(
    data: List[Dict[str, int]]
) -> List[Tuple[str, int]]:
    """Test complex transformation"""
    result: List[Tuple[str, int]] = []

    for item in data:
        for key in item.keys():
            value: int = item[key]
            pair: Tuple[str, int] = (key, value)
            result.append(pair)

    return result


def group_by_first_letter(
    words: List[str]
) -> Dict[str, List[str]]:
    """Test grouping operation"""
    groups: Dict[str, List[str]] = {}

    for word in words:
        if len(word) == 0:
            continue

        first_letter: str = word[0]

        if first_letter not in groups:
            groups[first_letter] = []

        groups[first_letter].append(word)

    return groups


def test_all_typing_features() -> None:
    """Run all typing module tests"""
    # Basic types
    numbers: List[int] = test_list_typing()
    ages: Dict[str, int] = test_dict_typing()
    colors: Set[str] = test_set_typing()
    person: Tuple[str, int, float] = test_tuple_typing()

    # Optional types
    opt_value: Optional[int] = test_optional_return(5)
    opt_param: int = test_optional_parameter(10)
    union_result: Union[int, str] = test_union_simple(-1)

    # Nested types
    matrix: List[List[int]] = test_nested_list()
    nested: Dict[str, Dict[str, int]] = test_nested_dict()
    tuples: List[Tuple[str, int]] = test_list_of_tuples()
    lists: Dict[str, List[int]] = test_dict_of_lists()

    # Complex signatures
    scores: List[float] = [85.5, 90.0, 88.5]
    user_result: Tuple[str, float] = process_user_data("Alice", 30, scores, None)

    # Collection operations
    d1: Dict[str, int] = {"a": 1, "b": 2}
    d2: Dict[str, int] = {"c": 3, "d": 4}
    merged: Dict[str, int] = merge_data(d1, d2)

    nums: List[int] = [-1, 2, -3, 4, 5]
    positive: List[int] = filter_positive(nums)

    # Generic operations
    first: Optional[int] = first_element([1, 2, 3])
    last: Optional[int] = last_element([1, 2, 3])
    division: Optional[float] = safe_divide(10, 3)

    data: Dict[str, int] = {"x": 10, "y": 20}
    value: Optional[int] = get_value(data, "x")

    # Points
    p1: Tuple[float, float] = create_point()
    p2: Tuple[float, float] = (6.0, 8.0)
    dist: float = distance_between_points(p1, p2)

    # Advanced patterns
    config: Dict[str, Any] = {"host": "localhost", "port": 8080, "timeout": 30}
    is_valid: bool = validate_config(config)

    dict_list: List[Dict[str, int]] = [{"a": 1}, {"b": 2}]
    transformed: List[Tuple[str, int]] = transform_data(dict_list)

    words: List[str] = ["apple", "banana", "apricot", "cherry"]
    grouped: Dict[str, List[str]] = group_by_first_letter(words)

    print("All typing module tests completed successfully")
