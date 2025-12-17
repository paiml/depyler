"""
DEPYLER-0978: Golden Annotated Example for Falsification Testing

This file contains EXPLICIT type annotations for every:
- Function parameter
- Return type
- Variable assignment

Purpose: Falsify hypothesis that type inference gaps are the convergence blocker.
If this file compiles to valid Rust, codegen is sound for known types.

Usage:
    mypy examples/golden_annotated_example.py --strict
    ./target/release/depyler transpile examples/golden_annotated_example.py -o /tmp/golden.rs
    rustc /tmp/golden.rs --crate-type lib
"""
from typing import List, Dict, Tuple, Optional, Callable, Any


def numeric_operations(x: int, y: int) -> int:
    """Infers numeric types from arithmetic operations."""
    sum_val: int = x + y
    diff: int = x - y
    product: int = x * y
    quotient: float = x / y

    # Comparison operations also provide hints
    if x > y:
        return sum_val
    else:
        return product


def string_manipulation(text: str) -> str:
    """Infers string type from string methods."""
    # String methods strongly suggest str type
    upper_text: str = text.upper()
    lower_text: str = text.lower()

    if text.startswith("Hello"):
        return text.replace("Hello", "Hi")

    return text.strip()


def list_processing(items: List[str]) -> List[str]:
    """Infers list type from list operations."""
    # List methods suggest list type
    items.append("new item")
    items.extend(["more", "items"])

    # Iteration suggests container type
    result: List[str] = []
    for item in items:
        result.append(item.upper())

    return result


def mixed_inference(data: List[int], multiplier: int) -> int:
    """Multiple inference sources for better confidence."""
    # 'data' used as iterator -> container type
    total: int = 0
    for value in data:
        # 'multiplier' used in numeric context -> numeric type
        total = total + value * multiplier

    # len() also suggests container for 'data'
    average: int = total // len(data)
    return average


def type_conversions_hint(value: str) -> Tuple[str, int, float]:
    """Type conversion functions provide strong hints."""
    # These conversions strongly suggest the target type
    as_string: str = str(value)  # Suggests value could be any type
    as_int: int = int(value)     # Suggests value is convertible to int
    as_float: float = float(value)  # Suggests value is convertible to float

    return (as_string, as_int, as_float)


def boolean_logic(a: bool, b: bool, c: bool) -> bool:
    """Boolean operations suggest bool type."""
    # Logical operators suggest boolean context
    if a and b:
        return True
    elif b or c:
        return False
    else:
        return not c


def dictionary_operations(mapping: Dict[str, str]) -> Optional[str]:
    """Dictionary method usage."""
    # Dictionary methods suggest dict type
    keys: List[str] = list(mapping.keys())
    values: List[str] = list(mapping.values())

    if "key" in mapping:
        return mapping.get("key", "default")

    return None


def function_composition(transform: Callable[[str], str], data: List[str]) -> List[str]:
    """Using parameters as callables."""
    # 'transform' used as callable suggests function type
    result: List[str] = []
    for item in data:
        transformed: str = transform(item)
        result.append(transformed)

    return result


def confidence_levels_demo(
    certain_str: str,
    probable_num: int,
    possible_container: List[int]
) -> Tuple[str, int, int]:
    """Demonstrates different confidence levels."""
    # High confidence: multiple string operations
    processed: str = certain_str.upper().strip().replace(" ", "_")

    # Medium confidence: single numeric operation
    doubled: int = probable_num * 2

    # Low confidence: minimal usage
    size: int = len(possible_container)

    return (processed, doubled, size)


# Additional test cases for edge scenarios

def simple_arithmetic(a: int, b: int) -> int:
    """Simple arithmetic with explicit types."""
    result: int = a + b
    return result


def simple_string_concat(s1: str, s2: str) -> str:
    """Simple string concatenation."""
    result: str = s1 + s2
    return result


def simple_list_sum(numbers: List[int]) -> int:
    """Sum a list of integers."""
    total: int = 0
    for n in numbers:
        total = total + n
    return total


def simple_dict_lookup(d: Dict[str, int], key: str) -> int:
    """Dictionary lookup with default."""
    value: int = d.get(key, 0)
    return value


def optional_handling(maybe_value: Optional[int]) -> int:
    """Handle optional values."""
    if maybe_value is None:
        return 0
    return maybe_value


def tuple_unpacking(pair: Tuple[int, str]) -> str:
    """Unpack a tuple."""
    num: int
    text: str
    num, text = pair
    result: str = f"{text}: {num}"
    return result


def list_comprehension_typed(numbers: List[int]) -> List[int]:
    """List comprehension with explicit type."""
    doubled: List[int] = [n * 2 for n in numbers]
    return doubled


def conditional_expression(flag: bool, a: int, b: int) -> int:
    """Conditional expression (ternary)."""
    result: int = a if flag else b
    return result


# Entry point for testing
def main() -> None:
    """Main function to exercise all examples."""
    # Test numeric operations
    num_result: int = numeric_operations(10, 5)
    print(f"Numeric: {num_result}")

    # Test string manipulation
    str_result: str = string_manipulation("Hello World")
    print(f"String: {str_result}")

    # Test list processing
    items: List[str] = ["a", "b", "c"]
    list_result: List[str] = list_processing(items)
    print(f"List: {list_result}")

    # Test mixed inference
    data: List[int] = [1, 2, 3, 4, 5]
    avg: int = mixed_inference(data, 2)
    print(f"Average: {avg}")

    # Test type conversions
    conv: Tuple[str, int, float] = type_conversions_hint("42")
    print(f"Conversions: {conv}")

    # Test boolean logic
    bool_result: bool = boolean_logic(True, False, True)
    print(f"Boolean: {bool_result}")

    # Test dictionary operations
    mapping: Dict[str, str] = {"key": "value", "other": "data"}
    dict_result: Optional[str] = dictionary_operations(mapping)
    print(f"Dict: {dict_result}")

    # Test simple functions
    arith: int = simple_arithmetic(5, 3)
    concat: str = simple_string_concat("Hello", " World")
    sum_val: int = simple_list_sum([1, 2, 3])
    lookup: int = simple_dict_lookup({"a": 1}, "a")

    print(f"Simple tests: {arith}, {concat}, {sum_val}, {lookup}")


if __name__ == "__main__":
    main()
