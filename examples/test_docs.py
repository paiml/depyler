#!/usr/bin/env python3
"""Example module for documentation generation.

This module demonstrates various Python features that Depyler
can document, including functions, classes, and type annotations.
"""

from typing import List, Optional, Dict, Union


def fibonacci(n: int) -> int:
    """Calculate the n-th Fibonacci number.
    
    This function uses an iterative approach for efficiency.
    
    Args:
        n: The position in the Fibonacci sequence (0-indexed)
        
    Returns:
        The n-th Fibonacci number
        
    Examples:
        >>> fibonacci(0)
        0
        >>> fibonacci(1)
        1
        >>> fibonacci(10)
        55
    """
    if n <= 1:
        return n
    
    a, b = 0, 1
    for _ in range(2, n + 1):
        a, b = b, a + b
    return b


def process_data(items: List[int], threshold: Optional[int] = None) -> Dict[str, int]:
    """Process a list of integers and return statistics.
    
    This function analyzes a list of integers and returns various
    statistics about the data.
    
    Args:
        items: List of integers to process
        threshold: Optional threshold for filtering (default: None)
        
    Returns:
        Dictionary containing statistics:
        - 'count': Total number of items
        - 'sum': Sum of all items
        - 'max': Maximum value
        - 'min': Minimum value
        - 'above_threshold': Count of items above threshold
    """
    stats = {
        'count': len(items),
        'sum': sum(items),
        'max': max(items) if items else 0,
        'min': min(items) if items else 0,
        'above_threshold': 0
    }
    
    if threshold is not None:
        stats['above_threshold'] = sum(1 for x in items if x > threshold)
    
    return stats


class DataProcessor:
    """A class for processing and analyzing data.
    
    This class provides methods for various data processing operations
    including filtering, transformation, and aggregation.
    
    Attributes:
        data: The internal data storage
        name: Optional name for the processor
    """
    
    def __init__(self, name: Optional[str] = None):
        """Initialize a new DataProcessor.
        
        Args:
            name: Optional name for this processor instance
        """
        self.data: List[int] = []
        self.name = name or "default"
    
    def add_data(self, value: int) -> None:
        """Add a single data point.
        
        Args:
            value: The integer value to add
        """
        self.data.append(value)
    
    def add_batch(self, values: List[int]) -> None:
        """Add multiple data points at once.
        
        Args:
            values: List of integer values to add
        """
        self.data.extend(values)
    
    def filter_data(self, predicate) -> List[int]:
        """Filter data based on a predicate function.
        
        Args:
            predicate: Function that returns True for items to keep
            
        Returns:
            List of filtered values
        """
        return [x for x in self.data if predicate(x)]
    
    def get_summary(self) -> Dict[str, Union[int, float]]:
        """Get a summary of the current data.
        
        Returns:
            Dictionary containing summary statistics
        """
        if not self.data:
            return {'count': 0, 'mean': 0.0}
        
        return {
            'count': len(self.data),
            'sum': sum(self.data),
            'mean': sum(self.data) / len(self.data),
            'max': max(self.data),
            'min': min(self.data)
        }
    
    @staticmethod
    def merge_processors(processors: List['DataProcessor']) -> 'DataProcessor':
        """Merge multiple processors into one.
        
        Args:
            processors: List of DataProcessor instances to merge
            
        Returns:
            New DataProcessor containing all data
        """
        merged = DataProcessor(name="merged")
        for proc in processors:
            merged.add_batch(proc.data)
        return merged
    
    @property
    def is_empty(self) -> bool:
        """Check if the processor has no data."""
        return len(self.data) == 0


def main():
    """Main entry point demonstrating usage."""
    # Create processor
    processor = DataProcessor("example")
    
    # Add some data
    processor.add_batch([1, 2, 3, 4, 5])
    
    # Get summary
    summary = processor.get_summary()
    print(f"Summary: {summary}")
    
    # Process data
    stats = process_data(processor.data, threshold=3)
    print(f"Stats: {stats}")


if __name__ == "__main__":
    main()