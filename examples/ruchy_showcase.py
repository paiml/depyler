#!/usr/bin/env python3
"""
Showcase of Python features that transpile to Ruchy script format.
This demonstrates the Pythonic to functional transformation capabilities.
"""

from typing import List, Optional


def fibonacci(n: int) -> int:
    """Calculate fibonacci number recursively."""
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)


def quicksort(arr: List[int]) -> List[int]:
    """Sort array using quicksort algorithm."""
    if len(arr) <= 1:
        return arr
    
    pivot = arr[0]
    less = [x for x in arr[1:] if x < pivot]
    greater = [x for x in arr[1:] if x >= pivot]
    
    return quicksort(less) + [pivot] + quicksort(greater)


def process_data(numbers: List[int]) -> List[int]:
    """Process data using functional pipeline style."""
    # This will be transformed to pipeline operators in Ruchy
    result = [x * 2 for x in numbers if x > 0]
    return result


def greet(name: str, title: Optional[str] = None) -> str:
    """Create a greeting with optional title."""
    if title:
        return f"Hello, {title} {name}!"
    else:
        return f"Hello, {name}!"


class DataProcessor:
    """A class for processing data."""
    
    def __init__(self, threshold: int = 0):
        self.threshold = threshold
    
    def filter_data(self, data: List[int]) -> List[int]:
        """Filter data based on threshold."""
        return [x for x in data if x > self.threshold]
    
    def transform_data(self, data: List[int]) -> List[int]:
        """Transform data by applying a function."""
        return [x * 2 + 1 for x in data]


async def fetch_and_process(url: str) -> str:
    """Async function that will map to Ruchy's async support."""
    # This would use actual async operations in practice
    data = await fetch_data(url)
    processed = process_text(data)
    return processed


async def fetch_data(url: str) -> str:
    """Simulate fetching data."""
    return f"Data from {url}"


def process_text(text: str) -> str:
    """Process text data."""
    return text.upper()


def pattern_matching_example(value):
    """Example that could be transformed to match expression."""
    if isinstance(value, int):
        return f"Integer: {value}"
    elif isinstance(value, str):
        return f"String: {value}"
    elif isinstance(value, list):
        return f"List with {len(value)} items"
    else:
        return "Unknown type"


def main():
    """Main entry point."""
    # Test fibonacci
    print(f"Fibonacci(10) = {fibonacci(10)}")
    
    # Test quicksort
    arr = [64, 34, 25, 12, 22, 11, 90]
    sorted_arr = quicksort(arr)
    print(f"Sorted array: {sorted_arr}")
    
    # Test data processing
    numbers = [1, -2, 3, -4, 5]
    processed = process_data(numbers)
    print(f"Processed: {processed}")
    
    # Test greeting
    print(greet("Alice"))
    print(greet("Bob", "Dr."))
    
    # Test class
    processor = DataProcessor(threshold=10)
    data = [5, 10, 15, 20, 25]
    filtered = processor.filter_data(data)
    transformed = processor.transform_data(filtered)
    print(f"Filtered and transformed: {transformed}")
    
    # Test pattern matching
    print(pattern_matching_example(42))
    print(pattern_matching_example("hello"))
    print(pattern_matching_example([1, 2, 3]))


if __name__ == "__main__":
    main()