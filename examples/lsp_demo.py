#!/usr/bin/env python3
"""
Example: Language Server Protocol (LSP) Demo

This example demonstrates various Python code patterns that would be
handled by Depyler's LSP server for IDE integration features like:
- Code completion
- Hover information
- Go to definition
- Find references
- Diagnostics
"""

# Import statements for completion suggestions
import math
import json
from typing import List, Dict, Optional

# Global variables for reference tracking
CONFIG_FILE = "config.json"
MAX_RETRIES = 3
DEBUG_MODE = True

# Type annotations for hover information
class User:
    """Represents a user in the system."""
    
    def __init__(self, name: str, age: int):
        """Initialize a new user.
        
        Args:
            name: The user's name
            age: The user's age
        """
        self.name = name
        self.age = age
        self.created_at = None
    
    def greet(self) -> str:
        """Return a greeting message."""
        return f"Hello, I'm {self.name}!"
    
    def is_adult(self) -> bool:
        """Check if the user is an adult (18+)."""
        return self.age >= 18

# Function with rich documentation
def calculate_fibonacci(n: int) -> int:
    """Calculate the nth Fibonacci number.
    
    The Fibonacci sequence is defined as:
    - F(0) = 0
    - F(1) = 1
    - F(n) = F(n-1) + F(n-2) for n > 1
    
    Args:
        n: The position in the Fibonacci sequence
        
    Returns:
        The nth Fibonacci number
        
    Raises:
        ValueError: If n is negative
        
    Examples:
        >>> calculate_fibonacci(0)
        0
        >>> calculate_fibonacci(1)
        1
        >>> calculate_fibonacci(10)
        55
    """
    if n < 0:
        raise ValueError("n must be non-negative")
    
    if n <= 1:
        return n
    
    # Variable references for tracking
    prev, curr = 0, 1
    for _ in range(2, n + 1):
        prev, curr = curr, prev + curr
    
    return curr

# Complex function for go-to-definition
def process_users(users: List[User], filter_adults: bool = True) -> Dict[str, List[User]]:
    """Process and categorize users.
    
    Args:
        users: List of User objects to process
        filter_adults: Whether to filter only adult users
        
    Returns:
        Dictionary with 'adults' and 'minors' keys
    """
    result = {"adults": [], "minors": []}
    
    for user in users:
        # Method call - LSP should resolve User.is_adult()
        if user.is_adult():
            result["adults"].append(user)
        else:
            result["minors"].append(user)
    
    # Using global variable - LSP should find definition
    if DEBUG_MODE:
        print(f"Processed {len(users)} users")
    
    return result

# Nested function definitions
def create_handler(prefix: str):
    """Create a message handler with a prefix."""
    
    def handler(message: str) -> str:
        """Handle a message by adding prefix."""
        # Reference to outer scope variable
        return f"{prefix}: {message}"
    
    return handler

# Class with inheritance for symbol hierarchy
class AdminUser(User):
    """An admin user with special privileges."""
    
    def __init__(self, name: str, age: int, permissions: List[str]):
        super().__init__(name, age)
        self.permissions = permissions
    
    def has_permission(self, permission: str) -> bool:
        """Check if admin has a specific permission."""
        return permission in self.permissions
    
    def greet(self) -> str:
        """Admin-specific greeting."""
        # Overrides parent method
        return f"Hello, I'm Admin {self.name}!"

# Function with multiple references
def validate_age(age: int) -> bool:
    """Validate if age is in acceptable range."""
    MIN_AGE = 0
    MAX_AGE = 150
    
    # Multiple references to parameters and local variables
    if age < MIN_AGE:
        return False
    if age > MAX_AGE:
        return False
    
    # Reference to the parameter again
    return age >= 0

# Code with potential diagnostics
def problematic_function():
    """Function that might trigger LSP diagnostics."""
    
    # Unused variable (warning)
    unused_var = 42
    
    # Type mismatch (error in typed context)
    result: int = "not an int"  # Type error
    
    # Undefined variable reference (error)
    # print(undefined_variable)
    
    # Unreachable code (warning)
    return result
    print("This won't execute")

# Lambda and comprehension expressions
data_processors = {
    "double": lambda x: x * 2,
    "square": lambda x: x ** 2,
    "stringify": lambda x: str(x)
}

# List comprehension with references
def process_numbers(numbers: List[int]) -> List[int]:
    """Process numbers using various transformations."""
    # Reference to data_processors
    doubled = [data_processors["double"](n) for n in numbers]
    
    # Nested comprehension
    matrix = [[i * j for j in range(3)] for i in range(3)]
    
    # Generator expression
    sum_of_squares = sum(n ** 2 for n in numbers if n > 0)
    
    return doubled

# Decorator usage
def log_calls(func):
    """Decorator to log function calls."""
    def wrapper(*args, **kwargs):
        print(f"Calling {func.__name__}")
        return func(*args, **kwargs)
    return wrapper

@log_calls
def important_operation(value: str) -> str:
    """An important operation that should be logged."""
    return value.upper()

# Context manager for with statements
class FileManager:
    """Context manager for file operations."""
    
    def __init__(self, filename: str, mode: str = 'r'):
        self.filename = filename
        self.mode = mode
        self.file = None
    
    def __enter__(self):
        self.file = open(self.filename, self.mode)
        return self.file
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        if self.file:
            self.file.close()

# Main execution with various LSP triggers
if __name__ == "__main__":
    # Create users - completion should suggest User class
    users = [
        User("Alice", 25),
        User("Bob", 17),
        AdminUser("Charlie", 30, ["read", "write", "delete"])
    ]
    
    # Process users - hover should show function signature
    categorized = process_users(users)
    
    # Access results - completion should suggest dict keys
    adults = categorized["adults"]
    
    # Method calls - go-to-definition should work
    for user in adults:
        print(user.greet())
    
    # Using imported modules - completion should work
    pi_value = math.pi
    sqrt_2 = math.sqrt(2)
    
    # JSON operations - hover should show types
    config_data = {"debug": DEBUG_MODE, "retries": MAX_RETRIES}
    json_str = json.dumps(config_data)
    
    # Fibonacci calculation - references should be found
    fib_10 = calculate_fibonacci(10)
    print(f"10th Fibonacci number: {fib_10}")
    
    # Handler creation - nested scopes
    info_handler = create_handler("INFO")
    error_handler = create_handler("ERROR")
    
    print(info_handler("Application started"))
    print(error_handler("Something went wrong"))
    
    # File operations with context manager
    try:
        with FileManager(CONFIG_FILE, 'w') as f:
            f.write(json_str)
    except IOError as e:
        print(f"Error writing config: {e}")
    
    # Data processing
    numbers = [1, 2, 3, 4, 5]
    processed = process_numbers(numbers)
    print(f"Processed: {processed}")
    
    # Decorator in action
    result = important_operation("hello world")
    print(f"Result: {result}")