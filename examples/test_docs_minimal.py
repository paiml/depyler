#!/usr/bin/env python3
"""Minimal example for documentation generation."""


def add(x: int, y: int) -> int:
    """Add two numbers together.
    
    Args:
        x: First number
        y: Second number
        
    Returns:
        Sum of x and y
    """
    return x + y


def multiply(x: int, y: int) -> int:
    """Multiply two numbers.
    
    Args:
        x: First number
        y: Second number
        
    Returns:
        Product of x and y
    """
    return x * y


class Calculator:
    """A simple calculator class."""
    
    def __init__(self):
        """Initialize the calculator."""
        self.result = 0
    
    def compute_sum(self, x: int, y: int) -> int:
        """Compute the sum of two numbers.
        
        Args:
            x: First number
            y: Second number
            
        Returns:
            Sum of x and y
        """
        self.result = x + y
        return self.result