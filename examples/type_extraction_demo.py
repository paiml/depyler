#!/usr/bin/env python3
"""
Example: Type Extraction Demonstration

This example showcases various Python type annotations that
Depyler's type extraction system can handle during transpilation.
"""

from typing import List, Dict, Set, Optional, Union, Tuple, Generic, TypeVar

# Type variables
T = TypeVar('T')
K = TypeVar('K')
V = TypeVar('V')

# Basic type annotations
def simple_types(
    a: int,
    b: float,
    c: str,
    d: bool,
    e: None
) -> int:
    """Function with simple type annotations."""
    return a

# Container types
def container_types(
    items: List[int],
    mapping: Dict[str, float],
    unique: Set[str],
    coords: Tuple[int, int, int]
) -> List[str]:
    """Function with container type annotations."""
    return [str(item) for item in items]

# Optional and Union types
def optional_types(
    maybe_value: Optional[int],
    either_type: Union[int, str],
    complex_union: Union[int, str, float, None]
) -> Optional[str]:
    """Function with optional and union types."""
    if maybe_value is not None:
        return str(maybe_value)
    return None

# Nested types
def nested_types(
    matrix: List[List[int]],
    lookup: Dict[str, List[float]],
    optional_dict: Optional[Dict[str, int]],
    union_list: List[Union[int, str]]
) -> Dict[str, List[Optional[int]]]:
    """Function with nested type annotations."""
    return {}

# Generic types
def generic_function(items: List[T]) -> T:
    """Generic function with type variable."""
    return items[0] if items else None

# Custom generic class
class Container(Generic[T]):
    """Custom generic container."""
    def __init__(self, value: T):
        self.value = value
    
    def get(self) -> T:
        return self.value
    
    def set(self, value: T) -> None:
        self.value = value

# Generic with multiple parameters
class Mapping(Generic[K, V]):
    """Generic mapping with key and value types."""
    def __init__(self):
        self.data: Dict[K, V] = {}
    
    def put(self, key: K, value: V) -> None:
        self.data[key] = value
    
    def get(self, key: K) -> Optional[V]:
        return self.data.get(key)

# Complex nested generics
def complex_generics(
    data: List[Optional[Dict[str, Union[int, float]]]],
    processor: Container[List[T]],
    mappings: Dict[str, Mapping[str, int]]
) -> Union[Container[int], List[Optional[str]]]:
    """Function with complex nested generic types."""
    return Container(42)

# Type aliases (custom types)
UserId = int
Username = str
UserData = Dict[UserId, Username]

def custom_types(
    user_id: UserId,
    username: Username,
    all_users: UserData
) -> Optional[Username]:
    """Function using type aliases."""
    return all_users.get(user_id)

# Tuple with variable length
def variable_tuple(*args: int) -> Tuple[int, ...]:
    """Function with variable-length tuple."""
    return args

# Function type annotation
from typing import Callable

def higher_order(
    func: Callable[[int, int], int],
    a: int,
    b: int
) -> int:
    """Function taking another function as parameter."""
    return func(a, b)

# Demonstrating all type extraction capabilities
def demo_all_types():
    """Demonstrate type extraction for various Python types."""
    print("Type Extraction Examples")
    print("=" * 40)
    
    # Simple types
    result1 = simple_types(1, 2.0, "hello", True, None)
    print(f"Simple types result: {result1}")
    
    # Container types
    result2 = container_types([1, 2, 3], {"a": 1.0}, {"x", "y"}, (1, 2, 3))
    print(f"Container types result: {result2}")
    
    # Optional types
    result3 = optional_types(42, "either", 3.14)
    print(f"Optional types result: {result3}")
    
    # Generic usage
    int_container = Container(42)
    str_container = Container("hello")
    print(f"Generic containers: {int_container.get()}, {str_container.get()}")
    
    # Mapping usage
    mapping = Mapping[str, int]()
    mapping.put("answer", 42)
    print(f"Generic mapping: {mapping.get('answer')}")
    
    # Custom types
    users = {1: "Alice", 2: "Bob"}
    username = custom_types(1, "Alice", users)
    print(f"Custom types result: {username}")

if __name__ == "__main__":
    demo_all_types()