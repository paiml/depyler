#!/usr/bin/env python3
"""Example module for documentation generation.

This module demonstrates various Python features that Depyler
can document, including functions and classes.
"""

from typing import List, Optional, Dict


def fibonacci(n: int) -> int:
    """Calculate the n-th Fibonacci number.
    
    This function uses an iterative approach for efficiency.
    
    Args:
        n: The position in the Fibonacci sequence (0-indexed)
        
    Returns:
        The n-th Fibonacci number
    """
    if n <= 1:
        return n
    
    a = 0
    b = 1
    for i in range(2, n + 1):
        temp = a + b
        a = b
        b = temp
    return b


def process_data(items: List[int]) -> Dict[str, int]:
    """Process a list of integers and return statistics.
    
    Args:
        items: List of integers to process
        
    Returns:
        Dictionary containing statistics
    """
    stats = {}
    stats["count"] = len(items)
    stats["sum"] = sum(items)
    
    return stats


class DataProcessor:
    """A class for processing and analyzing data.
    
    This class provides methods for various data processing operations.
    """
    
    def __init__(self, name: Optional[str] = None):
        """Initialize a new DataProcessor.
        
        Args:
            name: Optional name for this processor instance
        """
        self.data = []
        self.name = name or "default"
    
    def add_data(self, value: int) -> None:
        """Add a single data point.
        
        Args:
            value: The integer value to add
        """
        self.data.append(value)
    
    def get_count(self) -> int:
        """Get the count of data points.
        
        Returns:
            Number of data points
        """
        return len(self.data)
    
    @staticmethod
    def create_default() -> "DataProcessor":
        """Create a default processor.
        
        Returns:
            New DataProcessor instance
        """
        return DataProcessor("default")